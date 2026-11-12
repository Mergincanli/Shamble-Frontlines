# U4e round 1: engine-architect review (module boundaries, dependency rules)

Read copy: /home/user/Shamble-Frontlines/.gate/u4e-r1 (97bb9d3 + U4e's snapshot patch). Base copy: /home/user/Shamble-Frontlines/.gate/u4e-prefix (1e8c188).
Packet: u4e-packet.md, u4e-packet-tests.md. Read only; I run no cargo command (the test-engineer plants in the builder's copy).
Memory: none found (no agent-memory folder for engine-architect).

## Plan
1. Read the packet's "judged against" half (§5 U4e, §4.1, §4.2, §4.10, W1-S1, W1-N2, W1-PUB, W1-U4E, W1-TARGET, advisor-u4e-brief whole, B2.1, B5, Mustafa's words, carries) and note the rules that bind the boundaries.
2. Crate edges: each Cargo.toml (room, server, client, sim, link, proto), check.mjs's deps step and its rules; what each crate `use`s from the others (grep `use npt_`), npt-client naming only `Channel` from npt_link.
3. The seam (§4.2): diff the seam files between u4e-prefix and u4e-r1; carry 26's route.
4. Public items: every `pub` item added by the diff (from the diff, not the builder's list) vs plan §4.7, §4.1, W1-PUB; server path (crate-root re-exports with private modules); `Scene::effect_rows` as carry 51's one public home.
5. Shapes for U4f, U4g, U4h, U4r, U6: counters, Room<S> generics, ClientEndpoint, Encoder; anything built that no plan line or ruling asks for.
6. Findings, each written here as found; runs I would want named for the main session.

## Findings (a line after each)
