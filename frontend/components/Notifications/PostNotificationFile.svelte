<script>
  import { createEventDispatcher } from 'svelte';
  import { formatFileSize } from '../file';

  export let file = null;

  $: mimetype = file.mimetype.split('/')[0];
  $: className = ['notification__file', 'notification__file_' + mimetype].join(' ');

  const dispatch = createEventDispatcher();

  function handleClick() {
    dispatch('click');
  }
</script>

<div
  class={className}
  title={`${file.name}, ${file.width}x${file.height}, ${formatFileSize(file.size)}`}
  on:click|preventDefault={handleClick}>
  <picture>
    <img class="notification__file-preview" src="/thumb/{file.md5}?max_width=360" alt="Preview" />
  </picture>
</div>
