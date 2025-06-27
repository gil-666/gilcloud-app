// tailwind.config.js
module.exports = {
    content: [
        './index.html',
        './src/**/*.{vue,js,ts,jsx,tsx}',
    ],
    theme: {
        extend: {
            screens: {
                xl1400: '1400px', // your custom breakpoint
            },
        },
    },
    plugins: [],
}
W