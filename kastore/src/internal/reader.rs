use crate::SaveString;
use crate::internal::error::{Error, ParseError, ParseErrorKind, ParseSection};
#[derive(Debug, Clone, Copy)]
pub(crate) struct Reader<'a> {
    bytes: &'a [u8],
    offset: usize,
    section: ParseSection,
}

impl<'a> Reader<'a> {
    pub(crate) const MAX_COLLECTION_ITEMS: usize = 1_000_000;
    pub(crate) const MAX_SAVE_STRING_BYTES: usize = 16 * 1024 * 1024;

    pub(crate) fn with_context(bytes: &'a [u8], section: ParseSection) -> Self {
        Self {
            bytes,
            offset: 0,
            section,
        }
    }

    pub(crate) fn set_section(&mut self, section: ParseSection) {
        self.section = section;
    }

    pub(crate) fn position(&self) -> usize {
        self.offset
    }

    pub(crate) fn read_u8(&mut self, field_name: &'static str) -> std::result::Result<u8, Error> {
        let bytes = self.read_bytes(1, field_name)?;
        Ok(bytes[0])
    }

    pub(crate) fn read_byte_as_bool(
        &mut self,
        field_name: &'static str,
    ) -> std::result::Result<bool, Error> {
        Ok(self.read_u8(field_name)? != 0)
    }

    pub(crate) fn read_u16_be(
        &mut self,
        field_name: &'static str,
    ) -> std::result::Result<u16, Error> {
        let bytes = self.read_bytes(2, field_name)?;
        Ok(u16::from_be_bytes([bytes[0], bytes[1]]))
    }

    pub(crate) fn read_i16_be(
        &mut self,
        field_name: &'static str,
    ) -> std::result::Result<i16, Error> {
        let bytes = self.read_bytes(2, field_name)?;
        Ok(i16::from_be_bytes([bytes[0], bytes[1]]))
    }

    pub(crate) fn read_u32_be(
        &mut self,
        field_name: &'static str,
    ) -> std::result::Result<u32, Error> {
        let bytes = self.read_bytes(4, field_name)?;
        Ok(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    pub(crate) fn read_i32_be(
        &mut self,
        field_name: &'static str,
    ) -> std::result::Result<i32, Error> {
        let bytes = self.read_bytes(4, field_name)?;
        Ok(i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    pub(crate) fn read_bytes(
        &mut self,
        len: usize,
        field_name: &'static str,
    ) -> std::result::Result<&'a [u8], Error> {
        let start = self.offset;
        let remaining = self.bytes.len().saturating_sub(start);
        let end = self
            .offset
            .checked_add(len)
            .ok_or_else(|| self.truncated(field_name, start, len, remaining))?;
        let bytes = self
            .bytes
            .get(self.offset..end)
            .ok_or_else(|| self.truncated(field_name, start, len, remaining))?;
        self.offset = end;
        Ok(bytes)
    }

    pub(crate) fn read_len_u32(
        &mut self,
        field_name: &'static str,
    ) -> std::result::Result<usize, Error> {
        self.read_len_u32_with_max(field_name, Self::MAX_COLLECTION_ITEMS)
    }

    pub(crate) fn read_len_u32_with_max(
        &mut self,
        field_name: &'static str,
        max: usize,
    ) -> std::result::Result<usize, Error> {
        let offset = self.offset;
        let len = self.read_u32_be(field_name)?;
        let len = usize::try_from(len)
            .map_err(|_| self.invalid_value(field_name, offset, "count does not fit in usize"))?;

        if len > max {
            return Err(self.invalid_value(field_name, offset, "count exceeds parser limit"));
        }

        Ok(len)
    }

    pub(crate) fn read_vec_u32<T>(
        &mut self,
        count_field: &'static str,
        read_item: impl FnMut(&mut Self) -> std::result::Result<T, Error>,
    ) -> std::result::Result<Vec<T>, Error> {
        self.read_vec_u32_with_max(count_field, Self::MAX_COLLECTION_ITEMS, read_item)
    }

    pub(crate) fn read_vec_u32_with_max<T>(
        &mut self,
        count_field: &'static str,
        max: usize,
        mut read_item: impl FnMut(&mut Self) -> std::result::Result<T, Error>,
    ) -> std::result::Result<Vec<T>, Error> {
        let count = self.read_len_u32_with_max(count_field, max)?;
        let mut values = Vec::with_capacity(count);
        for _ in 0..count {
            values.push(read_item(self)?);
        }

        Ok(values)
    }

    pub(crate) fn read_save_string(
        &mut self,
        field_name: &'static str,
    ) -> std::result::Result<SaveString, Error> {
        let len = self.read_len_u32_with_max(field_name, Self::MAX_SAVE_STRING_BYTES)?;
        Ok(SaveString::from(self.read_bytes(len, field_name)?.to_vec()))
    }

    pub(crate) fn unexpected_value(
        &self,
        field_name: &'static str,
        offset: usize,
        expected: &'static str,
        actual: impl Into<String>,
    ) -> Error {
        self.error_at(
            field_name,
            offset,
            ParseErrorKind::UnexpectedValue {
                expected,
                actual: actual.into(),
            },
        )
    }

    pub(crate) fn invalid_value(
        &self,
        field_name: &'static str,
        offset: usize,
        message: &'static str,
    ) -> Error {
        self.error_at(field_name, offset, ParseErrorKind::InvalidValue { message })
    }

    fn truncated(
        &self,
        field_name: &'static str,
        offset: usize,
        needed: usize,
        remaining: usize,
    ) -> Error {
        self.error_at(
            field_name,
            offset,
            ParseErrorKind::Truncated { needed, remaining },
        )
    }

    fn error_at(&self, field_name: &'static str, offset: usize, kind: ParseErrorKind) -> Error {
        Error::Parse(ParseError {
            section: self.section,
            field: field_name,
            offset,
            kind,
        })
    }
}
