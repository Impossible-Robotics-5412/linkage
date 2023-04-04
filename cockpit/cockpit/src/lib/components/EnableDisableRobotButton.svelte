<script lang="ts">
	import { BackendCommunication } from '$lib/backend/backend-communication';
	import { robotCodeState } from '$lib/robot-code/state';
	import Button from './ui/Button.svelte';
</script>

<div
	class:enabled={$robotCodeState.enabled}
	class="enable-disable-robot-button">
	{#if $robotCodeState.enabled}
		<Button on:click={() => BackendCommunication.shared.disableLinkage()}>
			Disable
		</Button>
	{:else}
		<Button on:click={() => BackendCommunication.shared.enableLinkage()}>
			Enable
		</Button>
	{/if}
</div>

<style lang="scss">
	@use '../style/vars' as *;

	:global(.enable-disable-robot-button > button) {
		background: $c-green;
	}
	:global(.enable-disable-robot-button.enabled > button) {
		background: $c-red;
	}
</style>
