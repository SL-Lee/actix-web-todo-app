<script>
  import { getContext } from "svelte";

  import Modal from "./Modal.svelte";

  import TodoApi from "../todoApi";

  export let todo;

  let modalComponent;
  let todoStore = getContext("todoStore");

  function deleteTodo() {
    TodoApi.deleteTodo(todo.id).then((response) => {
      if (response.status === "success") {
        $todoStore = $todoStore.filter(
          (existingTodo) => existingTodo.id !== response.deletedTodoId
        );
        halfmoon.initStickyAlert({
          title: "To-do deleted",
          content: "To-do deleted successfully.",
          alertType: "alert-success",
          timeShown: 5000,
        });
        modalComponent.hideModal();
      } else {
        halfmoon.initStickyAlert({
          title: "Error while deleting to-do",
          content: "There was an error while deleting the to-do.",
          alertType: "alert-danger",
          timeShown: 5000,
        });
      }
    });
  }
</script>

<Modal
  bind:this={modalComponent}
  modalId="delete-todo-modal"
  modalTitle="Delete to-do?"
  okButtonLabel="Delete"
  onOk={deleteTodo}
  on:hidemodal
>
  <p>Are you sure you want to delete this to-do?</p>
</Modal>
