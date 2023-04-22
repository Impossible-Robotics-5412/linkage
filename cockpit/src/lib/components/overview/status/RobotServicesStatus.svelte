<script lang="ts">
	import type { SystemServiceInfo } from '$lib/types/system-info';
	import StatusItem from './StatusItem.svelte';
	import { Status } from '$lib/types/status';

	export let serviceInfo: SystemServiceInfo;

	$: carburetorStatus = serviceInfo?.carburetor_status
		? Status.GOOD
		: Status.BAD;
	$: gaugeStatus = serviceInfo?.gauge_status ? Status.GOOD : Status.BAD;
	$: linkageSocketStatus = serviceInfo?.linkage_socket_status
		? Status.GOOD
		: Status.BAD;

	$: carburetorInfo = serviceInfo?.carburetor_status ? 'Active' : 'Inactive';
	$: gaugeInfo = serviceInfo?.gauge_status ? 'Active' : 'Inactive';
	$: linkageSocketInfo = serviceInfo?.linkage_socket_status
		? 'Active'
		: 'Inactive';
</script>

<StatusItem
	info={linkageSocketInfo}
	status={linkageSocketStatus}
	label="Linkage Socket" />
<StatusItem
	info={carburetorInfo}
	status={carburetorStatus}
	label="Carburetor" />
<StatusItem info={gaugeInfo} status={gaugeStatus} label="Gauge" />
