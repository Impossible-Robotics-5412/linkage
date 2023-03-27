import type { PageLoad } from './$types';

export const load = (async ({ fetch }) => {
	await fetch('/backend/start', { method: 'post' });

	return {};
}) satisfies PageLoad;
