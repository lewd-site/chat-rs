<script>
  import RichTextBox from "./RichTextBox.svelte";
  import utils from "../utils";

  let message = "";
  let files = [];
  let previews = [];

  let inputFiles = null;
  let formElement;
  let messageElement;

  function setFiles(e) {
    if (inputFiles && inputFiles.length) {
      files = [...files, ...inputFiles].slice(0, 5);
    }

    updatePreviews();
    setTimeout(updateSize);
  }

  function removeFileAt(index) {
    files.splice(index, 1);

    updatePreviews();
    setTimeout(updateSize);
  }

  function updatePreviews(e) {
    previews.forEach(preview => URL.revokeObjectURL(preview));
    previews = files.map(file => ({
      type: file.type,
      name: file.name,
      size: file.size,
      src: URL.createObjectURL(file)
    }));
  }

  function handleSubmit(e) {
    let name = localStorage.getItem("settings.name") || "";
    let tripcode = localStorage.getItem("settings.tripcode") || "";
    if (tripcode.startsWith("#")) {
      tripcode = tripcode.substr(1);
    }

    name = `${name}#${tripcode}`;

    window.api.submitPost({ name, message, files }).then(() => {
      message = "";
      files = [];
      messageElement.clear();
      messageElement.focus();
      updatePreviews();
      setTimeout(updateSize);
    });
  }

  function handleChange() {
    setTimeout(updateSize);
  }

  function updateSize() {
    const scroll = utils.isAtBottom();

    const rect = formElement.getBoundingClientRect();
    document.querySelector(
      ".layout__post-list"
    ).style.marginBottom = `${rect.height + 46}px`;

    if (scroll) {
      setTimeout(utils.scrollToBottom);
    }
  }
</script>

<div class="post-form__left" />

<form
  class="post-form__main"
  method="POST"
  action="/api/v1/posts"
  enctype="multipart/form-data"
  on:submit|preventDefault={handleSubmit}
  bind:this={formElement}>
  <div class="post-form__previews-row">
    {#each previews as preview, index (preview.src)}
      {#if preview.type.startsWith('image/')}
        <picture title={preview.name}>
          <img
            class="post-form__preview post-form__preview_image"
            src={preview.src}
            alt="Preview"
            on:click|preventDefault={e => removeFileAt(index)} />
        </picture>
      {:else if preview.type.startsWith('audio/')}
        <div
          class="post-form__preview post-form__preview_audio"
          title={preview.name}
          on:click|preventDefault={e => removeFileAt(index)} />
      {:else if preview.type.startsWith('video/')}
        <video
          class="post-form__preview post-form__preview_video"
          autoplay
          loop
          muted
          title={preview.name}
          on:click|preventDefault={e => removeFileAt(index)}>
          <source src={preview.src} />
        </video>
      {/if}
    {/each}
  </div>

  <div class="post-form__message-row">
    <div class="post-form__attachment-wrapper">
      <label class="post-form__attachment">
        <input
          type="file"
          name="file"
          bind:files={inputFiles}
          on:change={setFiles}
          multiple
          hidden />
      </label>
    </div>

    <RichTextBox
      className="post-form__message"
      name="message"
      bind:value={message}
      bind:this={messageElement}
      on:change={handleChange} />

    <div class="post-form__submit-wrapper">
      <button class="post-form__submit" type="submit" />
    </div>
  </div>
</form>

<div class="post-form__right" />
