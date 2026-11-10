export const meta = {
  name: 'verify-2-2-plan-fold',
  description: 'Adversarially check that plan revision 2 folds the advisor conditions as written and breaks nothing',
  phases: [
    { title: 'Verify', detail: 'three lenses, each trying to refute the fold of its conditions' },
    { title: 'Critic', detail: 'what is still unchecked or missing' },
  ],
}

const SAVE = (part) => `
## Saving your progress
A usage limit can cut you off part way, and you would then be started again with this same task. Keep notes in .checkpoints/foundation-toolchain/${part}.md (make the folder; you may always write there). Look there first: if the first line names another task, move the file into .checkpoints/foundation-toolchain/old/ and start fresh; if the notes end with DONE, write your answer from them; otherwise go on after the last block they mark done. Trust the blocks they mark done: never reread, recheck or redo them (a later block may still read a file it needs). Start a new file with one line naming your task, then your plan: the blocks you will do, in order (a block: a source read, a file reviewed, a test group run, a section written). Write each finished block to its file at once. Add a notes line for each finding as you find it (what, where), and one after each finished block (done, what comes next). Redo a block the notes don't mark done, written over, never appended. Scratch files go in .checkpoints/foundation-toolchain/: text and small scripts only; builds and binaries go under .gate/foundation-toolchain/. This folder is pushed to a public repository: never put a secret, token, environment value, request header, cookie or raw payload in it, whether in a note, a script or a script's output; the saver's secret check only knows a few token formats and cannot read archives or binaries. End with DONE, then give your answer from your notes alone, opening no other file. Never commit or push.`

const COMMON = `Context (Shamble Frontlines repository, working folder /home/user/Shamble-Frontlines; main is origin/main = 2d22b69).
A planning session wrote a change-set plan for foundation step 2.2 (the toolchain). The project's advisor agent reviewed revision 1 (commit f535752) and gave APPROVE with conditions 1-6, saying: "No new plan review is needed if you fold conditions 1 to 5 in as written." Revision 2 (commit b6c20be) claims to fold conditions 1-5 as written and the Low items of condition 6. Your job is adversarial: try to show where revision 2 does NOT fold its conditions as written, misquotes a source, contradicts itself, or breaks something revision 1 had right. Default to "fails" or "unclear" when the evidence is not there; never assume.
Sources (read them, do not trust summaries):
- Revision 2: .checkpoints/foundation-toolchain/plan.md (first check that \`git show b6c20be:.checkpoints/foundation-toolchain/plan.md\` equals the working copy).
- Revision 1: \`git show f535752:.checkpoints/foundation-toolchain/plan.md\`.
- The advisor's review notes, with its conditions and findings F-1 to F-11: .checkpoints/foundation-toolchain/advisor-plan-rev1.md.
- Mustafa's words: .checkpoints/foundation-toolchain/mustafa-2026-11-10-session.md; \`git show 828a3e0:.checkpoints/foundation-restructure/mustafa-2026-11-10.md\` and \`...-answers.md\`; docs/STATUS.md lines 680-710 on main; the SCOPE options at \`git show d389d91:.checkpoints/foundation/plan.md\` (lines 500-540); the FREEZE-FDN options at \`git show 828a3e0:.checkpoints/foundation-restructure/plan.md\` (Part 5).
- The research notes: .checkpoints/foundation-toolchain/research-network.md.
- Rules: CLAUDE.md (the Never list's bullet on switching the main folder; "Reporting to Mustafa"), .claude/CHECKPOINTS.md (§4, §8, §10), .claude/agents/advisor.md.
Hard limits: never switch the main folder's branch (read other commits with git show, or a worktree under .gate/); never edit plan.md or any file outside .checkpoints/foundation-toolchain/; never commit or push. Severity scale: Critical, High, Medium, Low (CLAUDE.md). Cite file:line or command output for every verdict.`

