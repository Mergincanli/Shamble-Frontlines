export const meta = {
  name: 'npt-plan-review',
  description: 'Netcode proof test plan, step 4: adversarial review of plan revision 1 by security-auditor, engine-architect and the verifier',
  phases: [{ title: 'Review', detail: 'three independent reviewers, in parallel (3 agents)' }],
}

const R = '/home/user/Shamble-Frontlines'
const P = R + '/.checkpoints/netcode-proof-test'

const SAVE = (agent) => `## Saving your progress
A usage limit can cut you off part way, and you would then be started again with this same task. Keep notes in
${P}/${agent}-plan-review.md (make the folder; you may always write there). Look there first: if the first line names
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

const COMMON = `You are reviewing a change-set PLAN (nothing is built yet) for SHAMBLE FRONTLINES: plan revision 1 of the netcode
proof test (docs/SPIKE_PLAN.md item 1), at ${P}/plan.md (about 700 lines). Its sources sit beside it, each line-cited:
netcode-architect-digest.md (every PM-0001 mark with its setup, problems H1-H20), infra-engineer-digest.md (ADR 0006's
marks, Hetzner facts and prices, routes, Safari runners, DNS, certificates), test-engineer-digest.md (L1/L2, the client
harness, spikes/ rules, the container's toolchain, planted defects) and ssh-probe.md (this session cannot SSH; HTTPS
443 only, through a TLS-re-terminating gateway). The drafts are docs/adr/0001-0006 and docs/SPIKE_PLAN.md (merged).

Settled inputs (Mustafa, 2026-11-05; never reopen them): ADR 0001-0006 and SPIKE_PLAN as merged; P2 = 100 ms round trip,
+-20 ms jitter, 2 % loss, half each way; the 60 Hz run is an early signal only and outside this change-set; RS = CCX23,
LG = CAX21, both in fsn1, created and deleted by Mustafa per run; SSH only through a public key, never a Hetzner token;
each run's plan states its hours and cost cap for his OK; Safari tries GitHub's macOS runners first and any .github/
change goes to him for approval; shamblefrontlines.com at Cloudflare, one DNS record per test host, always "DNS only".

Your job: an ADVERSARIAL review in your lens. Find what is wrong, unsafe, missing, unproven or inconsistent with the
drafts; do not restate or praise the plan. Read the plan whole first, then check it against the sources your lens
needs. For each finding give: an id, a severity (Critical = unsafe or makes the test invalid; High = must change before
approval; Medium = should change before the step it affects; Low = note), the plan section, the problem, the evidence
(file:line or URL with the date read), and the exact change you want in the plan. Keep findings to what matters; at
most about 20. Write nothing outside your notes file: never edit the plan or any repository file.`

const SCHEMA = {
  type: 'object',
  properties: {
    verdict: { type: 'string', description: 'APPROVE, APPROVE WITH CONDITIONS, or REVISE (verifier: PASS, PASS WITH NOTES or FAIL)' },
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
    picks: { type: 'string', description: 'your picks or rulings on the readings in your lens (e.g. R-2, R-4, R-21, R-24), one line each; empty if none' },
    checked: { type: 'array', items: { type: 'string' }, description: 'verifier: each claim checked: claim | source | result; others: what you read' },
    notes_file: { type: 'string' },
  },
  required: ['verdict', 'findings', 'picks', 'checked', 'notes_file'],
}

const SEC = `${COMMON}

