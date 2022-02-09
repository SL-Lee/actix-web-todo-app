<script>
  import { createEventDispatcher } from "svelte";

  import CreateTodoModal from "./CreateTodoModal.svelte";

  const dispatch = createEventDispatcher();

  function createTodo() {
    dispatch("showmodal", { modal: CreateTodoModal });
  }
</script>

<button class="btn btn-primary" on:click={createTodo}>
  <i class="mr-10 fas fa-plus" />
  Create
</button>
