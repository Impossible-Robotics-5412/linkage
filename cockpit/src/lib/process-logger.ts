export interface Log {
	msg: string;
	level: LogLevel;
	file?: string;
	line?: number;
	timestamp: number;
	timestampString: string;
}

export enum LogLevel {
	ERROR = 1,
	WARN,
	INFO,
	DEBUG
}

export function logLevelLabel(level: LogLevel) {
	switch (level) {
		case LogLevel.ERROR:
			return 'Error';
		case LogLevel.WARN:
			return 'Warning';
		case LogLevel.INFO:
			return 'Info';
		case LogLevel.DEBUG:
			return 'Debug';
	}
}

export class ProcessLogger {
	private processLogSocket: WebSocket | undefined;

	constructor(public readonly address: string) {}

	start() {
		return new Promise<ReadableStream<Log>>((resolve, reject) => {
			if (this.processLogSocket) return;

			this.processLogSocket = new WebSocket(this.address);

			this.processLogSocket.addEventListener('error', reject);

			this.processLogSocket.onopen = () => {
				const stream = new ReadableStream<Log>({
					start: controller => {
						this.processLogSocket.addEventListener(
							'message',
							msg => {
								const logs: [Log] = JSON.parse(msg.data);
								for (const log of logs) {
									log.msg = log.msg.trimEnd();
									log.timestampString = new Date(
										log.timestamp
									).toLocaleTimeString();
									controller.enqueue(log);
								}
							}
						);
					}
				});

				resolve(stream);
			};
		});
	}

	stop() {
		this.processLogSocket?.close();
		this.processLogSocket = undefined;
	}

	async restart() {
		this.stop();
		await this.start();
	}
}
