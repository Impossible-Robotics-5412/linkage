<script lang="ts">
	import EnableDisableRobotButton from '$lib/components/EnableDisableRobotButton.svelte';
	import Loggers from '$lib/components/logger/Loggers.svelte';
	import StatusContainer from '$lib/components/status/StatusContainer.svelte';
	import Gamepads from '$lib/components/gamepads/Gamepads.svelte';
	import Container from '$lib/components/Container.svelte';

	const tabs = [
		{ label: "Gamepads", component: Gamepads },
		{ label: "Loggers", component: Loggers },
	];

	let selectedTab = 0;

	function selectTab(index: number) {
		selectedTab = index;
	}
</script>

<main>
	<div class="main-window">
		<StatusContainer />
		<svelte:component this={tabs[selectedTab].component} />
		<EnableDisableRobotButton />
		<Container>
			{#each tabs as tab, i}
				<button on:click={() => selectTab(i)}>{tab.label}</button>
			{/each}
		</Container>
	</div>
</main>

<style lang="scss">
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
