<script lang="ts">
	import {
		LogLevel,
		logLevelLabel,
		type Log,
		ProcessLogger
	} from '$lib/process-logger';
	import { tick } from 'svelte';
	import { loggerState } from '$lib/state/loggers';
	import type { Address } from '$lib/config';

	export let address: Address | undefined;
	export let closedStreamMessage = 'Logger stream is closed';
	export let canStartLogger = true;
	export let maxScrollback = 500;

	let logs: Log[] = [];
	let loggerElement: HTMLElement;
	let stream: ReadableStream<Log> | undefined;

	$: if (address && canStartLogger) {
		const processLogger = new ProcessLogger(
			`ws://${address.host}:${address.port}`
		);

		processLogger.start().then(logStream => {
			stream = logStream;
		});
	}

	async function startReadingStream() {
		clearLogger();
		const reader = stream.getReader();

		while (stream) {
			const { done, value } = await reader.read();
			if (done) {
				console.log('End of stream');
				break;
			}

			logs.push(value);
			if (logs.length > maxScrollback) logs.shift();
			logs = logs;
		}
		await reader.cancel();
		await stream?.cancel();
	}

	function clearLogger() {
		logs = [];
	}

	async function scrollToBottom() {
		if (!loggerElement) return;

		// BUG:	When we press the Enable/Disable button this won't update
		// 		as the scrollheight is 0 because we just made the other log invisible.
		const isScrolledToBottom =
			loggerElement.scrollHeight - loggerElement.clientHeight <=
			loggerElement.scrollTop + 32;

		if (!isScrolledToBottom) return;

		await tick();
		loggerElement.scrollTop =
			loggerElement.scrollHeight - loggerElement.clientHeight;
	}

	$: if (logs) scrollToBottom();
	$: if (stream) startReadingStream();
</script>

<div class="logger-output" bind:this={loggerElement}>
	{#if stream}
		{#each logs as log}
			{#if log.level <= $loggerState.level}
				<div
					class="line"
					class:level-error={log.level === LogLevel.ERROR}
					class:level-warn={log.level === LogLevel.WARN}
					class:level-info={log.level === LogLevel.INFO}
					class:level-debug={log.level === LogLevel.DEBUG}>
					<span title={`${log.file}:${log.line}`}>
						[{log.timestampString}
						{logLevelLabel(log.level)}] {log.msg}
					</span>
				</div>
			{/if}
		{/each}
	{:else}
		<div class="logger-closed-message">
			<h2>{closedStreamMessage}</h2>
		</div>
	{/if}
</div>

<style lang="scss">
	@use '../../../style/vars' as *;

	.logger-closed-message {
		width: 100%;
		height: 100%;

		display: flex;
		justify-content: center;
		align-items: center;
	}

	.logger-output {
		height: 100%;

		font-size: 14px;

		overflow: scroll;
		display: flex;
		flex-direction: column;
	}

	.line {
		padding: 2px 1.5rem;
		box-sizing: border-box;

		& span {
			word-wrap: break-word;
			white-space: pre-line;
		}
	}

	@mixin log-level(
		$selector,
		$text-color,
		$color,
		$border-color,
		$priority: false
	) {
		.line#{$selector} {
			background: $color;
			border-top: 1px solid $border-color;

			color: $text-color;

			@if $priority {
				border-bottom: 1px solid $border-color;

				& + .line {
					border-top: none;
				}
			}

			@if not $priority {
				&:first-child {
					border-top: none;
				}
			}

			&:last-child {
				border-bottom: 1px solid $border-color;
			}
		}
	}

	@include log-level(
		'.level-error',
		$c-primary,
		scale-color($c-red, $alpha: -85%),
		$c-red,
		true
	);
	@include log-level(
		'.level-warn',
		$c-primary,
		scale-color($c-orange, $alpha: -85%),
		$c-orange,
		true
	);
	@include log-level('.level-info', $c-primary, $c-background, $c-gray-2);
	@include log-level('.level-debug', $c-secondary, $c-background, $c-gray-2);
</style>
