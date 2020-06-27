<script>
  import Pickr from "@simonwep/pickr";
  import { onMount, onDestroy } from "svelte";
  import { scale, slide } from "svelte/transition";

  import TextBox from "./TextBox.svelte";
  import { showAuthModal } from "../stores/auth";
  import utils from "../utils";

  const MAX_FILES = 5;
  const RESIZE_DELAY = 200;

  let message = "";
  let files = [];
  let previews = [];
  let showMarkup = false;
  let markupPinned = localStorage.getItem("settings.markup_pinned") === "true";
  let disabled = false;

  let inputFiles = null;
  let formElement = null;
  let messageElement = null;
  let fileElement = null;
  let pickr = null;

  function addFiles(newFiles) {
    if (files.length >= MAX_FILES) {
      return;
    }

    newFiles = [...newFiles].slice(0, MAX_FILES - files.length);
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

    setTimeout(updateSize, RESIZE_DELAY);
  }

  function handleFilesChange(e) {
    if (!inputFiles || !inputFiles.length) {
      return;
    }

    addFiles(inputFiles);
  }

  function removeFileAt(index) {
    URL.revokeObjectURL(previews[index].src);

    files = files.filter((f, i) => i !== index);
    previews = previews.filter((p, i) => i !== index);

    setTimeout(updateSize, RESIZE_DELAY);
  }

  async function submit() {
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

      await window.api.getToken();
      if (!window.sso.hasAccessToken || window.sso.hasExpired) {
        showAuthModal.set(true);

        const handleAuthModalCanceled = () => {
          window.eventBus.unsubscribe(
            "authmodal_canceled",
            handleAuthModalCanceled
          );

          window.eventBus.unsubscribe(
            "authmodal_submitted",
            handleAuthModalSubmitted
          );
        };

        const handleAuthModalSubmitted = () => {
          window.eventBus.unsubscribe(
            "authmodal_canceled",
            handleAuthModalCanceled
          );

          window.eventBus.unsubscribe(
            "authmodal_submitted",
            handleAuthModalSubmitted
          );

          submit();
        };

        window.eventBus.subscribe(
          "authmodal_canceled",
          handleAuthModalCanceled
        );

        window.eventBus.subscribe(
          "authmodal_submitted",
          handleAuthModalSubmitted
        );

        return;
      }

      const post = await window.api.submitPost({ name, message, files });

      message = "";

      previews.forEach(p => URL.revokeObjectURL(p.src));

      files = [];
      previews = [];

      messageElement.clear();
      setTimeout(() => messageElement.focus());
      setTimeout(updateSize, RESIZE_DELAY);
    } finally {
      disabled = false;
    }
  }

  function handleSubmit(e) {
    submit();
  }

  function handleChange() {
    setTimeout(updateSize);
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

  function insertMarkup(tag, arg) {
    const range = messageElement.getSelectionRange();
    const textBefore = message.substr(0, range.start);
    const selectedText = message.substr(range.start, range.end - range.start);
    const textAfter = message.substr(range.end);
    const selectionStart =
      textBefore.length + tag.length + (arg ? `${arg}`.length + 1 : 0) + 2;
    const selectionEnd = selectionStart + selectedText.length;

    message = [
      textBefore,
      arg ? `[${tag}=${arg}]` : `[${tag}]`,
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

  function insertColor() {
    pickr.show();
  }

  function toggleMarkupPinned() {
    markupPinned = !markupPinned;
    localStorage["settings.markup_pinned"] = markupPinned.toString();
    setTimeout(updateSize, RESIZE_DELAY);
  }

  function handleKeyDown(e) {
    if (e.code === "Enter" && e.ctrlKey) {
      e.preventDefault();
      submit();
    } else if (e.code === "KeyB" && e.ctrlKey) {
      e.preventDefault();
      fileElement.click();
    } else if (e.code === "KeyB" && e.altKey) {
      e.preventDefault();
      insertMarkup("b");
    } else if (e.code === "KeyI" && e.altKey) {
      e.preventDefault();
      insertMarkup("i");
    } else if (e.code === "KeyT" && e.altKey) {
      e.preventDefault();
      insertMarkup("s");
    } else if (e.code === "KeyP" && e.altKey) {
      e.preventDefault();
      insertMarkup("spoiler");
    } else if (e.code === "KeyC" && e.altKey) {
      e.preventDefault();
      insertMarkup("code");
    }
  }

  function handlePaste(e) {
    const data = e.clipboardData || e.originalEvent.clipboardData;
    if (!data || !data.items) {
      return;
    }

    const files = [...data.items]
      .filter(
        item =>
          item.type.startsWith("image/") ||
          item.type.startsWith("audio/") ||
          item.type.startsWith("video/")
      )
      .map(item => item.getAsFile());

    if (files.length > 0) {
      addFiles(files);
    }
  }

  let hideMarkupTimeout = null;

  function handleFocusIn(e) {
    showMarkup = true;
    setTimeout(updateSize, RESIZE_DELAY);

    if (hideMarkupTimeout) {
      clearTimeout(hideMarkupTimeout);
      hideMarkupTimeout = null;
    }
  }

  function handleFocusOut(e) {
    if (!hideMarkupTimeout) {
      hideMarkupTimeout = setTimeout(() => {
        showMarkup = false;
        setTimeout(updateSize, RESIZE_DELAY);
      }, 150);
    }
  }

  onMount(() => {
    window.eventBus.subscribe("reply", handleReply);
    setTimeout(updateSize);

    pickr = Pickr.create({
      el: ".post-form__color-picker",
      theme: "nano",
      useAsButton: true,
      default: "#ffffff",
      defaultRepresentation: "HEX",
      position: "top-middle",
      components: {
        preview: true,
        opacity: true,
        hue: true,
        interaction: {
          cancel: true,
          save: true
        }
      },
      i18n: {
        "btn:save": "Вставить",
        "btn:cancel": "Отмена",
        "aria:btn:save": "Вставить",
        "aria:btn:cancel": "Отмена"
      }
    });

    pickr.on("save", () => {
      pickr.hide();
      insertMarkup("color", pickr.getColor().toHEXA());
      setTimeout(messageElement.focus());
    });

    pickr.on("cancel", () => {
      pickr.hide();
      setTimeout(messageElement.focus());
    });
  });

  onDestroy(() => {
    window.eventBus.unsubscribe("reply", handleReply);
    pickr.destroy();
  });
</script>

<div class="post-form__left" />

<form
  class="post-form__main"
  method="POST"
  action="/api/v1/posts"
  enctype="multipart/form-data"
  on:submit|preventDefault={handleSubmit}
  on:focusin={handleFocusIn}
  on:focusout={handleFocusOut}
  bind:this={formElement}
  tabindex="-1">
  {#if previews.length}
    <div class="post-form__previews-row" transition:slide={{ duration: 150 }}>
      {#each previews as preview, index (preview.src)}
        <div
          in:scale={{ duration: 150 }}
          out:scale={{ duration: 100 }}
          class="post-form__preview-wrapper"
          on:click|preventDefault={e => removeFileAt(index)}>
          {#if preview.type.startsWith('audio/')}
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
          {:else}
            <picture title={preview.name}>
              <img
                class="post-form__preview post-form__preview_image"
                src={preview.src}
                alt="Preview" />
            </picture>
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
            on:change={handleFilesChange}
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
        title="Alt+B"
        on:click|preventDefault={e => insertMarkup('b')}>
        Tt
      </button>

      <button
        class="post-form__italic"
        title="Alt+I"
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
        title="Alt+T"
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
        title="Alt+C"
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
        title="Alt+P"
        on:click|preventDefault={e => insertMarkup('spoiler')}>
        Spoiler
      </button>

      <button
        class="post-form__color"
        on:click|preventDefault={e => insertColor()}>
        Color
      </button>

      <button
        class="post-form__pin {markupPinned ? 'post-form__pin_pinned' : ''}"
        on:click|preventDefault={toggleMarkupPinned} />
    </div>
  {/if}

  <div class="post-form__color-picker" />

  <div
    class="post-form__message-row"
    on:keydown={handleKeyDown}
    on:paste={handlePaste}>
    <div class="post-form__attachment-wrapper">
      <label class="post-form__attachment">
        <input
          bind:this={fileElement}
          type="file"
          bind:files={inputFiles}
          on:change={handleFilesChange}
          multiple
          hidden
          {disabled}
          title="Ctrl+B" />
      </label>
    </div>

    <TextBox
      className="post-form__message"
      name="message"
      bind:value={message}
      bind:this={messageElement}
      on:change={handleChange}
      {disabled} />

    <div class="post-form__submit-wrapper">
      <button class="post-form__submit" type="submit" title="Ctrl+Enter" />
    </div>
  </div>
</form>

<div class="post-form__right" />
