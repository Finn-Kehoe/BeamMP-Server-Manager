<script lang="ts">
  import ServerStatusBar from "./lib/ServerStatusBar.svelte";
  import MapList from "./lib/MapList.svelte";
  import ModList from "./lib/ModList.svelte";
  import BottomBar from "./lib/BottomBar.svelte";
</script>

<main class="container">
  <!-- <div data-tauri-drag-region class="titlebar"></div> -->
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
  <div class="bottom-bar">
    <BottomBar />
  </div>
</main>

<style>
  .content-lists {
    display: flex;
    justify-content: center;
    padding: 15px;
  }
</style>
