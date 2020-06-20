<script>
  import { onMount, onDestroy } from "svelte";

  export let className = "";
  export let file = null;

  $: _className = ["video-player", className].filter(c => c).join(" ");
  $: src = file ? `/src/${file.md5}.${file.extension}` : null;

  let playerElement = null;
  let videoElement = null;
  let volumeElement = null;
  let seekElement = null;

  let paused = false;
  let fullscreen = false;
  let prevVolume = 0.5;
  let volume = 0.5;
  let currentTime = 0;
  let duration = 0;

  $: _currentTime = formatTime(currentTime);
  $: _duration = formatTime(duration);

  function formatTime(value) {
    if (!value) {
      return "0:00";
    }

    const seconds = Math.floor(value % 60);
    const minutes = Math.floor(value / 60);

    const _seconds = seconds.toString().padStart(2, "0");
    const _minutes = minutes.toString().padStart(1, "0");

    return `${_minutes}:${_seconds}`;
  }

  function handlePointerUp(e) {
    window.removeEventListener("pointerup", handlePointerUp);
    window.removeEventListener("pointermove", handlePointerMove);

    if (!paused) {
      videoElement.play();
    }
  }

  function handlePointerMove(e) {
    const rect = seekElement.getBoundingClientRect();
    const position = Math.min(
      Math.max(0, (e.clientX - rect.left) / rect.width),
      1
    );
    currentTime = videoElement.currentTime = position * videoElement.duration;
  }

  function handlePointerDown(e) {
    e.preventDefault();

    const rect = seekElement.getBoundingClientRect();
    const position = (e.clientX - rect.left) / rect.width;
    videoElement.pause();
    currentTime = videoElement.currentTime = position * videoElement.duration;

    window.addEventListener("pointerup", handlePointerUp);
    window.addEventListener("pointermove", handlePointerMove);
  }

  function handleVolumePointerUp(e) {
    window.removeEventListener("pointerup", handleVolumePointerUp);
    window.removeEventListener("pointermove", handleVolumePointerMove);
  }

  function handleVolumePointerMove(e) {
    const rect = volumeElement.getBoundingClientRect();
    const position = Math.min(
      Math.max(0, (e.clientX - rect.left) / rect.width),
      1
    );
    volume = videoElement.volume = position;
  }

  function handleVolumePointerDown(e) {
    window.addEventListener("pointerup", handleVolumePointerUp);
    window.addEventListener("pointermove", handleVolumePointerMove);
  }

  function handleLoaded(e) {
    duration = videoElement.duration;
    videoElement.volume = volume;
  }

  function handleVolumeChange(e) {
    volume = videoElement.volume;
    localStorage.setItem("settings.volume", volume.toString());
  }

  function handleFullscreenChange(e) {
    fullscreen = !!document.fullscreenElement;
  }

  function pause() {
    videoElement.pause();
    paused = true;
  }

  function play() {
    videoElement.play();
    paused = false;
  }

  function toggleVolume() {
    if (volume < 0.01) {
      if (prevVolume < 0.01) {
        volume = prevVolume = 0.5;
      } else {
        volume = prevVolume;
      }
    } else {
      prevVolume = volume;
      volume = 0;
    }

    videoElement.volume = volume;
  }

  function expand() {
    playerElement.requestFullscreen();
    fullscreen = true;
  }

  function shrink() {
    document.exitFullscreen();
    fullscreen = false;
  }

  let updateInterval = null;

  onMount(() => {
    document.addEventListener("fullscreenchange", handleFullscreenChange);

    prevVolume = volume =
      localStorage.getItem("settings.volume") !== null
        ? +localStorage.getItem("settings.volume")
        : 0.5;

    updateInterval = setInterval(() => {
      currentTime = videoElement.currentTime;
    }, 500);
  });

  onDestroy(() => {
    document.removeEventListener("fullscreenchange", handleFullscreenChange);

    if (updateInterval) {
      clearInterval(updateInterval);
      updateInterval = null;
    }
  });
</script>

<div class={_className} bind:this={playerElement}>
  {#if src}
    <video
      class="video-player__video"
      autoplay
      loop
      preload="metadata"
      bind:this={videoElement}
      on:loadedmetadata={handleLoaded}
      on:volumechange={handleVolumeChange}>
      <source {src} />
    </video>
  {/if}

  <div class="video-player__controls" on:pointerdown|stopPropagation>
    <div class="video-player__controls-inner">
      <div class="video-player__controls-left">
        {#if paused}
          <button
            class="video-player__controls-play"
            on:click|preventDefault={play} />
        {:else}
          <button
            class="video-player__controls-pause"
            on:click|preventDefault={pause} />
        {/if}

        <button
          class="video-player__controls-mute"
          on:click|preventDefault={toggleVolume} />

        <div
          class="video-player__volume"
          on:pointerdown|preventDefault={handleVolumePointerDown}
          bind:this={volumeElement}>
          <div
            class="video-player__volume-position"
            style="width: {100 * volume}%" />
          <div
            class="video-player__volume-handle"
            style="left: {100 * volume}%" />
        </div>

        <div class="video-player__controls-time">
          <span class="video-player__time">{_currentTime}</span>
          <span class="video-player__duration">{_duration}</span>
        </div>
      </div>

      <div class="video-player__controls-right">
        {#if fullscreen}
          <button
            class="video-player__controls-shrink"
            on:click|preventDefault={shrink} />
        {:else}
          <button
            class="video-player__controls-expand"
            on:click|preventDefault={expand} />
        {/if}
      </div>

      <div
        class="video-player__seek"
        on:pointerdown|preventDefault={handlePointerDown}
        bind:this={seekElement}>
        <div
          class="video-player__seek-time"
          style="width: {(100 * currentTime) / duration}%" />
        <div
          class="video-player__seek-handle"
          style="left: {(100 * currentTime) / duration}%" />
      </div>
    </div>
  </div>
</div>
