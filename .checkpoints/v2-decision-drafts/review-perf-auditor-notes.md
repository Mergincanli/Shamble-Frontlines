# perf-auditor review of the v2 decision drafts (plan step 10, wave 1) at commit 47eadd9 — notes
- Started 2026-11-02. No prior notes; no perf-auditor memory dir exists (no stored baselines). Docs at HEAD equal 47eadd9 (only checkpoint files changed since).
- Read: brief, plan rules, advisor §1, VISION_V2, GLOSSARY §1 (tick table arithmetic re-checked: all sums right), r2 budgets tables + worked estimate (1672-1795), R2-BUD-REC1..6.
- 0001 read: Decision parts 1-14, bandwidth estimate, budgets table, pass marks PM-1..30.
  Candidate findings so far:
  F-a (med/high) E3b fallback (PM-0001-54 -> rerun PM-0001-12 at mean<=160) is predicted by the draft's own estimate to fail: E3b 188.3 kbps > 160 (0001:223, 505, 539, 1177). Dead end in the decision tree.
  F-b (med) Upstream estimate 22.3 kbps (0001:513-519) uses R2's 8 B command x4, but the decided command has an aim point up to 80 bits (10 B) alone plus view time, press counters; redundancy cap 8; at P2 ~5-7 commands are unacked. Realistic 37-57 kbps vs the 48 kbps mark (PM-0001-13). Mark measurable; estimate misleading.
  F-c (med) PM-0001-12 measures the 160 kbps mean in SS (300 live projectiles, 4 VLUs >=20 parts, 8 effects/soldier) while the research mark and estimate are for ES (32 projectiles). No estimate for SS; spawn events alone could add ~25 kbps. Check stress-scene glossary.
  F-d (check) PM-0001-27: 8 rooms on 4 vCPU vs transport runtime <=1 vCPU per process; R6 A-2 density 8 rooms/VM. Check 0006 cost figures.
  F-a strengthened: in SS (300 live projectiles), E3b = 300 x 5.5 B = 1,650 B/snapshot > 1,067 B/interval hard cap -> E3b cannot fit at all in SS.
  F-c computed: SS vs ES on R2's own per-event sizes: shots 700/s x 4 B vs 72/s => +20.1 kbps; projectile spawns 300 live / 1.5 s implied life = 200/s x 11 B vs 21/s => +15.7 kbps; SS ~182 kbps > 160 mean (PM-0001-12 runs in SS).
  F-e (low/med) PM-0001-39: portal changes at exactly 1/s at a 60 Hz tick with snapshots on every 2nd tick: re-flood ticks always land on the same parity -> either always or never on encode ticks; the tight case (8.28 ceiling) is chosen by the script's phase, unstated.
- 0001 pass marks fully read (PM-1..57). Next: 0003 (frame/GPU budgets), then 0004, 0002, 0005, 0006, SPIKE_PLAN, README, OVERVIEW.
