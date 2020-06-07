<script>
  import { derived } from "svelte/store";

  import { posts } from "../stores.ts";

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

  export const postValues = derived(posts, posts => Object.values(posts));
</script>

{#each $postValues as post (post.id)}
  <section class="post-list__post post">
    <div class="post__header">
      <span class="post__name">{getName(post.name)}</span>
      <span class="post__tripcode">{post.tripcode}</span>
      <span class="post__id">{post.id}</span>
      <time class="post__date" datetime={post.created_at}>
        {formatTime(post.created_at)}
      </time>
    </div>

    <div class="post__content">{post.message}</div>

    <div class="post__footer" />
  </section>
{/each}
