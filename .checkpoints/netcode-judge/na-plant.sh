#!/usr/bin/env bash
# netcode-architect's plants for NPT-L2-ONEWAY: one at a time in the scratch copy, restored after each.
# Usage: na-plant.sh <name>   (runs the engine tests with the plant, prints the oneway_ results, restores)
set -u
S=/home/user/Shamble-Frontlines/.gate/netcode-judge/.gate/na-oneway
P=$S/netcode-proof/crates/judge/src
mkdir -p "$S/pristine"
[ -f "$S/pristine/eval.rs" ] || cp "$P/eval.rs" "$S/pristine/eval.rs"
[ -f "$S/pristine/marks.rs" ] || cp "$P/marks.rs" "$S/pristine/marks.rs"
cp "$S/pristine/eval.rs" "$P/eval.rs"; cp "$S/pristine/marks.rs" "$P/marks.rs"
case "$1" in
  a) perl -0pi -e 's/\.filter\(stamps\)\.filter\(\|l\| l\.header\.layer == "L2"\)/.filter(stamps)/' "$P/eval.rs" ;;
  b) perl -0pi -e 's/let net = \*written - Ratio::new/let net = *written + Ratio::new/' "$P/eval.rs" ;;
  c) perl -0pi -e 's/Ratio::new\(i128::from\(base\), 2\)/Ratio::int(i128::from(base))/' "$P/eval.rs" ;;
  d) perl -0pi -e 's/\[&str; 2\] = \["probe_up_delay_ns", "probe_down_delay_ns"\]/[&str; 3] = ["probe_up_delay_ns", "probe_down_delay_ns", "probe_rtt_ns"]/' "$P/marks.rs" ;;
  e) perl -0pi -e 's/let Some\(param\) = log\.header\.params\.get\("base_rtt_ns"\) else \{\n\s*return Err\(format!\(\n[^;]*?\)\);\n\s*\};/let Some(param) = log.header.params.get("base_rtt_ns") else { continue; };/s' "$P/eval.rs" ;;
  f) perl -0pi -e 's/let logs = run\.logs\.iter\(\)\.filter\(\|l\| log_matches\(l, &check\.from\)\);\n(\s*)let stamps = \|l: &&&Log\| l\.seconds\.iter\(\)\.any\(\|s\| s\.s\.contains_key\(series\)\);\n(\s*)for log in logs\.filter\(stamps\)/let logs = run.logs.iter().filter(|l| log_matches(l, &check.from));\n$1for log in logs/s' "$P/eval.rs" ;;
  g) perl -0pi -e 's/if net < Ratio::int\(0\) \{/if net < Ratio::int(-1_000_000_000) {/' "$P/eval.rs" ;;
  h) perl -0pi -e 's/stat: Stat::Mean \| Stat::Min \| Stat::Max \| Stat::Pct\(_\)/stat: Stat::Mean | Stat::Pct(_)/' "$P/marks.rs" ;;
  i) perl -0pi -e 's/if base\.replace\(ns\)\.is_some_and\(\|other\| other != ns\)/if base.get_or_insert(ns) != \&ns \&\& false/' "$P/eval.rs" ;;
  j) perl -0pi -e 's/if measure\.one_way_place\(\)\.is_some\(\)\n\s*&& \(c\.pooled \|\| matches!\(threshold, Threshold::Against \{ \.\. \}\)\)/if measure.one_way_place().is_some() \&\& c.pooled/s' "$P/marks.rs" ;;
  k) perl -0pi -e 's/\["probe_up_delay_ns", "probe_down_delay_ns"\]/["probe_up_delay_ns", "probe_up_x"]/' "$P/marks.rs" ;;
  none) ;;
  *) echo "unknown plant $1"; exit 2 ;;
esac
echo "== plant $1: diff vs pristine"
diff "$S/pristine/eval.rs" "$P/eval.rs"; diff "$S/pristine/marks.rs" "$P/marks.rs"
cd "$S/netcode-proof" && CARGO_TARGET_DIR=$S/target cargo test -p npt-judge --locked --no-fail-fast 2>&1 \
  | grep -E "^test .*(oneway_|FAILED)|^test result|^error" | sort | uniq
cp "$S/pristine/eval.rs" "$P/eval.rs"; cp "$S/pristine/marks.rs" "$P/marks.rs"
