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
- 0003 read: Decision 1-11, budget table (sums right: Low 3+16+4+2=25, High 2+6+3+1=12), spike section, PM-0003-1..43 rows.
  F-f (med) Low draw-call cap: Decision 0003:316-319 still says "about 786, inside 800"; PM-0003-12 itself notes R3's 786 counts no post pass (14 left; bloom alone 12). Add health bars <=2 (PM-0003-29), VFX pools (>=5 kinds), projectile kinds (4, x2 with shadow), ink+output: extras ~13-27 vs 14 slack. Decision text should carry it / name what is off at Low / state a cap.
  F-g (med) VISION_V2 §6.9 "without dropping frames": no presented-frame (mode P) mark; PM-0003-30 is a delta (<=2.0 ms GPU p99, <=200 draws) and PM-0003-18 is p95<=16.7 uncapped => up to 5% of frames over a refresh. Need a dropped-frame mark (mode P, RD High, BS with the 4 VLUs) or the Advisor's reading.
- 0004 spike table S1-S22 and PM-0004 rows (numbers) read; 0002 boundary spike + PM-0002-1..19 read.
  PM-0002-7's 0.1 MB/s is an allocation rate (clear). Wasm size: recorded not gated until the Advisor sets it (0002:237-242) - acceptable; gzip only (no brotli).
  F-h (low) PM-0003-36's "fails on any per-frame allocation in the render path" has no method/tool/threshold in CI (headless); a sampling profiler cannot prove zero (PM-0003-29 itself says so).
  F-i (low) Worker side on the client: no budget/leak check for the Worker's JS heap or Wasm linear memory (PM-0003-24 reads the page heap only), nor for the Worker's whole tick in a local match (sim step + local server + client endpoint); S18 jitter covers pacing only.
- 0005 GPU rules + PM-0005 rows read. Caps arithmetic right (200/4/2 = 25; uses all of 0003's margin over 160). PM-0005-32 measures effects at 8/16/32 per soldier (reported).
  Q-1 (question) stress scene's 8 active effects/soldier vs permanent upgrade picks (pick effects as attribute modifiers, AB-8): late-match count unknown until track length (Mustafa); PM-0005-32 reports but nothing gates.
- Next: 0006 (fleet density/egress/costs), SPIKE_PLAN, README, OVERVIEW; then the cross-doc agreement checks.
- 0006 read: findings, costs part 8, spike item 2, PM-0006-13..31.
  F-c confirmed by PM-0006-17 itself (0006:884): it notes SS's 700 shots/s ~10x R2's 72 and re-evaluates R2's formula at the run's counts for egress; PM-0001-12 gates R2's 160 kbps (set on ES) in SS. R2 formula at SS counts: 583.5 B + (2,800 shots + 240 reliable + 2,170 spawns)/30 = 757 B/snapshot = 181.7 kbps (+13.6 % over 160).
  F-j (med) Density "case to budget" 4 rooms/VM (0006:262-267, A-2) is not implied by ADR 0001's marks: transport runtime <= 1 vCPU per room (PM-0001-11) + tick (<=50 % p99) admit ~2 rooms per CCX23 at the ceilings (ceil(20/2)+1 = 11 x 86.49 = 951 EUR). Fix: a per-room CPU mark consistent with 4/VM (tick mean + transport <= 1 vCPU, proposed) or state the 2/VM case.
- Next: SPIKE_PLAN, README, OVERVIEW.
