use bytes::BytesMut;
use std::sync::Arc;

struct TlvBox {}

impl TlvBox {
    pub fn TlvBox() -> TlvBox {
        TlvBox {}
    }

    pub fn putByteValue(&self, typ: i32, value: BytesMut) {}
    pub fn putShortValue(&self, typ: i32, value: i16) {}
    pub fn putIntValue(&self, typ: i32, value: i32) {}
    pub fn putLongValue(&self, typ: i32, value: i64) {}
    pub fn putFloatValue(&self, typ: i32, value: f32) {}
    pub fn putDoubleValue(&self, typ: i32, value: f64) {}
    pub fn putStringValue(&self, typ: i32, value: String) {}
    pub fn putObjectValue(&self, typ: i32, value: TlvBox) {}
    pub fn putBytesValue(&self, typ: i32, value: Arc<BytesMut>) {}
    pub fn serialize() -> Arc<BytesMut> {}

    pub fn parse(&self, buffer: Arc<BytesMut>, offset: int, length: int) {}
    pub fn getByteValue(typ: int) -> BytesMut {}
    pub fn getShortValue(typ: int) -> i16 {}
    pub fn getIntValue(typ: int) -> i32 {}
    pub fn getLongValue(typ: int) -> i64 {}
    pub fn getFloatValue(typ: int) -> f32 {}
    pub fn getDoubleValue(typ: int) -> f64 {}
    pub fn getStringValue(typ: int) -> String {}
    pub fn getObjectValue(typ: int) -> TlvBox {}
    pub fn getBytesValue(typ: int) -> Arc<BytesMut> {}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_byte() {

    }
}
