<script>
  import { derived } from "svelte/store";

  import { markup } from "./markup";
  import { formatName } from "./post";
  import { hslide } from "../anim";
  import { mediaBoxFiles, mediaBoxFile } from "../stores/files";
  import {
    notifications,
    readNotification,
    newNotifications,
    removeNewNotification
  } from "../stores/notifications";
  import { utils } from "../utils";

  let isVisible = false;
  let name = localStorage.getItem("settings.name");
  let tripcode = localStorage.getItem("settings.tripcode");

  function handleNotificationHover(e, notification) {
    if (notification.read) {
      return;
    }

    readNotification(notification);
    window.api.readNotification(notification.id);
  }

  function handleNotificationClick(e, id) {
    const post = document.getElementById(`post_${id}`);
    if (post) {
      e.preventDefault();
      utils.scrollToElement(post);
      post.classList.add("post_highlight");
      setTimeout(() => post.classList.remove("post_highlight"), 500);
      return false;
    }
  }

  function handleNotiticationClose(e, notification) {
    readNotification(notification);
    window.api.readNotification(notification.id);
    removeNewNotification(notification);
  }

  function handleFileClick(post, file) {
    mediaBoxFiles.update(currentFile => {
      return currentFile !== file
        ? post.files.filter(
            file =>
              file.mimetype.startsWith("image/") ||
              file.mimetype.startsWith("video/")
          )
        : [];
    });

    mediaBoxFile.update(currentFile => {
      return currentFile !== file ? file : null;
    });
  }

  function handleNameChange(e) {
    localStorage["settings.name"] = e.target.value;
  }

  function handleTripcodeChange(e) {
    localStorage["settings.tripcode"] = e.target.value;
  }

  function formatDate(time) {
    const date = new Date(typeof time === "string" ? time + "Z" : time);
    const year = date.getFullYear();
    const month = (date.getMonth() + 1).toString().padStart(2, "0");

    const day = date
      .getDate()
      .toString()
      .padStart(2, "0");

    return `${day}.${month}.${year}`;
  }

  function formatTime(time) {
    const date = new Date(typeof time === "string" ? time + "Z" : time);

    const hours = date
      .getHours()
      .toString()
      .padStart(2, "0");

    const minutes = date
      .getMinutes()
      .toString()
      .padStart(2, "0");

    return `${hours}:${minutes}`;
  }

  function formatDateTime(time) {
    const date = formatDate(time);
    const dateNow = formatDate(Date.now());
    if (date === dateNow) {
      return formatTime(time);
    } else {
      return `${date} ${formatTime(time)}`;
    }
  }

  const unreadCount = derived(
    notifications,
    notifications => notifications.filter(n => !n.read).length
  );
</script>

