import { EventEmitter } from 'node:events';
import { WebSocketServer } from 'ws';

// FIXME: These log related types could be put in a TS common package.
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

class LogEmitter extends EventEmitter {}

export class Logger {
	private static emitter = new LogEmitter();
	private static socket: WebSocketServer | undefined;

	static init() {
		const port = 7640;
		this.socket = new WebSocketServer({ port: 7640 });
		this.socket.on('listening', () => {
			this.info(`Logger started on port ${port}`);
		});

		this.socket.on('connection', client => {
			this.emitter.on('log', (log: Log) => {
				client.send(JSON.stringify(log));
			});
		});

		this.emitter.on('log', (log: Log) => {
			console.log(`[${logLevelLabel(log.level)}] ${log.msg}`);
		});
	}

	static trace(message?: any, ...optionalParams: any[]) {
		this.log(LogLevel.TRACE, message, optionalParams);
	}

	static debug(message?: any, ...optionalParams: any[]) {
		this.log(LogLevel.DEBUG, message, optionalParams);
	}

	static info(message?: any, ...optionalParams: any[]) {
		this.log(LogLevel.INFO, message, optionalParams);
	}

	static warn(message?: any, ...optionalParams: any[]) {
		this.log(LogLevel.WARN, message, optionalParams);
	}

	static error(message?: any, ...optionalParams: any[]) {
		this.log(LogLevel.ERROR, message, optionalParams);
	}

	private static log(
		level: LogLevel,
		message?: any,
		...optionalParams: any[]
	) {
		const log: Log = {
			msg: [message, ...optionalParams].join(' '),
			level: level
		};

		this.emitter.emit('log', log);
	}
}
