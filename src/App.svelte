<script lang="ts">
  import SettingsOverlay from "./lib/SettingsOverlay.svelte";
  import ServerStatusBar from "./lib/ServerStatusBar.svelte";
  import MapList from "./lib/MapList.svelte";
  import ModList from "./lib/ModList.svelte";
  import BottomBar from "./lib/BottomBar.svelte";

  import { listen } from "@tauri-apps/api/event"
  import { invoke } from "@tauri-apps/api/tauri";
  import { modlist_has_been_changed, maplist_has_been_changed } from "./lib/stores"
  import { ModType } from "./lib/mod";

  async function handleFiles(event) {
    let thisModType: ModType;
    for (let i = 0; i < event.payload.length; i++) {
      // we get path to file from file drop event
      let thisPath = event.payload[i];
      await invoke("add_mod", { path: thisPath })
        .then((modType: ModType) => thisModType = modType)
        .catch((e) => {console.log("Error adding mod: ", e)});
      
      if (thisModType === ModType.Content) {
        modlist_has_been_changed.set(true);
      } else if (thisModType === ModType.Map) {
        maplist_has_been_changed.set(true);
      }
      
    }
  }

  listen("tauri://file-drop", event => {
    handleFiles(event);
  });
</script>

<main class="container">
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
</style>
