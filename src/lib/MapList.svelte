<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import type { Mod } from "./mod";
    import MapListItem from "./MapListItem.svelte";

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

    getModMaps();
</script>

<div>
    <ul>
        {#each mod_maps as item}
            <li><svelte:component this={MapListItem} modObject={item}/></li>
        {/each}
    </ul>
</div>