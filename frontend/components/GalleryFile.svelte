<script>
  import { createEventDispatcher } from "svelte";

  export let file = null;

  const dispatch = createEventDispatcher();

  function handleFileClick(file) {
    dispatch("fileClick", file);
  }

  function handleFavoriteClick(file) {
    dispatch("favoriteClick", file);
  }
</script>

{#if file.mimetype.startsWith('image/')}
  <li class="gallery__file gallery__file_image">
    <a
      class="gallery__file-link"
      href="/src/{file.md5}.{file.extension}"
      target="_blank"
      title={file.name}
      on:click|preventDefault={e => handleFileClick(file)}>
      <picture>
        <img
          class="gallery__file-preview"
          src="/thumb/{file.md5}?max_width=360"
          alt="Preview" />
      </picture>
    </a>

    <button
      type="button"
      class="gallery__favorite"
      on:click|stopPropagation|preventDefault={e => handleFavoriteClick(file)} />
  </li>
{:else if file.mimetype.startsWith('audio/')}
  <li class="gallery__file gallery__file_audio">
    <a
      class="gallery__file-link"
      href="/src/{file.md5}.{file.extension}"
      target="_blank"
      title={file.name}
      on:click|preventDefault={e => handleFileClick(file)}>
      <div class="gallery__file-preview" />
    </a>

    <button
      type="button"
      class="gallery__favorite"
      on:click|stopPropagation|preventDefault={e => handleFavoriteClick(file)} />
  </li>
{:else if file.mimetype.startsWith('video/')}
  <li class="gallery__file gallery__file_video">
    <a
      class="gallery__file-link"
      href="/src/{file.md5}.{file.extension}"
      target="_blank"
      title={file.name}
      on:click|preventDefault={e => handleFileClick(file)}>
      <picture>
        <img
          class="gallery__file-preview"
          src="/thumb/{file.md5}?max_width=360"
          alt="Preview" />
      </picture>
    </a>

    <button
      type="button"
      class="gallery__favorite"
      on:click|stopPropagation|preventDefault={e => handleFavoriteClick(file)} />
  </li>
{/if}
