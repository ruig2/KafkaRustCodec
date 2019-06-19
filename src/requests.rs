use num_traits::FromPrimitive;

use crate::primitives::{ApiKey, DecodeError, HeaderRequest};
use bytes::Buf;

pub trait BodyRequest {}

pub struct DecodedRequest {
    pub size: i32,
    pub header: HeaderRequest,
    pub body: Box<dyn BodyRequest>,
}

pub trait FromByte: Sized {
    fn decode(buf: &mut Buf) -> Result<Self, DecodeError>;
}

pub fn decode_buffer<F: FromByte>(buf: &mut Buf) -> Result<F, DecodeError> {
    FromByte::decode(buf)
}

impl FromByte for i32 {
    fn decode(buf: &mut Buf) -> Result<Self, DecodeError> {
        if buf.remaining() < 4 {
            //Err(DecodeError::BufferUnderflow);
        }
        Ok((buf.get_i32_be()))
    }
}

impl FromByte for i16 {
    fn decode(buf: &mut Buf) -> Result<Self, DecodeError> {
        if buf.remaining() < 4 {
            //Err(DecodeError::BufferUnderflow);
        }
        Ok((buf.get_i16_be()))
    }
}

pub struct BodyApiVersionRequest {}

impl BodyRequest for BodyApiVersionRequest {}
impl FromByte for BodyApiVersionRequest {
    fn decode(buf: &mut Buf) -> Result<Self, DecodeError> {
        Ok((BodyApiVersionRequest {}))
    }
}

pub struct MeatdataApiVersionRequest {}
impl BodyRequest for MeatdataApiVersionRequest {}
impl FromByte for MeatdataApiVersionRequest {
    fn decode(buf: &mut Buf) -> Result<Self, DecodeError> {
        Ok((MeatdataApiVersionRequest {}))
    }
}

impl DecodedRequest {
    pub fn decode(buf: &mut Buf) -> Result<Self, DecodeError> {
        let size: i32 = decode_buffer(buf)?;
        let header: HeaderRequest = decode_buffer(buf)?;

        let body: Box<dyn BodyRequest> = match FromPrimitive::from_i16(header.api_key) {
            Some(ApiKey::ApiVersions) => Box::new(BodyApiVersionRequest::decode(buf)?),
            Some(ApiKey::Metadata) => Box::new(MeatdataApiVersionRequest::decode(buf)?),
            _ => return Err(DecodeError::BadData),
        };

        Ok(DecodedRequest { size, header, body })
    }
}

pub struct Request {}

impl Request {
    pub fn new() -> Self {
        Request {}
    }
}

impl BodyRequest for Request {}
