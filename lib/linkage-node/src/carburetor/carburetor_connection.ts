import { createConnection, Socket } from 'net';
import { ConfigManager } from '../config/config_manager';
import { clampMotorValue } from '../util';
import { Logger } from '../logger/logger';

export class CarburetorConnection {
	public static readonly shared = new CarburetorConnection();

	private connection: Socket | undefined;

	public async connect(): Promise<void> {
		try {
			if (this.connection) {
				this.close();
			}

			const carburetorAddress =
				ConfigManager.shared.config?.carburetorAddress;
			if (!carburetorAddress) {
				throw new Error('Carburetor address not found in config');
			}

			this.connection = createConnection({
				host: carburetorAddress.host,
				port: carburetorAddress.port
			});

			this.connection.on('connect', () => {
				Logger.info('[CarburetorConnection] Connected.');
			});

			this.connection.on('error', error => {
				Logger.error(`[CarburetorConnection] ${error}`);
			});
		} catch (error) {
			Logger.info(`[CarburetorConnection] Failed to connect: ${error}.`);
		}
	}

	public close(): void {
		if (!this.connection) {
			return;
		}

		this.connection.destroy();
		this.connection = undefined;
		Logger.info('[CarburetorConnection] Disconnected.');
	}

	public sendMotorPacket(port: number, percentage: number) {
		if (!this.connection) {
			Logger.warn(
				`[CarburetorConnection] Failed to send motor packet: No connection found.`
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
				view.getInt8(3)
			])
		);
	}
}

enum CarburetorInstruction {
	CONTROL_MOTOR = 0,
	QUERY_BATTERY = 100,
	QUERY_MEMORY = 101,
	QUERY_CPU = 102
}
