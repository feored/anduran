use crate::Error;
use crate::internal::save_string::SaveString;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Writer {
    bytes: Vec<u8>,
}

impl Writer {
    pub(crate) fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    pub(crate) fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    pub(crate) fn write_u8(&mut self, value: u8) {
        self.bytes.push(value);
    }

    pub(crate) fn write_u16_be(&mut self, value: u16) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    pub(crate) fn write_i16_be(&mut self, value: i16) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    pub(crate) fn write_u32_be(&mut self, value: u32) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    pub(crate) fn write_i32_be(&mut self, value: i32) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    pub(crate) fn write_bytes(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes);
    }

    pub(crate) fn write_len_u32(
        &mut self,
        len: usize,
        field: &'static str,
    ) -> std::result::Result<(), Error> {
        let len = u32::try_from(len).map_err(|_| Error::InvalidModel {
            field,
            message: "count must fit in u32",
        })?;
        self.write_u32_be(len);
        Ok(())
    }

    pub(crate) fn write_save_string(
        &mut self,
        value: &SaveString,
        field: &'static str,
    ) -> std::result::Result<(), Error> {
        let as_bytes = value.as_bytes();
        self.write_len_u32(as_bytes.len(), field)?;
        self.write_bytes(as_bytes);
        Ok(())
    }

    pub(crate) fn write_byte_from_bool(&mut self, value: bool) {
        self.write_u8(if value { 1 } else { 0 });
    }
}
