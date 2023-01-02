<script>
  import { fly } from 'svelte/transition';
  import { save } from '@tauri-apps/api/dialog';
  import { invoke } from "@tauri-apps/api/tauri";
  import DriverSwitch from './DriverSwitch.svelte';

  let driver = "";

  let url = "";
  let engine = "";
  let mode = "";
  let token = "";

  async function saveUnixScript() {
    const output = await save({
      filters: [{
        name: 'Image',
        extensions: ["", ".sh"]
      }]
    });

    const success = await invoke("create_ptcec_unix_script", {
      output,
      url,
      engine,
      mode,
      token,
    });
  }
</script>


<div
  class="dialog"
  out:fly="{{ y: 250, duration: 250, delay: 0 }}"
  in:fly="{{ y: 250, duration: 250 }}"
>
  <div class="switch-wrap">
    <DriverSwitch bind:value={driver} />
  </div>
  <div class="content">
    <input id="url" placeholder="PawnTown CEC Url ..." bind:value={url} />
    <input id="engine" placeholder="Engine" bind:value={engine} />
    <input id="mode" placeholder="Mode" bind:value={mode} />
    <input id="token" placeholder="Token" bind:value={token} />
    <button on:click={saveUnixScript}>Create Executable</button>
  </div>
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

  .content {
    
  }
</style>