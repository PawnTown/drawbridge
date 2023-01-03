<script lang="ts">
  import AddRemote from "$lib/AddRemote.svelte";
  import RemoteList from "$lib/RemoteList/RemoteList.svelte";
  import TitleBar from "$lib/TitleBar.svelte";
  import type { Remote } from "../models/remote";
  import { onMount } from "svelte";
  import { GetStore } from "../services/storage";

  const store = GetStore();
  let remotes: Remote[] = [];

  onMount(() => {
		reloadRemotes();
	});

  const reloadRemotes =  async() => {
    remotes = (await store.get("remotes") ?? []);
    console.log(remotes);
  }

  let addRemoteVisible = false;
</script>

<TitleBar />

<div class="mainwrap">
  <RemoteList remotes={remotes} on:addPressed={() => addRemoteVisible = true} />
</div>

{#if addRemoteVisible}
  <AddRemote
    on:close={() => addRemoteVisible = false}
    on:success={() => { addRemoteVisible = false; reloadRemotes(); }}
  />
{/if}

<style>
</style>
