<script>
  import { onMount, onDestroy } from "svelte";
  import { createEventDispatcher } from "svelte";

  export let className = "";
  export let name = "";
  export let value = "";

  let element;

  const dispatch = createEventDispatcher();

  export function clear() {
    value = "";
    setTimeout(() => {
      element.style.height = "54px";
      element.style.height = element.scrollHeight + "px";
    });
  }

  export function focus() {
    element.focus();
  }

  function updateHeight() {
    element.style.height = "54px";
    element.style.height = element.scrollHeight + "px";
  }

  function handleInput(e) {
    updateHeight();
    dispatch("change", { value });
  }

  function handleMouseUp(e) {
    checkSelection();
  }

  function handleKeyUp(e) {
    checkSelection();
  }

  function checkSelection() {}

  onMount(() => {
    updateHeight();
  });

  onDestroy(() => {});
</script>

<textarea
  class={className}
  {name}
  bind:value
  bind:this={element}
  on:input={handleInput}
  on:mouseup={handleMouseUp}
  on:keyup={handleKeyUp} />
