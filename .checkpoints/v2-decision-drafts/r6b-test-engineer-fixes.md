# r6b test-engineer fixes (authors round, plan r6)

### FIX F-0001-KEY-RS-LG
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
- **Machines.** **RS**, the reference server: a Hetzner Cloud CCX23 (4 dedicated vCPU, 16 GB, x86-64 AMD) in one EU
  location (r6-infrastructure.md, R6-HOST-REC2, lines 645–650; [the reference server](../research/GLOSSARY.md#the-reference-server));
  every server time mark is the work's elapsed time on one vCPU of it with no other room on that vCPU
  ([the measuring unit](../research/GLOSSARY.md#the-measuring-unit-one-vcpu)): the room's tick thread is pinned alone to
  one vCPU and the transport runtime runs on the other three (proposed placement, so the measured vCPU holds only the
  tick), in every server time mark but the rooms-per-VM rows PM-0001-27 and PM-0001-63, which run unpinned as ADR
  0006's PM-0006-13 sets it (eight tick threads cannot each be alone on one of four vCPUs). **SH**, the shared CX33
  (4 vCPU, 8 GB) beside it, for the rooms-per-VM mark only (R6-HOST-REC2). **LG**, the load
  generator: one or more VMs in the same location (CX33s: "the CX33 runs the simulated clients", SPIKE-MACHINES,
  Mustafa, 2026-11-03) running the headless bot clients, which
  speak the real protocol (r5-toolchain-quality.md, R5-TEST-REC5, lines 1475–1480), never RS or SH; as many as a
  5-minute calibration run at the session's load shows keep each under 80 % of its CPU, their count in the run plan
  and their hours in the spike's bill (ADR 0006's spike item 2 and its LG key, PM-0006-18); on LG's addresses the
  per-address connection cap (part 10) is raised to the run's session count and recorded, except in PM-0001-25's run
  with the cap on (proposed: clients on RS would take CPU from the measured work; one 4-vCPU VM may not carry 384
  encrypted sessions, and the cap would refuse most sessions from one address: the infra review of 2026-11-02,
  findings 9 and 10). **ML, RD, MAC** as ADR 0003's pass marks name them (r3-rendering.md, R3-BUDGET-REC1, lines
AFTER:
- **Machines.** **RS**, the reference server: a Hetzner Cloud CCX23 (4 dedicated vCPU, 16 GB, x86-64 AMD) in
  Falkenstein (`fsn1`), "a CCX23 for the game server" (Mustafa, 2026-11-03; ADR 0006's RS key quotes his words), the
  machine every server row runs on (r6-infrastructure.md, R6-HOST-REC2, lines 645–650;
  [the reference server](../research/GLOSSARY.md#the-reference-server));
  every server time mark is the work's elapsed time on one vCPU of it with no other room on that vCPU
  ([the measuring unit](../research/GLOSSARY.md#the-measuring-unit-one-vcpu)): the room's tick thread is pinned alone to
  one vCPU and the transport runtime runs on the other three (proposed placement, so the measured vCPU holds only the
  tick), in every server time mark but the rooms-per-VM rows PM-0001-27 and PM-0001-63, which run unpinned as ADR
  0006's PM-0006-13 sets it (eight tick threads cannot each be alone on one of four vCPUs). No shared host is measured
  beside RS: the CX33 cannot be ordered in Falkenstein (Mustafa, 2026-11-03). **LG**, the load generator: "a CAX21 for
  the 48 simulated players" (Mustafa, 2026-11-03), Arm64, "so their build targets aarch64" (Mustafa, 2026-11-03, his
  console check), created in `fsn1` on the run day, running the headless bot clients, which speak the real protocol
  (r5-toolchain-quality.md, R5-TEST-REC5, lines 1475–1480), never on RS; a 5-minute calibration run at the session's
  load shows each CAX21 under 80 % of its CPU (ADR 0006's LG key). A run whose bots one CAX21 cannot carry under 80 %
  adds as many CAX21s as its 5-minute calibration shows are needed, inside the cost cap Mustafa OKs for that run
  (LG-COUNT, the option he chose, 2026-11-03); their count goes into the run plan, which names the CAX21 that carries
  MV (ADR 0006's metrics role), and their hours into the spike's bill (ADR 0006's spike item 2 and its LG key,
  PM-0006-18); on LG's addresses, the day's (the VMs are new at every run: ADR 0006's key), the per-address connection
  cap (part 10) is raised to the run's session count and recorded, except in PM-0001-25's run with the cap on
  (proposed: clients on RS would take CPU from the measured work; one 4-vCPU VM may not carry 384 encrypted sessions,
  and the cap would refuse most sessions from one address: the infra review of 2026-11-02, findings 9 and 10). LG's
  clients are native aarch64 builds, so no mark's verdict rests on them agreeing with x86-64 or Wasm: the rows whose
  client side computes in floating point, PM-0001-48 and PM-0001-54, count a mismatch seen on LG only once the same
  inputs replayed on L1 on an x86-64 machine mismatch too (the advisor's reading, 2026-11-03). **ML, RD, MAC** as ADR
  0003's pass marks name them (r3-rendering.md, R3-BUDGET-REC1, lines
END

### FIX F-0001-KEY-MAC
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
  judges, never a measured result, and nothing stands in for RD or MAC (the advisor's reading, 2026-11-03). Mustafa
AFTER:
  judges, never a measured result, and nothing stands in for RD, nor for MAC beyond the macOS runner below (the
  advisor's reading, 2026-11-03). For the netcode proof test's Safari rows, Mustafa put one step first (MACHINES-TIMING,
  2026-11-03): its plan checks "whether GitHub's macOS runners (real Safari, free while the repository is public) have
  a Safari new enough for WebTransport and can reach the test servers"; if they can, "that is the Safari check" (if
  not, "I borrow the Mac before the netcode proof test"), and a runner stands as MAC once the run's results record (i)
  the runner image's Safari version on the day, at least 26.4 (R2-TRN-F7), (ii) real Safari driven through
  safaridriver, not a WebKit build, and (iii) from the runner, one WebTransport session to RS that opens and carries
  datagrams and one WebSocket session on the domain, and then only in yes-or-no and count rows (PM-0001-28's
  four-browser path and library pick, its server then RS, since a runner sits on no local network of ours; PM-0001-14;
  PM-0001-25's Origin; ADR 0006's PM-0006-15), while a row whose limit is a time and ADR 0003's look proof test stay on
  the borrowed Mac, and the runner's workflow is the proof test's plan's, reviewed by security-auditor and
  infra-engineer (the limits: the advisor's reading, 2026-11-03). Mustafa
END

