<script>
  import { createEventDispatcher } from 'svelte';
  import NotificationFooter from './NotificationFooter';
  import PostNotificationFile from './PostNotificationFile';
  import { markup } from '../markup';
  import { hslide } from '../../anim';
  import { Notification } from '../../models/Notification';

  const TRANSITION_DIRATION = 150;

  export let className = null;
  export let notification = null;

  $: _className = [className, 'notification', 'notification_type_post'].filter(i => i).join(' ');

  const dispatch = createEventDispatcher();

  function handleHover() {
    dispatch('hover');
  }

  function handleClick() {
    dispatch('click');
  }

  function handleFileClick(file) {
    dispatch('fileClick', file);
  }

  function handleClose() {
    dispatch('close');
  }
</script>

<div
  class={_className}
  transition:hslide={{ duration: TRANSITION_DIRATION }}
  on:mouseover={handleHover}
  on:click|preventDefault={handleClick}>
  <button type="button" class="notification__close" on:click|preventDefault={handleClose} />

  <div class="notification__title">
    Ответ от
    <span class="notification__name">{notification.name}</span>
    <span class="notification__tripcode">{notification.tripcode}</span>
  </div>

  <div class="notification__message">
    {@html markup(notification.message)}
  </div>

  {#if notification.files.length}
    <div class="notification__files">
      {#each notification.files as file (file.id)}
        <PostNotificationFile {file} on:click={e => handleFileClick(file)} />
      {/each}
    </div>
  {/if}

  <NotificationFooter {notification} />
</div>
