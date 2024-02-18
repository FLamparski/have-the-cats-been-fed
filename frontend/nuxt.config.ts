// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },
  css: ["~/assets/styles/global.scss"],
  modules: ["@hebilicious/vue-query-nuxt"],
  routeRules: {
    '/backend/**': { proxy: 'http://backend:8080/**' }
  }
});
