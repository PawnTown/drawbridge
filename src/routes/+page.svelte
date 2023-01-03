<script lang="ts">
  import AddRemote from "$lib/AddRemote.svelte";
  import RemoteList from "$lib/RemoteList/RemoteList.svelte";
  import TitleBar from "$lib/TitleBar.svelte";
  import type { Remote } from "../models/remote";
  import { onMount } from "svelte";
  import { GetStore } from "../services/storage";
    import RemoteDetails from "$lib/RemoteDetails.svelte";

  const store = GetStore();
  let remotes: Remote[] = [];
  let visibleDetailsItem: Remote | null = null;
  let remoteToEdit: Remote | null = null;

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
  <RemoteList
    remotes={remotes}
    on:addPressed={() => addRemoteVisible = true}
    on:itemPressed={(item) => visibleDetailsItem = item.detail.remote}
  />
</div>

{#if visibleDetailsItem}
  <RemoteDetails
    remote={visibleDetailsItem}
    on:close={() => visibleDetailsItem = null}
    on:edit={() => {remoteToEdit = visibleDetailsItem; visibleDetailsItem = null;}}
    on:delete={() => {visibleDetailsItem = null; reloadRemotes();}}
  />
{/if}

{#if addRemoteVisible}
  <AddRemote
    on:close={() => addRemoteVisible = false}
    on:success={() => { addRemoteVisible = false; reloadRemotes(); }}
  />
{/if}

{#if remoteToEdit}
  <AddRemote
    remote={remoteToEdit}
    on:close={() => remoteToEdit = null}
    on:success={() => { remoteToEdit = null; reloadRemotes(); }}
  />
{/if}

<style>
</style>
