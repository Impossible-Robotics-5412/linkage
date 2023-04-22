import type { ComponentType } from 'svelte';
import { writable } from 'svelte/store';

import LinkageLoggerOutput from '$lib/components/overview/logger/LinkageLoggerOutput.svelte';
import CockpitBackendLoggerOutput from '$lib/components/overview/logger/CockpitBackendLoggerOutput.svelte';
import CarburetorLoggerOutput from '$lib/components/overview/logger/CarburetorLoggerOutput.svelte';
import { LogLevel } from '$lib/process-logger';

export interface LoggerTab {
	name: string;
	loggerOutputComponent: ComponentType;
}

export const loggerTabs = {
	'cockpit-backend': {
		name: 'Cockpit Backend',
		loggerOutputComponent: CockpitBackendLoggerOutput
	} satisfies LoggerTab,
	'linkage': {
		name: 'Linkage',
		loggerOutputComponent: LinkageLoggerOutput
	} satisfies LoggerTab,
	'carburetor': {
		name: 'Carburetor',
		loggerOutputComponent: CarburetorLoggerOutput
	} satisfies LoggerTab
};

export type LoggerTabId = keyof typeof loggerTabs;

export interface LoggerState {
	selectedTabId: LoggerTabId;
	level: LogLevel;
}

export const loggerState = writable<LoggerState>({
	selectedTabId: 'cockpit-backend',
	level: LogLevel.DEBUG
});
