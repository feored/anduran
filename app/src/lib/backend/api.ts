import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

import type { OpenedSaveDto } from './bindings/OpenedSaveDto';
import type { ScenarioMutationDto } from './bindings/ScenarioMutationDto';
import type { ScenarioMutationResultDto } from './bindings/ScenarioMutationResultDto';
import type { ValidationResultDto } from './bindings/ValidationResultDto';

export async function openSave(path: string): Promise<OpenedSaveDto> {
  return invoke<OpenedSaveDto>('open_save', { path });
}

export async function closeOpenSave(): Promise<void> {
  return invoke<void>('close_open_save');
}

export async function saveOpenSave(): Promise<OpenedSaveDto> {
  return invoke<OpenedSaveDto>('save_open_save');
}

export async function validateOpenSave(): Promise<ValidationResultDto> {
  return invoke<ValidationResultDto>('validate_open_save');
}

export async function mutateScenario(mutation: ScenarioMutationDto): Promise<ScenarioMutationResultDto> {
  return invoke<ScenarioMutationResultDto>('mutate_scenario', { mutation });
}

export async function pickSavePath(): Promise<string | null> {
  const selectedPath = await open({
    multiple: false,
    filters: [{ name: 'Heroes II save file', extensions: ['sav', 'savc'] }],
    title: 'Open save file'
  });

  if (selectedPath === null || Array.isArray(selectedPath)) {
    return null;
  }

  return selectedPath;
}

export function errorMessage(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}
