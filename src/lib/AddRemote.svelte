<script lang="ts">
  import { GetStore } from '../services/storage';
  import { createEventDispatcher, onMount } from 'svelte';
  import { fly } from 'svelte/transition';
  import { v4 as uuidv4 } from 'uuid';

  import DriverSwitch from './DriverSwitch.svelte';
  import type { Remote } from '../models/remote';
  
  export let remote: Remote = {
    driver: "ptcec",
    label: "",
    url: "",
    engine: "",
    runCommand: "",
    mode: "",
    token: "",
  };
  
  const store = GetStore();
  let animateTabs = false;
  const dispatch = createEventDispatcher();

  const onClose = () => {
    animateTabs = false;
    dispatch("close");
  };

  onMount(() => {
    animateTabs = true;
  });

  const savePTCECRemote = async () => {
    const remotes: Remote[] | null = await store.get('remotes');
    const filteredRemotes = (remotes ?? []).filter((r: Remote) => !remote.id || r.id !== remote.id);
    const newRemote = {
      id: remote.id ?? uuidv4(),
      driver: "ptcec",
      label: remote.label,
      url: remote.url,
      engine: remote.engine,
      mode: remote.mode,
      token: remote.token,
    };
    const newRemotes = [...filteredRemotes, newRemote];
    await store.set("remotes", newRemotes);
    animateTabs = false;
    dispatch("success", { remote: newRemote });
  };

  const saveSSHRemote = async () => {
    const remotes: Remote[] | null = await store.get('remotes');
    const filteredRemotes = (remotes ?? []).filter((r: Remote) => !remote.id || r.id !== remote.id);
    const newRemote = {
      id: remote.id ?? uuidv4(),
      driver: "ssh",
      label: remote.label,
      url: remote.url,
      runCommand: remote.runCommand,
    };
    const newRemotes = [...filteredRemotes, newRemote];
    await store.set("remotes", newRemotes);
    animateTabs = false;
    dispatch("success", { remote: newRemote });
  };
</script>


<div
  class="dialog"
  out:fly="{{ y: 250, duration: 250, delay: 0 }}"
  in:fly="{{ y: 250, duration: 250 }}"
>
  <button class="cancel" on:click={onClose}></button>
  <div class="switch-wrap">
    <DriverSwitch bind:value={remote.driver} />
  </div>

  {#if remote.driver === "ptcec"}
    <div
      class="content"
      out:fly="{{ x: -250, duration: animateTabs ? 250 : 0, delay: 0 }}"
      in:fly="{{ x: -250, duration: animateTabs ? 250 : 0 }}"
    >
      <div class="input-wrap prefix-icon label">
        <span>Label</span>
        <input placeholder="My Awesome Cluster" bind:value={remote.label} />
      </div>
      <div class="input-wrap prefix-icon url">
        <span>Cluster URL</span>
        <input placeholder="https://cec[...].pawn.town:50051" bind:value={remote.url} />
      </div>
      <div class="input-wrap prefix-icon engine">
        <span>Engine</span>
        <input placeholder="stockfish" bind:value={remote.engine} />
      </div>
      <div class="input-wrap prefix-icon mode">
        <span>Mode</span>
        <input placeholder="cluster" bind:value={remote.mode} />
      </div>
      <div class="input-wrap prefix-icon key">
        <span>Token</span>
        <input placeholder="ABC123" bind:value={remote.token} />
      </div>
      <button class="submit" on:click={savePTCECRemote}>{ remote.id === "" ? "Add Remote" : "Save Remote" }</button>
    </div>
  {/if}

  {#if remote.driver === "ssh"}
    <div
      class="content"
      out:fly="{{ x: 250, duration: animateTabs ? 250 : 0, delay: 0 }}"
      in:fly="{{ x: 250, duration: animateTabs ? 250 : 0 }}"
    >
      <div class="input-wrap prefix-icon label">
        <span>Label</span>
        <input placeholder="My Awesome Cluster" bind:value={remote.label} />
      </div>
      <div class="input-wrap prefix-icon url">
        <span>Cluster URL</span>
        <input placeholder="root@0.0.0.0" bind:value={remote.url} />
      </div>
      <div class="input-wrap prefix-icon command">
        <span>Run Command</span>
        <input placeholder="mpi stockfish" bind:value={remote.runCommand} />
      </div>
      
      <button class="submit" on:click={saveSSHRemote}>{ remote.id === "" ? "Add Remote" : "Save Remote" }</button>
    </div>
  {/if}
</div>

<style>
  .dialog {
    border-radius: 12px 12px 0 0;
    position: absolute;
    background-color: #101113;
    top: 34px;
    bottom: 0;
    left: 0;
    right: 0;
    padding: 12px;
  }

  .switch-wrap {
    display: flex;
    justify-content: center;
    align-items: center;
    margin: 12px 0;
  }

  .cancel {
    position: absolute;
    top: 12px;
    right: 12px;
    background-color: transparent;
    background-image: url('/close.svg');
    background-size: contain;
    border: none;
    width: 24px;
    height: 24px;
    filter: invert(1) brightness(0.5);
    cursor: pointer;
  }

  .cancel:hover {
    filter: invert(1) brightness(1);
  }

  .content {
    position: absolute;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding-top: 24px;
    left: 0;
    right: 0;
  }

  input {
    width: 100%;
    box-sizing: border-box;
    background-color: #272b30;
    border: none;
    padding: 12px;
    border-radius: 9px;
    font-size: 14px;
    outline: none;
  }

  input::placeholder {
    color: #4c5257;
  }

  .input-wrap {
    position: relative;
    width: 340px;
    margin: 7px 0;
    padding-top: 24px;
  }

  .input-wrap span {
    position: absolute;
    display: block;
    left: 9px;
    top: 0;
    font-size: 14px;
    color: #62666b;
  }
  
  .prefix-icon input {
    padding-left: 42px;
  }

  .prefix-icon::after {
    position: absolute;
    left: 7px;
    top: 24px;
    bottom: 0;
    width: 26px;
    background-size: contain;
    background-repeat: no-repeat;
    background-position: center;
    content: '';
  }

  .prefix-icon.url::after {
    background-image: url('/link.svg');
  }

  .prefix-icon.engine::after {
    background-image: url('/bolt.svg');
  }

  .prefix-icon.mode::after {
    background-image: url('/layer.svg');
  }

  .prefix-icon.key::after {
    background-image: url('/key.svg');
  }

  .prefix-icon.label::after {
    background-image: url('/label.svg');
  }

  .prefix-icon.command::after {
    background-image: url('/return.svg');
  }

  button.submit {
    background-color: #0175ff;
    border: none;
    padding: 14px 32px;
    border-radius: 12px;
    margin: 24px 0 0 0;
    font-weight: bold;
    font-size: 12px;
    cursor: pointer;
  }

  button.submit:hover {
    background-color: #1d82ff;
  }

  button.submit:active {
    background-color: #1065cb;
  }
</style>