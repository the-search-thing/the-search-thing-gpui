//! Spawn [`the-search-thing-sidecar`] with [`THE_SEARCH_THING_IPC_MODE`] native framing.

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use the_search_thing::sidecar::native_ipc::{
    read_framed_bincode, write_framed_bincode, NativeRequest, NativeResponse, NativeSearchRow,
};

const SIDECAR_BIN_ENV: &str = "THE_SEARCH_THING_SIDECAR";

fn sidecar_filename() -> &'static str {
    if cfg!(target_os = "windows") {
        "the-search-thing-sidecar.exe"
    } else {
        "the-search-thing-sidecar"
    }
}

pub fn resolve_sidecar_path() -> Result<PathBuf, String> {
    if let Ok(from_env) = std::env::var(SIDECAR_BIN_ENV) {
        let p = PathBuf::from(from_env);
        if p.exists() {
            return Ok(p);
        }
        return Err(format!(
            "{SIDECAR_BIN_ENV} is set but path does not exist: {}",
            p.display()
        ));
    }

    let gpui_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repo_root = gpui_manifest
        .parent()
        .ok_or_else(|| "gpui crate manifest has no parent (expected repo root)".to_string())?;

    let bin_name = sidecar_filename();
    for profile in ["debug", "release"] {
        let candidate = repo_root.join("target").join(profile).join(bin_name);
        if candidate.exists() {
            return Ok(candidate);
        }
    }

    Err(format!(
        "sidecar binary not found under {}target/{{debug,release}}/{}. Build it from the repo root (`cargo build -p the_search_thing --bin the-search-thing-sidecar`) or set {}.",
        repo_root.display(),
        bin_name,
        SIDECAR_BIN_ENV
    ))
}

/// One-shot native IPC search request (spawn → write framed [`NativeRequest::SearchQuery`] → read [`NativeResponse`]).
pub fn native_search(query: &str) -> Result<Vec<NativeSearchRow>, String> {
    let sidecar = resolve_sidecar_path()?;
    let query_owned = query.to_string();

    let mut cmd = Command::new(&sidecar);
    cmd.stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .env("THE_SEARCH_THING_IPC_MODE", "native")
        .current_dir(repo_root_for_sidecar(&sidecar));

    let mut child = cmd
        .spawn()
        .map_err(|e| format!("failed to spawn {}: {e}", sidecar.display()))?;

    {
        let mut stdin = child.stdin.take().ok_or_else(|| "sidecar stdin missing".to_string())?;
        write_framed_bincode(&mut stdin, &NativeRequest::SearchQuery { query: query_owned })
            .map_err(|e| e.to_string())?;
    }

    let output = child
        .wait_with_output()
        .map_err(|e| format!("sidecar wait: {e}"))?;

    if !output.status.success() {
        return Err(format!(
            "sidecar exited with {}; stderr logged above",
            output.status
        ));
    }

    let mut stdout = output.stdout.as_slice();
    match read_framed_bincode(&mut stdout).map_err(|e| e.to_string())? {
        NativeResponse::SearchQueryResult(body) => Ok(body.results),
        NativeResponse::Error { message, .. } => Err(message),
        other => Err(format!("unexpected native IPC response: {other:?}")),
    }
}

fn repo_root_for_sidecar(sidecar_path: &Path) -> PathBuf {
    sidecar_path
        .parent()
        .and_then(Path::parent)
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."))
}
