import { writable } from 'svelte/store';

export enum BackendStatus {
	DISCONNECTED = 'disconnected',
	CONNECTING = 'connecting',
	PROCESS_STARTED = 'process_started',
	STARTING_PROCESS = 'starting_process',
	STARTING_LOGGER = 'starting_logger',
	LOGGER_STARTED = 'logger_started',
	COMMUNICATION_STARTED = 'communication_started',
	STARTING_COMMUNICATION = 'starting_communication',
	CONNECTED = 'connected'
}

export interface State {
	status: BackendStatus;
}

export const backendState = writable<State>({
	status: BackendStatus.DISCONNECTED
});
