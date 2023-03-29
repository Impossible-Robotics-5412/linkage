import { BackendProcess } from '$lib/server/backend-process';
import type { FrontendResponse } from '../../../types/frontend-response';
import type { RequestHandler } from './$types';

export const POST = (async () => {
	try {
		BackendProcess.shared.start();
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
