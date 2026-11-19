// A fact the later tests need (R5-TEST-OQ1; PM-0002-30): does Playwright's clock reach the Worker? Recorded, not
// judged: the answer decides how later tests drive the tick, and either answer is a pass.
import { expect, test } from "@playwright/test";
import { needsBuild, report, type Report } from "./report.ts";

test("whether Playwright's clock reaches the Worker is recorded", async ({ page }, info) => {
  needsBuild();
  await page.clock.install();
  await page.goto("/");
  // The page's own timers and frames now stand still; blocks arrive as messages, which no clock holds back.
  // The first block, not only the report's hook: the hook stands before the Worker has loaded the Wasm.
  await page.waitForFunction(() => ((window as unknown as { __shambleReport?(): Report }).__shambleReport?.().tick ?? 0) >= 1);
  const start = (await report(page)).tick;
  await page.waitForTimeout(2000);
  const afterRealTime = (await report(page)).tick;
  await page.clock.fastForward(60_000);
  const afterFakeMinute = (await report(page)).tick;
  const ranOnRealTime = afterRealTime - start >= 30;
  const jumpedWithFakeTime = afterFakeMinute - afterRealTime >= 900;
  const fact = jumpedWithFakeTime
    ? "yes: the Worker's ticks followed the page's fake clock"
    : ranOnRealTime
      ? "no: the Worker kept ticking on real time while the page's clock stood still"
      : "unclear: the Worker ticked on neither clock";
  info.annotations.push({ type: "Playwright's clock reaches the Worker", description: `${fact} (ticks ${start}, ${afterRealTime}, ${afterFakeMinute})` });
  console.log(`Playwright's clock reaches the Worker: ${fact} (ticks ${start}, ${afterRealTime}, ${afterFakeMinute})`);
  expect(fact).not.toContain("unclear");
});
