use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::Write;
use std::iter::FromIterator;
use std::str;
use std::sync::Arc;

use bytes::{Bytes, BytesMut, BufMut, Buf};

pub struct TlvBox {
    m_objects: HashMap<i32, Bytes>,
    m_total_bytes: usize,
}

impl TlvBox {
    pub fn new() -> TlvBox {
        TlvBox {
            m_objects: HashMap::new(),
            m_total_bytes: 0,
        }
    }

    pub fn putShortValue(&mut self, typ: i32, value: i16) {
        let mut buf = Vec::with_capacity(2);
        buf.put_i16(value);
        self.putBytesValue(typ, Bytes::from(buf))
    }
    pub fn putIntValue(&self, typ: i32, value: i32) {
        unimplemented!()
    }
    pub fn putLongValue(&self, typ: i32, value: i64) {
        unimplemented!()
    }
    pub fn putFloatValue(&self, typ: i32, value: f32) {
        unimplemented!()
    }
    pub fn putDoubleValue(&self, typ: i32, value: f64) {
        unimplemented!()
    }
    pub fn putStringValue(&mut self, typ: i32, value: String) {
        let len = value.clone().len();
        let mut byts = BytesMut::with_capacity(len);
        write!(byts, "{}", &value).unwrap();

        self.putBytesValue(typ, byts.freeze());
    }
    pub fn putObjectValue(&self, typ: i32, value: TlvBox) {
        unimplemented!()
    }
    pub fn putBytesValue(&mut self, typ: i32, value: Bytes) {
        self.m_objects.insert(typ, value.clone());
        self.m_total_bytes += value.len() + 8;
    }
    pub fn serialize() -> Arc<BytesMut> {
        unimplemented!()
    }

    pub fn parse(&self, buffer: Arc<BytesMut>, offset: i32, length: i32) {}
    pub fn getBytesValue(&self, typ: i32) -> Option<Bytes> {
        let bytes = self.m_objects.get(typ.clone().borrow());
        match bytes {
            None => {
                None
            },
            Some(byts) => {
                Some(byts.clone())
            }
        }
    }
    pub fn getShortValue(&self, typ: i32) -> Option<i16> {
        let mut bytes = self.m_objects.get(typ.clone().borrow());
        match bytes {
            Some(x) => {
                Some(x.clone().get_i16())
            },
            None => None,
        }
    }
    pub fn getIntValue(&self, typ: i32) -> i32 {
        unimplemented!()
    }
    pub fn getLongValue(&self, typ: i32) -> i64 {
        unimplemented!()
    }
    pub fn getFloatValue(&self, typ: i32) -> f32 {
        unimplemented!()
    }
    pub fn getDoubleValue(&self, typ: i32) -> f64 {
        unimplemented!()
    }
    pub fn getStringValue(&self, typ: i32) -> String {
        let value = self.getBytesValue(typ);
        let buf = value.clone().unwrap().to_vec();
        let s = match str::from_utf8(&buf) {
            Ok(v) => {
                v
            },
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        String::from(s)
    }
    pub fn getObjectValue(&self, typ: i32) -> TlvBox {
        unimplemented!()
    }
}


#[cfg(test)]
mod tests {
    use std::str;

    use bytes::{BufMut, Bytes, BytesMut};

    use crate::tlv::tlvbox::TlvBox;

    #[test]
    fn test_parse_bytes() {
        let mut tlv_box = TlvBox::new();

        let mut buf = BytesMut::with_capacity(1024);
        buf.put(&b"hello world"[..]);
        tlv_box.putBytesValue(01, buf.clone().freeze());
        let value = tlv_box.getBytesValue(01);

        assert_eq!(11, value.clone().unwrap().len());
        assert_eq!(&b"hello world"[..], value.clone().unwrap());
    }

    #[test]
    fn test_parse_strings() {
        let mut tlv_box = TlvBox::new();
        tlv_box.putStringValue(01, String::from("hello, world"));
        let value = tlv_box.getBytesValue(01);

        assert_eq!(12, value.clone().unwrap().len());

        let a = format!("{:?}", Bytes::from(value.clone().unwrap()));

        assert_eq!("b\"hello, world\"", a);
        assert_eq!(1, tlv_box.m_objects.len());
        assert_eq!(20, tlv_box.m_total_bytes);

        assert_eq!("hello, world", tlv_box.getStringValue(01));
    }

    #[test]
    fn test_covert_short() {
        let mut tlv_box = TlvBox::new();
        tlv_box.putShortValue(01, 12);
        assert_eq!(12, tlv_box.getShortValue(01).unwrap());
    }
}
