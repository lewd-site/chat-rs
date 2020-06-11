<script>
  import { onMount } from "svelte";

  export let post;

  function getName(name) {
    if (!name || !name.length) {
      return "Anonymous";
    }

    return name;
  }

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

  function getThumbWidth(file) {
    const scale = 250 / Math.max(250, file.width, file.height);
    return Math.round(file.width * scale);
  }

  function getThumbHeight(file) {
    const scale = 250 / Math.max(250, file.width, file.height);
    return Math.round(file.height * scale);
  }
</script>

<div class="post__header">
  <span class="post__name">{getName(post.name)}</span>
  <span class="post__tripcode">{post.tripcode}</span>
  <span class="post__id">{post.id}</span>
  <time class="post__date" datetime={post.created_at}>
    {formatTime(post.created_at)}
  </time>
</div>

<div class="post__content">
  <div class={getFilesClass(post.files)}>
    {#each post.files as file (file.id)}
      <div class="post__file">
        <a href="/src/{file.md5}.{file.extension}" target="_blank">
          <picture>
            <img
              class="post__file-preview"
              src="/src/{file.md5}.{file.extension}"
              alt="Preview" />
          </picture>
        </a>
      </div>
    {/each}
  </div>

  <div class="post__message">{post.message}</div>
</div>

<div class="post__footer" />
