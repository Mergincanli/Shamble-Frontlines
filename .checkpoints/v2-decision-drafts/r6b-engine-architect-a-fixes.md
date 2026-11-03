# r6b engine-architect (a) fixes: ADR 0002 and ADR 0005 (outside the ability section and its GPU rules)

### FIX EA-A-1
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
   - **No arm64 leg** in the determinism job: the first fleet is x86-64 (ADR 0006, part 5), and native AArch64 joins the
     hash test before ADR 0006 may pick Arm hosts (ADR 0004).
AFTER:
   - **No arm64 leg** in the determinism job: the first fleet is x86-64 (ADR 0006, part 5), and native AArch64 joins the
     hash test before ADR 0006 may pick Arm hosts (ADR 0004). The netcode proof test's room server runs on the x86-64
     reference server and its only Arm machine is the load generator (ADR 0001's LG, CAX21 VMs), so the bot clients'
     aarch64 build is that spike's own; the playtest soak's bot VM is named when the first soak is planned, and if it
     is Arm, `devtools` gains an aarch64 build target then (a build, not a determinism leg), with the soak's desyncs
     judged by ADR 0001's rule that no pass mark's verdict rests on native aarch64 agreeing with x86-64 or Wasm.
END

### FIX EA-A-2
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
    the step-10 review). What is not known is the macOS job's wall time against CI's 10-minute mark (PM-0002-56).
AFTER:
    the step-10 review). What is not known is the macOS job's wall time against CI's 10-minute mark (PM-0002-56). The
    netcode proof test first checks whether those runners' Safari is new enough for WebTransport and reaches the test
    servers (Mustafa's answer to MACHINES-TIMING, 2026-11-03, in SPIKE_PLAN's machines paragraph); that check is the
    spike's own and leaves this question open, since it asks about pull requests.
END

### NOTE EA-A-3
no change: (docs/adr/0002-code-structure-build-toolchain.md) the rest of ADR 0002's own text, checked against his answers (line numbers on disk, before EA-A-1 and
EA-A-2), stays true. (1) 519-528, the soak: "Before each playtest one 3-hour soak runs on the reference server ... (its
VM: the bench's path below)": still true; its bot VM gets the one sentence of EA-A-1 at the arm64 leg, so 519-528 gain
none (condition 7: at most one sentence). (2) 540-548, the bench: "a CCX23 created per bench session in the spikes' own
Hetzner project and deleted after it", 20 x 0.1386 = 2.77 EUR (R6-HOST-F15, excluding VAT), and his SPIKE-MACHINES
words "I create and delete the VMs myself for each run; ...": all true under "a CCX23 for the game server" and "They'll
be created fresh on run day" (HETZNER-CORRECTION-2); every server run uses the CCX23 (HETZNER-ANSWERS-2, the option
text the session's), so no CCX13 and no measurement-only rule applies; the text names no location, no address and no
CX33. (3) 935, "Revisit when: ... ADR 0006 picks Arm hosts (an arm64 determinism leg)": true; an Arm load generator is
not "an ARM host" in ADR 0004 part 7's sense (the advisor's reading, 2026-11-03). (4) 1587, R5-CI-OQ7 "answered: no
arm64 leg while the fleet is x86-64": true (RS is the x86-64 CCX23). (5) 1027-1041, OQ-0002-5: stays open (EA-A-2 adds
only the pointer). (6) 881 and 945, Cloudflare Pages and Vite's dev server on localhost: no Hetzner machine. Greps over
ADR 0002 on disk: 0 hits for CX33, CCX13, CAX21, nbg1, hel1, fsn1, Falkenstein, Nuremberg, Helsinki, OQ-0006-1,
OQ-0006-2, R6-HOST-OQ1, R6-HOST-OQ2, "development server", "register" and shamblefrontlines; no "\bSH\b", "\bMV\b" or
"\bDS\b" machine; so nothing in ADR 0002 names the CX33, a location, the reference server's order, the domain's
registration or "the development server" for a Hetzner machine.
END

### NOTE EA-A-4
no change: (docs/adr/0002-code-structure-build-toolchain.md; test-engineer's text, the pass marks; named here, no change asked) 1159-1170, the key's MAC ("a macOS
machine with the latest Safari, for the rows that name Safari; the draft names none, so its model is recorded with each
result"): true, since a GitHub macOS runner stands as MAC for the netcode proof test's Safari rows only, and the later
spikes' Safari rows (the boundary spike's among them) are each spike's own plan's (AMENDMENT 2, the advisor's reading,
2026-11-03); 1341, "no arm64 leg": true (EA-A-1 keeps it); PM-0002-72 (1296) and PM-0002-76 (1300), tick times only
from one vCPU of the reference server and the 3-hour soak "on the reference server ... a CCX23 per session": true under
"a CCX23 for the game server" and "created fresh on run day" (HETZNER-CORRECTION-2); PM-0002-73 (1297) plays against
"the staging game server" and names no plan, so OQ-0006-22 (its x86 plan, infra-engineer) leaves it true. None names
the CX33, a location, an address, SH, MV or DS.
END

### NOTE EA-A-5
no change: (docs/adr/0005-content-abilities-assets.md) ADR 0005's text outside the ability section and its GPU rules (the frame, "Content data", "The asset
pipeline") names no Hetzner machine. Greps over ADR 0005 on disk: 0 hits for CX33, CCX13, CAX21, nbg1, hel1, fsn1,
Falkenstein, Nuremberg, Helsinki, arm64, aarch64, OQ-0006-1, OQ-0006-2, R6-HOST-OQ1, R6-HOST-OQ2, "register",
shamblefrontlines and the machines SH, MV, DS and LG; "Hetzner" and "CCX23" only at 2206 (test-engineer's key, EA-A-6).
"The development server" at 411, 417 and 744 (CD-7 and its spike item 5, mine) and at 2283, 2290, 2379 and 2382
(test-engineer's PM-0005-10, PM-0005-17 and the content checklist) is the native development server on the minimum
laptop or on Mustafa's PC ("ML: the development server native"; "MP: the development server on his PC"), never a
Hetzner machine, so neither his first correction (superseded, never quoted in the drafts) nor his later answers touch
it. 1053 ("the reference server", game-designer's ability section) names no plan or location.
END

### NOTE EA-A-6
no change: (docs/adr/0005-content-abilities-assets.md; test-engineer's text, the pass-mark key; named here, no change asked) 2204-2213. RS, "a Hetzner CCX23,
Linux, one EU location": true, the CCX23 in Falkenstein (`fsn1`) being one EU location ("both test servers will be in
Falkenstein, not Nuremberg: a CCX23 for the game server and a CAX21 for the 48 simulated players", HETZNER-CORRECTION-2);
naming `fsn1` there is test-engineer's choice, not needed for truth. MAC, "SPIKE_PLAN's macOS machine: the stable
Safari on the day of the run, its version recorded": true for this record's spikes (content, abilities, assets and
the look proof test's asset marks), since the GitHub macOS runner stands as MAC only for the netcode proof test's
Safari rows, and a row ADR 0003's look proof test needs stays on the borrowed Mac (AMENDMENT 2, the advisor's reading,
2026-11-03); PM-0005-2's "MAC: Safari (`JSON.parse`)" is the content spike's own plan's.
END

