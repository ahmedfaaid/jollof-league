/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
			fontFamily: {
				notosans: ['Noto Sans', 'sans-serif']
			}
		}
	},
	plugins: [require('@tailwindcss/forms')]
};
