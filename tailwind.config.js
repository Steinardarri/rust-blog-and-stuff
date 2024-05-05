const fs = require("fs");

/** @type {import('tailwindcss').Config} */
module.exports = {
  // configure the paths to all of your source files
  content: { files: ["./**/*.{html,md,rs,yaml}"] },
  theme: {
    fontFamily: {
      sans: ["Prompt", "sans-serif"],
      serif: ["Lustria", "Yrsa", "serif"],
      mono: ["Hack", "monospace"],
    },
    extend: {
      // extend base Tailwind CSS utility classes
      colors: {
        "nord0":  "#2e3440",
        "nord1":  "#3b4252",
        "nord2":  "#434c5e",
        "nord3":  "#4c566a",
        "nord4":  "#d8dee9",
        "nord5":  "#e5e9f0",
        "nord6":  "#eceff4",
        "nord7":  "#8fbcbb",
        "nord8":  "#88c0d0",
        "nord9":  "#81a1c1",
        "nord10": "#5e81ac",
        "nord11": "#bf616a",
        "nord12": "#d08770",
        "nord13": "#ebcb8b",
        "nord14": "#a3be8c",
        "nord15": "#b48ead"
      }
    },
  },

  // add plugins to your Tailwind CSS project
  plugins: [
    require("tailwind-typewriter")({
      wordsets: {
        aboutme: {
          words: [fs.readFileSync("res/content/aboutme.md", "utf8")],
          writeSpeed: 0.05,
          pauseBetween: 1,
          repeat: 0,
          eraseSpeed: 0,
          caretWidth: "3px",
        },
      },
    }),
  ],
};
