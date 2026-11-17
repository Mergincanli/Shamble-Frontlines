# D4, model B: the netcode-architect's independent model (U4f brief rev 3.1, section 11 row D4)

Agent: netcode-architect (xhigh), 2026-11-17. Task: an independent second model of D4, before the advisor's check
(it decides U4F-Q6). I did not read N/u4f-r3/d4/ (the main session's model). No Edit tool: this file is rewritten
whole with Write after each block.

## Plan

1. Read D4 (brief :1491), 2.1 (:278-292), U4F-Q6 (:1264-1286), NA-4 (:1609) and the code they cite. DONE.
2. Write every modelling choice the brief leaves open (below). DONE.
3. Write the read-only script `d4_model_b.py` (stdlib only, seeded `random.Random`, prints only). DONE.
4. Run it once: `python3 -I /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4f-r3/d4b/d4_model_b.py`;
   copy the whole output here. DONE (one run; the output below).
5. Read the output; return the schema. DONE (section "Reading").

## What the code says (read at the head; spikes/ unchanged since 0ad9661)

- **L1 per direction** (l1/src/lib.rs:149-163, :186-188, :214-221; the draw :823-847): delay =
  `(one_way + below(w, 2a + 1)) - a`, a uniform **integer** ns in [one_way - a, one_way + a] inclusive; P2: one_way
  50,000,000, a 10,000,000, loss 20,000 ppm each direction (:935: `below(w, 1e6) < loss_ppm`). The 1 % reorder step
  (:15-22, :1188) is on in every run: not in D4's parameters, not modelled (choice C11).
- **The client's step grid** (client/src/lib.rs:485-488): step n is due at `base + floor(n * 1e9 / tick_hz)` ns on
  the host clock (integer floor). The lab sets the host clock to SimClock + `offset_ns`, uniform in [0, 1,000 s)
  (lab/src/scenario.rs:930, :1131, :1134, :1195, :1201); every arrival is stamped `r.arrival_ns + offset_ns`
  (:1134, :1201): so as built a sample is timed by its datagram's arrival, not by the step.
- **The client's Pong to a room ping** (client :471-483): it answers at its next step with `hold_us` = step - arrival;
  irrelevant to (e)'s client-side estimate (that is the room's RTT, pump.rs:500-516).
- **The room's ping** (pump.rs:16, :387-404): first at the first pump after Joined (`next_ping_ns` = accept_ns,
  :269), then at the first pump at or after the last send + 250 ms; it carries `sent_ns = now_ns`, the server's
  exact ns. 250 ms = 7.5 ticks at 30 Hz, so against a 30 Hz client grid successive room pings alternate two phases
  half a tick apart.
- **The room's answer to a client Ping** (pump.rs:458-472): in the I/O step that reads it; `hold` = now - arrival
  (0 on L1: the runner wakes at the arrival event, scenario.rs module text :17-18); `tick` is the pump's argument.
- **The tick the pump is given** (scenario.rs:475-476, :480): `room.last_tick() + 1`, the next tick to run. RQ8's
  order puts the I/O step (2) before the tick (3) at one instant (scenario.rs:6-11), so a Ping arriving exactly at
  tick k's due instant is stamped k (tick k has not yet run).
- **The server's tick grid** (server/src/schedule.rs:5, :125-134): tick k due at `floor(k * 1e9 / 30)` on the loop's
  clock (SimClock in the lab).
- **-47** (marks.json:9095-9111): `event.connect_clock.offset_err_ns`, p99 <= 5,000,000 ns, `window: all`; the judge's
  p99 is nearest-rank, rank ceil(0.99 n), 1-based (judge/src/eval.rs:264-289, test :1749-1760). The model uses the
  same rank: n = 1,000 -> the 990th smallest; n = 3,000 -> the 2,970th.

## Modelling choices (every one the brief leaves open)

- **C1, the offset's sign.** h = host clock - server clock, the lab's `offset_ns` itself (true offset = h). Error =
  estimate - h; reported as abs(error). The sign does not change any abs value.
- **C2, integer ns throughout,** as the code: delays, the client's floor grid, the server's floor grid, the room's
  ping times. Estimates are formed from integers and divided once at the end (float only for the final ns error).
