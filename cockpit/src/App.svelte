<script lang="ts">
	import EnableDisableRobotButton from '$lib/components/EnableDisableRobotButton.svelte';
	import Loggers from '$lib/components/logger/Loggers.svelte';
	import StatusContainer from '$lib/components/status/StatusContainer.svelte';
	import { gamepadState } from '$lib/backend';
	import { AxisControl, ButtonControl } from '$lib/gamepad-data';
</script>

<main>
	<div class="main-window">
		<StatusContainer />
<!--		<Loggers />-->

		<div class="gamepads">
			{#each Object.values($gamepadState.gamepads) as gamepad}
				<div>
					<h2>Gamepad {gamepad.id}</h2>
					<table>
						<tr>
							<th>BUTTONS</th>
							<th>AXIS</th>
						</tr>
						{#each Object.entries(gamepad.buttons) as [control, value]}
							<tr>
								<td>{ButtonControl[control]}</td>
								<td>{value}</td>
							</tr>
						{/each}
						{#each Object.entries(gamepad.axis) as [control, value]}
							<tr>
								<td>{AxisControl[control]}</td>
								<td>{value}</td>
							</tr>
						{/each}
					</table>
				</div>
			{/each}
		</div>

		<EnableDisableRobotButton />
	</div>
</main>

<style lang="scss">
	main {
		width: 100vw;
		height: 100vh;
	}

	.gamepads {
		display: flex;
	  	gap: 4rem;

      table, th, td {
        border: 1px solid;
      }
	}

	.main-window {
		-webkit-box-sizing: border-box;
		-moz-box-sizing: border-box;
		box-sizing: border-box;
		padding: 1.5rem;

		width: 100%;
		height: 100%;

		display: grid;
		grid-template-columns: 16rem auto;
		grid-template-rows: auto min-content;
		gap: 1.5rem;
	}
</style>