const SCHEMA = {
  type: 'object',
  properties: {
    lens: { type: 'string' },
    checks: {
      type: 'array',
      items: {
        type: 'object',
        properties: {
          id: { type: 'string' },
          verdict: { type: 'string', enum: ['holds', 'fails', 'unclear'] },
          evidence: { type: 'string' },
          fix: { type: 'string' },
        },
        required: ['id', 'verdict', 'evidence'],
      },
    },
    other_findings: {
      type: 'array',
      items: {
        type: 'object',
        properties: {
          severity: { type: 'string', enum: ['Critical', 'High', 'Medium', 'Low'] },
          what: { type: 'string' },
          where: { type: 'string' },
          fix: { type: 'string' },
        },
        required: ['severity', 'what', 'where'],
      },
    },
  },
  required: ['lens', 'checks', 'other_findings'],
}

const LENSES = [
  {
    part: 'fold-check-a',
    label: 'conditions 1 and 5, his words',
    prompt: `Lens A: condition 1 (FDN-NOCODE's text and the K2 split) and condition 5 (the ADR Status check before K2), plus every quotation of Mustafa's words in revision 2.
Checks to return (ids C1a-C1f, C5, Q): C1a the question covers exactly three lines (CLAUDE.md:20-22, CLAUDE.md:52, advisor.md:107) and K2 is split into K2a (covered) and K2b (asked) consistently in every place revision 2 mentions K2 (In one screen, Part 5, Part 8 step 5, Part 9, Part 11, Decisions needed); C1b it quotes the SCOPE option (a) clause from d389d91 byte for byte and says his reply does not repeat it (check his reply's text yourself); C1c it opens in plain words ("your freeze exception for the foundation (FREEZE-FDN)"); C1d it says the answer is needed before step 5 only; C1e the entry meets CLAUDE.md's reporting rule (a short code, the question in plain words, why it matters in one line, the options, a recommendation; no unexplained code); C1f the two FREEZE-FDN texts it quotes match their sources byte for byte (the option (a) text and his words on main); C5 step 5 checks that ADR 0002's Status line on main records part 1 as accepted before K2a and K2b are written, and 1.3 says so; Q every other quotation attributed to Mustafa in revision 2 matches its source byte for byte (list each and its source).${SAVE('fold-check-a')}`,
  },
  {
    part: 'fold-check-b',
    label: 'conditions 2 and 3, the CI job',
    prompt: `Lens B: condition 2 (step 5's merge done in a worktree under .gate/, pushed from there), condition 3 (cancel-in-progress stated; each planted failure pushed alone and left red before its revert), and the CI-job Lows F-3 (probe judged on a TCP connection, DNS printed not judged), F-4 (control and probe host), F-5 (active-toolchain check; rustc --version inside the cut), F-10 (loopback up as root before dropping privileges) in Part 4.1-4.3 and Part 7.
Checks to return (ids C2a, C2b, C3a, C3b, F3, F4, F5, F10, Y): C2a the step-5 procedure obeys CLAUDE.md's Never bullet on switching and merging in the main folder; C2b it would actually work in git (for example: can \`git worktree add\` check out a branch the main folder already has checked out? does \`--detach\` plus \`git push origin HEAD:<branch>\` give a fast-forward, not a force?) and matches CHECKPOINTS §8; C3a cancel-in-progress is stated for the new workflow; C3b the planted-failure procedure says push alone, wait for red at the expected step, then revert, and lists the advisor's other rules (reverts are new commits, no [skip ci], no protected file, none in the final diff, run ids and failing steps in the report); F3, F4, F5, F10 each folded as the advisor wrote it; Y the YAML draft in 4.2 is correct as shell and as YAML: trace the quoting of \`sudo -n unshare --net -- bash -ec 'ip link set lo up; exec "$@"' lo-up setpriv ... -- env -i ... bash -euo pipefail -c '...'\` argument by argument, check which variables expand in which shell, check that every tool it calls is on the PATH it is called with (sudo's secure_path for ip and setpriv; the env -i PATH inside), check \`setpriv --regid=nogroup --clear-groups --no-new-privs\`, and check the \`rustup show active-toolchain\` case pattern against the real output format: if rustup is installed in this container, run it in a scratch folder under .gate/foundation-toolchain/ holding a rust-toolchain.toml that names a toolchain already installed (see \`rustup toolchain list\`; set RUSTUP_AUTO_INSTALL=0 so nothing downloads) and report the exact line it prints. Report any failure mode you find in other_findings.${SAVE('fold-check-b')}`,
  },
  {
    part: 'fold-check-c',
    label: 'condition 4, other Lows, whole diff',
    prompt: `Lens C: condition 4 (security-auditor's brief names the dependency review: the lockfile's package count, the packages with install scripts, every resolved address pointing at the npm registry), the remaining Lows of condition 6 (F-6 a Part 5 row for CLAUDE.md's Layout lines; F-7 each agent at its file's effort, engine-architect's being xhigh; F-11 local cargo runs with --target-dir under .gate/ and cleaned after; rows in 1.3 for engine-architect.md:52 and graphics-engineer.md:41 and the briefing; the report saying V2_PHASE0_BRIEF.md:20-21 and :144 stay stale), Part 9's record of the verdict and rulings, and the whole diff from revision 1 to revision 2.
Checks to return (ids C4, F6, F7, F11, B1, B2, P9, D, L): C4 condition 4 folded (E11 and step 7); F6, F7, F11 each folded as the advisor wrote it (check F-7 against the agents' frontmatter in .claude/agents/); B1 the 1.3 rows for engine-architect.md:52 and graphics-engineer.md:41 quote those lines correctly; B2 the V2_PHASE0_BRIEF note is in 1.3 and step 7, and the brief's :22 quote matches the file; P9 Part 9 records the advisor's verdict, its conditions, its rulings and its own checks faithfully (compare with advisor-plan-rev1.md, especially its last block), and names the reviewed SHA in full; D run \`git diff f535752 b6c20be -- .checkpoints/foundation-toolchain/plan.md\` and list every change no condition or ruling asked for, and anything revision 1 had that revision 2 dropped without a reason; L every line citation in revision 2's Appendix A and in its tables still holds on main 2d22b69 (spot-check at least twenty, including all new ones: CLAUDE.md:41-43, engine-architect.md:52, graphics-engineer.md:41, V2_PHASE0_BRIEF.md:20-22, the d389d91 range).${SAVE('fold-check-c')}`,
  },
]

phase('Verify')
const results = await parallel(LENSES.map((l) => () =>
  agent(`${COMMON}\n\n${l.prompt}`, { label: l.label, phase: 'Verify', schema: SCHEMA, effort: 'high' })))
const got = results.map((r, i) => ({ lens: LENSES[i].label, result: r }))
const missing = got.filter((g) => !g.result).map((g) => g.lens)
if (missing.length) log(`no result from: ${missing.join(', ')} (re-run those alone)`)

phase('Critic')
const critic = await agent(`${COMMON}

You are the completeness critic. Three checkers each took one lens on revision 2; their structured results are below (a null means that checker returned nothing, so its lens is unchecked). Do not redo their checks. Ask instead: which of the advisor's conditions 1-6 (including each Low item F-3 to F-11 and the two notes) or rulings did no checker test? Which claim in revision 2 that the build or Mustafa would rely on is unverified by anyone? Did any checker's evidence fall short of its verdict? Check those gaps yourself, briefly, and return them as checks (ids G1, G2, ...) and other_findings.

The checkers' results:
${JSON.stringify(got, null, 1)}${SAVE('fold-check-critic')}`, { label: 'completeness critic', phase: 'Critic', schema: SCHEMA, effort: 'high' })

return { checkers: got, critic }
