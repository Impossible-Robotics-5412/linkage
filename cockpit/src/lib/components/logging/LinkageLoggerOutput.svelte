<script lang="ts">
	import { ProcessLogger } from '$lib/process-logger';
	import { robotCodeState } from '$lib/state';
	import LoggerOutput from './LoggerOutput.svelte';

	const processLogger = new ProcessLogger('ws://raspberrypi.local:7640');

	let stream: ReadableStream;
	$: if ($robotCodeState.enabled) {
		processLogger.start().then(logStream => {
			stream = logStream;
		});
	} else {
		processLogger.stop();
		stream = null;
	}
</script>

<LoggerOutput
	closedStreamMessage="Enable the robot to see Linkage output"
	bind:stream />
