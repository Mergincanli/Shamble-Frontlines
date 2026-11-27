// UM-4's fix pass (advisor-um4-r1.md item 3): each mutant applied to the FIXED tree, its named test run, the file
// restored and its sha256 checked equal to the fix's afterwards. Also V1-V3: each ops helper site restored alone to its
// shipped text, which must turn the victim test red. Run: node /home/user/Shamble-Frontlines/.checkpoints/
// netcode-proof-test/um4-fix-mutants.mjs [id]. It writes only the logs named below (full paths), never exports a git
// variable, and runs `node --test` on check.test.mjs alone, filtered by name.
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";

const SPIKE = "/home/user/Shamble-Frontlines/spikes/netcode-proof";
const CHECK = `${SPIKE}/check.mjs`;
const OPERATOR = `${SPIKE}/ops/tests/operator.test.mjs`;
const REDACT = `${SPIKE}/ops/operator/redact-check.test.mjs`;
const FILTERED = "...Object.fromEntries(Object.entries(process.env).filter(([k]) => !k.startsWith(\"GIT_\"))),";

const T1 = "either part failing fails it";
const WEB = "the web step: npm ci in security's form";
const T2 = "reads the wrappers' summary from stdout";
const T4 = "U8's operator tests by Node's own glob";
const PIN = "both of its calls get the environment without any GIT_ variable";
const VICTIM = "leave the repository GIT_DIR names unchanged";

const MUTANTS = [
  {
    id: "M1", what: "drop opsStep's Node exit guard", file: CHECK, test: T1,
    from: "  if (!n.ok) return failedRun(n, `the ops step's node tests (${nodeCounts(n.out)})`);\n", to: "",
    log: "/home/user/Shamble-Frontlines/.gate/um4-fix/m1.log",
  },
  {
    id: "M3", what: "drop opsStep's wrappers exit guard", file: CHECK, test: T1,
    from: "  if (!w.ok) return failedRun(w, \"the ops step's wrappers test\", n.out);\n", to: "",
    log: "/home/user/Shamble-Frontlines/.gate/um4-fix/m3.log",
  },
  {
    id: "W1", what: "drop webStep's Node exit guard", file: CHECK, test: WEB,
    from: "  if (!n.ok) return failedRun(n, `the web step's node tests (${nodeCounts(n.out)})`, out);\n", to: "",
    log: "/home/user/Shamble-Frontlines/.gate/um4-fix/w1.log",
  },
  {
    id: "M4", what: "read the wrappers' summary from w.out", file: CHECK, test: T2,
    from: "const last = (w.stdout || w.out).trim()", to: "const last = w.out.trim()",
    log: "/home/user/Shamble-Frontlines/.gate/um4-fix/m4.log",
  },
  {
    id: "M10", what: "accept the wrappers' summary on any line", file: CHECK, test: T2,
    from: "const ran = last.match(/^wrappers\\.test\\.sh: (\\d+) ok, 0 failed$/);",
    to: "const ran = (w.stdout || w.out).match(/^wrappers\\.test\\.sh: (\\d+) ok, 0 failed$/m);",
    log: "/home/user/Shamble-Frontlines/.gate/um4-fix/m10.log",
  },
  {
    id: "M8", what: "OPS_BROWSER_MODULES set to ops/browser", file: CHECK, test: T4,
    from: "export const OPS_BROWSER_MODULES = join(HERE, \"ops\", \"browser\", \"node_modules\");",
    to: "export const OPS_BROWSER_MODULES = join(HERE, \"ops\", \"browser\");",
    log: "/home/user/Shamble-Frontlines/.gate/um4-fix/m8.log",
  },
  {
    id: "M-env", what: "opsStep passes base unfiltered", file: CHECK, test: PIN,
    from: "  const env = withoutGitVars(base);\n", to: "  const env = base;\n",
    log: "/home/user/Shamble-Frontlines/.gate/um4-fix/m-env.log",
  },
  {
    id: "M-wild", what: "the filter keeps GIT_DIR", file: CHECK, test: PIN,
    from: "filter(([k]) => !k.startsWith(\"GIT_\")));\n}", to: "filter(([k]) => !k.startsWith(\"GIT_\") || k === \"GIT_DIR\"));\n}",
    log: "/home/user/Shamble-Frontlines/.gate/um4-fix/m-wild.log",
  },
  {
    id: "V1", what: "operator.test.mjs:157 restored alone to its shipped text", file: OPERATOR, test: VICTIM,
    from: "{ cwd: repo, encoding: \"utf8\", env: Object.fromEntries(Object.entries(process.env).filter(([k]) => !k.startsWith(\"GIT_\"))) }",
    to: "{ cwd: repo, encoding: \"utf8\" }",
    log: "/home/user/Shamble-Frontlines/.gate/um4-fix/v1.log",
  },
  {
    id: "V2", what: "redact-check.test.mjs:161 restored alone to its shipped text", file: REDACT, test: VICTIM,
    from: FILTERED, to: "...process.env,", nth: 1, count: 2,
    log: "/home/user/Shamble-Frontlines/.gate/um4-fix/v2.log",
  },
  {
    id: "V3", what: "redact-check.test.mjs:197 restored alone to its shipped text", file: REDACT, test: VICTIM,
    from: FILTERED, to: "...process.env,", nth: 2, count: 2,
    log: "/home/user/Shamble-Frontlines/.gate/um4-fix/v3.log",
  },
];

const sha = (p) => createHash("sha256").update(readFileSync(p)).digest("hex");
const count = (text, s) => text.split(s).length - 1;
function nthIndex(text, s, n) {
  let at = -1;
  for (let i = 0; i < n; i++) {
    at = text.indexOf(s, at + 1);
    if (at < 0) return -1;
  }
  return at;
}
const env = Object.fromEntries(Object.entries(process.env).filter(([k]) => k !== "NODE_TEST_CONTEXT"));
const only = process.argv[2];

for (const m of MUTANTS) {
  if (only && m.id !== only) continue;
  const before = readFileSync(m.file, "utf8");
  const hash = sha(m.file);
  const n = count(before, m.from);
  if (n !== (m.count ?? 1)) throw new Error(`${m.id}: expected ${m.count ?? 1} occurrence(s) in ${m.file}, found ${n}`);
  const at = nthIndex(before, m.from, m.nth ?? 1);
  try {
    writeFileSync(m.file, before.slice(0, at) + m.to + before.slice(at + m.from.length));
    const r = spawnSync(process.execPath, ["--test", `--test-name-pattern=${m.test}`, "check.test.mjs"], { cwd: SPIKE, env, encoding: "utf8", timeout: 600_000 });
    const tests = Number(r.stdout.match(/^# tests (\d+)/m)?.[1] ?? 0);
    const fail = Number(r.stdout.match(/^# fail (\d+)/m)?.[1] ?? 0);
    const verdict = r.status !== 0 && fail > 0 && tests > 0 ? "KILLED" : "SURVIVED";
    writeFileSync(m.log, `mutant ${m.id}: ${m.what}\nfile ${m.file}\ntest pattern: ${m.test}\nexit ${r.status}; tests ${tests}, fail ${fail}: ${verdict}\n\n${r.stdout}${r.stderr}`);
    console.log(`${m.id}: ${verdict} (tests ${tests}, fail ${fail}, exit ${r.status}) ${m.log}`);
  } finally {
    writeFileSync(m.file, before);
    if (sha(m.file) !== hash) throw new Error(`${m.id}: ${m.file} not restored (sha256 differs)`);
  }
}
