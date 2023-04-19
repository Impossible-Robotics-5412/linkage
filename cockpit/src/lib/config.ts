import { invoke } from '@tauri-apps/api/tauri';

export interface Address {
	host: string;
	port: number;
}

export interface CockpitConfig {
	linkage_socket_address: Address;
	linkage_lib_address: Address;
	cockpit_backend_logger_address: Address;
	linkage_lib_logger_address: Address;
	carburetor_logger_address: Address;
}

export async function getConfig() {
	return new Promise<CockpitConfig>((resolve, reject) => {
		invoke('config')
			.then((config_json: string) => {
				const config: CockpitConfig = JSON.parse(config_json);
				resolve(config);
			})
			.catch(reject);
	});
}
