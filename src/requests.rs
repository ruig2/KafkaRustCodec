use crate::primitives::{DecodeErrors, FromByte, HeaderRequest, Message};
use bytes::Buf;
use std::fmt::{Error, Formatter};

#[derive(Debug)]
pub struct RequestApiVersions<'a> {
    pub size: i32,
    pub header: HeaderRequest<'a>,
}

impl<'a> Message for RequestApiVersions<'a> {
    fn get_size(&self) -> i32 {
        self.size
    }
}

impl<'a> Default for RequestApiVersions<'a> {
    fn default() -> Self {
        RequestApiVersions {
            size: 0,
            header: HeaderRequest::default(),
        }
    }
}

impl<'a> FromByte for RequestApiVersions<'a> {
    type R = RequestApiVersions<'a>;
    fn decode(&mut self, buf: &mut Buf) -> Result<(), DecodeErrors> {
        self.size.decode(buf);
        self.header.decode(buf);
        Ok(())
    }
}
