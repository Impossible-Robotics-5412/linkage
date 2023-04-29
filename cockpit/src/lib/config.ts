import { invoke } from '@tauri-apps/api/tauri';

export interface Address {
	host: string;
	port: number;
}

export interface CockpitConfig {
	linkage_socket_address: Address;
	linkage_lib_address: Address;
	gauge_address: Address;
	cockpit_backend_logger_address: Address;
	linkage_lib_logger_address: Address;
	carburetor_logger_address: Address;
}

export async function getConfig() {
	const configJson = (await invoke('get_config')) as string;
	const config: CockpitConfig = JSON.parse(configJson);
	return config;
}

export async function setRobotHost(host: string | null) {
	const config = await getConfig();
	config.carburetor_logger_address.host = host;
	config.gauge_address.host = host;
	config.linkage_lib_address.host = host;
	config.linkage_lib_logger_address.host = host;
	config.linkage_socket_address.host = host;

	await invoke('set_cockpit_config', {
		cockpitConfigJson: JSON.stringify(config)
	});
}
