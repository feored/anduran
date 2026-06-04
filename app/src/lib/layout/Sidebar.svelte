<script lang="ts">
	import { editorSections, type EditorSectionId } from "$lib/editor/sections";

	type AppSectionId = "library" | EditorSectionId;

	let {
		activeSection,
		hasOpenSave,
		onSelect,
	}: {
		activeSection: AppSectionId;
		hasOpenSave: boolean;
		onSelect: (section: AppSectionId) => void | Promise<void>;
	} = $props();
</script>

<aside class="border-r border-border bg-panel-muted">
	<div class="border-b border-border px-4 py-4">
		<strong class="block text-sm font-semibold">Anduran</strong>
	</div>

	<nav class="grid gap-1 p-2" aria-label="Sections">
		<button
			type="button"
			aria-current={activeSection === "library" ? "page" : undefined}
			class={[
				"rounded border px-3 py-2 text-left text-sm",
				activeSection === "library"
					? "border-accent bg-panel-elevated text-foreground"
					: "border-transparent text-muted-foreground hover:bg-panel-elevated hover:text-foreground",
			]}
			onclick={() => onSelect("library")}
		>
			{hasOpenSave ? "<- Library" : "Library"}
		</button>

		{#if hasOpenSave}
			{#each editorSections as section}
				<button
					type="button"
					aria-current={section.id === activeSection ? "page" : undefined}
					class={[
						"rounded border px-3 py-2 text-left text-sm",
						section.id === activeSection
							? "border-accent bg-panel-elevated text-foreground"
							: "border-transparent text-muted-foreground hover:bg-panel-elevated hover:text-foreground",
					]}
					onclick={() => onSelect(section.id)}
				>
					{section.label}
				</button>
			{/each}
		{/if}
	</nav>
</aside>
