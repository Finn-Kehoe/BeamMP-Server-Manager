<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { current_map } from "./stores";

    export let internal_name: string;
    export let external_name: string;

    let isActive = false;

    async function setAsCurrentMap() {
        await invoke("change_map", { mapName: internal_name });
        current_map.set(internal_name);
    }

    current_map.subscribe((map) => {
        if (map === internal_name) {
            isActive = true;
        } else {
            isActive = false;
        }
    });
</script>

<li>
    <div class="main-body" on:dblclick={setAsCurrentMap} class:active={isActive}>
        <div class="details">
            <p class="map-name">{external_name}</p>
            <p class="internal-name">{internal_name}</p>
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
    .active:first-child {
        margin-top: 3px;
    }
    .details {
        justify-self: center;
        text-align: left;
        padding-left: 5%;
    }
</style>