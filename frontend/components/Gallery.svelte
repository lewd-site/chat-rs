<script>
  import { hslide } from "../anim";
  import {
    galleryVisible,
    hideGallery,
    galleryFiles,
    setGalleryFiles
  } from "../stores/files";

  galleryVisible.subscribe(async value => {
    if (!value) {
      return;
    }

    setGalleryFiles(await window.api.getLatestFiles());
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
      <h3 class="gallery__content-title">Недавно использованные</h3>

      <ul class="gallery__files">
        {#each $galleryFiles as file (file.id)}
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
            </li>
          {/if}
        {/each}
      </ul>
    </section>
  </div>
{/if}
