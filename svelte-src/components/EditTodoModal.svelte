<script>
  import { getContext } from "svelte";

  import Modal from "./Modal.svelte";

  import TodoApi from "../todoApi";

  export let todo;

  let modalComponent, editTodoForm;
  let todoTitle = todo.title;
  let todoContents = todo.contents;
  let todoStore = getContext("todoStore");

  function handleFormSubmit() {
    TodoApi.editTodo(todo.id, todoTitle, todoContents).then((response) => {
      if (response.status === "success") {
        let index = $todoStore.findIndex((existingTodo) => existingTodo.id === todo.id);
        $todoStore[index] = response.todo;
        halfmoon.initStickyAlert({
          title: "To-do edited",
          content: "To-do edited successfully.",
          alertType: "alert-success",
          timeShown: 5000,
        });
        modalComponent.hideModal();
      } else {
        halfmoon.initStickyAlert({
          title: "Error while editing to-do",
          content: "There was an error while editing the to-do.",
          alertType: "alert-danger",
          timeShown: 5000,
        });
      }
    });
  }
</script>

<Modal
  bind:this={modalComponent}
  modalId="edit-todo-modal"
  modalTitle="Edit to-do"
  okButtonLabel="Edit"
  onOk={() => editTodoForm.requestSubmit()}
  on:hidemodal
>
  <form bind:this={editTodoForm} on:submit|preventDefault={handleFormSubmit}>
    <div class="form-group">
      <label for="todo-title-input">To-do Title</label>
      <input
        type="text"
        class="form-control"
        id="todo-title-input"
        placeholder="To-do Title"
        required="required"
        maxlength="100"
        bind:value={todoTitle}
      />
    </div>
    <div class="form-group">
      <label for="todo-contents-input">To-do Contents</label>
      <textarea
        class="form-control"
        id="todo-contents-input"
        placeholder="To-do Contents"
        maxlength="512"
        bind:value={todoContents}
      />
    </div>
    <!-- This hidden input is required so pressing enter on any input fields (excluding
      textareas) will automatically submit the form, which is expected behavior of web forms -->
    <input type="submit" hidden />
  </form>
</Modal>
