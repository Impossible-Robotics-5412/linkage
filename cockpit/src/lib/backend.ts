import { get, writable } from 'svelte/store';
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

export const systemInfo = writable<SystemInfo | undefined>(
	undefined,
	$systemInfo => {
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
	}
);

export interface RobotCodeState {
	enabled: boolean;
	changingState: boolean;
}

export const robotCode = writable<RobotCodeState>(
	{
		enabled: false,
		changingState: false
	},
	() => {
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
);

export interface GamepadState {
	gamepads: { [id: GamepadId]: GamepadData };
}

export const gamepadState = writable<GamepadState>(
	{
		gamepads: {}
	},
	set => {
		invoke('start_event_listener').then(() => {
			listen('gamepad_event', event => {
				const gamepadInputEvent = parseGamepadInputEvent(event.payload);
				if (!gamepadInputEvent) return;

				const state = get(gamepadState);
				if (gamepadInputEvent.eventType === EventType.DISCONNECTED) {
					delete state.gamepads[gamepadInputEvent.gamepadId];
				} else {
					if (!state.gamepads[gamepadInputEvent.gamepadId])
						state.gamepads[gamepadInputEvent.gamepadId] =
							new GamepadData(gamepadInputEvent.gamepadId);
					state.gamepads[
						gamepadInputEvent.gamepadId
					].handleGamepadInputEvent(gamepadInputEvent);
				}
				set(state);
			});
		});
	}
);

export async function enableRobotCode() {
	robotCode.update($robotCode => {
		$robotCode.changingState = true;
		return $robotCode;
	});
	return invoke('enable');
}

export async function disableRobotCode() {
	robotCode.update($robotCode => {
		$robotCode.changingState = true;
		return $robotCode;
	});
	return invoke('disable');
}
