<script lang='ts'>
	import Container from '$lib/components/Container.svelte';
	import { gamepadState } from '$lib/backend';
	import { AxisControl, ButtonControl, type GamepadId } from '$lib/gamepad-data';
	import List from '$lib/components/ui/List.svelte';
	import GamepadControlStatus from '$lib/components/gamepads/GamepadControlStatus.svelte';

	let gamepadId: GamepadId | null;
	$: gamepadDatas = Object.values($gamepadState.gamepads);
	$: if (gamepadDatas.length > 0) {
		if (!gamepadId) gamepadId = gamepadDatas[0].id;
	} else {
		gamepadId = null;
	}
	$: gamepad = $gamepadState.gamepads[gamepadId];
</script>

<Container scrollable>
	<div class='header' slot='header'>
		<h3>Gamepads</h3>

		<select bind:value={gamepadId}>
			{#if gamepadDatas.length === 0}
				<option value={null}>Select a gamepad</option>
			{/if}

			{#each gamepadDatas as gamepad}
				<option value={gamepad.id}>Gamepad {gamepad.id}</option>
			{/each}
		</select>
	</div>

	{#if gamepad}
		<div class='gamepad'>
			<List>
				<h3>Cardinal</h3>
				{#each [
					ButtonControl.NORTH,
					ButtonControl.EAST,
					ButtonControl.SOUTH,
					ButtonControl.WEST
				] as control}
					<GamepadControlStatus
						{control}
						label={ButtonControl[control]}
						controlMap={gamepad.buttons} />
				{/each}
				<h3>Dpad</h3>
				{#each [
					ButtonControl.DPAD_UP,
					ButtonControl.DPAD_RIGHT,
					ButtonControl.DPAD_DOWN,
					ButtonControl.DPAD_LEFT
				] as control}
					<GamepadControlStatus
						{control}
						label={ButtonControl[control]}
						controlMap={gamepad.buttons} />
				{/each}
				<h3>Menu</h3>
				{#each [
					ButtonControl.START,
					ButtonControl.MODE,
					ButtonControl.SELECT
				] as control}
					<GamepadControlStatus
						{control}
						label={ButtonControl[control]}
						controlMap={gamepad.buttons} />
				{/each}
				<h3>Other</h3>
				{#each [ButtonControl.C, ButtonControl.Z] as control}
					<GamepadControlStatus
						{control}
						label={ButtonControl[control]}
						controlMap={gamepad.buttons} />
				{/each}
			</List>

			<List>
				<h3>Sticks</h3>
				{#each [ButtonControl.LEFT_THUMB, ButtonControl.RIGHT_THUMB] as control}
					<GamepadControlStatus
						{control}
						label={ButtonControl[control]}
						controlMap={gamepad.buttons} />
				{/each}
				{#each [
					AxisControl.LEFT_STICK_X,
					AxisControl.LEFT_STICK_Y,
					AxisControl.LEFT_Z,
					AxisControl.RIGHT_STICK_X,
					AxisControl.RIGHT_STICK_Y,
					AxisControl.RIGHT_Z
				] as control}
					<GamepadControlStatus
						{control}
						axisPreview
						label={AxisControl[control]}
						controlMap={gamepad.axis} />
				{/each}
				<h3>Triggers</h3>
				{#each [
					ButtonControl.LEFT_TRIGGER,
					ButtonControl.RIGHT_TRIGGER,
				] as control}
					<GamepadControlStatus
						{control}
						label={ButtonControl[control]}
						controlMap={gamepad.buttons} />
				{/each}
				{#each [
					ButtonControl.LEFT_TRIGGER_2,
					ButtonControl.RIGHT_TRIGGER_2
				] as control}
					<GamepadControlStatus
						{control}
						axisPreview
						label={ButtonControl[control]}
						controlMap={gamepad.buttons} />
				{/each}
			</List>
		</div>
	{:else}
		<div class='no-gamepads-found'>
			<h2>No Gamepads Found</h2>
		</div>
	{/if}
</Container>

<style lang='scss'>
  @use '../../style/vars' as *;

  .header {
    display: flex;
    justify-content: space-between;

    width: 100%;
  }

  .gamepad {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 2rem;
  }

  .no-gamepads-found {
    width: 100%;
    height: 100%;

    display: flex;
    justify-content: center;
    align-items: center;
  }
</style>
