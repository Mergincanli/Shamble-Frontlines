# D4: the main session's ClockSync model, its output (U4f brief rev 3.1, section 11 row D4)

- **Script:** `.checkpoints/netcode-proof-test/u4f-r3/d4/d4_model.py`, sha256
  9fdf285e81344574c545139b53c13d3be2cef56f0d581c39971ac8aa47f3fecc, read only (it writes nothing; the parameters and
  every modelling choice are in its docstring).
- **Run:** `python3 -I .checkpoints/netcode-proof-test/u4f-r3/d4/d4_model.py`, 2026-11-17 between 19:00 and 19:04 UTC
  (the clock read before and after), in the main folder (no build, no cargo). The block below is its whole output;
  `--check` on this file re-computes it and compares byte for byte (its result is noted under the block).
- **Independent second model:** u4f-r3/d4b/ (the netcode-architect's, workflow wf_997ea01f-b4f), written without
  reading this one; the two are compared before the advisor's check.

```text
D4 model (the main session's), script sha256 9fdf285e81344574, Python 3.13.18
P = 33.333 ms; one-way 50 ms + U[-10, 10] ms each direction; 20 pings (every 3 steps); 8 room pings (every 250 ms); 1000 connects x seeds 1-3; bar p99 <= 5.0 ms

pass: no loss (D4's fixed parameters)
estimator                              phase arrivals |  p99 s1  p99 s2  p99 s3 |  pooled    mean     max | <= 5 ms
(a) mean                               (i)   as built |   5.611   6.358   6.058 |   6.063   0.063   7.833 | NO
(a) mean                               (i)   bitten   |  14.036  14.946  14.603 |  14.560  -8.224  16.979 | NO
(a) mean                               (ii)  as built |   8.819   8.603   8.656 |   8.685   0.140  11.218 | NO
(a) mean                               (ii)  bitten   |  17.264  17.635  17.095 |  17.264  -8.147  20.841 | NO
(b) lowest-RTT                         (i)   as built |  17.818  18.688  18.434 |  18.235  -0.031  21.822 | NO
(b) lowest-RTT                         (i)   bitten   |  23.812  24.583  24.459 |  24.324  -3.398  26.266 | NO
(b) lowest-RTT                         (ii)  as built |  18.006  18.886  18.586 |  18.569  -0.037  20.915 | NO
(b) lowest-RTT                         (ii)  bitten   |  23.342  24.134  23.495 |  23.752  -3.020  26.212 | NO
(e) room pings, half lowest RTT        -     as built |  12.985  12.669  12.753 |  12.847  -7.297  16.355 | NO
(e) room pings, half lowest RTT        -     bitten   |  25.099  25.403  25.536 |  25.403 -16.696  29.236 | NO
[add] exact stamp, mean                -     as built |   2.227   2.321   2.343 |   2.321   0.042   2.926 | yes
[add] exact stamp, mean                -     bitten   |  11.739  11.961  11.747 |  11.807  -8.245  14.517 | NO
[add] exact stamp, lowest-RTT          -     as built |   4.921   4.732   4.998 |   4.940   0.010   6.390 | yes
[add] exact stamp, lowest-RTT          -     bitten   |   9.897   9.881   9.921 |   9.895  -3.195   9.995 | NO
[add] newest sample                    (i)   as built |  22.616  21.567  21.832 |  21.936   0.286  25.225 | NO
[add] newest sample                    (i)   bitten   |  34.485  34.671  33.226 |  33.967  -7.971  39.009 | NO
[add] newest sample                    (ii)  as built |  21.945  21.663  21.407 |  21.761   0.414  25.070 | NO
[add] newest sample                    (ii)  bitten   |  32.018  33.766  32.966 |  32.966  -7.843  40.409 | NO
[add] (e) room pings, half mean RTT    -     as built |   5.455   5.577   5.344 |   5.485  -0.047   8.757 | NO
[add] (e) room pings, half mean RTT    -     bitten   |  17.560  18.180  18.189 |  18.036  -8.441  23.381 | NO

pass: P2's 2 % loss per direction ([add], a sensitivity pass)
estimator                              phase arrivals |  p99 s1  p99 s2  p99 s3 |  pooled    mean     max | <= 5 ms
(a) mean                               (i)   as built |   5.896   6.293   6.103 |   6.095   0.054   7.988 | NO
(a) mean                               (i)   bitten   |  14.127  15.027  15.213 |  14.784  -8.232  17.394 | NO
(a) mean                               (ii)  as built |   8.928   8.623   8.787 |   8.802   0.127  11.218 | NO
(a) mean                               (ii)  bitten   |  17.261  17.854  17.217 |  17.403  -8.159  20.841 | NO
(b) lowest-RTT                         (i)   as built |  18.016  18.888  18.748 |  18.434  -0.063  22.355 | NO
(b) lowest-RTT                         (i)   bitten   |  23.812  24.583  24.459 |  24.324  -3.421  26.266 | NO
(b) lowest-RTT                         (ii)  as built |  17.947  18.887  18.586 |  18.571  -0.047  20.915 | NO
(b) lowest-RTT                         (ii)  bitten   |  23.575  24.176  23.554 |  23.835  -3.154  26.212 | NO
(e) room pings, half lowest RTT        -     as built |  12.985  12.744  13.059 |  12.973  -7.245  16.355 | NO
(e) room pings, half lowest RTT        -     bitten   |  25.271  25.973  25.536 |  25.617 -16.677  31.010 | NO
[add] exact stamp, mean                -     as built |   2.332   2.299   2.417 |   2.344   0.041   3.004 | yes
[add] exact stamp, mean                -     bitten   |  11.808  11.942  11.829 |  11.848  -8.245  14.511 | NO
[add] exact stamp, lowest-RTT          -     as built |   5.122   4.783   5.020 |   5.020   0.011   6.390 | NO
[add] exact stamp, lowest-RTT          -     bitten   |   9.867   9.883   9.921 |   9.895  -3.242   9.995 | NO
[add] newest sample                    (i)   as built |  22.616  21.567  21.832 |  21.936   0.308  25.225 | NO
[add] newest sample                    (i)   bitten   |  34.485  33.595  33.226 |  33.827  -7.912  39.009 | NO
[add] newest sample                    (ii)  as built |  21.596  21.663  21.286 |  21.613   0.449  25.070 | NO
[add] newest sample                    (ii)  bitten   |  31.353  33.766  32.966 |  32.416  -7.770  38.351 | NO
[add] (e) room pings, half mean RTT    -     as built |   5.476   5.765   5.424 |   5.584  -0.040   9.066 | NO
[add] (e) room pings, half mean RTT    -     bitten   |  17.771  18.105  18.189 |  18.105  -8.425  25.177 | NO

PM-0001-47/tick-quantised, no-loss pass: the bite shows where the clean p99 is at most 5 ms and the bitten p99 above it
  (a) mean                               (i)  : clean   6.063, bitten  14.560: does not show (the clean run already fails)
  (a) mean                               (ii) : clean   8.685, bitten  17.264: does not show (the clean run already fails)
  (b) lowest-RTT                         (i)  : clean  18.235, bitten  24.324: does not show (the clean run already fails)
  (b) lowest-RTT                         (ii) : clean  18.569, bitten  23.752: does not show (the clean run already fails)
  (e) room pings, half lowest RTT        -    : clean  12.847, bitten  25.403: does not show (the clean run already fails)
  [add] exact stamp, mean                -    : clean   2.321, bitten  11.807: shows
  [add] exact stamp, lowest-RTT          -    : clean   4.940, bitten   9.895: shows
  [add] newest sample                    (i)  : clean  21.936, bitten  33.967: does not show (the clean run already fails)
  [add] newest sample                    (ii) : clean  21.761, bitten  32.966: does not show (the clean run already fails)
  [add] (e) room pings, half mean RTT    -    : clean   5.485, bitten  18.036: does not show (the clean run already fails)
```

**`--check` on this file** (before 19:04 UTC): "MATCH: the saved output equals a fresh run, byte for byte".

## What it says (the main session's reading, for the advisor's check of U4F-Q6)

1. **The Pong as built (U4F-Q6 (a)) passes with none of D4's three estimators, under either phase model.** The best is
   the mean: 6.06 ms pooled p99 with random phases (i), 8.69 ms with one phase per connect (ii), against 5 ms. The
   lowest-RTT sample keeps a whole tick's quantisation (about 18 ms). Under (ii), the realistic case on L1 (SC-1), the
   per-connect bias that averaging cannot remove is about ±6.7 ms at most where the up delay's 20 ms window falls
   inside one tick period (NA-4's estimate, confirmed: mean error 0.14 ms, max 11.2 ms).
2. **(e), the room's own pings (exact send time), fails as D4 defines it:** half the lowest own-ping RTT underestimates
   the one-way delay (mean error -7.3 ms; p99 12.8 ms). With half the mean RTT instead (an addition) it comes to
   5.49 ms: still above 5 ms, since 8 one-way samples in 2 s keep ±10 ms of down jitter each.
3. **An exact sub-tick stamp (U4F-Q6 (b)'s reference, an addition) passes with the mean:** 2.32 ms (2.34 ms with P2's
   2 % loss), and the tick-quantised bite then shows (bitten 11.8 ms, mean shift -8.2 ms, about a quarter tick). The
   lowest-RTT sample with an exact stamp is marginal: 4.94 ms without loss, 5.02 ms with it (fails).
4. **So:** by this model, -47 at P2 needs a sub-tick server time (U4F-Q6 (b)) or another source of exact server time,
   and its estimator must average (the mean of the 2 s of samples, or a filter at least as good); a lowest-RTT filter
   alone sits on the bar. The newest-sample bite fails every estimator's run by a wide margin (about 22 ms), so it
   shows under any of them.
5. **Not modelled:** ADR 0001:240-242's filter as such (drift limit, halving, reset), clock drift (L1's host clocks run
   at the server's rate), reordering (P2's 1 % of adjacent pairs; its effect on arrival times is not modelled), and
   loss beyond the sensitivity pass. These are D4's fixed parameters; the builder's f1 and the short runs p10, p11 and
   p21 measure the real endpoint.
