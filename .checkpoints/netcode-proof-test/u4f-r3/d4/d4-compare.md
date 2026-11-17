# D4: the two ClockSync models compared (before the advisor's check of U4f's brief)

- **Model A** (the main session's): u4f-r3/d4/d4_model.py, sha256 9fdf285e81344574…; output u4f-r3/d4/d4-output.md
  (its `--check` MATCH).
- **Model B** (the netcode-architect's, workflow wf_997ea01f-b4f, written without reading A): u4f-r3/d4b/d4_model_b.py,
  sha256 5c366536240849b5…; its notes and whole output u4f-r3/d4b/d4b-notes.md (63a2a627…); its result saved word for
  word as u4f-r3/d4b-r31-full.json (e8404004…, by u4f-r3/r31_round2_full.py). B works in integer ns as the code does
  (C2), centres the tick stamp as A subtracts half a tick (C4), and draws its own numbers.
- **The noise run:** u4f-r3/d4/d4_noise.py (sha256 daa4545cb2082406…, read only) runs A's own functions on 99 more
  seeds (101-199, 99,000 connects): the p99 over all of them, and the spread (sd) of a p99 over 3,000 connects, as D4
  pools them. Its whole output (2026-11-17, run between 19:05 and 19:28 UTC, the clock read before and after):

```text
d4_model.py sha256 9fdf285e81344574; seeds 101-199 x 1,000 connects, no loss
row                                                        | p99 all  sd 3k |    main       B    gap gap/sd
(a) mean / (i) / as built                                  |   5.975  0.109 |   6.063   6.134 -0.071  -0.46
(a) mean / (i) / bitten                                    |  14.447  0.199 |  14.560  14.276  0.284   1.01
(a) mean / (ii) / as built                                 |   8.867  0.125 |   8.685   8.831 -0.146  -0.82
(a) mean / (ii) / bitten                                   |  17.643  0.180 |  17.264  17.822 -0.558  -2.19
(b) lowest-RTT / (i) / as built                            |  18.322  0.204 |  18.235  18.391 -0.156  -0.54
(b) lowest-RTT / (i) / bitten                              |  24.085  0.258 |  24.324  23.997  0.327   0.90
(b) lowest-RTT / (ii) / as built                           |  18.373  0.173 |  18.569  18.281  0.288   1.17
(b) lowest-RTT / (ii) / bitten                             |  24.096  0.250 |  23.752  23.811 -0.059  -0.17
(e) room pings, half lowest RTT / - / as built             |  12.781  0.138 |  12.847  12.712  0.135   0.69
(e) room pings, half lowest RTT / - / bitten               |  25.721  0.249 |  25.403  25.833 -0.430  -1.22
[add] exact stamp, mean / - / as built                     |   2.346  0.049 |   2.321   2.312  0.009   0.13
[add] exact stamp, mean / - / bitten                       |  11.927  0.144 |  11.807  12.194 -0.387  -1.90
[add] exact stamp, lowest-RTT / - / as built               |   4.856  0.100 |   4.940   4.567  0.373   2.63
[add] exact stamp, lowest-RTT / - / bitten                 |   9.900  0.019 |   9.895   9.922 -0.027  -0.98
gap/sd = (main - B) / (sd x sqrt 2): the gap in units of the spread of a difference of two independent 3,000-connect p99s; within about +-2.5 reads as chance.
```

## Reading

1. **Every verdict agrees.** Both models: with the Pong as built, no D4 estimator passes 5 ms under either phase model;
   (e) as D4 words it fails (its half-lowest-RTT delay is biased by about 7.3 ms); with an exact sub-tick stamp the
   mean passes with a wide margin and the tick-quantised bite then shows; the lowest-RTT sample with an exact stamp
   sits on the bar.
2. **The gaps above 0.1 ms are chance, but one is at the edge.** 13 of the 14 rows lie within ±2.2 of the spread of
   a difference of two independent 3,000-connect p99s. The 14th, the exact-stamp lowest-RTT row as built, is at 2.63:
   B's 4.567 lies 2.9 sd below the 99,000-connect value (4.856), A's 4.940 lies 0.8 sd above it. B's code for it
   (d4_model_b.py:96-121, :140-156: the same first-minimum pick on the same midpoint error, (d_down - d_up) / 2, from
   the same uniform delays) shows no modelling difference; its three seeds all draw low (4.740, 4.478, 4.553). One row
   in 14 this far out has about an 11 % chance (two-sided p about 0.0085 a row). **The value to carry is the
   99,000-connect one: 4.856 ms, 0.14 ms under the bar, about 1.4 sd of a 3,000-connect p99 and less of a 1,000-connect
   judged run's**, so a lowest-RTT filter with an exact stamp would pass or fail a judged -47 run by chance.
3. **For U4F-Q6:** only (b), a sub-tick server time in the Pong, lets -47 pass at P2 by these models, and then only
   with an averaging estimator (the mean of the 2 s: 2.35 ms at 99,000 connects); both of -47's bites show against it
   (tick-quantised 11.9 ms; newest-sample about 8.9 ms with an exact stamp, about 22 ms with the tick stamp).
4. **Not modelled by either:** ADR 0001:240-242's filter as such (drift limit, halving, reset); clock drift; L1's 1 %
   reorder step; the 2 s read instant's cut (B: every sample arrives before it). Loss (P2's 2 %) changes no verdict in
   either model.
