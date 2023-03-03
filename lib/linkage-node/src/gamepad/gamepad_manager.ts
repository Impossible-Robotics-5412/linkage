import { PsController } from "./ps_controller";

export class GamepadManager {
  public static readonly shared = new GamepadManager();

  private _gamepads: Record<number, PsController> = {};

  public parseGamepadEvent(
    gamepadId: number,
    codePage: number,
    codeUsage: number,
    value: number,
  ): void {
    if (this._gamepads[gamepadId]) {
      this._gamepads[gamepadId].setValue(codePage, codeUsage, value);
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
