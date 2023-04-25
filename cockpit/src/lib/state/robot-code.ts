import { writable } from 'svelte/store';

export interface RobotCodeState {
	enabled: boolean;
	changingState: boolean;
}

export const robotCodeState = writable<RobotCodeState>({
	enabled: false,
	changingState: false
});
