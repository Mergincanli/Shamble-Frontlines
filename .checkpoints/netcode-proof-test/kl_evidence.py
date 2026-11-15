"""KEPT-LOGS build evidence (the advisor's C7; KL-CHANGE's build_plan_changes 1-4; the draft's (c) steps 1-6), run from
the repo root against the build worktree .gate/kept-logs-wt (branch claude/kept-logs-rule, from main 4460ef5).
Everything it writes goes under .gate/kept-logs-rule/ (gitignored): mutant copies, TAP files. It never packs or
deletes a real folder: the dry run's test copies one into a temp repository, and the real folder's fingerprint is
taken before and after.
  compare [<ref>]  the PR's four files (the working tree, or the commit <ref>) against the rebuilt ones (kl_build.py's
                   kl-build/expected/), byte for byte; with <ref>, also the PR's file list against main
  full             the whole suite: 10 entries, 9 pass, 1 skipped (the dry run); tar's and node's versions
  mutants          M1-M14, each one exact replacement matching once; every failing test with actual and expected
  dry              the dry run for the two real folders, with each real folder's fingerprint before and after
  kit              the routine-guard, settings and checkpoint tests at the head; counts equal to main's (base/)
  quotes           §11's two quotes, whitespace joined, against history.md:2095-2097 and :2105
  guard            the real guard hook (main's settings.json) fed the day-session pack command: no decision expected
Run:  python3 -I .checkpoints/netcode-proof-test/kl_evidence.py <step> [<ref>]"""
import hashlib
import json
import os
import re
import subprocess
import sys

REPO = "/home/user/Shamble-Frontlines"
WT = f"{REPO}/.gate/kept-logs-wt"
OUT = f"{REPO}/.gate/kept-logs-rule"
EXP = "/tmp/claude-0/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/scratchpad/kl-build/expected"
BASE = "4460ef5b8d6dc8566b29c15ded75c6f0c8be386b"
PATHS = [".claude/CHECKPOINTS.md", ".claude/scripts/kept-logs.mjs", ".claude/scripts/kept-logs.test.mjs",
         "docs/REPO_MAP.md"]
SCRIPT, TEST = ".claude/scripts/kept-logs.mjs", ".claude/scripts/kept-logs.test.mjs"
REAL = ["u4r-precheck-30-faf42aeb034c", "u4r-P3-fd4612050cd6"]
sha = lambda b: hashlib.sha256(b).hexdigest()
os.makedirs(OUT, exist_ok=True)


def run(args, cwd=WT, env=None):
    return subprocess.run(args, cwd=cwd, env=env, capture_output=True, text=True)


def node_test(tap, extra=None, args=()):
    env = {k: v for k, v in os.environ.items() if k not in ("SHAMBLE_ROLE", "KEPT_LOGS_SCRIPT", "KEPT_LOGS_DRY_RUN")}
    env.update(extra or {})
    r = run(["node", "--test", "--test-reporter=tap", *args, TEST], env=env)
    with open(f"{OUT}/{tap}", "w", encoding="utf-8") as fh:
        fh.write(r.stdout + (f"\n# stderr of the runner:\n{r.stderr}" if r.stderr else ""))
    return r.stdout


def entries(tap):
    lines, got = tap.split("\n"), []
    for i, l in enumerate(lines):
        m = re.match(r"^(not ok|ok) (\d+) - (.*?)( # SKIP.*)?$", l)
        if m:
            e = {"ok": m.group(1) == "ok", "n": int(m.group(2)), "name": m.group(3), "skip": bool(m.group(4)),
                 "line": l}
            for key in ("expected", "actual"):
                for j in range(i + 1, len(lines)):
                    if lines[j] == "  ...":
                        break
                    f = re.match(rf"^  {key}: ?(.*)$", lines[j])
                    if f:
                        v, k = f.group(1), j + 1
                        if v in ("|-", "|", ">-", ">"):
                            v = []
                            while k < len(lines) and lines[k].startswith("    "):
                                v.append(lines[k].strip())
                                k += 1
                            v = "\\n".join(v)
                        e[key] = v[:160]
                        break
            got.append(e)
    return got


def summary(tap):
    return {k: int(v) for k, v in re.findall(r"^# (tests|suites|pass|fail|cancelled|skipped|todo) (\d+)$", tap, re.M)}


def compare(ref=None):
    for p in PATHS:
        want = open(f"{EXP}/{p}", "rb").read()
        have = (subprocess.run(["git", "show", f"{ref}:{p}"], cwd=WT, capture_output=True, check=True).stdout if ref
                else open(f"{WT}/{p}", "rb").read())
        nl = have.count(b"\n")
        print(f"{'equal' if have == want else 'DIFFERS'}: {p} ({ref or 'working tree'}) {len(have)} bytes, "
              f"{nl} lines, sha256 {sha(have)}")
        assert have == want, p
    if ref:
        names = run(["git", "diff", "--name-status", BASE, ref]).stdout.split("\n")
        names = [n for n in names if n]
        print("files the PR changes (git diff --name-status against main 4460ef5):", *names, sep="\n  ")
        assert sorted(n.split("\t")[1] for n in names) == sorted(PATHS), names


