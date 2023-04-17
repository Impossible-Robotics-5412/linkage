<script lang="ts">
	import Container from '$lib/components/Container.svelte';
	import { loggerState, tabs } from '$lib/logger';
	import { LogLevel, logLevelLabel } from '$lib/process-logger';
</script>

<Container noPadding>
	<div class="header" slot="header">
		<h3>{tabs[$loggerState.selectedTabId].name}</h3>

		<div>
			<select bind:value={$loggerState.selectedTabId}>
				{#each Object.keys(tabs) as tabId}
					<option value={tabId}>{tabs[tabId].name}</option>
				{/each}
			</select>

			<select bind:value={$loggerState.level}>
				<option value={LogLevel.ERROR}>
					{logLevelLabel(LogLevel.ERROR)}
				</option>
				<option value={LogLevel.WARN}>
					{logLevelLabel(LogLevel.WARN)}
				</option>
				<option value={LogLevel.INFO}>
					{logLevelLabel(LogLevel.INFO)}
				</option>
				<option value={LogLevel.DEBUG}>
					{logLevelLabel(LogLevel.DEBUG)}
				</option>
			</select>
		</div>
	</div>

	<div class="output">
		{#each Object.keys(tabs) as tabId}
			<div
				class="tab"
				class:visible={$loggerState.selectedTabId === tabId}>
				<svelte:component this={tabs[tabId].loggerOutputComponent} />
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
