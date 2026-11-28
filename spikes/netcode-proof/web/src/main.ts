// main.ts: the page (u7-brief.md rev 1.1 Block T3; C1). It reads its link, starts the module Worker, posts the link's
// fields, and draws the status, #version and a counters table, all by textContent. For U8's save() it keeps
// window.nptRecords (at most 4,096 npt-log lines), window.nptDropped and window.nptClockResNs, read-only.
// A malformed link starts nothing and says why, never echoing the token.
import * as hooks from "./hooks.js";
import { RECORDS_MAX, STATUS, countCsp, forgetFragment, parseLink, versionText, type Link } from "./lib.js";
import type { StartMsg, ToPage } from "./worker.js";

/** The bites dist's hooks.js lists the bite ids its `bite` query may arm; the measured one lists none. */
export const BITE_IDS: readonly string[] = (hooks as unknown as { BITE_IDS?: readonly string[] }).BITE_IDS ?? [];
const COUNTERS_MS = 1000;

/** What the Worker tells the page, kept for the draw and for U8's reads. */
export class PageState {
  readonly records: string[] = [];
  dropped = 0;
  clockResNs = 0;
  status: string = STATUS.starting;
  counters: readonly (readonly [string, number])[] = [];
  /** The page's own `securitypolicyviolation` events, added to the Worker's `page_csp_violations` (U7R-NC-7). */
  csp = 0;
  rows(): readonly (readonly [string, number])[] {
    return this.counters.map(([n, v]) => [n, n === "page_csp_violations" ? v + this.csp : v] as const);
  }
  apply(m: ToPage): void {
    if (m.kind === "status") this.status = m.text;
    else if (m.kind === "clock") this.clockResNs = m.resNs;
    else if (m.kind === "counters") this.counters = m.names.map((n, i) => [n, m.values[i] ?? 0] as const);
    else m.lines.forEach((l) => (this.records.length < RECORDS_MAX ? this.records.push(l) : this.dropped++));
  }
}

export function startMessage(link: Link, host: string): StartMsg {
  return { kind: "start", link, host, wtTimeoutMs: hooks.WT_TIMEOUT_MS, mdsPeriodMs: hooks.MDS_READ_PERIOD_MS };
}

function setText(id: string, text: string): void {
  const e = document.getElementById(id);
  if (e !== null) e.textContent = text;
}

function drawCounters(rows: PageState["counters"]): void {
  const body = document.querySelector("#counters tbody");
  if (body === null) return;
  body.replaceChildren(...rows.map(([name, value]) => {
    const tr = document.createElement("tr");
    const th = document.createElement("th");
    const td = document.createElement("td");
    th.textContent = name;
    td.textContent = String(value);
    tr.append(th, td);
    return tr;
  }));
}

if (typeof document !== "undefined" && typeof Worker !== "undefined") {
  const state = new PageState();
  countCsp(document, () => state.csp++);
  const expose = (name: string, get: () => unknown): void => void Object.defineProperty(window, name, { get, enumerable: true });
  expose("nptRecords", () => state.records);
  expose("nptDropped", () => state.dropped);
  expose("nptClockResNs", () => state.clockResNs);
  setText("version", versionText(navigator as unknown as Parameters<typeof versionText>[0]));
  const link = parseLink(location.search, location.hash, BITE_IDS);
  if (link.ok) {
    forgetFragment(location, history);
    const w = new Worker(new URL("./worker.js", import.meta.url), { type: "module" });
    w.onmessage = (e: MessageEvent<ToPage>): void => {
      state.apply(e.data);
      setText("status", state.status);
    };
    w.onerror = (): void => setText("status", STATUS.ended("the Worker failed"));
    setText("status", STATUS.starting);
    w.postMessage(startMessage(link.value, location.hostname));
    setInterval(() => drawCounters(state.rows()), COUNTERS_MS);
  } else {
    setText("status", STATUS.badLink(link.why));
  }
}
