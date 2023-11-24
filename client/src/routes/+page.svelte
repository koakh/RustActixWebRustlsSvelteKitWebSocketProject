<script lang="ts">
	import { onMount } from 'svelte';
	import { websocket } from '$lib/stores/websocket';

	onMount(() => {
		// TODO:
		// websocket.connect('ws://localhost:3001');
		websocket.connect();
		// unMount cleanup
		return () => {
			websocket.close();
		};
	});

	let message = '';

  // TODO: actixweb seems that don't manage websockets send messages, we use requests
	const sendMessage = () => {
		websocket.send(JSON.stringify({message}));
		message = '';
	};
</script>

<main>
	<h1>Hello SvelteKit with WebSocket</h1>
	<input bind:value={message} placeholder="Type a message" />
	<button on:click={sendMessage}>Send</button>
</main>
