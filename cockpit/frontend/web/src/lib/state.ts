import { writable } from "svelte/store";

export const state = writable<State>({
  enabled: false,
});

export interface State {
  enabled: boolean;
}
