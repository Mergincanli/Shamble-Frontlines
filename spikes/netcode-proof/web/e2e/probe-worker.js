// probe-worker.js: the console probe's module Worker (probe.js): a marker line, then one connect the CSP refuses,
// counted by the dist's countCsp on this Worker's scope (as worker.ts counts), and a report of what happened. The
// target is plain http on loopback port 9: not 'self' (another port), not in serve.mjs's connect-src list, and on
// loopback, so even an unrefused connect stays on this machine.
import { countCsp } from "./lib.js";

let count = 0;
let violation = "none";
countCsp(self, () => count++);
self.addEventListener("securitypolicyviolation", (e) => {
  violation = e.effectiveDirective || e.violatedDirective || "unnamed";
});
console.log("npt-probe: worker console");
const t0 = performance.now();
let outcome = "fetched";
try {
  await fetch("http://127.0.0.1:9/npt-probe", { cache: "no-store" });
} catch {
  outcome = "refused";
}
const ms = Math.round(performance.now() - t0);
await new Promise((ok) => setTimeout(ok, 100));
postMessage({ count, text: `worker fetch ${outcome} in ${ms} ms; violation ${violation}` });
