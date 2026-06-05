<script lang="ts">
	import { onMount } from "svelte";

	import { errorMessage, listLibrarySaves } from "$lib/backend/api";
	import type { LibraryDto } from "$lib/backend/bindings/LibraryDto";
	import type { LibrarySaveEntryDto } from "$lib/backend/bindings/LibrarySaveEntryDto";
	import { openedSaveSession } from "$lib/editor/opened-save.svelte";
	import Button from "$lib/ui/Button.svelte";
	import Panel from "$lib/ui/Panel.svelte";

	let { onOpen }: { onOpen: (path?: string) => void | Promise<void> } = $props();

	let library = $state<LibraryDto | null>(null);
	let error = $state<string | null>(null);

	onMount(() => {
		void loadLibrary();
	});

	async function loadLibrary() {
		error = null;

		try {
			library = await listLibrarySaves();
		} catch (refreshError) {
			error = errorMessage(refreshError);
		}
	}

	async function openEntry(entry: LibrarySaveEntryDto) {
		if (entry.status !== "ready") {
			return;
		}

		await onOpen(entry.path);
	}

	function formatDate(timestamp: bigint | null) {
		if (timestamp === null) {
			return "Unknown";
		}

		return new Intl.DateTimeFormat(undefined, {
			dateStyle: "medium",
			timeStyle: "short",
		}).format(new Date(Number(timestamp)));
	}
</script>

<div class="grid gap-4">
	<Panel>
		<div class="flex flex-wrap items-center justify-between gap-4">
			<div class="min-w-0">
        <h1 class="text-base font-semibold">Library</h1>

        {#if error}
          <p class="mt-2 text-sm text-danger">{error}</p>
        {/if}

				{#if openedSaveSession.error}
					<p class="mt-2 text-sm text-danger">{openedSaveSession.error}</p>
				{/if}
			</div>

			<Button
				type="button"
				variant="primary"
				disabled={openedSaveSession.isLoading}
				onclick={() => onOpen()}
			>
				{openedSaveSession.isLoading ? "Opening..." : "Open Save..."}
			</Button>
		</div>
	</Panel>

	<Panel>
		<div class="overflow-x-auto">
			<table class="w-full border-collapse text-sm">
				<thead
					class="border-b border-border text-left text-xs uppercase tracking-wide text-muted-foreground"
				>
					<tr>
						<th class="min-w-64 px-3 py-2 font-medium">File</th>
						<th class="min-w-48 px-3 py-2 font-medium">Map</th>
						<th class="whitespace-nowrap px-3 py-2 font-medium">Dimensions</th>
						<th class="whitespace-nowrap px-3 py-2 font-medium">Difficulty</th>
						<th class="whitespace-nowrap px-3 py-2 font-medium">Mode</th>
						<th class="whitespace-nowrap px-3 py-2 font-medium">Version</th>
						<th class="whitespace-nowrap px-3 py-2 font-medium">Size</th>
						<th class="whitespace-nowrap px-3 py-2 font-medium">Modified</th>
					</tr>
				</thead>
				<tbody>
					{#if library === null}
						<tr>
							<td class="px-3 py-6 text-muted-foreground" colspan="8"
								>Loading saves...</td
							>
						</tr>
					{:else if library.entries.length === 0}
						<tr>
							<td class="px-3 py-6 text-muted-foreground" colspan="8"
								>No save files found.</td
							>
						</tr>
					{:else}
						{#each library.entries as entry (entry.path)}
							<tr
								class={[
									"border-b border-border last:border-b-0",
									entry.status === "ready"
										? "cursor-pointer hover:bg-panel-elevated"
										: "text-muted-foreground",
								]}
								onclick={entry.status === "ready"
									? () => openEntry(entry)
									: undefined}
								onkeydown={(event) => {
									if (entry.status !== "ready") {
										return;
									}

									if (event.key === "Enter" || event.key === " ") {
										event.preventDefault();
										void openEntry(entry);
									}
								}}
								tabindex={entry.status === "ready" ? 0 : undefined}
								aria-disabled={entry.status === "ready" ? undefined : "true"}
							>
								<td class="px-3 py-2 align-top">
									<div
										class="truncate font-mono text-foreground"
										title={entry.fileName}
									>
										{entry.fileName}
									</div>

									{#if entry.diagnostics.length > 0}
										<div
											class="mt-1 truncate text-xs text-danger"
											title={entry.diagnostics[0].message}
										>
											{entry.diagnostics[0].message}
										</div>
									{/if}
								</td>
								<td class="px-3 py-2 align-top">
									{#if entry.summary}
										<div
											class="truncate text-foreground"
											title={entry.summary.mapName.text}
										>
											{entry.summary.mapName.text || "Untitled"}
										</div>
										<div
											class="mt-1 truncate font-mono text-xs text-muted-foreground"
											title={entry.summary.mapFileName.text}
										>
											{entry.summary.mapFileName.text || entry.fileName}
										</div>
									{:else}
										<div class="mt-1 text-xs text-muted-foreground">
											Summary unavailable
										</div>
									{/if}
								</td>
								<td class="px-3 py-2 align-top text-muted-foreground">
									{#if entry.summary}
										{entry.summary.width}x{entry.summary.height}
									{:else}
										-
									{/if}
								</td>
								<td class="px-3 py-2 align-top text-muted-foreground"
									>{entry.summary?.difficulty ?? "-"}</td
								>
								<td class="px-3 py-2 align-top text-muted-foreground"
									>{entry.summary?.gameMode ?? "-"}</td
								>
								<td class="px-3 py-2 align-top text-muted-foreground"
									>{entry.summary ? `v${entry.summary.saveVersion}` : "-"}</td
								>
								<td class="px-3 py-2 align-top text-muted-foreground"
									>{(Number(entry.sizeBytes) / 1024).toFixed(1)} KB</td
								>
								<td class="px-3 py-2 align-top text-muted-foreground"
									>{formatDate(entry.modifiedTimestamp)}</td
								>
							</tr>
						{/each}
					{/if}
				</tbody>
			</table>
		</div>
	</Panel>
</div>
