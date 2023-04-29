<script lang="ts">
	import { getConfig, type CockpitConfig, setRobotHost } from '$lib/config';

	let config: CockpitConfig | null = null;
	let robotHost = null;

	fetchConfig();

	async function fetchConfig() {
		config = await getConfig();
		updateRobotHost();
	}

	function updateRobotHost() {
		if (
			config?.carburetor_logger_address.host ===
				config?.gauge_address.host &&
			config?.carburetor_logger_address.host ===
				config?.linkage_lib_address.host &&
			config?.carburetor_logger_address.host ===
				config?.linkage_lib_logger_address.host &&
			config?.carburetor_logger_address.host ===
				config?.linkage_socket_address.host &&
			config?.carburetor_logger_address.host ===
				config?.gauge_address.host
		) {
			robotHost = config?.carburetor_logger_address.host;
		} else {
			robotHost = null;
		}
	}

	async function updateConfig() {
		await setRobotHost(robotHost);
		await fetchConfig();
		updateRobotHost();
	}
</script>

<form on:submit|preventDefault={updateConfig}>
	<label for="robot-host">Robot Host</label>
	<input
		bind:value={robotHost}
		type="text"
		name="robot-host"
		placeholder="---"
		id="robot-host" />

	<button type="submit">Update Config</button>
</form>
