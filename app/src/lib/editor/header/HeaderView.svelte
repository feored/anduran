<script lang="ts">
  import type { OpenedSaveDto } from '$lib/backend/bindings/OpenedSaveDto';

  let { save }: { save: OpenedSaveDto } = $props();

  const rows = $derived([
    ['Map name', save.header.mapName.text || 'Untitled'],
    ['Map file', save.header.mapFilename.text || 'Unknown'],
    ['Size', `${save.header.width} x ${save.header.height}`],
    ['Difficulty', save.header.difficulty],
    ['Language', save.header.language],
    ['Game type', save.header.gameType],
    ['Requires Price of Loyalty', save.header.requiresPol ? 'Yes' : 'No'],
    ['Save version', save.source.saveVersion.toString()],
  ]);
</script>

<section>
  <div>
    <div>
      <p>{save.source.fileName}</p>
      <h1>{save.header.mapName.text || 'Untitled save'}</h1>
    </div>
  </div>

  {#if save.header.mapDescription.text}
    <p>{save.header.mapDescription.text}</p>
  {/if}

  <dl>
    {#each rows as [label, value]}
      <div>
        <dt>{label}</dt>
        <dd>{value}</dd>
      </div>
    {/each}
  </dl>
</section>