def full():
    print(run(["tar", "--version"]).stdout.split("\n")[0], "|", run(["node", "--version"]).stdout.strip())
    tap = node_test("full.tap")
    es, s = entries(tap), summary(tap)
    print("summary:", s)
    for e in es:
        print(e["line"])
    assert len(es) == 10 and s.get("pass") == 9 and s.get("skipped") == 1 and s.get("fail") == 0, s
    assert es[1]["ok"] and not es[1]["skip"] and es[1]["name"].startswith("GNU tar unpacks"), es[1]
    assert es[9]["skip"] and es[9]["name"].startswith("dry run"), es[9]


MUT = {  # planted bug: (the exact text, its replacement, what it breaks)
    "M1": ("    const bad = rows.filter((r, i) => got[i]?.path !== ", "    const bad = [].filter((r, i) => got[i]?.path !== ",
           "the read-back compare removed (bad always empty)"),
    "M2": ("    const got = await readBack(createReadStream(part));\n",
           "    rmSync(dir, { recursive: true });\n    const got = await readBack(createReadStream(part));\n",
           "the folder removed before the read-back"),
    "M3": ("  if (process.env.SHAMBLE_ROLE) refuse(", "  if (false) refuse(", "the routine refusal removed"),
    "M4": ("  if (differs(now, rows)) {\n", "  if (false) {\n", "the folder-equals-list check removed (first walk)"),
    "M5": ("  if (real !== abs) refuse(", "  if (false) refuse(", "realDir's link check removed"),
    "M6": ("  for (const ref of ['HEAD', pushed]) {\n", "  for (const ref of ['HEAD']) {\n",
           "the pushed-ref check removed"),
    "M7": ("    if (real === dir || real.startsWith(`${dir}/`)) refuse(", "    if (false) refuse(",
           "cache's worktree check removed"),
    "M8": ("      if (parseInt(field(h, 148, 156).trim(), 8) !== sum) throw", "      if (false) throw",
           "readBack's checksum check removed"),
    "M9": ("  if (exists(part)) {\n", "  if (false) {\n", "the stale-.part refusal removed"),
    "M10": ("\n  } catch (e) {\n", "\n  } catch (e) {\n    rmSync(part, { force: true });\n",
            "the catch removes the .part"),
    "M11": ("  if (newest(dir) >= OLD_BEFORE) {\n", "  if (false) {\n", "cache's date bound removed"),
    "M12": ("    if (differs(walk(dir, set, (msg) => { throw new Mismatch(msg); }), rows)) {\n"
            "      throw new Mismatch(`the folder changed while it was packed: its files or sizes now differ from "
            "${listRel}`);\n    }\n", "", "the second walk removed"),
    "M13": ("walk(dir, set, (msg) => { throw new Mismatch(msg); })", "walk(dir, set)",
            "the second walk refuses (exit 2) instead of a mismatch"),
    "M14": ("    if (differs(walk(dir, set, (msg) => { throw new Mismatch(msg); }), rows)) {",
            "    if (walk(dir, set, (msg) => { throw new Mismatch(msg); }).length !== rows.length) {",
            "the second walk compares only the count"),
}
HOMES = {"M1": [4], "M2": [4], "M3": [6], "M4": [4], "M5": [7, 8], "M6": [3], "M7": [8], "M8": [9], "M9": [4],
         "M10": [4], "M11": [8], "M12": [5], "M13": [5], "M14": [5]}


def mutants():
    src = open(f"{WT}/{SCRIPT}", encoding="utf-8").read()
    assert src.encode() == open(f"{EXP}/{SCRIPT}", "rb").read(), "the worktree's script is not the rebuilt one"
    ok = True
    for m, (old, new, what) in MUT.items():
        assert src.count(old) == 1, f"{m} matches {src.count(old)} times"
        os.makedirs(f"{OUT}/{m}", exist_ok=True)
        with open(f"{OUT}/{m}/kept-logs.mjs", "w", encoding="utf-8") as fh:
            fh.write(src.replace(old, new))
        tap = node_test(f"{m}.tap", {"KEPT_LOGS_SCRIPT": f"{OUT}/{m}/kept-logs.mjs"})
        fails = [e for e in entries(tap) if not e["ok"]]
        hit = all(any(e["n"] == n for e in fails) for n in HOMES[m])
        ok &= hit
        print(f"{m} ({what}): {'BITES' if hit else 'DOES NOT BITE'} its tests {HOMES[m]}; fails "
              + "; ".join(f"test {e['n']} (actual {e.get('actual')}, expected {e.get('expected')})" for e in fails))
    assert ok, "a planted bug did not fail its named test"


