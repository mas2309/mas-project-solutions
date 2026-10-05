// Service worker de MAS Finance.
// - Páginas: siempre desde la red (los datos financieros deben estar al día);
//   si no hay conexión se muestra la página offline.
// - /static/: responde desde caché y la actualiza en segundo plano
//   (los cambios de tema/íconos llegan en la siguiente carga).
// - API y peticiones que no son GET: no se interceptan.
// Al cambiar archivos precacheados, subir la versión de CACHE_NAME.

const CACHE_NAME = 'mas-finance-v2';
const OFFLINE_URL = '/static/offline.html';
const PRECACHE = [
  OFFLINE_URL,
  '/static/manifest.webmanifest',
  '/static/tailwind-config.js',
  '/static/icons/icon-192.png',
  '/static/icons/icon-512.png',
  '/static/icons/favicon.svg',
  '/static/icons/favicon-32.png',
];

self.addEventListener('install', (event) => {
  event.waitUntil(
    caches.open(CACHE_NAME)
      .then((cache) => cache.addAll(PRECACHE))
      .then(() => self.skipWaiting())
  );
});

self.addEventListener('activate', (event) => {
  event.waitUntil(
    caches.keys()
      .then((keys) => Promise.all(keys.filter((k) => k !== CACHE_NAME).map((k) => caches.delete(k))))
      .then(() => self.clients.claim())
  );
});

self.addEventListener('fetch', (event) => {
  const request = event.request;
  if (request.method !== 'GET') return;

  const url = new URL(request.url);
  if (url.origin !== self.location.origin) return;

  if (request.mode === 'navigate') {
    event.respondWith(
      fetch(request).catch(() => caches.match(OFFLINE_URL))
    );
    return;
  }

  if (url.pathname.startsWith('/static/')) {
    event.respondWith(
      caches.open(CACHE_NAME).then((cache) =>
        cache.match(request).then((cached) => {
          const network = fetch(request).then((response) => {
            if (response.ok) cache.put(request, response.clone());
            return response;
          });
          if (cached) {
            event.waitUntil(network.catch(() => {}));
            return cached;
          }
          return network;
        })
      )
    );
  }
});
