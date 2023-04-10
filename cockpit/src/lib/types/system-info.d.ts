interface SystemInfo {
	cpu?: {
		user: number;
		system: number;
		idle: number;
		temp?: number;
	};
	memory?: {
		swap?: {
			used: number;
			total: number;
		};
		mem?: {
			used: number;
			total: number;
		};
	};
	uptime?: number;
	service_info: {
		carburetor_status: boolean;
		gauge_status: boolean;
		linkage_socket_status: boolean;
	};
	robot_code_exists: boolean;
}
