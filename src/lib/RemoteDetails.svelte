<script lang="ts">
  import { save } from '@tauri-apps/api/dialog';
  import { invoke } from "@tauri-apps/api/tauri";
  import { fly, fade } from 'svelte/transition';
  import { createEventDispatcher } from 'svelte';
  import type { Remote } from '../models/remote';
  import { GetStore } from '../services/storage';

  const store = GetStore();

  export let remote: Remote;

  let deleteUIVisible = false;

  const dispatch = createEventDispatcher();

  const onMiddlewareEdit = () => {
      dispatch("editMiddleware");
  };

  const onEdit = () => {
      dispatch("edit");
  };

  const onClose = () => {
      dispatch("close");
  };

  const onDelete = async () => {
    let newRemotes : Remote[] | null = await store.get('remotes');
    await store.set("remotes", (newRemotes ?? []).filter((r: Remote) => r.id !== remote.id));
    dispatch("delete");
  };

  async function saveUnixScript() {
    let os: string = await invoke("get_os");

    const output = await save({
      filters: os === "win" ? [
        { name: 'Executable', extensions: ['exe'] },
        { name: 'Shortcut', extensions: ['lnk'] }
      ] : [
        { name: 'Bash-Script', extensions: ['sh'] },
      ]
    });

    let result : boolean = await invoke("create_shortcut", {
      output,
      id: remote.id,
    });

    // todo: success or error message
  }
</script>
  
<!-- svelte-ignore a11y-click-events-have-key-events -->
<div
  class="backdrop"
  out:fade="{{ duration: 250 }}"
  in:fade="{{ duration: 250 }}"
  on:click={onClose}
></div>

{#if deleteUIVisible}
<div
  class="dialog delete"
  out:fly="{{ y: 250, duration: 250, delay: 0 }}"
  in:fly="{{ y: 250, duration: 250 }}"
> 
  <div class="confirm-text">
    <h2>Are you sure you want to delete?</h2>
    <p>If you confirm this action, the remote will be deleted permanently.</p>
  </div>
  <div class="confirm-actions">
    <button class="cancel-delete" on:click={() => deleteUIVisible = false}>
      Cancel
    </button>
    <button class="confirm-delete" on:click={onDelete}>
      Yes, Delete
    </button>
  </div>
</div>
{:else}
<div
  class="dialog details"
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

      {#if remote.driver === "ptcec"}
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

      {#if remote.driver === "ssh"}
        <div class="data">
          <span>Driver</span>
          <p title="SSH">SSH</p>
        </div>
        <div class="data">
          <span>PrivateKey</span>
          <p title={remote.privateKeyFile ?? "-"}>{remote.privateKeyFile ?? "-"}</p>
        </div>
        <div class="data">
          <span>RunCommand</span>
          <p title={remote.runCommand}>{remote.runCommand}</p>
        </div>
      {/if}
    </div>
    <div class="actions">
      <div>
        <button class="shortcut" on:click={saveUnixScript}>
          <div class="icon magic"></div>
          Create Shortcut
        </button>
        <button on:click={onEdit}>
          <div class="icon edit"></div>
          Edit Details
        </button>
        <button class="small" on:click={onMiddlewareEdit}>
          <div class="icon middleware"></div>
          Edit Middleware
        </button>
      </div>
      <button class="delete" on:click={() => deleteUIVisible = true}>Delete</button>
    </div>
  </div>
</div>
{/if}

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
    padding: 12px;
  }

  .dialog.details {
    height: 390px;
  }

  .dialog.delete{
    height: 170px;
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

  .inner-wrap .actions button:hover {
    background-color: #383b3d;
  }

  .inner-wrap .actions button:active {
    background-color: #212324;
  }

  .inner-wrap .actions button.shortcut {
    background-color: #0175ff;
  }

  .inner-wrap .actions button.shortcut:hover {
    background-color: #1d82ff;
  }

  .inner-wrap .actions button.shortcut:active {
    background-color: #1065cb;
  }

  .inner-wrap .actions button.delete {
    background-color: #c33737;
  }

  .inner-wrap .actions button.delete:hover {
    background-color: #e94c4c;
  }

  .inner-wrap .actions button.delete:active {
    background-color: #9f2121;
  }

  .inner-wrap .actions button.small {
    display: flex;
    justify-content: flex-start;
    flex-wrap: nowrap;
    font-size: 11px;
  }

  .inner-wrap .actions button.small:hover {
    background-color: #252626;
  }

  .inner-wrap .actions button.small:active {
    background-color: #212324;
  }

  .inner-wrap .actions button.small {
    background-color: transparent;
    display: flex;
    justify-content: flex-start;
    flex-direction: row;
    align-items: center;
    flex-wrap: nowrap;
    padding: 7px 12px;
    width: 100%;
  }

  .inner-wrap .actions button.small .icon {
    width: 24px;
    height: 24px;
    margin: 0;
  }

  .inner-wrap .actions button .icon {
    display: block;
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

  .inner-wrap .actions button .icon.middleware {
    background-image: url('/middleware.svg');
    filter: invert(1);
  }

  .confirm-text {
    text-align: left;
    padding: 12px 7px;
  }

  .confirm-text h2 {
    margin: 0;
    font-size: 22px;
  }

  .confirm-text p {
    margin: 7px 0;
    font-size: 16px;
  }

  .confirm-actions {
    display: flex;
    justify-content: flex-end;
    padding: 12px 7px;
  }

  .confirm-actions button {
    background-color: #2b2d2e;
    border: none;
    padding: 14px 32px;
    border-radius: 12px;
    margin: 0 0 0 9px;
    font-weight: bold;
    font-size: 12px;
    cursor: pointer;
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .confirm-actions button:hover {
    background-color: #383b3d;
  }

  .confirm-actions button:active {
    background-color: #212324;
  }

  .confirm-actions button.confirm-delete {
    background-color: #c33737;
  }

  .confirm-actions button.confirm-delete:hover {
    background-color: #e94c4c;
  }

  .confirm-actions button.confirm-delete:active {
    background-color: #9f2121;
  }
</style>