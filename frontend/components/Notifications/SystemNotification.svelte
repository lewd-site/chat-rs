<script>
  import { createEventDispatcher } from 'svelte';
  import NotificationFooter from './NotificationFooter';
  import { hslide } from '../../anim';
  import { Notification } from '../../models/Notification';

  const TRANSITION_DIRATION = 150;

  export let className = null;
  export let notification = null;

  $: _className = [className, 'notification', 'notification_type_system'].filter(i => i).join(' ');

  const dispatch = createEventDispatcher();

  function handleClose() {
    dispatch('close');
  }
</script>

<div class={_className} transition:hslide={{ duration: TRANSITION_DIRATION }}>
  <button type="button" class="notification__close" on:click|preventDefault={handleClose} />

  <div class="notification__message">
    {@html notification.message}
  </div>

  <NotificationFooter {notification} />
</div>
