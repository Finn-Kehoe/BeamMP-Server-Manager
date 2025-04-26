<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { showSettingsModal, needsRestart } from "./stores";
    import ModalTemplate from "./ModalTemplate.svelte";
    import { ServerSettings, ManagerSettings } from "./settings";
    import { onMount } from "svelte";

    let serverSettings: ServerSettings = {server_name: "", auth_key: "", is_private: true, max_cars: 0, max_players: 0};
    let managerSettings: ManagerSettings = {auto_update: true};

    let tempServerName = serverSettings.server_name;
    let tempAuthKey = serverSettings.auth_key;
    let tempIsPrivate = serverSettings.is_private;
    let tempMaxPlayers = serverSettings.max_players;
    let tempMaxCars = serverSettings.max_cars;
    let tempAutoUpdate = managerSettings.auto_update;

    let initialized = 0;

    async function getServerSettings() {
        await invoke("read_server_settings")
            .then((settings: ServerSettings) => {serverSettings = settings; initialized++;})
            .catch((e) => console.log(e))
    }

    async function getManagerSettings() {
        await invoke("read_manager_settings")
            .then((settings: ManagerSettings) => {managerSettings = settings; initialized++;})
            .catch((e) => console.log(e))
    }

    async function openConfigFile() {
        await invoke("user_open_config_file");
    }

    onMount(() => {
        getServerSettings();
        getManagerSettings();
    });

    async function sendSettingsChange() {
        if (tempServerName !== serverSettings.server_name) {
            await invoke("update_server_config", {key: "Name", value: { type: "Str", value: tempServerName }}).then(() => serverSettings.server_name = tempServerName, () => tempServerName = serverSettings.server_name);
        } else if (tempAuthKey !== serverSettings.auth_key) {
            await invoke("update_server_config", {key: "AuthKey", value: { type: "Str", value: tempAuthKey }}).then(() => serverSettings.auth_key = tempAuthKey, () => tempAuthKey = serverSettings.auth_key);
        } else if (tempIsPrivate !== serverSettings.is_private) {
            await invoke("update_server_config", {key: "Private", value: { type: "Bool", value: tempIsPrivate }}).then(() => serverSettings.is_private = tempIsPrivate, () => tempIsPrivate = serverSettings.is_private);
        } else if (tempMaxPlayers !== serverSettings.max_players) {
            await invoke("update_server_config", {key: "MaxPlayers", value: { type: "Num", value: tempMaxPlayers }}).then(() => serverSettings.max_players = tempMaxPlayers, () => tempMaxPlayers = serverSettings.max_players);
        } else if (tempMaxCars !== serverSettings.max_cars) {
            await invoke("update_server_config", {key: "MaxCars", value: { type: "Num", value: tempMaxCars }}).then(() => serverSettings.max_cars = tempMaxCars, () => tempMaxCars = serverSettings.max_cars);
        } else if (tempAutoUpdate !== managerSettings.auto_update) {
            await invoke("update_manager_config", {key: "auto_update", value: { type: "Bool", value: tempAutoUpdate }}).then(() => managerSettings.auto_update = tempAutoUpdate, () => tempAutoUpdate = managerSettings.auto_update);
            // this setting doesn't require server restart, so subtracting from needs_restart cancels out the adding below
            needsRestart.update((_needsRestart) => _needsRestart - 1);
        }
        needsRestart.update((_needsRestart) => _needsRestart + 1);
    }

    $: if (initialized === 2) {
        tempServerName = serverSettings.server_name;
        tempAuthKey = serverSettings.auth_key;
        tempIsPrivate = serverSettings.is_private;
        tempMaxPlayers = serverSettings.max_players;
        tempMaxCars = serverSettings.max_cars;
        tempAutoUpdate = managerSettings.auto_update;
        initialized = 3;
    }

</script>

