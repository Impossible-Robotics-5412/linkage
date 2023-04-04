<script lang="ts">
	import Button from './ui/Button.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	
	let enabled = false;

	async function enable() {
		enabled = true;
		invoke('enable');
		console.log("Enable");
	}

	function disable() {
		invoke('disable');
		enabled = false;
		console.log("Disable");
	}
</script>

<div 
	class:enabled
	class="enable-disable-robot-button">
	{#if enabled}
		<Button on:click={disable}>
			Disable
		</Button>
	{:else}
		<Button on:click={enable}>
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
