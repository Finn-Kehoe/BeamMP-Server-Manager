<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import type { MapMod } from "./mod";
    import { currentMap, maplistHasBeenChanged } from "./stores";
    import ModalTemplate from "./ModalTemplate.svelte";

    export let modObject: MapMod;

    let isActive = false;
    let showDeletePrompt = false;

    async function setAsCurrentMap() {
        await invoke("change_map", { mapName: modObject.internal_name });
        currentMap.set(modObject.internal_name);
    }

    currentMap.subscribe((map) => {
        if (map === modObject.internal_name) {
            isActive = true;
        } else {
            isActive = false;
        }
    });

    async function deleteMap() {
        if (!isActive) {
            await invoke("delete_mod", { fileName: modObject.file_name })
            .catch((e) => { console.log("Error deleteing mod: ", e); });
    
            maplistHasBeenChanged.set(true);
        }
    }

</script>

<ModalTemplate show={showDeletePrompt} onClose={() => showDeletePrompt = false} width={"auto"}>
    <div class="delete-prompt-wrapper">
        <h2 class="delete-header">Delete Confirmation</h2>
        <hr class="delete-spacer"/>
        <p class="delete-prompt-text">
            Are you sure you want to delete "{modObject.external_name}"?
        </p>
        <div class="delete-action-buttons">
            <button class="button close-delete" on:click={() => showDeletePrompt = false}>Cancel</button>
            <button class="button true-delete" on:click={() => {deleteMap(); showDeletePrompt = false;}}>Delete</button>
        </div>
    </div>
</ModalTemplate>
<li>
    <div class="main-body" on:dblclick={setAsCurrentMap} class:active={isActive}>
        <div class="details">
            <p class="map-name">{modObject.external_name}</p>
            <p class="internal-name">{modObject.internal_name}</p>
        </div>
        <div class="action-button">
            <button class="delete button" on:click={() => showDeletePrompt = true}>
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 -960 960 960" fill="currentColor">
                    <path d="M300.62-128q-38.85 0-64.74-25.88Q210-179.77 210-218.62V-724h-40v-66h188v-38.77h246V-790h188v66h-40v505.38q0 38.35-26.14 64.48Q699.73-128 661.38-128H300.62ZM686-724H276v505.38q0 10.77 6.92 17.7 6.93 6.92 17.7 6.92h360.76q9.24 0 16.93-7.69 7.69-7.69 7.69-16.93V-724ZM371.31-275h66v-368h-66v368Zm153.38 0h66v-368h-66v368ZM276-724v530-530Z"/>
                </svg>
            </button>
        </div>

    </div>
</li>

<style>
    li:not(:last-child) {
        margin-bottom: 3px;
    }
    .main-body {
        background-color: #2b2b2b;
        height: 6rem;
        width: 20rem;
        border-style: none;
        border-radius: 8px;
        display: flex;
    }
    .main-body:hover {
        background-color: #313131;
    }
    .active {
        outline-style: solid !important;
        outline-width: 3px;
        outline-color: #ff7722;
    }
    .details {
        justify-self: center;
        text-align: left;
        padding-left: 5%;
    }
    .action-button {
        margin-left: auto;
        margin-top: auto;
        padding: 2%;
    }
    .delete {
        padding: 0%;
        width: 40px;
        height: 40px;
    }
    .delete.button svg {
        position: relative;
        color: inherit;
        height: 75%;
    }
    .delete.button:hover, .delete.button svg:hover {
        color: rgb(252, 77, 77);
    }
    .delete-header {
        margin-top: 0.5%;
        margin-bottom: 2%;
    }
    .delete-spacer {
        border: 0;
        padding: 0;
        margin: 2px;
        display: block;
        justify-self: center;
        height: 2px;
        width: 100%;
        border-top: 2px solid #3d3d3d;
    }
    .delete-prompt-text {
        margin-top: 1%;
        text-align: center;
        font-size: 1.1em;
    }
    .delete-action-buttons {
        display: flex;
        margin-left: 5%;
        margin-right: 5%;
    }
    .true-delete {
        margin-left: auto;
    }
    .true-delete:hover {
        color: #ffffff;
        background-color: rgb(252, 77, 77);
    }
</style>