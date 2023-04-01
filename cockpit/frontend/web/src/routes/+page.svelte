<script lang="ts">
	import { ProcessLogger } from '$lib/client/process-logger';
	import Container from '$lib/components/Container.svelte';
	import EnableDisableRobotButton from '$lib/components/EnableDisableRobotButton.svelte';
	import Logger from '$lib/components/Logger.svelte';
	import Status from '$lib/components/Status.svelte';
	import { onMount } from 'svelte';

	const runtimeLogger = new ProcessLogger(
		'ws://0.0.0.0:7640',
		'RuntimeLogger'
	);
	const backendLogger = new ProcessLogger(
		'ws://0.0.0.0:7642',
		'BackendLogger'
	);
	const carburetorLogger = new ProcessLogger(
		'ws://0.0.0.0:7644',
		'CarburetorLogger'
	);
</script>

<main>
	<Container>
		<div class="window">
			<Status />
			<Logger processLogger={runtimeLogger} />

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
