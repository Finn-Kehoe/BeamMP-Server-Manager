<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";

    enum ServerStatus {
        Stopped = "Stopped",
        Starting = "Starting",
        Running = "Running",
    };

    onMount(() => {
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
        // updateServerStatus function runs every 2000ms (2 seconds)
        const interval = setInterval(updateServerStatus, 2000);
        updateServerStatus();

        return () => clearInterval(interval);
    });

    let statusColor = "#db7100";

    let currentServerStatus: ServerStatus;

    let startDisabled = true;
    let stopDisabled = false;
    let restartDisabled = false;

    async function startServer() {
        await invoke("start_server");
    }

    async function stopServer() {
        await invoke("close_server");
    }

    async function restartServer() {
        await invoke("restart_server");
    }

</script>

<div class="statusbar">
    <div class="circle" style:background-color={statusColor}></div>
    <p class="status-text">Server Status</p>
    <div class="control-buttons">
        <button class="button start" on:click={startServer} disabled={startDisabled}>Start</button>
        <button class="button stop" on:click={stopServer} disabled={stopDisabled}>Stop</button>
        <button class="button restart" on:click={restartServer} disabled={restartDisabled}>Restart</button>
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
</style>