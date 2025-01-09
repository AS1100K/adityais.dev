/** @type {import('tailwindcss').Config} */
export default {
	content: [
		"./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}",
		"./adityais.dev-content/**/*.{md, mdx}",
	],
	theme: {
		extend: {
			borderWidth: {
				1: "1px",
			},
			width: {
				featured_project: "calc(100% - 2rem)",
			},
			minHeight: {
				full_page: "calc(100vh - 20rem)",
			},
			colors: {
				primary: "#56ba63",
				secondary: "#9cdda4",
				accent: "#6fd67d",
				background: "#f9fcf9",
				text: "#09100a",
			},
			fontFamily: {
				sans: ["Poppins", "sans-serif"],
			},
			fontWeight: {
				black: 900,
				extrabold: 800,
				bold: 700,
				semibold: 600,
				medium: 500,
				regular: 400,
			},
		},
	},
	plugins: [require("@tailwindcss/typography")],
};
