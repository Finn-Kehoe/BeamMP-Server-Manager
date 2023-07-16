<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri"
    import type { Mod } from "./mod";
    import ModListItem from "./ModListItem.svelte";

    let mods: Mod[] = [];

    async function getMods() {
        await invoke("get_mod_vehicles")
            .then((vehicles: Mod[]) => mods = vehicles)
            .catch((_) => mods = []);
    }
    /*
    function addItem(mod) {
        var l = $storedVehicles.length;
        $storedVehicles[l] = {id:vehicleIdIncrement, name: mod.internal_name}
        $vehicleIdIncrement++;
    }

    mods.forEach(mod => {
        addItem(mod);
    });
    */

    getMods();

</script>

<div>
    <ul class="list-body">
        {#each mods as item}
            <li><svelte:component this={ModListItem} modObject={item}/></li>
        {/each}
    </ul>
</div>

<style>
    
</style>