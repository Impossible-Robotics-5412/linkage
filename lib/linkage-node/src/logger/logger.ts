import { connect, createServer } from 'net';
import { EventEmitter } from 'node:events';

class LogEmitter extends EventEmitter {}

export class Logger {
	private static emitter = new LogEmitter();

	static init() {
		const server = createServer(socket => {
			this.info(`New logger connection: ${socket.remoteAddress}.`);
			this.emitter.on('log', message => {
				socket.write(
					JSON.stringify({
						msg: message,
						level: 3,
						file: null,
						line: null
					})
				);
			});
		});

		this.emitter.on('log', message => {
			console.log(message);
		});

		const port = 7640;
		server.listen(port, () => {
			this.info(`Logger started on port ${port}`);
		});
	}

	static trace(message: any) {
		this.emitter.emit('log', '[TRACE] ' + message);
	}
	static debug(message: any) {
		this.emitter.emit('log', '[DEBUG] ' + message);
	}
	static info(message: any) {
		this.emitter.emit('log', '[INFO] ' + message);
	}
	static warn(message: any) {
		this.emitter.emit('log', '[WARN] ' + message);
	}
	static error(message: any) {
		this.emitter.emit('log', '[ERROR] ' + message);
	}
}
