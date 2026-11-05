Task: skeptic, try to refute security-auditor delta finding SD-5 (CLONE_VALUE missing --revision; writtenBy clone folder) on A3 at ed3ea8c.
Plan:
B1 read code routine-guard.mjs CLONE_VALUE + clone in writtenBy at ed3ea8c
B2 read rulings (advisor-fix-round, auto-4 finding 11, builder-fix2/step9, security-auditor-delta SD-5)
B3 probe in a real shell (git --revision support, guard result)
B4 verdict
- B1 done: code at ed3ea8c (HEAD ed9976f differs only in .checkpoints): CLONE_VALUE :556-558 lacks --revision; clone block :603-616, dir = pos[1] if 2+ positionals; code added in delta (F-b, builder-fix2). Next B2.
- B2 done: advisor-fix-round.md:74 F-b "Fix in A3 now" (clone [opts] <url> [<dir>] asks, agents refused). No limit L1/L4/L9/L12-L15 covers clone option parsing. auto-4 finding 11 (MEDIUM) open; no note handled it (grep revision/finding 11). Auditor's sad-probe.out:30-32: control ask, --revision null, --bran deny. Next B3 own probe.
- B3 done (skeptic-sd5-probe.mjs/.out, probe2.mjs/.out; patched copies under .gate/kit-v3-3/sd5): at head `git clone --revision main <url> .claude/sub` null for main AND agent (control ask/deny); also null into `.claude` and `CLAUDE.md`; `--revision=main` form asks; unknown `--x val` null. Local git is 2.43 (no --revision), so git's acceptance rests on git 2.49 release notes / git-clone docs (not run here); parse-options accepts the split form for any string option.
- Finding (fix refinement): the proposed `[...pos, ...pos.map(cloneDir), ...gitDir]` adds a false ask when the SOURCE is the project root: `git clone <project> <outside>` and `git clone -b main <project> <outside>` go null -> ask (kitDir sees the project root). Better: `[...pos.slice(1), ...pos.map(cloneDir).filter((d) => d !== '.' && d !== '..'), ...gitDir]` (g2 variant: all fixed rows ask, project-root source null; `git clone . <outside>` still asks unless '.'/'..' filtered). Existing control test routine-guard.test.mjs:1555 clones O (not the project) so it would not catch the g1 false ask.
- B4 verdict: SD-5 stands (not refuted): real, in the delta (F-b, ruled "fix in A3 now" advisor-fix-round.md:74), no stated limit covers it, auto-4 finding 11 open.
DONE
