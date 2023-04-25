import { GamepadData, type GamepadId } from '$lib/gamepad-data';
import { writable } from 'svelte/store';

export interface GamepadState {
	gamepads: { [id: GamepadId]: GamepadData };
}

export const gamepadState = writable<GamepadState>({
	gamepads: {}
});
