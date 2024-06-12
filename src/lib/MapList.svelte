<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import type { Mod } from "./mod";
    import MapListItem from "./MapListItem.svelte";
    import { onMount } from "svelte";
    import { current_map } from "./stores";
    import VMapListItem from "./VMapListItem.svelte";

    let maps = [
        "Gridmap v2",
        "Johnson Valley",
        "Automation Test Track",
        "East Coast, USA",
        "Hirochi Raceway",
        "Italy",
        "Jungle Rock Island",
        "Industrial Site",
        "Small Island, USA",
        "Grid, Small, Pure",
        "Utah, USA",
        "West Coast, USA",
        "ETK Driver Experience Center",
        "Derby Arenas"
    ];

    let mod_maps: Mod[] = [];

    async function getModMaps() {
        await invoke("get_mod_maps")
            .then((_map: Mod[]) => mod_maps = _map)
            .catch((_) => mod_maps = []);
    }

    async function getCurrentMap() {
        await invoke("get_current_map")
            .then((_current_map: string) => current_map.set(_current_map))
            .catch((_) => current_map.set("gridmap_v2"));
    }

    onMount(() => {
        getModMaps();
        getCurrentMap();
    })

    // getModMaps();
</script>

<div>
    <ul>
        <!--Vanilla Maps-->
        <svelte:component this={VMapListItem} internal_name={"gridmap_v2"} external_name={"Gridmap v2"}/>
        <!--Modded Maps-->
        {#each mod_maps as item}
            <svelte:component this={MapListItem} modObject={item}/>
        {/each}
    </ul>
</div>

<style>
    ul {
        list-style-type: none;
    }
</style>