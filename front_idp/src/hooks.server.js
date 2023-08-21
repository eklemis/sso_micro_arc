import { redirect } from '@sveltejs/kit';
import { backend_base_url } from '$lib/stores';

const LOGIN_REQUIRED_ROUTES = ['/signin', '/signup'];

export async function handle({ event, resolve }) {
	console.log('HOOKS EXECUTE!');
	const cookies = event.cookies;
	const session_id = cookies.get('session_id');
	const auth_code = cookies.get('auth_code');
	const dest_route = event.route?.id?.trim();
	console.log('auth_code:', auth_code);
	const valid_session = await session_valid(session_id);
	if (!valid_session && !LOGIN_REQUIRED_ROUTES.includes(dest_route)) {
		throw redirect(307, '/signin');
	} else if (valid_session && LOGIN_REQUIRED_ROUTES.includes(dest_route)) {
		throw redirect(307, '/');
	}

	const response = await resolve(event);
	return response;
}

const session_valid = async (session_id) => {
	let base_url = '';
	backend_base_url.subscribe((url) => (base_url = url));
	let end_point = base_url + `session_status/${session_id}`;
	const resp = await fetch(end_point);
	resp.text().then((ms) => console.log('Resp. message: ', ms));
	if (resp.status === 200) {
		return true;
	}
	return false;
};
