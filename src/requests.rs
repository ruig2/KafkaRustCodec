use crate::primitives::{BodyRequest, DecodeError, FromByte};
use bytes::Buf;

pub struct BodyApiVersionRequest {}
impl BodyRequest for BodyApiVersionRequest {}
impl FromByte for BodyApiVersionRequest {
    fn decode(_: &mut Buf) -> Result<Self, DecodeError> {
        Ok(BodyApiVersionRequest {})
    }
}

pub struct BodyMeatdataApiVersionRequest {}
impl BodyRequest for BodyMeatdataApiVersionRequest {}
impl FromByte for BodyMeatdataApiVersionRequest {
    fn decode(_: &mut Buf) -> Result<Self, DecodeError> {
        Ok(BodyMeatdataApiVersionRequest {})
    }
}
