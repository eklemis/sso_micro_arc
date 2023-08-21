<script>
	import { onMount } from 'svelte';
	import { is_valid_email } from '$lib/validations';
	import { backend_base_url } from '$lib/stores';
	import { extract_session_id, extract_auth_code } from '$lib/extraction';
	import { setCookie } from '$lib/cookies';
	import axios from 'axios';
	import { page } from '$app/stores';
	/** @type {import('./$types').PageData} */
	export let data;
	let cred = {
		email: '',
		password: '',
		url: '',
		ipv4: '',
		user_agen: ''
	};
	let server_port = '';
	onMount(() => {
		if (!data.prev_url) {
			cred.url = window.location.origin;
		} else {
			cred.url = data.prev_url;
		}
		axios.get('https://api.ipify.org/').then((resp) => {
			cred.ipv4 = resp.data;
		});
		cred.user_agen = window.navigator.userAgent;
		backend_base_url.subscribe((base_url) => {
			server_port = base_url;
		});
		console.log('page url:', page);
	});

	let show_error = false;
	const submit_credentials = () => {
		const end_point = server_port + 'login';
		axios.post(end_point, cred).then((resp) => {
			let resp_data = resp.data;
			if (resp.status == 200) {
				let session_id = extract_session_id(resp_data);
				let auth_code = extract_auth_code(resp_data);
				//use cookie instead
				//set session id to be 2 days
				setCookie('session_id', session_id, 2);
				//set auth code to 5 mins
				setCookie('auth_code', auth_code, 0.00347222);
				console.log('Prepare redirect to:', cred.url);
				//throw redirect(308, cred.url !== '' ? cred.url : '/');
				window.location.href = cred.url;
			}
		});
	};
	const signin = () => {
		if (invalidEmail || emptyEmail || emptyPassword) {
			show_error = true;
		} else {
			show_error = false;
			submit_credentials();
		}
	};

	//DATA VALIDATION
	$: invalidEmail = !is_valid_email(cred.email);
	$: emptyEmail = cred.email.trim() === '';
	$: emptyPassword = cred.password.trim() === '';
</script>

