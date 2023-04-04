export type BackendMessage = [
	number,
	number,
	number,
	number,
	number,
	number,
	number,
	number
];

export const FrontendToBackendMessage: Record<string, BackendMessage> = {
	ENABLE_LINKAGE: [0x00, 0, 0, 0, 0, 0, 0, 0],
	DISABLE_LINKAGE: [0x01, 0, 0, 0, 0, 0, 0, 0]
};

export const BackendToFrontendMessage: Record<string, BackendMessage> = {
	ENABLED: [0x08, 0, 0, 0, 0, 0, 0, 0],
	DISABLED: [0x09, 0, 0, 0, 0, 0, 0, 0]
};
