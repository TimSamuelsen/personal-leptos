/** @type {import('tailwindcss').Config} */
module.exports = {
  content: { 
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      fontFamily: {
        'source-code-pro': ['Source Code Pro', 'monospace'],  // Custom font definition
      },
    },
  },
  plugins: [],
}