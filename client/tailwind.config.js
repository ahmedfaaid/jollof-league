/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
			fontFamily: {
				notosans: ['Noto Sans', 'sans-serif']
			},
			colors: {
				'columbia-blue': '#d1dbe4',
				'cadet-blue': '#a3b7ca',
				'weldon-blue': '#7593af',
				'queen-blue': '#476f95',
				'dark-cerulean': '#194a7a'
			}
		}
	},
	plugins: [require('@tailwindcss/forms')]
};
