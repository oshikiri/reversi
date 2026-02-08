import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";

export default defineConfig(() => ({
  base: "./",
  plugins: [wasm()],
  define: {
    "process.env.REVERSI_VERSION": JSON.stringify(
      process.env.REVERSI_VERSION ?? "",
    ),
  },
}));
