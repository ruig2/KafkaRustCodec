use super::requests;

use crate::requests::{decode_buffer, RequestApiVersions, TempFromByte};
use bytes::Buf;
use core::borrow::{Borrow, BorrowMut};
use std::fmt::{Error, Formatter};
use DecodeError::BadData;

enum ApiKey {
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

pub trait Message {
    fn get_size(&self) -> i32;
}

pub trait FromByte: Default {
    fn decode(&mut self, buf: &mut Buf) -> Result<(), DecodeError>;

    fn decode_new(buf: &mut Buf) -> Result<Self, DecodeError> {
        let mut temp = Self::default();
        match temp.decode(buf) {
            Ok(_) => Ok(temp),
            Err(e) => Err(e),
        }
    }
}

pub trait DecodableMessage: Message + FromByte {}

pub trait Request: DecodableMessage {
    fn get_header(&self) -> &HeaderRequest;
}

// Skip for now
pub trait Response: DecodableMessage {}

impl FromByte for i8 {
    fn decode(&mut self, buf: &mut Buf) -> Result<(), DecodeError> {
        *self = buf.get_i8();
        Ok(())
    }
}

impl FromByte for i16 {
    fn decode(&mut self, buf: &mut Buf) -> Result<(), DecodeError> {
        *self = buf.get_i16_be();
        Ok(())
    }
}

impl FromByte for i32 {
    fn decode(&mut self, buf: &mut Buf) -> Result<(), DecodeError> {
        *self = buf.get_i32_be();
        Ok(())
    }
}

impl FromByte for String {
    fn decode(&mut self, buf: &mut Buf) -> Result<(), DecodeError> {
        let mut length: i16 = 0;
        length.decode(buf);
        if length == 0 || length == -1 {
            return Ok(());
        }
        // Length should not be smaller than -1 according to the Kafka protocol
        if length < -1 {
            // ToDo: throw error
            return Ok(());
        }
        self.reserve(length as usize);
        // Any smart way to read a String from buffer?
        let mut temp_vec: Vec<u8> = vec![0; length as usize];
        buf.copy_to_slice(temp_vec.as_mut_slice());
        // ToDo: throw error
        *self = String::from_utf8_lossy(temp_vec.as_ref()).parse().unwrap();

        if self.len() != length as usize {
            // ToDo: throw error
            return Ok(());
        }
        Ok(())
    }
}

// Reference: https://github.com/spicavigo/kafka-rust/blob/c58cf5f30b35fad6ab163416d51d2b99a30da9c2/src/protocol/mod.rs#L108
#[derive(Debug)]
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

impl Default for HeaderRequest {
    fn default() -> HeaderRequest {
        Self::new(-1, -1, -1, String::new())
    }
}

impl FromByte for HeaderRequest {
    fn decode(&mut self, buf: &mut Buf) -> Result<(), DecodeError> {
        // ToDo: throw error if fails to decode
        self.api_key.decode(buf);
        self.api_version.decode(buf);
        self.correlation_id.decode(buf);
        self.client_id.decode(buf);
        Ok(())
    }
}

impl TempFromByte for String {
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

impl TempFromByte for HeaderRequest {
    fn decode(buf: &mut Buf) -> Result<Self, DecodeError> {
        Ok(HeaderRequest {
            api_key: decode_buffer(buf)?,
            api_version: decode_buffer(buf)?,
            correlation_id: decode_buffer(buf)?,
            client_id: decode_buffer(buf)?,
        })
    }
}

#[derive(Default, Debug, Clone)]
pub struct HeaderResponse {
    pub correlation_id: i32,
}

impl HeaderResponse {
    fn new(correlation_id: i32) -> HeaderResponse {
        HeaderResponse { correlation_id }
    }
}
