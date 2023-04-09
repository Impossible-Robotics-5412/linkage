import { writable } from 'svelte/store';

export interface RobotCodeState {
	enabled: boolean;
	changing: boolean;
}

export const robotCodeState = writable<RobotCodeState>({
	enabled: false,
	changing: false
});
