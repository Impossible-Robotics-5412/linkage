import type { PageServerLoad } from './$types';
import * as ip from 'ip';

export const ssr = false;
export const prerender = false;

export const load = (async ({ fetch }) => {
	await fetch('/backend/start', { method: 'post' });

	const ipAddress = ip.address();

	return { ipAddress };
}) satisfies PageServerLoad;
