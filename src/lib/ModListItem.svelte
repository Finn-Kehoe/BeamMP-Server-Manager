<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import type { Mod } from "./mod";
    import { deleted_mods, needs_restart } from "./stores";

    export let modObject: Mod;
    let isActive = modObject.is_active;
    let lastLoadedActivation = isActive;
    let hasLoaded = false;

    onMount(() => {
        hasLoaded = true;
    })

    async function changeActivation() {
        if (hasLoaded) {
            await invoke("change_mod_activation", { internalName: modObject.internal_name })
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
        await invoke("delete_mod", { internalName: modObject.internal_name })
        .catch((e) => { console.log("Error deleteing mod: ", e); });

        deleted_mods.update((deleted) => {
            deleted.push(modObject.internal_name);
            return deleted;
        });

        // TODO: refresh modlist
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
            <p class="internal-name">{modObject.internal_name}</p>
            <p class="car-name">{modObject.details["brand"]} {modObject.details["name"]}</p>
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
</li>

<style>
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
