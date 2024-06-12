import { writable } from "svelte/store";

export const current_map = writable("gridmap_v2"); // default map is gridmap v2

export const deleted_mods = writable<string[]>([]);
export const deleted_maps = writable<string[]>([]);