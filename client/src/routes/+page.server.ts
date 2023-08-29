import { VITE_API_URL } from '$env/static/private';
import { fail, redirect } from '@sveltejs/kit';
import { validateEmail, validatePhoneNumber } from '../utils/fns';
import type { Actions } from './$types';

const apiUrl = VITE_API_URL;

export const actions = {
	default: async ({ request }) => {
		const formData = await request.formData();
		const first_name = formData.get('first_name');
		const last_name = formData.get('last_name');
		const email = formData.get('email');
		const phone_number = formData.get('phone_number');

		if (!first_name || first_name.length < 2) {
			return fail(400, {
				error: true,
				field: 'first_name',
				message: 'Please enter a valid first name'
			});
		}

		if (!last_name || last_name.length < 2) {
			return fail(400, {
				error: true,
				field: 'last_name',
				message: 'Please enter a valid last name'
			});
		}

		if (!email || !validateEmail(email as string)) {
			return fail(400, {
				error: true,
				field: 'email',
				message: 'Please enter a valid email address'
			});
		}

		if (!phone_number || !validatePhoneNumber(phone_number as string)) {
			return fail(400, {
				error: true,
				field: 'phone_number',
				message: 'Please enter a  valid phone number'
			});
		}

		const req = await fetch(`${apiUrl}/user`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({ first_name, last_name, email, phone_number })
		});

		const res = await req.json();

		if (!res || res.error) {
			return fail(400, {
				error: true,
				field: null,
				message: 'There was an error. Please try again or contact me for assistance'
			});
		}

		throw redirect(301, '/success');
	}
} satisfies Actions;
