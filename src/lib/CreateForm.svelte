<script lang="ts">
  import { save } from '@tauri-apps/api/dialog';
  import { invoke } from "@tauri-apps/api/tauri";

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

<div>
  <div class="column">
    <input id="url" placeholder="PawnTown CEC Url ..." bind:value={url} />
    <input id="engine" placeholder="Engine" bind:value={engine} />
    <input id="mode" placeholder="Mode" bind:value={mode} />
    <input id="token" placeholder="Token" bind:value={token} />
    <button on:click={saveUnixScript}>Create Executable</button>
  </div>
</div>
