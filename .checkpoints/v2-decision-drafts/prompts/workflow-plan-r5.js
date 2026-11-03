// Workflow script (verbatim, run wf_c32107c2-a9a, 2026-11-03 about 17:31 Amsterdam; a first launch at effort high was stopped at once and relaunched at xhigh, the synthesis rule): the advisor, effort xhigh, model: the session's (Opus 5.5)
export const meta = {
  name: 'v2-drafts-plan-r5',
  description: "The advisor reviews plan revision 5: record Mustafa's design answers of 2026-11-03 in README and STATUS (PR #33)",
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
const r = await agent(`You are the project's Advisor agent. Review a plan before any edit, as your agent file says (main's .claude/agents/advisor.md; this branch does not change it). The change-set is v2-decision-drafts (SHAMBLE FRONTLINES; branch claude/laughing-lovelace-ox61se standing in for cs/v2-decision-drafts; PR #33, a draft, not merged; base main 9e7db28). Today you approved plan revision 4 (recording the stop's answers; your verdict: ${C}/advisor-verdict-r4.md); that work is applied (03ef222, 59d41e0) and its last verification (fixcheck-e) is finishing. Mustafa has now sent design answers, verbatim in ${C}/inputs/DESIGN-ANSWERS-mustafa-2026-11-03.md: "Record them in the README's design questions and in STATUS (in PR #33, or a docs follow-up if it's merged). They don't change the spike order."

The plan to review: ${C}/plan-r5-design-answers.md. Check it against his words and the docs (docs/adr/README.md rows 49–53, 58, 102, 111, 114, 115, 136, its open-count lines 23–25 and its "Design questions" section; docs/STATUS.md; the drafts' entries it leaves untouched). Judge: does it record his answers faithfully (word for word, "leaning", "not final", "assumed, to confirm", "still open", the Advisor's lean and pencil marked as the Advisor's), keep his scope (README and STATUS), keep each author to its own text, avoid over-reading, and fit plan revision 4's close (your conditions 7–9)? Rule on its decision DRAFT-ENTRIES, and on anything else it should say (for example how README's open count reads when README records answers the drafts' entries do not yet carry, and whether the name "amplifications" needs a home beyond README and STATUS). Verdict: APPROVE, APPROVE WITH CONDITIONS, REVISE or ESCALATE. Write it, verbatim as you would give it, to ${C}/advisor-verdict-r5.md (notes in the same file; if it ends with DONE, answer from it). You edit no doc and never commit or push.

Save as you go: write your verdict file as you work (notes first, the verdict block when composed); no binaries and no file over 1 MB in any scratch folder.

Return the structured result: the verdict, the conditions, each ruling, any escalation, and your file's path.`,
  { label: 'advisor: plan revision 5', phase: 'Review', agentType: 'advisor', effort: 'xhigh', schema: SCHEMA })
return r
