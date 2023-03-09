import {
  GamepadManager,
  Robot,
  SparkMotor,
  Subsystem,
} from "@impossiblerobotics/linkage";

class DriveSubsystem extends Subsystem {
  private leftMotor = new SparkMotor(0);
  private rightMotor = new SparkMotor(1);

  public tick(): void {
    const leftJoystickY = GamepadManager.shared.primaryGamepad.leftJoystickY;
    const rightJoystickY = GamepadManager.shared.primaryGamepad.rightJoystickY;

    this.leftMotor.setSpeedPercentage(leftJoystickY);
    this.rightMotor.setSpeedPercentage(rightJoystickY);
  }
}

class TestRobot extends Robot {
  public constructor() {
    super();

    this.registerSubsystem(new DriveSubsystem(this));
  }
}

new TestRobot().run();
