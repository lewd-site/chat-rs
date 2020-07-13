<script>
  import GalleryFile from "./GalleryFile.svelte";
  import { formatFileSize } from "./file";
  import { hslide } from "../anim";
  import {
    galleryVisible,
    hideGallery,
    favoriteFiles,
    setFavoriteFiles,
    recentFiles,
    setRecentFiles
  } from "../stores/files";

  let tab = "main";

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

  let tenorSearch = "";
  let tenorPage = 0;
  let tenorResults = [];
  let tenorTimeout = null;

  function handleTenorSearchChange(e) {
    if (tenorTimeout !== null) {
      clearTimeout(tenorTimeout);
      tenorTimeout = null;
    }

    if (!e.target.value.length) {
      tenorPage = 0;
      tenorResults = [];
      return;
    }

    tenorTimeout = setTimeout(async () => {
      tenorPage = 0;
      tenorResults = (await window.tenor.search(e.target.value, 0)).results;
      tenorTimeout = null;
    }, 500);
  }

  function handleTenorLoadMore(e) {
    if (tenorTimeout !== null) {
      clearTimeout(tenorTimeout);
      tenorTimeout = null;
    }

    tenorTimeout = setTimeout(async () => {
      tenorPage++;
      const newResults = (await window.tenor.search(tenorSearch, tenorPage))
        .results;
      tenorResults = [...tenorResults, ...newResults];
      tenorTimeout = null;
    }, 500);
  }

  function handleGifClick(gif) {
    const url = gif.media[0]["gif"].url;

    const xhr = new XMLHttpRequest();
    xhr.open("GET", url, true);
    xhr.responseType = "blob";

    xhr.addEventListener("readystatechange", e => {
      if (xhr.readyState !== XMLHttpRequest.DONE) {
        return;
      }

      const blob = xhr.response;
      blob.name = "tenor.gif";
      window.eventBus.dispatch("gallery_upload", blob);
      hideGallery();
    });

    xhr.send();

    window.tenor.share(tenorSearch, gif.id);
  }
</script>

{#if $galleryVisible}
  <div class="gallery__inner" transition:hslide={{ duration: 300, origin: '100% 0' }}>
    {#if tab === 'main'}
      <header class="gallery__header">
        <div
          class="gallery__title gallery__title_current"
          on:click|preventDefault={e => (tab = 'main')}>
          Картинки
        </div>

        <div
          class="gallery__title"
          on:click|preventDefault={e => (tab = 'gif')}>
          GIF
        </div>

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
    {:else if tab === 'gif'}
      <header class="gallery__header">
        <div
          class="gallery__title"
          on:click|preventDefault={e => (tab = 'main')}>
          Картинки
        </div>

        <div
          class="gallery__title gallery__title_current"
          on:click|preventDefault={e => (tab = 'gif')}>
          GIF
        </div>

        <button
          class="gallery__hide"
          type="button"
          on:click|preventDefault={hideGallery} />
      </header>

      <section class="gallery__content">
        <input
          type="text"
          class="gallery__search"
          placeholder="Search Tenor"
          name="search"
          autocomplete="off"
          on:input={handleTenorSearchChange}
          bind:value={tenorSearch}
          maxlength="255" />

        <ul class="gallery__files gallery__files_gifs">
          {#each tenorResults as item (item.id)}
            <li class="gallery__file gallery__file_image">
              <a
                class="gallery__file-link"
                href={item.url}
                target="_blank"
                title={`${item.media[0]['gif'].dims[0]}x${item.media[0]['gif'].dims[1]}, ${formatFileSize(item.media[0]['gif'].size)}`}
                on:click|preventDefault={e => handleGifClick(item)}>
                <picture>
                  <img
                    class="gallery__file-preview"
                    src={item.media[0]['nanogif'].url}
                    alt="Preview" />
                </picture>
              </a>
            </li>
          {/each}
        </ul>

        {#if tenorResults.length}
          <button
            class="gallery__button"
            type="button"
            on:click|preventDefault={handleTenorLoadMore}>
            Загрузить ещё
          </button>
        {/if}

        <span class="gallery__attribution">Powered By Tenor</span>
      </section>
    {/if}
  </div>
{/if}
