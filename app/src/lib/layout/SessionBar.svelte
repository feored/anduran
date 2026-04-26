<script lang="ts">
  import { openedSaveSession } from '$lib/editor/opened-save.svelte';
</script>

<header>
  <button type="button" disabled={openedSaveSession.isLoading} onclick={() => openedSaveSession.pickAndOpen()}>
    {openedSaveSession.isLoading ? 'Opening...' : 'Open Save...'}
  </button>

  {#if openedSaveSession.currentSave}
    <p>{openedSaveSession.currentSave.source.path}</p>
  {/if}

  {#if openedSaveSession.error}
    <p>{openedSaveSession.error}</p>
  {/if}
</header>
