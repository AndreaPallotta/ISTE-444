<script lang="ts">
  import LayoutGrid, { Cell } from '@smui/layout-grid';
  import { onMount } from 'svelte';
  import ItemCard from "../components/ItemCard.svelte";
  import SearchBar from '../components/SearchBar.svelte';
  import itemsStore from '../store/items.store';
  import notifStore from '../store/notification.store';
  import type { DeleteUserResponse } from '../types/ifaces';
  import type { Item } from '../types/models';
  import { axiosDelete, axiosGet } from '../utils/api.utils';

  const getItems = async () => {
    const response = await axiosGet<Item[], unknown>('/api/get_items');

    if (response.error || !response.data) {
        $notifStore.open(response.error ?? 'Error retrieving items', 'error');
        return;
    }

    $itemsStore = response.data.content;
  };

  const searchItem = async (e: CustomEvent<string>) => {
    const response = await axiosGet<Item[], unknown>(`/api/get_item/${e.detail}`);

    if (response.error || !response.data) {
        $notifStore.open(response.error ?? `Error searching for ${e.detail}`, 'error');
        return;
    }

    $itemsStore = response.data.content;
  };

  const deleteItem = async (e: CustomEvent<string>) => {
    console.log(e.detail);
    const response = await axiosDelete<DeleteUserResponse, object>('/api/delete_item', { id: e.detail });
    console.log({response});
    if (response.error || !response.data) {
        $notifStore.open(response.error ?? `Error deleting for ${e.detail}`, 'error');
        return;
    }

    await getItems();

    console.log('here');
    $notifStore.open(`Successfully deleted "${response.data.content.name}"`, 'success');
  };

  onMount(() => {
    getItems();
    return () => $itemsStore = [];
  });
</script>

<div id="home-wrapper">
    <SearchBar on:search={searchItem} on:clear={getItems} />
    <LayoutGrid>
    {#each $itemsStore as item}
        <Cell>
            <ItemCard {item} on:delete={deleteItem} />
        </Cell>
    {/each}
    </LayoutGrid>
</div>