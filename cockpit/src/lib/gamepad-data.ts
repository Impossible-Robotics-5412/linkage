enum ButtonControl {
	// Action Pad
	South = 1,
	East = 2,
	North = 4,
	West = 5,
	C = 3,
	Z = 6,
	// Triggers
	LeftTrigger = 7,
	LeftTrigger2 = 9,
	RightTrigger = 8,
	RightTrigger2 = 10,
	// Menu Pad
	Select = 11,
	Start = 12,
	Mode = 13,
	// Sticks
	LeftThumb = 14,
	RightThumb = 15,
	// D-Pad
	DpadUp = 16,
	DpadDown = 17,
	DpadLeft = 18,
	DpadRight = 19,

	Unknown = 0,
}

enum AxisControl {
	LeftStickX = 1,
	LeftStickY = 2,
	LeftZ = 3,
	RightStickX = 4,
	RightStickY = 5,
	RightZ = 6,
	DpadX = 7,
	DpadY = 8,
	Unknown = 0,
}

export type GamepadId = number;

export enum EventType {
	BUTTON_CHANGED = 0,
	AXIS_CHANGED = 1,
	CONNECTED = 2,
	DISCONNECTED = 3,
}

export interface GamepadInputEvent {
	gamepadId: GamepadId,
	eventType: EventType,
	control: AxisControl | ButtonControl,
	value: number
}

export function parseGamepadInputEvent(data: unknown): GamepadInputEvent|null {
	return {
		control: data.GamepadInputEvent.control,
		eventType: data.GamepadInputEvent.event_type,
		gamepadId: data.GamepadInputEvent.gamepad_id,
		value: data.GamepadInputEvent.value
	}
}

export class GamepadData {
	public axis: { [control: typeof AxisControl]: number } = {};
	public buttons: { [control: typeof ButtonControl]: number } = {};

	public handleGamepadInputEvent(gamepadInputEvent: GamepadInputEvent) {
		if (gamepadInputEvent.eventType === EventType.BUTTON_CHANGED) {
			this.buttons[gamepadInputEvent.control] = gamepadInputEvent.value
		} else if (gamepadInputEvent.eventType === EventType.AXIS_CHANGED) {
			this.axis[gamepadInputEvent.control] = gamepadInputEvent.value
		}
	}
}