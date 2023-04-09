import type { ComponentType } from 'svelte';

import LinkageLoggerOutput from '$lib/components/logging/LinkageLoggerOutput.svelte';
import CockpitBackendLoggerOutput from '$lib/components/logging/CockpitBackendLoggerOutput.svelte';
import { writable } from 'svelte/store';

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
	} satisfies Tab
};

export type TabId = keyof typeof tabs;

export interface LoggerState {
	selectedTabId: TabId;
}

export const loggerState = writable<LoggerState>({
	selectedTabId: 'cockpit-backend'
});
