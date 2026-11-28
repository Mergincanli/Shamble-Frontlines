export const meta = {
  name: 'npt-plan-digests',
  description: 'Netcode proof test plan, step 2: three line-cited requirement digests (ADR 0001, ADR 0006 + ops, ADR 0002 simulator + harness)',
  phases: [{ title: 'Digest', detail: 'netcode-architect, infra-engineer, test-engineer, in parallel (3 agents)' }],
}

const P = '/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test'

const SAVE = (agent) => `## Saving your progress
A usage limit can cut you off part way, and you would then be started again with this same task. Keep notes in
${P}/${agent}-digest.md (make the folder; you may always write there). Look there first: if the first line names
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

const COMMON = `You are helping plan the netcode proof test (docs/SPIKE_PLAN.md item 1, lines 130-204) as its own change-set in
SHAMBLE FRONTLINES (repository root: /home/user/Shamble-Frontlines). This step writes NO code and edits NO repository
file except your own notes file. The main session writes the plan; your job is a faithful, compact, line-cited digest
of what the merged drafts require in your area, plus your recommendations, always labelled as recommendations.

Settled inputs (Mustafa, 2026-11-05; never reopen them): ADR 0001-0006 and SPIKE_PLAN as merged on main; P2 = 100 ms
round trip, +-20 ms jitter, 2 % loss, split half each way (the loss 2 % on each direction, applied independently); the
60 Hz run is an early signal only and is NOT part of this change-set's close (the proof test closes at 30 Hz; the 60 Hz
margin run later reuses its kept server and harness); RS = a Hetzner CCX23 (x86-64, dedicated vCPU) for the game
server and LG = a CAX21 (Arm64) for the simulated clients, both in Falkenstein (fsn1), created and deleted by Mustafa
himself for each run; the session gets SSH access only through a public key it gives him, never a Hetzner token; each
run's plan states its hours and a cost cap for his OK; the Safari leg first tries GitHub's macOS runners, and any
.github/ change goes to Mustafa for approval; the domain shamblefrontlines.com is registered at Cloudflare, and each
test host gets one exact DNS record from Mustafa, always "DNS only" (grey cloud), so traffic goes straight to the server.

A fact found today (2026-11-05; evidence in ${P}/ssh-probe.md): this cloud session's container cannot reach any server
over SSH (no ssh client in the image; TCP to port 22 and to every port but 443 times out; port 443 passes only through
an egress gateway that re-terminates TLS, so SSH over 443 fails too; arbitrary public HTTPS hosts do work). It also
cannot be a WebSocket or WebTransport client of the test servers (WebSocket upgrades are unsupported by the proxy; QUIC
is UDP). So the plan must propose another route for the hands-on server work: Hetzner's "Cloud config" (cloud-init) at
server creation, or running the hands-on part from Claude Code on Mustafa's own Windows PC, or both.

Rules: cite file:line for every requirement; quote exact numbers, never round them; a number you could not find is
"not found", never a guess. A change to a pass mark or a threshold is a decision for the advisor or Mustafa: list it as
such, never write it as settled. Keep your digest compact (tables with short cells; cite prose by line instead of
restating it; aim under 40 KB). Read big sections in chunks of about 80 lines.`

const RETURN = {
  type: 'object',
  properties: {
    file: { type: 'string', description: 'absolute path of the digest file' },
    sections_done: { type: 'array', items: { type: 'string' } },
    top_issues: { type: 'array', items: { type: 'string' }, description: 'the 5-12 most important problems or decisions for the plan, one line each, with the source line' },
    recommendation: { type: 'string', description: 'one paragraph: your recommended shape for the plan in your area' },
  },
  required: ['file', 'sections_done', 'top_issues', 'recommendation'],
}

const NETCODE = `${COMMON}

Your part: ADR 0001 (docs/adr/0001-multiplayer-netcode.md), the owner of the proof test.
Read, in this order:
1. ADR 0001 "What a spike must prove" (lines 1237-1347), "### Pass marks" (1348-1630), "#### The netcode checklist"
   (1631-1683).
