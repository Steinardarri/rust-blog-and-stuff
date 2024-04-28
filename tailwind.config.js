const fs = require('fs');

/** @type {import('tailwindcss').Config} */
module.exports = {
  // configure the paths to all of your source files
  content: { files: [
    'node_modules/preline/dist/*.js',
    './**/*.{html,md,rs,yaml}',
  ]},
  theme: {
    extend: {
      // extend base Tailwind CSS utility classes
    },
  },

  // add plugins to your Tailwind CSS project
  plugins: [
    require('@tailwindcss/forms'),
    require('preline/plugin'),
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
