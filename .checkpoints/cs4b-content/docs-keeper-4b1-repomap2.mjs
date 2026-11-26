// docs-keeper, 4b-1 part 2: REPO_MAP rows made true for the final code (2039e3c). Each replacement must match once
// (or the stated count). Run once from the worktree root.
import { readFileSync, writeFileSync } from "node:fs";
const F = "docs/REPO_MAP.md";
let t = readFileSync(F, "utf8");
const rep = (from, to, count = 1) => {
  const n = t.split(from).length - 1;
  if (n !== count) throw new Error(`${n} matches (want ${count}) for: ${from.slice(0, 80)}`);
  t = t.split(from).join(to);
};
rep("change-set 4b-1, in review", "change-set 4b-1", 14);
rep("change-set 4b-1 (in review)", "change-set 4b-1", 2);
rep("[verified 2026-11-26 at ef2edec: `crates/devtools/Cargo.toml`]", "[verified 2026-11-26 at 2039e3c: `crates/devtools/Cargo.toml`]");
rep("[verified 2026-11-26 at ef2edec: the three manifests]", "[verified 2026-11-26 at 2039e3c: the three manifests]");
// The content crate's row.
rep("`json.rs` (bytes to a tree in the one cut grammar a content file has; anything else refused, duplicate keys included)",
  "`json.rs` (bytes to a tree in the one cut grammar a content file has; anything else refused, duplicate keys included; an id is no name Windows keeps for its devices; every list cut to its length when it closes)");
rep("`number.rs` (how a number is read and written: canonical text only, an exact tie written as a browser writes it)",
  "`number.rs` (how a number is read and written: canonical text only; an exact tie written as a browser writes it, the even text taken only if it reads back as the number)");
rep("bounds, spawns, axis-aligned boxes and ramps, regions;", "bounds, spawns (`at` the ground between the feet; which way `facing_deg` 0 points is pinned at step 8), axis-aligned shapes whose form is `Form::Box` or `Form::Ramp { rises_toward }`, regions;");
rep("(every refusal by kind, with the file, line and column or field) [verified 2026-11-26 at ef2edec: each file's header]",
  "(every refusal by kind, with the file, line and column or field) [verified 2026-11-26 at 2039e3c: each file's header; `map.rs`'s `Form`, `json.rs`'s device names and `shrink_to_fit`]");
// rules_scan.rs.
rep("planted breaches and controls inside it. It sees no types: integer code in `sim` writes the path form on its own type (`u32::to_le_bytes(x)`) (change-set 3a) |",
  "planted breaches and controls inside it. It sees no types: integer code in `sim` writes the path form on its own type (`u32::to_le_bytes(x)`) (change-set 3a). Since change-set 4b-1 it also reads `crates/content/src` (`the_content_crate_keeps_its_rules`: hashed maps, clocks, per-target code, every call that can panic outside tests, and the crate's lint guard read as denied; planted breaches and controls), so the guard is held by a test while CI runs no clippy; a file-level `#![cfg(test)]` is not seen as test code (it fails closed) [verified 2026-11-26 at 2039e3c] |");
// files.rs.
rep("`read_plain` and `write_plain`: the one read and write of a committed file for the Rust tests and tools, only when it is a plain file in a plain folder, never through a link |",
  "`read_plain` and `write_plain`: the one read and write of a committed file for the Rust tests and tools, only when it is a plain file in a plain folder, never through a link; `read_plain` opens the file once and reads through that handle, at most its cap; `write_plain` checks the name [verified 2026-11-26 at 2039e3c] |");
// content_proofs.rs.
rep("the inputs at the loader's caps; the bounded fuzz run's inputs and judge. The same code natively and in the test-only module |",
  "the inputs at the loader's caps (`LIMIT_CASES`, seven, each made alone by `limit_case(at)` and found by its name); the bounded fuzz run's inputs and judge, its verdicts counted in ten stages (`STAGES`; the map's own rules are one). The same code natively and in the test-only module [verified 2026-11-26 at 2039e3c] |");
// content_fuzz.rs.
rep("1,000,000 inputs in release, 50,000 in debug, no panic and every promise kept, each stage's minimum share asserted; a failure prints its seed (`SHAMBLE_FUZZ_SEED` runs another) |",
  "1,000,000 inputs in release (CI's release line runs it, `-p devtools`), 50,000 in debug, no panic and every promise kept, each of the ten stages' minimum share asserted; a failure prints its seed (`SHAMBLE_FUZZ_SEED` runs another); and the search for a key that is there twice, timed on two inputs of one shape so that a search over every pair fails |");
// content_proofs test row.
rep("the number corpus, the tree corpus, and every class of broken file against its verdict and the committed digests |",
  "the number corpus, the tree corpus, and every class of broken file against its verdict and the committed digests; the vectors read through `read_plain` |");
// vectors row.
rep("`content-corpus.txt` (13,067 numbers, each with a browser engine's bits and texts)",
  "`content-corpus.txt` (26,653 numbers, each with a browser engine's bits and texts: the 2,038 old ones, 5,000 doubles from random bits, 5,000 decimals, 2,000 real ties at 2 to 25 binary places, every power of two with the double under and over it in both signs, 29 edges)");
rep("`content-verdicts.txt` (the digests of the native verdicts per class of broken file, seed 20261126, made by `content_proofs::verdicts`)",
  "`content-verdicts.txt` (the digests of the native verdicts: 37 classes of broken file and the fuzz run's first inputs, seed 20261126, made by `content_proofs::verdicts`)");
// Node and browser rows.
rep("the number corpus, the tree corpus, the verdicts against the native digests and the inputs at the caps, on the test-only module in both profiles; a missing module fails the file at its imports |",
  "the number corpus, the tree corpus, the verdicts against the native digests and the seven inputs at the caps, on the test-only module in both profiles, the vectors read through `plain-file.ts`; its memory and time ceilings are backstops (memory per case is proven natively); a missing module fails the file at its imports |");
rep("`readPlainFile`: the one read of a committed recording for the tests under Node and the browser specs,",
  "`readPlainFile`: the one read of a committed recording or vector for the tests under Node and the browser specs (the older `rules.test.mjs` and `wasm-rules.spec.ts` still read theirs without it: a follow-up),");
rep("the content reader's proofs on the test-only module's release profile (the corpora, the verdicts against the native digests, the inputs at the caps), and the page's own engine asked for every number of the corpus |",
  "the content reader's proofs on the test-only module's release profile (the corpora, the verdicts against the native digests, the inputs at the caps), the vectors read through `plain-file.ts`, and the page's own engine asked for every number of the corpus |");
writeFileSync(F, t);
console.log("REPO_MAP: done");
