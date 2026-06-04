<script lang="ts">
  import { openedSaveSession } from '$lib/editor/opened-save.svelte';
  import ScenarioView from '$lib/editor/scenario/ScenarioView.svelte';
  import type { EditorSectionId } from '$lib/editor/sections';
  import StatusView from '$lib/editor/status/StatusView.svelte';
  import LibraryView from '$lib/library/LibraryView.svelte';
  import Sidebar from '$lib/layout/Sidebar.svelte';

  type AppSectionId = 'library' | EditorSectionId;

  let activeSection = $state<AppSectionId>('library');

  async function selectSection(section: AppSectionId) {
    if (section === 'library') {
      if (openedSaveSession.currentSave) {
        const closed = await openedSaveSession.returnToLibrary();
        if (!closed) {
          return;
        }
      }

      activeSection = 'library';
      return;
    }

    if (openedSaveSession.currentSave) {
      activeSection = section;
    }
  }

  async function openFromLibrary() {
    const opened = await openedSaveSession.pickAndOpen();

    if (opened) {
      activeSection = 'scenario';
    }
  }
</script>

<div class="grid min-h-screen grid-cols-[13rem_1fr] bg-background text-foreground">
  <Sidebar
    {activeSection}
    hasOpenSave={openedSaveSession.currentSave !== null}
    onSelect={selectSection}
  />

  <main class="min-w-0 p-4">
    {#if activeSection === 'library' || openedSaveSession.currentSave === null}
      <LibraryView onOpen={openFromLibrary} />
    {:else if activeSection === 'scenario'}
      <ScenarioView save={openedSaveSession.currentSave} />
    {:else}
      <StatusView save={openedSaveSession.currentSave} validation={openedSaveSession.validation} />
    {/if}
  </main>
</div>
