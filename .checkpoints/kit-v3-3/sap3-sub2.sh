#!/bin/bash
# security-auditor pass 3: blob -> gitlink type change of a kit file under .gitmodules ignore=all; is it listed?
# usage: bash sap3-sub2.sh <empty scratch dir>
set -u
R="$1"; mkdir -p "$R"; cd "$R" || exit 1
export GIT_CONFIG_GLOBAL=/dev/null GIT_CONFIG_NOSYSTEM=1 GIT_AUTHOR_NAME=t GIT_AUTHOR_EMAIL=t@t GIT_COMMITTER_NAME=t GIT_COMMITTER_EMAIL=t@t
F='--name-only -z --no-renames --no-relative'
git init -q -b main p; cd p || exit 1
mkdir .claude; echo '{"hooks":1}' > .claude/settings.json; echo rules > CLAUDE.md; git add -A; git commit -qm base; git branch base
H=$(git rev-parse HEAD)
git switch -qc tc
git rm -q --cached .claude/settings.json; rm .claude/settings.json
git update-index --add --cacheinfo "160000,$H,.claude/settings.json"
printf '[submodule "s"]\n\tpath = .claude/settings.json\n\turl = ./x\n\tignore = all\n' > .gitmodules; git add .gitmodules; git commit -qm tc
echo "--- blob->gitlink settings.json, ignore=all (base...tc):"; git diff $F base...tc | tr '\0' '\n'
echo "--- same, --ignore-submodules=none:"; git diff $F --ignore-submodules=none base...tc | tr '\0' '\n'
echo "--- same, two-dot base tc --:"; git diff $F base tc -- | tr '\0' '\n'
git switch -q main
echo "--- real switch to tc: what is .claude/settings.json after?"; git switch -q tc; ls -la .claude | sed -n '1,10p'; test -f .claude/settings.json && echo FILE || echo NOT-A-FILE
git switch -q main
# gitlink modify: two different SHAs
git switch -qc g1 base; git update-index --add --cacheinfo "160000,$H,.claude/sub"; printf '[submodule "s"]\n\tpath = .claude/sub\n\turl = ./x\n\tignore = all\n' > .gitmodules; git add .gitmodules; git commit -qm g1
git switch -qc g2; git update-index --cacheinfo "160000,$(git rev-parse tc),.claude/sub"; git commit -qm g2
echo "--- gitlink modify ignore=all (g1...g2):"; git diff $F g1...g2 | tr '\0' '\n'
echo "--- gitlink modify, --ignore-submodules=none:"; git diff $F --ignore-submodules=none g1...g2 | tr '\0' '\n'
echo "--- gitlink add, --ignore-submodules=none (base...g1):"; git diff $F --ignore-submodules=none base...g1 | tr '\0' '\n'
