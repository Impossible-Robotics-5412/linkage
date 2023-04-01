<script lang="ts">
	import {
		backendState,
		BackendStatus,
		getBackendStatusLabel
	} from '$lib/client/backend/state';
	import { robotCodeState } from '$lib/client/robot-code/state';
	import Container from './Container.svelte';
	import StatusItem from './StatusItem.svelte';
	import { Status } from './StatusItem.svelte';

	let cockpitBackendStatus = Status.BAD;
	$: {
		if ($backendState.status >= BackendStatus.CONNECTED)
			cockpitBackendStatus = Status.GOOD;
		else if ($backendState.status >= BackendStatus.PROCESS_STARTED)
			cockpitBackendStatus = Status.SEMI;
		else cockpitBackendStatus = Status.BAD;
	}

	let robotCodeStatus = Status.BAD;
	$: {
		if ($robotCodeState.enabled) robotCodeStatus = Status.GOOD;
		else robotCodeStatus = Status.BAD;
	}
</script>

<Container>
	<div slot="header">
		<h3>Status</h3>
	</div>

	<div class="status">
		<StatusItem
			info={getBackendStatusLabel($backendState.status)}
			label="Cockpit Backend"
			status={cockpitBackendStatus} />

		<StatusItem
			info={`Robot Code ${robotCodeStatus ? 'Enabled' : 'Disabled'}`}
			label="Robot Code"
			status={robotCodeStatus} />
	</div>
</Container>

<style lang="scss">
	.status {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;

		& :global(> *) {
			border-bottom: 1px solid $c-gray-1;
			padding-bottom: 0.5rem;

			&:last-child {
				border-bottom: none;
				padding-bottom: 0;
			}
		}
	}
</style>
