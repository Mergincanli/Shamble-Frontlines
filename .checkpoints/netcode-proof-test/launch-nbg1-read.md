# nbg1 plan read: the brief for B6 (advisor-stopb-plan.md's author)
Status: written 18:5x UTC on 11-28, after his "Continue" (18:33 UTC, history.md:8274-8288), in the advisor tool's order.

## The brief (to B6: resumed by SendMessage; else a new advisor agent with this file as its whole brief)
Your plan check of /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/nbg1-plan.md (DRAFT 0.1, with its dry
run): the stand-in your XH-2 named, to run only if his create-form look (msg-stopb-e2.md, For Mustafa 2) shows that
`fsn1` cannot host both servers and he picks `nbg1`. Nothing is committed in spikes/ before his word; the dry run's
worktree is removed, its logs are in /home/user/Shamble-Frontlines/.gate/nbg1/, its script is nbg1-dry.mjs.
Judge:
1. **Completeness:** are r1.sh:34, r1sh.test.mjs:199 and R1.md:63, :114, :135 all that a location change must touch
   for R1's day? Check what else could be bound to the location: rs.yaml, lg.yaml, ops/firewall/, ops/operator/
   (PROMPT-R1.md, settings.json, rules.md), r1.sh's other functions, npt-cert's DNS and ACME settings, and R1.md's Once
   list (the firewall objects, the kept addresses). Start from `git grep -n -i fsn1 -- spikes/netcode-proof` and the
   plan's "Left as they are" list.
2. **The one change to stopb-plan.md C1's order:** Y = N, with no list commit, when both builds' lists equal
   r1-dist.sha256's committed non-comment lines byte for byte. Is it sound (N changes no file under web/ or crates/
   and no Cargo or npm file)?
3. **The bars** for N and X'' (the plan's chain, steps 1-3) against stopb-plan.md C1-D3 and its Y' lines: anything
   missing or weaker? In particular the dry run's diff sha256 as N's bar.
4. **"Left as they are":** the marks.json notes (:11310, :11369), the test data, and the ADR and SPIKE_PLAN text that
   name fsn1. May they stay, named as stale words in R1's report, or does any need an edit, or his word, before R1?
5. **The held question** (no location offers CAX21): its wording, and whether it should carry options or a lean now.
6. Anything in the dry run's logs that does not hold.
Rule: a plan check only, no edit; verdict APPROVE, APPROVE WITH CONDITIONS or REVISE, each finding with its severity on
the one scale (Critical, High, Medium, Low). Write your blocks into
/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-stopb-plan.md under a new heading
"## nbg1 plan read" (your plan first, then each block as it is done); end with "Decisions needed" and "For Mustafa",
then the last line `Status: COMPLETE <date and UTC time>`. Your agent type has no Write or Edit tool: append each block
with one Bash heredoc (`cat >> <that full path> <<'EOF'`), that file only.
The same tool rules as before: git first, one git per call, no `git -C`; printing forms only; names of environment
variables only, never values; no GitHub writes; never `npm audit`. His rules, word for word (history.md:8259-8272,
item 7): "git and gh always start their own command, with no other program in front of them. In a shell command, spell
out every file you write with its full path: no variable, substitution or wildcard in the name of a file that is
written, moved, copied or deleted, or use the Write and Edit tools."
