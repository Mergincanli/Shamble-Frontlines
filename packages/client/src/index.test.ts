import { expect, test } from "vitest";
import { PACKAGE } from "./index.ts";

test("the stub names its package", () => {
  expect(PACKAGE).toBe("@shamble/client");
});
