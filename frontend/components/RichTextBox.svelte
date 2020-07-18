<script>
  import { onMount } from 'svelte';
  import { createEventDispatcher } from 'svelte';

  import RichTextBox from './RichTextBox.ts';

  export let className = '';
  export let name = '';
  export let value = '';

  let element;
  let richTextBox;

  const dispatch = createEventDispatcher();

  onMount(() => {
    richTextBox = new RichTextBox(element, newValue => {
      value = newValue;
      dispatch('change', { value });
    });
  });

  export function clear() {
    richTextBox.clear();
  }

  export function focus() {
    richTextBox.focus();
  }
</script>

<input type="hidden" {name} bind:value hidden />
<div class={className} bind:this={element} />
