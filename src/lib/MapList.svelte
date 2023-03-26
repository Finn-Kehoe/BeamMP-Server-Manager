<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri"

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
    ]

    async function getModMaps() {
        let moddedMaps: string[]
        await invoke("get_mod_maps")
            .then((_map: string[]) => moddedMaps = _map)
            .catch((_) => moddedMaps = [""])
        moddedMaps.forEach(map => {
            maps.push(map)
        })
    }

    getModMaps()
</script>