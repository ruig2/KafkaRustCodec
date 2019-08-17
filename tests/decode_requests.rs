use kafka_codec::primitives::{BodyRequest, DecodeError, DecodedRequest, HeaderRequest};
use kafka_codec::requests::{BodyApiVersionRequest, BodyMetadataRequest};

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

    let request = DecodedRequest::decode(KAFKA_REQUEST_API_VERSIONS)?;

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

#[test]
fn decode_request_metadata() -> Result<(), DecodeError> {
    const KAFKA_REQUEST_METADATA: &[u8] = &[
        // Request/response Size => INT32
        0, 0, 0, 24, // value: 24
        // api_key => INT16
        0, 3, // value: 3
        // api_version => INT16
        0, 2, // value: 2
        // correlation_id => INT32
        0, 0, 0, 1, // value: 1
        // client_id => NULLABLE_STRING
        0, 10, 99, 111, 110, 115, 117, 109, 101, 114, 45, 49, // len: 10, value: consumer-1
        // topics => array of STRING
        0, 0, 0, 2, // length of array: 2
        0, 2, 72, 105, // value: "Hi"
        0, 2, 79, 107, // value: "Ok"
    ];

    use std::io::Cursor;
    let mut buf = Cursor::new(KAFKA_REQUEST_METADATA);
    let request = DecodedRequest::decode(&mut buf)?;

    assert_eq!(24, request.size);
    assert_eq!(
        HeaderRequest {
            api_key: 3,
            api_version: 2,
            correlation_id: 1,
            client_id: String::from("consumer-1")
        },
        request.header
    );
    assert_eq!(
        BodyRequest::Metadata(BodyMetadataRequest {
            // ToDo: If we make 'topics' field private, then it cannot be tested here,
            // how to fix this elegantly?
            topics: String::from("Hi"),
            allow_auto_topic_creation: false
        }),
        request.body
    );

    Ok(())
}
