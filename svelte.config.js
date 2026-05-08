import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

export default {
  preprocess: vitePreprocess(), // ← esto es lo que faltaba
  compilerOptions: {
    dev: false,
    customElement: false,
  },
  vitePlugin: {
    inspector: true,
  },
};
