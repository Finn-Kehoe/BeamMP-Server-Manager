<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { current_map } from "./stores";

    export let internal_name: string;
    export let external_name: string;

    let is_active = false;

    async function setAsCurrentMap() {
        await invoke("change_map", { mapName: internal_name });
        current_map.set(internal_name);
    }

    current_map.subscribe((map) => {
        if (map == internal_name) {
            is_active = true;
        } else {
            is_active = false;
        }
    });
</script>

<li>
    <div class="main-body" on:dblclick={setAsCurrentMap} class:active={is_active}>
        <div class="details">
            <p class="map-name">{external_name}</p>
            <p class="internal-name">{internal_name}</p>
        </div>
    </div>
</li>

<style>
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
    }
</style>