# Mustafa's words to this session (2026-11-10), word for word: the parts the plan rests on

Source: his prompt to this cloud session (session_019YNMUSc7FA68ujhLgXfEJA, his personal account), 2026-11-10. Only the
parts this plan rests on are copied, each byte for byte.

```text
You are the foundation-toolchain chat for Shamble Frontlines, on Mustafa’s personal account. His words: “the main model and every agent stay Opus 5.5”, and on this account “Fable 5.2 may advise through the advisor tool”, which “is an extra. The recorded plan review by the advisor agent stays required”.
```

```text
The CI job is a new .github/workflows/ job. It installs the pinned toolchain with rustup before any cargo call, checks rustc --version against the pin, and runs cargo test --workspace --locked and npm ci && npm test --workspaces with no network after the install steps. Research how a runner cuts the network before promising PM-0002-37, and leave the gate and smoke jobs unchanged. Keep Appendix B’s rules, and justify narrowing the legacy rules’ globs or leave it to 2.5. List every kit, guard or CI edit with the guard’s protectedRel mark, and ask Mustafa whether his FREEZE-FDN covers each, quoting both texts: the option (a) he picked listed “for 2.2, its CI job, the constraint-scope lines and the rules’ globs”, while his words say it “does not stop the smallest edits the foundation needs to keep the kit true after its own moves”. Protecting the new pins in the guard would be a new guard check, so it waits for the gate step, 2.3. Also ask him where 2.2 sits in his merge order: the order ends at 2.1b, and SCOPE’s acceptance takes effect once the files have moved.
```

```text
Two answers in advance, so you do not have to stop for them:
- FREEZE-FDN and the CI job: yes, it is covered. My words on main, next to KIT-FREEZE-2 in STATUS, name "the toolchain's CI job" among the allowed edits. Each such edit still asks me and merges on my go. Guard checks for the new pins are not covered; they wait for the gate step (2.3).
- Merge order: 2.2 merges after 2.1b.
```
