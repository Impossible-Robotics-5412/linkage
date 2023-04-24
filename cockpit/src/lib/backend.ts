import { writable } from 'svelte/store';
import type { SystemInfo } from '$lib/types/system-info';
import { listen } from '@tauri-apps/api/event';
import { loggerState } from '$lib/logger';
import { invoke } from '@tauri-apps/api/tauri';
import type { GamepadId } from '$lib/gamepad-data';
import {
	EventType,
	GamepadData,
	parseGamepadInputEvent
} from '$lib/gamepad-data';

export const systemInfo = writable<SystemInfo | undefined>(undefined);

export interface RobotCodeState {
	enabled: boolean;
	changingState: boolean;
}

export const robotCode = writable<RobotCodeState>({
	enabled: false,
	changingState: false
});

export interface GamepadState {
	gamepads: { [id: GamepadId]: GamepadData };
}

export const gamepadState = writable<GamepadState>({
	gamepads: {}
});

export async function enableRobotCode() {
	robotCode.update($robotCode => {
		if (!$robotCode.enabled) $robotCode.changingState = true;
		return $robotCode;
	});
	return invoke('enable');
}

export async function disableRobotCode() {
	robotCode.update($robotCode => {
		if ($robotCode.enabled) $robotCode.changingState = true;
		return $robotCode;
	});
	return invoke('disable');
}

export function initializeListeners() {
	initializeSystemInfoListener();
	initializeGamepadEventListener();
	initializeLinkageLibStateListener();
}

function initializeLinkageLibStateListener() {
	listen('linkage_lib_state_change', event => {
		robotCode.update($robotCode => {
			$robotCode.changingState = false;
			if (event.payload === 'Enabled') {
				$robotCode.enabled = true;
				loggerState.update($loggerState => {
					$loggerState.selectedTabId = 'linkage';
					return $loggerState;
				});
			} else if (event.payload === 'Disabled') {
				$robotCode.enabled = false;
				loggerState.update($loggerState => {
					$loggerState.selectedTabId = 'cockpit-backend';
					return $loggerState;
				});
			}

			return $robotCode;
		});
	});
}

function initializeGamepadEventListener() {
	invoke('start_event_listener').then(() => {
		listen('gamepad_event', event => {
			gamepadState.update($gamepadState => {
				const gamepadInputEvent = parseGamepadInputEvent(event.payload);
				if (!gamepadInputEvent) return;

				if (gamepadInputEvent.eventType === EventType.DISCONNECTED) {
					delete $gamepadState.gamepads[gamepadInputEvent.gamepadId];
				} else {
					if (!$gamepadState.gamepads[gamepadInputEvent.gamepadId])
						$gamepadState.gamepads[gamepadInputEvent.gamepadId] =
							new GamepadData(gamepadInputEvent.gamepadId);
					$gamepadState.gamepads[
						gamepadInputEvent.gamepadId
					].handleGamepadInputEvent(gamepadInputEvent);
				}

				return $gamepadState;
			});
		});
	});
}

function initializeSystemInfoListener() {
	systemInfo.update($systemInfo => {
		// TODO: Check periodically if we have a connection with the robot.
		//  	 Currently you have to reload the app to
		//       Connect after a restart of the robot.

		const timeout_ms = 1000;

		let lastCheck = Date.now();

		setInterval(() => {
			if (Date.now() - lastCheck >= timeout_ms) {
				$systemInfo = undefined;
			}
		}, timeout_ms);

		invoke('start_gauge_connection')
			.then(() => {
				listen('received_system_info', event => {
					systemInfo.update($systemInfo => {
						$systemInfo = event.payload as SystemInfo;
						return $systemInfo;
					});

					lastCheck = Date.now();
				});
			})
			.catch(error => {
				console.error('Could connect to Gauge: ' + error);
			});

		return $systemInfo;
	});
}
