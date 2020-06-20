<script>
  import { hslide } from "../anim";

  let isVisible = false;
  let name = localStorage.getItem("settings.name");
  let tripcode = localStorage.getItem("settings.tripcode");

  function handleNameChange(e) {
    localStorage["settings.name"] = e.target.value;
  }

  function handleTripcodeChange(e) {
    localStorage["settings.tripcode"] = e.target.value;
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

    <section class="menu__content" />

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
