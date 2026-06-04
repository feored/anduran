<script lang="ts">
  import type { Snippet } from 'svelte';

  type BadgeTone = 'default' | 'success' | 'warning' | 'danger' | 'muted';

  const toneClasses: Record<BadgeTone, string> = {
    default: 'border-accent bg-panel-elevated text-accent',
    success: 'border-success bg-panel-elevated text-success',
    warning: 'border-warning bg-panel-elevated text-warning',
    danger: 'border-danger bg-panel-elevated text-danger',
    muted: 'border-border bg-panel-elevated text-muted-foreground',
  };

  let {
    tone = 'default',
    children,
  }: {
    tone?: BadgeTone;
    children?: Snippet;
  } = $props();
</script>

<span class={['inline-flex items-center rounded border px-2 py-0.5 text-xs font-medium', toneClasses[tone]]}>
  {@render children?.()}
</span>
