import { writable } from "svelte/store";

export const current_map = writable("gridmap_v2"); // default map is gridmap v2

export const needs_restart = writable(0); // 0 = no restart necessary, >0 = restart necessary

export const deleted_mods = writable<string[]>([]);
export const deleted_maps = writable<string[]>([]);