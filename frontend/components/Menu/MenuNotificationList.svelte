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

  function handleFileClick(e, notification) {
    hideGallery();

    const files = notification.files.filter(
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

  function handleHover(e, notification) {
    if (!notification.isRead) {
      Notifications.read(notification.id);
      NotificationPopups.removeByNotificationId(notification.id);
      window.api.readNotification(notification.id);
    }
  }

  function handleClick(e, notification) {
    const post = document.getElementById(`post_${notification.postId}`);
    if (post) {
      e.preventDefault();
      utils.scrollToElement(post);
      post.classList.add('post_highlight');
      setTimeout(() => post.classList.remove('post_highlight'), 500);
      return false;
    }
  }

  function handleClose(e, notification) {
    Notifications.remove(notification.id);
    NotificationPopups.removeByNotificationId(notification.id);
    window.api.deleteNotification(notification.id);
  }
</script>

<section class="menu__content">
  {#each $Notifications as notification (notification.id)}
    <PostNotification
      className="menu__notification"
      {notification}
      on:fileClick={e => handleFileClick(e, notification)}
      on:hover={e => handleHover(e, notification)}
      on:click={e => handleClick(e, notification)}
      on:close={e => handleClose(e, notification)} />
  {/each}
</section>
