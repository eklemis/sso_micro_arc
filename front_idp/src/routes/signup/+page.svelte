<script>
	import axios from 'axios';
	import { backend_base_url } from '$lib/stores';
	import { is_valid_email } from '$lib/validations';
	import { onMount } from 'svelte';
	/** @type {import('./$types').PageData} */
	export let data;

	let server_port = '';
	onMount(() => {
		backend_base_url.subscribe((base_url) => {
			server_port = base_url;
		});
	});
	let profile = {
		username: '',
		password: '',
		email: '',
		fullname: '',
		phone_number: '',
		profile_picture_url: ''
		//'https://pbs.twimg.com/profile_images/1553256945175633920/qpHziqx4_400x400.jpg'
	};
	const update_username_byemail = () =>
		(profile.username = profile.email.split('@')[0].split('.')[0]);

	let is_activated = false;

	/* Handling input validation */
	let err_message;
	$: emailOrUserNameTaken = err_message === 'EmailOrUserNameTaken';
	const weakPassword = (password) => {
		let hasLowercase = /[a-z]/.test(password);
		let hasUppercase = /[A-Z]/.test(password);
		let hasDigit = /[0-9]/.test(password);
		let hasSpecial = /[@#$%^&+=()-]/.test(password);
		let hasNoWhitespace = !/\s/.test(password);
		let hasCorrectLength = password.length >= 8 && password.length <= 20;

		return !(
			hasLowercase &&
			hasUppercase &&
			hasDigit &&
			hasSpecial &&
			hasNoWhitespace &&
			hasCorrectLength
		);
	};
	$: invalidPassword = err_message === 'InvalidPassword' || weakPassword(profile.password);
	$: invalidEmail = !is_valid_email(profile.email);
	$: emptyName = profile.fullname.trim() === '';
	$: emptyUserName = profile.username.trim() === '';

	/* Submit profile data */
	const extract_message = (text) => {
		const regex = /message:\s*"(.*?)"/;
		const match = text.match(regex);
		if (match && match[1]) {
			return match[1];
		} else {
			return '';
		}
	};
	const submit_profile = () => {
		let end_point = server_port + 'newuser';
		axios({
			method: 'post',
			url: end_point,
			data: profile
		})
			.then((resp) => {
				console.log(resp);
				if (resp.status === 200) {
					is_activated = true;
				}
			})
			.catch(function (error) {
				err_message = extract_message(String(error.response.data));
			});
	};
	$: console.log('err status: ', err_message);
</script>

{#if is_activated}
	<section class="w-full h-full p-4 flex flex-col items-center justify-center content-center">
		<div class="border h-48 p-8 gap-y-2 flex flex-col rounded">
			<h1 class="text-3xl font-bold">Registration success</h1>
			<p class="mt-3">
				Your account is successfully <span class="text-green-500 font-bold">Registered</span>
			</p>
			<p>
				We have sent you an email to confirm your ownership. Please open the email and activate your
				account.
			</p>
		</div>
	</section>
{:else}
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
						<p class="w-full text-4xl font-medium text-center leading-snug font-serif">
							Sign up for an account
						</p>
						<div class="w-full mt-6 mr-0 mb-0 ml-0 relative space-y-8">
							<div class="relative">
								<p
									class="bg-white pt-0 pr-2 pb-0 pl-2 -mt-3 mr-0 mb-0 ml-2 font-medium text-gray-600
                    absolute"
								>
									Full name
								</p>
								<input
									placeholder="John Doe"
									bind:value={profile.fullname}
									type="text"
									class="border placeholder-gray-400 focus:outline-none
                    focus:border-black w-full pt-4 pr-4 pb-4 pl-4 mt-2 mr-0 mb-0 ml-0 text-base block bg-white
                    border-gray-300 rounded-md"
								/>
								{#if emptyName}
									<div
										class="absolute right-0 top-0 w-[40%] p-1 rounded-sm border bg-red-500 border-red-500 flex justify-center"
									>
										<p class="text-[10px] font-bold italic text-white">*Can't be empty</p>
									</div>
								{/if}
							</div>
							<div class="relative">
								<p
									class="bg-white pt-0 pr-2 pb-0 pl-2 -mt-3 mr-0 mb-0 ml-2 font-medium text-gray-600 absolute"
								>
									Email
								</p>
								<input
									placeholder="123@ex.com"
									bind:value={profile.email}
									on:keyup={update_username_byemail}
									type="text"
									class="border placeholder-gray-400 focus:outline-none
                    focus:border-black w-full pt-4 pr-4 pb-4 pl-4 mt-2 mr-0 mb-0 ml-0 text-base block bg-white
                    border-gray-300 rounded-md"
								/>
								{#if emailOrUserNameTaken}
									<div
										class="absolute right-0 top-0 w-[40%] p-1 rounded-sm border bg-red-500 border-red-500 flex justify-center"
									>
										<p class="text-[10px] font-bold italic text-white">*Email or username taken</p>
									</div>
								{/if}
								{#if invalidEmail}
									<div
										class="absolute right-0 top-0 w-[40%] p-1 rounded-sm border bg-red-500 border-red-500 flex justify-center"
									>
										<p class="text-[10px] font-bold italic text-white">*Email format incorrect</p>
									</div>
								{/if}
							</div>
							<div class="relative">
								<p
									class="bg-white pt-0 pr-2 pb-0 pl-2 -mt-3 mr-0 mb-0 ml-2 font-medium text-gray-600
                    absolute"
								>
									User name
								</p>
								<input
									placeholder="John Doe"
									bind:value={profile.username}
									type="text"
									class="border placeholder-gray-400 focus:outline-none
                    focus:border-black w-full pt-4 pr-4 pb-4 pl-4 mt-2 mr-0 mb-0 ml-0 text-base block bg-white
                    border-gray-300 rounded-md"
								/>
								{#if emailOrUserNameTaken}
									<div
										class="absolute right-0 top-0 w-[40%] p-1 rounded-sm border bg-red-500 border-red-500 flex justify-center"
									>
										<p class="text-[10px] font-bold italic text-white">*Email or username taken</p>
									</div>
								{/if}
								{#if emptyUserName}
									<div
										class="absolute right-0 top-0 w-[40%] p-1 rounded-sm border bg-red-500 border-red-500 flex justify-center"
									>
										<p class="text-[10px] font-bold italic text-white">*Can't be empty</p>
									</div>
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
									bind:value={profile.password}
									type="password"
									class="border placeholder-gray-400 focus:outline-none
                    focus:border-black w-full pt-4 pr-4 pb-4 pl-4 mt-2 mr-0 mb-0 ml-0 text-base block bg-white
                    border-gray-300 rounded-md"
								/>
								{#if invalidPassword}
									<div
										class="absolute right-0 top-0 w-[45%] p-1 rounded-sm border bg-red-500 border-red-500 flex justify-center"
									>
										<p class="text-[10px] font-bold italic text-white">
											*Should use a strong password
										</p>
									</div>
								{/if}
							</div>
							<div class="relative">
								<p>
									Already have acount? <a
										class=" text-indigo-500 hover:text-indigo-600 ease font-bold"
										href={`/signin${data.prev_url}`}
										alt="registration link">Signin</a
									> here!
								</p>
							</div>
							<div class="relative">
								<button
									class="w-full inline-block pt-4 pr-5 pb-4 pl-5 text-xl font-medium text-center text-white bg-indigo-500
                    rounded-lg transition duration-200 hover:bg-indigo-600 ease"
									on:click={submit_profile}>Register Now</button
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
{/if}
