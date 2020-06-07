<script>
  import Api from "../services/api";

  let name = "";
  let message = "";

  let messageElement;

  function handleSubmit(e) {
    Api.submitPost({ name, message }).then(() => {
      message = "";
      messageElement.focus();
    });
  }
</script>

<form
  method="POST"
  action="/api/v1/posts"
  enctype="application/x-www-form-urlencoded"
  on:submit|preventDefault={handleSubmit}>
  <input type="hidden" name="name" bind:value={name} hidden />

  <div class="post-form__message-row">
    <div class="post-form__attachment-wrapper">
      <button class="post-form__attachment" type="button" />
    </div>

    <textarea
      class="post-form__message"
      name="message"
      bind:value={message}
      bind:this={messageElement} />

    <div class="post-form__submit-wrapper">
      <button class="post-form__submit" type="submit" />
    </div>
  </div>
</form>
