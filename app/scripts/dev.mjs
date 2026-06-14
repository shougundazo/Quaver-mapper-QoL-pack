import "./build.mjs";
import { context } from "esbuild";

const ctx = await context({
  entryPoints: ["src/main.tsx"],
  bundle: true,
  outfile: "dist/assets/index.js",
  format: "esm",
  platform: "browser",
  target: ["chrome111", "edge111", "firefox114", "safari16"],
  sourcemap: true,
  minify: false,
  loader: {
    ".css": "css",
  },
  define: {
    "process.env.NODE_ENV": '"development"',
  },
});

await ctx.watch();
const { host, port } = await ctx.serve({
  servedir: "dist",
  host: "127.0.0.1",
  port: 1420,
});

console.log(`dev server listening on http://${host}:${port}`);
