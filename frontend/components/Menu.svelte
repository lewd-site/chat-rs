<script>
  import { derived } from "svelte/store";

  import { hslide } from "../anim";
  import { notifications } from "../stores/notifications";

  let isVisible = false;
  let name = localStorage.getItem("settings.name");
  let tripcode = localStorage.getItem("settings.tripcode");

  function handleNameChange(e) {
    localStorage["settings.name"] = e.target.value;
  }

  function handleTripcodeChange(e) {
    localStorage["settings.tripcode"] = e.target.value;
  }

  export const notificationValues = derived(notifications, notifications =>
    Object.values(notifications)
  );

  function formatName(post) {
    if (!post.name && !post.tripcode) {
      return "Anonymous";
    }

    return post.name;
  }

  function formatTime(post) {
    const date = new Date(post.created_at);
    const h = date.getHours();
    const m = date.getMinutes();
    const _h = h.toString().padStart(2, "0");
    const _m = m.toString().padStart(2, "0");

    return `${_h}:${_m}`;
  }
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
      {#each $notificationValues as notification (notification.id)}
        <div class="menu__notification notification">
          <div class="notification__title">
            Ответ от
            <span class="notification__name">
              {formatName(notification.post.name)}
            </span>

            <span class="notification__tripcode">
              {notification.post.tripcode}
            </span>
          </div>

          <div class="notification__message">
            {notification.post.message_raw}
          </div>

          <div
            class="notification__footer"
            title={notification.post.created_at}>
            {formatTime(notification.post)}
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
    <button
      class="menu__show"
      type="button"
      on:click|preventDefault={e => (isVisible = true)} />
  </div>
{/if}