<ModalTemplate show={$showSettingsModal} onClose={() => $showSettingsModal = false}>
    <div class="settings-wrapper">
        <div class="header">
            <h2>Settings</h2>
        </div>
        <hr class="upper-div" />
        <div class="settings-body">
            <div class="server-name-setting">
                <div class="setting-title">
                    <h4>Server Name</h4>
                </div>
                <input class="input" bind:value={tempServerName} on:blur={sendSettingsChange}/>
            </div>
            <hr class="body-div" />
            <div class="authkey-setting">
                <div class="setting-title">
                    <h4>Authkey</h4>
                </div>
                <input class=input bind:value={tempAuthKey} on:blur={sendSettingsChange}/>
            </div>
            <hr class="body-div" />
            <div class="num private-server-setting">
                <div class="setting-title">
                    <h4>Private</h4>
                </div>
                <label class="on-off switch">
                    <input type="checkbox" bind:checked={tempIsPrivate} on:change={sendSettingsChange}>
                    <span class="slider round"></span>
                </label>
            </div>
            <hr class="body-div" />
            <div class="num max-players-setting">
                <div class="setting-title">
                    <h4>Max Players</h4>
                </div>
                <input type="number" class="num-input" bind:value={tempMaxPlayers} on:blur={sendSettingsChange}/>
            </div>
            <hr class="body-div" />
            <div class="num max-cars-setting">
                <div class="setting-title">
                    <h4>Max Cars</h4>
                </div>
                <input type="number" class="num-input" bind:value={tempMaxCars} on:blur={sendSettingsChange}/>
            </div>
            <hr class="body-div" />
            <div class="num autoupdate-setting">
                <div class="setting-title">
                    <h4>Automatically Update Server</h4>
                </div>
                <label class="on-off switch">
                    <input type="checkbox" bind:checked={tempAutoUpdate} on:change={sendSettingsChange}>
                    <span class="slider round"></span>
                </label>
            </div>
            <hr class="body-div" />
        </div>
        <div class="bottom-buttons">
            <div class="config-file-button">
                <button class="button" on:click={openConfigFile}>Open Server Config File</button>
            </div>
            <div class="close-button">
                <button class="button" on:click={() => $showSettingsModal = false}>Close</button>
            </div>
        </div>
    </div>
</ModalTemplate>

<style>
    .header h2 {
        margin-top: 0%;
        margin-bottom: 3%;
    }

    .settings-body {
        padding-top: 0 !important;
        text-align: left;
        padding: 2%;
        overflow: hidden;
        overflow-y: scroll;
    }

    .settings-body div {
        padding-top: 1.2%;
        padding-bottom: 1.2%;
    }

    .server-name-setting, .authkey-setting {
        margin-bottom: 1%;
    }

    ::-webkit-scrollbar {
        background-color: transparent;
        width: 10px;
    }

    ::-webkit-scrollbar-thumb {
        background-clip: padding-box;
        border: 3px solid transparent;
        /* width: 7px; */
        border-radius: 5%;
        background-color: #383838;
    }

    ::-webkit-scrollbar-thumb:hover {
        background-color: #3d3d3d;
    }

    .setting-title h4 {
        margin-top: 1%;
        margin-bottom: 1%;
    }

    hr {
        border: 0;
        padding: 0;
        margin: 2px;
        display: block;
    }

    .upper-div {
        justify-self: center;
        height: 2px;
        width: 100%;
        border-top: 2px solid #3d3d3d;
    }

    .body-div {
        height: 1px;
        border-top: 1px solid #3d3d3d;
    }

    .num {
        display: flex;
        align-items: center;
        padding-top: 2.5%;
        padding-bottom: 2.5%;
    }

    .num div.setting-title h4 {
        margin: auto + 1%;
    }

    .input {
        width: 85%;
        padding-left: 1em;
        padding-right: 1em;
        text-align: center;
    }

    .num-input {
        padding-left: 2%;
        padding-right: 2%;
        width: 10%;
        margin-left: 7%;
        text-align: center;
    }

    .switch {
        margin-left: 5%;
    }

    input::-webkit-outer-spin-button,
    input::-webkit-inner-spin-button {
        -webkit-appearance: none;
        margin: 0;
    }

    .bottom-buttons {
        display: flex;
    }

    .close-button {
        margin-left: auto;
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