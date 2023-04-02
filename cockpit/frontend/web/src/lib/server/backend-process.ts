import { ChildProcess, spawn } from 'child_process';

export class BackendProcess {
	private backendProcess?: ChildProcess;

	constructor() {
		process.on('exit', this.stopBackendProcess);
	}

	async start() {
		return new Promise<void>((resolve, reject) => {
			if (this.backendProcess) {
				resolve();
				return;
			}

			// FIXME: This should not be a relative path.
			this.backendProcess = spawn(
				'../../../target/debug/cockpit-backend'
			);

			this.backendProcess.on('spawn', () => {
				// FIXME: We should wait until the server has been started. this shouldn't be done like this.
				setTimeout(() => {
					resolve();
				}, 1000);
			});
			this.backendProcess.on('error', reject);
			this.backendProcess.on('exit', this.stopBackendProcess);
			this.backendProcess.on('close', this.stopBackendProcess);
		});
	}

	private stopBackendProcess() {
		this.backendProcess?.kill();
		this.backendProcess = undefined;
	}
}
