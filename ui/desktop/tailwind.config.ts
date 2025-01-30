/** @type {import('tailwindcss').Config} */
export default {
  darkMode: ['class'],
  content: ['./src/**/*.{js,jsx,ts,tsx}', './index.html'],
  plugins: [require('tailwindcss-animate'), require('@tailwindcss/typography')],
  theme: {
    extend: {
      fontFamily: {
        sans: ['Cash Sans'],
      },
      keyframes: {
        shimmer: {
          '0%': { backgroundPosition: '200% 0' },
          '100%': { backgroundPosition: '-200% 0' },
        },
        loader: {
          '0%': { left: 0, width: 0 },
          '50%': { left: 0, width: '100%' },
          '100%': { left: '100%', width: 0 },
        },
        popin: {
          from: { opacity: 0, transform: 'scale(0.95)' },
          to: { opacity: 1, transform: 'scale(1)' },
        },
        fadein: {
          '0%': { opacity: 0 },
          '100%': { opacity: 1 },
        },
        appear: {
          '0%': { opacity: 0, transform: 'translateY(12px)' },
          '100%': { opacity: 1, transform: 'translateY(0)' },
        },
        flyin: {
          '0%': { opacity: 0, transform: 'translate(-300%, 300%)' },
          '100%': { opacity: 1, transform: 'translate(0, 0)' },
        },
        wind: {
          '0%': { transform: 'translate(0, 0)' },
          '99.99%': { transform: 'translate(-100%, 100%)' },
          '100%': { transform: 'translate(0, 0)' },
        },
        rotate: {
          '0%': { transform: 'rotate(0deg)' },
          '100%': { transform: 'rotate(360deg)' },
        },
      },
      animation: {
        'shimmer-pulse': 'shimmer 4s ease-in-out infinite',
        'gradient-loader': 'loader 750ms ease-in-out infinite',
      },
      colors: {
        bgApp: 'var(--background-app)',
        bgSubtle: 'var(--background-subtle)',
        bgStandard: 'var(--background-standard)',
        bgProminent: 'var(--background-prominent)',

        borderSubtle: 'var(--border-subtle)',
        borderStandard: 'var(--border-standard)',

        textProminent: 'var(--text-prominent)',
        textStandard: 'var(--text-standard)',
        textSubtle: 'var(--text-subtle)',
        textPlaceholder: 'var(--text-placeholder)',

        iconProminent: 'var(--icon-prominent)',
        iconStandard: 'var(--icon-standard)',
        iconSubtle: 'var(--icon-subtle)',
        iconExtraSubtle: 'var(--icon-extra-subtle)',
        slate: 'var(--slate)',
        blockTeal: 'var(--block-teal)',
        blockOrange: 'var(--block-orange)',
      },
    },
  },
};
