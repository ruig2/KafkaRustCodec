use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::requests::{BodyApiVersionRequest, BodyMeatdataApiVersionRequest};
use bytes::Buf;
use std::fmt::{Error, Formatter};

#[derive(FromPrimitive)]
pub enum ApiKey {
    Metadata = 3,
    ApiVersions = 18,
}

pub enum ApiVersion {
    Version0,
    Version1,
    Version2,
}

#[derive(Debug)]
pub enum DecodeError {
    BufferUnderflow,
    BufferOverflow,
    BadData,
}

#[derive(Debug, PartialEq)]
pub struct HeaderRequest {
    pub api_key: i16,
    pub api_version: i16,
    pub correlation_id: i32,
    pub client_id: String,
}

impl std::fmt::Display for HeaderRequest {
    fn fmt(&self, _: &mut Formatter) -> Result<(), Error> {
        println!(
            "api_key: {}, api_version: {}, correlation_id: {}, client_id: {}",
            self.api_key, self.api_version, self.correlation_id, self.client_id
        );
        Ok(())
    }
}

impl FromByte for HeaderRequest {
    fn decode(buf: &mut Buf) -> Result<Self, DecodeError> {
        Ok(HeaderRequest {
            api_key: decode_buffer(buf)?,
            api_version: decode_buffer(buf)?,
            correlation_id: decode_buffer(buf)?,
            client_id: decode_buffer(buf)?,
        })
    }
}

#[derive(Debug)]
pub struct HeaderResponse {
    pub correlation_id: i32,
}

pub trait BodyRequest {}
pub struct DecodedRequest {
    pub size: i32,
    pub header: HeaderRequest,
    pub body: Box<dyn BodyRequest>,
}

impl DecodedRequest {
    pub fn decode(buf: &mut Buf) -> Result<Self, DecodeError> {
        let size: i32 = decode_buffer(buf)?;
        let header: HeaderRequest = decode_buffer(buf)?;

        let body: Box<dyn BodyRequest> = match FromPrimitive::from_i16(header.api_key) {
            Some(ApiKey::ApiVersions) => Box::new(BodyApiVersionRequest::decode(buf)?),
            Some(ApiKey::Metadata) => Box::new(BodyMeatdataApiVersionRequest::decode(buf)?),
            _ => return Err(DecodeError::BadData),
        };

        Ok(DecodedRequest { size, header, body })
    }
}

pub trait FromByte: Sized {
    fn decode(buf: &mut Buf) -> Result<Self, DecodeError>;
}

impl FromByte for i32 {
    fn decode(buf: &mut Buf) -> Result<Self, DecodeError> {
        if buf.remaining() < 4 {
            //Err(DecodeError::BufferUnderflow);
        }
        Ok(buf.get_i32_be())
    }
}

impl FromByte for i16 {
    fn decode(buf: &mut Buf) -> Result<Self, DecodeError> {
        if buf.remaining() < 4 {
            //Err(DecodeError::BufferUnderflow);
        }
        Ok(buf.get_i16_be())
    }
}

impl FromByte for String {
    fn decode(buf: &mut Buf) -> Result<Self, DecodeError> {
        let length: i16 = decode_buffer(buf)?;
        if length == 0 || length == -1 {
            return Ok(String::from(""));
        }
        // Length should not be smaller than -1 according to the Kafka protocol
        if length < -1 {
            // ToDo: throw error
            // return Ok(());
        }
        // Any smart way to read a String from buffer?
        let mut temp_vec: Vec<u8> = vec![0; length as usize];
        buf.copy_to_slice(temp_vec.as_mut_slice());
        // ToDo: throw error

        let result: String = String::from_utf8(temp_vec).unwrap();

        if result.len() != length as usize {
            // ToDo: throw error
            //return Ok(());
        }
        Ok(result)
    }
}

pub fn decode_buffer<F: FromByte>(buf: &mut Buf) -> Result<F, DecodeError> {
    FromByte::decode(buf)
}
