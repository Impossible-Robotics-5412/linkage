<script lang="ts">
	import { BackendCommunication } from '$lib/client/backend/backend-communication';
	import { robotCodeState } from '$lib/client/robot-code/state';
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
	.enable-disable-robot-button:global(> button) {
		background: $c-green;
	}
	.enable-disable-robot-button.enabled:global(> button) {
		background: $c-red;
	}
</style>