Your lens: security of the run setup and of what faces the internet. Threat-model first (assets, attackers, entry
points while a VM lives), then check at least:
1. The SSH key (plan section 1.5): does it meet SPIKE_PLAN:153-156 ("made and held without the session reading a secret
   it did not make")? Leak paths: the laptop session's transcripts and checkpoints, the results branch, chat, pastes.
2. Host-key checks (1.6, R-4): pick a method for RS and one for LG, with reasons; find flaws in the HTTPS-published
   fingerprints idea (order of issuance and publication, an attacker holding the address during http-01, a stale DNS
   record pointing at someone else's VM) and in the console option.
3. The wss certificate (8.4, R-2): http-01 on RS with TCP 80 open to all while issuing, staging first, the ledger, the
   6-day profile, CT exposure. Pick the method with conditions (SPIKE_PLAN:173-175 says this plan sets it with you).
4. Firewall and exposure (8.1, 8.2): the page host on TCP 31000 open to all (what may it serve, headers, no listing),
   the hash and host-key files, TCP 80, the exporter, ICMP on LG, IPv6 left on the VMs but with no AAAA record.
5. cloud-init (1.3, 4.4, 10): no secret in it; user data readable by every process on the VM; rustup-init by pinned
   SHA-256; clone at a full SHA with --locked; sshd hardening; what else it must do or must not do.
6. The Safari workflow shape (9): permissions, inputs and script injection, artifacts, pinned actions, who can dispatch
   on a public repository, what a hostile edit could gain. (ADR 0006:546-566 and .github/workflows/gate.yml show the
   repo's conventions.)
7. Public-repository hygiene (10, R-21): raw data, the day's VM addresses, Mustafa's home address, PM-0006-21's canaries.
8. The two-session split (12.3, R-23): the laptop session's own rules, credentials and settings (which .claude/ rules
   and hooks load there, and from which branch), its pushes to claude/npt-results, and what it must never do.
9. Hostile-input coverage for hours online (PM-0001-25, -26, -34, -59) and anything missing for an internet-exposed
   throwaway server; whether the flood tests between our own VMs risk a Hetzner abuse flag.
10. R-24 (a browser that cannot pin a hash) and any other reading in your lens.
Sources you may need: ADR 0006 (grep part 5, part 13, 'secret', 'B9', 'B12'), ADR 0001 part 10 (lines 419-473), the
infra digest sections E-H, docs.hetzner.com, docs.github.com, developers.cloudflare.com (WebFetch; others by
WebSearch).

${SAVE('security-auditor')}`

const ENG = `${COMMON}

Your lens: module boundaries, the Rust <-> TypeScript boundary, data flow, dependency rules, and every engine feature
tracing to a VISION_V2 section 6 need (a generic feature "for later" is REVISE). Check at least:
1. The layout (plan 4.1) against ADR 0002's rules for spikes/ (test-engineer digest C; ADR 0002:140-145, 944-947).
2. The crate split (4.1-4.3): npt-sim with no I/O, clock or transport dependency; npt-proto; npt-net's thin interface
   with L1 and L2 behind it; npt-client native and wasm32; npt-room; bots on npt-client; npt-judge. Cycles, misplaced
   parts, and where the simulated clock that lets L1 drive the server and 48 clients in one process lives.
3. The client harness (E7): built in ADR 0002 part 5 and 6's shape so the boundary spike measures the code it keeps
   (one input call and one step call per tick, datagrams stamped by the host, the Worker owns the connection, the page
   sees only the block: ADR 0002:325-373), and what it must leave to the boundary spike (PM-0002-*, B2 against B3,
   generated types, the isolation flag); does the plan pre-empt any boundary-spike decision?
4. Determinism the marks need (prediction exact to 1 mm needs native == Wasm): the rules the stand-in sim's step must
   follow, against ADR 0004 part 7 and GLOSSARY section 7 (the hash-map ban), and the native-against-Wasm hash in U7.
5. The S17 room loop (ADR 0004 part 5) and whether the plan's loop matches what the sim core spike will build next.
6. "Kept" (R-14; test-engineer digest G11): merging spikes/netcode-proof to main with the commit SHA in ADR 0001's
   results, against ADR 0002's migration plan (the foundation's restructure; nothing in crates/ or packages/ imports
   spikes/).
7. The symbols list (4.3): enough for a plan of this size? What is missing?
8. Anything in the plan that is a generic feature, or that decides something a later spike or the foundation owns.
Sources: docs/adr/0002 (grep part 5, part 6, 'spikes/', 'Worker host'), docs/adr/0004 (part 5, part 7),
docs/ENGINE_OVERVIEW.md, docs/research/GLOSSARY.md section 7, the test-engineer digest B and C.

${SAVE('engine-architect')}`

const VER = `${COMMON}

Your lens: the plan's claims against their sources (the verifier's research spot-check, applied to a plan). Your
verdict is PASS, PASS WITH NOTES or FAIL. Check at least these, each as "claim | source | result":
1. Completeness of section 6.2: every PM-0001-1 to -62 except -39, -40, -63, plus PM-0004-67 and PM-0006-11 to -27,
   appears with a place and session; list any mark missing, duplicated or misplaced (ADR 0001:1511-1578; ADR
   0006:1349-1365; ADR 0004:1193).
