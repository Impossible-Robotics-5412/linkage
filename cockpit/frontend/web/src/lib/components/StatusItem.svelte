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
	export let status: Status;
</script>

<div class="status-item" title={info}>
	<span>{label}</span>
	<div
		class:bad={status === Status.BAD}
		class:semi={status === Status.SEMI}
		class:good={status === Status.GOOD}
		class="indicator" />
</div>

<style lang="scss">
	.status-item {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.indicator {
		width: 12px;
		height: 12px;
		border-radius: 100%;

		background: $c-error;
	}

	.indicator.semi {
		background: $c-warning;
	}

	.indicator.good {
		background: $c-success;
	}
</style>
