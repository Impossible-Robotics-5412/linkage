<script lang="ts">
	import StatusItem from './StatusItem.svelte';
	import { Status } from './StatusItem.svelte';
	import Container from '../Container.svelte';
	import { robotCodeState } from '$lib/state/robot-code';
	import RobotSystemStatus from './RobotSystemStatus.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { listen } from '@tauri-apps/api/event';
	import RobotServicesStatus from './RobotServicesStatus.svelte';

	let systemInfo: SystemInfo | undefined;
	invoke('start_gauge_connection').then(() => {
		listen('received-system-info', event => {
			systemInfo = event.payload as SystemInfo;
		});
	});

	let robotCodeStatus = Status.BAD;
	$: {
		if ($robotCodeState.enabled) robotCodeStatus = Status.GOOD;
		else robotCodeStatus = Status.BAD;
	}

	$: robotCodeFoundInfo = systemInfo?.robot_code_exists
		? 'Found'
		: 'Not Found';
	$: robotCodeFoundStatus = systemInfo?.robot_code_exists
		? Status.GOOD
		: Status.BAD;
</script>

<Container>
	<div slot="header">
		<h3>Status</h3>
	</div>

	<div class="status">
		<h3>Robot Code</h3>
		<StatusItem
			info={`${robotCodeStatus ? 'Enabled' : 'Disabled'}`}
			label="State"
			status={robotCodeStatus} />
		<StatusItem
			info={robotCodeFoundInfo}
			label="Entrypoint"
			status={robotCodeFoundStatus} />

		<h3>Robot System</h3>
		<RobotSystemStatus {systemInfo} />
		<h3>Robot Services</h3>
		<RobotServicesStatus {systemInfo} />
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

		h3:not(:first-child) {
			padding-top: 1.5rem;
		}
	}
</style>
