<script lang="ts">
	import type {
		SystemCpuInfo,
		SystemMemoryInfo
	} from '$lib/types/system-info';
	import StatusItem from './StatusItem.svelte';
	import { Status } from '$lib/types/status';

	export let cpu: SystemCpuInfo;
	export let memory: SystemMemoryInfo;

	$: cpuSystemLoad = `${cpu?.system.toFixed(0)}%`;
	$: cpuUserLoad = `${cpu?.user.toFixed(0)}%`;
	$: cpuTemperature = `${cpu?.temp.toFixed(0)}℃`;

	$: memoryFraction = memory?.mem?.used / memory?.mem?.total;
	$: memoryInfo = `${(memoryFraction * 100).toFixed(0)}%`;
	$: swapFraction = memory?.swap?.used / memory?.swap?.total;
	$: swapInfo = `${(swapFraction * 100).toFixed(0)}%`;

	$: cpuSystemLoadStatus = statusFromRanges(cpu?.system, 50, 80);
	$: cpuUserLoadStatus = statusFromRanges(cpu?.user, 50, 80);
	$: cpuTemperatureStatus = statusFromRanges(cpu?.temp, 55, 70);

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

<StatusItem status={memoryStatus} info={memoryInfo} label="Memory" />

<StatusItem status={swapStatus} info={swapInfo} label="Swap Memory" />
