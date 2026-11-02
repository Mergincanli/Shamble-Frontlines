SUBS = [
("at least 99 % register on the server up to P2 and 95 % at P3; false hits (a miss in the view counted as a hit) below 0.5 % |",
 "at least 99 % register on the server up to P2 and 95 % at P3; false hits (a miss in the view counted as a hit) below 0.5 %; judged with the cap above every honest rewind of the run (the run's largest measured rewind rounded up to a whole tick, recorded; proposed: at P2 part 8's honest rewind is 233–383 ms, so a 200 or 250 ms cap would fail the mark by design, and what each candidate cap costs is PM-0001-49's record); recorded: per shot whether the view time was accepted or replaced by the server's estimate (part 8's estimate with the buffer wait and half a tick for the blend), and the share of honest shots replaced, per profile |"),
("| L1: 3 seeds; L2: 1 run per profile. Bites: a rewind that leaves out the interpolation delay (at 9 m/s, 100 ms is about 0.9 m) |",
 "| L1: 3 seeds; L2: 1 run per profile. Bites: a rewind that leaves out the interpolation delay (at 9 m/s, 100 ms is about 0.9 m); an estimate without the buffer wait (with a buffer of 2 ticks or more, honest view times miss it by more than the 33.3 ms tolerance and are replaced: the netcode review's finding 1) |"),
("| r2 R2-SYNC-REC5 mark 1 (lines 2179–2181); the draft, OQ-0001-7 |",
 "| r2 R2-SYNC-REC5 mark 1 (lines 2179–2181); the draft, part 8 (the estimate and its arithmetic) and OQ-0001-7; the netcode review of 2026-11-02, finding 1 |"),
]
