# engine-architect, cs4a "The recording", review round 2 (head f9f9971, source as at 07903d9)

Plan (read-only): (1) read round1-fold.md's last section (the fixes); (2) read `git diff eb14739..f9f9971 -- crates
packages`; (3) check E1-E5 each as fixed or not; (4) the export's surface after the pass (one export, no argument, one
number, &self) by reading and by the built file's imports/exports if a fresh build exists (no build of my own unless
needed; cargo --locked only); (5) boundaries after the pass: Fnv1a64 public in sim, the replay tool's `fields`
subcommand, files::plain, anything else new; (6) weigh rust-reviewer's new Low (write_state + STATE_BYTES to
#[cfg(test)]): does 4c or cs7 need them public; (7) verdict, short.

Findings (a line each as found):
