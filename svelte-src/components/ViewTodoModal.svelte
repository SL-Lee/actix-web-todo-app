<script>
  import Modal from "./Modal.svelte";

  export let todo;

  let modalComponent;
</script>

<Modal
  bind:this={modalComponent}
  modalId="view-todo-modal"
  modalTitle={todo.title}
  showSecondaryButton={false}
  okButtonLabel="Close"
  onOk={() => modalComponent.hideModal()}
  on:hidemodal
>
  <p class="font-weight-bold">
    Status:
    <span class="text-{todo.completed ? 'success' : 'secondary'}">
      {todo.completed ? "COMPLETED" : "INCOMPLETE"}
    </span>
  </p>
  <hr />
  {#if todo.contents}
    <p>{todo.contents}</p>
  {:else}
    <p class="text-muted font-italic">Content not provided</p>
  {/if}
</Modal>
