export interface Log {
	msg: string;
	level: LogLevel;
	file?: string;
	line?: number;
}

export enum LogLevel {
	/**
	 * The "error" level.
	 * Designates very serious errors.
	 */
	ERROR = 1,
	/**
	 * The "warn" level.
	 * Designates hazardous situations.
	 */
	WARN,
	/**
	 * The "info" level.
	 * Designates useful information.
	 */
	INFO,
	/**
	 * The "debug" level.
	 * Designates lower priority information.
	 */
	DEBUG,
	/**
	 * The "trace" level.
	 * Designates very low priority, often extremely verbose, information.
	 */
	TRACE
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
		case LogLevel.TRACE:
			return 'Trace';
	}
}

export class ProcessLogger {
	constructor(
		public readonly address: string,
		public readonly name: string
	) {}

	public start() {
		return new Promise<ReadableStream<Log>>((resolve, reject) => {
			const processLogSocket = new WebSocket(this.address);

			processLogSocket.addEventListener('error', reject);

			processLogSocket.onopen = () => {
				const stream = new ReadableStream<Log>({
					start: controller => {
						processLogSocket.addEventListener('message', msg => {
							controller.enqueue(JSON.parse(msg.data));
						});
					}
				});

				resolve(stream);
			};
		});
	}
}
