import { readFileSync } from 'fs';
import { homedir } from 'os';
import { join } from 'path';
import { parse as parseToml } from 'toml';
import { Config } from './config';

export class ConfigManager {
	public static shared = new ConfigManager();

	private _config: Config | undefined;

	public get config() {
		return this._config;
	}

	constructor() {
		try {
			const tomlString = readFileSync(
				join(homedir(), '.config/linkage/config.toml'),
				'utf8'
			);

			const toml = parseToml(tomlString)['linkage_lib'];

			this._config = {
				carburetorAddress: {
					host: toml['carburetor_address']['host'],
					port: toml['carburetor_address']['port']
				},
				port: toml['port']
			};
		} catch (err) {
			console.error(err);
		}
	}
}
