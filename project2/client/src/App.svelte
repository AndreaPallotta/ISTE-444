<script lang="ts">
  import TopAppBar, { AutoAdjust } from "@smui/top-app-bar";
  import { Route, Router } from "svelte-navigator";
  import NavBar from "./components/NavBar.svelte";
  import Notification from "./components/Notification.svelte";
  import routes from "./utils/routing";

  let topAppBar: TopAppBar;
</script>

<Router>
  <NavBar {topAppBar} />
  <AutoAdjust {topAppBar} class="page-wrapper">
    {#each routes as { path, component }}
        <Route {path} {component} primary={false} />
    {/each}
  </AutoAdjust>
</Router>
<Notification />