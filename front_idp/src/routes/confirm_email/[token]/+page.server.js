import axios from 'axios';
import { backend_base_url } from '$lib/stores';
import { redirect } from '@sveltejs/kit';

/** @type {import('./$types').PageLoad} */
export async function load({ params }) {
	let base_url;
	backend_base_url.subscribe((url) => {
		base_url = url;
	});
	let data;
	await axios
		.get(base_url + 'useractivation/' + params.token)
		.then((resp) => {
			if (resp.status === 200) {
				//console.log(resp.data);
				data = {
					confirmed: true
				};
			} else {
				throw redirect(308, '/');
			}
		})
		// eslint-disable-next-line no-unused-vars
		.catch((_err) => {
			throw redirect(308, '/');
		});

	return data;
}
