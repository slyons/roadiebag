/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["*.html", "./src/**/*.rs",],
    theme: {
        extend: {},
    },
    plugins: [require("@tailwindcss/typography"), require("daisyui")],
    daisyui: {
        themes: ["light", "dark", "cupcake"],
        logs: false,
    },
}

