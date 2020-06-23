<script>
  import { onMount, onDestroy } from "svelte";
  import { scale, slide } from "svelte/transition";

  import TextBox from "./TextBox.svelte";
  import utils from "../utils";

  const MAX_FILES = 5;

  let message = "";
  let files = [];
  let previews = [];
  let showMarkup = false;
  let markupPinned = localStorage.getItem("settings.markup_pinned") === "true";
  let disabled = false;

  let inputFiles = null;
  let formElement;
  let messageElement;

  function updateFiles(e) {
    if (!inputFiles || !inputFiles.length) {
      return;
    }

    if (files.length >= MAX_FILES) {
      return;
    }

    const newFiles = [...inputFiles].slice(0, MAX_FILES - files.length);

    files = [...files, ...newFiles];
    previews = [
      ...previews,
      ...newFiles.map(f => ({
        type: f.type,
        name: f.name,
        size: f.size,
        src: URL.createObjectURL(f)
      }))
    ];

    setTimeout(updateSize, 150);
  }

  function removeFileAt(index) {
    URL.revokeObjectURL(previews[index].src);

    files = files.filter((f, i) => i !== index);
    previews = previews.filter((p, i) => i !== index);

    setTimeout(updateSize, 150);
  }

  async function handleSubmit(e) {
    if (disabled) {
      return;
    }

    if (message.length === 0 && files.length === 0) {
      return;
    }

    let name = localStorage.getItem("settings.name") || "";
    let tripcode = localStorage.getItem("settings.tripcode") || "";
    if (tripcode.startsWith("#")) {
      tripcode = tripcode.substr(1);
    }

    if (tripcode.length) {
      name = `${name}#${tripcode}`;
    }

    try {
      disabled = true;
      const post = await window.api
        .submitPost({ name, message, files })
        .then(() => {
          message = "";

          previews.forEach(p => URL.revokeObjectURL(p.src));

          files = [];
          previews = [];

          messageElement.clear();
          messageElement.focus();

          setTimeout(updateSize, 150);
        });
    } finally {
      disabled = false;
    }
  }

  function handleChange() {
    setTimeout(updateSize);
  }

  function handleSelectionChange(e) {
    const length = e.detail.end - e.detail.start;
    showMarkup = length > 0;
    setTimeout(updateSize, 150);
  }

  function updateSize() {
    const scroll = utils.isAtBottom();

    const rect = formElement.getBoundingClientRect();
    document.querySelector(
      ".layout__post-list"
    ).style.marginBottom = `${rect.height + 32}px`;

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

  function insertMarkup(tag) {
    const range = messageElement.getSelectionRange();
    const textBefore = message.substr(0, range.start);
    const selectedText = message.substr(range.start, range.end - range.start);
    const textAfter = message.substr(range.end);
    const selectionStart = textBefore.length + tag.length + 2;
    const selectionEnd = selectionStart + selectedText.length;

    message = [
      textBefore,
      `[${tag}]`,
      selectedText,
      `[/${tag}]`,
      textAfter
    ].join("");

    setTimeout(() => {
      messageElement.updateHeight();
      messageElement.focus();
      messageElement.setSelectionRange({
        start: selectionStart,
        end: selectionEnd
      });
      setTimeout(updateSize);
    });
  }

  function toggleMarkupPinned() {
    markupPinned = !markupPinned;
    localStorage["settings.markup_pinned"] = markupPinned.toString();
    setTimeout(updateSize, 150);
  }

  onMount(() => {
    window.eventBus.subscribe("reply", handleReply);
    setTimeout(updateSize);
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
  {#if previews.length}
    <div class="post-form__previews-row" transition:slide={{ duration: 150 }}>
      {#each previews as preview, index (preview.src)}
        <div
          in:scale={{ duration: 150 }}
          out:scale={{ duration: 100 }}
          class="post-form__preview-wrapper"
          on:click|preventDefault={e => removeFileAt(index)}>
          {#if preview.type.startsWith('image/')}
            <picture title={preview.name}>
              <img
                class="post-form__preview post-form__preview_image"
                src={preview.src}
                alt="Preview" />
            </picture>
          {:else if preview.type.startsWith('audio/')}
            <div
              class="post-form__preview post-form__preview_audio"
              title={preview.name} />
          {:else if preview.type.startsWith('video/')}
            <video
              class="post-form__preview post-form__preview_video"
              autoplay
              loop
              muted
              disablePictureInPicture
              title={preview.name}>
              <source src={preview.src} />
            </video>
          {/if}
        </div>
      {/each}

      {#if previews.length < MAX_FILES}
        <label
          class="post-form__preview-add"
          in:scale={{ duration: 150 }}
          out:scale={{ duration: 100 }}>
          <input
            type="file"
            bind:files={inputFiles}
            on:change={updateFiles}
            multiple
            hidden
            {disabled} />
        </label>
      {/if}
    </div>
  {/if}

  {#if showMarkup || markupPinned}
    <div class="post-form__markup-row" transition:slide={{ duration: 150 }}>
      <button
        class="post-form__bold"
        on:click|preventDefault={e => insertMarkup('b')}>
        Tt
      </button>

      <button
        class="post-form__italic"
        on:click|preventDefault={e => insertMarkup('i')}>
        Tt
      </button>

      <button
        class="post-form__underline"
        on:click|preventDefault={e => insertMarkup('u')}>
        Tt
      </button>

      <button
        class="post-form__strike"
        on:click|preventDefault={e => insertMarkup('s')}>
        Tt
      </button>

      <button
        class="post-form__sup"
        on:click|preventDefault={e => insertMarkup('sup')}>
        <span>Tt</span>
      </button>

      <button
        class="post-form__sub"
        on:click|preventDefault={e => insertMarkup('sub')}>
        <span>Tt</span>
      </button>

      <button
        class="post-form__code"
        on:click|preventDefault={e => insertMarkup('code')}>
        Code
      </button>

      <button
        class="post-form__codeblock"
        on:click|preventDefault={e => insertMarkup('codeblock')}>
        Code Block
      </button>

      <button
        class="post-form__spoiler"
        on:click|preventDefault={e => insertMarkup('spoiler')}>
        Spoiler
      </button>

      <button
        class="post-form__pin {markupPinned ? 'post-form__pin_pinned' : ''}"
        on:click|preventDefault={toggleMarkupPinned} />
    </div>
  {/if}

  <div class="post-form__message-row">
    <div class="post-form__attachment-wrapper">
      <label class="post-form__attachment">
        <input
          type="file"
          bind:files={inputFiles}
          on:change={updateFiles}
          multiple
          hidden
          {disabled} />
      </label>
    </div>

    <TextBox
      className="post-form__message"
      name="message"
      bind:value={message}
      bind:this={messageElement}
      on:change={handleChange}
      on:selectionChange={handleSelectionChange}
      {disabled} />

    <div class="post-form__submit-wrapper">
      <button class="post-form__submit" type="submit" />
    </div>
  </div>
</form>

<div class="post-form__right" />
