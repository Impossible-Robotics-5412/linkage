<script lang="ts">
	import { BackendConnection } from '$lib/backend/connection';
	import { state } from '$lib/state';
	import type { PageData } from './$types';

	export let data: { ipAddress: string };

	let backend: BackendConnection | undefined;

	$state.host = data.ipAddress ?? '0.0.0.0';

	reconnect();

	function reconnect() {
		backend?.disconnect();
		backend = new BackendConnection($state.host, $state.port);
	}
</script>

<div>
	<button on:click={() => backend?.enableLinkage()}>Enable</button>
	<button on:click={() => backend?.disableLinkage()}>Disable</button>
	{$state.enabled ? 'Enabled' : 'Disabled'}
</div>

<br />

<form on:submit={reconnect}>
	<label for="backend-host">Backend Host</label>
	<input bind:value={$state.host} type="text" name="backend-host" id="backend-host" />
	<br />
	<label for="backend-port">Backend Port</label>
	<input bind:value={$state.port} type="number" name="backend-port" id="backend-port" />
	<input type="submit" value="Reconnect" />
</form>
