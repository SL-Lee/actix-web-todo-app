<script>
  import { setContext } from "svelte";

  import TodoDropdownMenu from "./TodoDropdownMenu.svelte";

  export let id, title, contents, completed;

  setContext("todoId", id);
</script>

<div class="col-12 col-md-6 col-xl-4">
  <div class="card m-15">
    <h2 class="card-title text-truncate" style="margin-right: 7rem;">{title}</h2>
    <p class="font-weight-bold">
      Status:
      <span class="text-{completed ? 'success' : 'secondary'}">
        {completed ? "COMPLETED" : "INCOMPLETE"}
      </span>
    </p>
    <hr />
    <p class={contents ? "text-truncate" : "text-muted font-italic"}>
      {contents ? contents : "Content not provided"}
    </p>
    <TodoDropdownMenu on:showmodal />
  </div>
</div>
