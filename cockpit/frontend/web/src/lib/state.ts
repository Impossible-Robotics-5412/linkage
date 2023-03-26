import { writable } from 'svelte/store';

export const state = writable<State>({
	enabled: false,
	host: '127.0.0.1',
	port: 3012
});

export interface State {
	enabled: boolean;

	host: string;
	port: number;
}
