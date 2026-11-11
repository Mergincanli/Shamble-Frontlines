#!/usr/bin/env bash
# test-engineer review, 2.1a: G4, a page under legacy/ reads ../README.md (inside the repo, outside legacy/), run with
# --smoke-only (CI's smoke job mode), on a scratch copy. Expect 5a FAIL naming "outside the build folder".
set -u
R=/home/user/Shamble-Frontlines
S=$R/.gate/fdn-scratch
G=$R/.gate/foundation-restructure/gaps
rm -rf "$G/g4"; mkdir -p "$G/g4/legacy"
cp -r "$S/tools" "$G/g4/tools"; rm -rf "$G/g4/tools/node_modules"; ln -s "$R/tools/node_modules" "$G/g4/tools/node_modules"
for f in shamble-core.js shamble-game.js shamble-editor.js index.html editor.html; do cp "$S/$f" "$G/g4/legacy/$f"; done
cp "$R/README.md" "$G/g4/README.md"
echo "pattern count $(grep -c 'function deploy(){ started=true;' "$G/g4/legacy/shamble-game.js") (want 1)"
sed -i "s#function deploy(){ started=true;#function deploy(){ const e3img=new window.Image(); e3img.src='../README.md'; started=true;#" "$G/g4/legacy/shamble-game.js"
mode=${1:-mutant}
if [ "$mode" = control ]; then cp "$S/shamble-game.js" "$G/g4/legacy/shamble-game.js"; fi
(cd "$G/g4" && PLAYWRIGHT_BROWSERS_PATH=$R/.gate/pw-browsers node tools/gate.mjs --smoke-only --json >"$G/g4.json" 2>"$G/g4.err"; echo "G4 $mode exit $?")
node -e 'const j=JSON.parse(require("fs").readFileSync(process.argv[1],"utf8"));console.log(j.scope,j.build);for(const c of j.checks)console.log(c.id,c.pass,c.summary.slice(0,90),"||",c.details.filter(d=>/outside/.test(d)).map(d=>d.replace(/file:\/\/\S*\/g4\//,"file:///.../g4/")).join(" | ").slice(0,200))' "$G/g4.json"