- **C3, the per-sample own-ping estimate (Cristian / NTP midpoint).** For own ping m: host send c_s (the step's host
  time, the Ping's echo), host arrival of the Pong c_r, server stamp s_hat: est_m = (c_s + c_r) / 2 - s_hat. Then
  est_m - h = (d_down - d_up) / 2 + (s_r - s_hat) [+ b / 2 bitten], where s_r is the Ping's server arrival and b the
  bite's delay of the Pong's stamp. RTT_m = c_r - c_s (hold 0, nothing subtracted).
- **C4, the stamp's half-tick bias.** With the as-built tick stamp k, the server instant s_r lies in
  (T_{k-1}, T_k] (C5), so every tick-stamp estimator uses the **centred** stamp s_hat = (T_{k-1} + T_k) / 2 (unbiased
  under a uniform phase). Used for the mean, the lowest-RTT and the newest-sample estimators alike. The uncentred
  stamp (s_hat = T_k) is printed once as a sensitivity line only (a fixed +P/2 = 16.7 ms bias: it fails by itself),
  not a schema row.
- **C5, a sample exactly at a tick instant.** Stamp k = the smallest k with T_k >= s_r (T_k = s_r gives k: RQ8's I/O
  before the tick). With the centred stamp that sample's quantisation error is +P/2 (the interval's right end).
- **C6, the exact sub-tick stamp (U4F-Q6 (b), an addition).** s_hat = s_r floored to the µs since the previous
  tick's due instant (the field's 1 µs unit; error at most 1 µs). Same C3 midpoint formula.
