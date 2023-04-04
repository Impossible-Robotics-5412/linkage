import { robotCodeState } from '$lib/robot-code/state';
import {
	BackendToFrontendMessage,
	FrontendToBackendMessage,
	type BackendMessage
} from './message';
import { BackendStatus, setBackendStatus } from './state';

export class BackendCommunication {
	static shared = new BackendCommunication();

	private commSocket: WebSocket | undefined;

	async connect() {
		this.disconnect();
		setBackendStatus(BackendStatus.CONNECTING);

		await this.startBackendCommunication();

		setBackendStatus(BackendStatus.CONNECTED);
	}

	disconnect() {
		this.commSocket?.close();
		setBackendStatus(BackendStatus.DISCONNECTED);
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

	private async startBackendCommunication() {
		return new Promise<void>(resolve => {
			setBackendStatus(BackendStatus.COMMUNICATION_STARTING);
			this.commSocket = new WebSocket(`ws://0.0.0.0:3012`);

			this.commSocket.onopen = () => {
				setBackendStatus(BackendStatus.COMMUNICATION_STARTED);

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
}
