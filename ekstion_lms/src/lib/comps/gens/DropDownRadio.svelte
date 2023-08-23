<script>
	export let options = [];
</script>

<div class="flex flex-col">
	<button
		id="dropdownRadioBgHoverButton"
		data-dropdown-toggle="dropdownRadioBgHover"
		class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center inline-flex items-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
		type="button"
		>Dropdown radio <svg
			class="w-2.5 h-2.5 ml-2.5"
			aria-hidden="true"
			xmlns="http://www.w3.org/2000/svg"
			fill="none"
			viewBox="0 0 10 6"
		>
			<path
				stroke="currentColor"
				stroke-linecap="round"
				stroke-linejoin="round"
				stroke-width="2"
				d="m1 1 4 4 4-4"
			/>
		</svg></button
	>

	<!-- Dropdown menu -->
	<div
		id="dropdownRadioBgHover"
		class="z-10 w-48 bg-white divide-y divide-gray-100 rounded-lg shadow dark:bg-gray-700 dark:divide-gray-600"
	>
		<ul
			class="p-3 space-y-1 text-sm text-gray-700 dark:text-gray-200"
			aria-labelledby="dropdownRadioBgHoverButton"
		>
			{#each options as opt}
				<li>
					<div class="flex items-center p-2 rounded hover:bg-gray-100 dark:hover:bg-gray-600">
						<input
							id="default-radio-4"
							type="radio"
							value=""
							name="default-radio"
							class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-700 dark:focus:ring-offset-gray-700 focus:ring-2 dark:bg-gray-600 dark:border-gray-500 outline-none"
						/>
						<label
							for="default-radio-4"
							class="w-full ml-2 text-sm font-medium text-gray-900 rounded dark:text-gray-300"
							>{opt}</label
						>
					</div>
				</li>
			{/each}
		</ul>
	</div>
</div>
