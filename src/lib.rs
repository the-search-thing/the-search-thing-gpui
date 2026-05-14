//! GPUI frontend crate. Core UI lives in `main`; [`ipc`] holds native-Rust IPC experiments
//! for replacing JSON-RPC over stdio when both ends are Rust.

pub mod ipc;
