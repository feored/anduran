<script lang="ts">
  import type { OpenedSaveDto } from '$lib/backend/bindings/OpenedSaveDto';
  import { openedSaveSession } from '$lib/editor/opened-save.svelte';
  import Panel from '$lib/ui/Panel.svelte';

  let { save }: { save: OpenedSaveDto } = $props();

  type ScenarioTextEdits = Parameters<typeof openedSaveSession.applyScenarioTextEdits>[0];

  let syncedRevision = $state<bigint | null>(null);
  let textEditTimeout: number | null = null;
  let draftName = $state('');
  let draftFileName = $state('');
  let draftDescription = $state('');

  $effect(() => {
    if (syncedRevision !== save.revision) {
      draftName = save.scenario.name.text;
      draftFileName = save.scenario.fileName.text;
      draftDescription = save.scenario.description.text;
      syncedRevision = save.revision;
    }
  });

  const canEditName = $derived(save.scenario.name.validUtf8);
  const canEditFileName = $derived(save.scenario.fileName.validUtf8);
  const canEditDescription = $derived(save.scenario.description.validUtf8);
  const hasTextChanges = $derived(
    (canEditName && draftName !== save.scenario.name.text) ||
      (canEditFileName && draftFileName !== save.scenario.fileName.text) ||
      (canEditDescription && draftDescription !== save.scenario.description.text)
  );

  const readonlyRows = $derived([
    ['Size', `${save.scenario.width} x ${save.scenario.height}`],
    ['Difficulty', save.scenario.difficulty],
    ['Language', save.scenario.language],
    ['Game type', save.scenario.gameType],
    ['Save version', save.source.saveVersion.toString()],
    ['Requires Price of Loyalty', save.scenario.requiresPol ? 'Yes' : 'No'],
  ]);

  const fieldClass = 'grid gap-1 border-b border-border py-3 last:border-b-0';
  const labelClass = 'flex items-center gap-2 text-xs font-medium text-muted';
  const inputClass =
    'w-full rounded border border-border-strong bg-background px-2 py-1.5 text-sm text-foreground outline-none focus:border-accent focus:bg-panel';
  const readonlyValueClass = 'min-w-0 text-sm text-muted-foreground';

  function pendingTextEdits(): ScenarioTextEdits {
    return {
      name: canEditName && draftName !== save.scenario.name.text ? draftName : undefined,
      fileName: canEditFileName && draftFileName !== save.scenario.fileName.text ? draftFileName : undefined,
      description:
        canEditDescription && draftDescription !== save.scenario.description.text ? draftDescription : undefined,
    };
  }

  function clearTextEditTimeout() {
    if (textEditTimeout !== null) {
      window.clearTimeout(textEditTimeout);
      textEditTimeout = null;
    }
  }

  function submitTextEdits() {
    if (!hasTextChanges) {
      return;
    }

    void openedSaveSession.applyScenarioTextEdits(pendingTextEdits());
  }

  $effect(() => {
    if (!hasTextChanges) {
      return;
    }

    clearTextEditTimeout();
    textEditTimeout = window.setTimeout(() => {
      textEditTimeout = null;
      submitTextEdits();
    }, 450);

    return clearTextEditTimeout;
  });

  function flushTextEdits() {
    clearTextEditTimeout();
    submitTextEdits();
  }
</script>

<Panel title="Scenario" meta={save.source.fileName}>
  <div>
    <div class={fieldClass}>
      <div class={labelClass}>
        <label for="scenario-name">Name</label>
        {#if openedSaveSession.isMutating}
          <span class="text-xs text-muted-foreground">Updating...</span>
        {/if}
      </div>

      {#if canEditName}
        <input
          id="scenario-name"
          class={[inputClass, 'px-3 py-2 text-lg font-semibold']}
          value={draftName}
          oninput={(event) => (draftName = event.currentTarget.value)}
          onblur={flushTextEdits}
        />
      {:else}
        <p class={readonlyValueClass}>{save.scenario.name.text || 'Untitled'}</p>
      {/if}
    </div>

    <div class={fieldClass}>
      <div class={labelClass}>
        <label for="scenario-description">Description</label>
      </div>

      {#if canEditDescription}
        <textarea
          id="scenario-description"
          class={[inputClass, 'min-h-28 resize-y leading-6']}
          value={draftDescription}
          oninput={(event) => (draftDescription = event.currentTarget.value)}
          onblur={flushTextEdits}
        ></textarea>
      {:else}
        <p class={[readonlyValueClass, 'leading-6']}>{save.scenario.description.text || 'No scenario description.'}</p>
      {/if}
    </div>

    <div class={fieldClass}>
      <div class={labelClass}>
        <label for="scenario-file">File</label>
      </div>

      {#if canEditFileName}
        <input
          id="scenario-file"
          class={[inputClass, 'font-mono']}
          value={draftFileName}
          oninput={(event) => (draftFileName = event.currentTarget.value)}
          onblur={flushTextEdits}
        />
      {:else}
        <p class={[readonlyValueClass, 'font-mono']}>{save.scenario.fileName.text || 'Unknown'}</p>
      {/if}
    </div>

    {#each readonlyRows as [label, value]}
      <div class={fieldClass}>
        <div class={labelClass}>
          <span>{label}</span>
        </div>
        <p class={[readonlyValueClass, label === 'Save version' || label === 'Size' ? 'font-mono' : '']}>{value}</p>
      </div>
    {/each}
  </div>
</Panel>
