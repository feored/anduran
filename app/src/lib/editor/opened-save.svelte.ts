import { errorMessage, openSave, pickSavePath } from '$lib/backend/api';
import type { OpenedSaveDto } from '$lib/backend/bindings/OpenedSaveDto';

class OpenedSaveSession {
  currentSave = $state<OpenedSaveDto | null>(null);
  isLoading = $state(false);
  error = $state<string | null>(null);

  async open(path: string) {
    const savePath = path.trim();
    if (!savePath) {
      this.error = 'No save file selected.';
      return;
    }

    this.isLoading = true;
    this.error = null;

    try {
      this.currentSave = await openSave(savePath);
    } catch (error) {
      this.error = errorMessage(error);
    } finally {
      this.isLoading = false;
    }
  }

  async pickAndOpen() {
    this.error = null;

    try {
      const path = await pickSavePath();

      if (path === null) {
        return;
      }

      await this.open(path);
    } catch (error) {
      this.error = errorMessage(error);
    }
  }
}

export const openedSaveSession = new OpenedSaveSession();
