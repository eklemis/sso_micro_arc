<script>
	import { onMount } from 'svelte';
	/** @type {import('./$types').PageData} */
	export let data;

	onMount(() => {
		console.log('data:', data);
	});
</script>

<section class="w-full h-full p-4 flex flex-col items-center justify-center content-center">
	<div class="border h-48 p-8 gap-y-2 flex flex-col rounded">
		<h1 class="text-3xl font-bold">Account activated</h1>
		<p class="mt-3">
			Your account is successfully <span class="text-green-500 font-bold">Activated</span>
		</p>
		<p>
			Please <a href="/login" class="p-1 border bg-blue-400 rounded text-white">Login</a> to start your
			journey with us!
		</p>
	</div>
</section>
