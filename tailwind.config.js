/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      fontFamily: {
        sans: ['Raleway', '-apple-system', 'BlinkMacSystemFont', 'Segoe UI', 'Roboto', 'Helvetica Neue', 'Arial', 'sans-serif'],
        serif: ['Bitter', 'Georgia', 'Cambria', 'Times New Roman', 'Times', 'serif'],
      },
      colors: {
        'ocean-dark': {
          DEFAULT: '#114b5f',
          600: '#0d3a4a',
        },
        'ocean': {
          50: '#e6f4f6',
          100: '#b3e0e5',
          DEFAULT: '#028090',
          500: '#028090',
          600: '#026d7a',
          700: '#015a64',
        },
        'ocean-mint': {
          DEFAULT: '#e4fde1',
          700: '#a8d6a4',
        },
        'ocean-blue': {
          DEFAULT: '#456990',
          400: '#5a7fa8',
        },
        'ocean-pink': {
          DEFAULT: '#f45b69',
          600: '#e14156',
        },
      },
    },
  },
  plugins: [require("daisyui")],
  daisyui: {
    themes: [
      {
        "archon-ocean": {
          "primary": "#028090",           // Teal
          "primary-content": "#ffffff",   // White text on primary
          "secondary": "#456990",         // Baltic blue
          "secondary-content": "#ffffff", // White text on secondary
          "accent": "#f45b69",            // Bubblegum pink
          "accent-content": "#ffffff",    // White text on accent
          "neutral": "#114b5f",           // Dark teal
          "neutral-content": "#ffffff",   // White text on neutral
          "base-100": "#114b5f",          // Dark teal background
          "base-200": "#028090",          // Teal secondary background
          "base-300": "#456990",          // Baltic blue tertiary background
          "base-content": "#ffffff",      // White text on base
          "info": "#114b5f",              // Dark teal for info
          "info-content": "#ffffff",      // White text on info
          "success": "#e4fde1",           // Frosted mint
          "success-content": "#114b5f",   // Dark teal text on success
          "warning": "#f45b69",           // Bubblegum pink for warning
          "warning-content": "#ffffff",   // White text on warning
          "error": "#f45b69",             // Bubblegum pink for error
          "error-content": "#ffffff",     // White text on error
        },
      },
    ],
  },
}
