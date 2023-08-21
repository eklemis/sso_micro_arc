import { IDP_BACKEND_BASE_URL, IDP_FRONTEND_BASE_URL } from '$env/static/private';
import { redirect } from '@sveltejs/kit';
// import { prisma_client } from './lib/store';
// import { PrismaClient } from '@prisma/client';
import axios from 'axios';
export async function handle({ event, resolve }) {
	const cookies = event.cookies;
	const sid = cookies.get('session_id');
	const auth_code = event.url.searchParams.get('auth_code');
	if (sid !== null) {
		const backend_endpoint = `${IDP_BACKEND_BASE_URL}session_status/${sid}`;
		const curr_url = event.url.href;
		const redirect_to = `${IDP_FRONTEND_BASE_URL}signin?next=${curr_url}`;
		await axios
			.get(backend_endpoint)
			.then((resp) => {
				//check reponse content
				if (resp.status !== 200) {
					throw redirect(307, redirect_to);
				}
			})
			.catch(() => {
				throw redirect(307, redirect_to);
			});
	} else if (auth_code && auth_code !== null) {
		await fetch(`${IDP_BACKEND_BASE_URL}token/${auth_code}`).then((resp) => {
			//UNIMPLEMENTED
			//TOKEN MANAGEMENT
			//HOW TOKEN IS STORED
			//HOW TOKEN IS REFRESHED
			console.log(resp);
		});
	}
	// } else if (sid) {
	// 	// validate session id
	// 	console.log('Session exist:', sid);
	// 	let prisma = get_prisma();
	// 	const session = await prisma.session.findUnique({
	// 		where: {
	// 			session_id: sid
	// 		}
	// 	});
	// 	console.log('got session:', session);
	// 	if (session !== null) {
	// 		//update last access time
	// 	}
	// }
	const response = await resolve(event);
	return response;
}

// const get_prisma = () => {
// 	let prisma;
// 	prisma_client.subscribe((client) => {
// 		prisma = client;
// 	});
// 	if (!prisma) {
// 		prisma = new PrismaClient();
// 		prisma_client.set(prisma);
// 	}
// 	return prisma;
// };
