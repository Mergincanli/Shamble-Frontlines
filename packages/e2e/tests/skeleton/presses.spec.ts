// "Presses are never lost or doubled" (ADR 0002's pass mark PM-0002-12), by the mark's own method, in Chromium: a
// seeded shim on the page's posting side delays each post by 0 to 100 ms and sends one in ten twice, so posts reach
// the Worker late, out of order and repeated; 10,000 scripted presses, most of them shorter than a tick; the page's
// own count of presses and releases must equal what the server's echo brought back, exactly.
//
// The second copy of a post goes out only after two newer posts have gone, so an older post really arrives behind
// a newer one whatever the frame rate: on a slow page, posts lie further apart than the mark's 100 ms.
//
// The mark's two bites are planted by hand and by the verifier, not here: a Worker that passes only the held
// buttons on (taps then never reach the server), and a Worker that takes the last post to arrive in place of the
// newest (a late post rolls the counts back).
import { expect, test } from "@playwright/test";
import { needsBuild, report, running } from "./report.ts";

const PRESSES = 10_000;
/** Presses a second over the six keys: 24 a second each, under the 30 a second each that the server takes. */
const RATE = 144;

test.beforeAll(() => void needsBuild());

test("10,000 presses under late, reordered and repeated posts: none lost, none doubled", async ({ page }) => {
  test.setTimeout(240_000);
  // The shim, in the page before its own code runs: every input post waits 0 to 100 ms, and one in ten is sent a
  // second time, after two newer posts have gone and their waits are over (without the buffers, which can ride
  // only once).
  await page.addInitScript(() => {
    let state = 0x2545_f491;
    const random = (): number => {
      state ^= state << 13;
      state ^= state >>> 17;
      state ^= state << 5;
      return (state >>> 0) / 0x1_0000_0000;
    };
    const shim = window as unknown as { __shim: { posts: number; twice: number } };
    shim.__shim = { posts: 0, twice: 0 };
    const copies: { message: object; after: number }[] = [];
    const send = Worker.prototype.postMessage as (this: Worker, message: unknown, transfer: Transferable[]) => void;
    Worker.prototype.postMessage = function (this: Worker, message: unknown, transfer?: unknown): void {
      const list = Array.isArray(transfer) ? (transfer as Transferable[]) : [];
      if ((message as { type?: unknown }).type !== "input") return send.call(this, message, list);
      const posts = (shim.__shim.posts += 1);
      setTimeout(() => send.call(this, message, list), 100 * random());
      while (copies[0] !== undefined && copies[0].after <= posts) {
        const late = copies.shift()?.message;
        shim.__shim.twice += 1;
        setTimeout(() => send.call(this, late, []), 100 + 100 * random());
      }
      if (random() < 0.1) copies.push({ message: { ...(message as object), returned: [] }, after: posts + 2 });
    };
  });
  await page.goto("/");
  await running(page);
  const started = Date.now();
  // The script, in the page: key events at a steady rate, driven by the clock so that a slow timer changes nothing.
  // Four presses in five are taps, down and up in one go (shorter than a frame); the fifth is held until that
  // key's next turn, or the end.
  const made = await page.evaluate(
    async ({ presses, rate }) => {
      const codes = ["KeyW", "KeyA", "KeyS", "KeyD", "Space", "ShiftLeft"];
      const down = new Set<string>();
      const key = (type: string, code: string): void => void window.dispatchEvent(new KeyboardEvent(type, { code }));
      // A number below `bound` from the generator's high bits: its low bits repeat after a few draws.
      let state = 0x9e37_79b9;
      const below = (bound: number): number => {
        state = (Math.imul(state, 1_664_525) + 1_013_904_223) >>> 0;
        return (state >>> 16) % bound;
      };
      const perKey = new Map<string, number>();
      const begun = performance.now();
      let done = 0;
      while (done < presses) {
        await new Promise((resolve) => setTimeout(resolve, 4));
        const due = Math.min(presses, Math.floor(((performance.now() - begun) * rate) / 1000));
        for (; done < due; done += 1) {
          const code = codes[below(codes.length)] ?? "KeyW";
          perKey.set(code, (perKey.get(code) ?? 0) + 1);
          if (down.delete(code)) key("keyup", code);
          key("keydown", code);
          if (below(5) === 0) down.add(code);
          else key("keyup", code);
        }
      }
      for (const code of down) key("keyup", code);
      return [...perKey.values()];
    },
    { presses: PRESSES, rate: RATE },
  );
  // Every one of the six keys had its share: about a sixth each.
  expect(made.reduce((sum, count) => sum + count, 0)).toBe(PRESSES);
  expect(made).toHaveLength(6);
  expect(Math.min(...made)).toBeGreaterThan(PRESSES / 8);
  // The presses waiting their turn are applied, and the keys have been quiet long enough for the last echo. A run
  // in which they never all come back goes on to its readings and fails there, with its numbers.
  const settled = page.waitForFunction(
    (wanted) => {
      const data = (window as unknown as { __shambleReport(): { presses: { echoed: number; lost: number | null } } }).__shambleReport();
      return data.presses.lost !== null && data.presses.echoed >= wanted;
    },
    PRESSES,
    { timeout: 30_000 },
  );
  await settled.catch(() => undefined);
  await page.waitForTimeout(1000);
  const end = await report(page);
  const shim = await page.evaluate(() => (window as unknown as { __shim: { posts: number; twice: number } }).__shim);
  console.log(
    `presses under the shim: ${JSON.stringify(end.presses)}, releases ${JSON.stringify(end.releases)} in ${((Date.now() - started) / 1000).toFixed(1)} s; ` +
      `${shim.posts} posts delayed, ${shim.twice} sent twice; the Worker ignored ${end.worker?.stalePosts} late or repeated posts; ` +
      `messages ${JSON.stringify(end.messages)}; ticks ${end.tick}, drops ${end.worker?.drops}`,
  );
  // Not one press and not one release lost or doubled.
  expect(end.presses).toEqual({ made: PRESSES, echoed: PRESSES, lost: 0 });
  expect(end.releases).toEqual({ made: PRESSES, echoed: PRESSES });
  // The shim did its work: posts came late and twice, each second copy behind two newer posts, and the Worker took
  // the newest by number every time (the last copies may still be on their way).
  expect(shim.twice).toBeGreaterThan(shim.posts / 20);
  expect(end.worker?.stalePosts ?? 0).toBeGreaterThanOrEqual(shim.twice - 2);
  // Nothing was refused on the way: no late post reached the simulation.
  expect(end.messages.refused).toBe(0);
  expect(end.errors).toBe(0);
});
