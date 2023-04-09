<script lang="ts">
	import StatusItem from './StatusItem.svelte';
	import { Status } from './StatusItem.svelte';
	import Container from './Container.svelte';
	import { robotCodeState } from '$lib/state/robot-code';

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
			info={`${robotCodeStatus ? 'Enabled' : 'Disabled'}`}
			label="Robot Status"
			status={robotCodeStatus} />
	</div>
</Container>

<style lang="scss">
	@use '../style/vars' as *;

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
