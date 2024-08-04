<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri"
    import type { Mod } from "./mod";
    import ModListItem from "./ModListItem.svelte";
    import { modlist_has_been_changed, needs_restart } from "./stores";
    import { onMount } from "svelte";

    let mods: Mod[] = [];

    async function getMods() {
        await invoke("get_mod_vehicles")
            .then((vehicles: Mod[]) => mods = vehicles)
            .catch((_) => mods = []);
    }

    modlist_has_been_changed.subscribe((hasBeenChanged) => {
        if (hasBeenChanged) {
            getMods();
            modlist_has_been_changed.set(false);
            needs_restart.update((_needs_restart) => _needs_restart + 1);
        }
    });

    onMount(() => {
        getMods();
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