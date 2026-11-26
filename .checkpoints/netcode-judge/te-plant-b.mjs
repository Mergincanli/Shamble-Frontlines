// test-engineer's planted defects for part b, applied ONLY to the scratch copy under .gate/te-plant-b (never the
// worktree's source). Usage: node te-plant-b.mjs <plant-name>|reset
// Each plant: one exact string replaced once in one file of the copy; `reset` restores the copy's files from the
// worktree's source. Run the tests after each plant with CARGO_TARGET_DIR=.gate/te-plant-b/target.
import { readFileSync, writeFileSync } from "node:fs";

const SRC = "/home/user/Shamble-Frontlines/.gate/netcode-judge/spikes/netcode-proof/crates/judge/src";
const COPY = "/home/user/Shamble-Frontlines/.gate/netcode-judge/.gate/te-plant-b/netcode-proof/crates/judge/src";
const FILES = ["eval.rs", "figures.rs"];

const PLANTS = {
  // Control (must be caught): the slot's last session gets the tail too.
  pb0_control_last_session_tail: [
    "figures.rs",
    "let left = !last && end.is_some_and(|end| b <= *end);",
    "let left = end.is_some_and(|end| b <= *end);",
  ],
  // PB1: a session_end from ANY session of the slot allows this session's tail (not only one in its own window).
  pb1_end_from_any_window: [
    "figures.rs",
    "let end = ended.get(slot).and_then(|v| v.range(x.from..x.to).next());",
    "let end = ended.get(slot).and_then(|v| v.iter().next_back());",
  ],
  // PB2: the LAST session_end in the window, not the first (decision 1 says the first).
  pb2_last_end_in_window: [
    "figures.rs",
    "let end = ended.get(slot).and_then(|v| v.range(x.from..x.to).next());",
    "let end = ended.get(slot).and_then(|v| v.range(x.from..x.to).next_back());",
  ],
  // PB3: admit records read from every log, a client-side log included.
  pb3_admit_from_any_log: [
    "figures.rs",
    'let admits = session_ticks(run, "admit", &["room"]);',
    'let admits = session_ticks(run, "admit", &["room", "client", "bots", "lab"]);',
  ],
  // PB4: -10's unit cost sums only slot 0's relevant_count.
  pb4_sync_first_slot_only: [
    "figures.rs",
    "        ns += i128::from(*w);\n        entities += list.iter().map(|e| i128::from(*e)).sum::<i128>();",
    "        ns += i128::from(*w);\n        entities += i128::from(list[0]);",
  ],
  // PB5: -10's unit cost from the last measured tick only.
  pb5_sync_last_tick_only: [
    "figures.rs",
    "        ns += i128::from(*w);\n        entities += list.iter().map(|e| i128::from(*e)).sum::<i128>();",
    "        ns = i128::from(*w);\n        entities = list.iter().map(|e| i128::from(*e)).sum::<i128>();",
  ],
  // PB6: R2's E as the mean over ticks of the LARGEST slot entry (not the mean over (tick, slot) entries).
  pb6_r2_e_largest_slot: [
    "figures.rs",
    "            entities += list.iter().map(|e| i128::from(*e)).sum::<i128>();",
    "            entities += list.iter().map(|e| i128::from(*e)).max().unwrap_or(0) * clients;",
  ],
  // PB9: the logging-on pool takes the part's valid runs of EVERY set, not this run's set.
  pb9_on_pool_every_set: [
    "figures.rs",
    "let on: Vec<&Run<'_>> = runs.iter().copied().filter(|r| here(r)).collect();",
    "let on: Vec<&Run<'_>> = runs.to_vec();",
  ],
  // PB17: the first pass keeps a run whose validity is undecided (a pre: not decided yet) in Inputs.parts.
  pb17_parts_keep_undecided: ["eval.rs", "                && v.undecided.is_empty()\n", "\n"],
  // PB18: the first pass reads moved parts too (condition 2: never a moved part's runs).
  pb18_parts_keep_moved: [
    "eval.rs",
    ".filter(|p| reads_figures && p.moved.is_none())",
    ".filter(|_| reads_figures)",
  ],
  // PB19: the first pass ignores the part's setup checks.
  pb19_parts_ignore_setup: [
    "eval.rs",
    "                && setup\n                    .map(|c| eval_check(c, &[*r], &set, ctx).outcome)\n                    .all(|o| o == Outcome::Pass)",
    "                && setup.count() < usize::MAX",
  ],
};

const name = process.argv[2];
if (name === "reset") {
  for (const f of FILES) writeFileSync(`${COPY}/${f}`, readFileSync(`${SRC}/${f}`, "utf8"));
  console.log("reset: copy equals the worktree's source");
  process.exit(0);
}
const plant = PLANTS[name];
if (!plant) {
  console.error(`unknown plant ${name}; known: ${Object.keys(PLANTS).join(", ")}`);
  process.exit(2);
}
const [file, from, to] = plant;
for (const f of FILES) writeFileSync(`${COPY}/${f}`, readFileSync(`${SRC}/${f}`, "utf8"));
const text = readFileSync(`${COPY}/${file}`, "utf8");
const n = text.split(from).length - 1;
if (n !== 1) {
  console.error(`${name}: ${n} matches in ${file}, not 1`);
  process.exit(1);
}
writeFileSync(`${COPY}/${file}`, text.replace(from, to));
console.log(`planted ${name} in the copy's ${file}`);
