<script>
export let value = 'ptcec';
</script>

<div class="switch">
  <div class={value === "ptcec" ? "toggle-back" : "toggle-back right"}></div>
  <div class="toggle-text">
    <button class={value === "ptcec" ? "active" : ""} on:click={() => value = 'ptcec'}>
      PTCEC
    </button>
    <button class={value === "ptcec" ? "" : "active"} on:click={() => value = 'ssh'}>
      SSH
    </button>
  </div>
</div>

<style>
  .switch {
    position: relative;
    background: #101113;
    border: 1px solid #28292b;
    overflow: hidden;
    height: 32px;
    width: 200px;
    border-radius: 9px;
  }

  .toggle-text {
    position: absolute;
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;
    display: flex;
  }

  .toggle-text > button {
    width: 50%;
    display: flex;
    justify-content: center;
    align-items: center;
    color: rgb(149, 149, 149);
    font-size: 12px;
    cursor: pointer;
    border: none;
  }

  .toggle-text > button.active {
    font-weight: bold;
    color: white;
  }

  .toggle-back {
    position: absolute;
    width: 50%;
    border-radius: 10px;
    background-color: #2e2e33;
    border: 2px solid #101113;
    left: 0;
    top: 0;
    bottom: 0;
    transition: all ease-in-out 0.1s;
    box-sizing: border-box;
  }

  .toggle-back.right {
    left: 50%;
  }
</style>