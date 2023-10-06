<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";

    let hasAuth: boolean;
    invoke("has_authkey")
        .then((result: boolean) => hasAuth = result)
        .catch(() => hasAuth = false);
        
</script>