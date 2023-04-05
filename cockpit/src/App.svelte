<script lang="ts">
	import EnableDisableRobotButton from '$lib/components/EnableDisableRobotButton.svelte';
	import Loggers from '$lib/components/logging/Loggers.svelte';
	import Status from '$lib/components/Status.svelte';
</script>

<main>
	<div class="main-window">
		<Status />
		<Loggers />

		<EnableDisableRobotButton />
	</div>
</main>

<style>
	main {
		width: 100vw;
		height: 100vh;
	}

	.main-window {
		-webkit-box-sizing: border-box;
		-moz-box-sizing: border-box;
		box-sizing: border-box;
		padding: 1.5rem;

		width: 100%;
		height: 100%;

		display: grid;
		grid-template-columns: 16rem auto;
		grid-template-rows: auto min-content;
		gap: 1.5rem;
	}
</style>
