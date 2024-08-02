<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import type { Mod } from "./mod";
    import MapListItem from "./MapListItem.svelte";
    import { onMount } from "svelte";
    import { current_map, needs_restart } from "./stores";
    import VMapListItem from "./VMapListItem.svelte";
    import { get } from "svelte/store";

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
    let lastLoadedMap = "";
    let mapHasBeenChanged = false;

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
        lastLoadedMap = get(current_map);
    })

    current_map.subscribe((map) => {
        // if the selected map is changed to not the loaded one, add to needs_restart
        if (map !== lastLoadedMap) {
            if (!mapHasBeenChanged) {
                needs_restart.update((_needs_restart) => _needs_restart + 1);
                mapHasBeenChanged = true;
            }
        // if the loaded map is reselected, subtract to needs_restart
        } else {
            needs_restart.update((_needs_restart) => _needs_restart - 1);
        }
    });

    needs_restart.subscribe((_needs_restart) => {
        // when server is restarted, reset the currently loaded map
        if (_needs_restart === 0) {
            lastLoadedMap = get(current_map);
            mapHasBeenChanged = false;
        }
    });

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