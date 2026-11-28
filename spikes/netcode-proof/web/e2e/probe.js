// probe.js: T5 (ii)'s console probe and the positive control of page_csp_violations (pass mark 7; U7R-NC-7). The page
// and its module Worker each make one connect the CSP refuses (plain http on loopback port 9: not 'self', not in
// serve.mjs's connect-src, and on loopback, so even an unrefused connect stays on this machine), each counted by the
// dist's own countCsp (lib.js) on the target the page uses: the document here, the Worker's scope there. Both counts
// 1 shows the counter sees a refusal on each side; the Worker also logs a marker, to show its console is seen.
import { countCsp } from "./lib.js";

let page = 0;
countCsp(document, () => page++);
const w = new Worker(new URL("./probe-worker.js", import.meta.url), { type: "module" });
w.onmessage = async (e) => {
  try {
    await fetch("http://127.0.0.1:9/npt-probe-page", { cache: "no-store" });
  } catch {
    // refused
  }
  await new Promise((ok) => setTimeout(ok, 100));
  document.getElementById("status").textContent = `Done: page csp ${page}; worker csp ${e.data.count}; ${e.data.text}`;
  window.probeDone = true;
};
