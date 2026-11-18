import { defineConfig } from "@playwright/test";

// The `node` project runs the browser-free tests in tests/node/. It is what the package's `test` script runs, in CI
// too, so it starts no server and needs no build. The browser tests (the rest of tests/, one folder per owner: Part
// 15.2 of the build plan) get their own project when they arrive.
export default defineConfig({
  retries: 0,
  forbidOnly: process.env["CI"] !== undefined,
  projects: [{ name: "node", testDir: "./tests/node" }],
});
