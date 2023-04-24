<script lang="ts">
	import { disableRobotCode, initializeListeners } from '$lib/backend';
	import Sidebar from '$lib/components/Sidebar.svelte';
	import type { Tab } from '$lib/types/tab';

	let selectedTab: Tab;

	const entries = performance.getEntriesByType('navigation');
	entries.forEach((entry: PerformanceNavigationTiming) => {
		if (entry.type === 'reload') {
			console.log(`${entry.name} was reloaded. Disabling robot code.`);
			disableRobotCode();
		}
	});

	initializeListeners();
</script>

<main>
	<Sidebar bind:selectedTab />
	<div class="main-window">
		{#if selectedTab}
			<svelte:component this={selectedTab.component} />
		{:else}
			<h2>No tab selected</h2>
		{/if}
	</div>
</main>

<style lang="scss">
	main {
		width: 100vw;
		height: 100vh;
		display: flex;
	}

	.main-window {
		width: 100%;
		height: 100%;
	}
</style>
