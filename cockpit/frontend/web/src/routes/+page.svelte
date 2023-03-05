<script lang="ts">
	import { BackendConnection } from '$lib/backend/connection';
	import { state } from '$lib/state';
	import type { PageData } from './$types';

	export let data: PageData;

	$: backend = new BackendConnection($state.host, $state.port);

	$state.host = data.serverNetworkInterfaceInfo?.address ?? '0.0.0.0';
</script>

<div>
	<button on:click={() => backend.enableLinkage()}>Enable</button>
	<button on:click={() => backend.disableLinkage()}>Disable</button>
	{$state.enabled ? 'Enabled' : 'Disabled'}
</div>

<br />

<div>
	<label for="backend-host">Backend Host</label>
	<input bind:value={$state.host} type="text" name="backend-host" id="backend-host" />
	<br />
	<label for="backend-port">Backend Port</label>
	<input bind:value={$state.port} type="number" name="backend-port" id="backend-port" />
</div>
