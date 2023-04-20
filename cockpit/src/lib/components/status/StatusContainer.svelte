<script lang="ts">
	import StatusItem from './StatusItem.svelte';
	import Container from '../Container.svelte';
	import RobotSystemStatus from './RobotSystemStatus.svelte';
	import RobotServicesStatus from './RobotServicesStatus.svelte';
	import { robotCode, systemInfo } from '$lib/backend';
	import { Status } from '$lib/types/status';
	import List from '$lib/components/ui/List.svelte';

	$: robotCodeStatus = $robotCode.enabled ? Status.GOOD : Status.BAD;
	$: robotConnectionStatus = $systemInfo ? Status.GOOD : Status.BAD;

	$: robotCodeFoundInfo = $systemInfo?.robot_code_exists
		? 'Found'
		: 'Not Found';
	$: robotCodeFoundStatus = $systemInfo?.robot_code_exists
		? Status.GOOD
		: Status.BAD;
</script>

<Container scrollable>
	<div slot="header">
		<h3>Status</h3>
	</div>

	<List>
		<h3>Robot Connection</h3>
		<StatusItem
			info={`${robotConnectionStatus ? 'Connected' : 'Not Connected'}`}
			label="Connection"
			status={robotConnectionStatus} />

		<h3>Robot Code</h3>
		<StatusItem
			info={`${robotCodeStatus ? 'Enabled' : 'Disabled'}`}
			label="State"
			status={robotCodeStatus} />
		<StatusItem
			info={robotCodeFoundInfo}
			label="Entrypoint"
			status={robotCodeFoundStatus} />

		{#if $systemInfo?.cpu && $systemInfo?.memory}
			<h3>Robot System</h3>
			<RobotSystemStatus
				memory={$systemInfo.memory}
				cpu={$systemInfo.cpu} />
		{/if}

		{#if $systemInfo?.service_info}
			<h3>Robot Services</h3>
			<RobotServicesStatus serviceInfo={$systemInfo?.service_info} />
		{/if}
	</List>
</Container>

<style lang="scss">
	@use '../../style/vars' as *;
</style>
