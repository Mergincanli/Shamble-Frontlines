export const meta = {
  name: 'npt-plan-review2',
  description: 'Netcode proof test plan, step 6: review round 2 on revision 2 (security-auditor fixes; netcode-architect and infra-engineer as OQ-0006-16 deciders and owners)',
  phases: [{ title: 'Review 2', detail: 'three reviewers in parallel (3 agents)' }],
}

const R = '/home/user/Shamble-Frontlines'
const P = R + '/.checkpoints/netcode-proof-test'

const SAVE = (agent) => `## Saving your progress
A usage limit can cut you off part way, and you would then be started again with this same task. Keep notes in
${P}/${agent}-plan-review2.md (make the folder; you may always write there). Look there first: if the first line names
another task, move the file into ${P}/old/ and start fresh; if the notes end with DONE, write your answer from
them; otherwise go on after the last block they mark done. Trust the blocks they mark done: never reread, recheck
or redo them (a later block may still read a file it needs). Start a new file with one line naming your task, then
your plan: the blocks you will do, in order (a block: a source read, a file reviewed, a test group run, a section
written). Write each finished block to its file at once. Add a notes line for each finding as you find it (what,
where), and one after each finished block (done, what comes next). Redo a block the notes don't mark done, written
over, never appended. Scratch files go in ${P}/: text and small scripts only; builds and binaries go under
.gate/netcode-proof-test/. This folder is pushed to a public repository: never put a secret, token, environment value,
request header, cookie or raw payload in it, whether in a note, a script or a script's output; the saver's secret
check only knows a few token formats and cannot read archives or binaries. End with DONE, then give your answer
from your notes alone, opening no other file. Never commit or push.`

const COMMON = `You are reviewing REVISION 2 of a change-set PLAN (nothing is built yet) for SHAMBLE FRONTLINES: the netcode proof
test (docs/SPIKE_PLAN.md item 1), at ${P}/plan.md (about 1,020 lines). Revision 1 was commit 786b7d2; review round 1
returned 49 findings, compacted in ${P}/review1-findings.md, and the plan's section 0.1 maps each to its fix. The
line-cited digests beside it (netcode-architect-digest.md, infra-engineer-digest.md, test-engineer-digest.md) and
ssh-probe.md are its sources; the drafts are docs/adr/0001-0006 and docs/SPIKE_PLAN.md (merged).

Settled inputs (Mustafa, 2026-11-05; never reopen them): ADR 0001-0006 and SPIKE_PLAN as merged; P2 = 100 ms round trip,
+-20 ms jitter, 2 % loss, half each way; the 60 Hz run is an early signal only and outside this change-set; RS = CCX23,
LG = CAX21, both in fsn1, created and deleted by Mustafa per run; SSH only through a public key, never a Hetzner token;
each run's plan states its hours and cost cap for his OK; Safari tries GitHub's macOS runners first and any .github/
change goes to him for approval; shamblefrontlines.com at Cloudflare, one DNS record per test host, always "DNS only".

This is round 2: review what changed and what your lens owns, adversarially. Do not re-raise a round-1 finding that is
fixed; do raise a round-1 finding that is not fixed or fixed wrongly, and anything new the fixes introduced. For each
finding: an id, a severity (Critical = unsafe or makes the test invalid; High = must change before approval; Medium =
should change before the step it affects; Low = note), the plan section, the problem, the evidence (file:line or URL
with the date read), and the exact change. At most about 15 findings. Write nothing outside your notes file: never
edit the plan or any repository file. Your verdict: APPROVE, APPROVE WITH CONDITIONS (list them), or REVISE.`

const SCHEMA = {
  type: 'object',
  properties: {
    verdict: { type: 'string' },
    findings: {
      type: 'array',
      items: {
        type: 'object',
        properties: {
          id: { type: 'string' },
          severity: { type: 'string', enum: ['Critical', 'High', 'Medium', 'Low'] },
          section: { type: 'string' },
          problem: { type: 'string' },
          evidence: { type: 'string' },
          change: { type: 'string' },
        },
        required: ['id', 'severity', 'section', 'problem', 'evidence', 'change'],
      },
    },
    round1_status: { type: 'array', items: { type: 'string' }, description: 'security-auditor: one line per round-1 finding F1-F19: fixed / partly (what is left) / not fixed. Others: empty' },
    picks: { type: 'string', description: 'your word on the readings in your lens (R-numbers), one line each' },
    notes_file: { type: 'string' },
  },
  required: ['verdict', 'findings', 'round1_status', 'picks', 'notes_file'],
}

