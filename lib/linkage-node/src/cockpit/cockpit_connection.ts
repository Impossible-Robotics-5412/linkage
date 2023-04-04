import { GamepadManager } from '../gamepad';
import { createServer, Server } from 'net';
import { ConfigManager } from '../config/config_manager';
import { Logger } from '../logger/logger';

enum CockpitInstruction {
	GAMEPAD_EVENT = 0x20
}

interface CockpitMessage {
	instruction: CockpitInstruction;
	data1: number;
	data2: number;
	data3: number;
	data4: number;
	data5: number;
	data6: number;
	data7: number;
}

function cockpitMessageFromBytes(bytes: number[]): CockpitMessage {
	if (bytes.length !== 8) {
		throw Error('Failed to parse [${bytes}]: Invalid number of bytes!');
	}

	return {
		instruction: bytes[0],
		data1: bytes[1],
		data2: bytes[2],
		data3: bytes[3],
		data4: bytes[4],
		data5: bytes[5],
		data6: bytes[6],
		data7: bytes[7]
	};
}

export class CockpitConnection {
	public static readonly shared = new CockpitConnection();

	private server: Server | undefined;

	public async listen(): Promise<void> {
		if (this.server) {
			this.close();
		}

		this.server = createServer(socket => {
			Logger.info(
				`[CockpitConnection] New connection: ${socket.remoteAddress}.`
			);

			socket.on('readable', () => {
				let chunk: Buffer;
				while ((chunk = socket?.read(8))) {
					this.onMessage(cockpitMessageFromBytes(Array.from(chunk)));
				}
			});

			socket.on('close', () => {
				Logger.error(
					`[CockpitConnection] Socket closed ${socket.remoteAddress}`
				);
			});

			socket.on('error', error => {
				Logger.error(
					`[CockpitConnection] Socket error on socket ${socket.remoteAddress} ${error}`
				);
			});
		});

		this.server.on('error', error => {
			Logger.info(`[CockpitConnection] Failed to connect: ${error}.`);
		});

		const port = ConfigManager.shared.config?.port;
		this.server.listen(port, () => {
			Logger.info(
				`[CockpitConnection] Started listening on port ${port}.`
			);
		});
	}

	public close(): void {
		if (!this.server) {
			return;
		}

		this.server.close();
		this.server = undefined;
		Logger.info('[CockpitConnection] Disconnected.');
	}

	private onMessage(message: CockpitMessage): void {
		if (message.instruction === CockpitInstruction.GAMEPAD_EVENT) {
			const gamepadId = message.data4;
			const eventType = message.data5;
			const control = message.data6;
			const value = message.data7;

			Logger.info(
				`[CockpitConnection] Gamepad Event: ${gamepadId}, ${eventType}, ${control}, ${value}.`
			);

			GamepadManager.shared.parseGamepadEvent(
				gamepadId,
				eventType,
				control,
				value
			);
		} else {
			Logger.info(
				`[CockpitConnection] Invalid instruction: ${message.instruction}.`
			);
		}
	}
}
