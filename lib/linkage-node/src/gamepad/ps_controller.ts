import { clamp, mapRange } from '../util';
import {
  AxisControl,
  ButtonControl,
  Control,
  EventType
} from './gamepad_manager';

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
   * @returns Whether or not the left joystick button is pressed.
   */
  leftJoystickButton = false;

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
   * @returns Whether or not the right joystick button is pressed.
   */
  rightJoystickButton = false;

  /**
   * @returns Whether or not the share button is pressed.
   */
  share = false;

  /**
   * @returns Whether or not the options button is pressed.
   */
  options = false;

  /**
   * @returns Whether or not the home button is pressed.
   */
  home = false;

  public setValue(eventType: EventType, control: Control, value: number): void {
    if (eventType === EventType.AXIS_CHANGED) {
      switch (control as AxisControl) {
        case AxisControl.LEFT_STICK_X:
          this.leftJoystickX = clamp(value / 255, 0, 1);
          break;
        case AxisControl.LEFT_STICK_Y:
          this.leftJoystickY = clamp(value / 255, 0, 1);
          break;
        case AxisControl.RIGHT_STICK_X:
          this.rightJoystickX = clamp(value / 255, 0, 1);
          break;
        case AxisControl.RIGHT_STICK_Y:
          this.rightJoystickY = clamp(value / 255, 0, 1);
          break;
        case AxisControl.DPAD_X:
          this.dpadLeft = false;
          this.dpadRight = false;
          if (value <= -0.5) this.dpadLeft = true;
          else if (value >= 0.5) this.dpadRight = true;
          break;
        case AxisControl.DPAD_Y:
          this.dpadDown = false;
          this.dpadUp = false;
          if (value <= -0.5) this.dpadDown = true;
          else if (value >= 0.5) this.dpadUp = true;
          break;
        default:
          break;
      }
    } else if (eventType === EventType.BUTTON_CHANGED) {
      switch (control as ButtonControl) {
        case ButtonControl.SOUTH:
          this.cross = value >= 127;
          break;
        case ButtonControl.EAST:
          this.circle = value >= 127;
          break;
        case ButtonControl.NORTH:
          this.triangle = value >= 127;
          break;
        case ButtonControl.WEST:
          this.square = value >= 127;
          break;
        case ButtonControl.LEFT_TRIGGER:
          this.leftBumper = value >= 127;
          break;
        case ButtonControl.LEFT_TRIGGER_2:
          this.leftTrigger = clamp(value / 255, 0, 1);
          break;
        case ButtonControl.RIGHT_TRIGGER:
          this.rightBumper = value >= 127;
          break;
        case ButtonControl.RIGHT_TRIGGER_2:
          this.rightTrigger = clamp(value / 255, 0, 1);
          break;
        case ButtonControl.SELECT:
          this.share = value >= 127;
          break;
        case ButtonControl.START:
          this.options = value >= 127;
          break;
        case ButtonControl.MODE:
          this.home = value >= 127;
          break;
        case ButtonControl.LEFT_THUMB:
          this.leftJoystickButton = value >= 127;
          break;
        case ButtonControl.RIGHT_THUMB:
          this.rightJoystickButton = value >= 127;
          break;
        case ButtonControl.DPAD_UP:
          this.dpadUp = value >= 127;
          break;
        case ButtonControl.DPAD_DOWN:
          this.dpadDown = value >= 127;
          break;
        case ButtonControl.DPAD_LEFT:
          this.dpadLeft = value >= 127;
          break;
        case ButtonControl.DPAD_RIGHT:
          this.dpadRight = value >= 127;
          break;
        default:
          break;
      }
    }
  }
}
