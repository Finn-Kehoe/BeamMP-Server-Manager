<script lang="ts">
    import ModalTemplate from "./ModalTemplate.svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import { showAuthModal } from "./stores";

    let tempAuthKey = ""

    async function updateAuthKey() {
        if (tempAuthKey !== "") {
            await invoke("update_server_config", {key: "AuthKey", value: { type: "Str", value: tempAuthKey }}).then(() => $showAuthModal = false, () => {});
        }
    }

</script>

<ModalTemplate show={$showAuthModal} onClose={() => $showAuthModal = false}>
    <div class="checkpoint-wrapper">
        <div class="header">
            <h2>AuthKey Checkpoint</h2>
        </div>
        <hr class="upper-div" />
        <div class="body">
            <input class="input" placeholder="Paste AuthKey Here" bind:value={tempAuthKey} on:blur={updateAuthKey} />
        </div>
        <div class="footer">
            <div class="close-button">
                <button class="button" on:click={() => $showAuthModal = false}>Close</button>
            </div>
        </div>
    </div>
</ModalTemplate>

<style>
    .header h2 {
        margin-top: 0%;
        margin-bottom: 3%;
    }
    hr {
        border: 0;
        padding: 0;
        margin: 2px;
        display: block;
        justify-self: center;
        height: 2px;
        width: 100%;
        border-top: 2px solid #3d3d3d;
    }
    .body {
        padding-top: 2%;
        padding-bottom: 3%;
    }
    .input {
        width: 85%;
        padding-left: 1em;
        padding-right: 1em;
        text-align: center;
    }
</style>