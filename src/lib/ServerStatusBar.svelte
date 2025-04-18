<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";
    import { needsRestart } from "./stores";

    enum ServerStatus {
        Stopped = "Stopped",
        Starting = "Starting",
        Running = "Running",
    };

    onMount(() => {
        needsRestart.set(0);

        async function updateServerStatus() {
            await invoke("check_server_status")
                .then((status: ServerStatus) => currentServerStatus = status)
                .catch((_) => currentServerStatus = ServerStatus.Stopped);

            switch (currentServerStatus) {
                case ServerStatus.Stopped:
                    statusColor = "#c61824"; // red
                    // if server is off, we don't want to be able to turn it off again
                    startDisabled = false;
                    stopDisabled = true;
                    restartDisabled = true;
                    break;
                case ServerStatus.Starting:
                    statusColor = "#db7100" // orange
                    // if server is starting, we don't want to be able to turn it on again
                    startDisabled = true;
                    stopDisabled = false;
                    restartDisabled = false;
                    break;
                case ServerStatus.Running:
                    statusColor = "#008b02"; // green
                    // if server is on, we don't want to be able to turn it on again
                    startDisabled = true;
                    stopDisabled = false;
                    restartDisabled = false;
                    break;
                default:
                    statusColor = "#ffffff";
                    break;
            }
        }
        // updateServerStatus function runs every 1000ms (1 second)
        const interval = setInterval(updateServerStatus, 1000);
        updateServerStatus();

        return () => clearInterval(interval);
    });

    let statusColor = "#db7100";

    let currentServerStatus: ServerStatus;

    let startDisabled = true;
    let stopDisabled = false;
    let restartDisabled = false;

    let restartFlashing = false;

    async function startServer() {
        await invoke("start_server");
        if (restartFlashing) {
            needsRestart.set(0);
        }
    }

    async function stopServer() {
        await invoke("close_server");
        if (restartFlashing) {
            needsRestart.set(0);
        }
    }

    async function restartServer() {
        await invoke("restart_server");
        // reset needs_restart when server is restarted
        if (restartFlashing) {
            needsRestart.set(0);
        }
    }

    needsRestart.subscribe((_needs_restart) => {
        // if needs_restart is "true" (non-zero), then make restart button flash
        if (_needs_restart > 0) {
            restartFlashing = true;
        } else {
            restartFlashing = false;
        }
    });

</script>

<div class="statusbar">
    <div class="circle" style:background-color={statusColor}></div>
    <p class="status-text">Server Status</p>
    <div class="control-buttons">
        <button class="button start" on:click={startServer} disabled={startDisabled}>Start</button>
        <button class="button stop" on:click={stopServer} disabled={stopDisabled}>Stop</button>
        <button class="button restart" on:click={restartServer} disabled={restartDisabled} class:flashing={(restartFlashing && !restartDisabled)}>Restart</button>
    </div>
</div>

<style>
    .statusbar {
        display: flex;
        align-items: center;
        background-color: #2b2b2b;
        border-radius: 5px;
        padding-left: 2%;
    }
    .circle {
        background-color: purple;
        height: 40px;
        width: 40px;
        border-radius: 50%;
    }
    .status-text {
        font-size: 1.5em;
        font-weight: 600;
        padding-left: 0.5%;
        margin: 20px;
    }
    .control-buttons {
        margin-left: auto;
        padding-right: 1%;
    }
    .button {
        outline: none;
        border: 1px solid transparent;
        border-radius: 8px;
        font-size: 1.2em;
        font-weight: 500;
        padding: 0.6em 1.2em;
        color: #f6f6f6;
        background-color: #383838;
    }
    .button:hover {
        cursor: pointer;
        background-color: #3d3d3d;
    }
    .button:disabled {
        cursor: default !important;
        background-color: #999999;
    }
    .flashing {
        border: 1px solid transparent;
        animation: flash 2s linear infinite alternate;
    }
    @keyframes flash {
        50% { border-color: #ff7722; }
    }
</style>