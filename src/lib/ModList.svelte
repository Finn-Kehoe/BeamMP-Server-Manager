<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri"
    import type { ContentMod } from "./mod";
    import ModListItem from "./ModListItem.svelte";
    import MultiModListItem from "./MultiModListItem.svelte";
    import { modlistHasBeenChanged, needsRestart } from "./stores";
    import { onMount } from "svelte";

    let mods: ContentMod[] = [];

    async function getMods() {
        await invoke("get_mod_content")
            .then((content: ContentMod[]) => mods = content)
            .catch((_) => mods = []);
    }

    modlistHasBeenChanged.subscribe((hasBeenChanged) => {
        if (hasBeenChanged) {
            getMods();
            modlistHasBeenChanged.set(false);
            needsRestart.update((_needsRestart) => _needsRestart + 1);
        }
    });

    onMount(() => {
        getMods();
    })

</script>

<div>
    <ul class="list-body">
        {#each mods as item}
            {#if item.inner_content.length === 1}
                <svelte:component this={ModListItem} modObject={item}/>
            {:else}
                <svelte:component this={MultiModListItem} modObject={item}/>
            {/if}
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