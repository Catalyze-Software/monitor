import { sveltekit } from "@sveltejs/kit/vite"
import { defineConfig } from "vite"
import inject from "@rollup/plugin-inject"

export default defineConfig({
  plugins: [sveltekit()],

  build: {
    target: "esnext",

    rollupOptions: {
      plugins: [
        inject({
          modules: { Buffer: ["buffer", "Buffer"] },
        }),
      ],
    },
  },

  define: {
    "process.env": JSON.stringify({
      ...process.env
    }),
  },
})
