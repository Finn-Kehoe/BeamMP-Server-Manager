<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import type { ContentMod } from "./mod";
    import { modlist_has_been_changed, needs_restart } from "./stores";

    export let modObject: ContentMod;
    let isActive = modObject.is_active;
    let lastLoadedActivation = isActive;
    let hasLoaded = false;
    let showExpanded = false;

    onMount(() => {
        hasLoaded = true;
    })

    async function changeActivation() {
        if (hasLoaded) {
            await invoke("change_mod_activation", { fileName: modObject.file_name })
            .catch((e) => { console.log("Error changing activation state for mod: ", e); });

            // if the mod activation is changed from previously loaded value, add to needs_restart
            if (lastLoadedActivation !== isActive) {
                needs_restart.update((_needs_restart) => _needs_restart + 1);
            // if it is put back to previously loaded value, subtract from needs_restart
            } else {
                needs_restart.update((_needs_restart) => _needs_restart - 1);
            }
        }
    }

    async function deleteMod() {
        await invoke("delete_mod", { fileName: modObject.file_name })
        .catch((e) => { console.log("Error deleteing mod: ", e); });

        modlist_has_been_changed.set(true);
    }

    needs_restart.subscribe((_needs_restart) => {
        // when server is restarted, reset currently loaded activation state
        if (_needs_restart === 0) {
            lastLoadedActivation = isActive;
        }
    });

    $: isActive, changeActivation();
    
</script>

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
            <button class="delete button" on:click={deleteMod}>
                <svg
                    style="fill: currentColor;"
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 512 512">
                        <path
                            d="M199 103v50h-78v30h270v-30h-78v-50H199zm18 18h78v32h-78v-32zm-79.002 80l30.106 286h175.794l30.104-286H137.998zm62.338 13.38l.64 8.98 16 224 .643 8.976-17.956 1.283-.64-8.98-16-224-.643-8.976 17.956-1.283zm111.328 0l17.955 1.284-.643 8.977-16 224-.64 8.98-17.956-1.284.643-8.977 16-224 .64-8.98zM247 215h18v242h-18V215z"
                        />
                </svg>
                <!-- <img src="/trash-can.svg" alt="Delete" /> -->
            </button>
        </div>
    </div>
    <div class="expanded-body">
        {#if showExpanded}
            {#each modObject.inner_content as thisInner}
                <div class="expanded-details">
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
        width: 100%;
    }
    .flipped {
        transform: rotate(180deg);
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
     /* The switch - the box around the slider */
    .switch {
    position: relative;
    display: inline-block;
    width: 50px;
    height: 24px;
    }

    /* Hide default HTML checkbox */
    .switch input {
    opacity: 0;
    width: 0;
    height: 0;
    }

    /* The slider */
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

    /* Rounded sliders */
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
</style>
