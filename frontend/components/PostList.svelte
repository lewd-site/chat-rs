<script>
  import { derived } from "svelte/store";

  import Post from "./Post.svelte";
  import { posts } from "../stores/posts";

  const postValues = derived(posts, posts => Object.values(posts));
</script>

{#each $postValues as post (post.id)}
  <section id={`post_${post.id}`} class="post-list__post post">
    <Post {post} />
  </section>
{/each}
