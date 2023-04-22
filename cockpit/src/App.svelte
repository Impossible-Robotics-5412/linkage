<script lang='ts'>
	import Sidebar from '$lib/components/Sidebar.svelte';
	import type { Tab } from '$lib/logger';

	let selectedTab: Tab;
</script>

<main>
	<Sidebar bind:selectedTab />
	<div class='main-window'>
		{#if selectedTab}
			<svelte:component this={selectedTab.component} />
		{:else}
			<h2>No tab selected</h2>
		{/if}
	</div>
</main>

<style lang='scss'>
  main {
    width: 100vw;
    height: 100vh;
    display: flex;
  }

  .main-window {
    width: 100%;
    height: 100%;
  }
</style>
