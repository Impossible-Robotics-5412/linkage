export enum ButtonControl {
	// Action Pad
	SOUTH = 1,
	EAST = 2,
	NORTH = 4,
	WEST = 5,
	// Triggers
	LEFT_TRIGGER = 7,
	LEFT_TRIGGER_2 = 9,
	RIGHT_TRIGGER = 8,
	RIGHT_TRIGGER_2 = 10,
	// Menu Pad
	SELECT = 11,
	START = 12,
	MODE = 13,
	// Sticks
	LEFT_THUMB = 14,
	RIGHT_THUMB = 15,
	// D-Pad
	DPAD_UP = 16,
	DPAD_DOWN = 17,
	DPAD_LEFT = 18,
	DPAD_RIGHT = 19,
	// Other
	C = 3,
	Z = 6
}

export enum AxisControl {
	LEFT_STICK_X = 1,
	LEFT_STICK_Y = 2,
	LEFT_Z = 3,
	RIGHT_STICK_X = 4,
	RIGHT_STICK_Y = 5,
	RIGHT_Z = 6,
	DPAD_X = 7,
	DPAD_Y = 8
}

export type GamepadId = number;

export enum EventType {
	BUTTON_CHANGED = 0,
	AXIS_CHANGED = 1,
	CONNECTED = 2,
	DISCONNECTED = 3
}

export interface GamepadInputEvent {
	gamepadId: GamepadId;
	eventType: EventType;
	control: AxisControl | ButtonControl;
	value: number;
}

export function parseGamepadInputEvent(
	data: unknown
): GamepadInputEvent | null {
	return {
		control: data.GamepadInputEvent.control,
		eventType: data.GamepadInputEvent.event_type,
		gamepadId: data.GamepadInputEvent.gamepad_id,
		value: data.GamepadInputEvent.value
	};
}

export class GamepadData {
	public axis: Record<AxisControl, number> = {
		[AxisControl.LEFT_STICK_X]: 127.0,
		[AxisControl.LEFT_STICK_Y]: 127.0,
		[AxisControl.LEFT_Z]: 127.0,
		[AxisControl.RIGHT_STICK_X]: 127.0,
		[AxisControl.RIGHT_STICK_Y]: 127.0,
		[AxisControl.RIGHT_Z]: 127.0,
		[AxisControl.DPAD_X]: 127.0,
		[AxisControl.DPAD_Y]: 127.0
	};
	public buttons: Record<ButtonControl, number> = {
		[ButtonControl.SOUTH]: 0.0,
		[ButtonControl.EAST]: 0.0,
		[ButtonControl.NORTH]: 0.0,
		[ButtonControl.WEST]: 0.0,
		[ButtonControl.C]: 0.0,
		[ButtonControl.Z]: 0.0,
		[ButtonControl.LEFT_TRIGGER]: 0.0,
		[ButtonControl.LEFT_TRIGGER_2]: 0.0,
		[ButtonControl.RIGHT_TRIGGER]: 0.0,
		[ButtonControl.RIGHT_TRIGGER_2]: 0.0,
		[ButtonControl.SELECT]: 0.0,
		[ButtonControl.START]: 0.0,
		[ButtonControl.MODE]: 0.0,
		[ButtonControl.LEFT_THUMB]: 0.0,
		[ButtonControl.RIGHT_THUMB]: 0.0,
		[ButtonControl.DPAD_UP]: 0.0,
		[ButtonControl.DPAD_DOWN]: 0.0,
		[ButtonControl.DPAD_LEFT]: 0.0,
		[ButtonControl.DPAD_RIGHT]: 0.0
	};

	public constructor(public readonly id: GamepadId) {}

	public handleGamepadInputEvent(gamepadInputEvent: GamepadInputEvent) {
		if (gamepadInputEvent.eventType === EventType.BUTTON_CHANGED) {
			this.buttons[gamepadInputEvent.control] = gamepadInputEvent.value;
		} else if (gamepadInputEvent.eventType === EventType.AXIS_CHANGED) {
			this.axis[gamepadInputEvent.control] = gamepadInputEvent.value;
		}
	}
}
