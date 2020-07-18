<script>
  import PostNotification from '../Notifications/PostNotification';
  import SystemNotification from '../Notifications/SystemNotification';
  import {
    PostNotification as PostNotificationModel,
    SystemNotification as SystemNotificationModel,
  } from '../../models/Notification';
  import Notifications from '../../stores/Notifications';
  import NotificationPopups from '../../stores/NotificationPopups';
  import { hideGallery, mediaBoxFiles, mediaBoxFile } from '../../stores/files';
  import { utils } from '../../utils';

  function handleFileClick(e, popup) {
    hideGallery();

    const files = popup.notification.files.filter(
      file => file.mimetype.startsWith('image/') || file.mimetype.startsWith('video/'),
    );
    mediaBoxFiles.set(files);

    const file = e.detail;
    mediaBoxFile.update(currentFile => {
      if (!currentFile) {
        return file;
      }

      return currentFile.id != file.id ? file : null;
    });
  }

  function handleClick(e, popup) {
    const post = document.getElementById(`post_${popup.notification.postId}`);
    if (post) {
      e.preventDefault();
      utils.scrollToElement(post);
      post.classList.add('post_highlight');
      setTimeout(() => post.classList.remove('post_highlight'), 500);
      return false;
    }
  }

  function handleClose(e, popup) {
    Notifications.read(popup.notification.id);
    NotificationPopups.remove(popup.id);
    if (popup.notification instanceof PostNotificationModel) {
      window.api.readNotification(popup.notification.id);
    }
  }
</script>

<div class="menu__messages">
  {#each $NotificationPopups as popup (popup.id)}
    {#if popup.notification instanceof PostNotificationModel}
      <PostNotification
        className="menu__message"
        notification={popup.notification}
        on:fileClick={e => handleFileClick(e, popup)}
        on:click={e => handleClick(e, popup)}
        on:close={e => handleClose(e, popup)} />
    {:else if popup.notification instanceof SystemNotificationModel}
      <SystemNotification
        className="menu__message"
        notification={popup.notification}
        on:close={e => handleClose(e, popup)} />
    {/if}
  {/each}
</div>
