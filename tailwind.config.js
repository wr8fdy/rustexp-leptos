/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "class",
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {},
    container: {
      center: true,
    },
  },
  plugins: [],
};
