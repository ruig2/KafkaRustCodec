use kafka_codec::primitives::{BodyRequest, DecodeError, DecodedRequest, HeaderRequest};
use kafka_codec::requests::BodyApiVersionRequest;
use std::any::Any;

#[test]
fn decode_request_api_version() -> Result<(), DecodeError> {
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

    use std::io::Cursor;
    let mut buf = Cursor::new(KAFKA_REQUEST_API_VERSIONS);
    let request = DecodedRequest::decode(&mut buf)?;

    assert_eq!(20, request.size);
    assert_eq!(
        HeaderRequest {
            api_key: 18,
            api_version: 2,
            correlation_id: 1,
            client_id: String::from("consumer-1")
        },
        request.header
    );
    assert_eq!(
        BodyRequest::ApiVersions(BodyApiVersionRequest {}),
        request.body
    );

    Ok(())
}
