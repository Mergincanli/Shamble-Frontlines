R7 applier3 TL: fixes3-ops-TL.json for area TL (5 findings)
verifier-tlkt#0 TL10.where: rejected (condition 8: renumbers a kit citation to today's lines). git show 48ffb51:.claude/settings.json has PostToolUse :97, command :101. Instead: where now says "at 48ffb51:" (+ block :97-104; syntax-gate.mjs unchanged since, git diff 48ffb51 HEAD empty) and v2_note gets "Changed by kit-v3-0 (...)" (30ff4a4, 1c259ce added allow rules).
verifier-tlkt#5 TL16.v2_note: accepted, fix_op as written. EDITOR_V2:35 symmetry, :39 validation panel.
verifier-tlkt#6 TL3.evidence: accepted, own replace op dating STATUS:125 (2026-10-14, 8675367) and adding battery 138/714/353 today.
verifier-tlkt#7 TL4.evidence: accepted, own replace op dating STATUS:125-126 and adding battery 51/51/0.
verifier-tlkt#8 TL4.v2_note: accepted, fix_op as written (EDITOR_V2:51 has "(index.html)").
apply-ops.py TL: 6 ops applied cleanly.
DONE
