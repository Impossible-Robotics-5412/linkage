<script lang="ts">
	import { disableRobotCode } from '$lib/backend';
	import Sidebar from '$lib/components/Sidebar.svelte';
	import type { LoggerTab } from '$lib/logger';

	let selectedTab: LoggerTab;

	const entries = performance.getEntriesByType('navigation');
	entries.forEach(entry => {
		if (entry.type === 'reload') {
			console.log(`${entry.name} was reloaded. Disabling robot code.`);
			disableRobotCode();
		}
	});
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
