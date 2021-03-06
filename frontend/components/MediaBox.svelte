<script>
  import { fade } from 'svelte/transition';
  import VideoPlayer from './VideoPlayer.svelte';
  import { formatFileSize } from './file';
  import { hslide } from '../anim';
  import { mediaBoxFiles, mediaBoxFile } from '../stores/files';

  const PADDING_H = 96;

  let content = null;

  let offsetX = 0;
  let offsetY = 0;
  let width = 0;
  let height = 0;

  let dragStartOffsetX = 0;
  let dragStartOffsetY = 0;
  let dragStartMouseX = 0;
  let dragStartMouseY = 0;
  let isDragging = false;
  let isDragged = false;

  function getViewWidth() {
    return window.innerWidth - PADDING_H;
  }

  function getViewHeight() {
    return window.innerHeight;
  }

  function handlePointerUp(e) {
    window.removeEventListener('pointerup', handlePointerUp);
    window.removeEventListener('pointermove', handlePointerMove);

    if (!isDragged) {
      mediaBoxFile.set(null);
    }

    isDragging = false;
    isDragged = false;
  }

  function handlePointerMove(e) {
    const windowWidth = getViewWidth();
    const windowHeight = getViewHeight();

    const mouseX = Math.max(0, Math.min(e.clientX, windowWidth));
    const mouseY = Math.max(0, Math.min(e.clientY, windowHeight));

    const deltaX = mouseX - dragStartMouseX;
    const deltaY = mouseY - dragStartMouseY;

    offsetX = dragStartOffsetX + deltaX;
    offsetY = dragStartOffsetY + deltaY;

    if (Math.abs(deltaX) > 4 || Math.abs(deltaY) > 4) {
      isDragged = true;
    }
  }

  function handlePointerDown(e) {
    if (e.button !== 0) {
      return;
    }

    e.preventDefault();

    window.addEventListener('pointerup', handlePointerUp);
    window.addEventListener('pointermove', handlePointerMove);

    dragStartOffsetX = offsetX;
    dragStartOffsetY = offsetY;
    dragStartMouseX = e.clientX;
    dragStartMouseY = e.clientY;
    isDragging = true;
    isDragged = false;
  }

  let src = null;

  mediaBoxFile.subscribe(file => {
    if (!file) {
      return;
    }

    const windowWidth = getViewWidth();
    const windowHeight = getViewHeight();

    const scale = Math.min(Math.min(windowWidth, 800) / file.width, windowHeight / file.height, 1);

    width = file.width * scale;
    height = file.height * scale;
    offsetX = windowWidth / 2 - width / 2;
    offsetY = windowHeight / 2 - height / 2;

    // Unset an old image and set a new one only at the next iteration of the event loop
    // to avoid flash of the old image.
    src = null;
    setTimeout(() => {
      src = `/src/${file.md5}.${file.extension}`;
    });

    if (file.mimetype === 'video/x-tiktok') {
      const existingScript = document.getElementById('tiktok-loader');
      if (existingScript) {
        existingScript.remove();
      }

      const script = document.createElement('script');
      script.src = 'https://www.tiktok.com/embed.js';
      script.async = true;
      script.id = 'tiktok-loader';
      document.head.appendChild(script);
    }
  });

  const WHEEL_SCALE_STEP = 1.1;
  const CLICK_SCALE_STEP = 1.5;
  const MIN_WIDTH = 100;
  const MIN_HEIGHT = 100;
  const MAX_WIDTH = 8000;
  const MAX_HEIGHT = 8000;

  function handleZoom(mouseX, mouseY, scale) {
    if (!$mediaBoxFile) {
      return;
    }

    const newWidth = width * scale;
    const newHeight = height * scale;

    const minWidth =
      typeof $mediaBoxFile.min_width !== 'undefined'
        ? Math.max($mediaBoxFile.min_width, MIN_WIDTH)
        : MIN_WIDTH;

    const maxWidth =
      typeof $mediaBoxFile.max_width !== 'undefined'
        ? Math.min($mediaBoxFile.max_width, MAX_WIDTH)
        : MAX_WIDTH;

    if (newWidth < $mediaBoxFile.width && newWidth < minWidth) {
      return;
    }

    if (newHeight < $mediaBoxFile.height && newHeight < MIN_HEIGHT) {
      return;
    }

    if (newWidth > $mediaBoxFile.width && newWidth > maxWidth) {
      return;
    }

    if (newHeight > $mediaBoxFile.height && newHeight > MAX_HEIGHT) {
      return;
    }

    const deltaWidth = newWidth - width;
    const deltaHeight = newHeight - height;

    const relativeX = (mouseX - offsetX) / width;
    const relativeY = (mouseY - offsetY) / height;

    offsetX -= deltaWidth * relativeX;
    offsetY -= deltaHeight * relativeY;

    dragStartOffsetX -= deltaWidth * relativeX;
    dragStartOffsetY -= deltaHeight * relativeY;

    width = newWidth;
    height = newHeight;
  }

  function handleWheel(e) {
    if (!$mediaBoxFile) {
      return;
    }

    if (e.deltaY === 0) {
      return;
    }

    e.preventDefault();

    const scale = e.deltaY < 0 ? WHEEL_SCALE_STEP : 1 / WHEEL_SCALE_STEP;
    handleZoom(e.clientX, e.clientY, scale);
  }

  function handleCloseClick() {
    mediaBoxFile.set(null);
  }

  function handleZoomInClick() {
    if (!$mediaBoxFile) {
      return;
    }

    const windowWidth = getViewWidth();
    const windowHeight = getViewHeight();

    const originX = Math.max(0, Math.min(offsetX + width / 2, windowWidth));

    const originY = Math.max(0, Math.min(offsetY + height / 2, windowHeight));

    handleZoom(originX, originY, CLICK_SCALE_STEP);
  }

  function handleZoomOutClick() {
    if (!$mediaBoxFile) {
      return;
    }

    const windowWidth = getViewWidth();
    const windowHeight = getViewHeight();

    const originX = Math.max(0, Math.min(offsetX + width / 2, windowWidth));

    const originY = Math.max(0, Math.min(offsetY + height / 2, windowHeight));

    handleZoom(originX, originY, 1 / CLICK_SCALE_STEP);
  }

  function handleZoomOriginal() {
    if (!$mediaBoxFile) {
      return;
    }

    const windowWidth = getViewWidth();
    const windowHeight = getViewHeight();

    const originX = Math.max(0, Math.min(offsetX + width / 2, windowWidth));

    const originY = Math.max(0, Math.min(offsetY + height / 2, windowHeight));

    const currentScale = width / $mediaBoxFile.width;

    handleZoom(originX, originY, 1 / currentScale);
  }

  function handleZoomFit() {
    if (!$mediaBoxFile) {
      return;
    }

    const windowWidth = getViewWidth();
    const windowHeight = getViewHeight();

    const originX = Math.max(0, Math.min(offsetX + width / 2, windowWidth));

    const originY = Math.max(0, Math.min(offsetY + height / 2, windowHeight));

    const currentScale = width / $mediaBoxFile.width;

    const scale = Math.min(
      windowWidth / $mediaBoxFile.width,
      windowHeight / $mediaBoxFile.height,
      1,
    );

    handleZoom(originX, originY, scale / currentScale);
  }

  function handlePrevious() {
    if ($mediaBoxFiles.length === 1) {
      return;
    }

    let index = $mediaBoxFiles.findIndex(file => $mediaBoxFile.id == file.id) - 1;
    if (index < 0) {
      index += $mediaBoxFiles.length;
    }

    mediaBoxFile.set($mediaBoxFiles[index]);
  }

  function handleNext() {
    if ($mediaBoxFiles.length === 1) {
      return;
    }

    let index = $mediaBoxFiles.findIndex(file => $mediaBoxFile.id == file.id) + 1;
    if (index >= $mediaBoxFiles.length) {
      index -= $mediaBoxFiles.length;
    }

    mediaBoxFile.set($mediaBoxFiles[index]);
  }

  function handleFileClick(file) {
    if (!$mediaBoxFile || $mediaBoxFile.id != file.id) {
      mediaBoxFile.set(file);
    }
  }

  function getGalleryFileClass(file, currentFile) {
    const classes = [
      'media-box__file',
      file.id == currentFile.id ? 'media-box__file_current' : null,
      'media-box__file_' + file.mimetype.split('/')[0],
    ];

    return classes.filter(c => c).join(' ');
  }

  function handleGalleryWheel(e) {
    if (!$mediaBoxFile) {
      return;
    }

    if (e.deltaY === 0) {
      return;
    }

    e.preventDefault();

    if (e.deltaY < 0) {
      handlePrevious();
    } else {
      handleNext();
    }
  }

  let searchItems = [];

  $: {
    if ($mediaBoxFile) {
      const path = `${window.location.origin}/src/${$mediaBoxFile.md5}.${$mediaBoxFile.extension}`;
      const pathURL = encodeURIComponent(path);

      searchItems = [
        {
          name: 'IQDB',
          url: `https://iqdb.org/?url=${pathURL}`,
        },
        {
          name: 'Google',
          url: `https://www.google.com/searchbyimage?image_url=${pathURL}`,
        },
        {
          name: 'Yandex',
          url: `https://yandex.ru/images/search?rpt=imageview&img_url=${pathURL}`,
        },
        {
          name: 'TinEye',
          url: `https://tineye.com/search/?url=${pathURL}`,
        },
        {
          name: 'SauceNAO',
          url: `https://saucenao.com/search.php?url=${pathURL}`,
        },
        {
          name: 'trace.moe',
          url: `https://trace.moe/?url=${pathURL}`,
        },
      ];
    }
  }
