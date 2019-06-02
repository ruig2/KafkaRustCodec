pub mod primitives;
pub mod requests;

fn main() {
    /*
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
        0, 10, 99, 111, 110, 115, 117, 109, 101, 114, 45, 49,
    ]; // value: consumer-1
    // Just echo to make things easier for now
    const KAFKA_RESPONSE_API_VERSIONS: &[u8] = KAFKA_REQUEST_API_VERSIONS;

    use bytes::Buf;
    use std::io::Cursor;

    let mut buf = Cursor::new(KAFKA_REQUEST_API_VERSIONS);
    let req = primitives::Request::decode_new(&mut buf);

    println!("Hello, world!");
    */
}

