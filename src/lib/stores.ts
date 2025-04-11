import { writable } from "svelte/store";

export const currentMap = writable<string>();

export const needsRestart = writable(0); // 0 = no restart necessary, >0 = restart necessary

export const modlistHasBeenChanged = writable(false);
export const maplistHasBeenChanged = writable(false);

export const showSettingsModal = writable(false);