<script lang="ts">
	import { Backend } from '$lib/client/backend';
	import { backendState } from '$lib/client/backend-state';
	import { state } from '$lib/state';
</script>

<div>
	<button on:click={() => Backend.shared.enableLinkage()}>Enable</button>
	<button on:click={() => Backend.shared.disableLinkage()}>Disable</button>
	<pre>Robot Code Status: {$state.enabled ? 'Enabled' : 'Disabled'}</pre>
	<pre>Backend Status:    {$backendState.status}</pre>
</div>