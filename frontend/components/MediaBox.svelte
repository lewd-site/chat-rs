<script>
  import VideoPlayer from "./VideoPlayer.svelte";
  import { mediaBoxFiles, mediaBoxFile } from "../stores/files";

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

  function handlePointerUp(e) {
    window.removeEventListener("pointerup", handlePointerUp);
    window.removeEventListener("pointermove", handlePointerMove);

    if (!isDragged) {
      mediaBoxFile.set(null);
    }

    isDragging = false;
    isDragged = false;
  }

  function handlePointerMove(e) {
    const mouseX = Math.max(0, Math.min(e.clientX, window.innerWidth));
    const mouseY = Math.max(0, Math.min(e.clientY, window.innerHeight));

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

    window.addEventListener("pointerup", handlePointerUp);
    window.addEventListener("pointermove", handlePointerMove);

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

    const scale = Math.min(
      window.innerWidth / file.width,
      window.innerHeight / file.height,
      1
    );

    width = file.width * scale;
    height = file.height * scale;
    offsetX = window.innerWidth / 2 - width / 2;
    offsetY = window.innerHeight / 2 - height / 2;

    // Unset an old image and set a new one only at the next iteration of the event loop
    // to avoid flash of the old image.
    src = null;
    setTimeout(() => {
      src = `/src/${file.md5}.${file.extension}`;
    });
  });

  const WHEEL_SCALE_STEP = 1.1;
  const CLICK_SCALE_STEP = 1.5;
  const MIN_WIDTH = 100;
  const MIN_HEIGHT = 100;
  const MAX_WIDTH = 8000;
  const MAX_HEIGHT = 8000;

  function handleZoom(mouseX, mouseY, scale) {
    const newWidth = width * scale;
    const newHeight = height * scale;

    if (newWidth < $mediaBoxFile.width && newWidth < MIN_WIDTH) {
      return;
    }

    if (newHeight < $mediaBoxFile.height && newHeight < MIN_HEIGHT) {
      return;
    }

    if (newWidth > $mediaBoxFile.width && newWidth > MAX_WIDTH) {
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
    const originX = Math.max(
      0,
      Math.min(offsetX + width / 2, window.innerWidth)
    );

    const originY = Math.max(
      0,
      Math.min(offsetY + height / 2, window.innerHeight)
    );

    handleZoom(originX, originY, CLICK_SCALE_STEP);
  }

  function handleZoomOutClick() {
    const originX = Math.max(
      0,
      Math.min(offsetX + width / 2, window.innerWidth)
    );

    const originY = Math.max(
      0,
      Math.min(offsetY + height / 2, window.innerHeight)
    );

    handleZoom(originX, originY, 1 / CLICK_SCALE_STEP);
  }

  function handleZoomOriginal() {
    const originX = Math.max(
      0,
      Math.min(offsetX + width / 2, window.innerWidth)
    );

    const originY = Math.max(
      0,
      Math.min(offsetY + height / 2, window.innerHeight)
    );

    const currentScale = width / $mediaBoxFile.width;

    handleZoom(originX, originY, 1 / currentScale);
  }

  function handleZoomFit() {
    const originX = Math.max(
      0,
      Math.min(offsetX + width / 2, window.innerWidth)
    );

    const originY = Math.max(
      0,
      Math.min(offsetY + height / 2, window.innerHeight)
    );

    const currentScale = width / $mediaBoxFile.width;

    const scale = Math.min(
      window.innerWidth / $mediaBoxFile.width,
      window.innerHeight / $mediaBoxFile.height,
      1
    );

    handleZoom(originX, originY, scale / currentScale);
  }

  function handlePrevious() {
    if ($mediaBoxFiles.length === 1) {
      return;
    }

    let index = $mediaBoxFiles.findIndex(file => $mediaBoxFile === file) - 1;
    if (index < 0) {
      index += $mediaBoxFiles.length;
    }

    mediaBoxFile.set($mediaBoxFiles[index]);
  }

  function handleNext() {
    if ($mediaBoxFiles.length === 1) {
      return;
    }

    let index = $mediaBoxFiles.findIndex(file => $mediaBoxFile === file) + 1;
    if (index >= $mediaBoxFiles.length) {
      index -= $mediaBoxFiles.length;
    }

    mediaBoxFile.set($mediaBoxFiles[index]);
  }

  function handleFileClick(file) {
    if ($mediaBoxFile !== file) {
      mediaBoxFile.set(file);
    }
  }

  function getGalleryFileClass(file, currentFile) {
    const classes = [
      "media-box__file",
      file === currentFile ? "media-box__file_current" : null,
      "media-box__file_" + file.mimetype.split("/")[0]
    ];

    return classes.filter(c => c).join(" ");
  }

  function handleGalleryWheel(e) {
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
</script>

{#if $mediaBoxFile !== null}
  <div class="media-box__controls" on:wheel={handleGalleryWheel}>
    <button
      class="media-box__close"
      title="Закрыть"
      on:click|preventDefault={handleCloseClick} />

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

    {#if $mediaBoxFiles.length > 1}
      <button
        class="media-box__prev"
        on:click|preventDefault={handlePrevious} />

      <div class="media-box__files">
        {#each $mediaBoxFiles as file (file.id)}
          <div
            class={getGalleryFileClass(file, $mediaBoxFile)}
            on:click|preventDefault={e => handleFileClick(file)}>
            <picture>
              <img
                class="media-box__preview"
                src="/thumb/{file.md5}?max_width=360"
                alt="Preview" />
            </picture>
          </div>
        {/each}
      </div>

      <button class="media-box__next" on:click|preventDefault={handleNext} />
    {/if}
  </div>

  <div
    class="media-box__content media-box__content_{$mediaBoxFile.mimetype.split('/')[0]}"
    style="left: {offsetX}px; top: {offsetY}px; width: {width}px; height: {height}px"
    bind:this={content}
    on:pointerdown={handlePointerDown}
    on:wheel={handleWheel}>
    {#if $mediaBoxFile.mimetype.startsWith('image/') && src}
      <picture>
        <img class="media-box__media media-box__media_image" {src} alt="" />
      </picture>
    {:else if $mediaBoxFile.mimetype.startsWith('video/') && src}
      <VideoPlayer
        className="media-box__media media-box__media_video"
        file={$mediaBoxFile} />
    {/if}
  </div>
{/if}
