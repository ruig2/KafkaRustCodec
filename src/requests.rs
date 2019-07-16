use crate::primitives::{
    decode_buffer, ApiVersion, BodyRequest, DecodeError, FromByte, FromByteWithVersion,
};
use bytes::Buf;

#[derive(PartialEq, Debug)]
pub struct BodyApiVersionRequest {}
impl FromByteWithVersion for BodyApiVersionRequest {
    fn decode_with_version(_: &mut Buf, _: i16) -> Result<Self, DecodeError> {
        Ok(BodyApiVersionRequest {})
    }
}

#[derive(PartialEq, Debug)]
pub struct BodyMetadataRequest {
    pub topics: Vec<String>,
    pub allow_auto_topic_creation: bool,
}
impl FromByteWithVersion for BodyMetadataRequest {
    fn decode_with_version(buf: &mut Buf, api_version: i16) -> Result<Self, DecodeError> {
        if api_version <= 3 {
            Ok(BodyMetadataRequest {
                topics: decode_buffer(buf)?,
                allow_auto_topic_creation: false,
            })
        } else {
            Ok(BodyMetadataRequest {
                topics: decode_buffer(buf)?,
                allow_auto_topic_creation: decode_buffer(buf)?,
            })
        }
    }
}
