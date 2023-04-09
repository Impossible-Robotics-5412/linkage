<script lang="ts" context="module">
	export enum Status {
		BAD,
		SEMI,
		GOOD
	}
</script>

<script lang="ts">
	export let info: string;
	export let label: string;
	export let status: Status | undefined = undefined;
</script>

<div class="status-item" title={info}>
	<div class="main">
		<span>{label}</span>
		<span
			class="info"
			class:bad={status === Status.BAD}
			class:semi={status === Status.SEMI}
			class:good={status === Status.GOOD}>
			{info}
		</span>
	</div>
</div>

<style lang="scss">
	@use '../../style/vars' as *;

	.main {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.info {
		color: $c-secondary;
		font-size: 12px;

		&.bad {
			color: $c-error;
			font-weight: bold;
		}

		&.semi {
			color: $c-warning;
			font-weight: bold;
		}

		&.good {
			color: $c-success;
			font-weight: bold;
		}
	}
</style>
