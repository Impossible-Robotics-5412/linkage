<script lang="ts">
	import Container from '$lib/components/Container.svelte';
	import { loggerState } from '$lib/state/loggers';
	import { LogLevel, logLevelLabel } from '$lib/process-logger';
	import { type Address, getConfig } from '$lib/config';
	import LoggerOutput from '$lib/components/overview/logger/LoggerOutput.svelte';
	import { robotCodeState } from '$lib/state/robot-code';

	interface LoggerTab {
		name: string;
		address: Address;
		closedStreamMessage?: string;
		canStartLogger?: boolean;
	}

	let loggerTabs: Record<string, LoggerTab>;
	$: {
		getConfig().then(config => {
			loggerTabs = {
				'cockpit-backend': {
					name: 'Cockpit Backend',
					address: config.cockpit_backend_logger_address
				},
				'linkage': {
					name: 'Linkage',
					address: config.linkage_lib_logger_address,
					closedStreamMessage:
						'Enable the robot to see Linkage output',
					canStartLogger: $robotCodeState.enabled
				},
				'carburetor': {
					name: 'Carburetor',
					address: config.carburetor_logger_address
				}
			};
		});
	}
</script>

{#if loggerTabs}
	<Container noPadding>
		<div class="header" slot="header">
			<h3>{loggerTabs[$loggerState.selectedTabId].name}</h3>

			<div>
				<select bind:value={$loggerState.selectedTabId}>
					{#each Object.entries(loggerTabs) as [id, tab]}
						<option value={id}>{tab.name}</option>
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
			{#each Object.entries(loggerTabs) as [id, tab]}
				<div
					class="tab"
					class:visible={$loggerState.selectedTabId === id}>
					<LoggerOutput
						closedStreamMessage={tab.closedStreamMessage}
						address={tab.address}
						canStartLogger={tab.canStartLogger} />
				</div>
			{/each}
		</div>
	</Container>
{:else}
	<pre>Loading...</pre>
{/if}

<style lang="scss">
	@use '../../../style/vars' as *;

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