def fingerprint(path):
    r = subprocess.run(["bash", "-c", f"find {path} -type f -printf '%P %s %T@\\n' | sort | sha256sum"],
                       capture_output=True, text=True, check=True)
    return r.stdout.split()[0]


def dry():
    for s in REAL:
        real = f"{REPO}/.gate/netcode-proof-test/runs/{s}"
        before = fingerprint(real)
        tap = node_test(f"dry-{s}.tap", {"KEPT_LOGS_DRY_RUN": real}, ("--test-name-pattern=dry run",))
        after = fingerprint(real)
        es = [e for e in entries(tap) if e["name"].startswith("dry run")]
        diag = [l for l in tap.split("\n") if re.match(r"^\s*# (u4r-|kept-logs: )", l)]
        print(f"{s}: fingerprint before {before}, after {after}: {'equal' if before == after else 'DIFFERENT'}")
        print("  " + es[0]["line"] if es else "  (no dry-run entry)", *diag, sep="\n  ")
        assert before == after and es and es[0]["ok"] and not es[0]["skip"], s


def kit():
    for t in (".claude/scripts/checkpoint.test.mjs", ".claude/hooks/settings.test.mjs",
              ".claude/hooks/routine-guard.test.mjs"):
        n = os.path.basename(t)[:-4]
        env = {k: v for k, v in os.environ.items() if k != "SHAMBLE_ROLE"}
        r = run(["node", "--test", t], env=env)
        os.makedirs(f"{OUT}/head", exist_ok=True)
        with open(f"{OUT}/head/{n}.tap", "w", encoding="utf-8") as fh:
            fh.write(r.stdout + r.stderr)
        head, base = summary(r.stdout + r.stderr), summary(open(f"{OUT}/base/{n}.tap", encoding="utf-8").read())
        print(f"{t}: exit {r.returncode}; head {head}; main {base}: {'equal' if head == base else 'DIFFERENT'}")
        assert r.returncode == 0 and head == base, t


def quotes():
    cp = open(f"{WT}/.claude/CHECKPOINTS.md", encoding="utf-8").read()
    sec = cp[cp.index("## 11. Kept raw logs"):]
    j = lambda s: " ".join(s.split())
    q1 = j(re.search(r'His words: "([^"]*)"', sec).group(1))
    q2 = j(re.search(r'\("([^"]*)"\)', sec).group(1))
    h = open(f"{REPO}/.checkpoints/netcode-proof-test/history.md", encoding="utf-8").read().split("\n")
    w1, w2 = j(" ".join(h[2094:2097])), h[2104]
    print(f"quote 1: {'equal' if q1 == w1 else 'DIFFERENT'} to history.md:2095-2097 ({len(q1)} chars)")
    print(f"quote 2: {'in' if q2 in w2 else 'NOT IN'} history.md:2105 ({len(q2)} chars)")
    assert q1 == w1 and q2 in w2


def guard():
    s = json.load(open(f"{WT}/.claude/settings.json", encoding="utf-8"))
    h = next(x for g in s["hooks"]["PreToolUse"] for x in g.get("hooks", [])
             if any("guard.mjs" in str(a) for a in x.get("args", [])))
    args = [str(a).replace("${CLAUDE_PROJECT_DIR}", WT) for a in h["args"]]
    cmd = ("node .claude/scripts/kept-logs.mjs pack u4r-P3-fd4612050cd6 --results "
           "spikes/netcode-proof/results/u4r/fd4612050cd6/P3")
    env = {k: v for k, v in os.environ.items() if k != "SHAMBLE_ROLE"}
    env["CLAUDE_PROJECT_DIR"] = WT
    inp = {"hook_event_name": "PreToolUse", "tool_name": "Bash", "tool_input": {"command": cmd}, "cwd": WT}
    r = subprocess.run([h["command"], *args], input=json.dumps(inp), env=env, capture_output=True, text=True,
                       timeout=30)
    print(f"guard on `{cmd}`: exit {r.returncode}, stdout {r.stdout.strip()!r}, stderr {r.stderr.strip()!r}")
    assert r.returncode == 0 and r.stdout.strip() == "", "the guard gave a decision"


step = sys.argv[1]
{"compare": lambda: compare(sys.argv[2] if len(sys.argv) > 2 else None), "full": full, "mutants": mutants,
 "dry": dry, "kit": kit, "quotes": quotes, "guard": guard}[step]()
print(f"{step}: PASS")
