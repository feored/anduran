<script lang="ts">
  import DiagnosticsView from '$lib/editor/diagnostics/DiagnosticsView.svelte';
  import HeaderView from '$lib/editor/header/HeaderView.svelte';
  import { openedSaveSession } from '$lib/editor/opened-save.svelte';
  import type { EditorSectionId } from '$lib/editor/sections';
  import SessionBar from '$lib/layout/SessionBar.svelte';
  import Sidebar from '$lib/layout/Sidebar.svelte';

  let activeSection = $state<EditorSectionId>('header');
</script>

<div>
  <Sidebar {activeSection} onSelect={(section) => (activeSection = section)} />

  <main>
    <SessionBar />

    <div>
      {#if openedSaveSession.currentSave}
        {#if activeSection === 'header'}
          <HeaderView save={openedSaveSession.currentSave} />
        {:else}
          <DiagnosticsView diagnostics={openedSaveSession.currentSave.diagnostics} />
        {/if}
      {:else}
        <section>
          <h1>Open a save file</h1>
          <p>No save opened.</p>
        </section>
      {/if}
    </div>
  </main>
</div>
