<script lang="ts">
export let title: String;
export let value = false;
</script>

<button class="checkbox-wrap" class:checked={value} on:click={() => value = !value}>
  <div class="checkbox">
    <div class="icon"></div>
  </div>
  <span>{title}</span>
</button>

<style>
  .checkbox-wrap {
    display: flex;
    align-items: center;
    border: none;
    cursor: pointer;
  }

  .checkbox-wrap span {
    font-size: 14px;
    color: #74787e;
    padding-left: 12px;
    transition: cubic-bezier(0.175, 0.885, 0.32, 1.275) 0.4s;
  }

  .checkbox-wrap.checked span {
    font-size: 14px;
    color: #0175ff;
  }

  .checkbox-wrap .checkbox {
    position: relative;
    background: transparent;
    border: 1px solid #74787e;
    overflow: hidden;
    height: 24px;
    width: 24px;
    border-radius: 7px;
    transform: rotate(45deg);
    transition: cubic-bezier(0.175, 0.885, 0.32, 1.275) 0.4s;
  }

  .checkbox-wrap.checked .checkbox {
    background: #0175ff;
    border: 1px solid #0175ff;
    transform: rotate(0);
  }

  .checkbox-wrap .checkbox .icon {
    position: absolute;
    top: 3px;
    bottom: 3px;
    left: 3px;
    right: 3px;
    background-color: transparent;
    background-image: url("/check.svg");
    background-size: contain;
    background-repeat: no-repeat;
    transform: rotate(-90deg);
    filter: invert(1);
    opacity: 0;
    transition: cubic-bezier(0.175, 0.885, 0.32, 1.275) 0.6s;
  }

  .checkbox-wrap.checked .checkbox .icon {
    transform: rotate(0deg);
    opacity: 1;
  }
</style>