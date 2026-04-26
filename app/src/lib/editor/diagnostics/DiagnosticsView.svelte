<script lang="ts">
  import type { DiagnosticDto } from '$lib/backend/bindings/DiagnosticDto';

  let { diagnostics }: { diagnostics: DiagnosticDto[] } = $props();
</script>

<section>
  <div>
    <h1>Diagnostics</h1>
    <span>{diagnostics.length}</span>
  </div>

  {#if diagnostics.length === 0}
    <p>No parser diagnostics were reported for this save.</p>
  {:else}
    <div>
      {#each diagnostics as diagnostic}
        <article>
          <strong>{diagnostic.kind}</strong>
          <span>{diagnostic.section}{diagnostic.field ? `.${diagnostic.field}` : ''}</span>
          <p>{diagnostic.message}</p>
          {#if diagnostic.offset !== null}
            <small>Offset {diagnostic.offset}</small>
          {/if}
        </article>
      {/each}
    </div>
  {/if}
</section>
