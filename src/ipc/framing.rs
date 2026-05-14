//! Length-prefixed bincode frames: `u32_le(len) || payload`.
//!
//! This is intentionally simpler than JSON-RPC: one frame per logical message, no line trimming,
//! trivial backpressure (read exact length). Swap `bincode` for `postcard` or capnp later without
//! changing GPUI call sites if [`crate::ipc::BackendTransport`] stays stable.

use std::io::{Read, Write};

use serde::{de::DeserializeOwned, Serialize};

const MAX_FRAME_BYTES: usize = 256 * 1024 * 1024;

#[derive(Debug, thiserror::Error)]
pub enum FramingError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("frame length {len} exceeds maximum {MAX_FRAME_BYTES}")]
    FrameTooLarge { len: usize },
    #[error("bincode: {0}")]
    Bincode(#[from] bincode::Error),
}

pub fn write_framed_bincode<W: Write, T: Serialize>(writer: &mut W, msg: &T) -> Result<(), FramingError> {
    let payload = bincode::serialize(msg)?;
    let len = u32::try_from(payload.len()).map_err(|_| FramingError::FrameTooLarge {
        len: payload.len(),
    })?;
    writer.write_all(&len.to_le_bytes())?;
    writer.write_all(&payload)?;
    Ok(())
}

pub fn read_framed_bincode<R: Read, T: DeserializeOwned>(reader: &mut R) -> Result<T, FramingError> {
    let mut len_buf = [0u8; 4];
    reader.read_exact(&mut len_buf)?;
    let len = u32::from_le_bytes(len_buf) as usize;
    if len > MAX_FRAME_BYTES {
        return Err(FramingError::FrameTooLarge { len });
    }
    let mut payload = vec![0u8; len];
    reader.read_exact(&mut payload)?;
    Ok(bincode::deserialize(&payload)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ipc::{NativeRequest, NativeResponse, SearchHit};

    #[test]
    fn roundtrip_request_response() {
        let req = NativeRequest::SearchQuery {
            query: "hello".into(),
            limit: 50,
        };
        let mut buf = Vec::new();
        write_framed_bincode(&mut buf, &req).unwrap();
        let mut cursor = std::io::Cursor::new(buf);
        let got: NativeRequest = read_framed_bincode(&mut cursor).unwrap();
        assert_eq!(got, req);

        let res = NativeResponse::SearchResults {
            hits: vec![SearchHit {
                path: "/tmp/a.txt".into(),
                snippet: "hello world".into(),
                score: 1.0,
            }],
        };
        let mut buf = Vec::new();
        write_framed_bincode(&mut buf, &res).unwrap();
        let mut cursor = std::io::Cursor::new(buf);
        let got: NativeResponse = read_framed_bincode(&mut cursor).unwrap();
        assert_eq!(got, res);
    }
}
