export interface LoggerMessage {
	stream: 'out' | 'err';
	data: string;
}
