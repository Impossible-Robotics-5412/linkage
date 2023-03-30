<script lang="ts">
	import type { LoggerMessage } from '../../types/logger-message';
	import Container from './Container.svelte';

	export let stream: ReadableStream | undefined;

	let messages: LoggerMessage[] = [];
	let loggerElement: HTMLElement;

	async function startReading(stream: ReadableStream) {
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

	function scrollToBottom() {
		if (loggerElement) {
			const isScrolledToBottom =
				loggerElement.scrollHeight - loggerElement.clientHeight <=
				loggerElement.scrollTop + 32;

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

	$: if (messages) scrollToBottom();
	$: if (stream) startReading(stream);
</script>

<Container noPadding>
	<div slot="header">
		<h3>Cockpit Backend Log</h3>
	</div>

	<div class="lines" bind:this={loggerElement}>
		{#each messages as message}
			<div
				class="line"
				class:out={message.stream === 'out'}
				class:err={message.stream === 'err'}>
				<span>{message.data}</span>
			</div>
		{/each}
	</div>
</Container>

<!-- <div bind:this={loggerElement} class="lines" /> -->
<style lang="scss">
	$error-border-color: $c-red;

	.lines {
		font-size: 14px;
		height: 100%;
		overflow: scroll;
	}

	.line {
		padding: 2px 1.5rem;
		width: 100%;
		box-sizing: border-box;

		border-bottom: 1px solid $c-gray-2;

		&:not(.err):has(+ .line.err) {
			border-bottom-color: $error-border-color;
		}
	}

	.line.err {
		background: scale-color($c-red, $alpha: -85%);
		border-bottom-color: $error-border-color;
	}
</style>
