<script lang="ts">
  import { editorSections, type EditorSectionId } from '$lib/editor/sections';

  let {
    activeSection,
    onSelect,
  }: {
    activeSection: EditorSectionId;
    onSelect: (section: EditorSectionId) => void;
  } = $props();
</script>

<aside>
  <div>
    <strong>Anduran</strong>
    <span>Save editor</span>
  </div>

  <nav aria-label="Editor sections">
    {#each editorSections as section}
      <button
        type="button"
        aria-current={section.id === activeSection ? 'page' : undefined}
        onclick={() => onSelect(section.id)}
      >
        {section.label}
      </button>
    {/each}
  </nav>
</aside>
