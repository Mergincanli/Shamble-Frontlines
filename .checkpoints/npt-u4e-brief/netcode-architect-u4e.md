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
B7c small fixes pass (cite ends, client clippy lines, -62/server-echo in Q10), whole-file rewrite

Started 06:06 Amsterdam. Only Write is available (no Edit tool): notes rewritten whole each time.

Notes:
- B1-B6b done. Key cites: plan U4e :685-716; Lanes :814-845; §6 :846-863; §7 :864-887; §4.11 :379-394; plan.md §4.7
  :568-616; carries :145-174; advisor cond 6 :125-129, cond 10 :143-149, nit d :164-165, U4s conds :304-335, rulings
  :349-375, F2 :436-465; check.mjs STEPS :1474-1507 (10), deps :1447-1472, biteHomeProblems :723-747 (doc :712-722);
  client/clippy.toml :1-9 (clock ban only); seam.rs :21-287; ticks.rs :56, :70; link Loopback :129-212.
- B7a done; B7b done 06:16: draft complete, sections 1-8: Start, What to build (2.1-2.8), carries (8, verbatim fenced
  block of carries :145-174), build rules, end checks, return list (12 items), recheck table (28 rows R1-R28),
  questions U4E-Q1..Q10.
- Finding for B7c: -62/server-echo's site is the room's event routing (§4.14 :458; seam.rs:219-220 "never to the
  shooter's own client"), which U4e builds (EventQueue), while -62 moves in U4g (:794): add to Q10.
Question codes:
- U4E-Q1 lt-cmd-seq, lt-press planted in U4e? (rec: yes)
- U4E-Q2 lt-tick-offset's unit proof needs a replay (U4g) (rec: build the compare, show it as wrong command named)
- U4E-Q3 bites-only id lists for npt-room / npt-server, not on §4.1 (rec: yes, bites-only, reported)
- U4E-Q4 tick16's id home: proto (U4a a5) vs client (carry 24); one-crate rule (rec: after R11; keep in proto)
- U4E-Q5 rewind_at for a claimed view time outside the ring before U4h (rec: no Rewind, counted)
- U4E-Q6 EventQueue cap value (rec: provisional data value, U4r measures)
- U4E-Q7 snap_incomplete / burst_eviction / hs: U4e exposes, U4r writes (rec)
- U4E-Q8 client/clippy.toml hash-map bans (rec: yes in U4e)
- U4E-Q9 Pong hold-time limit is rate-dependent (rec: F2 pattern, rate as argument)
- U4E-Q10 carry 24's -45/-62 ids (rec: only ids whose sites U4e builds) + -62/server-echo's room site
Needs a run (day): base test counts both builds (cargo test --list, with and without --all-features, shared target);
  carry 26 copy cost; Send asserts compile; check.mjs step count at <base>.
Not done / left: CARGO_NET_OFFLINE and df rules are from the task's shape, not plan §7.1 text (said in draft); the
  df floor is a <floor> placeholder; the notes-folder path for u4e-build.md to be confirmed by the day session.
Next: B7c.
