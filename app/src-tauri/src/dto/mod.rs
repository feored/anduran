use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct OpenedSaveDto {
    pub source: SourceDto,
    pub header: HeaderDto,
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct SourceDto {
    pub path: String,
    pub file_name: String,
    pub save_version: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct HeaderDto {
    pub map_name: SaveStringDto,
    pub map_filename: SaveStringDto,
    pub map_description: SaveStringDto,
    pub width: u16,
    pub height: u16,
    pub difficulty: String,
    pub language: String,
    pub game_type: String,
    pub requires_pol: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct SaveStringDto {
    pub text: String,
    pub raw_bytes: Vec<u8>,
    pub valid_utf8: bool,
    pub modified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticDto {
    pub severity: String,
    pub kind: String,
    pub section: String,
    pub field: Option<String>,
    pub offset: Option<usize>,
    pub message: String,
}
