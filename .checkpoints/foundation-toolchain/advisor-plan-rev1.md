Task: advisor plan review, foundation step 2.2 (toolchain), plan revision 1 at f535752.

Plan (blocks, in order):
B1 verify plan at f535752 == working copy
B2 read plan.md (whole)
B3 read Mustafa's words (session file, 828a3e0 words + answers, STATUS 680-710)
B4 read sketch (828a3e0 plan Part 2/5/App B) + advisor-plan-rev2 (Q-A3, G0)
B5 read research-network.md; spot-check claims vs plan use
B6 check repo rules (CLAUDE.md, advisor.md, V2_PHASE0_BRIEF Don't list) vs plan 1.3
B7 check Part 5 edit list vs guard (protectedRel, mergeAllowed), run them
B8 review workflow draft Part 4.2 (permissions, pins, cut, probe/control, failure modes); PR #40 files at 411eeee
B9 rulings Q2.2-a, Q2.2-c, three changes, mutation commits, globs to 2.5, FDN-NOCODE
B10 verdict written

Notes:
- B1 done: git show f535752:plan.md == working copy (diff empty). Next B2.
- B2 done: plan read (421 lines). Session words file read (untracked, not in f535752; quotes in plan 1.2 match it).
- F? (K2 split): K2 bundles (i) heading scope to legacy/ (covered: "the lines that scope the legacy constraints") with (ii) "Phase 0 writes no v2 code" + exception at CLAUDE.md:51-53, which is the same kind of edit as K3/K4 (FDN-NOCODE). Plan marks K2 wholly "covered" -> inconsistent; (ii) belongs in FDN-NOCODE. Check CLAUDE.md lines.
- Note: his session prompt asks to "ask Mustafa whether his FREEZE-FDN covers each" kit edit; plan asks only K3/K4; K1 answered; K2 claimed covered. Check against that.
- B3 done: his words match plan 1.2 (decision "may plan and build foundation steps 2.1 and 2.2 now"; SCOPE (a); FREEZE-FDN at STATUS:686-690; LANE-RULES Models addition STATUS ~700-710). His 2.2 prompt: "Plan only, no build" -> build needs his start (plan says so). Next B4.
- B4 done: sketch Part 2 (Q2.2-a..d), Part 5, App B read; advisor rev2 Q-A3 provisional (a) confirmed in notes.
- F-1 (Medium): SCOPE (a) as he picked it (plan rev3, d389d91:.checkpoints/foundation/plan.md:524-536) reads "...and 2.2's kit edit scopes the constraints and adds your 2026-11-10 decision as the exception to the "no v2 code" lines". FDN-NOCODE omits this option text; it bears directly on the question (his own reply words do not repeat it, so asking is still right, but quote it). Plan 1.2 also lacks it.
- F-2 (Medium): K2's second half (CLAUDE.md:51-53 "Phase 0 writes no v2 code" + exception) is the same "no v2 code" edit as K3/K4, yet K2 is marked "covered". Fold into FDN-NOCODE (three lines, not two), or split K2.
- Sketch App B rule 4 said "2.2 (or 2.5 ...) narrows" the globs: leaving to 2.5 is within it.
- B5 pre (CLAUDE.md/advisor.md/ADR lines): CLAUDE.md:20-22, :45, :51-53; advisor.md:107, :322-323; ADR 0002:1432-1438, PM-37/38/39 at :1289-1291, G0 :569, PM-62 :1314 all upheld.
- Design notes so far (for B8): M2 fails at the pin check's sed (bash -e on failed $(...) assignment) before any cargo: OK. overflow should_panic test discriminates release overflow-checks: OK (E1 proves). M3: path dep changes Cargo.lock -> --locked error: OK.
- F-3 (Low/Medium, failure mode): probe judges getent success as "network reachable"; if nsswitch uses nss-resolve (unix socket to systemd-resolved, not netns-scoped) getent can succeed inside the cut -> false red (fail-closed). Suggest: probe verdict on a TCP connect (curl; ideally also an IP-literal connect resolved outside), getent printed only.
- F-4 (Low): control depends on crates.io being up (index.crates.io); job with zero deps otherwise doesn't need it. Accept or pick an endpoint the runner needs anyway.
- B5 done: research-network.md read. Claims the plan rests on: sudo, container --network, runner endpoints, unshare -n, setpriv flags, nnp, rustup 1.28.0 no-arg install, 1.28.1 RUSTUP_AUTO_INSTALL, override precedence, npm --offline/ci/if-present, cargo fetch, Node dates, image versions, ubuntu-latest->26.04: all QUOTED with URLs. INFERRED: missing test script fails (L3 proves); new netns = lo only (E2 prints; pass criterion expects it). ASSUMED: setpriv/unshare on the image (command -v), A1-A3 (E1/E2 prove), toolchain naming 1.99.1-x86_64 (E2 log). My local spot-checks (Ubuntu 24.04.5 container, same OS as runner): setpriv+unshare ship in util-linux 2.39.3, Essential: yes (dpkg -S) -> presence near-certain; `setpriv --help`: "--regid <gid|group>" accepts a name -> nogroup OK. nsswitch here "files dns" (runner may differ; F-3 stands as a failure mode only).
- F-5 (Low): job prints `rustup show active-toolchain` but does not assert it names rust-toolchain.toml; image stable = pin (N1), so the run-time assertion of "from rust-toolchain.toml alone" rests on a human reading E2's log. Tighten: grep the override source in that line; print rustc --version inside the cut too.
Next B6 (rules) and B7 (guard marks).
- B6 done (rules): plan 1.3 holds: no rule forbids 2.2 beyond his decision + SCOPE (a) + FREEZE-FDN. Brief :20-21/:144 cited right; brief :22 even names "Phase 1: the repository restructure, toolchain, CI gate v2". Additions for 1.3 (Low): engine-architect.md:52 and graphics-engineer.md:41 ("Never: ... write engine code in phase 0") bind those agents' own writing only (they review 2.2; brief them on his decision); CLAUDE.md Never "run unreviewed code fetched from the internet" -> npm/rustup downloads follow tools/' lockfile practice; make the dependency review explicit in security-auditor's brief (lockfile package count, install-script packages, registry-only resolved URLs). advisor.md:323: K2 is a hard-constraint change; its accepted ADR = ADR 0002 Status line recording part 1 (2.1b's docs pass, his go) must be on main before K2 is written/merged -> make it an explicit check in step 5. V2_PHASE0_BRIEF stays stale (the claude.ai Advisor's doc): name it in For Mustafa/STATUS follow-up (Low).
Next B7.
- B7 done: ran main's protectedRel/mergeAllowed myself (advisor-marks.mjs, this folder) on 35 paths: toolchain.yml, gate.yml, CLAUDE.md, advisor.md, rules/pages.md, rules/netcode.md = P not M; .gitignore, root manifests, crates/*, packages/* = neither; docs/REPO_MAP, STATUS, ROADMAP, reports = M; docs/adr/README.md, docs/DECISIONS.md = not M; any .npmrc, nested CLAUDE.md/AGENTS.md = P. Matches Part 5 exactly. Guard line refs (:28-29, :35-48, :84-100, :1486) upheld; merge check reads only gate+smoke (risk 2 right). Rules' globs: tools.md `**/tools/**` does not match crates/devtools; netcode.md `**/net/**` matches packages/net (observation right). syntax-gate.mjs checks .js/.mjs/.cjs/.html only (.ts untouched). gate.mjs/smoke.mjs do no repo-wide scan. .gitignore already has node_modules/ and dist/.
- F-6 (Low): Part 5 misses a row for CLAUDE.md Layout (:41-43, after 2.1b: legacy/ paths): after 2.2 the repo has crates/ and packages/ that Layout doesn't name. Either K5 under "paths in ... CLAUDE.md" (one ask) or "none: REPO_MAP carries it", stated.
- F-7 (Low): Part 8 says every agent at "its file's effort (high)"; engine-architect.md:6 is xhigh. Say "its file's effort".
Next B8 (workflow review).
- B8 done (workflow + coexistence). PR #40 411eeee: rust-toolchain.toml = plan's four keys; check.mjs cargoConfigFiles ~:1225, referenceProblems ~:1251 (plan's ranges right); spike has its own [workspace], no package.json. STATUS :680-690, :700-710, :918-919 upheld. gate.yml: ubuntu-24.04, same checkout/setup-node SHAs, contents: read, persist-credentials false, concurrency gate-...-ref cancel-in-progress true. Draft holds: permissions minimal, no secrets/caches/pull_request_target; sudo->unshare->setpriv order right; $(id -u) expands outside; env -i PATH from outer shell; control outside + probe inside is a real control; M2 fails at sed under -eo pipefail before any cargo; M3 fails at cargo fetch --locked (path dep changes the lock).
- F-8 (Medium): step 5 "merge main" into the branch: CLAUDE.md Never lists `merge` as switching the main folder; only `git merge --ff-only origin/main` is allowed there. A non-ff merge must run in a worktree under .gate/ (push from there), or the plan says how it stays ff/allowed.
- F-9 (Medium, with the mutation ruling): concurrency cancel-in-progress not stated; if copied from gate.yml (true), pushing a mutation's revert before its run ends cancels the evidence. State cancel-in-progress, and push each mutation alone, wait for its red conclusion at the expected step, then push its revert.
- F-10 (Low): lo stays down in the new netns; any tool that binds 127.0.0.1 fails (false red; 2.5's local server needs loopback). Tightening: `ip link set lo up` as root inside the namespace before setpriv; probe unchanged.
- F-11 (Low): local E1 cargo runs write root /target/; CLAUDE.md checkpoints rule 3 and CHECKPOINTS §10 put session builds under .gate/ and clean them: use --target-dir .gate/foundation-toolchain/target locally (CI keeps /target/).
Next B9 rulings, B10 verdict.
- Spot-checks (mine): `npm view` 2026-11-10: typescript latest 7.0.2, 6.0 line 6.0.2/6.0.3; vitest@5.1.0 engines "^22.12.0 || ^24.0.0 || >=26.0.0"; @playwright/test@1.65.0 engines ">=20" -> all match B4. `rustup toolchain install --help` (1.28.2 local): "Install or update the given toolchains, or by default the active toolchain" -> no-arg install from rust-toolchain.toml upheld.

B9 rulings (done):
- Q2.2-a: (a) FINAL. G0 is a gate-v2 stage; PM-0002-39 already puts it under --prove; 2.2 keeps the pin check + M2 (fails closed before any cargo). Register entry = decision doc, his go.
- Q2.2-c: (a) Node 22 FINAL. Follow-up in STATUS with a date: one Node 26 bump change-set for tools/ (gate files: his ask) and v2 together after 2.3, merged before 2027-03-31 (22 EOL 2027-04-30).
- Three changes: approved. Lint tools to 2.3 (no check uses them in 2.2; less lock and supply-chain surface); .gitignore two lines (/target/ anchored, test-results/ unanchored: right); own workflow file (gate.yml byte-identical per his words; 2.3 replaces it).
- Mutation commits on own branch: approved with F-9 conditions + new commits only (git revert; never force-push/reset), no [skip ci], no protected file, final diff free of them, run ids + failing step in the report.
- Plan 1.3: holds; add the B6 Lows.
- Part 5: complete on P marks (reproduced); add K2 split (F-2) and a Layout row (F-6).
- FDN-NOCODE: needed (his prompt asks per edit; his FREEZE-FDN words don't name these lines); fix per F-1/F-2; lead with plain words, say it is needed before step 5 only (not for the build).
- Globs to 2.5: justified (after 2.1b they match legacy files only; 2.2 adds no page/shamble-*.js; FREEZE-FDN (a)'s option listed them, so 2.5 or an earlier need can still ask).
- 4.2: holds; tighten F-3, F-5, F-10; F-4 optional; F-9 required.

B10 verdict (done): APPROVE f5357520329a1ce76b7bf3ec3a873ea86f378cbc with conditions: 1 FDN-NOCODE rewrite (F-1, F-2, plain lead, timing) before it goes to him; 2 step 5 merge in a .gate/ worktree or ff-only (F-8); 3 concurrency + mutation sequencing (F-9); 4 security-auditor brief names the dependency review; 5 step 5 checks ADR 0002 Status line records part 1 on main before K2; 6 Lows F-3,F-5,F-6,F-7,F-10,F-11, brief engine-architect/graphics on his decision, V2_PHASE0_BRIEF stale noted for the claude.ai Advisor (fold or follow-up; none blocks).
DONE
