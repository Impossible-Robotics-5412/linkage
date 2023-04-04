<script lang="ts">
	import Button from './ui/Button.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { listen } from '@tauri-apps/api/event';

	let enabled = false;

	listen('linkage-lib-state-change', event => {
		console.log(event);

		if (event.payload === 'Enabled') enabled = true;
		else if (event.payload === 'Disabled') enabled = false;
	});

	async function enable() {
		invoke('enable');
	}

	async function disable() {
		invoke('disable');
	}
</script>

<div class:enabled class="enable-disable-robot-button">
	{#if enabled}
		<Button on:click={disable}>Disable</Button>
	{:else}
		<Button on:click={enable}>Enable</Button>
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
