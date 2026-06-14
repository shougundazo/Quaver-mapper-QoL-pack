import { rm, mkdir, writeFile } from "node:fs/promises";
import { build } from "esbuild";

await rm("dist", { recursive: true, force: true });
await mkdir("dist/assets", { recursive: true });

await build({
  entryPoints: ["src/main.tsx"],
  bundle: true,
  outfile: "dist/assets/index.js",
  format: "esm",
  platform: "browser",
  target: ["chrome111", "edge111", "firefox114", "safari16"],
  sourcemap: false,
  minify: false,
  loader: {
    ".css": "css",
  },
  define: {
    "process.env.NODE_ENV": '"production"',
  },
});

await writeFile(
  "dist/index.html",
  `<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Quaver Mapper QoL Pack</title>
    <script type="module" crossorigin src="/assets/index.js"></script>
    <link rel="stylesheet" crossorigin href="/assets/index.css" />
  </head>
  <body>
    <div id="root"></div>
  </body>
</html>
`,
);
