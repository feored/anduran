import {
  closeOpenSave,
  errorMessage,
  mutateScenario,
  openSave,
  pickSavePath,
  saveOpenSave,
  validateOpenSave
} from '$lib/backend/api';
import type { OpenedSaveDto } from '$lib/backend/bindings/OpenedSaveDto';
import type { ScenarioMutationDto } from '$lib/backend/bindings/ScenarioMutationDto';
import type { ValidationResultDto } from '$lib/backend/bindings/ValidationResultDto';

type ScenarioTextEdits = {
  name?: string;
  fileName?: string;
  description?: string;
};

class OpenedSaveSession {
  currentSave = $state<OpenedSaveDto | null>(null);
  validation = $state<ValidationResultDto | null>(null);
  isLoading = $state(false);
  isMutating = $state(false);
  isSaving = $state(false);
  isValidating = $state(false);
  error = $state<string | null>(null);

  async open(path: string): Promise<boolean> {
    const savePath = path.trim();
    if (!savePath) {
      this.error = 'No save file selected.';
      return false;
    }

    this.isLoading = true;
    this.error = null;

    try {
      const openedSave = await openSave(savePath);
      this.currentSave = openedSave;
      this.validation = null;
      void this.validate();
      return true;
    } catch (error) {
      this.error = errorMessage(error);
      return false;
    } finally {
      this.isLoading = false;
    }
  }

  async pickAndOpen(): Promise<boolean> {
    this.error = null;

    try {
      const path = await pickSavePath();

      if (path === null) {
        return false;
      }

      return await this.open(path);
    } catch (error) {
      this.error = errorMessage(error);
      return false;
    }
  }

  async returnToLibrary(): Promise<boolean> {
    if (this.currentSave?.dirty && !window.confirm('Discard unsaved changes and return to the library?')) {
      return false;
    }

    this.error = null;

    try {
      await closeOpenSave();
      this.currentSave = null;
      this.validation = null;
      return true;
    } catch (error) {
      this.error = errorMessage(error);
      return false;
    }
  }

  async applyScenarioTextEdits(edits: ScenarioTextEdits) {
    if (this.currentSave === null) {
      this.error = 'No save file is currently open.';
      return;
    }

    const mutations: ScenarioMutationDto[] = [];

    if (edits.name !== undefined) {
      mutations.push({ type: 'setName', text: edits.name });
    }

    if (edits.fileName !== undefined) {
      mutations.push({ type: 'setFileName', text: edits.fileName });
    }

    if (edits.description !== undefined) {
      mutations.push({ type: 'setDescription', text: edits.description });
    }

    if (mutations.length === 0) {
      return;
    }

    this.isMutating = true;
    this.error = null;

    try {
      for (const mutation of mutations) {
        const result = await mutateScenario(mutation);
        this.currentSave = {
          ...this.currentSave,
          scenario: result.scenario,
          dirty: result.dirty,
          revision: result.revision
        };
        this.validation = result.validation;
      }
    } catch (error) {
      this.error = errorMessage(error);
    } finally {
      this.isMutating = false;
    }
  }

  async save() {
    if (this.currentSave === null) {
      this.error = 'No save file is currently open.';
      return;
    }

    this.isSaving = true;
    this.error = null;

    try {
      const openedSave = await saveOpenSave();
      this.currentSave = openedSave;
    } catch (error) {
      this.error = errorMessage(error);
    } finally {
      this.isSaving = false;
    }
  }

  async validate() {
    if (this.currentSave === null) {
      this.error = 'No save file is currently open.';
      return;
    }

    this.isValidating = true;
    this.error = null;

    try {
      const validation = await validateOpenSave();
      if (this.currentSave?.revision === validation.revision) {
        this.validation = validation;
      }
    } catch (error) {
      this.error = errorMessage(error);
    } finally {
      this.isValidating = false;
    }
  }
}

export const openedSaveSession = new OpenedSaveSession();
