use super::requests;

use crate::requests::RequestApiVersions;
use bytes::Buf;
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

///////////////////////////////
pub enum ApiVersions {
    Version0,
    Version1,
    Version2,
}

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
    pub content: &'a str,
}

//impl<'a> NullableString<'a> {
//    pub fn new(content: &'a str) -> Self {
//        NullableString {
//            length: str.length,
//            content: str,
//        }
//    }
//}

impl<'a> Default for NullableString<'a> {
    fn default() -> NullableString<'static> {
        NullableString {
            length: -1,
            content: "",
        }
    }
}

impl<'a> FromByte for NullableString<'a> {
    type R = NullableString<'a>;

    fn decode(&mut self, buf: &mut Buf) -> Result<(), DecodeErrors> {
        self.length.decode(buf);
        if self.length < -1 {
            return Err(BadData);
        } else if self.length == 0 {
            self.content = "";
        } else {
            /*
            let mut bytes = vec![0; self.length as usize];
            buf.copy_to_slice(&mut bytes);
            self.content = match std::str::from_utf8(&bytes) {
                Ok(s) => s,
                Err(e) => return Err(BadData),
            }
            */
            self.content = "skip read";
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
    //pub client_id: &'a NullableString<'a>,
}

impl std::fmt::Display for HeaderRequest {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        println!(
            "api_key: {}, api_version: {}, correlation_id: {}",
            self.api_key, self.api_version, self.correlation_id,
        );
        Ok(())
    }
}

impl HeaderRequest {
    fn new(
        api_key: i16,
        api_version: i16,
        correlation_id: i32,
        //client_id: &'a NullableString,
    ) -> HeaderRequest {
        HeaderRequest {
            api_key,
            api_version,
            correlation_id,
            //client_id,
        }
    }
}

impl Default for HeaderRequest {
    fn default() -> Self {
        //Self::new(-1, -1, -1, &NullableString::default())
        //let null_str_default = NullableString::default();
        //Self::new(-1, -1, -1, &null_str_default)
        Self::new(-1, -1, -1)
    }
}

impl FromByte for HeaderRequest {
    type R = HeaderRequest;

    fn decode(&mut self, buf: &mut Buf) -> Result<(), DecodeErrors> {
        self.api_key.decode(buf);
        self.api_version.decode(buf);
        self.correlation_id.decode(buf);
        //self.client_id.decode(buf);
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

//pub fn hello() {
//    println!("hello");
//}

//fn main() {
//    // The statements here will be executed when the compiled binary is called
//
//    // Print text to the console
//    hello();
//}

enum ApiKeys {
    Metadata = 3,
    ApiVersions = 18,
}