2. The profiles P0-P4 and B1 and the half-each-way split as the plan states them (ADR 0001:567-580, 1436-1450).
3. The budget numbers the plan quotes (ADR 0001:718-735): 16.67 ms, 8.33 ms, 50 us, 20 us, 1 MB, 64 MB, 160 and
   256 kbps, 48 kbps, the 1,000 B datagram cap until measured.
4. What the macOS runner may stand in for (SPIKE_PLAN:49-57; ADR 0001:1394-1404) and the named-machine rule (ADR
   0001:1406-1408; SPIKE_PLAN:156-160) as sections 6.3 and 9 state them.
5. Hetzner prices and the "price includes IPv4" claim (www.hetzner.com/cloud pages; fetch them today), and the
   rounding and internal-traffic claims (docs.hetzner.com billing FAQ).
6. Safari and SafariDriver versions on the macos-26 and macos-15 arm64 images (github.com/actions/runner-images, the
   image READMEs; fetch them today), and that workflow_dispatch runs only from a file on the default branch
   (docs.github.com).
7. Let's Encrypt's rate limits (5 per exact set of names per 7 days; 50 per registered domain per 7 days) and the
   6-day profile (WebSearch; letsencrypt.org is not on the fetch allowlist: cite what you find).
8. The cost arithmetic of sections 7.1 and 7.2 (recompute every row and the totals).
9. ADR 0006 part 5's firewall list (ADR 0006:225-242) against sections 8.1-8.2, and PM-0006-15's "equals" (0006:1353).
10. The reorder arithmetic of R-5 (uniform +- half the jitter per direction; 30 Hz spacing 33.33 ms) and the packet
    count of R-6 (ADR 0001:1513; ADR 0002:1301, 1335-1338).
11. That today's gate reads only the five legacy files (tools/gate.mjs) and CI has no Rust job (.github/workflows/gate.yml).
12. The 60 Hz group's exclusion (ADR 0001:1580-1585) and SPIKE_PLAN:141's mark list.
13. SPIKE_PLAN:153-156 (the SSH key), 65-66 (host keys) and 173-175 (the certificate method) as the plan quotes them.
14. Any other number or quote in the plan that a decision rests on (spot-check at least three more).
A finding is any claim that is wrong, unsourced or overstated; give the correct value with its source.

${SAVE('verifier')}`

phase('Review')
const jobs = [
  { key: 'security-auditor', prompt: SEC },
  { key: 'engine-architect', prompt: ENG },
  { key: 'verifier', prompt: VER },
]
const out = await parallel(jobs.map(j => () =>
  agent(j.prompt, { label: `review:${j.key}`, phase: 'Review', agentType: j.key, effort: 'xhigh', schema: SCHEMA })
    .then(r => ({ key: j.key, result: r }))))
const got = out.filter(Boolean).filter(o => o.result)
log(`reviews returned: ${got.length} of ${jobs.length}`)
return { sent: jobs.map(j => j.key), returned: got.map(g => g.key), results: got }
