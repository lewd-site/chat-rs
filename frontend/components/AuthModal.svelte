<script>
  import { SystemNotification } from '../models/Notification';
  import { showAuthModal, token } from '../stores/auth';
  import NotificationPopups from '../stores/NotificationPopups';

  let email = '';
  let password = '';
  let error = '';

  function handleEmailChange(e) {
    localStorage['auth_email'] = e.target.value;
  }

  function handleCancel(e) {
    showAuthModal.set(false);
    error = '';
    window.eventBus.dispatch('authmodal_canceled');
  }

  async function handleSubmit(e) {
    const authButton = document.getElementById('login');

    try {
      token.set(await window.sso.loginByEmail(email, password));
      showAuthModal.set(false);
      error = '';

      authButton.setAttribute('hidden', '');
      window.eventBus.dispatch('authmodal_submitted');

      const notification = new SystemNotification(`Вы вошли как ${email}`, new Date());
      NotificationPopups.add(notification);
    } catch (e) {
      error = e;

      authButton.removeAttribute('hidden');
    }
  }
</script>

{#if $showAuthModal}
  <form class="auth-modal__content" on:submit|preventDefault={handleSubmit}>
    <input
      class="auth-modal__email"
      type="email"
      placeholder="E-Mail"
      bind:value={email}
      on:change={handleEmailChange} />

    <input
      class="auth-modal__password"
      type="password"
      placeholder="Пароль"
      bind:value={password} />

    {#if error}
      <p class="auth-modal__error">{error}</p>
    {/if}

    <div class="auth-modal__actions">
      <button type="button" class="auth-modal__cancel" on:click|preventDefault={handleCancel}>
        Отмена
      </button>

      <button type="submit" class="auth-modal__submit">Отправить</button>
    </div>
  </form>
{/if}