2. ADR 0001 "Budgets for the netcode proof test" (718-736), "Components and data flow" (591-626), "State ownership"
   (627-653), "Message catalogue" (654-682), "Bandwidth estimate" (683-717), "Failure modes" (737-758), "Anti-cheat
   implications" (759-781), "Test plan" (1727-1758); the Decision part that defines the network profiles P0-P4 and the
   bursts (part 13; grep), and the decision parts the marks cite for exact values (tick and snapshot rates, encoding,
   interest, the transport and its crates, the input buffer and command timing, the rewind cap rule).
3. ADR 0001 "Open questions" only for the questions this test decides or depends on (OQ-0001-5, -6, -7, -15, -16 and
   any other a mark names).
4. docs/research/GLOSSARY.md section 2 (the test-mode list), section 3 (the stress scene), section 8 (the netcode proof
   test) and its time-statistics rules.
Do not read docs/research/r2-netcode.md whole; consult it only for a line a mark cites that you cannot read otherwise.

Write your digest to ${P}/netcode-architect-digest.md with these sections:
A. Pass-mark table: one row per mark PM-0001-1 to PM-0001-63 (and any lettered sub-marks): id | what it measures | the
   exact threshold | the run setup it needs (profile, N, transport, browser(s), machine RS/LG/L1/ML/RD/MAC/MP, duration,
   repetitions) | who judges (script or Mustafa) | conditional (the condition) or not | source line. Flag the 60 Hz
   margin run's marks (PM-0001-39, -40, -63: outside this change-set's close) and the rows under the Arm rule
   (PM-0001-17, -19, -48, -54; lines 1337-1346).
B. The profiles P0-P4 and every burst or bottleneck (B1 and the rest) exactly, with lines; how the half-each-way split
   and the 2 % per direction apply.
C. The proof test's budget table, exact.
D. The netcode checklist for Mustafa (in Brave, at P2): its steps, condensed but faithful, with lines; what he must have
   on screen and what he judges.
E. The build list the marks imply: every component (the sim stand-in with its stress-scene counts, the server loop, the
   transports behind one interface, encoding, interest, prediction and reconciliation, interpolation and its adaptive
   delay, clock sync, lag compensation and its history, the input buffer and command timing, bots as protocol clients,
   the load generator, hostile and fuzz clients, the flood, the 8-room density runs, recordings and per-tick state
   hashes, instrumentation of tick time and its shares, bandwidth and memory), each with the marks that need it and the
   decision part that fixes its design.
F. The decisions the test makes (the R2-...-OQ... and OQ-0001-... ids): for each, the rule that decides it and the run
   that feeds it.
G. What else the plan must deliver (for example the message catalogue under docs/design/ that .claude/rules/netcode.md
   asks for; the run-record format; the "reported, not judged" items).
H. Problems: conflicts between marks, marks that look infeasible or unmeasurable as written, browser-support doubts
   (for example serverCertificateHashes and datagrams on Safari and Firefox; the "clean local network" of the
   99-of-100 connection runs; where four browsers physically run), and every place where the session's lack of SSH
   changes a mark's setup. For each, a proposed handling inside the drafts' rules.
I. Your recommended build and run order, in small resumable units (sized S or M), with the runs grouped into VM
   sessions and their rough hours (a recommendation).

${SAVE('netcode-architect')}`

const INFRA = `${COMMON}

Your part: ADR 0006's share of the proof test and the operations of its runs.
Read:
1. ADR 0006 (docs/adr/0006-infrastructure-operations.md): "What a spike must prove" (lines 1167-1242); the "### Pass
   marks" rows PM-0006-11 to PM-0006-27 (inside 1243-1406); "#### The operations checklist" (1430-1437); and the
   Decision parts the marks cite (game servers, the firewall and port ranges OQ-0006-16, certificates and OQ-0006-5's
   development case (i), metrics and logs, egress and the bill, secrets), its key (the machines RS, LG, MV; the day's
   addresses; the SSH host-key checks), and the open questions OQ-0006-5, -15, -16, -17, -19, -20.
2. ADR 0004 (docs/adr/0004-simulation-core.md): PM-0004-67 (S17: the server loop keeps a 30 Hz tick from elapsed time)
   and the online halves of PM-0004-61 and PM-0004-70 (grep the ids).
