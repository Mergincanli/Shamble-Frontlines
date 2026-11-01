#!/usr/bin/env bash
# The write guard of plan revision 3 (advisor condition 1), for the working tree about to be committed.
set -u
cd "$(git rev-parse --show-toplevel)"
git fetch -q origin main || { echo "GUARD: fetch failed"; exit 1; }
BASE=9e7db28
MB=$(git merge-base origin/main HEAD)
[ "$(git rev-parse $MB)" = "$(git rev-parse $BASE)" ] || echo "GUARD note: main moved; merge base is $MB"
allowed='^(docs/LOOK_V2\.md|docs/ENGINE_OVERVIEW\.md|docs/SPIKE_PLAN\.md|docs/adr/README\.md|docs/adr/000[1-6]-[a-z0-9-]+\.md|docs/reports/2026-1[01]-[0-9][0-9]-v2-decision-drafts(-appendix)?\.md|docs/research/GLOSSARY\.md|docs/STATUS\.md|docs/ROADMAP\.md|docs/REPO_MAP\.md|\.checkpoints/v2-decision-drafts(\.md|/.*))$'
bad=$( { git diff --name-only $MB; git ls-files --others --exclude-standard; } | sort -u | grep -Ev "$allowed" )
fail=0
if [ -n "$bad" ]; then echo "GUARD FAIL: paths outside the allowed list:"; echo "$bad"; fail=1; fi
git diff --quiet $BASE -- docs/research/r1-simulation-core.md docs/research/r2-netcode.md docs/research/r3-rendering.md docs/research/r4-physics-navigation.md docs/research/r5-toolchain-quality.md docs/research/r6-infrastructure.md docs/research/UNREAD_SOURCES.md docs/LEGACY_INVENTORY.md || { echo "GUARD FAIL: a research file or the inventory changed"; fail=1; }
# Condition 1's check, in its exact form: the base GLOSSARY ends without a final newline (its line 742 is followed by an
# unterminated last line), so `head -n 742` would drop that line; the base file must instead be a byte prefix of today's.
n=$(git show $BASE:docs/research/GLOSSARY.md | wc -c)
git show $BASE:docs/research/GLOSSARY.md | cmp -s - <(head -c "$n" docs/research/GLOSSARY.md) || { echo "GUARD FAIL: GLOSSARY's base text changed"; fail=1; }
[ $fail = 0 ] && echo "GUARD PASS"
exit $fail
