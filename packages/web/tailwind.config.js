module.exports = {
  content: ['./index.html', './src/**/*.{ts,tsx}'],
  theme: {
    fontFamily: {
      sans: ['Roboto FlexVariable', 'system-ui'],
      serif: ['Roboto FlexVariable', 'system-ui'],
      display: ['Fredoka', 'system-ui'],
    },
    extend: {
      gridTemplateRows: {
        header: 'auto auto',
      },
      gridTemplateColumns: {
        header: 'auto auto',
      },
    },
  },
  plugins: [require('@tailwindcss/typography')],
};
