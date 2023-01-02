<script>
  import CreateForm from "$lib/CreateForm.svelte";
</script>

<div class="row">
  <a href="https://pawn.town" target="_blank">
    <img src="/pawntown_logo.png" class="logo" />
  </a>
</div>

<h1>PawnTown DrawBridge</h1>

<div class="mainwrap">
  <CreateForm />
</div>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>
