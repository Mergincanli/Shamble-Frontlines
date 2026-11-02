SUBS = [
("  Latency is the round trip. L1 runs",
 "  Latency is the round trip, and each direction carries half the profile's latency and a uniform jitter of ± half its\n  jitter, so the round trip keeps the profile's mean and range (part 13; OQ-0001-16, the Advisor's, before the test\n  starts; the half split of latency is R5-TEST-REC4's, r5-toolchain-quality.md lines 1455–1456, proposed there, and\n  the jitter's split and its uniform shape are part 13's proposal): at P2, 50 ± 10 ms each way. A packet's expected\n  arrival is its send time plus that one-way delay, and every lateness in these marks is counted from it. L1 runs"),
]
