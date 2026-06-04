<script lang="ts">
  import type { Snippet } from 'svelte';
  import type { HTMLButtonAttributes } from 'svelte/elements';

  type ButtonVariant = 'primary' | 'secondary' | 'ghost';
  type ButtonSize = 'sm' | 'md';

  let {
    variant = 'secondary',
    size = 'md',
    class: className = '',
    children,
    ...rest
  }: HTMLButtonAttributes & {
    variant?: ButtonVariant;
    size?: ButtonSize;
    children?: Snippet;
  } = $props();

  const variantClass = $derived(
    variant === 'primary'
      ? 'border-foreground bg-foreground text-background hover:border-muted hover:bg-muted'
      : variant === 'ghost'
        ? 'border-transparent bg-transparent text-muted-foreground hover:bg-panel-elevated hover:text-foreground'
        : 'border-border bg-panel-elevated text-foreground hover:border-border-strong hover:bg-panel'
  );

  const sizeClass = $derived(size === 'sm' ? 'h-8 px-3 text-xs' : 'h-9 px-4 text-sm');
</script>

<button
  class={[
    'inline-flex items-center justify-center rounded border font-medium',
    'disabled:cursor-not-allowed disabled:border-border disabled:bg-panel disabled:text-muted',
    'focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-accent',
    variantClass,
    sizeClass,
    className,
  ]}
  {...rest}
>
  {@render children?.()}
</button>
