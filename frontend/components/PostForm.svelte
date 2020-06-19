<script>
  import { onMount, onDestroy } from "svelte";
  import TextBox from "./TextBox.svelte";
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

  function handleReply(postId) {
    const selection = window
      .getSelection()
      .toString()
      .replace(/\r/g, "")
      .trim()
      .replace(/^(.+)$/gm, "> $1");

    const range = messageElement.getSelectionRange();
    const textBefore = message.substr(0, range.start);
    const textAfter = message.substr(range.end);
    let cursor = textBefore.length;

    const replyToTheSamePost =
      textBefore.lastIndexOf(`>>${postId}`) !== -1 &&
      textBefore.lastIndexOf(`>>${postId}`) === textBefore.lastIndexOf(">>");

    let textToInsert = replyToTheSamePost ? "" : `>>${postId}`;

    if (selection.length) {
      if (textToInsert.length) {
        textToInsert += `\n${selection}`;
      } else {
        textToInsert = selection;
      }
    }

    if (textToInsert.length) {
      if (textBefore.length && !textBefore.endsWith("\n")) {
        textToInsert = `\n${textToInsert}`;
      }

      if (textAfter.length) {
        if (textAfter.startsWith("\n")) {
          textToInsert = `${textToInsert}\n`;
        } else {
          textToInsert = `${textToInsert}\n\n`;
          cursor--;
        }
      } else {
        textToInsert = `${textToInsert}\n`;
      }
    }

    message = [textBefore, textToInsert, textAfter].join("");
    cursor += textToInsert.length;

    setTimeout(() => {
      messageElement.updateHeight();
      messageElement.focus();
      messageElement.setSelectionRange({ start: cursor, end: cursor });
      setTimeout(updateSize);
    });
  }

  onMount(() => {
    window.eventBus.subscribe("reply", handleReply);
  });

  onDestroy(() => {
    window.eventBus.unsubscribe("reply", handleReply);
  });
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

    <TextBox
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
