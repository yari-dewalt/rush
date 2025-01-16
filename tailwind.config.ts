import type { Config } from 'tailwindcss';
const { createThemes } = require('tw-colors');

export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],

	theme: {
		extend: {}
	},

	plugins: [
    createThemes({
      light: {
        'primary': 'steelblue',
        'secondary': 'darkblue',
        'brand': '#F3F3F3',
      },
      dark: {
        'background': '#23232B',
        'primary': '#202026',
        'secondary': '#2F2F3D',
        'text-primary': '#E0E0E0',
        'text-secondary': '#A0A0A0',
        'accent-primary': '#7AAACF',
      },
      dracula: {

      },
    })
  ]
} satisfies Config;
