import { BackendProcess } from '$lib/server/backend-process';
import type { FrontendResponse } from '../../../types/frontend-response';
import type { RequestHandler } from './$types';

const backendProcess = new BackendProcess();

export const POST = (async () => {
	try {
		await backendProcess.start();
		return new Response(
			JSON.stringify({
				success: true,
				data: {}
			} satisfies FrontendResponse)
		);
	} catch (error: any) {
		return new Response(
			JSON.stringify({
				success: false,
				error
			} satisfies FrontendResponse)
		);
	}
}) satisfies RequestHandler;