3. docs/research/GLOSSARY.md "the reference server" and section 8; docs/SPIKE_PLAN.md lines 22-70 (Machines) and
   130-204 (the spike).
4. Official sources, read today, each cited with its URL and the date read (docs.hetzner.com, www.hetzner.com,
   docs.github.com, github.com, raw.githubusercontent.com and developers.cloudflare.com are fetchable; for anything
   else use WebSearch and cite, or write "not fetched"): Hetzner Cloud CCX23 and CAX21 (vCPU, RAM, disk, the hourly and
   monthly price in fsn1, the primary IPv4's price, included traffic and the overage price per TB); Hetzner Cloud
   Firewalls (rules, limits, UDP); Hetzner "Cloud config" (cloud-init user data: format, size limit, when it runs, where
   its output is logged); SSH keys at server creation; GitHub-hosted macOS runners (the current images, arm64 or x64,
   their Safari and safaridriver versions from the image's software list in actions/runner-images, how often images
   update, minutes for public repositories, outbound TCP and UDP/QUIC); Cloudflare DNS ("DNS only" proxy status, the
   record fields, TTL, CAA); Let's Encrypt challenge types (HTTP-01, TLS-ALPN-01, DNS-01) and its rate limits
   (certificates per registered domain per week, duplicate certificates). If a page cannot be fetched, say so; never
   guess a number.

Write your digest to ${P}/infra-engineer-digest.md with these sections:
A. Pass-mark table PM-0006-11 to PM-0006-27, plus PM-0004-67 and the online halves of PM-0004-61 and -70: id | what |
   exact threshold | run setup | who judges | conditional | source line.
B. The operations checklist for Mustafa: its steps, condensed but faithful, with lines.
C. Machines and addresses: RS, LG, MV; how many CAX21s (LG-COUNT, its 80 % rule and 5-minute calibration); what each
   run records (the day's addresses, the host keys); the firewall and port ranges; the metrics role; logs.
D. A Hetzner facts table (URL and date for each); then the cost per hour of one RS + one LG, and of one RS + two LGs,
   as the basis of each run's cap.
E. The route for the hands-on server work, given the session cannot SSH. Assess (1) cloud-init at creation: what it can
   do alone (hardening, firewall, a user, packages, fetching a pinned build from the public repository or a release by
   commit SHA and checksum, a fixed run script, publishing results), how results get back to the session (which can
   read public HTTPS), what Mustafa must paste and when; (2) Claude Code on Mustafa's Windows PC doing the hands-on part
   (the key on his PC, his steps, what the cloud session still does); (3) any hybrid. For each: Mustafa's effort per
   run, security (secrets, attack surface), reliability, and fit with the drafts' rules (no Hetzner token; the VMs
   exist only during a run; the SSH key "made and held without the session reading a secret it did not make",
   SPIKE_PLAN line 154). Recommend one.
F. The Safari leg on GitHub's macOS runners: the facts (image, Safari version on the day against the 26.4 that
   R2-TRN-F7 gives for WebTransport, safaridriver, UDP egress), SPIKE_PLAN's three record conditions (i)-(iii) (lines
   49-57), the rows a runner may stand in for (PM-0001-28's four-browser path and library pick, PM-0001-14, PM-0001-25's
   Origin, PM-0006-15) against those that stay on a borrowed Mac; a proposed workflow shape (trigger, permissions, no
   secrets, pinned action SHAs, what it uploads) with its security points, for security-auditor's and Mustafa's review;
   state that any .github/ change goes to Mustafa for approval.
