use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::primitives::{
    ApiKey, DecodableMessage, DecodeError, FromByte, HeaderRequest, Message, Request,
};
use bytes::Buf;
use std::fmt::{Error, Formatter};

#[derive(Debug)]
// Kafka request to get the Api Version information of the Kafka broker
pub struct RequestApiVersions {
    // ToDo: move size and header out of the struct
    // so that we can decode according to the header.api_key dynamically
    // Maybe create struct for message body only (without size and header)?

    // I'd like to include size in the requests because we may need information such as the
    // average size of requests and show that on the dashboard
    pub size: i32,
    pub header: HeaderRequest,
}

impl Message for RequestApiVersions {
    // Since self.size (an variable rather than a method of Message) is used, we cannot move the
    // implementation of get_size() to the Message trait, and we have to copy this get_size
    // implementation for all the Kafka requests, right?
    fn get_size(&self) -> i32 {
        self.size
    }
}

impl Default for RequestApiVersions {
    fn default() -> Self {
        RequestApiVersions {
            size: 0,
            header: HeaderRequest::default(),
        }
    }
}

impl FromByte for RequestApiVersions {
    // ToDo: handle decode errors accordingly
    fn decode(&mut self, buf: &mut Buf) -> Result<(), DecodeError> {
        // ToDo: Maybe move size and header out of the struct and out of decode()
        // so that we can decode dynamically according to header.api_key
        self.size.decode(buf);
        self.header.decode(buf);
        Ok(())
    }
}

impl DecodableMessage for RequestApiVersions {}

impl Request for RequestApiVersions {
    fn get_header(&self) -> &HeaderRequest {
        &self.header
    }
}

pub trait BodyRequest {}

pub struct DecodedRequest {
    pub size: i32,
    pub header: HeaderRequest,
    pub body: Box<dyn BodyRequest>,
}

pub trait TempFromByte: Sized {
    fn decode(buf: &mut Buf) -> Result<Self, DecodeError>;
}

pub fn decode_buffer<F: TempFromByte>(buf: &mut Buf) -> Result<F, DecodeError> {
    TempFromByte::decode(buf)
}

impl TempFromByte for i32 {
    fn decode(buf: &mut Buf) -> Result<Self, DecodeError> {
        if buf.remaining() < 4 {
            //Err(DecodeError::BufferUnderflow);
        }
        Ok((buf.get_i32_be()))
    }
}

impl TempFromByte for i16 {
    fn decode(buf: &mut Buf) -> Result<Self, DecodeError> {
        if buf.remaining() < 4 {
            //Err(DecodeError::BufferUnderflow);
        }
        Ok((buf.get_i16_be()))
    }
}

pub struct BodyApiVersionRequest {}

impl BodyRequest for BodyApiVersionRequest {}
impl TempFromByte for BodyApiVersionRequest {
    fn decode(buf: &mut Buf) -> Result<Self, DecodeError> {
        Ok((BodyApiVersionRequest {}))
    }
}

pub struct MeatdataApiVersionRequest {}
impl BodyRequest for MeatdataApiVersionRequest {}
impl TempFromByte for MeatdataApiVersionRequest {
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

pub struct tempRequest {}

impl tempRequest {
    pub fn new() -> Self {
        tempRequest {}
    }
}

impl BodyRequest for tempRequest {}
