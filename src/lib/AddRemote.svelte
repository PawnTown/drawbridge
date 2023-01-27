<script lang="ts">
  import { GetStore } from '../services/storage';
  import { createEventDispatcher, onMount } from 'svelte';
  import { fly } from 'svelte/transition';
  import { v4 as uuidv4 } from 'uuid';

  import DriverSwitch from './DriverSwitch.svelte';
  import { saveRemote, type Remote } from '../models/remote';
  import FileDrop from './FileDrop.svelte';

  export let remote: Remote = {
    driver: "ptcec",
    label: "",
    url: "",
    engine: "",
    runCommand: "",
    mode: "",
    token: "",
    privateKeyFile: "",
    middlewareLua: "",
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

  const save = async () => {
    const remotes: Remote[] = await store.get('remotes') ?? [];
    const newRemotes = saveRemote(remotes, remote);
    await store.set("remotes", newRemotes);
    animateTabs = false;
    dispatch("success", { remote: newRemotes[newRemotes.length - 1] });
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
        <input type="text" autocorrect="off" autocapitalize="none" placeholder="My Awesome Cluster" bind:value={remote.label} />
      </div>
      <div class="input-wrap prefix-icon url">
        <span>Cluster URL</span>
        <input type="url" autocorrect="off" autocapitalize="none" placeholder="https://cec[...].pawn.town:50051" bind:value={remote.url} />
      </div>
      <div class="input-wrap prefix-icon engine">
        <span>Engine</span>
        <input type="text" autocorrect="off" autocapitalize="none" placeholder="stockfish" bind:value={remote.engine} />
      </div>
      <div class="input-wrap prefix-icon mode">
        <span>Mode</span>
        <input type="text" autocorrect="off" autocapitalize="none" placeholder="cluster" bind:value={remote.mode} />
      </div>
      <div class="input-wrap prefix-icon token">
        <span>Token</span>
        <input type="text" autocorrect="off" autocapitalize="none" placeholder="ABC123" bind:value={remote.token} />
      </div>
      <button class="submit" on:click={save}>{ remote.id === "" ? "Add Remote" : "Save Remote" }</button>
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
        <input type="text" autocorrect="off" autocapitalize="none" placeholder="My Awesome Cluster" bind:value={remote.label} />
      </div>
      <div class="input-wrap prefix-icon url">
        <span>Cluster URL</span>
        <input type="url" autocorrect="off" autocapitalize="none" placeholder="root@0.0.0.0" bind:value={remote.url} />
      </div>
      <div class="input-wrap">
        <span>Private Key File</span>
        <FileDrop bind:value={remote.privateKeyFile} />
      </div>
      <div class="input-wrap prefix-icon command">
        <span>Run Command</span>
        <input type="text" autocorrect="off" autocapitalize="none" placeholder="mpi stockfish" bind:value={remote.runCommand} />
      </div>
      
      <button class="submit" on:click={save}>{ remote.id === "" ? "Add Remote" : "Save Remote" }</button>
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

  .prefix-icon.token::after {
    background-image: url('/token.svg');
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