<script>
  import AddRemote from "$lib/AddRemote.svelte";
  import RemoteList from "$lib/RemoteList/RemoteList.svelte";
  import TitleBar from "$lib/TitleBar.svelte";

  let addRemoteVisible = false;
</script>

<TitleBar />

<div class="mainwrap">
  <RemoteList on:addPressed={() => addRemoteVisible = true} />
</div>

{#if addRemoteVisible}
  <AddRemote />
{/if}

<style>
</style>
