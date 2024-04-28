const fs = require('fs');

/** @type {import('tailwindcss').Config} */
module.exports = {
  // configure the paths to all of your source files
  content: { files: [
    './**/*.{html,md,rs,yaml}',
  ]},
  theme: {
    fontFamily: {
      sans: ['Prompt', 'sans-serif'],
      serif: ['Lustria', 'Yrsa', 'serif'],
      mono: ['Hack', 'monospace'],
    },
    extend: {
      // extend base Tailwind CSS utility classes
    },
  },

  // add plugins to your Tailwind CSS project
  plugins: [
    require('tailwind-typewriter')({
        wordsets: {
            aboutme: {
              words: [fs.readFileSync('res/content/aboutme.md', 'utf8')],
              writeSpeed: 0.05,
              pauseBetween: 1,
              repeat: 0,
              eraseSpeed: 0,
              caretWidth: '3px'
            }
        }
    })
  ],
}
