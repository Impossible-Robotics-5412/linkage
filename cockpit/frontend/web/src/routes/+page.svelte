<script lang="ts">
	import { BackendConnection } from '$lib/backend/connection';

	const backend = new BackendConnection();
</script>

<button on:click={() => backend.enableLinkage()}>Enable</button>
<button on:click={() => backend.disableLinkage()}>Disable</button>
