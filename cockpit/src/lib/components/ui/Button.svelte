<script lang="ts">
	export let disabled = false;
</script>

<button {disabled} on:click>
	<slot />
</button>

<style lang="scss">
	@use '../../style/vars' as *;

	button {
		width: 100%;
		height: 3rem;

		border: none;
		border-radius: 6px;
		border: 1px solid #0002;

		background: $c-accent;

		cursor: pointer;

		color: white;
		font-weight: bold;

		&:hover {
			opacity: 0.9;
		}

		&:disabled {
			opacity: 0.5;
		}
	}
</style>
