<script lang="ts">
  import TopAppBar, { AutoAdjust } from "@smui/top-app-bar";
  import { Route, Router } from "svelte-navigator";
  import NavBar from "./components/NavBar.svelte";
  import Home from "./pages/Home.svelte";
  import Login from "./pages/Login.svelte";
  import Signup from "./pages/Signup.svelte";
  import type { IRoute } from "./utils/routing";

  let topAppBar: TopAppBar;
  const routes: IRoute[] = [
    {
      path: '/',
      name: 'Home',
      component: Home,
      isProtected: true,
    },
    {
      path: '/login',
      name: 'Login',
      component: Login,
    },
    {
      path: '/signup',
      name: 'Sign Up',
      component: Signup,
    }
  ];
</script>

<Router>
  <NavBar {topAppBar} {routes} />
  <AutoAdjust {topAppBar} class="page-wrapper">
    {#each routes as { path, component }}
        <Route {path} {component}></Route>
    {/each}
  </AutoAdjust>
</Router>

<style>
  :global(body),
  :global(html) {
    display: block;
    padding: 0;
  }

  :global(#app) {
    padding: 0;
    margin: 0;
    width: 100vw;
  }

  :global(.page-wrapper) {
    display: block;
    height: 100vh;
    width: 100vw;
  }
</style>