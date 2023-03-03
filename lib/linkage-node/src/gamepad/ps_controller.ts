import { clamp, mapRange } from "../util";
import {
  CodePage,
  GamepadButtonPageCode,
  GamepadGenericDesktopPageCode,
} from "./codes";

/**
 * Represents the state of a xbox game controller.
 */
export class PsController {
  /**
   * @returns Whether or not the triangle button is pressed.
   */
  triangle = false;

  /**
   * @returns Whether or not the square button is pressed.
   */
  square = false;

  /**
   * @returns Whether or not the cross button is pressed.
   */
  cross = false;

  /**
   * @returns Whether or not the circle button is pressed.
   */
  circle = false;

  /**
   * @returns Whether or not the up button of the D-Pad is pressed.
   */
  dpadUp = false;

  /**
   * @returns Whether or not the down button of the D-Pad is pressed.
   */
  dpadDown = false;

  /**
   * @returns Whether or not the left button of the D-Pad is pressed.
   */
  dpadLeft = false;

  /**
   * @returns Whether or not the right button of the D-Pad is pressed.
   */
  dpadRight = false;

  /**
   * @returns Whether or not the left bumper is pressed.
   */
  leftBumper = false;

  /**
   * @returns Whether or not the right bumper is pressed.
   */
  rightBumper = false;

  /**
   * How far the left trigger has been pressed.
   *
   * @returns A value between 0.0 and 1.0 indicating not pressed at all or fully pressed respectively.
   */
  leftTrigger = 0;

  /**
   * How far the right trigger has been pressed.
   *
   * @returns A value between 0.0 and 1.0 indicating not pressed at all or fully pressed respectively.
   */
  rightTrigger = 0;
  /**
   * The X axis of the left joystick.
   *
   * @returns A value between -1.0 and 1.0 indicating completely to the left and completely to the right respectively. A value of 0.0 means it is horizontally centered.
   */
  leftJoystickX = 0;

  /**
   * The Y axis of the left joystick.
   *
   * @returns A value between -1.0 and 1.0 indicating completely at the bottom and completely at the bottom respectively. A value of 0.0 means it is vertically centered.
   */
  leftJoystickY = 0;

  /**
   * The X axis of the right joystick.
   *
   * @returns A value between -1.0 and 1.0 indicating completely to the left and completely to the right respectively. A value of 0.0 means it is horizontally centered.
   */
  rightJoystickX = 0;

  /**
   * The Y axis of the right joystick.
   *
   * @returns A value between -1.0 and 1.0 indicating completely at the bottom and completely at the bottom respectively. A value of 0.0 means it is vertically centered.
   */
  rightJoystickY = 0;

  /**
   * @returns Whether or not the share button is pressed.
   */
  share = false;

  /**
   * @returns Whether or not the options button is pressed.
   */
  options = false;

  public setValue(codePage: number, codeUsage: number, value: number): void {
    if (codePage === CodePage.BUTTON_PAGE) {
      switch (codeUsage) {
        case GamepadButtonPageCode.USAGE_BTN_SIMPLE_0:
          this.square = value > 127;
          break;
        case GamepadButtonPageCode.USAGE_BTN_SIMPLE_1:
          this.cross = value > 127;
          break;
        case GamepadButtonPageCode.USAGE_BTN_SIMPLE_2:
          this.circle = value > 127;
          break;
        case GamepadButtonPageCode.USAGE_BTN_SIMPLE_3:
          this.triangle = value > 127;
          break;

        case GamepadButtonPageCode.USAGE_BTN_DPAD_UP:
          this.dpadUp = value > 127;
          break;
        case GamepadButtonPageCode.USAGE_BTN_DPAD_RIGHT:
          this.dpadRight = value > 127;
          break;
        case GamepadButtonPageCode.USAGE_BTN_DPAD_DOWN:
          this.dpadDown = value > 127;
          break;
        case GamepadButtonPageCode.USAGE_BTN_DPAD_LEFT:
          this.dpadLeft = value > 127;
          break;

        case GamepadButtonPageCode.USAGE_BTN_LT:
          this.leftBumper = value > 127;
          break;
        case GamepadButtonPageCode.USAGE_BTN_RT:
          this.rightBumper = value > 127;
          break;

        case GamepadButtonPageCode.USAGE_BTN_START:
          this.share = value > 127;
          break;
        case GamepadButtonPageCode.USAGE_BTN_SELECT:
          this.options = value > 127;
          break;

        default:
          break;
      }
    } else if (codePage === CodePage.GENERIC_DESKTOP) {
      switch (codeUsage) {
        case GamepadGenericDesktopPageCode.USAGE_AXIS_RSTICKX:
          this.leftTrigger = clamp(value / 255, 0, 1);
          break;
        case GamepadGenericDesktopPageCode.USAGE_AXIS_RSTICKY:
          this.rightTrigger = clamp(value / 255, 0, 1);
          break;

        case GamepadGenericDesktopPageCode.USAGE_AXIS_LSTICKX:
          this.leftJoystickX = mapRange(value, 0, 255, -1, 1);
          break;
        case GamepadGenericDesktopPageCode.USAGE_AXIS_LSTICKY:
          this.leftJoystickY = mapRange(value, 0, 255, -1, 1);
          break;
        case GamepadGenericDesktopPageCode.USAGE_AXIS_RT2:
          this.rightJoystickX = mapRange(value, 0, 255, -1, 1);
          break;
        case GamepadGenericDesktopPageCode.USAGE_AXIS_LT2:
          this.rightJoystickY = mapRange(value, 0, 255, -1, 1);
          break;

        default:
          break;
      }
    }
  }
}
