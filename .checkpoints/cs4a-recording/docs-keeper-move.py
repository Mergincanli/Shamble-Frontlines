# docs-keeper, change-set 4a's docs pass: moves STATUS lines of merged pull requests to the archive by line range,
# byte for byte (his OLD-LINES). Run once from the worktree root; it checks the lines it moves before moving them.
S, A = 'docs/STATUS.md', 'docs/STATUS_ARCHIVE.md'
st = open(S, encoding='utf-8').read().split('\n')
ar = open(A, encoding='utf-8').read().split('\n')
row = st[24]                      # STATUS line 25: ci-after-3a's in-flight row (#71, merged)
wait = st[41:45]                  # STATUS lines 42-45: its "Waiting on Mustafa" bullet
fu3a = st[91:96]                  # STATUS lines 92-96: the "From change-set 3a" follow-ups bullet (#65, merged)
assert row.startswith('| none yet (title and text in the notes') and 'ci-after-3a' in row
assert wait[0].startswith('- ci-after-3a: his go') and wait[3].endswith('2026-11-25-ci-after-3a.md`).')
assert fu3a[0].startswith('- **From change-set 3a**') and fu3a[4].endswith('for the drafts 3A-DEPARTURES.')
assert ar[245] == '  3B-OUTAGE-256).' and ar[246] == '' and ar[247] == '## Targets'
assert ar[1741] == '' and ar[1742] == '## Decisions pending: the rest'
inflight = ['',
 "_Moved from STATUS (2026-11-25, change-set 4a's docs pass, under his OLD-LINES of 2026-11-18): STATUS's row for the "
 "merged ci-after-3a (PR #71, 07d673b, merged 2026-11-25 20:49:09 UTC by `merged_at`), moved by script, byte for byte, "
 "so its cells read as they stood when it left STATUS:_",
 '', '| PR | Work | Branch | State | Waiting on |', '|---|---|---|---|---|', row, '',
 "_And STATUS's \"Waiting on Mustafa\" bullet for ci-after-3a (#71), moved by the same docs pass and rule, byte for byte:_",
 ''] + wait
followups = [
 "- **Moved from STATUS (2026-11-25, change-set 4a's docs pass)** (under his OLD-LINES: lines about merged pull requests,",
 "  moved by script, byte for byte): STATUS's Follow-ups bullet for change-set 3a (PR #65, merged 2026-11-24 as 28254a7)",
 "  is the bullet below. Its pointer to \"the archive's block\" leads to the block \"From change-set 3a\" above; since #71's",
 "  merge (07d673b) 3A-GEN-CHECK and 3A-RULES-STEP's CI half are done (the block \"From ci-after-3a\" above), and change-set",
 "  4's plan places 3A-FMA-CHECK and 3A-FEATURE-VERSION in 4b, 3A-SCAN-HELPERS and 3A-PART5-CLASH in 4b or 4c."] + fu3a
ar = ar[:246] + inflight + ar[246:1741] + followups + ar[1741:]
st = st[:24] + ['@@ROW70@@'] + st[25:41] + ['@@WAIT70@@'] + st[45:91] + ['@@FU3A@@'] + st[96:]
open(A, 'w', encoding='utf-8').write('\n'.join(ar))
open(S, 'w', encoding='utf-8').write('\n'.join(st))
print('moved: STATUS 25 ->archive In flight; 42-45 -> same; 92-96 -> archive Follow-ups')
