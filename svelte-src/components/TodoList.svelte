<script>
  import { setContext } from "svelte";
  import { writable } from "svelte/store";

  import CreateTodoButton from "./CreateTodoButton.svelte";
  import Todo from "./Todo.svelte";

  import TodoApi from "../todoApi";

  let todoStore = writable([]);
  TodoApi.getTodos().then((todos) => ($todoStore = todos));

  setContext("todoStore", todoStore);

  let modalComponent, inspectedTodo;

  function showModal(event) {
    modalComponent = event.detail.modal;
    inspectedTodo = event.detail.todo;
  }

  function hideModal() {
    modalComponent = null;
  }
</script>

<!-- `inspectedTodo` will be `undefined` if the modal component to render is the <CreateTodoModal>
  (since creating to-dos does not require an existing to-do as input -- as opposed to viewing
  to-dos, for example. Thus, this if-block will prevent Svelte from warning about <CreateTodoModal>
  being created with unknown prop 'todo'. -->
{#if inspectedTodo}
  <svelte:component this={modalComponent} todo={inspectedTodo} on:hidemodal={hideModal} />
{:else}
  <svelte:component this={modalComponent} on:hidemodal={hideModal} />
{/if}

<CreateTodoButton on:showmodal={showModal} />

{#if $todoStore.length > 0}
  <div class="row" style="margin-left: -1.5rem; margin-right: -1.5rem;">
    {#each $todoStore as todo}
      <Todo
        id={todo.id}
        title={todo.title}
        contents={todo.contents}
        completed={todo.completed}
        on:showmodal={showModal}
      />
    {/each}
  </div>
{:else}
  <p class="text-muted text-center font-italic">No To-dos yet</p>
{/if}
