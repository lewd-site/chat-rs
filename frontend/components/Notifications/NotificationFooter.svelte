<script>
  import { Notification } from '../../models/Notification';

  export let notification = null;
</script>

<div class="notification__footer">
  {#if notification.isRead}
    <span class="notification__read" />
  {:else}
    <span class="notification__unread" />
  {/if}

  <span class="notification__date">{Notification.formatDateTime(notification.createdAt)}</span>
</div>
