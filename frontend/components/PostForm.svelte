<script>
  import Api from "../services/api";

  let name = "";
  let message = "";
  let files = [];
  let previews = [];

  let inputFiles = null;
  let messageElement;

  function setFiles(e) {
    if (inputFiles && inputFiles.length) {
      files = [...inputFiles].slice(0, 5);
    } else {
      files = [];
    }

    updatePreviews();
  }

  function removeFileAt(index) {
    files.splice(index, 1);

    updatePreviews();
  }

  function updatePreviews(e) {
    previews.forEach(preview => URL.revokeObjectURL(preview));
    previews = files.map(file => URL.createObjectURL(file));
  }

  function handleSubmit(e) {
    Api.submitPost({ name, message, files }).then(() => {
      message = "";
      files = [];
      updatePreviews();
      messageElement.focus();
    });
  }
</script>

<form
  method="POST"
  action="/api/v1/posts"
  enctype="multipart/form-data"
  on:submit|preventDefault={handleSubmit}>
  <input type="hidden" name="name" bind:value={name} hidden />

  <div class="post-form__previews-row">
    {#each previews as preview, index (preview)}
      <picture>
        <img
          class="post-form__preview"
          src={preview}
          alt="Preview"
          on:click|preventDefault={e => removeFileAt(index)} />
      </picture>
    {/each}
  </div>

  <div class="post-form__message-row">
    <div class="post-form__attachment-wrapper">
      <label class="post-form__attachment">
        <input
          type="file"
          name="file"
          bind:files={inputFiles}
          on:change={setFiles}
          multiple
          hidden />
      </label>
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
