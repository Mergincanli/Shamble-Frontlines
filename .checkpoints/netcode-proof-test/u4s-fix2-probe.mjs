// rust-reviewer's read-only probe of the last round's bite-id grep (G1a): which of marks.json's bite ids BITE_ID reads,
// and what biteHomeProblems says of one uppercase id in two crates (an in-memory fixture; nothing on disk is written).
//   node u4s-fix2-probe.mjs
import { readFileSync } from "node:fs";
import { pathToFileURL } from "node:url";

const SPIKE = "/home/user/Shamble-Frontlines/.claude/worktrees/wf_aa2cc979-004-1/spikes/netcode-proof";
const { BITE_ID, biteHomeProblems } = await import(pathToFileURL(`${SPIKE}/check.mjs`).href);

const marks = JSON.parse(readFileSync(`${SPIKE}/marks/marks.json`, "utf8"));
const list = Array.isArray(marks) ? marks : (marks.marks ?? Object.values(marks));
const ids = list.flatMap((m) => (m.bites ?? []).map((b) => b.id));
const missed = ids.filter((id) => !BITE_ID.test(id));
console.log(`marks.json bite ids: ${ids.length}; BITE_ID reads ${ids.length - missed.length}; misses ${missed.length}`);
console.log(`missed: ${missed.join(" ")}`);

const wide = /^PM-\d{4}-\d+\/[A-Za-z0-9.-]+$/;
console.log(`a wide form [A-Za-z0-9.-]+ reads ${ids.filter((id) => wide.test(id)).length} of ${ids.length}`);

const twice = (id) => {
  const files = {
    "crates/room/src/rewind.rs": `#[cfg(feature = "bites")]\npub const BITE: &str = "${id}";\n`,
    "crates/sim/src/step.rs": `#[cfg(feature = "bites")]\npub const BITE: &str = "${id}";\n`,
  };
  return biteHomeProblems(Object.keys(files).sort(), (f) => files[f]);
};
for (const id of ["PM-0001-7/B-S", "PM-0001-5/B-S-sim", "PM-0001-35/quinn-proto-0.11.17", "PM-0001-7/untrimmed"]) {
  const p = twice(id);
  console.log(`${id} in room and sim: ${p.length ? `refused: ${p[0]}` : "PASSES (not seen)"}`);
}
