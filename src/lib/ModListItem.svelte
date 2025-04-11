<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import type { ContentMod } from "./mod";
    import { modlistHasBeenChanged, needsRestart } from "./stores";

    export let modObject: ContentMod;
    let isActive = modObject.is_active;
    let lastLoadedActivation = isActive;
    let hasLoaded = false;

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

<li>
    <div class="main-body">
        <div class="details">
            <p class="car-name">{modObject.inner_content[0].brand} {modObject.inner_content[0].name}</p>
            <p class="internal-name">{modObject.inner_content[0].internal_name}</p>
        </div>
        <div class="action-buttons">
            <label class="on-off switch">
                <input type="checkbox" bind:checked={isActive}>
                <span class="slider round"></span>
            </label>
            <button class="delete button" on:click={deleteMod}>
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
        border-radius: 8px;
        display: flex;
    }
    .main-body:hover {
        background-color: #313131;
    }
    .details {
        padding: 4%;
        align-self: center;
        text-align: left;
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
</style>