<div class="bg-white relative lg:py-9">
	<div
		class="flex flex-col items-center justify-between pt-0 pr-10 pb-0 pl-10 mt-0 mr-auto mb-0 ml-auto max-w-7xl
	xl:px-5 lg:flex-row"
	>
		<div class="flex flex-col items-center w-full pt-5 pr-10 pb-20 pl-10 lg:pt-20 lg:flex-row">
			<div class="w-full bg-cover relative max-w-md lg:max-w-2xl lg:w-7/12">
				<div class="flex flex-col items-center justify-center w-full h-full relative lg:pr-10">
					<img
						src="https://res.cloudinary.com/macxenon/image/upload/v1631570592/Run_-_Health_qcghbu.png"
						class="btn-"
						alt=""
					/>
				</div>
			</div>
			<div class="w-full mt-20 mr-0 mb-0 ml-0 relative z-10 max-w-2xl lg:mt-0 lg:w-5/12">
				<div
					class="flex flex-col items-start justify-start pt-10 pr-10 pb-10 pl-10 bg-white shadow-2xl rounded-xl
		  relative z-10"
				>
					<p class="w-full text-4xl font-medium text-center leading-snug font-serif">Sign in</p>
					<div class="w-full mt-6 mr-0 mb-0 ml-0 relative space-y-8">
						<div class="relative">
							<p
								class="bg-white pt-0 pr-2 pb-0 pl-2 -mt-3 mr-0 mb-0 ml-2 font-medium text-gray-600 absolute"
							>
								Email
							</p>
							<input
								placeholder="123@ex.com"
								bind:value={cred.email}
								type="text"
								class="border placeholder-gray-400 focus:outline-none
				focus:border-black w-full pt-4 pr-4 pb-4 pl-4 mt-2 mr-0 mb-0 ml-0 text-base block bg-white
				border-gray-300 rounded-md"
							/>
							{#if show_error}
								{#if emptyEmail}
									<div
										class="absolute right-0 top-0 w-[40%] p-1 rounded-sm border bg-red-500 border-red-500 flex justify-center"
									>
										<p class="text-[10px] font-bold italic text-white">*Email can't be empty</p>
									</div>
								{:else if invalidEmail}
									<div
										class="absolute right-0 top-0 w-[40%] p-1 rounded-sm border bg-red-500 border-red-500 flex justify-center"
									>
										<p class="text-[10px] font-bold italic text-white">*Email format incorrect</p>
									</div>
								{/if}
							{/if}
						</div>

						<div class="relative">
							<p
								class="bg-white pt-0 pr-2 pb-0 pl-2 -mt-3 mr-0 mb-0 ml-2 font-medium text-gray-600
				absolute"
							>
								Password
							</p>
							<input
								placeholder="Password"
								bind:value={cred.password}
								type="password"
								class="border placeholder-gray-400 focus:outline-none
				focus:border-black w-full pt-4 pr-4 pb-4 pl-4 mt-2 mr-0 mb-0 ml-0 text-base block bg-white
				border-gray-300 rounded-md"
							/>
							{#if show_error && emptyPassword}
								<div
									class="absolute right-0 top-0 w-[40%] p-1 rounded-sm border bg-red-500 border-red-500 flex justify-center"
								>
									<p class="text-[10px] font-bold italic text-white">*Password can't be empty</p>
								</div>
							{/if}
						</div>
						<div class="relative">
							<p>
								Don't have account yet? <a
									class=" text-indigo-500 hover:text-indigo-600 ease font-bold"
									href={`/signup/${data.prev_url}`}
									alt="registration link">Register</a
								> first!
							</p>
						</div>
						<div class="relative">
							<button
								class="w-full inline-block pt-4 pr-5 pb-4 pl-5 text-xl font-medium text-center text-white bg-indigo-500
				rounded-lg transition duration-200 hover:bg-indigo-600 ease"
								on:click={signin}>LOGIN</button
							>
						</div>
					</div>
				</div>
				<svg
					viewbox="0 0 91 91"
					class="absolute top-0 left-0 z-0 w-32 h-32 -mt-12 -ml-12 text-yellow-300
		  fill-current"
					><g stroke="none" strokewidth="1" fillrule="evenodd"
						><g fillrule="nonzero"
							><g
								><g
									><circle cx="3.261" cy="3.445" r="2.72" /><circle
										cx="15.296"
										cy="3.445"
										r="2.719"
									/><circle cx="27.333" cy="3.445" r="2.72" /><circle
										cx="39.369"
										cy="3.445"
										r="2.72"
									/><circle cx="51.405" cy="3.445" r="2.72" /><circle
										cx="63.441"
										cy="3.445"
										r="2.72"
									/><circle cx="75.479" cy="3.445" r="2.72" /><circle
										cx="87.514"
										cy="3.445"
										r="2.719"
									/></g
								><g transform="translate(0 12)"
									><circle cx="3.261" cy="3.525" r="2.72" /><circle
										cx="15.296"
										cy="3.525"
										r="2.719"
									/><circle cx="27.333" cy="3.525" r="2.72" /><circle
										cx="39.369"
										cy="3.525"
										r="2.72"
									/><circle cx="51.405" cy="3.525" r="2.72" /><circle
										cx="63.441"
										cy="3.525"
										r="2.72"
									/><circle cx="75.479" cy="3.525" r="2.72" /><circle
										cx="87.514"
										cy="3.525"
										r="2.719"
									/></g
								><g transform="translate(0 24)"
									><circle cx="3.261" cy="3.605" r="2.72" /><circle
										cx="15.296"
										cy="3.605"
										r="2.719"
									/><circle cx="27.333" cy="3.605" r="2.72" /><circle
										cx="39.369"
										cy="3.605"
										r="2.72"
									/><circle cx="51.405" cy="3.605" r="2.72" /><circle
										cx="63.441"
										cy="3.605"
										r="2.72"
									/><circle cx="75.479" cy="3.605" r="2.72" /><circle
										cx="87.514"
										cy="3.605"
										r="2.719"
									/></g
								><g transform="translate(0 36)"
									><circle cx="3.261" cy="3.686" r="2.72" /><circle
										cx="15.296"
										cy="3.686"
										r="2.719"
									/><circle cx="27.333" cy="3.686" r="2.72" /><circle
										cx="39.369"
										cy="3.686"
										r="2.72"
									/><circle cx="51.405" cy="3.686" r="2.72" /><circle
										cx="63.441"
										cy="3.686"
										r="2.72"
									/><circle cx="75.479" cy="3.686" r="2.72" /><circle
										cx="87.514"
										cy="3.686"
										r="2.719"
									/></g
								><g transform="translate(0 49)"
									><circle cx="3.261" cy="2.767" r="2.72" /><circle
										cx="15.296"
										cy="2.767"
										r="2.719"
									/><circle cx="27.333" cy="2.767" r="2.72" /><circle
										cx="39.369"
										cy="2.767"
										r="2.72"
									/><circle cx="51.405" cy="2.767" r="2.72" /><circle
										cx="63.441"
										cy="2.767"
										r="2.72"
									/><circle cx="75.479" cy="2.767" r="2.72" /><circle
										cx="87.514"
										cy="2.767"
										r="2.719"
									/></g
								><g transform="translate(0 61)"
									><circle cx="3.261" cy="2.846" r="2.72" /><circle
										cx="15.296"
										cy="2.846"
										r="2.719"
									/><circle cx="27.333" cy="2.846" r="2.72" /><circle
										cx="39.369"
										cy="2.846"
										r="2.72"
									/><circle cx="51.405" cy="2.846" r="2.72" /><circle
										cx="63.441"
										cy="2.846"
										r="2.72"
									/><circle cx="75.479" cy="2.846" r="2.72" /><circle
										cx="87.514"
										cy="2.846"
										r="2.719"
									/></g
								><g transform="translate(0 73)"
									><circle cx="3.261" cy="2.926" r="2.72" /><circle
										cx="15.296"
										cy="2.926"
										r="2.719"
									/><circle cx="27.333" cy="2.926" r="2.72" /><circle
										cx="39.369"
										cy="2.926"
										r="2.72"
									/><circle cx="51.405" cy="2.926" r="2.72" /><circle
										cx="63.441"
										cy="2.926"
										r="2.72"
									/><circle cx="75.479" cy="2.926" r="2.72" /><circle
										cx="87.514"
										cy="2.926"
										r="2.719"
									/></g
								><g transform="translate(0 85)"
									><circle cx="3.261" cy="3.006" r="2.72" /><circle
										cx="15.296"
										cy="3.006"
										r="2.719"
									/><circle cx="27.333" cy="3.006" r="2.72" /><circle
										cx="39.369"
										cy="3.006"
										r="2.72"
									/><circle cx="51.405" cy="3.006" r="2.72" /><circle
										cx="63.441"
										cy="3.006"
										r="2.72"
									/><circle cx="75.479" cy="3.006" r="2.72" /><circle
										cx="87.514"
										cy="3.006"
										r="2.719"
									/></g
								></g
							></g
						></g
					></svg
				>
				<svg
					viewbox="0 0 91 91"
					class="absolute bottom-0 right-0 z-0 w-32 h-32 -mb-12 -mr-12 text-indigo-500
		  fill-current"
					><g stroke="none" strokewidth="1" fillrule="evenodd"
						><g fillrule="nonzero"
							><g
								><g
									><circle cx="3.261" cy="3.445" r="2.72" /><circle
										cx="15.296"
										cy="3.445"
										r="2.719"
									/><circle cx="27.333" cy="3.445" r="2.72" /><circle
										cx="39.369"
										cy="3.445"
										r="2.72"
									/><circle cx="51.405" cy="3.445" r="2.72" /><circle
										cx="63.441"
										cy="3.445"
										r="2.72"
									/><circle cx="75.479" cy="3.445" r="2.72" /><circle
										cx="87.514"
										cy="3.445"
										r="2.719"
									/></g
								><g transform="translate(0 12)"
									><circle cx="3.261" cy="3.525" r="2.72" /><circle
										cx="15.296"
										cy="3.525"
										r="2.719"
									/><circle cx="27.333" cy="3.525" r="2.72" /><circle
										cx="39.369"
										cy="3.525"
										r="2.72"
									/><circle cx="51.405" cy="3.525" r="2.72" /><circle
										cx="63.441"
										cy="3.525"
										r="2.72"
									/><circle cx="75.479" cy="3.525" r="2.72" /><circle
										cx="87.514"
										cy="3.525"
										r="2.719"
									/></g
								><g transform="translate(0 24)"
									><circle cx="3.261" cy="3.605" r="2.72" /><circle
										cx="15.296"
										cy="3.605"
										r="2.719"
									/><circle cx="27.333" cy="3.605" r="2.72" /><circle
										cx="39.369"
										cy="3.605"
										r="2.72"
									/><circle cx="51.405" cy="3.605" r="2.72" /><circle
										cx="63.441"
										cy="3.605"
										r="2.72"
									/><circle cx="75.479" cy="3.605" r="2.72" /><circle
										cx="87.514"
										cy="3.605"
										r="2.719"
									/></g
								><g transform="translate(0 36)"
									><circle cx="3.261" cy="3.686" r="2.72" /><circle
										cx="15.296"
										cy="3.686"
										r="2.719"
									/><circle cx="27.333" cy="3.686" r="2.72" /><circle
										cx="39.369"
										cy="3.686"
										r="2.72"
									/><circle cx="51.405" cy="3.686" r="2.72" /><circle
										cx="63.441"
										cy="3.686"
										r="2.72"
									/><circle cx="75.479" cy="3.686" r="2.72" /><circle
										cx="87.514"
										cy="3.686"
										r="2.719"
									/></g
								><g transform="translate(0 49)"
									><circle cx="3.261" cy="2.767" r="2.72" /><circle
										cx="15.296"
										cy="2.767"
										r="2.719"
									/><circle cx="27.333" cy="2.767" r="2.72" /><circle
										cx="39.369"
										cy="2.767"
										r="2.72"
									/><circle cx="51.405" cy="2.767" r="2.72" /><circle
										cx="63.441"
										cy="2.767"
										r="2.72"
									/><circle cx="75.479" cy="2.767" r="2.72" /><circle
										cx="87.514"
										cy="2.767"
										r="2.719"
									/></g
								><g transform="translate(0 61)"
									><circle cx="3.261" cy="2.846" r="2.72" /><circle
										cx="15.296"
										cy="2.846"
										r="2.719"
									/><circle cx="27.333" cy="2.846" r="2.72" /><circle
										cx="39.369"
										cy="2.846"
										r="2.72"
									/><circle cx="51.405" cy="2.846" r="2.72" /><circle
										cx="63.441"
										cy="2.846"
										r="2.72"
									/><circle cx="75.479" cy="2.846" r="2.72" /><circle
										cx="87.514"
										cy="2.846"
										r="2.719"
									/></g
								><g transform="translate(0 73)"
									><circle cx="3.261" cy="2.926" r="2.72" /><circle
										cx="15.296"
										cy="2.926"
										r="2.719"
									/><circle cx="27.333" cy="2.926" r="2.72" /><circle
										cx="39.369"
										cy="2.926"
										r="2.72"
									/><circle cx="51.405" cy="2.926" r="2.72" /><circle
										cx="63.441"
										cy="2.926"
										r="2.72"
									/><circle cx="75.479" cy="2.926" r="2.72" /><circle
										cx="87.514"
										cy="2.926"
										r="2.719"
									/></g
								><g transform="translate(0 85)"
									><circle cx="3.261" cy="3.006" r="2.72" /><circle
										cx="15.296"
										cy="3.006"
										r="2.719"
									/><circle cx="27.333" cy="3.006" r="2.72" /><circle
										cx="39.369"
										cy="3.006"
										r="2.72"
									/><circle cx="51.405" cy="3.006" r="2.72" /><circle
										cx="63.441"
										cy="3.006"
										r="2.72"
									/><circle cx="75.479" cy="3.006" r="2.72" /><circle
										cx="87.514"
										cy="3.006"
										r="2.719"
									/></g
								></g
							></g
						></g
					></svg
				>
			</div>
		</div>
	</div>
</div>
