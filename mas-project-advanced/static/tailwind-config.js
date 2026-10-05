// Tema de marca de Mas Finance (compartido por todas las plantillas).
// Paleta: Azul noche #10243B · Verde más #2FCB8F · Verde texto #0B7A52 · Niebla #F6F8F7
// Tipografía: Plus Jakarta Sans
tailwind.config = {
    theme: {
        extend: {
            colors: {
                // Verde de marca. 500 = Verde más (acentos), 600 = Verde texto (botones/enlaces, contraste AA con blanco)
                primary: {
                    50: '#ecfbf4', 100: '#d1f5e5', 200: '#a5ebcd', 300: '#6fdcb0', 400: '#45d29c',
                    500: '#2FCB8F', 600: '#0B7A52', 700: '#096544', 800: '#085237', 900: '#06402b',
                },
                navy: { DEFAULT: '#10243B', light: '#1B3552', dark: '#0A1828' },
                sidebar: { DEFAULT: '#10243B', hover: '#1B3552', active: '#0A1828' },
                niebla: '#F6F8F7',
            },
            fontFamily: {
                sans: ['"Plus Jakarta Sans"', 'ui-sans-serif', 'system-ui', 'sans-serif'],
            },
        },
    },
};
