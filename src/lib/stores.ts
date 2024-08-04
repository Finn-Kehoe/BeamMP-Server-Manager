import { writable } from "svelte/store";

export const current_map = writable<string>();

export const needs_restart = writable(0); // 0 = no restart necessary, >0 = restart necessary

export const modlist_has_been_changed = writable(false);
export const maplist_has_been_changed = writable(false);