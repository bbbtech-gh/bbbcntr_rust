// See the Tailwind configuration guide for advanced usage
// https://tailwindcss.com/docs/configuration

// const plugin = require("tailwindcss/plugin")
// const fs = require("fs")
// const path = require("path")

module.exports = {
  content: [
    "../src/**",
  ],
  theme: {
    extend: {
      colors: {
        brand: "#FD4F00",
      },
      animation: {
        glotext: 'glotext 5s ease infinite',
        changegrad: 'changegrad 5s infinite',
        slideinabv: 'slideinabv 1.34s ease'
      },
      keyframes: {
        glotext: {
          '0%, 100%': {
            'background-size': '200% 200%',
            'background-position': 'left center',
          },
          '50%': {
            'background-size': '200% 200%',
            'background-position': 'right center',
          },
        },
        changegrad: {
          '0%, 100%': {
            background: 'radial-gradient(circle, #e5653e, #000)',
          },
          '33%': {
            background: 'radial-gradient(circle, #6366f1, #000)',
          },
          '66%': {
            background: 'radial-gradient(circle, #3b82f6, #000)',
          },
        },
      }
    },
  },
  plugins: []
}
