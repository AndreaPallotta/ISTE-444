<script lang="ts">
  import IconButton from "@smui/icon-button";
  import TopAppBar, {
    Row,
    Section,
    Title
  } from '@smui/top-app-bar';
  import { Link } from 'svelte-navigator';
  import routes from "../utils/routing";

  export let topAppBar: TopAppBar;
</script>

<TopAppBar bind:this={topAppBar} variant="standard" color="primary">
  <Row>
    <Section>
        <IconButton class="material-icons">menu</IconButton>
        <Title>E-Commerce With RANS</Title>
    </Section>
    <Section align="end" toolbar>
        {#each routes as { path, name, isProtected }}
            <!-- {#if !isProtected} -->
                <Link to={path} class="nav-link">{name}</Link>
            <!-- {/if} -->
        {/each}
    </Section>
  </Row>
</TopAppBar>