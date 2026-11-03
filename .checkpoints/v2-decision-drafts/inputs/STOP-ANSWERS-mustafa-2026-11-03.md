# Answers to the stop (Mustafa, 2026-11-03, about 12:28 Amsterdam, in the session's chat; word for word)

Answers to the stop (Mustafa, 2026-11-03; the Advisor's calls are marked).

SPIKE-ORDER: (b) accept with one change. The six drafts are the basis for the spikes, in SPIKE_PLAN's order, but wave 1 runs one heavy spike at a time: the netcode proof test first, then the sim core spike (then the 60 Hz run), then the boundary spike, then the look proof test once its models exist. The preview hosting check may run alongside any of them. Reason: seven usage-limit cuts in three days.
TICK-60-HEADROOM: confirm, with one addition: 60 Hz also needs the goal budget to hold at the rooms per VM measured at 60 Hz.
TICK-60-SETUP (Advisor): (a), as an early signal only. The final 60 Hz call is measured again with the real sim at slice 6.
NET-PROFILES (Advisor): yes. "100 ms" is P2: 100 ms round trip, ±20 ms jitter, 2 % loss (VISION_V2 §4's "about 100 ms of ping"). Latency and jitter split half each way; loss as P2 defines it.
REWIND-CAP: no value now. Rule: the cap covers the honest rewind the proof test measures at P2, rounded up; beyond the cap, the victim's view wins. Aim for the smallest input buffer that holds.
SPIKE-MACHINES: a separate Hetzner project. I create and delete the VMs myself for each run; sessions get SSH access only and no Hetzner token until the security baseline. VMs exist only during a run. Certificates: a private hash-pinned certificate for WebTransport; for the WebSocket leg, a domain I register. Test PC: my own, in Brave; the CX33 runs the simulated clients.
LOOK-TEST-INPUTS: yes, the asset research's export and slot-naming part runs before the look proof test. Route: Blender (Spline's skins and named slots are unproven). I make the test models with AI help in Blender; blockouts with a skin, named colour slots and LOOK_V2's proportions are enough.
TOY-BOX: reword both to point at LOOK_V2, in a small kit change-set after PR #30 is updated, together with the checkpoint anti-waste rules.
OQ-0002-8 (Advisor): (a).
OQ-0005-1 (Advisor): the client's data, keyed by the same id; the content check fails if a simulation id has no client entry.
Also the Advisor's, at the stop: OQ-0001-14: yes, a VISION_V2 §3 or §4 trace counts, marked as such. ROOM-CRASH: end the match, as proposed, and keep the crash and match records. OQ-0003-17: at most 6 of 600 presented frames may miss a refresh with the very large units on screen (recommended desktop, High); the p95 rows stay as diagnostics; I confirm this reading. OQ-0003-20: judge after 15 minutes of play. OQ-0005-AB12: tranek's write-up is accepted for concepts; any specific claim a record relies on is checked against Epic's documentation once it can be read. OQ-0005-AB13: scripting comes back only when a designed ability can't be built from data.
LATER: a design session after the netcode proof test.

Next: record these, mark PR #33 ready, and once it's approved, squash-merge it so the 31 MB of scratch build output never reaches main's history. Then the netcode proof test as its own change-set, plan first.
