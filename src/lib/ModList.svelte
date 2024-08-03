<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri"
    import type { Mod } from "./mod";
    import ModListItem from "./ModListItem.svelte";
    import { deleted_mods } from "./stores";
    import { onMount } from "svelte";

    let mods: Mod[] = [];

    async function getMods() {
        await invoke("get_mod_vehicles")
            .then((vehicles: Mod[]) => mods = vehicles)
            .catch((_) => mods = []);
    }
    /*
    deleted_mods.subscribe((deleted) => {
        console.log("Mods list: ", mods.length);
        deleted.forEach((deleted_mod, index) => {
            console.log("Mod to be deleted: ", deleted_mod);
            mods.forEach((mod, _index) => {
                if (mod.internal_name === deleted_mod) mods.splice(_index, 1);
            })
            console.log("Mods list:", mods.length);
            deleted_mods.update((old_deleted) => {
                old_deleted.splice(index, 1);
                return old_deleted;
            });
        })
    });
    */

    deleted_mods.subscribe((deleted) => {
        // TODO: get mods again but change it so it checks stored mods against current mods
    });

    onMount(() => {
        getMods();
        console.log("Initial mods list: ", mods.length);
    })

</script>

<div>
    <ul class="list-body">
        {#each mods as item}
            <svelte:component this={ModListItem} modObject={item}/>
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