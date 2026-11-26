import { defineConfig } from "@playwright/test";

// Three projects. `node` runs the browser-free tests in tests/node/: it is what the package's `test` script runs, in
// CI too, so it starts no server and needs no build. `chromium` runs the browser tests in the rest of tests/ (one
// folder per owner: Part 15.2 of the build plan) on a built page, which SHAMBLE_DIST names:
//
//   SHAMBLE_DIST="$PWD/packages/client/dist" SHAMBLE_CHROMIUM=/opt/pw-browsers/chromium \
//     node node_modules/@playwright/test/cli.js test -c packages/e2e/playwright.config.ts --project=chromium
//
// `firefox` runs only the rule tests (tests/rules/) and the recording's replay test (tests/replay/, his FIREFOX-REPLAY
// of 2026-11-26), neither of which draws, in Mozilla's own Firefox build (Mustafa's FIREFOX-SOURCE (c), 2026-11-25):
// on GitHub's runner that Firefox gives the page no WebGL, so the drawing tests in Firefox wait for a follow-up (his
// FIREFOX-GL (a)). Playwright drives it over WebDriver BiDi (`channel: "moz-firefox"`) in a fresh
// profile with Playwright's own test preferences, not a player's. SHAMBLE_FIREFOX names its program, as
// SHAMBLE_CHROMIUM names Chromium's.
const dist = process.env["SHAMBLE_DIST"];
const browser = process.env["SHAMBLE_CHROMIUM"];
const firefox = process.env["SHAMBLE_FIREFOX"];
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
    {
      name: "firefox",
      testDir: "./tests",
      testMatch: ["rules/**/*.spec.ts", "replay/**/*.spec.ts"],
      workers: 1,
      timeout: 90_000,
      use: {
        browserName: "firefox",
        baseURL: ADDRESS,
        launchOptions: firefox === undefined ? { channel: "moz-firefox" } : { channel: "moz-firefox", executablePath: firefox },
      },
    },
  ],
  // The zip's small server, from the built page's own folder; only when a build is named.
  ...(dist === undefined ? {} : { webServer: { command: "node serve.mjs", cwd: dist, url: ADDRESS, reuseExistingServer: false } }),
});
