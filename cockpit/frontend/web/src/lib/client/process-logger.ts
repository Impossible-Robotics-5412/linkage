export class ProcessLogger {
	constructor(
		public readonly address: string,
		public readonly name: string
	) {}

	public start() {
		return new Promise<ReadableStream>((resolve, reject) => {
			const processLogSocket = new WebSocket(this.address);

			processLogSocket.addEventListener('error', reject);

			processLogSocket.onopen = () => {
				const stream = new ReadableStream({
					start: controller => {
						processLogSocket.addEventListener('message', msg => {
							controller.enqueue(msg.data);
						});
					}
				});

				resolve(stream);
			};
		});
	}
}
