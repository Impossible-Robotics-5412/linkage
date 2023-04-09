<script lang="ts">
	import StatusItem from './StatusItem.svelte';
	import { Status } from './StatusItem.svelte';
	import Container from '../Container.svelte';
	import { robotCodeState } from '$lib/state/robot-code';
	import { listen } from '@tauri-apps/api/event';
	import { invoke } from '@tauri-apps/api/tauri';

	let robotCodeStatus = Status.BAD;
	$: {
		if ($robotCodeState.enabled) robotCodeStatus = Status.GOOD;
		else robotCodeStatus = Status.BAD;
	}

	interface SystemInfo {
		cpu?: {
			user: number;
			system: number;
			idle: number;
			temp?: number;
		};
		memory?: {
			swap?: {
				used: number;
				total: number;
			};
			mem?: {
				used: number;
				total: number;
			};
		};
		uptime?: number;
	}

	let systemInfo: SystemInfo | undefined;
	invoke('start_gauge_connection').then(() => {
		listen('received-system-info', event => {
			systemInfo = event.payload;
		});
	});
</script>

<Container>
	<div slot="header">
		<h3>Status</h3>
	</div>

	<div class="status">
		<StatusItem
			info={`${robotCodeStatus ? 'Enabled' : 'Disabled'}`}
			label="Robot Status"
			status={robotCodeStatus} />

		{#if systemInfo?.cpu}
			<StatusItem
				info={`${systemInfo.cpu.idle.toFixed(0)}%`}
				label="Robot CPU Idle" />
			<StatusItem
				info={`${systemInfo.cpu.system.toFixed(0)}%`}
				label="Robot CPU System" />
			<StatusItem
				info={`${systemInfo.cpu.user.toFixed(0)}%`}
				label="Robot CPU User" />
			<StatusItem
				info={`${systemInfo.cpu.temp.toFixed(0)}℃`}
				label="Robot CPU Temperature" />
		{/if}

		{#if systemInfo?.memory?.swap}
			<StatusItem
				info={`${(
					(systemInfo.memory.swap.used /
						systemInfo.memory.swap.total) *
					100
				).toFixed(0)}%`}
				label="Robot Swap Memory" />
		{/if}

		{#if systemInfo?.memory?.swap}
			<StatusItem
				info={`${(
					(systemInfo.memory.mem.used / systemInfo.memory.mem.total) *
					100
				).toFixed(0)}%`}
				label="Robot Memory" />
		{/if}
	</div>
</Container>

<style lang="scss">
	@use '../../style/vars' as *;

	.status {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;

		& :global(> *) {
			border-bottom: 1px solid $c-gray-1;
			padding-bottom: 0.5rem;

			&:last-child {
				border-bottom: none;
				padding-bottom: 0;
			}
		}
	}
</style>
