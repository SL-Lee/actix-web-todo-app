<script>
  import { getContext } from "svelte";

  import Modal from "./Modal.svelte";

  import TodoApi from "../todoApi";

  let modalComponent, createTodoForm, todoTitle, todoContents;
  let fetchTodos = getContext("fetchTodos");

  function handleFormSubmit() {
    TodoApi.createTodo(todoTitle, todoContents).then((response) => {
      if (response.status === "Success") {
        fetchTodos();
        halfmoon.initStickyAlert({
          title: "To-do created",
          content: "To-do created successfully.",
          alertType: "alert-success",
          timeShown: 5000,
        });
        modalComponent.hideModal();
      } else {
        halfmoon.initStickyAlert({
          title: "Error while creating to-do",
          content: "There was an error while creating the to-do.",
          alertType: "alert-danger",
          timeShown: 5000,
        });
      }
    });
  }
</script>

<Modal
  bind:this={modalComponent}
  modalId="create-todo-modal"
  modalTitle="Create new to-do"
  okButtonLabel="Create"
  onOk={() => createTodoForm.requestSubmit()}
  on:hidemodal
>
  <form bind:this={createTodoForm} on:submit|preventDefault={handleFormSubmit}>
    <div class="form-group">
      <label for="todo-title-input">To-do Title</label>
      <input
        type="text"
        class="form-control"
        id="todo-title-input"
        placeholder="To-do Title"
        required="required"
        bind:value={todoTitle}
      />
    </div>
    <div class="form-group">
      <label for="todo-contents-input">To-do Contents</label>
      <textarea
        class="form-control"
        id="todo-contents-input"
        placeholder="To-do Contents"
        bind:value={todoContents}
      />
    </div>
    <!-- This hidden input is required so pressing enter on any input fields (excluding
      textareas) will automatically submit the form, which is expected behavior of web forms -->
    <input type="submit" hidden />
  </form>
</Modal>