{#if isVisible}
  <div class="menu__inner" transition:hslide={{ duration: 300 }}>
    <header class="menu__header">
      <h2 class="menu__title">Меню</h2>
      <button
        class="menu__hide"
        type="button"
        on:click|preventDefault={e => (isVisible = false)} />
    </header>

    <section class="menu__content">
      {#each $notifications as notification (notification.id)}
        <div
          class="menu__notification notification"
          on:mouseover={e => handleNotificationHover(e, notification)}
          on:click|preventDefault={e => handleNotificationClick(e, notification.post.id)}>
          <div class="notification__title">
            Ответ от
            <span class="notification__name">
              {formatName(notification.post)}
            </span>

            <span class="notification__tripcode">
              {notification.post.tripcode}
            </span>
          </div>

          <div class="notification__message">
            {@html markup(notification.post.message)}
          </div>

          {#if notification.post.files.length}
            <div class="notification__files">
              {#each notification.post.files as file (file.id)}
                {#if file.mimetype.startsWith('image/')}
                  <div
                    class="notification__file notification__file_image"
                    title={file.name}
                    on:click|preventDefault={e => handleFileClick(notification.post, file)}>
                    <picture>
                      <img
                        class="notification__file-preview"
                        src="/thumb/{file.md5}?max_width=360"
                        alt="Preview" />
                    </picture>
                  </div>
                {:else if file.mimetype.startsWith('video/')}
                  <div
                    class="notification__file notification__file_video"
                    title={file.name}
                    on:click|preventDefault={e => handleFileClick(notification.post, file)}>
                    <picture>
                      <img
                        class="notification__file-preview"
                        src="/thumb/{file.md5}?max_width=360"
                        alt="Preview" />
                    </picture>
                  </div>
                {:else if file.mimetype.startsWith('audio/')}
                  <div
                    class="notification__file notification__file_audio"
                    title={file.name} />
                {/if}
              {/each}
            </div>
          {/if}

          <div class="notification__footer">
            {#if notification.read}
              <span class="notification__read" />
            {:else}
              <span class="notification__unread" />
            {/if}
            <span class="notification__date">
              {formatDateTime(notification.post.created_at)}
            </span>
          </div>
        </div>
      {/each}
    </section>

    <footer class="menu__footer">
      <form class="menu__footer-inputs" autocomplete="off">
        <div class="menu__footer-input-wrapper">
          <input
            class="menu__footer-input"
            type="text"
            placeholder="Имя"
            name="name"
            autocomplete="off"
            on:change={handleNameChange}
            bind:value={name} />
        </div>

        <div class="menu__footer-input-wrapper">
          <input
            class="menu__footer-input"
            type="text"
            placeholder="Трипкод"
            name="tripcode"
            autocomplete="off"
            on:change={handleTripcodeChange}
            bind:value={tripcode} />
        </div>
      </form>
    </footer>
  </div>
{:else}
  <div class="menu__bar">
    <div class="menu__show-wrapper">
      <button
        class="menu__show"
        type="button"
        on:click|preventDefault={e => (isVisible = true)} />

      {#if $unreadCount > 0}
        <div class="menu__unread-count">{$unreadCount}</div>
      {/if}
    </div>
  </div>

  <div class="menu__messages">
    {#each $newNotifications as notification (notification.id)}
      <div
        class="menu__message notification"
        on:click|preventDefault={e => handleNotificationClick(e, notification.post.id)}>
        <button
          type="button"
          class="notification__close"
          on:click|preventDefault={e => handleNotiticationClose(e, notification)} />

        <div class="notification__title">
          Ответ от
          <span class="notification__name">
            {formatName(notification.post)}
          </span>

          <span class="notification__tripcode">
            {notification.post.tripcode}
          </span>
        </div>

        <div class="notification__message">
          {@html markup(notification.post.message)}
        </div>

        {#if notification.post.files.length}
          <div class="notification__files">
            {#each notification.post.files as file (file.id)}
              {#if file.mimetype.startsWith('image/')}
                <div
                  class="notification__file notification__file_image"
                  title={file.name}
                  on:click|preventDefault={e => handleFileClick(notification.post, file)}>
                  <picture>
                    <img
                      class="notification__file-preview"
                      src="/thumb/{file.md5}?max_width=360"
                      alt="Preview" />
                  </picture>
                </div>
              {:else if file.mimetype.startsWith('video/')}
                <div
                  class="notification__file notification__file_video"
                  title={file.name}
                  on:click|preventDefault={e => handleFileClick(notification.post, file)}>
                  <picture>
                    <img
                      class="notification__file-preview"
                      src="/thumb/{file.md5}?max_width=360"
                      alt="Preview" />
                  </picture>
                </div>
              {:else if file.mimetype.startsWith('audio/')}
                <div
                  class="notification__file notification__file_audio"
                  title={file.name} />
              {/if}
            {/each}
          </div>
        {/if}

        <div class="notification__footer">
          {#if notification.read}
            <span class="notification__read" />
          {:else}
            <span class="notification__unread" />
          {/if}
          <span class="notification__date">
            {formatDateTime(notification.post.created_at)}
          </span>
        </div>
      </div>
    {/each}
  </div>
{/if}
