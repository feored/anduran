use std::path::Path;

use kastore::{Diagnostic, ParseReport, SaveGame, SaveString};

use crate::dto::{DiagnosticDto, HeaderDto, OpenedSaveDto, SaveStringDto, SourceDto};

pub fn opened_save_from_report(
    path: &Path,
    report: ParseReport<SaveGame>,
) -> OpenedSaveDto {
    let save = report.value;
    let file_info = &save.header.file_info;

    OpenedSaveDto {
        source: SourceDto {
            path: path.to_string_lossy().into_owned(),
            file_name: path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_default(),
            save_version: save.source_version.as_u16(),
        },
        header: HeaderDto {
            map_name: save_string_dto(&file_info.name),
            map_filename: save_string_dto(&file_info.filename),
            map_description: save_string_dto(&file_info.description),
            width: file_info.width,
            height: file_info.height,
            difficulty: file_info.difficulty.to_string(),
            language: file_info.main_language.to_string(),
            game_type: save.header.game_type.to_string(),
            requires_pol: save.header.requires_pol,
        },
        diagnostics: report.diagnostics.iter().map(diagnostic_dto).collect(),
    }
}

fn save_string_dto(value: &SaveString) -> SaveStringDto {
    SaveStringDto {
        text: value.to_string_lossy(),
        raw_bytes: value.as_bytes().to_vec(),
        valid_utf8: value.as_utf8().is_ok(),
        modified: false,
    }
}

fn diagnostic_dto(diagnostic: &Diagnostic) -> DiagnosticDto {
    DiagnosticDto {
        severity: diagnostic.severity.to_string(),
        kind: diagnostic.kind.to_string(),
        section: diagnostic.section.to_string(),
        field: diagnostic.field.map(str::to_string),
        offset: diagnostic.offset,
        message: diagnostic.message.clone(),
    }
}
