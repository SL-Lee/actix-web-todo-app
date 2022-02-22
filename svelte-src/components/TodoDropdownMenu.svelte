<script>
  import { createEventDispatcher, getContext } from "svelte";

  import DeleteTodoModal from "./DeleteTodoModal.svelte";
  import DropdownItem from "./DropdownItem.svelte";
  import EditTodoModal from "./EditTodoModal.svelte";
  import ViewTodoModal from "./ViewTodoModal.svelte";

  import TodoApi from "../todoApi";

  let todoStore = getContext("todoStore");
  let todoId = getContext("todoId");
  $: todo = $todoStore.find((todo) => todo.id === todoId);
  const dispatch = createEventDispatcher();

  function showViewTodoModal() {
    dispatch("showmodal", { modal: ViewTodoModal, todo });
  }

  function updateTodoStatus() {
    TodoApi.updateTodoStatus(todoId, !todo.completed).then((response) => {
      if (response.status === "success") {
        let index = $todoStore.findIndex((existingTodo) => existingTodo.id === todoId);
        $todoStore[index].completed = response.newTodoStatus;
      } else {
        halfmoon.initStickyAlert({
          title: "Error while updating to-do status",
          content: "There was an error while updating the to-do status.",
          alertType: "alert-danger",
          timeShown: 5000,
        });
      }
    });
  }

  function showEditTodoModal() {
    dispatch("showmodal", { modal: EditTodoModal, todo });
  }

  function showDeleteTodoModal() {
    dispatch("showmodal", { modal: DeleteTodoModal, todo });
  }
</script>

<div class="dropdown position-absolute">
  <button class="btn" data-toggle="dropdown"><i class="fas fa-ellipsis-h" /></button>
  <div class="dropdown-menu dropdown-menu-right">
    <DropdownItem iconClass="fa-eye" on:click={showViewTodoModal}>View</DropdownItem>
    <DropdownItem iconClass="fa-{todo.completed ? 'times' : 'check'}" on:click={updateTodoStatus}>
      Mark {todo.completed ? "Incomplete" : "Complete"}
    </DropdownItem>
    <DropdownItem iconClass="fa-edit" on:click={showEditTodoModal}>Edit</DropdownItem>
    <DropdownItem iconClass="fa-trash" on:click={showDeleteTodoModal}>Delete</DropdownItem>
  </div>
</div>
