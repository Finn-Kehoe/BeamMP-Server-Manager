<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import type { Mod } from "./mod";
    import { current_map } from "./stores";

    export let modObject: Mod;

    let is_active = false;

    async function setAsCurrentMap() {
        await invoke("change_map", { mapName: modObject.internal_name });
        current_map.set(modObject.internal_name);
    }

    current_map.subscribe((map) => {
        if (map === modObject.internal_name) {
            is_active = true;
        } else {
            is_active = false;
        }
    });

    async function deleteMap() {
        // TODO: implement
    }

</script>

<li>
    <div class="main-body" on:dblclick={setAsCurrentMap} class:active={is_active}>
        <div class="details">
            <p class="map-name">{modObject.details["title"]}</p>
            <p class="internal-name">{modObject.internal_name}</p>
        </div>
        <div class="action-button">
            <button class="delete button" on:click={deleteMap}>
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
        border-style: solid !important;
        border-width: 3px;
        border-color: #ff7722;
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
</style>