<script lang="ts">
  import AddRemote from "$lib/AddRemote.svelte";
  import RemoteList from "$lib/RemoteList/RemoteList.svelte";
  import TitleBar from "$lib/TitleBar.svelte";
  import type { Remote } from "../models/remote";
  import type { SettingsModel } from "../models/settings";
  import { onMount } from "svelte";
  import { GetStore } from "../services/storage";
  import RemoteDetails from "$lib/RemoteDetails.svelte";
  import SettingsToggle from "$lib/SettingsToggle.svelte";
  import Settings from "$lib/Settings.svelte";
  import Middleware from "$lib/Middleware.svelte";

  const store = GetStore();
  let remotes: Remote[] = [];
  let visibleDetailsItem: Remote | null = null;
  let remoteToEdit: Remote | null = null;
  let remoteToEditMiddleware: Remote | null = null;
  let settingsVisible: boolean = false;

  let settings: SettingsModel = {
    enableLogs: false,
    logFile: "log.txt",
    csCompilerPath: "",
  };

  onMount(() => {
    loadSettings();
		reloadRemotes();
	});

  const reloadRemotes =  async() => {
    remotes = (await store.get("remotes") ?? []);
  }

  const loadSettings = async () => {
    settings = (await store.get("settings") ?? {
      csCompilerPath: "",
    });
  }

  const saveSettings = async (newSettings: SettingsModel) => {
    await store.set("settings", newSettings);
    loadSettings();
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
  <div class="action">
    <SettingsToggle on:onPressed={() => settingsVisible = true} />
  </div>
</div>

{#if visibleDetailsItem}
  <RemoteDetails
    remote={visibleDetailsItem}
    on:close={() => visibleDetailsItem = null}
    on:editMiddleware={() => {remoteToEditMiddleware = visibleDetailsItem; visibleDetailsItem = null;}}
    on:edit={() => {remoteToEdit = visibleDetailsItem; visibleDetailsItem = null;}}
    on:delete={() => {visibleDetailsItem = null; reloadRemotes();}}
  />
{/if}

{#if remoteToEditMiddleware}
  <Middleware
    remote={remoteToEditMiddleware}
    on:close={() => remoteToEditMiddleware = null}
    on:success={(item) => {remoteToEditMiddleware = null; visibleDetailsItem = item.detail.remote; reloadRemotes();}}
  />
{/if}

{#if addRemoteVisible}
  <AddRemote
    on:close={() => addRemoteVisible = false}
    on:success={(item) => { addRemoteVisible = false; visibleDetailsItem = item.detail.remote; reloadRemotes(); }}
  />
{/if}

{#if remoteToEdit}
  <AddRemote
    remote={remoteToEdit}
    on:close={() => remoteToEdit = null}
    on:success={(item) => { remoteToEdit = null; visibleDetailsItem = item.detail.remote; reloadRemotes(); }}
  />
{/if}

{#if settingsVisible}
  <Settings
    settings={settings}
    on:close={() => settingsVisible = false}
    on:success={(e) => { settingsVisible = false; saveSettings(e.detail); }}
  />
{/if}

<style scoped>
  .action {
    position: absolute;
    bottom: 0;
    right: 0;
    padding: 10px;
  }
</style>
