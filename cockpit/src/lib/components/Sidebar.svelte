<script lang="ts">
	import OverviewTab from '$lib/components/overview/OverviewTab.svelte';
	import type { Tab } from '$lib/types/tab';
	import { IconDeviceGamepad2, IconRobot } from '@tabler/icons-svelte';
	import GamepadsTab from '$lib/components/gamepads/GamepadsTab.svelte';

	const tabs: Tab[] = [
		{ label: 'Overview', component: OverviewTab, iconComponent: IconRobot },
		{
			label: 'Gamepads',
			component: GamepadsTab,
			iconComponent: IconDeviceGamepad2
		}
	];

	export let selectedTab: Tab = tabs[0];
</script>

<div class="sidebar">
	{#each tabs as tab}
		<button
			title={tab.label}
			class:selected={selectedTab === tab}
			on:click={() => (selectedTab = tab)}>
			<svelte:component this={tab.iconComponent} size={32} stroke={1.5} />
		</button>
	{/each}
</div>

<style lang="scss">
	@use '../style/vars' as *;

	.sidebar {
		border-right: 1px solid $c-gray-2;
		padding: 1.5rem 0.75rem;

		display: flex;
		flex-direction: column;
		gap: 0.75rem;

		& :global(.icon-gamepad) {
			width: 2.5rem;
			height: 2.5rem;
		}
	}

	.selected {
		background: $c-gray-1;
	}

	button {
		border: none;
		background: none;
		width: 3rem;
		height: 3rem;
		border-radius: 8px;
		cursor: pointer;
		transition: 50ms;
	}

	button:hover {
		background: $c-gray-2;
	}
</style>
