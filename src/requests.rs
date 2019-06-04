use crate::primitives::{
    DecodableMessage, DecodeErrors, FromByte, HeaderRequest, Message, Request,
};
use bytes::Buf;
use std::fmt::{Error, Formatter};

#[derive(Debug)]
pub struct RequestApiVersions {
    pub size: i32,
    pub header: HeaderRequest,
}

impl Message for RequestApiVersions {
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
    type R = RequestApiVersions;
    fn decode(&mut self, buf: &mut Buf) -> Result<(), DecodeErrors> {
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

#[derive(Debug)]
pub struct RequestApiVersions_2 {
    pub size: i32,
    pub header: HeaderRequest,
}

impl Message for RequestApiVersions_2 {
    fn get_size(&self) -> i32 {
        self.size
    }
}

impl Default for RequestApiVersions_2 {
    fn default() -> Self {
        RequestApiVersions_2 {
            size: 0,
            header: HeaderRequest::default(),
        }
    }
}

impl FromByte for RequestApiVersions_2 {
    type R = RequestApiVersions_2;
    fn decode(&mut self, buf: &mut Buf) -> Result<(), DecodeErrors> {
        self.size.decode(buf);
        self.header.decode(buf);
        Ok(())
    }
}

impl DecodableMessage for RequestApiVersions_2 {}

impl Request for RequestApiVersions_2 {
    fn get_header(&self) -> &HeaderRequest {
        &self.header
    }
}
