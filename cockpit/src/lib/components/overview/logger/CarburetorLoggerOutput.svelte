<script lang="ts">
	import { ProcessLogger } from '$lib/process-logger';
	import LoggerOutput from './LoggerOutput.svelte';
	import { getConfig } from '$lib/config';

	let processLogger: ProcessLogger;
	getConfig().then(config => {
		processLogger = new ProcessLogger(
			`ws://${config.carburetor_logger_address.host}:${config.carburetor_logger_address.port}`
		);
	});

	let stream: ReadableStream;
	$: if (processLogger) {
		processLogger.start().then(logStream => {
			stream = logStream;
		});
	} else {
		processLogger?.stop();
		stream = null;
	}
</script>

<LoggerOutput bind:stream />
