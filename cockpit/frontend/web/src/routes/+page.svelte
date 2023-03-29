<script lang="ts">
	import { Backend } from '$lib/client/backend';
	import { backendState, BackendStatus } from '$lib/client/backend-state';
	import Logger from '$lib/components/Logger.svelte';
	import { state } from '$lib/state';

	let showLogger = false;
	$: if ($backendState.status === BackendStatus.LOGGER_STARTED)
		showLogger = true;
</script>

<div>
	<button on:click={() => Backend.shared.enableLinkage()}>Enable</button>
	<button on:click={() => Backend.shared.disableLinkage()}>Disable</button>
	<pre>Robot Code Status: {$state.enabled ? 'Enabled' : 'Disabled'}</pre>
	<pre>Backend Status:    {$backendState.status}</pre>
</div>

<h2>Backend Log</h2>
{#if showLogger}
	{#if Backend.shared.loggerStream}
		<Logger stream={Backend.shared.loggerStream} />
	{/if}
{/if}
