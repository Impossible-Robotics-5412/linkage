import { ChildProcess, spawn } from 'child_process';
import { WebSocketServer } from 'ws';
import type { LoggerMessage } from '../../types/logger-message';

export class BackendProcess {
	static shared = new BackendProcess();

	private server?: WebSocketServer;
	private backendProcess?: ChildProcess;

	async start() {
		if (!this.backendProcess) {
			await this.startBackendProcess();
		}

		if (!this.server) {
			await this.startLoggerServer();
		} else {
			console.log(
				'[BackendProcess] Already running... Closing all old connections'
			);
			this.server?.clients.forEach(client => client.close());
		}

		process.on('exit', this.stopBackendProcess);
	}

	private async startLoggerServer() {
		return new Promise<void>((resolve, reject) => {
			this.server = new WebSocketServer({
				host: '0.0.0.0',
				port: 4276
			});

			this.server.on('error', error => {
				console.log('[BackendLoggerServer] ERROR:', error);
				reject(error);
			});

			this.server.on('listening', () => {
				resolve();
			});

			this.server.on('connection', socket => {
				console.log(`[BackendLoggerServer] New socket connection`);

				socket.on('close', () => {
					console.log(
						`[BackendLoggerServer] Socket connection closed`
					);
				});

				this.backendProcess?.stdout?.on('data', data => {
					socket.send(
						JSON.stringify({
							stream: 'out',
							data: data.toString()
						} satisfies LoggerMessage)
					);
				});

				this.backendProcess?.stderr?.on('data', data => {
					socket.send(
						JSON.stringify({
							stream: 'err',
							data: data.toString()
						} satisfies LoggerMessage)
					);
				});

				this.backendProcess?.on('close', code => {
					// FIXME: Tell frontend when the backend process has been stopped.
					// socket.send(JSON.stringify({ stream: null, data: code }));
					this.server?.close();
				});

				this.backendProcess?.on('exit', code => {
					// FIXME: Tell frontend when the backend process has been stopped.
					// socket.send(JSON.stringify({ stream: null, data: code }));
					this.server?.close();
				});
			});
		});
	}

	private async startBackendProcess() {
		return new Promise<void>((resolve, reject) => {
			// FIXME: This should not be a relative path.
			this.backendProcess = spawn(
				'../../../target/debug/cockpit-backend'
			);

			this.backendProcess.on('spawn', () => resolve());

			this.backendProcess?.on('error', error => {
				reject(error);
			});
		});
	}

	private stopBackendProcess() {
		this.backendProcess?.kill();
		this.backendProcess = undefined;
	}
}
