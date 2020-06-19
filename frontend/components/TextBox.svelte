<script>
  import { onMount, onDestroy, createEventDispatcher } from "svelte";

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

  export function getSelectionRange() {
    return { start: element.selectionStart, end: element.selectionEnd };
  }

  export function setSelectionRange(range) {
    element.selectionStart = range.start;
    element.selectionEnd = range.end;
  }

  export function updateHeight() {
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
