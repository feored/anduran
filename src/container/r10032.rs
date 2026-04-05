use super::{ContainerHeader, DecodedContainer};
use crate::Error;
use crate::internal::reader::Reader;
use crate::version::SaveVersion;

pub(crate) const MAGIC_NUMBER: u16 = 0xFF03;
pub(crate) const REQUIRES_POL: u16 = 0x4000;

pub(crate) fn decode(bytes: &[u8]) -> std::result::Result<DecodedContainer, Error> {
    let mut reader = Reader::new(bytes);
    let magic_number = reader.read_u16_be("magic number")?;

    if magic_number != MAGIC_NUMBER {
        return Err(Error::InvalidContainer("unexpected magic number"));
    }

    let _save_version_string = reader.read_string("save version string")?;
    let save_version_number = reader.read_u16_be("save version")?;

    let save_version = match save_version_number {
        10032 => SaveVersion::V10032,
        _ => return Err(Error::UnsupportedSaveVersion),
    };

    let requires_pol = (reader.read_u16_be("flags")? & REQUIRES_POL) != 0;

    let map_file_info = crate::container::MapFileInfo {
        filename: reader.read_string("map filename")?,
        name: reader.read_string("map name")?,
        description: reader.read_string("map description")?,
    };

    Ok(DecodedContainer {
        save_version,
        header: ContainerHeader {
            requires_pol,
            map_file_info,
        },
        payload: reader.remaining().to_vec(),
    })
}
