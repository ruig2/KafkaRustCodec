use crate::primitives::{DecodeErrors, FromByte, HeaderRequest, Message, Request};
use crate::requests::{RequestApiVersions, RequestApiVersions_2};
use bytes::Buf;

pub mod primitives;
pub mod requests;

// ToDo: assign the result to a reference in the parameter rather than return the entire Request
pub fn decode_request(buf: &mut Buf) -> impl Request {
    let mut header = HeaderRequest::default();
    let mut len: i32 = 0;
    len.decode(buf);
    header.decode(buf);
    println!("{}", header);

    let mut req: RequestApiVersions = RequestApiVersions::default();
    req
}

#[test]
fn test_decode_request() {
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
    // Just echo to make things easier for now
    const KAFKA_RESPONSE_API_VERSIONS: &[u8] = KAFKA_REQUEST_API_VERSIONS;

    use bytes::Buf;
    use std::io::Cursor;

    let mut buf = Cursor::new(KAFKA_REQUEST_API_VERSIONS);
    decode_request(&mut buf);
}

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
    // Just echo to make things easier for now
    const KAFKA_RESPONSE_API_VERSIONS: &[u8] = KAFKA_REQUEST_API_VERSIONS;

    use bytes::Buf;
    use std::io::Cursor;

    let mut buf = Cursor::new(KAFKA_REQUEST_API_VERSIONS);
    //let mut req = RequestApiVersions::default();
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
