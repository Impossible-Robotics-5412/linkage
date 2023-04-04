<script lang="ts">
	import { ProcessLogger } from '$lib/process-logger';
	import Container from '$lib/components/Container.svelte';
	import LoggerOutput from './LoggerOutput.svelte';

	interface Tab {
		name: string;
		processLogger: ProcessLogger;
	}

	const tabs: Record<string, Tab> = {
		backend: {
			name: 'Cockpit Backend',
			processLogger: new ProcessLogger('ws://0.0.0.0:7642')
		}
		// carburetor: {
		// 	name: 'Carburetor',
		// 	processLogger: new ProcessLogger('ws://0.0.0.0:7644')
		// }
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
				<LoggerOutput processLogger={tabs[tabId].processLogger} />
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
