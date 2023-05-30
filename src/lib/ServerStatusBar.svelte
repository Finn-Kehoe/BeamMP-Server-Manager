<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";

    async function restart_server() {
        await invoke("restart_server")
    }
</script>