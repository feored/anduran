import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

import type { OpenedSaveDto } from './bindings/OpenedSaveDto';

export async function openSave(path: string): Promise<OpenedSaveDto> {
  return invoke<OpenedSaveDto>('open_save', { path });
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
