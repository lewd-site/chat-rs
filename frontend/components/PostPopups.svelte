<script>
  import { derived } from 'svelte/store';

  import Post from './Post.svelte';
  import {
    popups,
    setPopupHoverById,
    setPopupPinnedById,
    checkPopupById
  } from '../stores/post_popups';
  import { posts } from '../stores/posts';

  export const values = derived([popups, posts], ([popups, posts]) => {
    return Object.values(popups).map(popup => ({
      popup,
      post: posts[popup.postId]
    }));
  });

  function getStyle(popup) {
    const styles = [];
    if (popup.bottomToTop) {
      styles.push(`bottom: ${document.body.offsetHeight - popup.top}px`);
    } else {
      styles.push(`top: ${popup.top}px`);
    }

    if (popup.rightToLeft) {
      styles.push(`right: ${document.body.offsetWidth - popup.left}px`);
    } else {
      styles.push(`left: ${popup.left}px`);
    }

    return styles.join('; ');
  }

  function getClass(popup) {
    const classes = ['post-popups__post', 'post'];
    if (popup.bottomToTop) {
      classes.push('post-popups__post_bottom-to-top');
    }

    if (popup.rightToLeft) {
      classes.push('post-popups__post_right-to-left');
    }

    if (popup.fade) {
      classes.push('post-popups__post_fade');
    }

    return classes.join(' ');
  }

  function handleMouseEnter(e, popup) {
    if (!e.target.hasAttribute('data-post-popup')) {
      return;
    }

    setPopupHoverById(popup.id, true);
  }

  function handleMouseLeave(e, popup) {
    if (!e.target.hasAttribute('data-post-popup')) {
      return;
    }

    setPopupHoverById(popup.id, false);
    checkPopupById(popup.id);
  }
</script>

{#each $values as value (value.popup.id)}
  <section
    style={getStyle(value.popup)}
    class={getClass(value.popup)}
    data-post-popup={value.popup.id}
    on:mouseenter={e => handleMouseEnter(e, value.popup)}
    on:mouseleave={e => handleMouseLeave(e, value.popup)}>
    {#if value.post}
      <Post post={value.post}>
        <button
          slot="after_header"
          type="button"
          class="post__pin {value.popup.pinned ? 'post__pin_pinned' : ''}"
          on:click|preventDefault={e => setPopupPinnedById(value.popup.id, !value.popup.pinned)} />
      </Post>
    {/if}
  </section>
{/each}
