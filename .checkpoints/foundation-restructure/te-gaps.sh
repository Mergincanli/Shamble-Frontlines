#!/usr/bin/env bash
# test-engineer review, 2.1a: three untested paths, on scratch copies under .gate/foundation-restructure/ (never the tracked files).
# G1 the guarded import's failure path; G2 a <script src="../x"> under legacy/; G3 village.py's default core path.
set -u
R=/home/user/Shamble-Frontlines
S=$R/.gate/fdn-scratch
G=$R/.gate/foundation-restructure/gaps
rm -rf "$G"; mkdir -p "$G"
mk() {  # mk <dir>: a legacy/ layout row with the edited tools
  mkdir -p "$1/legacy"
  cp -r "$S/tools" "$1/tools"; rm -rf "$1/tools/node_modules"; ln -s "$R/tools/node_modules" "$1/tools/node_modules"
  for f in shamble-core.js shamble-game.js shamble-editor.js index.html editor.html; do cp "$S/$f" "$1/legacy/$f"; done
}
# G0 control: legacy/ layout, --no-smoke
mk "$G/g0"; (cd "$G/g0" && node tools/gate.mjs 44446a5 --no-smoke >/dev/null 2>"$G/g0.err"; echo "G0 control exit $? (want 0)")
# G1: smoke.mjs fails to load
mk "$G/g1"; echo 'this is not javascript (' >> "$G/g1/tools/smoke.mjs"
(cd "$G/g1" && node tools/gate.mjs 44446a5 --no-smoke >/dev/null 2>"$G/g1.err"; echo "G1 broken smoke.mjs exit $? (want 2): $(head -1 "$G/g1.err")")
# G2: index.html under legacy/ loads a script from the parent folder (inside the repo, outside legacy/)
mk "$G/g2"; cp "$S/shamble-core.js" "$G/g2/shamble-core-copy.js"
n=$(grep -c '<script src="shamble-core.js"' "$G/g2/legacy/index.html"); echo "G2 pattern count $n (want 1)"
sed -i 's#<script src="shamble-core.js"#<script src="../shamble-core-copy.js"#' "$G/g2/legacy/index.html"
(cd "$G/g2" && node tools/gate.mjs 44446a5 --no-smoke --json >"$G/g2.json" 2>"$G/g2.err"; echo "G2 ../ script under legacy/ exit $? (want 1)")
node -e 'const j=JSON.parse(require("fs").readFileSync(process.argv[1],"utf8"));for(const c of j.checks)if(!c.pass&&!c.skipped)console.log("   FAIL",c.id,c.details.filter(d=>/outside/.test(d)).slice(0,1).join(""))' "$G/g2.json" 2>/dev/null || head -2 "$G/g2.err"
# G3: village.py default core path; the aabbs file is missing on purpose, so reaching that open means the core was read
mkdir -p "$G/g3/tools" "$G/g3/legacy"; cp "$S/tools/village.py" "$G/g3/tools/"
cp "$S/shamble-core.js" "$G/g3/legacy/shamble-core.js"; echo '// no map here' > "$G/g3/shamble-core.js"
o=$(cd "$G/g3" && python3 -I tools/village.py 2>&1 | tail -1); echo "G3a legacy/ good + root decoy: $o"
rm "$G/g3/legacy/shamble-core.js"; o=$(cd "$G/g3" && python3 -I tools/village.py 2>&1 | tail -1); echo "G3b root decoy only (want IndexError): $o"
cp "$S/shamble-core.js" "$G/g3/shamble-core.js"; o=$(cd "$G/g3" && python3 -I tools/village.py 2>&1 | tail -1); echo "G3c root good only: $o"
