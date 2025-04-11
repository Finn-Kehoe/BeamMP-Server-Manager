<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import type { MapMod } from "./mod";
    import MapListItem from "./MapListItem.svelte";
    import { onMount } from "svelte";
    import { currentMap, needsRestart, maplistHasBeenChanged } from "./stores";
    import VMapListItem from "./VMapListItem.svelte";
    import { get } from "svelte/store";

    let modMaps: MapMod[] = [];
    let lastLoadedMap = "";
    let mapHasBeenChanged = false;

    async function getModMaps() {
        await invoke("get_mod_maps")
            .then((_map: MapMod[]) => modMaps = _map)
            .catch((_) => modMaps = []);
    }

    async function getCurrentMap() {
        await invoke("get_current_map")
            .then((_currentMap: string) => { lastLoadedMap = _currentMap; currentMap.set(_currentMap); })
            .catch((_) => currentMap.set("gridmap_v2"));
    }

    onMount(() => {
        getModMaps();
        getCurrentMap();
    })

    maplistHasBeenChanged.subscribe((hasBeenChanged) => {
        if (hasBeenChanged) {
            getModMaps();
            maplistHasBeenChanged.set(false);
            needsRestart.update((_needsRestart) => _needsRestart + 1);
        }
    });

    currentMap.subscribe((map) => {
        // if the selected map is changed to not the loaded one, add to needs_restart
        if (map !== lastLoadedMap) {
            if (!mapHasBeenChanged) {
                needsRestart.update((_needsRestart) => _needsRestart + 1);
                mapHasBeenChanged = true;
            }
        // if the loaded map is reselected, subtract to needs_restart
        } else {
                needsRestart.update((_needsRestart) => _needsRestart !== 0 ? _needsRestart - 1 : 0);
        }
    });

    needsRestart.subscribe((_needsRestart) => {
        // when server is restarted, reset the currently loaded map
        if (_needsRestart === 0) {
            lastLoadedMap = get(currentMap);
            mapHasBeenChanged = false;
        }
    });

</script>

<div>
    <ul>
        <!--Vanilla Maps-->
        <svelte:component this={VMapListItem} internal_name={"gridmap_v2"} external_name={"Gridmap v2"}/>
        <svelte:component this={VMapListItem} internal_name={"johnson_valley"} external_name={"Johnson Valley"}/>
        <svelte:component this={VMapListItem} internal_name={"automation_test_track"} external_name={"Automation Test Track"}/>
        <svelte:component this={VMapListItem} internal_name={"east_coast_usa"} external_name={"East Coast, USA"}/>
        <svelte:component this={VMapListItem} internal_name={"hirochi_raceway"} external_name={"Hirochi Raceway"}/>
        <svelte:component this={VMapListItem} internal_name={"italy"} external_name={"Italy"}/>
        <svelte:component this={VMapListItem} internal_name={"jungle_rock_island"} external_name={"Jungle Rock Island"}/>
        <svelte:component this={VMapListItem} internal_name={"industrial"} external_name={"Industrial Site"}/>
        <svelte:component this={VMapListItem} internal_name={"small_island"} external_name={"Small Island, USA"}/>
        <svelte:component this={VMapListItem} internal_name={"smallgrid"} external_name={"Grid, Small, Pure"}/>
        <svelte:component this={VMapListItem} internal_name={"utah"} external_name={"Utah, USA"}/>
        <svelte:component this={VMapListItem} internal_name={"west_coast_usa"} external_name={"West Coast, USA"}/>
        <svelte:component this={VMapListItem} internal_name={"driver_training"} external_name={"ETK Driver Experience Center"}/>
        <svelte:component this={VMapListItem} internal_name={"derby"} external_name={"Derby Arenas"}/>
        <!--Modded Maps-->
        {#each modMaps as item}
            <svelte:component this={MapListItem} modObject={item}/>
        {/each}
    </ul>
</div>

<style>
    ul {
        height: 25em;
        list-style-type: none;
        overflow: hidden;
        overflow-y: scroll;
        padding-right: 5px;
        margin-right: -5px;
    }
    ::-webkit-scrollbar {
        background-color: transparent;
        width: 10px;
    }
    ::-webkit-scrollbar-thumb {
        background-clip: padding-box;
        border: 3px solid transparent;
        /* width: 7px; */
        border-radius: 5%;
        background-color: #383838;
    }
    ::-webkit-scrollbar-thumb:hover {
        background-color: #3d3d3d;
    }
</style>