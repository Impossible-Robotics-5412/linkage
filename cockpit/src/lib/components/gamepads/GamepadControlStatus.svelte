<script lang="ts">
	import { ButtonControl, AxisControl } from '$lib/gamepad-data';
	import { Status } from '$lib/types/status';
	import StatusItem from '$lib/components/overview/status/StatusItem.svelte';

	export let control: ButtonControl | AxisControl;
	export let controlMap: {[control: AxisControl|ButtonControl]: number};
	export let label: string;
	export let axisPreview = false;
	$: value = controlMap[control];

	function titleCase(string: string) {
		return string.toLowerCase().replace(/^_*(.)|_+(.)/g, (_, c, d) => {
			return c ? c.toUpperCase() : ' ' + d.toUpperCase();
		});
	}

	function map(
		value: number,
		in_min: number,
		in_max: number,
		out_min: number,
		out_max: number
	) {
		return (
			((value - in_min) * (out_max - out_min)) / (in_max - in_min) +
			out_min
		);
	}
</script>

{#if axisPreview}
	<StatusItem
		label={titleCase(label)}
		info={map(value, 0, 255, -1, 1).toFixed(1).replace('-0', '0')} />
{:else}
	<StatusItem
		label={titleCase(label)}
		info={value > 127 ? 'Pressed' : 'Not pressed'}
		status={value > 127 ? Status.GOOD : null} />
{/if}
