import { createConnection, Socket } from "net";
import { clampMotorValue } from "../util";

export class CarburetorConnection {
  public static readonly shared = new CarburetorConnection();

  private connection: Socket | undefined;

  public async connect(): Promise<void> {
    try {
      if (this.connection) {
        this.close();
      }

      // FIXME: the hostname and port should be in a config file.
      this.connection = createConnection({ host: "0.0.0.0", port: 48862 });

      this.connection.on("connect", () => {
        console.log(`[CarburetorConnection] Connected.`);
      });

      this.connection.on("error", (error) => {
        console.error(`[CarburetorConnection] ${error}`);
      });
    } catch (error) {
      console.log(`[CarburetorConnection] Failed to connect: ${error}.`);
    }
  }

  public close(): void {
    if (!this.connection) {
      return;
    }

    this.connection.destroy();
    this.connection = undefined;
    console.log(`[CarburetorConnection] Disconnected`);
  }

  public sendMotorPacket(port: number, percentage: number) {
    if (!this.connection) {
      console.warn(
        `[CarburetorConnection] Failed to send motor packet: No connection found.`,
      );
    }

    const view = new DataView(new ArrayBuffer(4));
    view.setFloat32(0, clampMotorValue(percentage), false);

    this.connection?.write(
      new Uint8Array([
        CarburetorInstruction.CONTROL_MOTOR,
        port,
        0,
        0,
        view.getInt8(0),
        view.getInt8(1),
        view.getInt8(2),
        view.getInt8(3),
      ]),
    );
  }
}

enum CarburetorInstruction {
  CONTROL_MOTOR = 0,
  QUERY_BATTERY = 100,
  QUERY_MEMORY = 101,
  QUERY_CPU = 102,
}
