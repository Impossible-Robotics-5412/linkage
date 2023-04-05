<script lang="ts">
	import { LogLevel, logLevelLabel, type Log } from '$lib/process-logger';

	export let stream: ReadableStream;
	export let maxScrollback = 1000;
	export let closedStreamMessage = 'Logger stream is closed';

	let logs: Log[] = [];
	let loggerElement: HTMLElement;

	async function startReadingStream() {
		const reader = stream.getReader();

		while (stream) {
			const { done, value } = await reader.read();
			if (done) {
				console.log('End of stream');
				break;
			}

			logs = [value, ...logs];
			if (logs.length > maxScrollback) {
				logs.pop();
				logs = logs;
			}
		}
		reader.cancel();
		stream?.cancel();
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

	$: if (logs) scrollToBottom();
	$: if (stream) startReadingStream();
</script>

<div class="logger-output" bind:this={loggerElement}>
	{#if stream}
		{#each logs as log}
			<div
				class="line"
				class:level-error={log.level === LogLevel.ERROR}
				class:level-warn={log.level === LogLevel.WARN}
				class:level-info={log.level === LogLevel.INFO}
				class:level-debug={log.level === LogLevel.DEBUG}
				class:level-trace={log.level === LogLevel.TRACE}>
				<pre>[{logLevelLabel(log.level)}] {log.msg}</pre>
			</div>
		{/each}
	{:else}
		<div class="logger-closed-message">
			<h2>{closedStreamMessage}</h2>
		</div>
	{/if}
</div>

<style lang="scss">
	@use '../../style/vars' as *;

	.logger-closed-message {
		width: 100%;
		height: 100%;

		display: flex;
		justify-content: center;
		align-items: center;
	}

	.logger-output {
		font-size: 14px;
		height: 100%;
		overflow: scroll;
		display: flex;
		flex-direction: column;
		margin-top: -1px;
	}

	.line {
		padding: 2px 1.5rem;
		width: 100%;
		box-sizing: border-box;

		border-top: 1px solid $c-gray-2;
	}

	@mixin log-level($selector, $text-color, $color, $border-color: $c-gray-2) {
		.line#{$selector} {
			background: $color;
			border-top-color: $border-color;

			color: $text-color;

			&:not(#{$selector}):has(+ .line#{$selector}) {
				border-top-color: $border-color;
			}
		}
	}

	@include log-level(
		'.level-error',
		$c-primary,
		scale-color($c-red, $alpha: -85%),
		$c-red
	);
	@include log-level(
		'.level-warn',
		$c-primary,
		scale-color($c-orange, $alpha: -85%),
		$c-orange
	);
	@include log-level('.level-info', $c-primary, $c-background);
	@include log-level('.level-debug', $c-secondary, $c-background);
	@include log-level('.level-trace', $c-secondary, $c-background);
</style>
