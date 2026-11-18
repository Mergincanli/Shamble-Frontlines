import { expect, test } from "vitest";
import { isPageToWorker, MODE_LOCAL, type PageToWorker } from "./messages.ts";

test("local play is mode 0", () => {
  expect(MODE_LOCAL).toBe(0);
});

test("every message the page may send is accepted", () => {
  const good: PageToWorker[] = [
    { type: "start", padBytes: 0 },
    { type: "start", padBytes: 65536 },
    { type: "input", sequence: 0, viewTime: 0, returned: [] },
    { type: "input", sequence: 0xffff_ffff, viewTime: 1234.5, returned: [new ArrayBuffer(8), new ArrayBuffer(8)] },
    { type: "visibility", hidden: true },
    { type: "visibility", hidden: false },
    { type: "report-request" },
  ];
  for (const message of good) expect(isPageToWorker(message), JSON.stringify(message)).toBe(true);
});

test("anything else is refused", () => {
  const bad: unknown[] = [
    null,
    undefined,
    "start",
    42,
    [],
    {},
    { type: "stop" },
    { type: "start" },
    { type: "start", padBytes: -1 },
    { type: "start", padBytes: 1.5 },
    { type: "start", padBytes: 0x1_0000_0000 },
    { type: "start", padBytes: "8" },
    { type: "input", sequence: 1, viewTime: 0 },
    { type: "input", sequence: -1, viewTime: 0, returned: [] },
    { type: "input", sequence: 1, viewTime: Number.NaN, returned: [] },
    { type: "input", sequence: 1, viewTime: Number.POSITIVE_INFINITY, returned: [] },
    { type: "input", sequence: 1, viewTime: 0, returned: [new Uint8Array(8)] },
    { type: "input", sequence: 1, viewTime: 0, returned: "none" },
    { type: "visibility" },
    { type: "visibility", hidden: "yes" },
  ];
  for (const message of bad) expect(isPageToWorker(message), JSON.stringify(message)).toBe(false);
});
