import { ChildProcess, spawn } from 'child_process';
import type { RequestHandler } from './$types';

let backendProcess: ChildProcess | undefined = undefined;

export const POST = (({}) => {
	if (backendProcess?.pid) return new Response();

	backendProcess = spawn('../../../target/debug/cockpit-backend');

	backendProcess?.stdout?.on('data', data => {
		process.stdout.write(`[CockpitBackend-out]: ${data}`);
	});

	backendProcess?.stderr?.on('data', data => {
		process.stderr.write(`[CockpitBackend-err]: ${data}`);
	});

	backendProcess?.on('close', code => {
		console.log(`exited CockpitBackend with code ${code}`);
	});

	return new Response();
}) satisfies RequestHandler;
