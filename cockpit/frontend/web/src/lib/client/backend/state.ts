import { writable } from 'svelte/store';

export enum BackendStatus {
	DISCONNECTED,
	CONNECTING,
	PROCESS_STARTING,
	PROCESS_STARTED,
	COMMUNICATION_STARTING,
	COMMUNICATION_STARTED,
	CONNECTED
}

export interface State {
	status: BackendStatus;
}

export const backendState = writable<State>({
	status: BackendStatus.DISCONNECTED
});

export function getBackendStatusLabel(status: BackendStatus) {
	switch (status) {
		case BackendStatus.DISCONNECTED:
			return 'Disconnected';
		case BackendStatus.CONNECTING:
			return 'Connecting';
		case BackendStatus.PROCESS_STARTING:
			return 'Process starting';
		case BackendStatus.PROCESS_STARTED:
			return 'Process started';
		case BackendStatus.COMMUNICATION_STARTING:
			return 'Communication starting';
		case BackendStatus.COMMUNICATION_STARTED:
			return 'Communication started';
		case BackendStatus.CONNECTED:
			return 'Connected';
	}
}