</script>

{#if $mediaBoxFile !== null}
  <div
    class="media-box__controls"
    on:wheel={handleGalleryWheel}
    transition:hslide={{ duration: 300, origin: '100% 0' }}>
    <div class="media-box__buttons">
      <button class="media-box__close" title="Закрыть" on:click|preventDefault={handleCloseClick} />

      <button
        class="media-box__zoom-in"
        title="Увеличить"
        on:click|preventDefault={handleZoomInClick} />

      <button
        class="media-box__zoom-out"
        title="Уменьшить"
        on:click|preventDefault={handleZoomOutClick} />

      <button
        class="media-box__original"
        title="Оригинальный размер"
        on:click|preventDefault={handleZoomOriginal} />

      <button
        class="media-box__fit"
        title="По размеру окна"
        on:click|preventDefault={handleZoomFit} />

      <div class="media-box__search" title="Поиск по картинке">
        <span class="media-box__search-icon" />

        <div class="media-box__search-popup">
          <ul class="media-box__search-list">
            {#each searchItems as item}
              <li class="media-box__search-item">
                <a class="media-box__search-link" href={item.url} target="_blank">{item.name}</a>
              </li>
            {/each}
          </ul>
        </div>
      </div>

      <a title="Загрузить" href={src} download={$mediaBoxFile.name}>
        <span class="media-box__download" />
      </a>
    </div>

    {#if $mediaBoxFiles.length > 1}
      <button class="media-box__prev" on:click|preventDefault={handlePrevious} />

      <div class="media-box__files">
        {#each $mediaBoxFiles as file}
          <div
            class={getGalleryFileClass(file, $mediaBoxFile)}
            title={`${file.name}, ${file.width}x${file.height}, ${formatFileSize(file.size)}`}
            on:click|preventDefault={e => handleFileClick(file)}>
            <picture>
              <img class="media-box__preview" src="/thumb/{file.md5}?max_width=360" alt="Preview" />
            </picture>
          </div>
        {/each}
      </div>

      <button class="media-box__next" on:click|preventDefault={handleNext} />
    {/if}
  </div>

  <div
    class="media-box__content media-box__content_{$mediaBoxFile.mimetype.split('/')[0]}
    {isDragging ? 'media-box__content_dragging' : ''}"
    style="left: {offsetX}px; top: {offsetY}px; width: {width}px; height: {height}px"
    bind:this={content}
    on:pointerdown={handlePointerDown}
    on:wheel={handleWheel}
    transition:fade={{ duration: 0 }}>
    {#if $mediaBoxFile.mimetype.startsWith('image/') && src}
      <picture>
        <img class="media-box__media media-box__media_image" {src} alt="" />
      </picture>
    {:else if ['video/x-coub', 'video/x-tiktok', 'video/x-youtube'].indexOf($mediaBoxFile.mimetype) !== -1 && src}
      <div class="media-box__handle">
        <span class="media-box__title">{$mediaBoxFile.name}</span>
      </div>

      <div class="media-box__media media-box__media_embed">
        {@html $mediaBoxFile.html}
      </div>
    {:else if $mediaBoxFile.mimetype.startsWith('video/') && src}
      <VideoPlayer className="media-box__media media-box__media_video" file={$mediaBoxFile} />
    {/if}
  </div>
{/if}
