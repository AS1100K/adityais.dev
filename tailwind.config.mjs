/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}"],
  theme: {
    extend: {
      colors: {
        primary: "#56ba63",
        secondary: "#9cdda4",
        accent: "#6fd67d",
        background: "#f9fcf9",
        text: "#09100a",
      },
    },
  },
  plugins: [],
};
