import { writable } from 'svelte/store';

export interface RobotCodeState {
	enabled: boolean;
}

export const robotCodeState = writable<RobotCodeState>({
	enabled: false
});
