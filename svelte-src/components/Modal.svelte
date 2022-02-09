<script>
  import { onMount, createEventDispatcher } from "svelte";

  export let modalId, modalTitle;
  export let showSecondaryButton = true;
  export let okButtonLabel = "OK";
  export let onOk = () => {};

  let modalElement;
  let show = false;
  const dispatch = createEventDispatcher();

  export function hideModal() {
    show = false;
    setTimeout(() => dispatch("hidemodal"), 300);
  }

  function handleEscape() {
    let handleKeyUp = (e) => {
      if (e.key === "Escape") {
        hideModal();
      }
    };

    document.addEventListener("keyup", handleKeyUp);

    return {
      destroy() {
        document.removeEventListener("keyup", handleKeyUp);
      },
    };
  }

  function handleClickOutside(node) {
    let handleClick = (e) => {
      if (!node.contains(e.target)) {
        hideModal();
      }
    };

    setTimeout(() => document.addEventListener("click", handleClick));

    return {
      destroy() {
        document.removeEventListener("click", handleClick);
      },
    };
  }

  onMount(() => {
    document.getElementById("modal-root").appendChild(modalElement);
    setTimeout(() => (show = true));
  });
</script>

<div
  bind:this={modalElement}
  class="modal"
  class:show
  id={modalId}
  role="dialog"
  data-overlay-dismissal-disabled="true"
  data-esc-dismissal-disabled="true"
  use:handleEscape
>
  <div class="modal-dialog" role="document">
    <div class="modal-content" use:handleClickOutside>
      <button class="close" on:click={hideModal}>×</button>
      <h5 class="modal-title text-break">{modalTitle}</h5>
      <slot />
      <div class="text-right mt-20">
        {#if showSecondaryButton}
          <button class="btn mr-5" on:click={hideModal}>Cancel</button>
        {/if}
        <button class="btn btn-primary" on:click={onOk}>{okButtonLabel}</button>
      </div>
    </div>
  </div>
</div>
