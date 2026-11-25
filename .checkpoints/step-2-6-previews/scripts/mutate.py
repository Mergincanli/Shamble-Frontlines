# Mutation sweep for hosting/preview.mjs: each mutation breaks one guard; the suite must fail on every one.
import subprocess, shutil, sys, pathlib
SUITE = pathlib.Path(sys.argv[1]); SRC = pathlib.Path(sys.argv[2]).read_text()
M = [
  ("check-build reads the folder on any outcome", "if (outcome !== 'success') {", "if (outcome === 'never') {"),
  ("no dot-name rule", "if (name.startsWith('.')) return 'a name starting with a dot';", ""),
  ("no host-file rule", "if (HOST_FILES.has(name.toLowerCase())) return", "if (false) return"),
  ("links followed", "if (!st.isFile()) { refusals.push", "if (false) { refusals.push"),
  ("no size limit", "if (st.size >= MAX_FILE_BYTES)", "if (false)"),
  ("no file-count limit", "if (files.length > MAX_FILES)", "if (false)"),
  ("headers not compared", "else if (!io.fs.readFileSync(join(root, '_headers')).equals(headers))", "else if (false)"),
  ("find-pr accepts forks", "&& p.head?.repo?.full_name === name", ""),
  ("find-pr takes the first of several", "if (matches.length !== 1) {", "if (matches.length === 0) {"),
  ("commit form not checked first", "if (!PATTERNS.commit.test(sha)) throw new Refusal(`the run's commit", "if (false) throw new Refusal(`the run's commit"),
  ("token status not checked", "|| result.status !== 'active')", ")"),
  ("production branch not checked", "|| PATTERNS.branch.test(record.production_branch))", ")"),
  ("read-url: any subdomain", "if (!new RegExp(`^https://pr-[1-9][0-9]{0,5}\\\\.${literal(sub)}$`).test(alias) || !PATTERNS.link.test(alias))", "if (!PATTERNS.link.test(alias))"),
  ("read-url: production accepted", "if (d.environment !== 'preview')", "if (false)"),
  ("redirects followed", "const res = await io.fetch(url, { method: 'GET', redirect: 'manual'", "const res = await io.fetch(url, { method: 'GET', redirect: 'follow'"),
  ("immutable not required on assets", "if (hashed && !immutable)", "if (false)"),
  ("no noindex check", "if (!/(?:^|[\\s,])noindex(?:$|[\\s,])/i.test(h['x-robots-tag'] ?? ''))", "if (false)"),
  ("no bite", "if (cacheBite.length === 0 || noindexBite.length === 0)", "if (false)"),
  ("comment: other PR's link accepted", "|| !link.startsWith(`https://pr-${pr}.`)", ""),
  ("comment: any author's marked comment edited", "c?.user?.login === BOT && c?.user?.type === 'Bot' &&", ""),
  ("comment: written after a newer push", "|| p.json?.head?.sha !== sha", ""),
  ("reconcile: deletes on a failed lookup", "if (state === 'closed') {", "if (state !== 'open') {"),
  ("reconcile: no cap", "return { now: candidates.slice(0, max)", "return { now: candidates"),
  ("reconcile: an incomplete listing goes on", "if (!listing.complete) throw", "if (false) throw"),
  ("reconcile: any branch name", "|| !PATTERNS.branch.test(branch)", ""),
  ("show() lets line breaks through", "const json = JSON.stringify(value === undefined ? null : value) ?? 'null';", "const json = String(value);"),
  ("output written without its shape check", "if (!pattern.test(value)) throw new Refusal(`the output", "if (false) throw new Refusal(`the output"),
  ("annotate: no % escaping", "io.out(`::${kind}::${message.replace(/%/g, '%25')", "io.out(`::${kind}::${message.replace(/%/g, '%')"),
  ("comment: the number's prefix accepted (no dot)", "|| !link.startsWith(`https://pr-${pr}.`)", "|| !link.startsWith(`https://pr-${pr}`)"),
  ("reconcile: duplicates not removed", "items: items.filter((d) => typeof d?.id !== 'string' || (!ids.has(d.id) && ids.add(d.id)))", "items"),
  ("check-build: only the plain hand-over name", "/^[ \\t]*name:[ \\t]*([\"']?)preview-site\\1[ \\t]*(?:#.*)?$/m", "/^[ \\t]*name:[ \\t]*preview-site[ \\t]*$/m"),
]
bad = []
for name, find, repl in M:
    n = SRC.count(find)
    if n != 1: bad.append(f"ANCHOR x{n}: {name}"); continue
    (SUITE / 'hosting/preview.mjs').write_text(SRC.replace(find, repl))
    r = subprocess.run(['node', '--test', 'hosting/preview.test.mjs'], cwd=SUITE, capture_output=True, text=True, timeout=300)
    fails = [l for l in r.stdout.splitlines() if l.startswith('# fail ')]
    verdict = 'caught' if r.returncode != 0 else 'SURVIVED'
    print(f"{verdict:9} {fails[0] if fails else ''}  {name}")
    if r.returncode == 0: bad.append(name)
(SUITE / 'hosting/preview.mjs').write_text(SRC)
print('survivors or bad anchors:', bad)
