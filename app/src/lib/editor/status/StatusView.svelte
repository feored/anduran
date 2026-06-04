<script lang="ts">
  import type { OpenedSaveDto } from '$lib/backend/bindings/OpenedSaveDto';
  import type { ValidationResultDto } from '$lib/backend/bindings/ValidationResultDto';
  import { openedSaveSession } from '$lib/editor/opened-save.svelte';
  import Badge from '$lib/ui/Badge.svelte';
  import Button from '$lib/ui/Button.svelte';
  import Panel from '$lib/ui/Panel.svelte';

  let { save, validation }: { save: OpenedSaveDto; validation: ValidationResultDto | null } = $props();
</script>

<Panel title="Status" meta={save.dirty ? 'Unsaved changes' : 'Saved'}>
  <div class="grid gap-5">
    <section class="grid gap-3 border-b border-border pb-4">
      <div class="flex items-center justify-between gap-4">
        <div class="min-w-0">
          <h2 class="text-xs font-semibold uppercase text-muted">Save file</h2>
          <p class="mt-1 truncate font-mono text-sm text-muted-foreground">
            {save.source.path}
          </p>
        </div>

        <div class="flex shrink-0 items-center gap-2">
          {#if save.dirty}
            <Badge tone="warning">Unsaved</Badge>
          {:else}
            <Badge tone="success">Saved</Badge>
          {/if}

          <Button
            type="button"
            variant="primary"
            disabled={openedSaveSession.isSaving}
            onclick={() => openedSaveSession.save()}
          >
            {openedSaveSession.isSaving ? 'Saving...' : 'Save'}
          </Button>
        </div>
      </div>
    </section>

    <section class="grid gap-3 border-b border-border pb-4">
      <h2 class="text-xs font-semibold uppercase text-muted">Model validation</h2>

      {#if openedSaveSession.isValidating}
        <p class="text-sm text-muted-foreground">Validating...</p>
      {:else if validation === null}
        <p class="text-sm text-muted-foreground">Validation has not been run for the current save state.</p>
      {:else if validation.issues.length === 0}
        <p class="text-sm text-success">No model validation issues were reported.</p>
      {:else}
        <div class="grid gap-3">
          {#each validation.issues as issue}
            <article class="rounded border border-warning bg-panel-elevated p-3">
              <div class="font-mono text-xs text-warning">{issue.field}</div>
              <p class="mt-2 text-sm text-foreground">{issue.message}</p>
            </article>
          {/each}
        </div>
      {/if}
    </section>

    <section class="grid gap-3">
      <h2 class="text-xs font-semibold uppercase text-muted">Parser diagnostics</h2>

      {#if save.diagnostics.length === 0}
        <p class="text-sm text-muted-foreground">No parser diagnostics were reported for this save.</p>
      {:else}
        <div class="grid gap-3">
          {#each save.diagnostics as diagnostic}
            {@const severity = diagnostic.severity.toLowerCase()}
            <article class="rounded border border-border bg-panel-muted p-3">
              <div class="flex items-center gap-2">
                <Badge tone={severity.includes('error') ? 'danger' : severity.includes('warn') ? 'warning' : 'muted'}>
                  {diagnostic.severity}
                </Badge>
                <span class="text-xs text-muted-foreground">{diagnostic.kind}</span>
                <span class="font-mono text-xs text-muted-foreground">
                  {diagnostic.section}{diagnostic.field ? `.${diagnostic.field}` : ''}
                </span>
              </div>

              <p class="mt-2 text-sm text-foreground">{diagnostic.message}</p>

              {#if diagnostic.offset !== null}
                <small class="mt-2 block font-mono text-xs text-muted">Offset {diagnostic.offset}</small>
              {/if}
            </article>
          {/each}
        </div>
      {/if}
    </section>
  </div>
</Panel>
