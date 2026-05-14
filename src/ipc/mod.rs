//! Experimental IPC layer for the GPUI rewrite.
//!
//! Today the Electron app speaks JSON-RPC 2.0 as NDJSON over the sidecar stdin/stdout.
//! When the UI is Rust-only, we can keep that for compatibility **or** swap the wire format
//! behind a small trait ([`BackendTransport`]) without rewriting Helix / indexing internals first.
//!
//! Suggested rollout:
//! 1. Implement [`BackendTransport`] for JSON-RPC subprocess (production parity with TS client).
//! 2. Add an optional sidecar mode (or sibling binary) that speaks [`framing`] + [`native_proto`].
//! 3. Benchmark end-to-end; keep JSON-RPC as fallback for debugging and external tools.

mod framing;
mod native_proto;

pub use framing::{read_framed_bincode, write_framed_bincode, FramingError};
pub use native_proto::{NativeRequest, NativeResponse, SearchHit};

/// Frontend-facing abstraction: GPUI calls these methods; transport can be JSON-RPC or binary.
///
/// Sync for now so experiments work without pulling in async traits; GPUI can wrap calls in
/// `cx.background_spawn` per gpui-ce async examples.
pub trait BackendTransport {
    fn ping(&mut self) -> Result<(), BackendError>;
    fn search_query(&mut self, query: &str, limit: u32) -> Result<Vec<SearchHit>, BackendError>;
}

#[derive(Debug, thiserror::Error)]
pub enum BackendError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("framing: {0}")]
    Framing(#[from] FramingError),
    #[error("backend error {code}: {message}")]
    Remote { code: i32, message: String },
}
