#!/bin/bash
# security-auditor pass 3: does a .gitmodules ignore=all hide a gitlink at a kit path from git diff --name-only?
# usage: bash sap3-sub.sh <empty scratch dir>
set -u
R="$1"; mkdir -p "$R"; cd "$R" || exit 1
export GIT_CONFIG_GLOBAL=/dev/null GIT_CONFIG_NOSYSTEM=1 GIT_AUTHOR_NAME=t GIT_AUTHOR_EMAIL=t@t GIT_COMMITTER_NAME=t GIT_COMMITTER_EMAIL=t@t
F='--name-only -z --no-renames --no-relative'
git init -q -b main p; cd p || exit 1
mkdir .claude; echo '{}' > .claude/settings.json; git add -A; git commit -qm base; git branch base
H=$(git rev-parse HEAD)
git switch -qc sm; git update-index --add --cacheinfo "160000,$H,.claude/sub"
printf '[submodule "s"]\n\tpath = .claude/sub\n\turl = ./x\n\tignore = all\n' > .gitmodules; git add .gitmodules; git commit -qm sm
echo "--- add gitlink, ignore=all (base...sm):"; git diff $F base...sm | tr '\0' '\n'
git switch -qc sm2; git update-index --cacheinfo "160000,$(git rev-parse base~0),.claude/sub"; git commit -qm mod --allow-empty
echo "--- modify gitlink, ignore=all (sm...sm2):"; git diff $F sm...sm2 | tr '\0' '\n'
echo "--- two-dot form (sm sm2 --):"; git diff $F sm sm2 -- | tr '\0' '\n'
echo "--- --ignore-submodules=none:"; git diff $F --ignore-submodules=none sm...sm2 | tr '\0' '\n'
echo "--- cached, gitlink change staged:"; git switch -q sm; git update-index --cacheinfo "160000,$(git rev-parse sm2),.claude/sub"; git diff --cached $F | tr '\0' '\n'
