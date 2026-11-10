Task: npt-u4e-brief step 1 (netcode-architect drafts the U4e builder brief; room and codec path), night 2026-11-10.

Plan (blocks, in order):
B1 read u4-build-plan.md §5 U4e (:685-716), Lanes (:814-845), §6, §7
B2 read u4s-carries.md §U4e (:145-175)
B3 read advisor-u4.md (verdict, U4s rulings, F1/F2)
B4 Glob wave-1 briefs, read one for shape
B5 read §4.1-§4.10, §4.14 (skim for U4e-relevant names)
B6 skim code: crates room/server/client/proto, sim seam, check.mjs
B6b read plan §5 U4a, U4b, U4d (names for the recheck table); plan.md §4.7 (public surface); §4.11
B7a write draft §1-§3 (written 06:13)
B7b write whole draft: §1-§8 (written 06:16)
B7c small fixes pass (cite ends, client clippy lines, -62/server-echo in Q10), whole-file rewrite (written 06:18)

Started 06:06 Amsterdam; draft complete 06:18. Only Write is available (no Edit tool): notes rewritten whole.

Notes:
- B1-B7c done. Key cites: plan U4e :685-716; Lanes :814-845; §6 :846-863; §7 :864-887; §4.11 :379-394; plan.md §4.7
  :568-616; carries :145-174; advisor cond 6 :125-129, cond 10 :143-149, nit d :164-165, U4s conds :304-335, rulings
  :349-375, F2 :436-465; check.mjs STEPS :1474-1507 (10), deps :1447-1472, biteHomeProblems :723-747; client/clippy.toml
  :1-9 (clock ban only); seam.rs :21-287; ticks.rs :56, :70; link Loopback :129-212 (no loss; rtt Some(0)).
- No wave-1 builder brief exists in the spike's .checkpoints (Glob for *brief* empty): shape taken from the task.
Draft state (/home/user/Shamble-Frontlines/.checkpoints/npt-u4e-brief/u4e-brief-draft.md), all sections complete:
  header + citation key; 1 Start; 2 What to build (2.1 room, 2.2 server library, 2.3 client, 2.4 tests e1-e12,
  2.5 -18 end to end, 2.6 bites table (8 rows), 2.7 seam rule, 2.8 public items); 3 carries (8 quoted verbatim:
  21-27, 39, fenced block of carries :145-174); 4 build rules; 5 end checks; 6 return list (12 items); 7 recheck
  table (28 rows R1-R28: U4a R1-R14, U4b R15-R16, U4d R17-R24, plus R25 check.mjs steps (U4a/U4d2), R26 bite ids
  (U4d/U4a), R27-R28 U4j's marks ids and log fields); 8 questions U4E-Q1..Q10.
Question codes:
- U4E-Q1 lt-cmd-seq, lt-press: sites U4e's (:691, §4.14 :452-453), U4r judges six -58 bites (:737), U4e's list omits
  them (:711-712). Rec: U4e plants both.
- U4E-Q2 lt-tick-offset's proof needs a replay (U4g's Predictor :786). Rec: U4e builds the compare, shows "wrong
  command named"; U4g re-shows.
- U4E-Q3 bites-only BITES lists in npt-room / npt-server not on §4.1 or plan.md §4.7. Rec: yes, reported (npt-client
  precedent).
- U4E-Q4 tick16 id home: U4a plants in proto (a5 :532), carry 24 says client half in npt-client, check.mjs one-crate
  rule (:712-747). Rec: decide after recheck R11; keep ids in proto, client takes the bite through a passed value.
- U4E-Q5 rewind_at for a claimed view tick outside the ring before U4h. Rec: no Rewind, counted, logged.
- U4E-Q6 EventQueue cap value (measured, §4.7 :325-326). Rec: provisional data value; U4r measures.
- U4E-Q7 snap_incomplete / burst_eviction / hs: who writes them. Rec: U4e exposes, U4r writes.
- U4E-Q8 client/clippy.toml hash-map bans (§4.11 :380 vs §5 U4e :691). Rec: yes in U4e.
- U4E-Q9 Pong hold-time limit (one tick period, catalogue :92) rate-dependent. Rec: F2 pattern, rate as argument.
- U4E-Q10 carry 24's -45/-62 ids (sites U4g's :789-790) and -62/server-echo's room site (§4.14 :458). Rec: U4e plants
  only what it builds (server-echo yes; -45, cues-from-replay in U4g).
Needs a run (day):
- base test counts, both builds: from spikes/netcode-proof/, CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/
  shared-target CARGO_NET_OFFLINE=true cargo test --locked --workspace -- --list (and with --all-features).
- check.mjs step count at <base> (node check.mjs, same env); carry 26 per-tick copy cost; e8 Send asserts compile.
- R11: git grep -n 'tick16"' <base> -- spikes/netcode-proof/crates (a grep, the day session's).
Left / not settled:
- CARGO_NET_OFFLINE and df rules are from the task's shape, not plan §7.1's text (said in the draft); df floor is a
  <floor> placeholder; the u4e-build.md folder to be confirmed against wave 1's builders.
- Plan silent, said in the draft, not added: Join replacing a live session (Kick(replaced), catalogue :93); e7's scene;
  the -18 gap mechanism over Loopback (draft suggests a test-only Link wrapper, length in ns).
- Not verified: no build, no run, no test (night, by the task); marks.json ids not read (R27).
DONE
