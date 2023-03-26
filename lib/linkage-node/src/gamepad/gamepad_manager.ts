import { PsController } from './ps_controller';

export enum EventType {
  BUTTON_CHANGED = 0,
  AXIS_CHANGED = 1,
  CONNECTED = 2,
  DISCONNECTED = 3
}

export type Control = ButtonControl | AxisControl;

export enum ButtonControl {
  // Action Pad
  SOUTH = 1,
  EAST = 2,
  NORTH = 4,
  WEST = 5,
  C = 3,
  Z = 6,
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

  UNKNOWN = 0
}

export enum AxisControl {
  LEFT_STICK_X = 1,
  LEFT_STICK_Y = 2,
  LEFT_Z = 3,
  RIGHT_STICK_X = 4,
  RIGHT_STICK_Y = 5,
  RIGHT_Z = 6,
  DPAD_X = 7,
  DPAD_Y = 8,
  UNKNOWN = 0
}

export class GamepadManager {
  public static readonly shared = new GamepadManager();

  private _gamepads: Record<number, PsController> = {};

  public parseGamepadEvent(
    gamepadId: number,
    eventType: EventType,
    control: number,
    value: number
  ): void {
    if (this._gamepads[gamepadId]) {
      this._gamepads[gamepadId].setValue(eventType, control, value);
    } else {
      this._gamepads[gamepadId] = new PsController();
    }
  }

  public get primaryGamepad(): PsController {
    return this._gamepads[0] ?? new PsController();
  }

  public get secondaryGamepad(): PsController {
    return this._gamepads[1] ?? new PsController();
  }
}
