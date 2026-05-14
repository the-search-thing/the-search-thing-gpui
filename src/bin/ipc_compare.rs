//! Compare serialization cost: JSON-RPC-shaped payloads vs framed bincode (native prototype).
//!
//! Run: `cargo run --bin ipc-compare --manifest-path the-search-thing-gpui/Cargo.toml --release`
//!
//! This does **not** hit the real sidecar; it isolates wire-format overhead so we know whether
//! a binary protocol is worth a sidecar dual-mode listener.

use std::hint::black_box;
use std::time::Instant;

use gpui_port_tst::ipc::{
    read_framed_bincode, write_framed_bincode, NativeRequest, NativeResponse, SearchHit,
};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
struct JsonRpcRequestShape {
    jsonrpc: &'static str,
    id: u64,
    method: &'static str,
    params: serde_json::Value,
}

#[derive(Serialize)]
struct JsonRpcResponseShape {
    jsonrpc: &'static str,
    id: u64,
    result: serde_json::Value,
}

fn synthetic_hits(n: usize) -> Vec<SearchHit> {
    (0..n)
        .map(|i| SearchHit {
            path: format!("/project/src/module_{i}.rs"),
            snippet: format!("hit number {i} with some text"),
            score: i as f32 * 0.01,
        })
        .collect()
}

fn bench_json_roundtrip(iterations: usize, hits: &[SearchHit]) {
    let params = json!({ "q": "native rust experiment", "limit": hits.len() });
    let result_json = json!({ "hits": hits });

    let t0 = Instant::now();
    for i in 0..iterations {
        let req = JsonRpcRequestShape {
            jsonrpc: "2.0",
            id: i as u64,
            method: "search.query",
            params: params.clone(),
        };
        let line = serde_json::to_string(&req).expect("request json");
        black_box(line.len());

        let res = JsonRpcResponseShape {
            jsonrpc: "2.0",
            id: i as u64,
            result: result_json.clone(),
        };
        let line = serde_json::to_string(&res).expect("response json");
        black_box(line.len());
    }
    let elapsed = t0.elapsed();
    let ms_per_iter = elapsed.as_secs_f64() * 1000.0 / iterations as f64;
    println!(
        "JSON-RPC-shaped serde_json (req+res serialize): {:?} total | {:.3} ms/iter ({:.1} μs/iter)",
        elapsed,
        ms_per_iter,
        ms_per_iter * 1000.0,
    );
}

fn bench_bincode_framed_roundtrip(iterations: usize, hits: &[SearchHit]) {
    let t0 = Instant::now();
    for _ in 0..iterations {
        let req = NativeRequest::SearchQuery {
            query: "native rust experiment".into(),
            limit: hits.len() as u32,
        };
        let mut buf = Vec::new();
        write_framed_bincode(&mut buf, &req).expect("encode req");
        black_box(buf.len());

        let res = NativeResponse::SearchResults {
            hits: hits.to_vec(),
        };
        let mut buf = Vec::new();
        write_framed_bincode(&mut buf, &res).expect("encode res");
        let mut cursor = std::io::Cursor::new(buf);
        let _: NativeResponse = read_framed_bincode(&mut cursor).expect("decode res");
    }
    let elapsed = t0.elapsed();
    let ms_per_iter = elapsed.as_secs_f64() * 1000.0 / iterations as f64;
    println!(
        "Framed bincode (encode req + encode/decode res): {:?} total | {:.3} ms/iter ({:.1} μs/iter)",
        elapsed,
        ms_per_iter,
        ms_per_iter * 1000.0,
    );
}

fn main() {
    let iterations = 500_usize;
    let hit_counts = [50_usize, 500, 5_000];

    println!(
        "ipc-compare ({} iterations); measuring serialization only — no subprocess\n",
        iterations
    );

    for &n in &hit_counts {
        let hits = synthetic_hits(n);
        println!("--- {} search hits ---", n);
        bench_json_roundtrip(iterations, &hits);
        bench_bincode_framed_roundtrip(iterations, &hits);
        println!();
    }
}
