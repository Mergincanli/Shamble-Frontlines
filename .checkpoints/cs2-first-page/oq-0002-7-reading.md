# OQ-0002-7, the by-day reading: is wasm-bindgen's new home healthy for a multi-year project?

_Written by the main session on 2026-11-18 21:08 UTC (from the clock). The plan's step 0 (`plan.md`, section 8). The question, its
one researched option and its decider are ADR 0002's (`docs/adr/0002-code-structure-build-toolchain.md` :1085-1093):
"Decider: a by-day reading of its maintenance issue (#4533) and the guide's governance page"; "What decides it: what
those two pages say about maintainers and release policy." Read by a fetch agent (Opus 5.5) on 2026-11-18 between
21:05 and 21:07 UTC; the release dates below it were read by the main session from the registry directly at 21:07 UTC.
The agent's whole answer is in this session's record; the quotes here are the pages' words as it gave them._

## What was read
- A. The maintenance issue: https://github.com/wasm-bindgen/wasm-bindgen/issues/4533 ("New home, new maintainers").
- B. The guide's governance page: https://wasm-bindgen.github.io/wasm-bindgen/contributing/governance , with its two
  neighbours, the Team page (`contributing/team.html`) and the Publishing page (`contributing/publishing.html`).
- C. The releases page: https://github.com/wasm-bindgen/wasm-bindgen/releases (first page).
- D. The registry: https://crates.io/api/v1/crates/wasm-bindgen (versions and dates).

## Maintainers
- A is open; alexcrichton opened it on 2025-07-14; its newest comment is of 2025-09-02 (guybedford).
- A, the issue's body: "The current primary maintainer of this repository is @daxpedda"; "At this time the new set of
  maintainers will be @daxpedda and @guybedford with Cloudflare"; the author steps back ("I will personally be
  removing myself from all permissions").
- B's Team page lists three core members (daxpedda, Ingvar Stepanyan, Guy Bedford) and eight collaborators. Neither
  page names Cloudflare.
- B: "The project is not owned by any specific entity or group and belongs to the community as a whole."

## How it is run
- B: "The core team is responsible for project governance and moderation, operating based on a majority vote."
- B: "All pull requests must be reviewed and approved of by at least one other collaborator".
- B: core members are chosen by a core team vote; taking part means "regular attendance at monthly core meetings".
- One odd detail the agent noted: B says it covers the "wasm-bindgen organization", but that link points to
  github.com/rustwasm.

## Release policy
- **None is written on A or B.** No schedule, no compatibility or versioning promise. The Publishing page gives the
  steps of a release only.
- C, the notes of 0.2.129: "Library MSRV bumped from 1.77 to 1.81, per the 2-year MSRV policy." That is the one
  policy found, and it is about the Rust version a release needs.
- D, what the project has done (not a promise): twelve releases between 2026-03-31 (0.2.117) and 2026-10-30
  (0.2.130), the last five on 2026-06-24, 2026-08-08, 2026-09-04, 2026-09-25 and 2026-10-30. 0.2.130 is the newest
  and is marked "Latest" on C.
- ADR 0002's own text says compatibility broke twice in 2026, in 0.2.123 and 0.2.128 (:1086-1088). Nothing read today
  says that will not happen again.

## Anything that says unmaintained, deprecated or moving again
- Nothing found. A records the move of July 2025 to the new organisation (the Rust and WebAssembly Working Group
  "no longer exists") and a call for more maintainers at that time; B and the Team page say new collaborators are
  welcome.

## What the reading says, against the decider's two words
- **Maintainers: yes.** Three core members by name, a written way of deciding, reviews required, and releases about
  monthly through 2026.
- **Release policy: not written,** apart from the 2-year rule for the Rust version. So the project gives no promise
  that a later 0.2.x release keeps working with our code or with an older command-line tool.
- What that means for us: option (a) of OQ-0002-7, the only one researched ("keep wasm-bindgen behind the exact pin
  and the CI mismatch check (part 2), with every bump its own change-set"), is exactly the guard for a missing
  promise. The plan pins `=0.2.130` and its build script refuses any other tool version (PM-0002-18).
- **The plan's own rule** (step 0): "If the pages show no active maintainer or no release policy, the change-set stops
  before step 2 and tells him." No release policy is written, so this goes to Mustafa before step 2, as question
  WB-HEALTH, with the recommendation to go on behind the exact pin.

## What was not read, or only in part
- The guide's index showed no table of contents; the agent found the governance page through a summary of the guide's
  print version, then read the three pages in full.
- The releases page was read word for word up to 0.2.128; the dates below that come from the registry (D), not from C.
- No other binding generator was looked at (ADR 0002: "no other binding generator was researched").
