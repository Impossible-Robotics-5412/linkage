import { state } from "$lib/state";

export class BackendConnection {
  private websocket: WebSocket | undefined;

  constructor() {
    this.connect();
  }

  connect() {
    console.log("Connecting with backend.");

    this.websocket = new WebSocket("ws://127.0.0.1:3012");

    this.websocket.onmessage = (message: MessageEvent<Blob>) => {
      message.data.arrayBuffer().then((buffer) => {
        this.onMessage(Array.from(new Uint8Array(buffer)) as BackendMessage);
      });
    };

    this.websocket.onopen = () => {
      console.log("Opened connection with backend.");
    };

    this.websocket.onerror = () => {
      console.log("An error occured on the backend connection.");
    };

    this.websocket.onclose = () => {
      console.log("Closed connection with backend.");
    };
  }

  enableLinkage() {
    this.sendMessage(BackendRxMessage.ENABLE_LINKAGE);
  }

  disableLinkage() {
    this.sendMessage(BackendRxMessage.DISABLE_LINKAGE);
  }

  private sendMessage(message: BackendMessage) {
    if (
      !this.websocket ||
      this.websocket.readyState === this.websocket.CLOSED
    ) {
      this.connect();
      return;
    }

    this.websocket.send(new Uint8Array(message));
  }

  private onMessage(message: BackendMessage) {
    console.log(message);

    if (message[0] === BackendTxMessage.ENABLED[0]) {
      state.update(($state) => {
        $state.enabled = true;
        return $state;
      });
    } else if (message[0] === BackendTxMessage.DISABLED[0]) {
      state.update(($state) => {
        $state.enabled = false;
        return $state;
      });
    }
  }
}

export type BackendMessage = [
  number,
  number,
  number,
  number,
  number,
  number,
  number,
  number,
];

const BackendRxMessage: Record<string, BackendMessage> = {
  ENABLE_LINKAGE: [0x00, 0, 0, 0, 0, 0, 0, 0],
  DISABLE_LINKAGE: [0x01, 0, 0, 0, 0, 0, 0, 0],
};

const BackendTxMessage: Record<string, BackendMessage> = {
  ENABLED: [0x00, 0, 0, 0, 0, 0, 0, 0],
  DISABLED: [0x01, 0, 0, 0, 0, 0, 0, 0],
};
