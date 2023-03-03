import { Robot } from "../robot";

/**
 * A subsystem is a single part of the robot (e.g. drivetrain or intake).
 */
export abstract class Subsystem {
  /**
   * The tick function is called once every 20ms.
   * @protected
   * @abstract
   */
  public tick(): void {}

  /**
   * The shutdown function is called when the program exits.
   * @protected
   */
  public shutdown(): void {}

  /**
   * Creates a new subsystem.
   *
   * @param robot The robot class associated with this subsystem.
   */
  public constructor(protected robot: Robot) {}
}
