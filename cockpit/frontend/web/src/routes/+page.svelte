<script lang="ts">
	import Container from '$lib/components/Container.svelte';
	import EnableDisableRobotButton from '$lib/components/EnableDisableRobotButton.svelte';
	import Loggers from '$lib/components/Loggers.svelte';
	import Status from '$lib/components/Status.svelte';
</script>

<main>
	<Container>
		<div class="window">
			<Status />
			<Loggers />

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
