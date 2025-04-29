<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import type { ContentMod } from "./mod";
    import { modlistHasBeenChanged, needsRestart } from "./stores";
    import { slide } from "svelte/transition";
    import ModalTemplate from "./ModalTemplate.svelte";

    export let modObject: ContentMod;
    let isActive = modObject.is_active;
    let lastLoadedActivation = isActive;
    let hasLoaded = false;
    let showExpanded = false;
    let showDeletePrompt = false;

    onMount(() => {
        hasLoaded = true;
    })

    async function changeActivation() {
        if (hasLoaded) {
            await invoke("change_mod_activation", { fileName: modObject.file_name })
            .catch((e) => { console.log("Error changing activation state for mod: ", e); });

            // if the mod activation is changed from previously loaded value, add to needs_restart
            if (lastLoadedActivation !== isActive) {
                needsRestart.update((_needsRestart) => _needsRestart + 1);
            // if it is put back to previously loaded value, subtract from needs_restart
            } else {
                needsRestart.update((_needsRestart) => _needsRestart - 1);
            }
        }
    }

    async function deleteMod() {
        await invoke("delete_mod", { fileName: modObject.file_name })
        .catch((e) => { console.log("Error deleteing mod: ", e); });

        modlistHasBeenChanged.set(true);
    }

    needsRestart.subscribe((_needsRestart) => {
        // when server is restarted, reset currently loaded activation state
        if (_needsRestart === 0) {
            lastLoadedActivation = isActive;
        }
    });

    $: isActive, changeActivation();
    
</script>


<ModalTemplate show={showDeletePrompt} onClose={() => showDeletePrompt = false} width={"auto"}>
    <div class="delete-prompt-wrapper">
        <h2 class="delete-header">Delete Confirmation</h2>
        <hr class="delete-spacer"/>
        <p class="delete-prompt-text">
            Are you sure you want to delete "{modObject.file_name}"?
        </p>
        <div class="delete-action-buttons">
            <button class="button close-delete" on:click={() => showDeletePrompt = false}>Cancel</button>
            <button class="button true-delete" on:click={() => {deleteMod(); showDeletePrompt = false;}}>Delete</button>
        </div>
    </div>
</ModalTemplate>
<li>
    <div class="main-body">
        <div class="details">
            <button class="multi-toggle" on:click={() => showExpanded = !showExpanded}>
                <svg class:flipped={!showExpanded} 
                    viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <g id="SVGRepo_bgCarrier" stroke-width="0"></g>
                    <g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g>
                    <g id="SVGRepo_iconCarrier">
                        <path fill-rule="evenodd" clip-rule="evenodd" d="M6.29289 8.79289C6.68342 8.40237 7.31658 8.40237 7.70711 8.79289L12 13.0858L16.2929 8.79289C16.6834 8.40237 17.3166 8.40237 17.7071 8.79289C18.0976 9.18342 18.0976 9.81658 17.7071 10.2071L12.7071 15.2071C12.3166 15.5976 11.6834 15.5976 11.2929 15.2071L6.29289 10.2071C5.90237 9.81658 5.90237 9.18342 6.29289 8.79289Z" fill="#ffffff">
                        </path> 
                    </g>
                </svg>
            </button>
            <p class="mod-name">{modObject.file_name}</p>
        </div>
        <div class="action-buttons">
            <label class="on-off switch">
                <input type="checkbox" bind:checked={isActive}>
                <span class="slider round"></span>
            </label>
            <button class="delete button" on:click={() => showDeletePrompt = true}>
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 -960 960 960" fill="currentColor">
                    <path d="M300.62-128q-38.85 0-64.74-25.88Q210-179.77 210-218.62V-724h-40v-66h188v-38.77h246V-790h188v66h-40v505.38q0 38.35-26.14 64.48Q699.73-128 661.38-128H300.62ZM686-724H276v505.38q0 10.77 6.92 17.7 6.93 6.92 17.7 6.92h360.76q9.24 0 16.93-7.69 7.69-7.69 7.69-16.93V-724ZM371.31-275h66v-368h-66v368Zm153.38 0h66v-368h-66v368ZM276-724v530-530Z"/>
                </svg>
            </button>
        </div>
    </div>
    <div class="expanded-body">
        {#if showExpanded}
            {#each modObject.inner_content as thisInner}
                <div class="expanded-details" transition:slide="{{duration: 250}}">
                    <p class="internal-name">{thisInner.internal_name}</p>
                    <p class="car-name">{thisInner.brand} {thisInner.name}</p>
                </div>
            {/each}
        {/if}
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
        border-radius: 8px;
        display: flex;
    }
    .main-body:hover {
        background-color: #313131;
    }
    .details {
        display: flex;
        align-items: center;
    }
    .multi-toggle {
        background-color: transparent;
        border: none;
        box-shadow: none;
        padding-left: 3%;
        width: 25%;
    }
    .multi-toggle svg {
        transition-property: transform;
        transition-duration: 0.25s;
        transition-timing-function: ease-in-out;
        width: 100%;
    }
    .flipped {
        transform: rotate(-180deg);
    }
    .action-buttons {
        margin-left: auto;
        display: flex;
        flex-direction: column;
        padding: 2%;
    }
    .delete {
        margin-top: auto;
        justify-self: flex-end;
        align-self: flex-end;
        padding: 0%;
        width: 75%;
        height: 50%;
    }
    .delete.button svg {
        color: inherit;
        height: 75%;
    }
    .delete.button:hover, .delete.button svg:hover {
        color: rgb(252, 77, 77);
    }
    .switch {
    position: relative;
    display: inline-block;
    width: 50px;
    height: 24px;
    }
    .switch input {
    opacity: 0;
    width: 0;
    height: 0;
    }
    .slider {
    position: absolute;
    cursor: pointer;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: #1a1a1a;
    -webkit-transition: .4s;
    transition: .4s;
    }
    .slider:before {
    position: absolute;
    content: "";
    height: 20px;
    width: 20px;
    left: 2px;
    bottom: 2px;
    background-color: white;
    -webkit-transition: .4s;
    transition: .4s;
    }
    input:checked + .slider {
    background-color: #ff7722;
    }
    input:focus + .slider {
    box-shadow: 0 0 1px #ff7722;
    }
    input:checked + .slider:before {
    -webkit-transform: translateX(26px);
    -ms-transform: translateX(26px);
    transform: translateX(26px);
    }
    .slider.round {
    border-radius: 34px;
    }
    .slider.round:before {
    border-radius: 50%;
    }
    .expanded-body {
        margin-left: auto;
        margin-right: auto;
        width: 95%;
    }
    .expanded-details {
        border-radius: 8px;
        background-color: #2b2b2b;
    }
    .expanded-details:hover {
        background-color: #313131;
    }
    .expanded-details p {
        margin-bottom: 2%;
        margin-top: 2%;
    }
    .expanded-details .internal-name {
        padding-top: 1.5%;
    }
    .expanded-details .car-name {
        padding-bottom: 1.5%;
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
