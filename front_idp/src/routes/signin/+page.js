/** @type {import('./$types').PageLoad} */
export async function load({ url }) {
	const next_hub = url.searchParams.get('next');
	const data = { prev_url: next_hub ? next_hub : url.href };
	console.log('Signin page data:', url.searchParams.get('next'));
	return data;
}
