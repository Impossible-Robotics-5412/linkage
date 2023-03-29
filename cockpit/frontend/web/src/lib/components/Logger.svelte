<script lang="ts">
	import type { LoggerMessage } from '../../types/logger-message';

	export let stream: ReadableStream;

	let messages: LoggerMessage[] = [];
	let loggerElement: HTMLElement;

	async function startReading() {
		const reader = stream.getReader();
		while (true) {
			const { done, value } = await reader.read();
			if (done) {
				console.log('End of stream');
				break;
			}

			messages = [...messages, value];
		}
	}

	$: if (messages) {
		scrollToBottom();
	}

	function scrollToBottom() {
		if (loggerElement) {
			const isScrolledToBottom =
				loggerElement.scrollHeight - loggerElement.clientHeight <=
				loggerElement.scrollTop + 10;
			if (!isScrolledToBottom) return;

			// FIXME: This setTimeout is needed because otherwise it
			//        will scroll to the second to last element.
			//        Not sure why but it is't all that clean...
			setTimeout(() => {
				loggerElement.scrollTop =
					loggerElement.scrollHeight - loggerElement.clientHeight;
			});
		}
	}

	startReading();
</script>

<div bind:this={loggerElement} class="logger">
	{#each messages as message}
		<pre>[{message.stream}]: {message.data}</pre>
	{/each}
</div>

<style>
	.logger {
		border: 1px solid black;
		overflow: scroll;
		height: 600px;
		width: 600px;
	}
</style>
