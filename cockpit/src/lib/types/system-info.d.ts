export interface SystemInfo {
	cpu?: SystemCpuInfo;
	memory?: SystemMemoryInfo;
	service_info: SystemServiceInfo;
	uptime?: number;
	robot_code_exists: boolean;
}

export interface SystemCpuInfo {
	user: number;
	system: number;
	idle: number;
	temp?: number;
}

export interface SystemMemoryInfo {
	swap?: {
		used: number;
		total: number;
	};
	mem?: {
		used: number;
		total: number;
	};
}

export interface SystemServiceInfo {
	carburetor_status: boolean;
	gauge_status: boolean;
	linkage_socket_status: boolean;
}
