import type { Config } from 'tailwindcss';
import { createThemes } from 'tw-colors';

export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],

	theme: {
		extend: {}
	},

	plugins: [
    createThemes({
      light: {
        'background': '#E5E5E8',
        'primary': '#E3E3E6',
        'secondary': '#FFFFFF',
        'text-primary': '#1C1C1E',
        'text-secondary': '#6B6B6F',
        'accent-primary': '#007AFF',
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
        'background': '#282A36',
        'primary': '#21222C',
        'secondary': '#44475A',
        'text-primary': '#b9c5eb',
        'text-secondary': '#94a7e3',
        'accent-primary': '#BD93F9',
      },
      coffee: {
        'background': '#4A3730',
        'primary': '#2C1810',
        'secondary': '#9C7B68',
        'text-primary': '#F5E6DB',
        'text-secondary': '#CCBAB1',
        'accent-primary': '#D35400', 
      },
      forest: {
        'background': '#2D4F35',
        'primary': '#1B2D20',
        'secondary': '#4A6B50',
        'text-primary': '#E8F1E9',
        'text-secondary': '#B8C7BB',
        'accent-primary': '#8BC34A',
      }
    })
  ]
} satisfies Config;
