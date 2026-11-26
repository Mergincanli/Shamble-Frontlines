# docs-keeper, MAIN-WALL: insert his saved messages into docs/DECISIONS.md, quoted by script, byte-checked.
import hashlib, sys, os
root = sys.argv[1]
notes = os.path.join(root, '.checkpoints/main-wall')
dec = os.path.join(root, 'docs/DECISIONS.md')
CHAT = ("the chat he calls the Guard fix chat, session_019YNMUUc6oiiNVygR9H56Q7, a Claude Code chat on Opus 5.5 on the "
        "branch `claude/confident-gates-tbnq38`")
# (file, UTC time, message id, scope words, bold summary, what it answers (chat's words, not his))
E = [
 ("mustafa-2026-11-26T1837-speed-order.md", "17:37:10", "31a506d9-739f-4757-80ce-4a70f9a81f99", "whole",
  "SPEED-ORDER (b): the review tracks (with SOURCE-GO (a)) come before the guard change-set (R4-1's fix with "
  "GUARD-KIT); the queue after MAIN-WALL is now the review tracks, then the guard change-set",
  "the chat's question SPEED-ORDER, asked after the wall was on (the chat's text, not his): \"should the review tracks "
  "come before the guard change-set? (a) keep your order, guard change-set first; (b) review tracks first.\" His \"B\" "
  "answers it with (b). It changes the order of his item 4 of 2026-11-26, 14:09:28 UTC (below) for these two "
  "change-sets, and the line of his review-tracks brief of 2026-11-25, 17:28:24 UTC that the second guard change-set "
  "\"still goes first\" if it has not started (\"Queued\", above); his interim rule on git and gh stands until R4-1's "
  "fix merges"),
 ("mustafa-2026-11-26T1831-builder-sent.md", "17:31:41", "de4e9b89-d3ea-4b36-b32f-82d18b26e96d", "whole",
  "done, and the chat is to send its block to Builder V5",
  "the chat's message it answers is not in the pushed notes; what \"done\" covers is not recorded here"),
 ("mustafa-2026-11-26T1827-done-all.md", "17:27:17", "2f215f0d-dfd4-45bd-8228-40c62c019460", "whole",
  "done, except deleting the throwaway repository (sandbox-1126); he asks whether to delete it now",
  "the chat's ask it answers is not in the pushed notes; what \"done all\" covers is not recorded here"),
 ("mustafa-2026-11-26T1811-answers.md", "17:11:23", "9d4ec4da-c9b0-46d1-a494-78e936bf6d3b", "whole",
  "WHICH-ACCOUNT: this chat runs on his first personal Claude account, and three Claude accounts of his work on the "
  "project (two personal, one university); TOKEN-RELEASE (c), with his words on speed; T1E-BEFORE-DELETE (b); "
  "CLICK-11-GO (a), his go on adding the wall to this repository; T4-GRADING (a), T4 graded from GitHub's refusal text",
  "the chat's questions after the two reviews of the throwaway repository's results (its checkpoint, steps 15 and 16): "
  "WHICH-ACCOUNT, which Claude account this chat is on (the line that starts \"WHICH-ACCOUNT:\" reads as the chat's "
  "question, pasted back by him, and his answer is the line that starts with \">\" [assumed: from the form]); "
  "TOKEN-RELEASE, when the other lane's token sitting may start, (c): after T10, one day-session merge and one PR "
  "Advisor merge under the wall; T1E-BEFORE-DELETE, whether T1e runs before the throwaway repository is deleted, (b): "
  "it was not run; CLICK-11-GO, his go on click 11; T4-GRADING, (a): T4 graded from GitHub's refusal text, since his "
  "Settings has no Rule Insights page (16:48:48 UTC, below). It replaces T4-BASELINE (b)'s grading by the rule record "
  "(16:15:39 UTC, below). The CLAUDE.md lines that say two accounts are unchanged: a follow-up"),
 ("mustafa-2026-11-26T1801-stalling-own-lines.md", "17:01:04", "1a4fd6e1-bc01-488a-9974-f9a0ddf942b0",
  "his own lines only: the test chat's text he relayed in the same message is left out",
  "his question, in plain terms, on what this chat and the throwaway repository's test chats are for and how they help "
  "the project; Builder V5 has waited hours for MAIN-WALL's merge; the work feels to him like stalling",
  "nothing the chat asked"),
 ("mustafa-2026-11-26T1748-no-insights.md", "16:48:48", "75e2520d-405d-4b9c-98d3-df127adf48d9", "whole",
  "his repository Settings shows no Rules Insights page, only Rulesets: the basis on which T4 was graded from GitHub's "
  "refusal text (his T4-GRADING (a), 17:11:23 UTC, above)",
  "the chat's ask for the rule record (the rule insights) of the throwaway repository [assumed: from the plan's T4 pass mark; the ask is not in the pushed notes]"),
 ("mustafa-2026-11-26T1746-manual-tasks.md", "16:46:59", "4d6c6951-b885-49fa-a13a-579029387c64", "whole",
  "he will not take again the screenshots that show the rulesets saved (he sent them already, and dislikes these manual "
  "tasks); he will add the one of ruleset-1 on gate and smoke",
  "the chat's ask for screenshots of the throwaway repository's saved rulesets [assumed: the ask is not in the pushed notes]"),
 ("mustafa-2026-11-26T1739-setup-done-own-lines.md", "16:39:33", "a07f7ff8-f008-4419-a69f-ccccf41f8fc4",
  "his own lines only: the test chat's text he relayed in the same message is left out",
  "his setup clicks on the throwaway repository done and the test chat started for sandbox-1126; a screenshot of the "
  "ruleset page added",
  "the plan's clicks on the throwaway repository (its checkpoint, step 13: clicks 1, 3, 4 and 5 done)"),
 ("mustafa-2026-11-26T1715-plan-go.md", "16:15:39", "c2bcfe0c-6bf0-4e92-9139-f348c7ee5820", "whole",
  "PLAN-GO (a), his go on MAIN-WALL's plan; T4-BASELINE (b), no baseline run",
  "the chat's two questions, in this order (its checkpoint, step 11): PLAN-GO, his go on the plan at 149bec7 as revised "
  "after the advisor agent's APPROVE; T4-BASELINE, whether the main proof (T4) includes a control with the second "
  "ruleset off: (b), no baseline, T4 passing only on the record naming the second ruleset's update rule, else "
  "\"unproven\". T4's grading was later set by his T4-GRADING (a) (17:11:23 UTC, above)"),
 ("mustafa-2026-11-26T1705-exception-scope.md", "16:05:26", "bbde400f-a630-4daa-83c9-5dcb65ce32f6", "whole",
  "EXCEPTION-SCOPE (d): no exception for his account from today's rule; the plan stands as approved, so his account "
  "does not skip gate and smoke; it replaces his request of 15:57:28 UTC (below)",
  "the chat's question EXCEPTION-SCOPE after his request of 15:57:28 UTC (its checkpoint, steps 9 and 10): sessions act "
  "as his account, so an exception for his account very likely reaches every session too; his (d) is no exception"),
 ("mustafa-2026-11-26T1657-exception.md", "15:57:28", "b3bd1246-ffe0-492f-9199-b3b7d9e5e897",
  "as saved (the saved file ends at an open bracket and a colon)",
  "replaced at 16:05:26 UTC by his EXCEPTION-SCOPE (d) (above), so not in force: his request for an exception for his "
  "account from today's rule (pull request, gate and smoke)",
  "the chat's description of the rule; the first line reads as the chat's text, pasted back by him (it addresses him: "
  "\"meaning your account\"), and his reply is the line that starts with \">\" [assumed: from the form]"),
 ("mustafa-2026-11-26T1628-throwaway-public.md", "15:28:51", "6fe0dfd1-e3ff-478f-9d3b-adbfd5cd9d51", "whole",
  "THROWAWAY-PUBLIC (a): the throwaway repository is public, with neutral names, and deleted as soon as the results are "
  "reviewed",
  "the question THROWAWAY-PUBLIC from the advisor agent's REVISE of the plan (the chat's checkpoint, steps 4 and 6)"),
 ("mustafa-2026-11-26T1545-merge-go.md", "14:56:06", "8248067c-7efd-48cd-ae38-45fd23d9eaad", "whole",
  "his go to merge #73 (guard-wrappers) at exactly 7bff799f34119fd97588bda0edfc25c46d0f6a9e as a merge commit, with "
  "that id as the expected head, after his Advisor's merge check and the advisor agent's verdict on that id; then "
  "MAIN-WALL's plan, in his order; #74 takes main in after this merge; the interim rule on git and gh unchanged",
  "his item 7 of 14:09:28 UTC (below): the final full head id, his Advisor's merge check before his go; #73 merged at 4a62873e (2026-11-26, 14:56:33 UTC, the merge "
  "commit's date, read with git)"),
]
out = []
for f, t, mid, scope, summary, answers in E:
    b = open(os.path.join(notes, f), 'rb').read()
    s = b.decode('utf-8')
    body = s[:-1] if s.endswith('\n') else s
    q = '\n'.join(('> ' + l) if l else '>' for l in body.split('\n'))
    head = (f"- **Taken by Mustafa, 2026-11-26, {t} UTC: {summary}** ({CHAT}; message {mid}, {scope}; copied by script "
            f"from the chat's saved file `.checkpoints/main-wall/{f}` (readable at commit <SHA-NOTES>), which the chat "
            f"saved by script from its transcript). What it answers, in the chat's words (not his): {answers}. Copied by "
            f"script ({len(b)} bytes, sha256 {hashlib.sha256(b).hexdigest()}):")
    out.append((head, q, b))
text = open(dec, encoding='utf-8').read()
anchor = '## Decisions, newest first\n'
assert text.count(anchor) == 1 and 'message 31a506d9' not in text
ins = ''.join(h + '\n' + q + '\n' for h, q, _ in out)
text = text.replace(anchor, anchor + ins)
open(dec, 'w', encoding='utf-8').write(text)
# byte check: re-read, find each block, strip one prefix, compare to source
text = open(dec, encoding='utf-8').read()
for h, q, b in out:
    i = text.index(h) + len(h) + 1
    lines = text[i:].split('\n')
    got = []
    for l in lines:
        if l == '>': got.append('')
        elif l.startswith('> '): got.append(l[2:])
        else: break
    rec = '\n'.join(got)
    src = b.decode('utf-8'); src = src[:-1] if src.endswith('\n') else src
    assert rec == src, h[:80]
    print('OK', len(b), hashlib.sha256(b).hexdigest()[:12], h[24:60])
