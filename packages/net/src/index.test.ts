import { expect, test } from "vitest";
import { PACKAGE } from "./index.ts";

test("the stub names its package", () => {
  expect(PACKAGE).toBe("@shamble/net");
});

// Planted failure M1b (reverted in the next commit): a test that needs the network must fail inside the cut.
test("planted M1b: this test needs the network", async () => {
  const res = await fetch("https://github.com/");
  expect(res.ok).toBe(true);
});
