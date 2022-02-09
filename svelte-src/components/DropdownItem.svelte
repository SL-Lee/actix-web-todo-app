<script>
  export let iconClass;
</script>

<!-- svelte-ignore a11y-missing-attribute -->
<a class="dropdown-item" style="cursor: pointer;" on:click>
  <i class="mr-10 fas {iconClass}" />
  <slot />
</a>
