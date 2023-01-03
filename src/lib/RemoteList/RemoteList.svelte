<script lang="ts">
    import type { Remote } from "../../models/remote";
    import { createEventDispatcher } from "svelte";
    import RemoteListRow from "./RemoteListRow.svelte";

    const dispatch = createEventDispatcher();

    export let remotes: Remote[] = [];

    const onAdd = () => {
        dispatch("addPressed");
    };
</script>

<div>
  <div class="section">
    <div class="head">
      <div class="icon"></div>
      <h2>Remotes</h2>
      <div class="actions">
        <button class="add-button" on:click={onAdd}>
          <div class="add-icon"></div>
        </button>
      </div>
    </div>
    <div class="items">
      {#each remotes as remote}
        <RemoteListRow remote={remote} />
      {/each}
    </div>
  </div>
</div>

<style scoped>
.section .head {
  display: flex;
  justify-content: flex-start;
  align-items: center;
  margin-bottom: 10px;
  padding: 0 24px;
}

.section .head .icon {
  width: 18px;
  height: 18px;
  background-size: contain;
  background-repeat: no-repeat;
  background-image: url(./remote.svg);
  filter: invert(1);
}

.section .head h2 {
  font-size: 14px;
  font-weight: bold;
  margin: 0 0 0 9px;
}

.section .head .actions {
  flex-grow: 1;
  display: flex;
  justify-content: flex-end;
}

.items {
  padding: 0 14px;
}

.add-button {
  border: none;
  padding: 2px;
  background-color: #2e2e33;
  cursor: pointer;
  border-radius: 4px;
}

.add-button .add-icon {
  height: 16px;
  width: 16px;
  background-size: contain;
  background-repeat: no-repeat;
  background-image: url(./add.svg);
  filter: invert(1);
}
</style>