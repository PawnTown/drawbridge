<script lang="ts">
  import { save } from '@tauri-apps/api/dialog';
  import { invoke } from "@tauri-apps/api/tauri";
  import { fly, fade } from 'svelte/transition';
  import { createEventDispatcher } from 'svelte';
  import type { Remote } from '../models/remote';

  export let remote: Remote;

  const dispatch = createEventDispatcher();

  const onClose = () => {
      dispatch("close");
  };


  async function saveUnixScript() {
    const output = await save({
      filters: [{
        name: 'Image',
        extensions: ["", ".sh"]
      }]
    });

    if (remote.driver === "PTCEC") {
      const success = await invoke("create_ptcec_unix_script", {
        output,
        url: remote.url,
        engine: remote.engine,
        mode: remote.mode,
        token: remote.token,
      });
    }
  }
</script>
  
<!-- svelte-ignore a11y-click-events-have-key-events -->
<div
  class="backdrop"
  out:fade="{{ duration: 250 }}"
  in:fade="{{ duration: 250 }}"
  on:click={onClose}
></div>

<div
  class="dialog"
  out:fly="{{ y: 250, duration: 250, delay: 0 }}"
  in:fly="{{ y: 250, duration: 250 }}"
>
  <button class="cancel" on:click={onClose}></button>
  
  <div class="inner-wrap">
    <div class="info">
      <div class="data">
        <span>Label</span>
        <p>{remote.label}</p>
      </div>
    
      <div class="data">
        <span>URL</span>
        <p title={remote.url}>{remote.url}</p>
      </div>

      {#if remote.driver === "PTCEC"}
        <div class="data">
          <span>Driver</span>
          <p title="PTCEC">PTCEC</p>
        </div>
        <div class="data">
          <span>Engine</span>
          <p title={remote.engine}>{remote.engine}</p>
        </div>
        <div class="data">
          <span>Mode</span>
          <p title={remote.mode}>{remote.mode}</p>
        </div>
        <div class="data">
          <span>Token</span>
          <p title={"***"}>***</p>
        </div>
      {/if}
    </div>
    <div class="actions">
      <div>
        <button class="shortcut" on:click={saveUnixScript}>
          <div class="icon magic"></div>
          Create Shortcut
        </button>
        <button>
          <div class="icon edit"></div>
          Edit Details
        </button>
      </div>
      <button class="remove">Remove</button>
    </div>
  </div>
</div>

<style>
  .backdrop {
    position: absolute;
    background-color: rgba(0, 0, 0, 0.84);
    top: 34px;
    bottom: 0;
    left: 0;
    right: 0;
  }

  .dialog {
    border-radius: 12px 12px 0 0;
    position: absolute;
    background-color: #101113;
    bottom: 0;
    left: 0;
    right: 0;
    height: 390px;
    padding: 12px;
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

  .data {
    text-align: left;
    margin-bottom: 12px;
  }

  .data span {
    color: #8e8e8e;
    font-size: 10px;
    font-weight: 500;
    letter-spacing: 0.5px;
    text-transform: uppercase;
  }

  .data p {
    color: #8e8e8e;
    font-size: 14px;
    margin: 0;
    font-weight: bold;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .inner-wrap {
    display: flex;
    justify-content: space-between;
    height: 100%;
  }

  .inner-wrap .info {
    width: 64%;
    padding: 24px 7px 0 7px;
  }

  .inner-wrap .actions {
    width: 30%;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    padding: 32px 0 2px 0; 
  }

  .inner-wrap .actions button {
    background-color: #2b2d2e;
    border: none;
    padding: 14px 32px;
    border-radius: 12px;
    margin: 0 0 9px 0;
    font-weight: bold;
    font-size: 12px;
    cursor: pointer;
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .inner-wrap .actions button.shortcut {
    background-color: #0175ff;
  }

  .inner-wrap .actions button.remove {
    background-color: #c33737;
  }

  .inner-wrap .actions button .icon {
    width: 32px;
    height: 32px;
    background-size: contain;
    background-repeat: no-repeat;
    background-position: center;
    margin-bottom: 12px;
  }

  .inner-wrap .actions button .icon.magic {
    background-image: url('/magic.svg');
    filter: invert(1);
  }

  .inner-wrap .actions button .icon.edit {
    background-image: url('/edit.svg');
    filter: invert(1);
  }
</style>