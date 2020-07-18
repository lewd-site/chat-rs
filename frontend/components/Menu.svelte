<script>
  import { derived } from 'svelte/store';
  import { formatFileSize } from './file';
  import { markup } from './markup';
  import { formatName, trimStart, extractReplies } from './post';
  import { hslide } from '../anim';
  import MenuNotificationList from './Menu/MenuNotificationList';
  import MenuNotificationPopupList from './Menu/MenuNotificationPopupList';
  import { userUuid } from '../stores/auth';
  import {
    mediaBoxFiles,
    mediaBoxFile,
    nsfwMode,
    setNSFWMode,
    hideGallery,
    showOriginalFiles,
  } from '../stores/files';
  import Notifications from '../stores/Notifications';
  import NotificationPopups from '../stores/NotificationPopups';
  import { posts } from '../stores/posts';
  import { utils } from '../utils';

  let isVisible = false;
  let tab = 'main';
  let name = localStorage.getItem('settings.name');
  let tripcode = localStorage.getItem('settings.tripcode');

  function handleNameChange(e) {
    localStorage['settings.name'] = e.target.value;
  }

  function handleTripcodeChange(e) {
    localStorage['settings.tripcode'] = e.target.value;
  }

  function formatDate(time) {
    const date = new Date(typeof time === 'string' ? time + 'Z' : time);
    const year = date.getFullYear();
    const month = (date.getMonth() + 1).toString().padStart(2, '0');

    const day = date
      .getDate()
      .toString()
      .padStart(2, '0');

    return `${day}.${month}.${year}`;
  }

  function formatTime(time) {
    const date = new Date(typeof time === 'string' ? time + 'Z' : time);

    const hours = date
      .getHours()
      .toString()
      .padStart(2, '0');

    const minutes = date
      .getMinutes()
      .toString()
      .padStart(2, '0');

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
    Notifications,
    Notifications => Notifications.filter(n => !n.isRead).length,
  );

  let originalTitle = null;
  unreadCount.subscribe(count => {
    if (originalTitle === null) {
      originalTitle = document.title;
    }

    if (count > 0) {
      document.title = `${originalTitle} [${count}]`;
    } else {
      document.title = originalTitle;
    }
  });
</script>

{#if isVisible}
  <div class="menu__inner" transition:hslide={{ duration: 300 }}>
    {#if tab === 'main'}
      <header class="menu__header">
        <h2 class="menu__title">Меню</h2>

        <button
          class="menu__hide"
          type="button"
          on:click|preventDefault={e => (isVisible = false)} />
      </header>

      <MenuNotificationList />
    {:else if tab === 'settings'}
      <header class="menu__header">
        <button
          type="button"
          class="menu__header-back"
          on:click|preventDefault={e => (tab = 'main')} />

        <h2 class="menu__title">Настройки</h2>
      </header>

      <section class="menu__content">
        <label class="menu__checkbox">
          <input
            type="checkbox"
            checked={$nsfwMode}
            on:change={e => setNSFWMode(e.target.checked)}
            hidden />
          <span class="menu__checkbox-label">NSFW-режим</span>
          <div class="menu__checkbox-mark" />
        </label>

        <label class="menu__checkbox">
          <input
            type="checkbox"
            checked={['gif', 'all'].indexOf($showOriginalFiles) !== -1}
            on:change={e => showOriginalFiles.set(e.target.checked ? 'gif' : 'none')}
            hidden />
          <span class="menu__checkbox-label">Заменять превью GIF оригиналами</span>
          <div class="menu__checkbox-mark" />
        </label>

        <label class="menu__checkbox">
          <input
            type="checkbox"
            checked={$showOriginalFiles === 'all'}
            on:change={e => showOriginalFiles.set(e.target.checked ? 'all' : 'none')}
            hidden />
          <span class="menu__checkbox-label">Заменять все превью оригиналами</span>
          <div class="menu__checkbox-mark" />
        </label>
      </section>
    {/if}

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
            bind:value={name}
            maxlength="255" />
        </div>

        <div class="menu__footer-input-wrapper">
          <input
            class="menu__footer-input"
            type="text"
            placeholder="Трипкод"
            name="tripcode"
            autocomplete="off"
            on:change={handleTripcodeChange}
            bind:value={tripcode}
            maxlength="255" />
        </div>
      </form>

      <div class="menu__footer-buttons">
        <button
          type="button"
          class="menu__footer-settings"
          on:click|preventDefault={e => (tab = 'settings')} />
      </div>
    </footer>
  </div>
{:else}
  <div class="menu__bar">
    <div class="menu__show-wrapper">
      <button class="menu__show" type="button" on:click|preventDefault={e => (isVisible = true)} />

      {#if $unreadCount > 0}
        <div class="menu__unread-count">{$unreadCount}</div>
      {/if}
    </div>
  </div>

  <MenuNotificationPopupList />
{/if}
