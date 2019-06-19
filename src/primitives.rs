use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::requests::{decode_buffer, FromByte};
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

impl HeaderRequest {
    pub fn new(
        api_key: i16,
        api_version: i16,
        correlation_id: i32,
        client_id: String,
    ) -> HeaderRequest {
        HeaderRequest {
            api_key,
            api_version,
            correlation_id,
            client_id,
        }
    }
}

impl std::fmt::Display for HeaderRequest {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        println!(
            "api_key: {}, api_version: {}, correlation_id: {}, client_id: {}",
            self.api_key, self.api_version, self.correlation_id, self.client_id
        );
        Ok(())
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

#[derive(Debug, Clone)]
pub struct HeaderResponse {
    pub correlation_id: i32,
}

impl HeaderResponse {
    fn new(correlation_id: i32) -> HeaderResponse {
        HeaderResponse { correlation_id }
    }
}
