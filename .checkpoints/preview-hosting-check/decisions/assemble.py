# Assembles docs/reports/2026-11-24-preview-hosting-check.md from report-body.md (written by hand) and three appendices
# made by script: A the results log (headings demoted two levels; the account ID's first and last four characters
# replaced by a marker), B the workflow texts as run (from the test repository's commits), C the advisor tool's
# consultations (from the checkpoint). Run from the repository root; the test repository's clone is read with git.
import hashlib, re, subprocess, sys, json
N = '.checkpoints/preview-hosting-check/'
body = open(N + 'report-body.md', encoding='utf-8').read()
log = open(N + 'results-log.md', encoding='utf-8').read()
REDACT = 'd17d...d8e7'
n_red = log.count(REDACT)
log = log.replace(REDACT, "[the account ID's first and last four characters, compared by him; not copied here]")
log = '\n'.join(('##' + l) if l.startswith('#') else l for l in log.rstrip('\n').split('\n'))
appA = ("_Moved whole from `.checkpoints/preview-hosting-check/results-log.md` by script (" +
        f"{len(open(N + 'results-log.md', 'rb').read())} bytes before the move; its headings demoted two levels; " +
        f"{n_red} places where it gave the account ID's first and last four characters replaced by a marker). Every time " +
        "is UTC unless marked; times about this chat's own process are transcript times._\n\n" + log + '\n')
def show(rev, path):
    return subprocess.run(['git', 'show', f'{rev}:{path}'], cwd='/home/user/shamble-preview-check', capture_output=True, check=True).stdout
parts = []
for title, rev, path in [
    ('deploy.yml', '347d5e0', '.github/workflows/deploy.yml'),
    ('close.yml', '347d5e0', '.github/workflows/close.yml'),
    ('probe.yml', '347d5e0', '.github/workflows/probe.yml'),
    ('browsers.yml', '347d5e0', '.github/workflows/browsers.yml'),
    ('deploy.yml on cs/b9-edit (the first token-test variant)', '7f3a684', '.github/workflows/deploy.yml'),
    ('deploy.yml on cs/b9-prod (the second token-test variant)', '9965630', '.github/workflows/deploy.yml'),
    ('check-token.yml on check-token (the token check, one run)', '863b639', '.github/workflows/check-token.yml'),
]:
    b = show(rev, path); h = hashlib.sha256(b).hexdigest()
    t = b.decode('utf-8')
    assert '```' not in t
    parts.append(f"### {title}\nAt commit {rev} of Mergincanli/shamble-preview-check; {len(b)} bytes, sha256 {h}.\n\n```yaml\n{t.rstrip(chr(10))}\n```\n")
appB = ("_Copied by script from the test repository's commits (`git show <commit>:<path>`). `main` 347d5e0 holds the four "
        "workflows that ran every run on `main` and every test pull request; the two token-test variants ran only on their "
        "own pull requests (#9, #10) in sitting 3; check-token.yml ran once (two attempts) on pull request #2._\n\n" + '\n'.join(parts))
card = open('.checkpoints/preview-hosting-check.md', encoding='utf-8').read()
i = card.index('## Advisor tool consultations'); j = card.index('\n## ', i + 5)
cons = card[i:j].split('\n', 1)[1].strip('\n')
appC = ("_Copied by script from the checkpoint's section \"Advisor tool consultations\" (his words of 2026-11-11: when, the "
        "question, what changed; each called before the step it bore on was saved as done, except the slip of 2026-11-15 "
        "told under \"The process\")._\n\n" + cons + '\n')
out = body.replace('APPENDIXA\n', appA).replace('APPENDIXB\n', appB).replace('APPENDIXC\n', appC)
left = [k for k in ['APPENDIXA', 'APPENDIXB', 'APPENDIXC'] if k in out]
assert not left, left
# Until the report is final it stays in the notes folder (the saver keeps it); --final writes it into docs/reports/.
dest = 'docs/reports/2026-11-24-preview-hosting-check.md' if '--final' in sys.argv else N + 'report-assembled.md'
open(dest, 'w', encoding='utf-8').write(out)
print('to', dest)
holes = re.findall(r'\b[A-Z0-9]{6,}\b', out)
print('written;', out.count('\n'), 'lines; placeholders still open:', sorted(set(h for h in holes if h in {'PRNUMBER','VERDICTTABLE','EVIDENCE','CARDSHA','VERIFIERLINE','DOCSKEEPERLINE','RISK13ROUTE','PROBEAFTER','STEPNEXT','DECISIONSNEEDED','FORMUSTAFA','RISK13ROUTE'})))
