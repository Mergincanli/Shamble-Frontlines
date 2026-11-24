import { defineConfig } from "@playwright/test";

// Two projects. `node` runs the browser-free tests in tests/node/: it is what the package's `test` script runs, in
// CI too, so it starts no server and needs no build. `chromium` runs the browser tests in the rest of tests/ (one
// folder per owner: Part 15.2 of the build plan) on a built page, which SHAMBLE_DIST names:
//
//   SHAMBLE_DIST="$PWD/packages/client/dist" SHAMBLE_CHROMIUM=/opt/pw-browsers/chromium \
//     node node_modules/@playwright/test/cli.js test -c packages/e2e/playwright.config.ts --project=chromium
const dist = process.env["SHAMBLE_DIST"];
const browser = process.env["SHAMBLE_CHROMIUM"];
export const ADDRESS = "http://127.0.0.1:4173";

export default defineConfig({
  retries: 0,
  forbidOnly: process.env["CI"] !== undefined,
  use: { trace: "off" },
  projects: [
    { name: "node", testDir: "./tests/node" },
    {
      name: "chromium",
      testDir: "./tests",
      testIgnore: "node/**",
      // One browser at a time: the checks that depend on the machine's speed were tuned with no second browser
      // beside them. And a test's own limit above the 60 s wait in report.ts, so that the wait's bound is the real one.
      workers: 1,
      timeout: 90_000,
      use: { baseURL: ADDRESS, launchOptions: browser === undefined ? {} : { executablePath: browser } },
    },
  ],
  // The zip's small server, from the built page's own folder; only when a build is named.
  ...(dist === undefined ? {} : { webServer: { command: "node serve.mjs", cwd: dist, url: ADDRESS, reuseExistingServer: false } }),
});
