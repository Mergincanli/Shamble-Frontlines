// Workflow script (verbatim, copied at launch, run wf_f32d92bc-247, 2026-11-03 about 17:34 Amsterdam): the advisor, effort xhigh, model: the session's (Opus 5.5); replaces the stopped wf_c32107c2-a9a
export const meta = {
  name: 'v2-drafts-plan-r5b',
  description: "The advisor reviews plan revision 5: Mustafa's design answers, his answers to the stop's escalations, COMMUNITY-SERVERS, and the last fix round for fixcheck-e's findings (PR #33)",
  phases: [ { title: 'Review', detail: 'the advisor: APPROVE, REVISE or ESCALATE' } ],
}
const C = '/home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts'
const SCHEMA = { type: 'object', properties: {
  verdict: { type: 'string', enum: ['APPROVE', 'APPROVE WITH CONDITIONS', 'REVISE', 'ESCALATE'] },
  conditions: { type: 'array', items: { type: 'string' } },
  rulings: { type: 'array', items: { type: 'object', properties: { code: { type: 'string' }, ruling: { type: 'string' } }, required: ['code', 'ruling'] } },
  escalations: { type: 'array', items: { type: 'string' } }, file: { type: 'string' } },
  required: ['verdict', 'conditions', 'rulings', 'escalations', 'file'] }
phase('Review')
const r = await agent(`You are the project's Advisor agent. Review a plan before any edit, as your agent file says (main's .claude/agents/advisor.md; this branch does not change it). The change-set is v2-decision-drafts (SHAMBLE FRONTLINES; branch claude/laughing-lovelace-ox61se standing in for cs/v2-decision-drafts; PR #33, a draft, not merged; base main 9e7db28). Today you approved plan revision 4 (recording the stop's answers; your verdict: ${C}/advisor-verdict-r4.md); that work is applied (03ef222, 59d41e0) and its verification is done: fixcheck-e1, e2, e3 (${C}/verifier-fixcheck-e1-verdicts.md, -e2-, -e3-), and, after a usage cut, e2b and e3b (${C}/verifier-fixcheck-e2b-verdicts.md, -e3b-; e2b's verdict FAIL for two medium items, PM-0005-4's presented kinds and Appendix A's stale counts). Mustafa then sent two messages, verbatim: ${C}/inputs/DESIGN-ANSWERS-mustafa-2026-11-03.md ("Record them in the README's design questions and in STATUS (in PR #33, or a docs follow-up if it's merged). They don't change the spike order.") and ${C}/inputs/ANSWERS-2-mustafa-2026-11-03.md (the four escalations answered; COMMUNITY-SERVERS, new; and "after e2b, e3b and their fix round, stop. No further re-check rounds; anything left goes into the PR as notes. The spikes are the real test, and we've had ten usage cuts.").

The plan to review: ${C}/plan-r5-design-answers.md (revision 5; a first version covering only the design answers went to you at 17:31 and was stopped at once when his second message came, so this one review covers all of it). Check it against his words, the findings files and the docs. Judge: does it record his answers faithfully (word for word; "leaning", "not final", "assumed, to confirm", "still open"; the Advisor's lean and pencil marked as the Advisor's), keep his scopes (the design answers in README and STATUS; COMMUNITY-SERVERS as an open question in ADR 0006 and README, the door kept open, not designed), route each fixcheck-e finding to its author, keep each author to its own text, avoid over-reading, honour his "no further re-check rounds" (the scripts and the gate still run; leftovers go into the PR as notes; your own review of the final head before the merge stays, under CLAUDE.md's merge rule), and fit plan revision 4's close (your conditions 7–9)? Rule on its decisions (DRAFT-ENTRIES, OQ-0003-1, the proof test's "doesn't measure client frames"), and on anything else it should say (for example: the open count after the round; whether the stand-in for ML changes any key's "MP is never a measured result" rule; COMMUNITY-SERVERS' constraints against ADR 0001 part 11 and ADR 0006). Verdict: APPROVE, APPROVE WITH CONDITIONS, REVISE or ESCALATE. Write it, verbatim as you would give it, to ${C}/advisor-verdict-r5.md (notes in the same file; if it ends with DONE, answer from it). You edit no doc and never commit or push.

Save as you go: write your verdict file as you work (notes first, the verdict block when composed); no binaries and no file over 1 MB in any scratch folder.

Return the structured result: the verdict, the conditions, each ruling, any escalation, and your file's path.`,
  { label: 'advisor: plan revision 5', phase: 'Review', agentType: 'advisor', effort: 'xhigh', schema: SCHEMA })
return r
