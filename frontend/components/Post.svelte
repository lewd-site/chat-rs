<script>
  import { onMount } from "svelte";

  export let post;

  function getName(name, tripcode) {
    if ((!name || !name.length) && (!tripcode || !tripcode.length)) {
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

  function escapeHtml(input) {
    return input
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;")
      .replace(/"/g, "&quot;")
      .replace(/'/g, "&#039;");
  }

  function getSegmentMarkup(segment) {
    let markup = escapeHtml(segment.text);
    segment.tags.forEach(tag => {
      switch (tag.type) {
        case "Bold":
          markup = `<strong class="markup markup_bold">${markup}</strong>`;
          break;

        case "Italic":
          markup = `<em class="markup markup_italic">${markup}</em>`;
          break;

        case "Underline":
          markup = `<span class="markup markup_underline">${markup}</span>`;
          break;

        case "Strike":
          markup = `<del class="markup markup_strike">${markup}</del>`;
          break;

        case "Superscript":
          markup = `<sup class="markup markup_superscript">${markup}</sup>`;
          break;

        case "Subscript":
          markup = `<sub class="markup markup_subscript">${markup}</sub>`;
          break;

        case "Code":
          markup = `<pre class="markup markup_code">${markup}</pre>`;
          break;

        case "CodeBlock":
          markup = `<pre class="markup markup_codeblock">${markup}</pre>`;
          break;

        case "Spoiler":
          markup = `<span class="markup markup_spoiler">${markup}</span>`;
          break;

        case "RefLink":
          markup = `<a class="markup markup_reflink" href="#post_${tag.id}">${markup}</a>`;
          break;

        case "Quote":
          markup = `<span class="markup markup_quote">${markup}</span>`;
          break;
      }
    });

    return markup;
  }

  function getMarkup(segments) {
    const markup = [];
    for (let segment of segments) {
      markup.push(getSegmentMarkup(segment));
    }

    return markup.join("");
  }
</script>

<div class="post__header">
  <span class="post__name">{getName(post.name, post.tripcode)}</span>
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
              src="/thumb/{file.md5}?max_width=360"
              alt="Preview" />
          </picture>
        </a>
      </div>
    {/each}
  </div>

  <div class="post__message">
    {@html getMarkup(post.message)}
  </div>
</div>

<div class="post__footer" />
