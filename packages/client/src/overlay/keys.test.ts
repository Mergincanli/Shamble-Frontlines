import { expect, test } from "vitest";
import { FLASH_MS, KeyLights } from "./keys.ts";

test("a box is lit while the echo holds its button, and for a moment after an echoed press", () => {
  const lights = new KeyLights();
  expect(lights.lit(0, 0)).toBe(0);
  // Held in the echo: lit as long as it is held, and dark at once when it is not.
  expect(lights.lit(0b101, 1000)).toBe(0b101);
  expect(lights.lit(0, 1001)).toBe(0);
  // A tap: the echo shows a press of buttons 1 and 15, and nothing held.
  lights.pressed(0x8002, 2000);
  expect(lights.lit(0, 2000)).toBe(0x8002);
  expect(lights.lit(0, 2000 + FLASH_MS - 1)).toBe(0x8002);
  expect(lights.lit(0, 2000 + FLASH_MS)).toBe(0);
  // A second press before the light ends keeps it lit from then; other buttons are not touched.
  lights.pressed(0b10, 3000);
  lights.pressed(0b10, 3100);
  lights.pressed(0, 3200);
  expect(lights.lit(0b1, 3100 + FLASH_MS - 1)).toBe(0b11);
  expect(lights.lit(0b1, 3100 + FLASH_MS)).toBe(0b1);
});
