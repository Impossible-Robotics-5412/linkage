export interface Config {
	port: Port;
	carburetorAddress: Address;
}

export interface Address {
	host: Host;
	port: Port;
}

export type Host = string;
export type Port = number;
