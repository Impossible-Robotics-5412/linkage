import type { ComponentType } from 'svelte';
import { writable } from 'svelte/store';

import LinkageLoggerOutput from '$lib/components/logger/LinkageLoggerOutput.svelte';
import CockpitBackendLoggerOutput from '$lib/components/logger/CockpitBackendLoggerOutput.svelte';
import CarbutetorLoggerOutput from '$lib/components/logger/CarbutetorLoggerOutput.svelte';

export interface Tab {
	name: string;
	loggerOutputComponent: ComponentType;
}

export const tabs = {
	'cockpit-backend': {
		name: 'Cockpit Backend',
		loggerOutputComponent: CockpitBackendLoggerOutput
	} satisfies Tab,
	'linkage': {
		name: 'Linkage',
		loggerOutputComponent: LinkageLoggerOutput
	} satisfies Tab,
	'carburetor': {
		name: 'Carburetor',
		loggerOutputComponent: CarbutetorLoggerOutput
	} satisfies Tab
};

export type TabId = keyof typeof tabs;

export interface LoggerState {
	selectedTabId: TabId;
}

export const loggerState = writable<LoggerState>({
	selectedTabId: 'cockpit-backend'
});