G. DNS and certificates: for each test host, the exact Cloudflare record (type, name, the value as a placeholder for
   the day's address, TTL, proxy status "DNS only"); whether an AAAA record is needed; a CAA record if you advise one;
   the issuance options for the WebSocket leg's browser-trusted certificate with no Cloudflare or Hetzner token in the
   session (with the rate limits); and the WebTransport development certificate (self-signed, ECDSA, at most 14 days,
   hash-pinned): where it is made and how its hash reaches the clients.
H. Problems: conflicts, infeasible items, missing inputs, each with a proposed handling.

${SAVE('infra-engineer')}`

const TEST = `${COMMON}

Your part: the link simulator, the load layer, the client harness, and the evidence plan.
Read:
1. ADR 0002 (docs/adr/0002-code-structure-build-toolchain.md): the network simulator's design (L1 and L2; R5-TEST-REC4
   and REC5: grep "L1", "L2", "simulator", "R5-TEST-REC4", "R5-TEST-REC5"); the spike text on the client harness the
   netcode proof test builds first and the boundary spike shares (grep "client harness", "Worker host", "plain
   renderer"); SPIKE-SCOPE (a) and every rule for spikes/ (layout, exemptions, toolchain pins, how CI and the gate treat
   it); and its pass marks that name the netcode proof test (grep "netcode proof test" in its "### Pass marks").
2. ADR 0004 (docs/adr/0004-simulation-core.md): part 5 (S17, the server loop's 30 Hz from elapsed time; grep "S17") and
   PM-0004-67; PM-0004-61 and -70 (their online halves).
3. ADR 0001 (docs/adr/0001-multiplayer-netcode.md): "Test plan" (1727-1758), "Migration plan" (1684-1726), and part 13's
   profile and burst definitions only (grep "P0").
4. docs/research/GLOSSARY.md sections 2, 3, 8 and "time statistics".
5. docs/research/r5-toolchain-quality.md only for the exact text of R5-TEST-REC4, R5-TEST-REC5 and R5-TEST-OQ2 (grep).
6. This container's toolchain, read-only: rustc -V, cargo -V, rustup target list --installed, rustup toolchain list,
   node -v, and whether wasm-pack, wasm-bindgen, wasm-opt or clang are on the PATH (command -v). Run nothing else.

Write your digest to ${P}/test-engineer-digest.md with these sections:
A. The link simulator as designed: L1 (where it sits, how it is seeded and made deterministic, what it injects:
   latency, jitter, loss, reorder, duplication, bandwidth caps, bursts) and L2 (where it sits: under a custom quinn
   endpoint, at the OS level, or else; what R5-TEST-OQ2 asks), with lines; how a profile's latency and jitter split
   half each way and the 2 % loss per direction apply.
B. The client harness: what the proof test must build (the page, its module Worker host, the plain renderer, on the
   throwaway sim), what it must leave to the boundary spike, with ADR 0002's exact text and lines.
C. spikes/ rules: the layout, what is exempt, the toolchain pins and versions the drafts fix (Rust toolchain, the wasm
   target, Node, TypeScript, three.js), whether the legacy gate or CI touch spikes/, how a throwaway spike is reviewed
   and kept (the kept server and harness the 60 Hz run reuses); then the container's toolchain as found.
D. S17 / PM-0004-67 and the online halves of PM-0004-61 and -70: the exact marks and setups.
E. The evidence plan under CLAUDE.md's netcode rule ("deterministic tests with injected latency, jitter and loss"): the
   test groups (unit, property, deterministic network tests on L1 with seeds, L2 runs, load runs, soaks, planted
   defects); what runs in this container (Linux x86-64, Rust and Node present, no SSH, HTTPS-only egress) against what
   runs on RS and LG or in browsers; how results are recorded (files and formats), and how each scripted mark gets its
   pass or fail from data.
F. Every planted defect the drafts list for the proof test (grep "planted"), each with the mark that must catch it.
G. Problems, and a recommended evidence and test order in small resumable units (a recommendation).

${SAVE('test-engineer')}`

phase('Digest')
const jobs = [
  { key: 'netcode-architect', prompt: NETCODE },
  { key: 'infra-engineer', prompt: INFRA },
  { key: 'test-engineer', prompt: TEST },
]
const out = await parallel(jobs.map(j => () =>
  agent(j.prompt, { label: `digest:${j.key}`, phase: 'Digest', agentType: j.key, effort: 'xhigh', schema: RETURN })
    .then(r => ({ key: j.key, result: r }))))
const got = out.filter(Boolean).filter(o => o.result)
log(`digests returned: ${got.length} of ${jobs.length}`)
return { sent: jobs.map(j => j.key), returned: got.map(g => g.key), results: got }
