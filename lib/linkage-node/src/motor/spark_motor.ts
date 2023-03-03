import { CarburetorConnection } from "../carburetor/carburetor_connection";
import { clampMotorValue } from "../util";

/**
 * Represents a Spark motor controller.
 */
export class SparkMotor {
  /**
   * Creates a new reference to a spark motor controller.
   *
   * @param port The Pwm Port of the motor.
   */
  public constructor(public readonly port: number) {}

  /**
   * Sets the speed of the motor.
   *
   * @param percentage a number between -1.0 and 1.0 representing full speed backwards and full speed forwards respectively.
   * A value of 0.0 sets the motor to it's neutral mode.
   */
  public setSpeedPercentage(percentage: number): void {
    CarburetorConnection.shared
      .sendMotorPacket(this.port, clampMotorValue(percentage));
  }
}
