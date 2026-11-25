# Builds the report's appendix: every changed passage of ADR 0006, before (main 09faefa) and after (the working tree),
# each asserted to occur exactly once in its file. Run from the repo root. Prints Markdown.
import subprocess
P = 'docs/adr/0006-infrastructure-operations.md'
base = subprocess.run(['git', 'show', '09faefa:' + P], capture_output=True, text=True, check=True).stdout
head = open(P, encoding='utf-8').read()

def line_of(doc, s):
    return doc[:doc.index(s)].count('\n') + 1

E = []
def add(code, place, before, after):
    if before is not None:
        assert base.count(before) == 1, (code, 'before', base.count(before))
    assert head.count(after) == 1, (code, 'after', head.count(after))
    E.append((code, place, before, after))

add('E1', "PM-0006-2, the \"number and statistic\" cell",
    "previews `X-Robots-Tag: noindex`, production none; the preview built with",
    "previews `X-Robots-Tag: noindex`; production none on its own addresses (the project's `pages.dev` host, and a custom domain if CF has one, as in PM-0006-8), while on production's one-off (hash) addresses `X-Robots-Tag` is recorded, not judged (revised 2026-11-25, Mustafa's ONE-OFF-HEADER (a): the preview hosting check read `noindex` on all 7 paths of production's one-off addresses 0354374e and de1c3e5e, written by no rule of the production build; a one-off address works like a preview address, and the header there keeps old builds out of search engines); the preview built with")
add('E1b', "PM-0006-2, the \"what it proves\" cell",
    "The host serves each header set as generated, on both URL forms (part 1; decides R5-PREV-OQ6)",
    "The host serves each header set as generated, on both URL forms (production's one-off addresses: the host's own `X-Robots-Tag` recorded, not judged) (part 1; decides R5-PREV-OQ6)")
add('R3-11', "PM-0006-2, the \"method and tool\" cell",
    "each read on its branch alias and on its deployment's hash URL;",
    "each read on its branch alias and on its deployment's hash URL, and production on a custom domain if CF has one;")
add('E2', "PM-0006-2, the bites in \"runs and noise\"",
    "a `noindex` rule added to production |",
    "a `noindex` rule added to production (fails on production's own address) |")
add('E3', "part 1, the build count",
    "Pages' Free plan allows 500 builds a\n   month (R6-HOST-F3, r6 lines 72–79), and whether a direct upload counts against them is a spike item (part 14). If\n   uploads count, only the newest push of each pull request deploys (ADR 0002's concurrency cancel) and the month's\n   deploys are counted; a paid plan is Mustafa's after a price read (proposed, not researched).",
    "Pages' Free plan allows 500 builds a\n   month (R6-HOST-F3, r6 lines 72–79). Whether a direct upload counts against them is not answered: the preview\n   hosting check saw no build count in the dashboard, before or after its ten uploads (2026-11-18), and Cloudflare's\n   docs do not say (read 2026-11-24 and 2026-11-25; PM-0006-7; `docs/reports/2026-11-24-preview-hosting-check.md`). If\n   uploads count, only the newest push of each pull request deploys (ADR 0002's concurrency cancel) and the month's\n   deploys are counted; until someone shows a count, the previews (foundation step 2.6) work that way anyway (Mustafa's\n   STEP4-COUNT (b), 2026-11-25, 00:55 Amsterdam time). A paid plan is Mustafa's after a price read (proposed, not\n   researched).")
add('R3-10', "part 1, *Why:*",
    "link with no cap on previews, and GitHub Pages cannot send those headers;",
    "link with no cap on previews (as read on 2026-10-28; Pages' limits page, read 2026-11-25, no longer says so and\n   reads \"Preview deployments older than 30 days are deleted, unless they are one of the project's 100 most recent\n   deployments\" (developers.cloudflare.com/pages/platform/limits/): follow-up ADR6-PAGES-RETENTION), and GitHub Pages\n   cannot send those headers;")
i = head.index("   *Workers static assets, weighed again*")
j = head.index("check again on Workers.", i) + len("check again on Workers.")
add('E7', "part 1, a new paragraph after *Need:* (15 lines)", None, head[i:j])
add('E6', "spike 1's list",
    "- whether a direct upload counts against Pages' 500 builds a month (R6-HOST-REC1, r6 lines 633–634);\n",
    "- whether a direct upload counts against Pages' 500 builds a month (R6-HOST-REC1, r6 lines 633–634); not answered by\n  the check, which saw no count (part 1);\n")
add('E4', "PM-0006-7, the \"number and statistic\" cell",
    "anything else recorded as read; if uploads count, part 1's consequence follows (only the newest push per pull request is deployed, and the month's deploys are counted) |",
    "anything else recorded as read; if uploads count, part 1's consequence follows (only the newest push per pull request is deployed, and the month's deploys are counted), and until a count is shown it holds anyway. Not answered by the preview hosting check (revised 2026-11-25, Mustafa's STEP4-COUNT (b)): the dashboard showed no count before or after the ten uploads (2026-11-18), and Cloudflare's docs do not say (read 2026-11-24 and 2026-11-25) |")
add('E9', "PM-0006-7, the \"runs and noise\" cell (the advisor agent's words)",
    "| one pair of readings; passes when both are recorded |",
    "| one pair of readings; passes when both are recorded, each as the count the page shows or as \"no count shown\" (revised 2026-11-25: the advisor agent's reading of Mustafa's STEP4-COUNT (b), never his decision; \"no count shown\" leaves the question unanswered, part 1) |")
add('E11', "the hosting checklist's intro",
    "or the numbers it asks for.",
    "or the readings it asks for.")
add('E10', "the hosting checklist, step 4 (the advisor agent's words)",
    "read the count again. Expected: two numbers recorded (PM-0006-7).",
    "read the count again. Expected: two readings recorded, each a number or\n   \"no count shown\" (PM-0006-7; revised 2026-11-25, the advisor agent's reading of Mustafa's STEP4-COUNT (b)). Read\n   on 2026-11-18, before and after the ten uploads: no count shown, so the question is not answered (part 1).")

def q(s):
    s = s.rstrip('\n')
    return '\n'.join('> ' + l.strip() if l.strip() else '>' for l in s.split('\n'))

out = []
for code, place, before, after in E:
    hl = line_of(head, after)
    out.append(f"**{code}. {place}** (now :{hl}" + (f"; was :{line_of(base, before)}" if before else '') + ")")
    out.append('')
    if before is not None:
        out.append('Before:')
        out.append(q(before))
        out.append('')
    out.append('After:' if before is not None else 'Added:')
    out.append(q(after))
    out.append('')
print('\n'.join(out))
print('<!-- entries:', len(E), '-->')
