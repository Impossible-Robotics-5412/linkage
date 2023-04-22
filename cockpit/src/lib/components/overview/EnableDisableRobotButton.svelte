<script lang="ts">
	import Button from '../ui/Button.svelte';
	import {
		disableRobotCode,
		enableRobotCode,
		robotCode,
		systemInfo
	} from '$lib/backend';

	$: buttonDisabled =
		$robotCode.changingState || !$systemInfo?.robot_code_exists;
</script>

<div class:enabled={$robotCode.enabled} class="enable-disable-robot-button">
	{#if $robotCode.enabled}
		<Button disabled={buttonDisabled} on:click={disableRobotCode}>
			Disable
		</Button>
	{:else}
		<Button disabled={buttonDisabled} on:click={enableRobotCode}>
			Enable
		</Button>
	{/if}
</div>

<style lang="scss">
	@use '../../style/vars' as *;

	:global(.enable-disable-robot-button > button) {
		background: $c-green;
	}
	:global(.enable-disable-robot-button.enabled > button) {
		background: $c-red;
	}
</style>
