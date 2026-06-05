use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct OpenedSaveDto {
    pub source: SourceDto,
    pub scenario: ScenarioDto,
    pub diagnostics: Vec<DiagnosticDto>,
    pub dirty: bool,
    pub revision: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct ScenarioMutationResultDto {
    pub scenario: ScenarioDto,
    pub dirty: bool,
    pub revision: u64,
    pub validation: ValidationResultDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct ValidationResultDto {
    pub revision: u64,
    pub issues: Vec<ValidationIssueDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct ValidationIssueDto {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct LibraryDto {
    pub entries: Vec<LibrarySaveEntryDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct LibrarySaveEntryDto {
    pub path: String,
    pub file_name: String,
    pub size_bytes: u64,
    pub modified_timestamp: Option<u64>,
    pub status: String,
    pub summary: Option<SaveSummaryDto>,
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct SaveSummaryDto {
    pub save_version: u16,
    pub map_name: SaveStringDto,
    pub map_file_name: SaveStringDto,
    pub width: u16,
    pub height: u16,
    pub difficulty: String,
    pub game_mode: String,
    pub requires_pol: bool,
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
pub struct ScenarioDto {
    pub name: SaveStringDto,
    pub file_name: SaveStringDto,
    pub description: SaveStringDto,
    pub width: u16,
    pub height: u16,
    pub difficulty: String,
    pub language: String,
    pub game_type: String,
    pub requires_pol: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ScenarioMutationDto {
    SetName { text: String },
    SetFileName { text: String },
    SetDescription { text: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct SaveStringDto {
    pub text: String,
    pub raw_bytes: Vec<u8>,
    pub valid_utf8: bool,
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
