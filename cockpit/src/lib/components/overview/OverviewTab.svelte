<script lang='ts'>
	import StatusContainer from '$lib/components/overview/status/StatusContainer.svelte';
	import EnableDisableRobotButton from '$lib/components/overview/EnableDisableRobotButton.svelte';
	import Loggers from '$lib/components/overview/logger/Loggers.svelte';
</script>

<div class='overview'>
	<StatusContainer />
	<Loggers/>
	<EnableDisableRobotButton />
</div>

<style>
	.overview {
        box-sizing: border-box;
        padding: 1.5rem;

        width: 100%;
        height: 100%;

        display: grid;
        grid-template-columns: 16rem auto;
        grid-template-rows: auto min-content;
        gap: 1.5rem;
	}
</style>