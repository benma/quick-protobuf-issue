// Automatically generated rust module for 'a.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct A {
    pub common: Option<b::Common>,
    pub b: Option<b::B>,
}

impl<'a> MessageRead<'a> for A {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.common = Some(r.read_message::<b::Common>(bytes)?),
                Ok(18) => msg.b = Some(r.read_message::<b::B>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for A {
    fn get_size(&self) -> usize {
        0
        + self.common.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.b.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.common { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.b { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

