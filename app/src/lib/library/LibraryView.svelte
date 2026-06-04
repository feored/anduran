<script lang="ts">
  import { openedSaveSession } from '$lib/editor/opened-save.svelte';
  import Button from '$lib/ui/Button.svelte';

  let { onOpen }: { onOpen: () => void | Promise<void> } = $props();
</script>

<div class="grid justify-start gap-3">
  <Button type="button" variant="primary" disabled={openedSaveSession.isLoading} onclick={onOpen}>
    {openedSaveSession.isLoading ? 'Opening...' : 'Open Save...'}
  </Button>

  {#if openedSaveSession.error}
    <p class="text-sm text-danger">{openedSaveSession.error}</p>
  {/if}
</div>
