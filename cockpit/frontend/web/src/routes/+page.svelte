<script lang="ts">
	import { onMount } from 'svelte';

	let websocket: WebSocket;

	onMount(() => {
		websocket = new WebSocket('ws://127.0.0.1:3012');
	});

	function enable() {
		websocket.send(new Uint8Array([0, 0, 0, 0, 0, 0, 0, 0]));
		console.log('enable');
	}

	function disable() {
		websocket.send(new Uint8Array([1, 0, 0, 0, 0, 0, 0, 0]));
		console.log('disable');
	}
</script>

<button on:click={enable}>Enable</button>
<button on:click={disable}>Disable</button>
