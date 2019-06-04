use super::requests;

use crate::requests::RequestApiVersions;
use bytes::Buf;
use core::borrow::{Borrow, BorrowMut};
use std::fmt::{Error, Formatter};
use DecodeErrors::BadData;

pub trait Message {
    fn get_size(&self) -> i32;
}

pub trait DecodableMessage: Message + FromByte {}

pub trait Request: DecodableMessage {
    fn get_header(&self) -> &HeaderRequest;
}

pub trait Response: DecodableMessage {}

#[derive(Debug)]
pub enum DecodeErrors {
    BufferUnderflow,
    BufferOverflow,
    BadData,
}

pub trait FromByte: Default {
    type R: Default + FromByte;
    fn decode(&mut self, buf: &mut Buf) -> Result<(), DecodeErrors>;

    fn decode_new(buf: &mut Buf) -> Result<RequestApiVersions, DecodeErrors> {
        //let mut temp: Self = Default::default();
        let mut temp = requests::RequestApiVersions {
            size: 0,
            header: HeaderRequest::default(),
        };
        match temp.decode(buf) {
            Ok(_) => Ok(temp),
            //Err(e) => Err(e),
            Err(e) => Ok(temp),
        }
    }
}

impl FromByte for i8 {
    type R = i8;
    fn decode(&mut self, buf: &mut Buf) -> Result<(), DecodeErrors> {
        *self = buf.get_i8();
        Ok(())
    }
}

impl FromByte for i16 {
    type R = i16;
    fn decode(&mut self, buf: &mut Buf) -> Result<(), DecodeErrors> {
        *self = buf.get_i16_be();
        Ok(())
    }
}

impl FromByte for i32 {
    type R = i32;
    fn decode(&mut self, buf: &mut Buf) -> Result<(), DecodeErrors> {
        *self = buf.get_i32_be();
        Ok(())
    }
}

impl FromByte for String {
    type R = String;

    fn decode(&mut self, buf: &mut Buf) -> Result<(), DecodeErrors> {
        let mut length: i16 = 0;
        length.decode(buf);
        if length == 0 || length == -1 {
            return Ok(());
        }
        if length < -1 {
            // ToDo: throw error
            return Ok(());
        }
        self.reserve(length as usize);
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

impl std::fmt::Display for HeaderRequest {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        println!(
            "api_key: {}, api_version: {}, correlation_id: {}, client_id: {}",
            self.api_key, self.api_version, self.correlation_id, self.client_id
        );
        Ok(())
    }
}

impl HeaderRequest {
    fn new(
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

impl Default for HeaderRequest {
    fn default() -> HeaderRequest {
        Self::new(-1, -1, -1, String::new())
    }
}

impl FromByte for HeaderRequest {
    type R = HeaderRequest;

    fn decode(&mut self, buf: &mut Buf) -> Result<(), DecodeErrors> {
        self.api_key.decode(buf);
        self.api_version.decode(buf);
        self.correlation_id.decode(buf);
        self.client_id.decode(buf);
        Ok(())
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

enum ApiKeys {
    Metadata = 3,
    ApiVersions = 18,
}

pub enum ApiVersions {
    Version0,
    Version1,
    Version2,
}
