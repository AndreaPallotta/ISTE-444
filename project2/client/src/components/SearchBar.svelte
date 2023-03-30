<script lang="ts">
  import Button from "@smui/button/src/Button.svelte";
  import IconButton from '@smui/icon-button';
  import { createEventDispatcher, onMount } from "svelte";

  let query = "";

  const dispatch = createEventDispatcher();

  const handleSearch = () => {
    dispatch("search", query);
  }

  const handleClear = () => {
    query = "";
    dispatch("clear");
  }

  onMount(() => {
    const input = document.getElementById("search-input");
    input.focus();
  });
</script>

<div id="search-bar">
  <input
    id="search-input"
    type="text"
    placeholder="Search..."
    bind:value={query}
  />
  {#if query}
    <IconButton class="material-icons" on:click={handleClear}>clear</IconButton>
  {/if}
  <Button disabled={query.trim().length === 0} on:click={handleSearch} variant="raised" style="padding: 1.5rem;">Search</Button>
</div>