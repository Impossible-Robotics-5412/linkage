<script lang="ts">
	import { ProcessLogger } from '$lib/process-logger';
	import { robotCodeState } from '$lib/state';
	import LoggerOutput from './LoggerOutput.svelte';
	import { invoke } from '@tauri-apps/api/tauri';

	interface Address {
		host: string;
		port: number;
	}

	interface Config {
		linkage_socket_address: Address;
		linkage_lib_address: Address;
		linkage_lib_logger_address: Address;
	}

	let processLogger: ProcessLogger;
	invoke('config').then((config_json: string) => {
		const config = JSON.parse(config_json);
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
