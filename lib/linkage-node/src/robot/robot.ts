import { Subsystem } from '../subsystem';
import { CockpitConnection } from '../cockpit/cockpit_connection';
import { CarburetorConnection } from '../carburetor/carburetor_connection';
import { Logger } from '../logger/logger';

/**
 * The robot class is the main entry point for the entire robot program. All logic starts here.
 */
export abstract class Robot {
	private _isRunning = false;
	private subsystems: Subsystem[] = [];
	private internalTickTimer: NodeJS.Timer | undefined;

	/**
	 * The tick function is called once every 20ms.
	 * @protected
	 */
	protected tick(): void {}

	/**
	 * The shutdown function is called when the program exits.
	 * @protected
	 */
	protected shutdown(): void {}

	/**
	 * Starts the robot code. This should only be called once per program.
	 */
	public async run(): Promise<void> {
		Logger.init();
		Logger.info('Starting.');

		await Promise.all([
			await CockpitConnection.shared.listen(),
			await CarburetorConnection.shared.connect()
		]);

		// FIXME: This shouldn't use setInterval but this is okay for initial testing period.
		this.internalTickTimer = setInterval(() => {
			this.internalTick();
		}, 20);

		process.on('SIGINT', this.internalShutdown);
		process.on('SIGTERM', this.internalShutdown);

		// NOTE: This makes sure we close the connection when te systemd socket
		//       is closed. It will close when the stdin stream is closed.
		process.stdin.on('readable', () => process.stdin.read());
		process.stdin.on('end', () => this.internalShutdown());

		this._isRunning = true;
	}

	/**
	 * Registers a new subsystem. This makes sure the robot code knows about the subsystem and will update it when needed.
	 * @param subsystem The subsystem to register.
	 */
	public registerSubsystem(subsystem: Subsystem): void {
		this.subsystems.push(subsystem);
	}

	private internalTick(): void {
		for (const subsystem of this.subsystems) {
			subsystem.tick();
		}

		this.tick();
	}

	private internalShutdown(): void {
		Logger.info('Shutting Down.');

		clearInterval(this.internalTickTimer);

		for (const subsystem of this.subsystems) {
			subsystem.shutdown();
		}

		this.shutdown();

		CockpitConnection.shared.close();
		CarburetorConnection.shared.close();

		process.exit(0);
	}

	/**
	 * @returns Whether or not the robot code is running.
	 */
	public get isRunning(): boolean {
		return this._isRunning;
	}
}
