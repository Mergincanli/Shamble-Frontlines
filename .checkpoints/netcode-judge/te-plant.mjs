// test-engineer's planted defects for part a, applied ONLY to the scratch copy under .gate/te-plant (never the
// worktree's source). Usage: node te-plant.mjs <plant-name>|reset
// Each plant: one exact string replaced once in one file of the copy; `reset` restores the copy's files from the
// worktree's source.
import { readFileSync, writeFileSync } from "node:fs";

const SRC = "/home/user/Shamble-Frontlines/.gate/netcode-judge/spikes/netcode-proof/crates/judge/src";
const COPY = "/home/user/Shamble-Frontlines/.gate/netcode-judge/.gate/te-plant/netcode-proof/crates/judge/src";
const FILES = ["eval.rs", "marks.rs"];

const PLANTS = {
  // P1: a moved check still chosen for a bite run that matches no part.
  p1_bite_nomatch_keeps_moved_check: ["eval.rs", "c.moved.is_none() && c.parts.is_empty()", "c.parts.is_empty()"],
  // P2: a bite run on kept AND moved parts takes the moved part's rules and checks too.
  p2_mixed_bite_run_uses_moved_parts: [
    "eval.rs",
    "(matched.is_empty() || !kept.is_empty()).then_some(kept)",
    "(matched.is_empty() || !kept.is_empty()).then_some(matched.clone())",
  ],
  // P3: bite runs on moved parts only are NOT left out.
  p3_bite_runs_on_moved_parts_kept: [
    "eval.rs",
    "(matched.is_empty() || !kept.is_empty()).then_some(kept)",
    "Some(if kept.is_empty() { matched.clone() } else { kept })",
  ],
  // P4: the MOVED reason line drops the moved checks.
  p4_why_line_drops_checks: [
    "eval.rs",
    `.map(|c| format!("check {:?} to {}", c.name, c.moved.to)),`,
    `.filter(|_| false).map(|c| format!("check {:?} to {}", c.name, c.moved.to)),`,
  ],
  // P5: the outside refusal ignores a moved bite (only the mark's own label refused).
  p5_outside_refusal_mark_only: [
    "marks.rs",
    "if kind == Kind::Outside && (m.moved.is_some() || moved_piece) {",
    "if kind == Kind::Outside && m.moved.is_some() {",
  ],
  // P6: the precondition refusal forgets moved checks.
  p6_pre_refusal_forgets_checks: [
    "marks.rs",
    "|| pre.parts.iter().any(|p| p.moved.is_some())\n                            || pre.checks.iter().any(|c| c.moved.is_some()) =>",
    "|| pre.parts.iter().any(|p| p.moved.is_some()) =>",
  ],
  // P7: the pooled checks bypass `applies` (a moved pooled check still judged).
  p7_pooled_ignores_moved: [
    "eval.rs",
    ".filter(|c| c.pooled && applies(c, &part.name))",
    ".filter(|c| c.pooled && (c.parts.is_empty() || c.parts.iter().any(|p| p == &part.name)))",
  ],
  // P8: a moved part's PartOut keeps the kept verdict computation (judged, then relabelled MOVED).
  p8_moved_part_judged_then_relabelled: [
    "eval.rs",
    `            v.parts.push(PartOut {
                name: part.name.clone(),
                verdict: VerdictKind::Moved,
                why: vec![format!("{}: {}", part.name, m.line())],
                runs: vec![],`,
    `            let s = part_select(part);
            let mm: Vec<&Run<'_>> = serving.iter().copied().filter(|r| run_matches(r, &s)).collect();
            let judged = eval_part(mark, part, &mm, all, ctx);
            v.parts.push(PartOut {
                name: part.name.clone(),
                verdict: VerdictKind::Moved,
                why: vec![format!("{}: {}", part.name, m.line())],
                runs: judged.runs,`,
  ],
  // P9: a moved ws_bound check is let through (the ws_bound refusal disabled).
  p9_moved_ws_bound_allowed: [
    "marks.rs",
    "        if ws_bound.is_some() {\n            return Err(",
    "        if ws_bound.is_some() && false {\n            return Err(",
  ],
  // P10: blank check uses is_empty, not trim (a " " label passes).
  p10_blank_not_trimmed: ["marks.rs", "if text.trim().is_empty() {", "if text.is_empty() {"],
  // P11: MOVED ranked above NOT-RUN (a missing kept run hidden).
  p11_moved_above_not_run: [
    "eval.rs",
    "    } else if part_is(VerdictKind::NotRun)\n",
    "    } else if !mark.any_moved() && part_is(VerdictKind::NotRun)\n",
  ],
  // P12: any_moved forgets bites (a mark with only a moved bite reads PASS).
  p12_any_moved_forgets_bites: ["marks.rs", "            || self.bites.iter().any(|b| b.moved.is_some())\n", "\n"],
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
