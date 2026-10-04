// Registro del service worker (habilita instalar MAS Finance como app)
if ('serviceWorker' in navigator) {
  window.addEventListener('load', function () {
    navigator.serviceWorker.register('/sw.js', { scope: '/' }).catch(function (err) {
      console.warn('No se pudo registrar el service worker:', err);
    });
  });
}
