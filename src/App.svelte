<script lang="ts">
  import AuthKeyCheckpoint from "./lib/AuthKeyCheckpoint.svelte";
  import SettingsOverlay from "./lib/SettingsOverlay.svelte";
  import ServerStatusBar from "./lib/ServerStatusBar.svelte";
  import MapList from "./lib/MapList.svelte";
  import ModList from "./lib/ModList.svelte";
  import BottomBar from "./lib/BottomBar.svelte";

  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { getCurrentWebview } from '@tauri-apps/api/webview';
  import { modlistHasBeenChanged, maplistHasBeenChanged } from "./lib/stores"
  import { ModType } from "./lib/mod";

  async function handleFiles(files) {
    let thisModType: ModType;
    for (let i = 0; i < files.length; i++) {
      // we get path to file from file drop event
      let thisPath = files[i];
      await invoke("add_mod", { path: thisPath })
        .then((modType: ModType) => thisModType = modType)
        .catch((e) => {console.log("Error adding mod: ", e)});
      
      if (thisModType === ModType.Content) {
        modlistHasBeenChanged.set(true);
      } else if (thisModType === ModType.Map) {
        maplistHasBeenChanged.set(true);
      }
    }
  }

  onMount(async () => {

    const unlisten = await getCurrentWebview().onDragDropEvent((event) => {
      if (event.payload.type === 'drop') {
        console.log('Dropped files:', event.payload.paths);
        handleFiles(event.payload.paths);
      }
    });

    return () => {
      unlisten();
    };
  });
</script>

<main class="container">
  <AuthKeyCheckpoint />
  <SettingsOverlay />
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
    padding: 2%;
    padding-bottom: 0%;
  }
  .bottom-bar {
    margin-top: 0.75%;
  }
</style>
