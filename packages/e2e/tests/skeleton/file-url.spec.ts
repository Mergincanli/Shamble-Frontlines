// What a browser does with the built page opened as a file, by double-click (OQ-0002-1's first result, here for
// Chromium; Brave on Mustafa's PC is the one that counts). Recorded: either the page runs, or its plain line says
// how to serve it.
import path from "node:path";
import { pathToFileURL } from "node:url";
import { expect, test } from "@playwright/test";
import { needsBuild } from "./report.ts";

test("the built page opened as a file either runs or says how to serve it", async ({ page }, info) => {
  const messages: string[] = [];
  page.on("console", (message) => messages.push(message.text()));
  await page.goto(pathToFileURL(path.join(needsBuild(), "index.html")).href);
  await page.waitForTimeout(3000);
  const ran = (await page.locator("html").getAttribute("data-backend")) !== null;
  const notice = page.locator("#notice");
  const result = ran ? "the page ran from a file" : `the page did not run from a file; the browser said: ${messages[0] ?? "(nothing)"}`;
  info.annotations.push({ type: "opened as a file", description: result });
  console.log(`opened as a file: ${result}`);
  if (ran) await expect(notice).toBeHidden();
  else await expect(notice).toContainText('run "node serve.mjs"');
});
