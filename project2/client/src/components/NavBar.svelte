<script lang="ts">
  import IconButton from "@smui/icon-button";
  import TopAppBar, {
    Row,
    Section,
    Title
  } from '@smui/top-app-bar';
  import { Link } from 'svelte-navigator';
  import type { IRoute } from '../utils/routing';

  export let routes: IRoute[];
  export let topAppBar: TopAppBar;
</script>

<TopAppBar bind:this={topAppBar} variant="standard" color="secondary">
  <Row>
    <Section>
        <IconButton class="material-icons">menu</IconButton>
        <Title>Project 2</Title>
    </Section>
    <Section align="end" toolbar>
        {#each routes as { path, name, isProtected }}
            {#if !isProtected}
                <Link to={path} class="nav-link">{name}</Link>
            {/if}
        {/each}
    </Section>
  </Row>
</TopAppBar>

<style>
    :global(.nav-link) {
        padding-right: 1.5rem;
        color: white;
        text-decoration: none;
    }
</style>