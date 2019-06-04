use super::requests;

use crate::requests::RequestApiVersions;
use bytes::Buf;
use core::borrow::Borrow;
use std::fmt::{Error, Formatter};
use std::ptr::null;
use DecodeErrors::BadData;

pub trait Message {
    fn get_size(&self) -> i32;
}

pub trait DecodableMessage: Message + FromByte {}

pub trait Request: DecodableMessage {
    fn get_header(&self) -> &HeaderRequest;
}

pub trait Response: DecodableMessage {}

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

impl FromByte for i32 {
    type R = i32;
    fn decode(&mut self, buf: &mut Buf) -> Result<(), DecodeErrors> {
        *self = buf.get_i32_be();
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

impl FromByte for i8 {
    type R = i8;
    fn decode(&mut self, buf: &mut Buf) -> Result<(), DecodeErrors> {
        *self = buf.get_i8();
        Ok(())
    }
}

/*
pub trait FromByte {
    type R: Default + FromByte;

    fn decode<T>(&mut self, buffer: &mut T) -> Result<(), ()>;
    fn decode_new<T>(buffer: &mut T) -> Result<Self::R, ()> {
        let mut temp: Self::R = Default::default();
        match temp.decode(buffer) {
            Ok(_) => Ok(temp),
            Err(e) => Err(e),
        }
    }
}
*/

///////////////////////////////

#[derive(Debug)]
pub struct NullableString<'a> {
    pub length: i16,
    pub content: &'a String,
}

impl<'a> std::fmt::Display for NullableString<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        print!("len: {}, str: {}", self.length, self.content.to_string());
        Ok(())
    }
}

const nullableStringDefault: NullableString = NullableString {
    length: -1,
    content: String { vec: vec![] }.borrow(),
};

impl<'a> Default for NullableString<'a> {
    fn default() -> Self {
        nullableStringDefault
    }
}

impl<'a> FromByte for NullableString<'a> {
    type R = NullableString<'a>;

    fn decode(&mut self, buf: &mut Buf) -> Result<(), DecodeErrors> {
        self.length.decode(buf);
        println!("decoded {} ", self.length);
        /*
        if self.length < -1 {
            return Err(BadData);
        } else if self.length == 0 {
            self.content = "";
        } else {
            let mut bytes = vec![0; self.length as usize];
            buf.copy_to_slice(&mut bytes);
            self.content = std::str::from_utf8(&bytes).unwrap();
        }
        */
        self.content = &String::from("error");
        Ok(())
    }
}

// Reference: https://github.com/spicavigo/kafka-rust/blob/c58cf5f30b35fad6ab163416d51d2b99a30da9c2/src/protocol/mod.rs#L108
#[derive(Debug)]
pub struct HeaderRequest<'a> {
    pub api_key: i16,
    pub api_version: i16,
    pub correlation_id: i32,
    pub client_id: &'a mut NullableString<'a>,
}

impl<'a> std::fmt::Display for HeaderRequest<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        println!(
            "api_key: {}, api_version: {}, correlation_id: {}, client_id: {}",
            self.api_key, self.api_version, self.correlation_id, self.client_id
        );
        Ok(())
    }
}

impl<'a> HeaderRequest<'a> {
    fn new(
        api_key: i16,
        api_version: i16,
        correlation_id: i32,
        client_id: &'a mut NullableString<'a>,
    ) -> HeaderRequest<'a> {
        HeaderRequest {
            api_key,
            api_version,
            correlation_id,
            client_id,
        }
    }
}

impl<'a> Default for HeaderRequest<'a> {
    fn default() -> HeaderRequest<'a> {
        Self::new(-1, -1, -1, &mut nullableStringDefault)
    }
}

impl<'a> FromByte for HeaderRequest<'a> {
    type R = HeaderRequest<'a>;

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
