<script lang="ts">
	import { ProcessLogger } from '$lib/process-logger';
	import LoggerOutput from './LoggerOutput.svelte';

	const processLogger = new ProcessLogger('ws://0.0.0.0:7642');

	let stream: ReadableStream;
	processLogger.start().then(logStream => {
		stream = logStream;
	});
</script>

<LoggerOutput bind:stream />
