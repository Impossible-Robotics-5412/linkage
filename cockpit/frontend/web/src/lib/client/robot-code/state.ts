import { writable } from 'svelte/store';

export const robotCodeState = writable<RobotCodeState>({
	enabled: false
});

export interface RobotCodeState {
	enabled: boolean;
}
