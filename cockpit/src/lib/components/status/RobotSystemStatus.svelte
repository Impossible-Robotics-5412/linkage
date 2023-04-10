<script lang="ts">
	import StatusItem, { Status } from './StatusItem.svelte';

	export let systemInfo: SystemInfo | undefined;

	$: cpuSystemLoad = `${systemInfo?.cpu?.system.toFixed(0)}%`;
	$: cpuUserLoad = `${systemInfo?.cpu?.user.toFixed(0)}%`;
	$: cpuTemperature = `${systemInfo?.cpu?.temp.toFixed(0)}℃`;

	$: memoryFraction =
		systemInfo?.memory?.mem?.used / systemInfo?.memory?.mem?.total;
	$: memory = `${(memoryFraction * 100).toFixed(0)}%`;
	$: swapFraction =
		systemInfo?.memory?.swap?.used / systemInfo?.memory?.swap?.total;
	$: swap = `${(swapFraction * 100).toFixed(0)}%`;

	$: cpuSystemLoadStatus = statusFromRanges(systemInfo?.cpu?.system, 50, 80);
	$: cpuUserLoadStatus = statusFromRanges(systemInfo?.cpu?.user, 50, 80);
	$: cpuTemperatureStatus = statusFromRanges(systemInfo?.cpu?.temp, 55, 70);

	$: memoryStatus = statusFromRanges(memoryFraction, 0.75, 0.9);
	$: swapStatus = statusFromRanges(swapFraction, 0.75, 0.9);

	function statusFromRanges(
		value: number,
		semiBorder: number,
		badBorder: number
	) {
		if (value >= badBorder) return Status.BAD;
		else if (value >= semiBorder) return Status.SEMI;
		else return null;
	}
</script>

{#if systemInfo?.cpu}
	<StatusItem
		status={cpuSystemLoadStatus}
		info={cpuSystemLoad}
		label="CPU System Load" />
	<StatusItem
		status={cpuUserLoadStatus}
		info={cpuUserLoad}
		label="CPU User Load" />
	<StatusItem
		status={cpuTemperatureStatus}
		info={cpuTemperature}
		label="CPU Temperature" />
{/if}

{#if systemInfo?.memory?.mem}
	<StatusItem status={memoryStatus} info={memory} label="Memory" />
{/if}

{#if systemInfo?.memory?.swap}
	<StatusItem status={swapStatus} info={swap} label="Swap Memory" />
{/if}
