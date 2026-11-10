Task: npt-u4e-brief step 1 (netcode-architect drafts the U4e builder brief; room and codec path), night 2026-11-10.

Plan (blocks, in order):
B1 read u4-build-plan.md §5 U4e (:685-716), Lanes (:814-845), §6, §7
B2 read u4s-carries.md §U4e (:145-175)
B3 read advisor-u4.md (verdict, U4s rulings, F1/F2)
B4 Glob wave-1 briefs, read one for shape
B5 read §4.1-§4.10, §4.14 (skim for U4e-relevant names)
B6 skim code: crates room/server/client/proto, sim seam
B7 write draft sections 2, 3, 1, 4, 5, 6, recheck table, questions

Started 06:06 Amsterdam; target draft 06:40. Only Write is available (no Edit tool): notes rewritten whole each time.

Notes:
- B1 done (plan U4e :685-716, Lanes :814-845, §6 :846-863, §7 :864-887 read).
- B2 done: carries 21-27 and 39 at u4s-carries.md:146-174 (quoted from there).
- B4 done: no *brief* file in the spike's .checkpoints (Glob empty); shape from the task prompt.
- Finding: plan :706 asks npt-client wrong-channel drop; :712 bites list; :691 room/clippy.toml hash-map bans.
- B3 done: advisor-u4.md cond 6 (:125-129, -18 end to end), cond 10 (:143-149), Q21 :220 and :157; U4s cond 4
  (:334-335, word for word, report carries by number); cond 2 (:318-320, carry 39's origin); F2 ruling (:436-465:
  refill and window take rate as argument, pinned at 30 and 60 Hz: refill 60 and 120/s; window 32,000 B/s at both;
  route (b) for anything new, :450-458); O1 (:343-347) is U4r's, not U4e's.
- Finding: advisor :126-127 says "after the burst", plan :708-709 says "through a 1 s gap" (same thing, -18's B1 burst).
Next: B5.
