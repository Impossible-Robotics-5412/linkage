import { BackendCommunication } from '$lib/client/backend/backend-communication';
import { BackendStatus, setBackendStatus } from '$lib/client/backend/state';
import type { FrontendResponse } from '../types/frontend-response';
import type { PageLoad } from './$types';

export const load = (async ({ fetch }) => {
	await startBackend(fetch);
	BackendCommunication.shared.connect();
}) satisfies PageLoad;

async function startBackend(fetch: any) {
	setBackendStatus(BackendStatus.PROCESS_STARTING);
	const response = await fetch('/backend/start', { method: 'post' });
	const data: FrontendResponse = await response.json();
	if (data.success) {
		setBackendStatus(BackendStatus.PROCESS_STARTED);
	} else {
		console.error(data.error);
	}

	return data;
}
