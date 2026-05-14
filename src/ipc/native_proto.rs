use serde::{Deserialize, Serialize};

/// One row of search UI state (mirror sidecar `search.query` semantics loosely).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchHit {
    pub path: String,
    pub snippet: String,
    pub score: f32,
}

/// Rust-native request envelope (no JSON-RPC wrapper bytes).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NativeRequest {
    Ping,
    SearchQuery { query: String, limit: u32 },
}

/// Rust-native response envelope.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NativeResponse {
    Pong,
    SearchResults { hits: Vec<SearchHit> },
    Error { code: i32, message: String },
}
