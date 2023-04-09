<script lang="ts">
	import Container from '$lib/components/Container.svelte';
	import type { ComponentType } from 'svelte';

	import LinkageLoggerOutput from './LinkageLoggerOutput.svelte';
	import CockpitBackendLoggerOutput from './CockpitBackendLoggerOutput.svelte';

	interface Tab {
		name: string;
		loggerOutputComponent: ComponentType;
	}

	const tabs: Record<string, Tab> = {
		backend: {
			name: 'Cockpit Backend',
			loggerOutputComponent: CockpitBackendLoggerOutput
		},
		linkage: {
			name: 'Linkage',
			loggerOutputComponent: LinkageLoggerOutput
		}
	};

	let currentTabId = Object.keys(tabs)[0];
</script>

<Container noPadding>
	<div class="header" slot="header">
		<h3>{tabs[currentTabId].name}</h3>

		<select bind:value={currentTabId}>
			{#each Object.keys(tabs) as tabId}
				<option value={tabId}>{tabs[tabId].name}</option>
			{/each}
		</select>
	</div>

	<div class="output">
		{#each Object.keys(tabs) as tabId}
			<div class="tab" class:visible={currentTabId === tabId}>
				<svelte:component
					this={tabs[currentTabId].loggerOutputComponent} />
			</div>
		{/each}
	</div>
</Container>

<style lang="scss">
	@use '../../style/vars' as *;

	.header {
		display: flex;
		justify-content: space-between;

		width: 100%;
	}

	.output {
		width: 100%;
		height: 100%;
	}

	.tab {
		width: 100%;
		height: 100%;

		display: none;
		&.visible {
			display: block;
		}
	}
</style>