const SEC = `${COMMON}

Your lens: security. You raised F1-F19 in round 1 (your round-1 notes: ${P}/security-auditor-plan-review.md). Check each
one's fix in revision 2 (sections named in 0.1) and report its status. Then threat-model the NEW surface the fixes add:
the local control socket and its users; capabilities carried in URL fragments (browser history, screenshots, Referer,
the page's own scripts, extensions in Mustafa's Brave profile); ProxyJump through RS and RS's forwarding rule; the
laptop session's extra settings file (can --settings add denies that hold, given deny-before-allow, and does the plan
claim more than settings can enforce?) and its folders; redact-check; FIXED-IP (decision 3: a CT-listed name pointing
at a kept Primary IP that is ours but unassigned between runs); the density days with VMs idle and rooms stopped; the
per-room-slot users; the Safari workflow's guard step and the single-use public join token; the ACME client choice and
its fallback. Give your word on R-2, R-4, R-21, R-23, R-24, R-25 and FIXED-IP.

${SAVE('security-auditor')}`

const NET = `${COMMON}

Your lens: you own ADR 0001 and wrote the netcode digest. Check revision 2 against ADR 0001 (and ADR 0004 part 5 for
S17), and in particular:
1. OQ-0006-16 (you are a co-decider with infra-engineer): the port plan in section 8.1 (UDP 30001-30032, TCP
   31000-31032 with the page host on 31000; this plan changed infra-engineer's UDP 30000-30031 / TCP 31000-31031). Give
   your word: agree, or the numbers you want and why.
2. The netcode design changes: the npt-room library vs npt-server binary split and the placement table (4.4); the
   poll-based Link in npt-link with Loopback (4.1, 4.7); L1 in npt-l1 on npt-lab's SimClock and step order; the control
   plane (10.2) and roster (10.3) against ADR 0001's message catalogue and its checks (ADR 0001:654-682, 220-275,
   419-473); the WebSocket delay inside the accept path (R-25) against ADR 0001:1444-1445.
3. The run mapping (6.2, 7.2) against each mark's setup in your digest: every row in the right run with its setup
   (profile, N, transport, machine, length, repetitions), the order constraints (H17: -36 before -14's cap; -48's test
   cap, then -49's rule cap, then checklist steps 7 and 10; the clean-run references before -24/-25), and the hours
   per block (are they credible for the runs the marks ask for?).
4. Your word, as ADR 0001's owner, on R-5, R-6, R-9, R-10, R-11, R-12, R-15, R-16, R-17, R-20 (name R2-XU-OQ7's rule if
   you can find it), R-25 and R-26, and on which of them truly change a mark (and so need R-PR) versus only read it.
5. The checklist plan (section 11 item 3) against ADR 0001:1631-1681.

${SAVE('netcode-architect')}`

const INF = `${COMMON}

Your lens: you wrote the infra digest; you own CI, hosting, servers and costs, and you are a co-decider of OQ-0006-16.
Check revision 2 against ADR 0006 and your digest, and in particular:
1. OQ-0006-16: the port plan in section 8.1 (this plan changed your UDP 30000-30031 / TCP 31000-31031 to UDP
   30001-30032 / TCP 31000-31032 with the page host on 31000). Give your word: agree, or what you want and why.
2. The runs (7.1, 7.2): the hours per block and their working, the density rotation (each N three times of day,
   pairwise 6 h; -27 and PM-0006-13 in D1-D3), the run-day grouping (R-26), the caps and their arithmetic, and the gap
   to OQ-0006-20's estimate.
3. Mustafa's per-run steps (7.3): can every step be done in Hetzner's console as written (a firewall chosen in the
   create form; editing a firewall's sources after creation; a Primary IPv4 kept and assigned at creation; the
   operator key chosen at creation)? Check docs.hetzner.com today. Is the LG-first order with no SSH rule, then RS,
   then LG's SSH source = RS's IPv4, workable? Is FIXED-IP (decision 3) priced and described right (a Primary IPv4's
   hourly and monthly price, billing when assigned vs unassigned)?
4. The firewall lists per IPv4/IPv6 (8.2), the DNS records (8.3), the certificate operations (8.4: the ACME client in
   the workspace, staging, the CT ledger via crt.sh, revocation at teardown), and the Safari workflow rules (9).
5. cloud-init feasibility (10.5): users, sshd drop-in, nftables first, rustup-init per architecture by pinned SHA-256,
   the build as npt-build, the 32 KiB limit; and R1's Windows guard check (12.3).
Give your word on R-1, R-3, R-18, R-19, R-26 and FIXED-IP.

${SAVE('infra-engineer')}`

phase('Review 2')
const jobs = [
  { key: 'security-auditor', prompt: SEC },
  { key: 'netcode-architect', prompt: NET },
  { key: 'infra-engineer', prompt: INF },
]
const out = await parallel(jobs.map(j => () =>
  agent(j.prompt, { label: `review2:${j.key}`, phase: 'Review 2', agentType: j.key, effort: 'xhigh', schema: SCHEMA })
    .then(r => ({ key: j.key, result: r }))))
const got = out.filter(Boolean).filter(o => o.result)
log(`round-2 reviews returned: ${got.length} of ${jobs.length}`)
return { sent: jobs.map(j => j.key), returned: got.map(g => g.key), results: got }
