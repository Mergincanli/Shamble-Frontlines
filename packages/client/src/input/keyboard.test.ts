import { expect, test } from "vitest";
import { BUTTON_NAMES, KeyboardInput } from "./keyboard.ts";

const words = (keys: KeyboardInput): number[] => [keys.word(0), keys.word(1), keys.word(2), keys.word(3)];

test("each key is its button: held while down, counted once per press, by the key's place and not its letter", () => {
  const keys = new KeyboardInput();
  expect(BUTTON_NAMES).toEqual(["W", "A", "S", "D", "Space", "Shift"]);
  const codes = ["KeyW", "KeyA", "KeyS", "KeyD", "Space", "ShiftLeft"];
  codes.forEach((code, button) => {
    expect(keys.key(code, true, false, 10 + button)).toBe(true);
    expect(keys.held).toBe((2 << button) - 1);
  });
  // Buttons 0 to 3 in the first word, one count per byte; 4 and 5 in the second.
  expect(words(keys)).toEqual([0x0101_0101, 0x0000_0101, 0, 0]);
  expect([keys.presses, keys.releases, keys.changedAt]).toEqual([6, 0, 15]);
  codes.forEach((code) => keys.key(code, false, false, 20));
  expect([keys.held, keys.presses, keys.releases]).toEqual([0, 6, 6]);
  // The counts never go back.
  expect(words(keys)).toEqual([0x0101_0101, 0x0000_0101, 0, 0]);
  // A key that is not the game's is nobody's: nothing changes, and the browser keeps it.
  for (const code of ["KeyQ", "Enter", "", "keyw"]) expect(keys.key(code, true, false, 30)).toBe(false);
  expect([keys.held, keys.presses, keys.changedAt]).toEqual([0, 6, 20]);
});

test("a held key's repeats count nothing, and a tap between two frames is still counted", () => {
  const keys = new KeyboardInput();
  keys.key("Space", true, false, 1);
  for (let i = 0; i < 30; i += 1) expect(keys.key("Space", true, true, 2 + i)).toBe(true);
  expect([keys.held, keys.presses, keys.changedAt]).toEqual([1 << 4, 1, 1]);
  keys.key("Space", false, false, 40);
  // Down and up again before anyone read it: nothing is held, and the count holds the tap.
  keys.key("KeyW", true, false, 41);
  keys.key("KeyW", false, false, 42);
  expect([keys.held, keys.word(0), keys.presses, keys.releases]).toEqual([0, 1, 2, 2]);
  // A key-up without its key-down (the key went down before the page had the focus) is nothing.
  keys.key("KeyA", false, false, 50);
  expect([keys.presses, keys.releases, keys.changedAt]).toEqual([2, 2, 42]);
});

test("either Shift is the one button, held while either is down", () => {
  const keys = new KeyboardInput();
  keys.key("ShiftLeft", true, false, 1);
  keys.key("ShiftRight", true, false, 2);
  expect([keys.held, keys.presses]).toEqual([1 << 5, 1]);
  keys.key("ShiftLeft", false, false, 3);
  expect([keys.held, keys.releases]).toEqual([1 << 5, 0]);
  keys.key("ShiftRight", false, false, 4);
  expect([keys.held, keys.presses, keys.releases, keys.changedAt]).toEqual([0, 1, 1, 4]);
});

test("losing the focus releases every key, and the counts stay", () => {
  const keys = new KeyboardInput();
  keys.key("KeyW", true, false, 1);
  keys.key("KeyD", true, false, 2);
  keys.clear(9);
  expect([keys.held, keys.presses, keys.releases, keys.changedAt]).toEqual([0, 2, 2, 9]);
  expect(keys.word(0)).toBe(0x0100_0001);
  // Cleared twice, or with nothing held, changes nothing; the key-ups that still come are nothing either.
  keys.clear(10);
  keys.key("KeyW", false, false, 11);
  expect([keys.releases, keys.changedAt]).toEqual([2, 9]);
});

test("a button's count wraps at 256", () => {
  const keys = new KeyboardInput();
  for (let tap = 1; tap <= 600; tap += 1) {
    keys.key("KeyA", true, false, tap);
    keys.key("KeyA", false, false, tap);
    expect(keys.word(0)).toBe((tap % 256) << 8);
  }
  expect([keys.presses, keys.releases]).toEqual([600, 600]);
});
