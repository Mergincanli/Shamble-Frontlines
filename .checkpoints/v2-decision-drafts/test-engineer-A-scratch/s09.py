SUBS = [
("per profile P1 to P4 and B1, on L1 and on L2: measured loss, mean delay and jitter over 100,000 packets within 5 % (relative) of the profile;",
 "per profile P1 to P4 and B1, on L1 and on L2, in each direction: measured loss, mean one-way delay and jitter over 100,000 packets within 5 % (relative) of the profile's per-direction values (half its latency, a uniform jitter of ± half its jitter: the key's split, OQ-0001-16), and the round trip's mean within 5 % of the profile's latency;"),
("on L2, the injected delay set to the profile's RTT minus the path's measured base RTT (median of 10 s of pings at session start; proposed)",
 "on L2, the injected delay in each direction set to half of the profile's RTT minus the path's measured base RTT (median of 10 s of pings at session start; proposed), each direction's delays logged at the impairment layer"),
]
