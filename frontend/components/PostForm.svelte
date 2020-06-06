<script>

</script>

<form method="POST" action="/api/v1/posts" enctype="application/x-www-form-urlencoded">
  <input type="hidden" name="name" value="" hidden />

  <div class="post-form__message-row">
    <div class="post-form__attachment-wrapper">
      <button class="post-form__attachment" type="button" />
    </div>

    <textarea class="post-form__message" name="message" />

    <div class="post-form__submit-wrapper">
      <button class="post-form__submit" type="submit" />
    </div>
  </div>
</form>
