# test-engineer round 1: extra mutations of hosting/preview.mjs (same harness as mutate.py)
import subprocess, sys, pathlib
SUITE = pathlib.Path(sys.argv[1]); SRC = (SUITE / 'hosting/preview.mjs').read_text()
M = [
 ("T1 comment: a failed write passes", "if (w.status !== (mine ? 200 : 201)) throw", "if (false) throw"),
 ("T2 comment: closed PR still written", "if (p.json?.state !== 'open' || p.json?.head?.sha !== sha", "if (p.json?.head?.sha !== sha"),
 ("T3 comment: fork head repo accepted", "|| p.json?.head?.repo?.full_name !== name) {", ") {"),
 ("T4 comment: PR read failure not fatal", "if (p.status !== 200) throw new Refusal(`pull request ${pr} could not be read", "if (false) throw new Refusal(`pull request ${pr} could not be read"),
 ("T5 comment: comment-list failure goes on (POSTs a duplicate)", "if (!list.ok) throw new Refusal(`the pull request's comments", "if (false) throw new Refusal(`the pull request's comments"),
 ("T6 githubList: >10 pages counted as complete", "return { ok: false, items, why: `longer than ${MAX_PAGES} pages` };", "return { ok: true, items };"),
 ("T7 verify-token: HTTP status ignored", "if (v.status !== 200 || v.json?.success !== true || result.status !== 'active')", "if (result.status !== 'active')"),
 ("T8 verify-token: project HTTP/success ignored", "if (p.status !== 200 || p.json?.success !== true || record.name !== name)", "if (record.name !== name)"),
 ("T9 fetchServed: no retry on 5xx", "if (!(res.status >= 500 || res.status === 404 || blocked)) return last;", "if (!(res.status === 404 || blocked)) return last;"),
 ("T10 fetchServed: no retry on 403/1010", "const blocked = res.status === 403 && /1010/.test((await res.text()).slice(0, 4000));", "const blocked = false;"),
 ("T11 fetchServed: a thrown fetch leaves status 200", "last.error = errorName(err);", "last.error = errorName(err); last.status = 200;"),
 ("T12 monthCount: unreadable list counted as 0", "if (r.status !== 200 || !Array.isArray(r.json)) return null;\n    let wholePageOlder", "if (r.status !== 200 || !Array.isArray(r.json)) return n;\n    let wholePageOlder"),
 ("T13 read-url: id shape not checked", "if (typeof d.deployment_id !== 'string' || !PATTERNS.deployment.test(d.deployment_id)) {", "if (typeof d.deployment_id !== 'string') {"),
 ("T14 check-build: second check skipped", "if (second.refusals.length > 0) {", "if (false) {"),
 ("T15 check-headers: build list refusals ignored", "if (tree.refusals.length > 0) reportRefusals(io, 'the build\\'s file list'", "if (false) reportRefusals(io, 'the build\\'s file list'"),
 ("T16 find-pr: list failure ignored", "if (!list.ok) throw new Refusal(`the open pull requests", "if (false) throw new Refusal(`the open pull requests"),
 ("T17 find-pr: state field ignored", "list.items.filter((p) => p?.state === 'open' && ", "list.items.filter((p) => "),
 ("T18 listPreviews: >40 pages counted complete", "return { complete: false, items, why: `more than ${CLOUDFLARE_MAX_PAGES} pages` };", "return { complete: true, items };"),
 ("T19 setOutput: GITHUB_OUTPUT not required absolute", "if (!file || !isAbsolute(file)) throw", "if (!file) throw"),
 ("T20 annotate: line breaks not escaped", ".replace(/\\r/g, '%0D').replace(/\\n/g, '%0A')}`);", "}`);"),
 ("T21 checkHeaders: retry gives up after 1 try", "for (let attempt = 1; attempt <= 5; attempt += 1) {\n    last = { status: 0", "for (let attempt = 1; attempt <= 2; attempt += 1) {\n    last = { status: 0"),
 ("T22 decide: newest success can be deleted (<=)", "if (madeAt(d) < madeAt(newest)) candidates.push", "if (madeAt(d) <= madeAt(newest)) candidates.push"),
 ("T23 decide: closed check by loose state", "if (state === 'closed') {\n      for", "if (state !== 'open' && state !== undefined) {\n      for"),
 ("T24 verify-token: no 14-day warning", "ends - io.now().getTime() < 14 * 24 * 3600 * 1000", "ends - io.now().getTime() < 0"),
 ("T25 delete: success flag ignored", "const ok = del.status === 200 && del.json?.success === true;", "const ok = del.status === 200;"),
]
bad = []
for name, find, repl in M:
    n = SRC.count(find)
    if n != 1: print(f"ANCHOR x{n}: {name}"); bad.append(name); continue
    (SUITE / 'hosting/preview.mjs').write_text(SRC.replace(find, repl))
    r = subprocess.run(['node', '--test', 'hosting/preview.test.mjs'], cwd=SUITE, capture_output=True, text=True, timeout=300)
    fails = [l for l in r.stdout.splitlines() if l.startswith('# fail ')]
    v = 'caught' if r.returncode != 0 else 'SURVIVED'
    print(f"{v:9} {fails[0] if fails else ''}  {name}", flush=True)
(SUITE / 'hosting/preview.mjs').write_text(SRC)
