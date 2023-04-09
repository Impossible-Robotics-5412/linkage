<script lang="ts">
	import { getConfig } from '$lib/config';
	import { ProcessLogger } from '$lib/process-logger';
	import { robotCodeState } from '$lib/state';
	import LoggerOutput from './LoggerOutput.svelte';

	let processLogger: ProcessLogger;
	getConfig().then(config => {
		processLogger = new ProcessLogger(
			`ws://${config.linkage_lib_logger_address.host}:${config.linkage_lib_logger_address.port}`
		);
	});

	let stream: ReadableStream;
	$: if (processLogger && $robotCodeState.enabled) {
		processLogger.start().then(logStream => {
			stream = logStream;
		});
	} else {
		processLogger?.stop();
		stream = null;
	}
</script>

<LoggerOutput
	closedStreamMessage="Enable the robot to see Linkage output"
	bind:stream />
