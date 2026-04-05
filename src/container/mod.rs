mod r10032;

use crate::Error;
use crate::SaveVersion;
use crate::version::ContainerRevision;

pub(crate) fn decode_container(
    revision: ContainerRevision,
    bytes: &[u8],
) -> std::result::Result<DecodedContainer, Error> {
    match revision {
        ContainerRevision::R10032 => r10032::decode(bytes),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ContainerHeader {
    pub requires_pol: bool,
    pub map_file_info: MapFileInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MapFileInfo {
    pub filename: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DecodedContainer {
    pub save_version: SaveVersion,
    pub header: ContainerHeader,
    pub payload: Vec<u8>,
}

#[cfg(test)]
mod tests;
