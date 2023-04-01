import { robotCodeState } from '$lib/client/robot-code/state';
import type { FrontendResponse } from '../../../types/frontend-response';
import {
	BackendToFrontendMessage,
	FrontendToBackendMessage,
	type BackendMessage
} from './message';
import { backendState, BackendStatus } from './state';

export class Backend {
	static shared = new Backend();

	private commSocket: WebSocket | undefined;

	constructor() {
		this.connect();
	}

	async connect() {
		this.disconnect();
		this.setStatus(BackendStatus.CONNECTING);

		await this.startBackend();
		await this.startBackendCommunication();

		this.setStatus(BackendStatus.CONNECTED);
	}

	disconnect() {
		this.commSocket?.close();
		this.setStatus(BackendStatus.DISCONNECTED);
	}

	enableLinkage() {
		this.sendMessage(FrontendToBackendMessage.ENABLE_LINKAGE);
	}

	disableLinkage() {
		this.sendMessage(FrontendToBackendMessage.DISABLE_LINKAGE);
	}

	private sendMessage(message: BackendMessage) {
		if (
			!this.commSocket ||
			this.commSocket.readyState === WebSocket.CLOSED
		) {
			console.log(
				'Communication socket needs to be open to send messages!'
			);
			return;
		}

		this.commSocket.send(new Uint8Array(message));
	}

	private async startBackend() {
		this.setStatus(BackendStatus.PROCESS_STARTING);
		const response = await fetch('/backend/start', { method: 'post' });
		const data: FrontendResponse = await response.json();
		if (data.success) {
			this.setStatus(BackendStatus.PROCESS_STARTED);
		} else {
			console.error(data.error);
		}

		return data;
	}

	private async startBackendCommunication() {
		return new Promise<void>(resolve => {
			this.setStatus(BackendStatus.COMMUNICATION_STARTING);
			this.commSocket = new WebSocket(`ws://0.0.0.0:3012`);

			this.commSocket.onopen = () => {
				this.setStatus(BackendStatus.COMMUNICATION_STARTED);
				resolve();
			};

			this.commSocket.onmessage = (message: MessageEvent<Blob>) => {
				message.data.arrayBuffer().then(buffer => {
					const backendMessage = Array.from(
						new Uint8Array(buffer)
					) as BackendMessage;
					this.onMessage(backendMessage);
				});
			};
		});
	}

	private onMessage(message: BackendMessage) {
		if (message[0] === BackendToFrontendMessage.ENABLED[0]) {
			robotCodeState.update($state => {
				$state.enabled = true;
				return $state;
			});
		} else if (message[0] === BackendToFrontendMessage.DISABLED[0]) {
			robotCodeState.update($state => {
				$state.enabled = false;
				return $state;
			});
		}
	}

	private setStatus(status: BackendStatus) {
		backendState.update($backendState => {
			$backendState.status = status;
			return $backendState;
		});
	}
}
