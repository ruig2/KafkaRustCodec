use crate::primitives::{DecodeErrors, FromByte, HeaderRequest, Message, Request};
use crate::requests::RequestApiVersions;
use bytes::Buf;

pub mod primitives;
pub mod requests;

// ToDo: Move to test
fn main() -> Result<(), DecodeErrors> {
    const KAFKA_REQUEST_API_VERSIONS: &[u8] = &[
        // Request/response Size => INT32
        0, 0, 0, 20, // value: 20
        // api_key => INT16
        0, 18, // value: 18
        // api_version => INT16
        0, 2, // value: 2
        // correlation_id => INT32
        0, 0, 0, 1, // value: 1
        // client_id => NULLABLE_STRING
        0, 10, 99, 111, 110, 115, 117, 109, 101, 114, 45, 49, // len: 10, value: consumer-1
    ];

    use bytes::Buf;
    use std::io::Cursor;

    let mut buf = Cursor::new(KAFKA_REQUEST_API_VERSIONS);
    RequestApiVersions::decode_new(&mut buf)
        .and_then({
            |req| {
                println!("req.size: {}", req.size);
                println!("req.header: {}", req.header);
                Ok(())
            }
        })
        .or_else(|err| Err(err))
}
