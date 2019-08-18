use crate::primitives::{
    decode_buffer, ApiVersion, BodyRequest, DecodeError, FromByte, FromByteWithVersion,
};
use bytes::Buf;

#[derive(PartialEq, Debug)]
pub struct BodyUnsupportedResponse {}
impl FromByteWithVersion for BodyUnsupportedResponse {
    fn decode_with_version(_: &mut Buf, _: i16) -> Result<Self, DecodeError> {
        Ok(BodyUnsupportedResponse {})
    }
}
