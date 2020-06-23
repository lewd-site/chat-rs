<script>
  import AudioPlayer from "./AudioPlayer.svelte";
  import { markup } from "./markup";
  import { formatName } from "./post";
  import { userUuid } from "../stores/auth";
  import { mediaBoxFiles, mediaBoxFile } from "../stores/files";
  import {
    hasPopup,
    addPopup,
    setPopupHover,
    checkPopup
  } from "../stores/post_popups";
  import { posts } from "../stores/posts";

  export let post;

  const POPUP_OPEN_TIME = 100;

  let _posts = $posts;

  function formatTime(time) {
    const date = new Date(time + "Z");
    const year = date.getFullYear();
    const month = (date.getMonth() + 1).toString().padStart(2, "0");

    const day = date
      .getDate()
      .toString()
      .padStart(2, "0");

    const hours = date
      .getHours()
      .toString()
      .padStart(2, "0");

    const minutes = date
      .getMinutes()
      .toString()
      .padStart(2, "0");

    const seconds = date
      .getSeconds()
      .toString()
      .padStart(2, "0");

    return `${hours}:${minutes}:${seconds} ${day}.${month}.${year}`;
  }

  function getFilesClass(files) {
    const classes = ["post__files", `post__files_layout-${files.length}`];
    if (files.length === 2) {
      if (files[0].width + files[1].width < files[0].height + files[1].height) {
        classes.push("post__files_vertical");
      } else {
        classes.push("post__files_horizontal");
      }
    } else if (files.length > 0) {
      if (post.files[0].width < post.files[0].height) {
        classes.push("post__files_vertical");
      } else {
        classes.push("post__files_horizontal");
      }
    }

    return classes.join(" ");
  }

  function handleFileClick(file) {
    mediaBoxFiles.update(currentFiles => {
      return post.files.filter(
        file =>
          file.mimetype.startsWith("image/") ||
          file.mimetype.startsWith("video/")
      );
    });

    mediaBoxFile.update(currentFile => {
      if (!currentFile) {
        return file;
      }

      return +currentFile.id !== +file.id ? file : null;
    });
  }

  let showPopupTimeout = null;

  function handleMouseOver(e) {
    let target = e.target.closest("[data-show-post-popup]");
    if (!target) {
      return;
    }

    if (hasPopup(target)) {
      setPopupHover(target, true);
    } else {
      const parentPopup = target.closest("[data-post-popup]");
      const layoutBox = document.body.getBoundingClientRect();
      const top = e.clientY - layoutBox.top;
      const left = e.clientX - layoutBox.left;
      const bottomToTop = e.clientY > window.innerHeight / 2;
      const rightToLeft = e.clientX > window.innerWidth / 2;

      showPopupTimeout = setTimeout(() => {
        addPopup(
          target,
          parentPopup ? +parentPopup.getAttribute("data-post-popup") : null,
          +target.getAttribute("data-show-post-popup"),
          top,
          left,
          bottomToTop,
          rightToLeft
        );
      }, POPUP_OPEN_TIME);
    }
  }

  function handleMouseOut(e) {
    let target = e.target.closest("[data-show-post-popup]");
    if (!target) {
      return;
    }

    if (showPopupTimeout) {
      clearTimeout(showPopupTimeout);
      showPopupTimeout = null;
    }

    setPopupHover(target, false);
    checkPopup(target);
  }

  function handleReplyClick(e) {
    window.eventBus.dispatch("reply", post.id);
  }

  function getImageFiles(files) {
    return files.filter(file => file.mimetype.startsWith("image/"));
  }
</script>

<div class="post__header">
  <span class="post__name">{formatName(post)}</span>
  <span class="post__tripcode">{post.tripcode}</span>
  <span class="post__id">{post.id}</span>
  <time class="post__date" datetime={post.created_at}>
    {formatTime(post.created_at)}
  </time>
  {#if post.user_uuid === null || post.user_uuid !== $userUuid}
    <button class="post__reply" on:click|preventDefault={handleReplyClick} />
  {/if}
  <slot name="after_header" />
</div>

<div
  class="post__content"
  on:mouseover={handleMouseOver}
  on:mouseout={handleMouseOut}>
  <div class={getFilesClass(getImageFiles(post.files))}>
    {#each getImageFiles(post.files) as file (file.id)}
      <div class="post__file">
        <a
          href="/src/{file.md5}.{file.extension}"
          target="_blank"
          title={file.name}
          on:click|preventDefault={e => handleFileClick(file)}>
          <picture>
            <img
              class="post__file-preview"
              src="/thumb/{file.md5}?max_width=360"
              alt="Preview" />
          </picture>
        </a>
      </div>
    {/each}
  </div>

  <div class="post__message">
    {@html markup(post.message)}
  </div>

  <div class="post__videos">
    {#each post.files.filter(file =>
      file.mimetype.startsWith('video/')
    ) as file (file.id)}
      <div class="post__video">
        <a
          href="/src/{file.md5}.{file.extension}"
          target="_blank"
          title={file.name}
          on:click|preventDefault={e => handleFileClick(file)}>
          <picture>
            <img
              class="post__video-preview"
              src="/thumb/{file.md5}?max_width=360"
              alt="Preview" />
          </picture>
        </a>
      </div>
    {/each}
  </div>

  <div class="post__audios">
    {#each post.files.filter(file =>
      file.mimetype.startsWith('audio/')
    ) as file (file.id)}
      <AudioPlayer className="post__audio" {file} />
    {/each}
  </div>
</div>

<div
  class="post__footer"
  on:mouseover={handleMouseOver}
  on:mouseout={handleMouseOut}>
  {#if post.reply_from}
    {#each post.reply_from as id (id)}
      <a
        class="post__footer-reflink reflink"
        href="#post_{id}"
        data-ref-link={id}
        data-show-post-popup={id}>
        {id}
        {#if $posts[id]}
          <span class="reflink__author">
            <span class="reflink__name">{formatName($posts[id])}</span>
            <span class="reflink__tripcode">{$posts[id].tripcode}</span>
          </span>
        {/if}
      </a>
    {/each}
  {/if}
</div>
