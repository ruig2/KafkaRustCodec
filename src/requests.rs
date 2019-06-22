use crate::primitives::{BodyRequest, DecodeError, FromByte};
use bytes::Buf;

#[derive(PartialEq, Debug)]
pub struct BodyApiVersionRequest {}
impl FromByte for BodyApiVersionRequest {
    fn decode(_: &mut Buf) -> Result<Self, DecodeError> {
        Ok(BodyApiVersionRequest {})
    }
}

#[derive(PartialEq, Debug)]
pub struct BodyMetadataRequest {}
impl FromByte for BodyMetadataRequest {
    fn decode(_: &mut Buf) -> Result<Self, DecodeError> {
        Ok(BodyMetadataRequest {})
    }
}
