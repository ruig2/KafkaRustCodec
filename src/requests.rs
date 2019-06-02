/*
use bytes::Buf;

#[derive(Debug)]
pub struct RequestApiVersions<'a> {
    size: i32,
    header: HeaderRequest<'a>,
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
    fn decode(&mut self, buf: &mut Buf) -> Result<(), DecodeErrors> {
        self.size.decode(buf);
        self.header.decode(buf);
        Ok(())
    }
}
*/
