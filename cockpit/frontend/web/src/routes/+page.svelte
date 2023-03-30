<script lang="ts">
	import { Backend } from '$lib/client/backend/backend';
	import { backendState, BackendStatus } from '$lib/client/backend/state';
	import Container from '$lib/components/Container.svelte';
	import EnableDisableRobotButton from '$lib/components/EnableDisableRobotButton.svelte';
	import Logger from '$lib/components/Logger.svelte';
	import Status from '$lib/components/Status.svelte';

	let loggerStream: ReadableStream | undefined;
	$: if ($backendState.status === BackendStatus.LOGGER_STARTED)
		loggerStream = Backend.shared.loggerStream;
</script>

<main>
	<Container>
		<div class="window">
			<Status />
			<Logger stream={loggerStream} />

			<EnableDisableRobotButton />
		</div>
	</Container>
</main>

<style lang="scss">
	main {
		display: flex;
		justify-content: center;
		align-items: center;

		width: 100vw;
		height: 100vh;
	}

	.window {
		display: grid;
		grid-template-columns: 25% auto;
		grid-template-rows: auto min-content;
		gap: 1.5rem;

		width: 840px;
		height: 320px;
	}
</style>
