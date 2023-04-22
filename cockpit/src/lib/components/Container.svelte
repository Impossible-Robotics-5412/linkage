<script lang="ts">
	export let noPadding = false;
	export let scrollable = false;
</script>

<div class="container">
	{#if $$slots.header}
		<div class="header">
			<slot name="header" />
		</div>
	{/if}

	<div class:scrollable class:noPadding class="content">
		<slot />
	</div>
</div>

<style lang="scss">
	@use '../style/vars' as *;

	.container {
		border: 1px solid $c-gray-2;
		box-sizing: border-box;

		border-radius: 6px;
		overflow: hidden;
		// This transform is needed to make border-radius work on safari for some reason...
		transform: translateZ(0);

		background-color: $c-background;

		width: 100%;
		height: 100%;

		display: grid;
		grid-template-columns: 100%;
		grid-template-rows: auto calc(100% - 3rem);
	}

	.header {
		display: flex;
		align-items: center;

		padding: 0 1.5rem;
		height: 3rem;

		background: $c-gray-1;

		border-bottom: 1px solid $c-gray-2;
	}

	.content {
		&:not(.noPadding) {
			&:not(.scrollable) {
				padding: 1.5rem;
			}
		}

		&.scrollable {
			height: 100%;
			overflow-y: scroll;
			padding: 1.5rem;
			box-sizing: border-box;
		}
	}
</style>
