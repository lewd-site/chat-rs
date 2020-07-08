<script>
  import { onMount, onDestroy } from "svelte";
  import { formatFileSize } from "./file";

  export let className = "";
  export let file = null;

  let audioElement = null;
  let seekElement = null;

  let expanded = false;
  let volume = 0.5;
  let currentTime = 0;
  let duration = 0;

  $: _currentTime = formatTime(currentTime);
  $: _duration = formatTime(duration);

  $: _className = [
    "audio-player",
    className,
    expanded ? "audio-player_expanded" : null
  ]
    .filter(c => c)
    .join(" ");

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

  let updateInterval = null;

  export function open() {
    expanded = true;

    volume =
      localStorage.getItem("settings.volume") !== null
        ? +localStorage.getItem("settings.volume")
        : 0.5;

    if (updateInterval) {
      clearInterval(updateInterval);
    }

    setTimeout(() => {
      audioElement.play();
    });

    updateInterval = setInterval(() => {
      currentTime = audioElement.currentTime;
    }, 500);

    window.eventBus.dispatch("audio_open", file);
  }

  export function close() {
    expanded = false;
    if (updateInterval) {
      clearInterval(updateInterval);
      updateInterval = null;
    }

    currentTime = 0;
  }

  function handleIconClick() {
    if (expanded) {
      close();
    } else {
      open();
    }
  }

  function handleAudioOpen(_file) {
    if (file.id != _file.id) {
      close();
    }
  }

  function handlePointerUp(e) {
    window.removeEventListener("pointerup", handlePointerUp);
    window.removeEventListener("pointermove", handlePointerMove);

    audioElement.play();
  }

  function handlePointerMove(e) {
    const rect = seekElement.getBoundingClientRect();
    const position = Math.min(
      Math.max(0, (e.clientX - rect.left) / rect.width),
      1
    );
    currentTime = audioElement.currentTime = position * audioElement.duration;
  }

  function handlePointerDown(e) {
    e.preventDefault();

    const rect = seekElement.getBoundingClientRect();
    const position = (e.clientX - rect.left) / rect.width;
    audioElement.pause();
    currentTime = audioElement.currentTime = position * audioElement.duration;

    window.addEventListener("pointerup", handlePointerUp);
    window.addEventListener("pointermove", handlePointerMove);
  }

  function handleLoaded(e) {
    duration = audioElement.duration;
    audioElement.volume = volume;
  }

  function handleVolumeChange(e) {
    volume = audioElement.volume;
    localStorage.setItem("settings.volume", volume.toString());
  }

  onMount(() => {
    window.eventBus.subscribe("audio_open", handleAudioOpen);
  });

  onDestroy(() => {
    window.eventBus.unsubscribe("audio_open", handleAudioOpen);

    if (updateInterval) {
      clearInterval(updateInterval);
      updateInterval = null;
    }
  });
</script>

<div class={_className} title={`${file.name}, ${formatFileSize(file.size)}`}>
  <div class="audio-player__icon" on:click|preventDefault={handleIconClick} />

  {#if file && expanded}
    <audio
      autoplay
      loop
      preload="metadata"
      hidden
      bind:this={audioElement}
      on:loadedmetadata={handleLoaded}
      on:volumechange={handleVolumeChange}>
      <source src="/src/{file.md5}.{file.extension}" />
    </audio>

    <div class="audio-player__main">
      <div class="audio-player__title">{file.name}</div>

      <div class="audio-player__controls">
        <div class="audio-player__time">{_currentTime}</div>

        <div
          class="audio-player__seek"
          on:pointerdown|preventDefault={handlePointerDown}
          bind:this={seekElement}>
          <div
            class="audio-player__seek-time"
            style="width: {(100 * currentTime) / duration}%" />
          <div
            class="audio-player__seek-handle"
            style="left: {(100 * currentTime) / duration}%" />
        </div>

        <div class="audio-player__duration">{_duration}</div>
      </div>
    </div>
  {/if}
</div>
