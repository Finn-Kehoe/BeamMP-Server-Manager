<script lang="ts">
    import ModalTemplate from "./ModalTemplate.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { showAuthModal } from "./stores";
    import { openUrl } from "@tauri-apps/plugin-opener";

    let tempAuthKey = ""

    async function updateAuthKey() {
        if (tempAuthKey !== "") {
            await invoke("update_server_config", {key: "AuthKey", value: { type: "Str", value: tempAuthKey }}).then(() => $showAuthModal = false, () => {});
        }
    }

    async function openHelpUrl() {
        await openUrl("https://docs.beammp.com/server/create-a-server/#2-obtaining-an-authentication-key");
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
            <p class="help-text">For information about how to obtain an AuthKey, click <button class="href-button" on:click={openHelpUrl}>here</button>.</p>
        </div>
        <div class="footer">
            <div class="close-button">
                <button class="button" on:click={() => { if (tempAuthKey != "") { $showAuthModal = false }}}>Close</button>
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
        background-color: #1a1a1a;
    }
    .help-text {
        font-size: 11pt;
    }
    .href-button {
        background-color: inherit;
        border: 0;
        padding: 0;
        box-shadow: none;
        color: #4c93ff;
    }
    .href-button:hover {
        background-color: inherit;
        color: #0061a1;
    }
</style>