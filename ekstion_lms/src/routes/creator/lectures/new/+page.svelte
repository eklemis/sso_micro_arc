<script>
	import DropDownRadio from '$lib/comps/gens/DropDownRadio.svelte';
	import { DocType } from '$lib/enums';

	console.log(DocType);
</script>

<nav />
<section>
	<h3 class=" text-3xl">New Lecture</h3>
	<form class="flex flex-col gap-y-3">
		<div class="flex items-baseline">
			<label for="title" class=" text-zinc-500 w-28">Title</label>
			<input id="title" type="text" class="border" />
		</div>
		<div class="flex items-baseline border">
			<label for="type" class=" text-zinc-500 w-28">Type</label>
			<DropDownRadio
				options={[
					'MD Doc',
					'Video',
					'Screencast - Presentation',
					'Screencast - Coding',
					'Quiz',
					'Formulir',
					'Survey'
				]}
			/>
		</div>
	</form>
</section>
