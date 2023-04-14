<script lang="ts">
	import Button from './ui/Button.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { listen } from '@tauri-apps/api/event';
	import { robotCodeState } from '$lib/state/robot-code';
	import { loggerState } from '$lib/state/logger';

	listen('linkage_lib_state_change', event => {
		$robotCodeState.changing = false;
		if (event.payload === 'Enabled') {
			$robotCodeState.enabled = true;
			$loggerState.selectedTabId = 'linkage';
		} else if (event.payload === 'Disabled') {
			$robotCodeState.enabled = false;
			$loggerState.selectedTabId = 'cockpit-backend';
		}
	});

	async function enable() {
		$robotCodeState.changing = true;
		invoke('enable');
	}

	async function disable() {
		$robotCodeState.changing = true;
		invoke('disable');
	}
</script>

<div
	class:enabled={$robotCodeState.enabled}
	class="enable-disable-robot-button">
	{#if $robotCodeState.enabled}
		<Button disabled={$robotCodeState.changing} on:click={disable}>
			Disable
		</Button>
	{:else}
		<Button disabled={$robotCodeState.changing} on:click={enable}>
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
