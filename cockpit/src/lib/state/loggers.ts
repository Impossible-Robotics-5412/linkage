import { writable } from 'svelte/store';
import { LogLevel } from '$lib/process-logger';

export interface LoggerState {
	selectedTabId: string;
	level: LogLevel;
}

export const loggerState = writable<LoggerState>({
	selectedTabId: 'cockpit-backend',
	level: LogLevel.DEBUG
});
