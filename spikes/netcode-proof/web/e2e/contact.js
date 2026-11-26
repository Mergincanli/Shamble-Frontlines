// contact.js: T5 (i)'s early-contact page (u7-brief.md rev 1.1:129-132), before the Wasm client exists. It uses the
// page's own lib.js and hooks.js (the hash file, the chooser, the room's ports) to open one session and record what
// happened, then waits for the room to close it (its 2 s Join deadline: no Join is ever sent), at most 10 s.
// `?mode=wt&room=NN`: WebTransport with the room's hash, then its control stream. `?mode=ws&room=NN`: wss straight.
// `?mode=wt-wrong&room=NN`: WebTransport with each hash byte XOR 0xff (a control: it must fail; no fallback).
// The result goes to window.contact (and #out) for the driver; it holds no token, URL or bind value.
import { WT_TIMEOUT_MS } from "./hooks.js";
import { connect } from "./lib.js";

const t0 = performance.now();
const ms = () => Math.round(performance.now() - t0);
const q = new URLSearchParams(location.search);
const mode = q.get("mode");
const room = q.get("room") ?? "";
const out = { mode, room, steps: [], statuses: [], violations: [] };
const step = (what, extra = {}) => out.steps.push({ what, ms: ms(), ...extra });
const sleep = (t) => new Promise((ok) => setTimeout(() => ok("timeout"), t));
document.addEventListener("securitypolicyviolation", (e) => {
  out.violations.push({ directive: e.effectiveDirective, blocked: String(e.blockedURI).slice(0, 60) });
});

function done() {
  out.endMs = ms();
  document.getElementById("status").textContent = "Done";
  document.getElementById("out").textContent = JSON.stringify(out, null, 1);
  window.contact = out;
  window.contactDone = true;
}

async function wtSession(wt) {
  step("webtransport ready", { maxDatagramSize: wt.datagrams.maxDatagramSize });
  try {
    const bidi = await wt.createBidirectionalStream();
    step("control stream open");
    const w = bidi.writable.getWriter();
    try {
      await w.write(new Uint8Array(0));
      step("empty write accepted");
    } catch (e) {
      step("empty write refused", { error: e?.name ?? "error" });
    }
  } catch (e) {
    step("control stream refused", { error: e?.name ?? "error" });
  }
  const end = await Promise.race([
    wt.closed.then((v) => ({ how: "closed", closeCode: v?.closeCode ?? null, reason: String(v?.reason ?? "").slice(0, 80) }),
      (e) => ({ how: "closed with an error", error: e?.name ?? "error", source: e?.source ?? null })),
    sleep(10000),
  ]);
  if (end === "timeout") {
    wt.close();
    step("closed by the page at 10 s");
  } else {
    step("webtransport session ended", end);
  }
}

async function wsSession(ws) {
  step("websocket open", { protocol: ws.protocol, extensions: ws.extensions });
  const end = await Promise.race([
    new Promise((ok) => {
      ws.onclose = (e) => ok({ how: "closed", code: e.code, wasClean: e.wasClean, reason: String(e.reason).slice(0, 80) });
      ws.onerror = () => step("websocket error event");
    }),
    sleep(10000),
  ]);
  if (end === "timeout") {
    ws.close();
    step("closed by the page at 10 s");
  } else {
    step("websocket ended", end);
  }
}

if (!["wt", "ws", "wt-wrong"].includes(mode) || !/^(0[0-9]|[12][0-9]|3[01])$/.test(room)) {
  step("a bad contact link");
  done();
} else {
  const link = {
    suite: mode === "wt-wrong" ? "wt-attempts" : "smoke", room, transport: mode === "ws" ? "ws" : "wt", attempt: 0,
    token: "", wrongHash: mode === "wt-wrong", bite: null,
  };
  step("connect starts");
  const chosen = await connect({
    link,
    host: location.hostname,
    timeoutMs: WT_TIMEOUT_MS,
    fetchText: async (path) => {
      const r = await fetch(path, { cache: "no-store", credentials: "omit", redirect: "error" });
      if (!r.ok) throw new Error("not found");
      return r.text();
    },
    webTransport: (url, options) => new WebTransport(url, options),
    webSocket: (url) => new WebSocket(url),
    setTimer: (fn, t) => setTimeout(fn, t),
    clearTimer: (id) => clearTimeout(id),
    onStatus: (s) => out.statuses.push({ text: s, ms: ms() }),
  });
  step(`chosen: ${chosen.kind}`, chosen.kind === "failed" ? { why: chosen.why } : chosen.kind === "ws" ? { fellBack: chosen.fellBack } : {});
  if (chosen.kind === "wt") await wtSession(chosen.wt);
  else if (chosen.kind === "ws") await wsSession(chosen.ws);
  done();
}
