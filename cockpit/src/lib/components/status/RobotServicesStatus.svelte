<script lang="ts">
	import StatusItem from './StatusItem.svelte';
	import { Status } from './StatusItem.svelte';

	export let systemInfo: SystemInfo | undefined;

	$: carburetorStatus = systemInfo?.service_info.carburetor_status
		? Status.GOOD
		: Status.BAD;
	$: gaugeStatus = systemInfo?.service_info.gauge_status
		? Status.GOOD
		: Status.BAD;
	$: linkageSocketStatus = systemInfo?.service_info.linkage_socket_status
		? Status.GOOD
		: Status.BAD;

	$: carburetorInfo = systemInfo?.service_info.carburetor_status
		? 'Active'
		: 'Inactive';
	$: gaugeInfo = systemInfo?.service_info.gauge_status
		? 'Active'
		: 'Inactive';
	$: linkageSocketInfo = systemInfo?.service_info.linkage_socket_status
		? 'Active'
		: 'Inactive';
</script>

{#if systemInfo?.service_info}
	<StatusItem
		info={linkageSocketInfo}
		status={linkageSocketStatus}
		label="Linkage Socket" />
	<StatusItem
		info={carburetorInfo}
		status={carburetorStatus}
		label="Carburetor" />
	<StatusItem info={gaugeInfo} status={gaugeStatus} label="Gauge" />
{/if}
