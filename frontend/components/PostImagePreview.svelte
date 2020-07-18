<script>
  import { createEventDispatcher } from 'svelte';
  import { formatFileSize } from './file';
  import { showOriginalFiles } from '../stores/files';

  export let entry = null;
  export let showOriginal = false;

  const thumbnailUrl = `/thumb/${entry.file.md5}?max_width=360`;
  const originalUrl = `/src/${entry.file.md5}.${entry.file.extension}`;
  let previewUrl = '';

  $: if (showOriginal) {
    switch ($showOriginalFiles) {
      case 'all':
        previewUrl = originalUrl;
        break;

      case 'gif':
        previewUrl =
          entry.file.mimetype === 'image/gif' ? originalUrl : thumbnailUrl;
        break;

      case 'none':
      default:
        previewUrl = thumbnailUrl;
        break;
    }
  } else {
    previewUrl = thumbnailUrl;
  }

  const dispatch = createEventDispatcher();

  function handleFileClick() {
    dispatch('fileClick', entry.file);
  }
</script>

<div
  class="post__file"
  style={`top: ${entry.top}px; left: ${entry.left}px; width: ${entry.width}px; height: ${entry.height}px;`}>
  <a
    href="/src/{entry.file.md5}.{entry.file.extension}"
    target="_blank"
    title={`${entry.file.name}, ${entry.file.width}x${entry.file.height}, ${formatFileSize(entry.file.size)}`}
    on:click|preventDefault={handleFileClick}>
    <picture>
      <img class="post__file-preview" src={previewUrl} alt="Preview" />
    </picture>
  </a>
</div>
