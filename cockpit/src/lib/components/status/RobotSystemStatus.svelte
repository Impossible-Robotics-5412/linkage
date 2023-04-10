<script lang="ts">
	import StatusItem from './StatusItem.svelte';

	export let systemInfo: SystemInfo | undefined;

	$: cpuSystemLoad = `${systemInfo?.cpu?.system.toFixed(0)}%`;
	$: cpuUserLoad = `${systemInfo?.cpu?.user.toFixed(0)}%`;
	$: cpuTemperature = `${systemInfo?.cpu?.temp.toFixed(0)}℃`;

	$: memory = `${(
		(systemInfo?.memory?.mem?.used / systemInfo?.memory?.mem?.total) *
		100
	).toFixed(0)}%`;
	$: swap = `${(
		(systemInfo?.memory?.swap?.used / systemInfo?.memory?.swap?.total) *
		100
	).toFixed(0)}%`;
</script>

{#if systemInfo?.cpu}
	<StatusItem info={cpuSystemLoad} label="CPU System Load" />
	<StatusItem info={cpuUserLoad} label="CPU User Load" />
	<StatusItem info={cpuTemperature} label="CPU Temperature" />
{/if}

{#if systemInfo?.memory?.mem}
	<StatusItem info={memory} label="Memory" />
{/if}

{#if systemInfo?.memory?.swap}
	<StatusItem info={swap} label="Swap Memory" />
{/if}