- **C7, phase models.** Server ticks at T_k = floor(k * 1e9 / 30). Per connect, seeded: h uniform in [0, 1,000 s)
  (the lab's range), phi uniform integer in [0, 33,333,333], the client's first ping at server time
  S0 = T_300 + phi (10 s in), the client's step grid on the host clock base = S0 + h, steps at base +
  floor(n * 1e9 / 30); own ping m (m = 0..19) at step 3m, so its server send is S0 + m * 1e8 exactly.
  - (ii) **phase-locked:** the tick grid as is; every ping meets it at exactly phase phi (3m ticks of the floor grid
    are exactly m * 1e8 ns from tick 300, so no ns drift), moved only by d_up.
  - (i) **uniform per sample:** for own ping m only, the server's tick grid used to quantise that sample is shifted by
    u_m, uniform integer in [0, 33,333,333], drawn per sample (as if the client's host rate wandered): everything
    else (sends, the client's step grid, the bite) is unchanged, so (i) and (ii) differ only in the quantisation.
- **C8, the room-ping schedule's phase.** Per connect a seeded start R0 uniform integer in [0, 250 ms) after S0; room
  ping i (i = 0..7) sent at server time S0 + R0 + i * 250 ms exactly (the pump's slip to its next wake is left out:
  on a busy 48-client L1 run it is well under 1 ms). Its phase against the client's step grid is therefore one
  seeded phase per connect, alternating by half a tick (250 ms = 7.5 ticks). The same schedule under (i) and (ii):
  the phase models are about the server's tick grid against the client's own pings, and (e)'s stamp is exact, so
  (e)'s rows are the same under (i) and (ii) by construction (reported twice, as the schema's grid asks).
- **C9, (e)'s combination of the 8 room pings.** Each room ping i gives est_i = c_a,i - sent_i - owd, with c_a,i its
  host arrival stamp (bitten under (2)), sent_i its exact server ns, and owd = min over own pings of RTT_m / 2 (the
  observed RTT: bitten under (2)). The brief does not say how the 8 combine, so both are reported, each a main
  (e) row: **e/mean-of-8** (the mean of the 8 est_i) and **e/lowest-of-8** (the est_i of the smallest c_a - sent, the
  lowest-delay filter that matches the half-lowest-RTT owd; the room ping is a one-way sample so its "lowest" is its
  lowest one-way). Expectation before the run: e/mean carries a bias of about mean(d_down) - min(RTT)/2 ≈ 50 - 42.6
  ≈ +7.4 ms (half the lowest of 20 triangular RTTs sits below the mean one-way delay), so it fails; e/lowest pairs two
  lower envelopes and should be near 0 with a few ms of spread.
- **C10, the bite (arrivals (2), PM-0001-47/tick-quantised).** Every arrival stamp (own Pongs and room pings) moved to
  the first client step at or after its arrival (an arrival exactly at a step is read by that step: the lab polls
  `poll_recv(now)`, which includes arrivals at `now`). The send stamps stay on the step grid (they already are). So a
  bitten own-ping RTT is a whole number of ticks: at P2 3 ticks (exactly 100,000,000 ns on the floor grid, since
  3 ticks of it are exactly 1e8 ns) or 4. **Ties in the lowest-RTT pick:** the first minimum in send order (oldest
  wins; strict `<`); bitten 3-tick RTTs tie exactly, so the pick is the oldest sample whose true RTT is at most
  100 ms.
- **C11, what is not modelled.** P2's 2 % loss each way is **not** in the main rows (D4 fixes 20 client samples and 8
  room pings); it is an **addition**: own sample kept only if both its Ping and Pong survive (0.98^2), room ping kept
  at 0.98; the lowest-RTT owd for (e) over the surviving own samples. The 1 % reorder step (l1 :15-22) only ever
  delays a datagram (to 1 ns after its successor in the flow), so it never lowers a lowest-RTT pick but can shift a
  mean by up to about (one tick + jitter) / 40 per reordered datagram: not modelled (D4's parameters leave it out);
  named as a known gap. Also out: the ADR's filter rules (drift and reset thresholds, ADR 0001:240-242), the
  read-instant cut (all 20 Pongs and all 8 room pings arrive before the first client step at or after 2 s from the
  first sample: the 20th Pong by about S0 + 2.03 s bitten, the 8th room ping by S0 + 2.06 s, the read at or after
  S0 + 2.08 s), and any constant between the room's tick numbers and the server's ns (the model gives the client the
  exact tick-to-ns map; scenario.rs maps a room tick t to `due_ns(t - tick_start)`, :1176-1178, which a real ClockSync
  must apply too).
- **C12, draws.** One `random.Random(seed)` per seed (1, 2, 3); per connect a fixed number of draws in a fixed order
  (h, phi, R0; then per own ping d_up, d_down, u_m, the two loss draws; then per room ping d_room and its loss draw),
  so every variant (phase, arrivals, stamp, estimator, loss) is computed on the same draws: paired comparisons.
- **C13, the bite "shows"** for an estimator under a phase model when its clean (arrivals (1)) p99 is at most
  5,000,000 ns and its bitten (arrivals (2)) p99 is above it; judged pooled (3,000) and also per seed (all three seeds
  agreeing), both printed.
- **C14, the additions** (each marked addition = true): the mean and the lowest-RTT estimators with the exact
  sub-tick stamp (C6; U4F-Q6 (b)'s reference); the newest-sample estimator (own ping 19 alone, the newest to arrive:
  sends 100 ms apart against a 40 ms spread of RTT, so the last sent is the last to arrive) with the tick stamp as
  built and with the exact stamp; and the 2 % loss rows (C11).

## The run (one run, 2026-11-17; the whole printed output, copied)

Command: `python3 -I /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4f-r3/d4b/d4_model_b.py`

```
D4 model B: P2 per direction 50 ms + U[-10, 10] ms (integer ns), 30 Hz, 20 own pings every 3 steps,
8 room pings every 250 ms, 1,000 connects x seeds 1, 2, 3; p99 nearest-rank of |estimate - true|; bar 5 ms.
stamps: tick = the next tick to run, centred (T_{k-1} + T_k) / 2; exact = the server's arrival ns to the us;
room-exact = the room ping's own sent_ns; tick-uncentred = T_k (sensitivity only).
phase (i) uniform per sample; (ii) one seeded phase per connect. arrivals clean = as built (datagram
arrival); bitten = moved to the client step that reads it (PM-0001-47/tick-quantised). loss = P2's 2 %.
all values in ms; 'med sgn' = the median signed error (estimate - true).

=== loss off (D4's parameters) ===
estimator      stamp          ph  arr    loss  p99 s1  p99 s2  p99 s3  pooled  med sgn     max pass kind
mean           tick           i   clean  off    6.236   6.293   5.743   6.134   -0.047   8.941 no   main
mean           tick           i   bitten off   14.067  14.095  14.572  14.276    8.267  17.742 no   main
mean           tick           ii  clean  off    8.536   9.017   8.921   8.831    0.158  11.296 no   main
mean           tick           ii  bitten off   17.757  17.651  18.034  17.822    8.545  20.059 no   main
lowest-RTT     tick           i   clean  off   18.311  18.316  18.493  18.391    0.105  21.369 no   main
lowest-RTT     tick           i   bitten off   23.948  23.599  24.167  23.997    3.135  26.086 no   main
lowest-RTT     tick           ii  clean  off   18.342  17.859  18.384  18.281    0.175  20.846 no   main
lowest-RTT     tick           ii  bitten off   23.778  24.003  23.689  23.811    3.268  26.380 no   main
e/mean-of-8    room-exact     i   clean  off   12.634  13.091  12.491  12.712    7.292  15.291 no   main
e/mean-of-8    room-exact     i   bitten off   25.424  26.250  25.165  25.833   16.807  28.952 no   main
e/mean-of-8    room-exact     ii  clean  off   12.634  13.091  12.491  12.712    7.292  15.291 no   main
e/mean-of-8    room-exact     ii  bitten off   25.424  26.250  25.165  25.833   16.807  28.952 no   main
e/lowest-of-8  room-exact     i   clean  off    6.430   6.203   6.375   6.424   -0.716  12.118 no   main
e/lowest-of-8  room-exact     i   bitten off   15.817  15.508  15.971  15.790    2.390  20.619 no   main
e/lowest-of-8  room-exact     ii  clean  off    6.430   6.203   6.375   6.424   -0.716  12.118 no   main
e/lowest-of-8  room-exact     ii  bitten off   15.817  15.508  15.971  15.790    2.390  20.619 no   main
mean           exact          i   clean  off    2.208   2.406   2.402   2.312   -0.008   3.456 yes  addition
mean           exact          i   bitten off   12.115  12.166  12.298  12.194    8.346  14.880 no   addition
mean           exact          ii  clean  off    2.208   2.406   2.402   2.312   -0.008   3.456 yes  addition
mean           exact          ii  bitten off   12.115  12.166  12.298  12.194    8.346  14.880 no   addition
lowest-RTT     exact          i   clean  off    4.740   4.478   4.553   4.567   -0.002   6.705 yes  addition
lowest-RTT     exact          i   bitten off    9.932   9.914   9.918   9.922    4.169  10.000 no   addition
lowest-RTT     exact          ii  clean  off    4.740   4.478   4.554   4.568   -0.002   6.705 yes  addition
lowest-RTT     exact          ii  bitten off    9.932   9.914   9.917   9.922    4.169  10.000 no   addition
newest         tick           i   clean  off   21.831  22.208  22.360  22.208   -0.119  24.643 no   addition
newest         tick           i   bitten off   34.103  32.756  33.055  33.497    8.437  41.133 no   addition
newest         tick           ii  clean  off   21.461  21.718  21.725  21.718    0.239  24.927 no   addition
newest         tick           ii  bitten off   33.996  33.921  32.852  33.733    8.290  41.070 no   addition
newest         exact          i   clean  off    8.974   8.846   9.100   8.904    0.052   9.804 no   addition
newest         exact          i   bitten off   23.854  23.540  23.924  23.759    8.386  25.926 no   addition
newest         exact          ii  clean  off    8.974   8.845   9.100   8.903    0.051   9.804 no   addition
newest         exact          ii  bitten off   23.855  23.540  23.924  23.759    8.387  25.926 no   addition
mean           tick-uncentred i   clean  off   22.903  22.153  21.811  22.467  -16.714  25.180 no   sensitivity
mean           tick-uncentred i   bitten off   14.688  14.036  14.143  14.473   -8.399  17.119 no   sensitivity
mean           tick-uncentred ii  clean  off   24.603  25.076  24.999  24.977  -16.508  27.963 no   sensitivity
mean           tick-uncentred ii  bitten off   16.986  17.641  17.428  17.445   -8.122  20.960 no   sensitivity
lowest-RTT     tick-uncentred i   clean  off   34.269  34.352  34.745  34.595  -16.561  36.974 no   sensitivity
lowest-RTT     tick-uncentred i   bitten off   36.163  35.657  35.346  35.913  -13.531  40.737 no   sensitivity
lowest-RTT     tick-uncentred ii  clean  off   34.914  34.164  33.515  34.189  -16.491  37.512 no   sensitivity
lowest-RTT     tick-uncentred ii  bitten off   36.230  35.456  36.161  36.070  -13.399  42.001 no   sensitivity

=== loss 2 % each way (every row an addition) ===
estimator      stamp          ph  arr    loss  p99 s1  p99 s2  p99 s3  pooled  med sgn     max pass kind
mean           tick           i   clean  on     6.588   6.279   5.754   6.198   -0.028   9.082 no   addition (loss)
mean           tick           i   bitten on    14.196  14.407  15.095  14.763    8.297  17.852 no   addition (loss)
mean           tick           ii  clean  on     8.579   9.026   8.864   8.824    0.077  11.227 no   addition (loss)
mean           tick           ii  bitten on    17.780  17.653  17.949  17.790    8.505  20.744 no   addition (loss)
lowest-RTT     tick           i   clean  on    18.168  18.316  18.493  18.391    0.073  21.369 no   addition (loss)
lowest-RTT     tick           i   bitten on    23.948  23.599  23.827  23.948    3.216  26.086 no   addition (loss)
lowest-RTT     tick           ii  clean  on    18.342  17.859  18.410  18.289    0.221  20.846 no   addition (loss)
lowest-RTT     tick           ii  bitten on    23.778  24.003  23.689  23.811    3.241  26.380 no   addition (loss)
e/mean-of-8    room-exact     i   clean  on    12.634  12.966  12.554  12.712    7.242  15.291 no   addition (loss)
e/mean-of-8    room-exact     i   bitten on    24.992  26.250  25.621  25.760   16.797  28.952 no   addition (loss)
e/mean-of-8    room-exact     ii  clean  on    12.634  12.966  12.554  12.712    7.242  15.291 no   addition (loss)
e/mean-of-8    room-exact     ii  bitten on    24.992  26.250  25.621  25.760   16.797  28.952 no   addition (loss)
e/lowest-of-8  room-exact     i   clean  on     6.680   6.203   6.424   6.496   -0.747  12.118 no   addition (loss)
e/lowest-of-8  room-exact     i   bitten on    15.872  15.719  16.013  15.971    2.491  20.619 no   addition (loss)
e/lowest-of-8  room-exact     ii  clean  on     6.680   6.203   6.424   6.496   -0.747  12.118 no   addition (loss)
e/lowest-of-8  room-exact     ii  bitten on    15.872  15.719  16.013  15.971    2.491  20.619 no   addition (loss)
mean           exact          i   clean  on     2.299   2.396   2.409   2.396    0.004   3.482 yes  addition (loss)
mean           exact          i   bitten on    12.294  12.533  12.343  12.357    8.375  14.880 no   addition (loss)
mean           exact          ii  clean  on     2.299   2.396   2.409   2.396    0.004   3.482 yes  addition (loss)
mean           exact          ii  bitten on    12.294  12.533  12.343  12.357    8.375  14.880 no   addition (loss)
lowest-RTT     exact          i   clean  on     4.740   4.514   4.871   4.740   -0.009   6.705 yes  addition (loss)
lowest-RTT     exact          i   bitten on     9.913   9.914   9.918   9.918    4.173  10.000 no   addition (loss)
lowest-RTT     exact          ii  clean  on     4.740   4.514   4.872   4.740   -0.009   6.705 yes  addition (loss)
lowest-RTT     exact          ii  bitten on     9.913   9.914   9.917   9.917    4.174  10.000 no   addition (loss)
newest         tick           i   clean  on    21.822  22.208  22.336  22.092    0.080  24.643 no   addition (loss)
newest         tick           i   bitten on    34.586  32.756  33.319  33.524    8.471  41.133 no   addition (loss)
newest         tick           ii  clean  on    21.430  21.718  21.541  21.541    0.221  24.927 no   addition (loss)
newest         tick           ii  bitten on    34.025  33.171  32.895  33.733    8.236  41.070 no   addition (loss)
newest         exact          i   clean  on     8.978   8.846   9.121   8.925    0.073   9.804 no   addition (loss)
newest         exact          i   bitten on    23.854  23.451  23.924  23.705    8.401  25.926 no   addition (loss)
newest         exact          ii  clean  on     8.978   8.845   9.122   8.925    0.072   9.804 no   addition (loss)
newest         exact          ii  bitten on    23.855  23.451  23.924  23.705    8.401  25.926 no   addition (loss)
mean           tick-uncentred i   clean  on    23.159  22.545  21.805  22.629  -16.694  25.749 no   sensitivity (loss)
mean           tick-uncentred i   bitten on    14.810  14.137  14.138  14.573   -8.370  16.850 no   sensitivity (loss)
mean           tick-uncentred ii  clean  on    24.977  25.279  24.961  25.090  -16.590  27.894 no   sensitivity (loss)
mean           tick-uncentred ii  bitten on    17.450  17.640  17.445  17.640   -8.162  20.368 no   sensitivity (loss)
lowest-RTT     tick-uncentred i   clean  on    34.267  34.352  34.745  34.595  -16.594  36.974 no   sensitivity (loss)
lowest-RTT     tick-uncentred i   bitten on    36.163  35.657  35.346  35.913  -13.451  40.737 no   sensitivity (loss)
lowest-RTT     tick-uncentred ii  clean  on    34.914  34.164  33.555  34.255  -16.446  37.512 no   sensitivity (loss)
lowest-RTT     tick-uncentred ii  bitten on    36.230  35.456  36.279  36.161  -13.426  42.001 no   sensitivity (loss)

=== does the tick-quantised bite show? (clean p99 <= 5 ms and bitten p99 > 5 ms) ===
loss off mean           tick           ph i   clean   6.134 bitten  14.276  pooled no  all seeds no  (masked: clean fails)
loss off mean           tick           ph ii  clean   8.831 bitten  17.822  pooled no  all seeds no  (masked: clean fails)
loss off lowest-RTT     tick           ph i   clean  18.391 bitten  23.997  pooled no  all seeds no  (masked: clean fails)
loss off lowest-RTT     tick           ph ii  clean  18.281 bitten  23.811  pooled no  all seeds no  (masked: clean fails)
loss off e/mean-of-8    room-exact     ph i   clean  12.712 bitten  25.833  pooled no  all seeds no  (masked: clean fails)
loss off e/mean-of-8    room-exact     ph ii  clean  12.712 bitten  25.833  pooled no  all seeds no  (masked: clean fails)
loss off e/lowest-of-8  room-exact     ph i   clean   6.424 bitten  15.790  pooled no  all seeds no  (masked: clean fails)
loss off e/lowest-of-8  room-exact     ph ii  clean   6.424 bitten  15.790  pooled no  all seeds no  (masked: clean fails)
loss off mean           exact          ph i   clean   2.312 bitten  12.194  pooled yes all seeds yes (yes)
loss off mean           exact          ph ii  clean   2.312 bitten  12.194  pooled yes all seeds yes (yes)
loss off lowest-RTT     exact          ph i   clean   4.567 bitten   9.922  pooled yes all seeds yes (yes)
loss off lowest-RTT     exact          ph ii  clean   4.568 bitten   9.922  pooled yes all seeds yes (yes)
loss off newest         tick           ph i   clean  22.208 bitten  33.497  pooled no  all seeds no  (masked: clean fails)
loss off newest         tick           ph ii  clean  21.718 bitten  33.733  pooled no  all seeds no  (masked: clean fails)
loss off newest         exact          ph i   clean   8.904 bitten  23.759  pooled no  all seeds no  (masked: clean fails)
loss off newest         exact          ph ii  clean   8.903 bitten  23.759  pooled no  all seeds no  (masked: clean fails)
loss on  mean           tick           ph i   clean   6.198 bitten  14.763  pooled no  all seeds no  (masked: clean fails)
loss on  mean           tick           ph ii  clean   8.824 bitten  17.790  pooled no  all seeds no  (masked: clean fails)
loss on  lowest-RTT     tick           ph i   clean  18.391 bitten  23.948  pooled no  all seeds no  (masked: clean fails)
loss on  lowest-RTT     tick           ph ii  clean  18.289 bitten  23.811  pooled no  all seeds no  (masked: clean fails)
loss on  e/mean-of-8    room-exact     ph i   clean  12.712 bitten  25.760  pooled no  all seeds no  (masked: clean fails)
loss on  e/mean-of-8    room-exact     ph ii  clean  12.712 bitten  25.760  pooled no  all seeds no  (masked: clean fails)
loss on  e/lowest-of-8  room-exact     ph i   clean   6.496 bitten  15.971  pooled no  all seeds no  (masked: clean fails)
loss on  e/lowest-of-8  room-exact     ph ii  clean   6.496 bitten  15.971  pooled no  all seeds no  (masked: clean fails)
loss on  mean           exact          ph i   clean   2.396 bitten  12.357  pooled yes all seeds yes (yes)
loss on  mean           exact          ph ii  clean   2.396 bitten  12.357  pooled yes all seeds yes (yes)
loss on  lowest-RTT     exact          ph i   clean   4.740 bitten   9.918  pooled yes all seeds yes (yes)
loss on  lowest-RTT     exact          ph ii  clean   4.740 bitten   9.917  pooled yes all seeds yes (yes)
loss on  newest         tick           ph i   clean  22.092 bitten  33.524  pooled no  all seeds no  (masked: clean fails)
loss on  newest         tick           ph ii  clean  21.541 bitten  33.733  pooled no  all seeds no  (masked: clean fails)
loss on  newest         exact          ph i   clean   8.925 bitten  23.705  pooled no  all seeds no  (masked: clean fails)
loss on  newest         exact          ph ii  clean   8.925 bitten  23.705  pooled no  all seeds no  (masked: clean fails)

=== does the newest-sample bite show? (the estimator passes clean; the newest sample, same stamp, clean, fails) ===
loss off mean        tick   ph i   estimator   6.134 newest  22.208  shows: masked: estimator fails
loss off mean        tick   ph ii  estimator   8.831 newest  21.718  shows: masked: estimator fails
loss off lowest-RTT  tick   ph i   estimator  18.391 newest  22.208  shows: masked: estimator fails
loss off lowest-RTT  tick   ph ii  estimator  18.281 newest  21.718  shows: masked: estimator fails
loss off mean        exact  ph i   estimator   2.312 newest   8.904  shows: yes
loss off mean        exact  ph ii  estimator   2.312 newest   8.903  shows: yes
loss off lowest-RTT  exact  ph i   estimator   4.567 newest   8.904  shows: yes
loss off lowest-RTT  exact  ph ii  estimator   4.568 newest   8.903  shows: yes
loss on  mean        tick   ph i   estimator   6.198 newest  22.092  shows: masked: estimator fails
loss on  mean        tick   ph ii  estimator   8.824 newest  21.541  shows: masked: estimator fails
loss on  lowest-RTT  tick   ph i   estimator  18.391 newest  22.092  shows: masked: estimator fails
loss on  lowest-RTT  tick   ph ii  estimator  18.289 newest  21.541  shows: masked: estimator fails
loss on  mean        exact  ph i   estimator   2.396 newest   8.925  shows: yes
loss on  mean        exact  ph ii  estimator   2.396 newest   8.925  shows: yes
loss on  lowest-RTT  exact  ph i   estimator   4.740 newest   8.925  shows: yes
loss on  lowest-RTT  exact  ph ii  estimator   4.740 newest   8.925  shows: yes

=== diagnostics ===
own samples whose Ping reached the room exactly at a tick instant (phase ii, all seeds): 0
loss addition: own samples kept 57554 of 60000 (95.92 %), room pings kept 23527 of 24000 (98.03 %)
connects with no own sample / no room ping (loss on, counted per arrivals): 0 0
bitten lowest RTT in ticks, per connect (loss off, all seeds): {3: 3000}
```

## Reading (mine; the advisor weighs it against model A)

1. **As built (U4F-Q6 (a)) no estimator passes -47 at P2, under either phase model.** Mean of samples: pooled p99
   6.134 ms (i) and 8.831 ms (ii); lowest-RTT 18.391 / 18.281 ms (one sample keeps its whole ±16.7 ms quantisation).
   NA-4 holds: phase-locking moves the mean from 6.1 to 8.8 ms (max 11.3 ms), the per-connect bias of up to about
   ±6.7 ms plus the samples' spread. Even uniform phases (i) fail the mean, by 1.1 ms: 20 samples of a 33.3 ms
   quantisation leave a spread of about 2.2 ms (σ), p99 about 6 ms. Analytic check before the run: σ_mean ≈
   sqrt(33.3²/12/20 + 16.7/20) ≈ 2.34 ms -> p99 ≈ 6.0 ms; the run gives 6.13.
2. **(e) as the brief words it fails both ways.** e/mean-of-8: 12.712 ms, median +7.29 ms (half the lowest of 20 RTTs,
   about 42.7 ms, is below the 50 ms mean one-way delay: the bias I expected, +7.4 ms). e/lowest-of-8: 6.424 ms,
   median -0.72 ms (two lower envelopes; one direction's jitter left uncancelled). Identical under (i) and (ii) by
   construction (C8). Not run, an estimate only: an (e) with mean RTT / 2 for the one-way delay would centre the bias
   but keep about 2.2 ms σ (8 one-way samples, ±10 ms each), p99 about 5.8 ms: still above 5 ms.
3. **(b) (an exact sub-tick server stamp, the additions) passes, and both bites show.** Mean: 2.312 ms clean (all
   seeds 2.2-2.4), bitten 12.194 ms; lowest-RTT: 4.567 ms clean (seeds 4.740, 4.478, 4.553: a thin 0.26-0.52 ms
   margin; with 2 % loss 4.740 pooled and seed 3 4.871), bitten 9.922 ms. Newest-sample (exact): 8.904 ms, so its
   bite shows against both clean estimators. Same under (i) and (ii) (the exact stamp has no phase).
4. **TE-6's worry, under (b):** the lowest-RTT pick's tick-quantised bite still shows (9.92 ms) because every bitten
   RTT collapses to exactly 3 ticks (3,000 of 3,000 connects: 100,000,000 ns on the floor grid), so a strict-`<`
   first-minimum pick takes the oldest sample with true RTT at most 100 ms, whose error is 50 ms - d_up, spread over
   ±10 ms. That rests on the filter's tie rule: a filter that averages every tied lowest sample (one reading of ADR
   0001:240's "over the lowest-RTT samples") was not modelled; its bitten bias would be about +3.3 ms (my estimate:
   E[50 - d_up | RTT <= 100 ms]) with a smaller spread, so 5.3's short run (TE-6) is still needed.
5. **Under the tick stamp every bite is masked** (the clean estimator already fails), the newest-sample bite included.
6. **The half-tick bias must be centred:** the uncentred stamp (T_k) gives a -16.5 to -16.7 ms median (sensitivity
   rows), so a builder that takes the next tick's due instant as the sample's server instant fails by about P/2.
7. **A sample exactly at a tick instant:** 0 of 60,000 under (ii) (integer ns, a 20 ms jitter window); the rule
   (stamped with that tick, RQ8's I/O before the tick) is in the model but does not move a number.
8. **Loss (addition):** 2 % each way moves no verdict; the largest change is the lowest-RTT exact row's seed 3
   (4.553 -> 4.871 ms).

What it means for U4F-Q6 (data for the advisor, not a ruling): of (a), (b) and (e), only (b) lets -47 pass at P2 with
both of its bites showing; among (b)'s estimators the mean has the margin (2.3 ms against 5 ms), while a single
lowest-RTT sample passes by 0.13-0.52 ms per seed.

## Progress

- Blocks 1-5 complete (reads, choices, script, one run, reading). Returned the schema. Nothing left open in this task.
