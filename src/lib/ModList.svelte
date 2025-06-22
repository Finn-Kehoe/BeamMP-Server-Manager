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
    {#if mods.length !== 0}
        <ul class="list-body">
            {#each mods as item}
                {#if item.inner_content.length === 1}
                    <svelte:component this={ModListItem} modObject={item}/>
                {:else}
                    <svelte:component this={MultiModListItem} modObject={item}/>
                {/if}
            {/each}
        </ul>
    {:else}
        <div class="empty-prompt">
            <p class="empty-prompt-text">Drag and drop to add modded content.</p>
        </div>
    {/if}
</div>

<style>
    .empty-prompt {
        height: 25em;
        width: 20rem;
        margin-top: 16px;
        margin-bottom: 16px;
        margin-left: 40px;
        margin-right: 5px;
        background-image: url("data:image/svg+xml,%3csvg width='100%25' height='100%25' xmlns='http://www.w3.org/2000/svg'%3e%3crect width='100%25' height='100%25' fill='none' rx='40' ry='40' stroke='%233D3D3DFF' stroke-width='7' stroke-dasharray='20' stroke-dashoffset='0' stroke-linecap='round'/%3e%3c/svg%3e");
        border-radius: 40px;
        display: flex;
        justify-content: center;
        align-items: center;
        text-align: center;
    }
    .empty-prompt-text {
        margin-left: 20px;
        margin-right: 20px;
        color: #636363;
        font-size: 1.1em;
    }
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