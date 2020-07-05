<script>
  import { createEventDispatcher } from "svelte";
  import { getEmbed } from "./embed";

  export let url = "";

  let embed = null;

  $: {
    if (url) {
      getEmbed(url).then(e => (embed = e));
    } else {
      embed = null;
    }
  }

  const dispatch = createEventDispatcher();

  function handleFileClick() {
    dispatch("fileClick", embed);
  }
</script>

<div class="post__embed">
  {#if embed}
    <a
      href={url}
      target="_blank"
      title={embed.name}
      on:click|preventDefault={handleFileClick}>
      <picture>
        <img
          class="post__embed-preview"
          src={embed.thumbnail_url}
          alt="Preview" />
      </picture>
    </a>
  {/if}
</div>
