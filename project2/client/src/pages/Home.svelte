<script lang="ts">
  import LayoutGrid, { Cell } from '@smui/layout-grid';
  import { onMount } from 'svelte';
  import ItemCard from "../components/ItemCard.svelte";
  import SearchBar from '../components/SearchBar.svelte';
  import itemsStore from '../store/items.store';
  import notifStore from '../store/notification.store';
  import type { Item } from '../types/models';
  import { axiosGet } from '../utils/api.utils';

  const getItems = async () => {
    const response = await axiosGet('/api/get_items');

    if (response.error || !response.data) {
        $notifStore.open(response.error ?? 'Error retrieving items', 'error');
        return;
    }

    $itemsStore = response.data.content as Item[];
  };

  const searchItem = async (e: CustomEvent<string>) => {
    const response = await axiosGet(`/api/get_item/${e.detail}`);

    if (response.error || !response.data) {
        $notifStore.open(response.error ?? `Error searching for ${e.detail}`, 'error');
        return;
    }

    $itemsStore = response.data.content as Item[];
  };

  onMount(() => {
    getItems();
    return () => $itemsStore = [];
  })
</script>

<div id="home-wrapper">
    <SearchBar on:search={searchItem} on:clear={getItems} />
    <LayoutGrid>
    {#each $itemsStore as item}
        <Cell>
            <ItemCard {item} />
        </Cell>
    {/each}
    </LayoutGrid>
</div>