<script lang="ts">
  import ServerStatusBar from "./lib/ServerStatusBar.svelte";
  import MapList from "./lib/MapList.svelte";
  import ModList from "./lib/ModList.svelte";
</script>

<main class="container">
  <div class="server-status-bar">
    <ServerStatusBar />
  </div>

  <div class="main-content">
    <div class="content-lists">
      <div class="mod-list">
        <ModList />
      </div>
      <div class="map-list">
        <MapList />
      </div>
    </div>
  </div>
</main>

<style>
  .main-content {
    display: flex;
  }

  .content-lists {
    display: flex;
    padding: 15px;
  }
</style>
