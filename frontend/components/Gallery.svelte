<script>
  import GalleryFile from "./GalleryFile.svelte";
  import { hslide } from "../anim";
  import {
    galleryVisible,
    hideGallery,
    favoriteFiles,
    setFavoriteFiles,
    recentFiles,
    setRecentFiles
  } from "../stores/files";

  galleryVisible.subscribe(async value => {
    if (!value) {
      return;
    }

    setFavoriteFiles(await window.api.getFavoriteFiles());
    setRecentFiles(await window.api.getLatestFiles());
  });

  function handleFileClick(file) {
    const url = `/src/${file.md5}.${file.extension}`;

    const xhr = new XMLHttpRequest();
    xhr.open("GET", url, true);
    xhr.responseType = "blob";

    xhr.addEventListener("readystatechange", e => {
      if (xhr.readyState !== XMLHttpRequest.DONE) {
        return;
      }

      const blob = xhr.response;
      blob.name = file.name;
      window.eventBus.dispatch("gallery_upload", blob);
      hideGallery();
    });

    xhr.send();
  }

  async function handleAddFavClick(file) {
    await window.api.createFavoriteFile(file.md5);
    setFavoriteFiles(await window.api.getFavoriteFiles());
    setRecentFiles(await window.api.getLatestFiles());
  }

  async function handleRemoveFavClick(file) {
    await window.api.deleteFavoriteFile(file.md5);
    setFavoriteFiles(await window.api.getFavoriteFiles());
    setRecentFiles(await window.api.getLatestFiles());
  }
</script>

{#if $galleryVisible}
  <div class="gallery__inner" transition:hslide={{ duration: 300 }}>
    <header class="gallery__header">
      <h2 class="gallery__title">Картинки</h2>

      <button
        class="gallery__hide"
        type="button"
        on:click|preventDefault={hideGallery} />
    </header>

    <section class="gallery__content">
      {#if $favoriteFiles.length}
        <h3 class="gallery__content-title">Избранное</h3>

        <ul class="gallery__files gallery__files_favorite">
          {#each $favoriteFiles as file}
            <GalleryFile
              {file}
              on:fileClick={e => handleFileClick(e.detail)}
              on:favoriteClick={e => handleRemoveFavClick(e.detail)} />
          {/each}
        </ul>
      {/if}

      {#if $recentFiles.length}
        <h3 class="gallery__content-title">Недавно использованные</h3>

        <ul class="gallery__files gallery__files_recent">
          {#each $recentFiles as file (file.id)}
            <GalleryFile
              {file}
              on:fileClick={e => handleFileClick(e.detail)}
              on:favoriteClick={e => handleAddFavClick(e.detail)} />
          {/each}
        </ul>
      {/if}
    </section>
  </div>
{/if}
