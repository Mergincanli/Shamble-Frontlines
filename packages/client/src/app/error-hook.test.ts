import { expect, test } from "vitest";
import { installErrorHook } from "./error-hook.ts";

/** A stand-in for the window: it keeps the listeners, and a test fires them. */
function setUp() {
  const listeners = new Map<string, (event: unknown) => void>();
  const target = { addEventListener: (type: string, listener: (event: unknown) => void) => void listeners.set(type, listener) };
  const hook = installErrorHook(target as unknown as Window);
  const fire = (type: string, event: unknown): void => listeners.get(type)?.(event);
  return { hook, fire, types: () => [...listeners.keys()].sort() };
}

test("it counts the three kinds of error a page does not handle, and what other code notes", () => {
  const { hook, fire, types } = setUp();
  expect(types()).toEqual(["error", "securitypolicyviolation", "unhandledrejection"]);
  expect(hook.log).toEqual({ count: 0, last: "" });
  fire("error", { message: "boom" });
  expect(hook.log).toEqual({ count: 1, last: "boom" });
  fire("securitypolicyviolation", { violatedDirective: "script-src", blockedURI: "https://elsewhere.example/x.js" });
  expect(hook.log).toEqual({ count: 2, last: "refused by the page's policy: script-src https://elsewhere.example/x.js" });
  fire("unhandledrejection", { reason: new Error("nobody caught this") });
  expect(hook.log).toEqual({ count: 3, last: "nobody caught this" });
  fire("unhandledrejection", { reason: 42 });
  expect(hook.log).toEqual({ count: 4, last: "42" });
  hook.note("the Worker failed");
  expect(hook.log).toEqual({ count: 5, last: "the Worker failed" });
});

test("the newest error's text is kept as one line of bounded length", () => {
  const { hook } = setUp();
  // A trap's text has a line break after its place; a tab, a carriage return, an escape and a delete are no better.
  hook.note("panicked at crates/sim/src/world.rs:42:14:\nattempt to add with overflow");
  expect(hook.log.last).toBe("panicked at crates/sim/src/world.rs:42:14: attempt to add with overflow");
  hook.note("a\tb\r\nc\u0000d\u001b[31me\u007ff");
  expect(hook.log.last).toBe("a b c d [31me f");
  // A run of control characters becomes one space, so a report line cannot be padded out of sight.
  hook.note(`first${"\n".repeat(50)}second`);
  expect(hook.log.last).toBe("first second");
  hook.note("x".repeat(1000));
  expect(hook.log.last).toBe("x".repeat(300));
  // Whatever is passed becomes text first.
  hook.note(undefined as unknown as string);
  expect(hook.log.last).toBe("undefined");
  expect(hook.log.count).toBe(5);
});
