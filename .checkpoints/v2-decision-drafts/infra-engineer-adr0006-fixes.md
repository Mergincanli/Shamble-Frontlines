# ADR 0006 fixes (plan step 9), infra-engineer, 2026-11-01

Against `docs/adr/0006-infrastructure-operations.md` as it stands after commit cf00def. Each BEFORE is copied from the
file by line number (whole lines) and is unique in it. Findings 3, 5, 6 and 14 are not mine (assembly, the pass-mark
table, the ledger). Finding 11: "inferred" is replaced by the verifier's confirmation at all three places, and the
branch-alias gap is named. Finding 4 also changes Appendix A's count: adopted 14 → 13, changed 3 → 4. Finding 8 also
fixes line 304 (the same Sentry quote, with the source's apostrophe).

### FIX 1 (part 10, line 300)
BEFORE:
    created in Sentry's EU location; `userInfo` off, so the SDK does not send IP addresses. Tab crashes: Reporting API
AFTER:
    created in Sentry's EU location; `userInfo` off, so the SDK infers no IP address, and its deny lists for HTTP
    headers, cookies and query parameters set too, because IP addresses can still reach Sentry through them
    (r6-infrastructure.md, Findings §3, R6-OBS-F10, lines 327–331, "inferred" there; Sentry's data-collected page says it
    in its own words, read by the verifier on 2026-11-01). In the browser the SDK sends only the Referer and User-Agent
    headers, and no cookies unless its httpClient integration is on (the same page, read by the skeptic on 2026-11-01),
    so by default the query parameters in the URLs it sends are the channel left. Tab crashes: Reporting API
END

### FIX 1 (Options Q8, line 469)
BEFORE:
| **Sentry browser SDK with `wasmIntegration`, debug files uploaded at build (C1, proposed)** | Medium: a CI upload job (ADR 0002) | Free tier, then per error (R6-COST-F3, r6 lines 229–235; base prices not read) | Wasm frames with symbols, "still developing" (R6-OBS-F7, r6 lines 297–306) | EU storage chosen at creation; IP off by setting (R6-OBS-F9, F10, r6 lines 318–331); a CI secret | None | From zero | **Pros:** Rust function names in traps. **Cons:** an unproven Wasm path |
AFTER:
| **Sentry browser SDK with `wasmIntegration`, debug files uploaded at build (C1, proposed)** | Medium: a CI upload job (ADR 0002) | Free tier, then per error (R6-COST-F3, r6 lines 229–235; base prices not read) | Wasm frames with symbols, "still developing" (R6-OBS-F7, r6 lines 297–306) | EU storage chosen at creation; IP off by setting plus deny lists for headers, cookies and query parameters (R6-OBS-F9, F10, r6 lines 318–331; part 10); a CI secret | None | From zero | **Pros:** Rust function names in traps. **Cons:** an unproven Wasm path |
END

### FIX 2 (Options Q3, line 426)
BEFORE:
| Hetzner dedicated root servers; Netcup, OVHcloud, Scaleway | not researched (R6-HOST-OQ1, OQ4, r6 lines 852–855 and 862–865) | not researched | not researched | not researched | not researched | not researched | Read only if OQ-0006-1 rules Hetzner Cloud out |
AFTER:
| Hetzner dedicated root servers; Netcup, OVHcloud, Scaleway | not researched (R6-HOST-OQ1, OQ4, r6 lines 852–855 and 862–865) | not researched | not researched | not researched | not researched | not researched | Netcup, OVHcloud, Scaleway: read only if OQ-0006-1 rules Hetzner Cloud out; Hetzner's dedicated root servers, "a further candidate at goal scale", are read before part 8's goal budget is fixed, whatever OQ-0006-1's answer (R6-HOST-OQ4, r6 lines 863–865) |
END

### FIX 2 (OQ-0006-1, lines 601–604)
BEFORE:
  - *Options:* (a) orderable: the reference server stands; (b) a real shortage: read the fallbacks first, Fly.io's UDP
    page, GameLift's EU prices and server SDK, Hetzner's dedicated root servers, and Netcup, OVHcloud or Scaleway (none
    read: R6-HOST-OQ1, OQ4, r6 lines 852–855 and 862–865), then name a new reference server, which every draft citing N34
    follows. The IPv4 price is read at sign-up; it is under 1 EUR per VM either way (R6-HOST-OQ7, r6 lines 875–876).
AFTER:
  - *Options:* (a) orderable: the reference server stands; (b) a real shortage: read the fallbacks first, Fly.io's UDP
    page, GameLift's EU prices and server SDK, and Netcup, OVHcloud or Scaleway (none read: R6-HOST-OQ1, OQ4, r6 lines
    852–855 and 862–863), then name a new reference server, which every draft citing N34 follows. Whatever the answer,
    Hetzner's dedicated root servers, "a further candidate at goal scale", are read before part 8's goal budget is fixed
    (R6-HOST-OQ4, r6 lines 863–865; migration step 1). The IPv4 price is read at sign-up; it is under 1 EUR per VM
    either way (R6-HOST-OQ7, r6 lines 875–876).
END

### FIX 2 (Migration plan step 1, line 938)
BEFORE:
   figure; PREV-DELETE settled, or the client host moved to Workers static assets.
AFTER:
   figure, fixed only after a read of the price and location pages of Hetzner's dedicated root servers (R6-HOST-OQ4,
   r6 lines 863–865); PREV-DELETE settled, or the client host moved to Workers static assets.
END

### FIX 2 (Appendix A row R6-HOST-OQ4, line 1065)
BEFORE:
| R6-HOST-OQ4 | open: OQ-0006-1 | the fallbacks are read only if Hetzner is ruled out |
AFTER:
| R6-HOST-OQ4 | open: OQ-0006-1 | Fly.io, GameLift, Netcup, OVHcloud and Scaleway are read only if Hetzner Cloud is ruled out; Hetzner's dedicated root servers are read before part 8's goal budget is fixed, whatever OQ-0006-1's answer (r6 lines 862–865; migration step 1) |
END

### FIX 4 (part 5, line 181)
BEFORE:
   TCP range for the WebSocket fallback (R2-TRN-REC3, as R6-HOST-REC2 says, r6 lines 648–649), plus the one rule part 9's metrics path may need, and nothing
AFTER:
   TCP range for the WebSocket fallback (R2-TRN-REC3, as R6-HOST-REC2 says, r6 lines 648–649), plus the one rule part 9's metrics path may need
   (proposed: a change to R6-HOST-REC2's "plus nothing else", r6 line 649, because a Prometheus scrape needs an inbound
   port while a push needs none, R6-OBS-F4, r6 lines 279–284; spike 2 checks it), and nothing
END

### FIX 4 (Appendix A row R6-HOST-REC2, line 1050)
BEFORE:
| R6-HOST-REC2 | adopted | part 5: a working pick until OQ-0006-1 is answered; its marks restated for N = 48 at 30 Hz (spike 2) |
AFTER:
| R6-HOST-REC2 | changed: one firewall rule added for part 9's metrics path | part 5: R6-HOST-REC2 opens the two game ranges "plus nothing else" (r6 line 649); a Prometheus scrape needs an inbound port (R6-OBS-F4, r6 lines 279–284), so the rule is proposed and spike 2 tests it; otherwise as recommended: a working pick until OQ-0006-1 is answered; its marks restated for N = 48 at 30 Hz (spike 2) |
END

### FIX 7 (part 7, line 225)
BEFORE:
   netcode proof test runs on the development certificates; the production form is needed from the first online
AFTER:
   netcode proof test runs on the development certificates for WebTransport (which certificate its WebSocket fallback
   presents is OQ-0006-5); the production form is needed from the first online
END

### FIX 7 (OQ-0006-5, lines 637–646)
BEFORE:
- **OQ-0006-5. How are the per-hostname production certificates issued, and how long do they live?**
  - *Why it matters:* every production game VM needs one before the first online playtest, and the challenge decides
    which port or secret each VM needs.
  - *Options:* ACME http-01 (TCP port 80 open on each game VM); dns-01 per name (a DNS provider API key, a secret);
    tls-alpn-01 (RFC 8737: "This connection MUST use TCP port 443."); and, only if the security baseline accepts a
    shared key, the wildcard certificate (option C2) (R6-HOST-F24, r6 lines 198–204; R6-HOST-OQ6, r6 lines 869–874).
  - *Decider:* security-auditor in the security baseline, with the port plan of ADR 0001's transport.
  - *What decides it:* a read of the chosen certificate authority's policy (Let's Encrypt's pages were outside the
    allow list) and the baseline's key-handling rules.
  - *Carries:* R6-HOST-OQ6.
AFTER:
- **OQ-0006-5. How are the per-hostname production certificates issued, how long do they live, and which certificate
  does the WebSocket fallback present on development and staging servers?**
  - *Why it matters:* every production game VM needs one before the first online playtest, and the challenge decides
    which port or secret each VM needs. A WebSocket cannot pin a certificate hash: its constructor takes only a URL and
    sub-protocols (r2-netcode.md, Findings, R2-ROOM-F12, lines 1494–1500), so the fallback's `wss` endpoint on a
    development or staging server needs a certificate the browsers trust, and the netcode proof test's WebSocket leg
    needs one first (PM-0006-15).
  - *Options:* for production, ACME http-01 (TCP port 80 open on each game VM); dns-01 per name (a DNS provider API
    key, a secret); tls-alpn-01 (RFC 8737: "This connection MUST use TCP port 443."); and, only if the security baseline
    accepts a shared key, the wildcard certificate (option C2) (R6-HOST-F24, r6 lines 198–204; R6-HOST-OQ6, r6 lines
    869–874). For the WebSocket fallback on development and staging servers: not researched (R2-TRN-REC5's
    self-signed, pinned certificates, r2 lines 2700–2703, serve WebTransport only).
  - *Decider:* security-auditor in the security baseline, with the port plan of ADR 0001's transport; the development
    and staging case before the netcode proof test starts (the Advisor, from a short read).
  - *What decides it:* a read of the chosen certificate authority's policy (Let's Encrypt's pages were outside the
    allow list) and the baseline's key-handling rules; for development and staging, a read of how those servers can
    present a trusted certificate.
  - *Carries:* R6-HOST-OQ6 (and the verifier's finding 7, which has no research id).
END

### FIX 8 (Context, finding 5, line 77)
BEFORE:
5. Sentry's EU storage is chosen when the organization is created "and can't be changed", and its browser SDK sends
AFTER:
5. Sentry's EU storage is chosen when the organization is created: "once selected, your data storage location can’t be
   changed", and its browser SDK sends
END

### FIX 8 (part 10, line 304; same quote, apostrophe as in the source)
BEFORE:
    which "can't be changed" later (R6-OBS-F9, r6 lines 318–326), after he has read Sentry's prices (OQ-0006-8); its upload
AFTER:
    which "can’t be changed" later (R6-OBS-F9, r6 lines 318–326), after he has read Sentry's prices (OQ-0006-8); its upload
END

### FIX 8 (Trade-off, lines 533–534)
BEFORE:
  is less tidy than R6-COST-REC1's single range, but A-2 itself says 8 rooms per VM is "a density to measure, not one
  the server tick budget guarantees" (r6 lines 516–522): at the server tick budget's ceiling, 50 % of one vCPU per room,
AFTER:
  is less tidy than R6-COST-REC1's single range, but A-2 itself says 8 rooms per VM is "a density to measure at the tick
  rate R1-TICK-REC2's rule picks (R2-ROOM-REC2 mark 2), not one the server tick budget guarantees" (r6 lines 516–522;
  that rate is now TICK-30's 30 Hz): at the server tick budget's ceiling, 50 % of one vCPU per room,
END

### FIX 8 (part 13, line 379)
BEFORE:
    host's deploy token is an account-scoped "Cloudflare Pages, Edit" API token, the scope the research read
AFTER:
    host's deploy token is an API token with the scope the research read, "select *Account*, *Cloudflare Pages* and *Edit*"
END

### FIX 8 (Appendix A row R5-PREV-REC5, line 1037)
BEFORE:
| R5-PREV-REC5 | changed: PREVIEW-WALL (a) applied | part 2: offline previews public but unlisted unless they carry unreleased art; every multiplayer preview behind the access wall from the first one, so the mark "opens in Brave without a login" holds only until then (OQ-0006-4) |
AFTER:
| R5-PREV-REC5 | changed: PREVIEW-WALL (a) applied | part 2: offline previews public but unlisted unless they carry unreleased art; every multiplayer preview behind the access wall from the first one, so the mark "a preview link opens in Brave on Windows without a login" (r5 line 1649) holds only until then (OQ-0006-4) |
END

### FIX 9 (Options Q3 row G1, line 422)
BEFORE:
| **G1. Hetzner Cloud, dedicated vCPU (CCX23 as the reference server, proposed)** | Medium: our own VMs, a supervisor, a firewall; no load balancer, the lobby hands out host and port | CCX23 86.49 EUR a month (0.1386 an hour), 20 TB included, 1 EUR per extra TB (R6-HOST-F15, F16, r6 lines 150–163) | "highly predictable resources", sold for gaming servers (R6-HOST-F14, r6 lines 144–149) | Firewall filters UDP, stateful, drops what no rule allows (R6-HOST-F18, r6 lines 169–174) | Fair server ticks serve VISION_V2 §4's "fair up to about 100 ms of ping" | From zero | **Pros:** predictable tick; egress free at full load. **Cons:** EU plans showed "currently unavailable" on 2026-10-28 (R6-HOST-F20, r6 lines 180–182); a single provider |
AFTER:
| **G1. Hetzner Cloud, dedicated vCPU (CCX23 as the reference server, proposed)** | Medium: our own VMs, a supervisor, a firewall; no load balancer, the lobby hands out host and port | CCX23 86.49 EUR a month (0.1386 an hour), 20 TB included, 1 EUR per extra TB (an inferred reading: R6-HOST-F15, F16, r6 lines 150–163) | "highly predictable resources", sold for gaming servers (R6-HOST-F14, r6 lines 144–149) | Firewall filters UDP, stateful, drops what no rule allows (R6-HOST-F18, r6 lines 169–174) | Fair server ticks serve VISION_V2 §4's "fair up to about 100 ms of ping" | From zero | **Pros:** predictable tick; egress free at full load. **Cons:** EU plans showed "currently unavailable" on 2026-10-28 (R6-HOST-F20, r6 lines 180–182); a single provider |
END

### FIX 10 (Trade-off, line 517)
BEFORE:
  default, an access policy and deletion through Wrangler (R6-HOST-REC1, r6 lines 627–634). The strongest argument
AFTER:
  default, an access policy and deletion through Wrangler (R6-HOST-REC1, r6 lines 627–634; r5-toolchain-quality.md,
  R5-PREV-REC2, lines 1607–1609). The strongest argument
END

### FIX 11 (part 2, line 151)
BEFORE:
   (r6-infrastructure.md, Findings §1, R6-HOST-F4, lines 80–86, inferred), so the game itself stays public.
AFTER:
   (r6-infrastructure.md, Findings §1, R6-HOST-F4, lines 80–86, inferred there; Cloudflare's preview-deployments page
   states it, read by the verifier on 2026-11-01), so the game itself stays public.
END

### FIX 11 (OQ-0006-4, line 635)
BEFORE:
    domain: R6-HOST-F4, r6 lines 80–86, inferred).
AFTER:
    domain: R6-HOST-F4, r6 lines 80–86, which Cloudflare's page confirms, read by the verifier on 2026-11-01; whether
    branch-alias links are covered is not stated there).
END

### FIX 11 (What a spike must prove, line 732)
BEFORE:
  (OQ-0006-4; R6-HOST-F4, inferred), and Mustafa logs in once per Brave session and the link works after that
AFTER:
  (OQ-0006-4; R6-HOST-F4, confirmed on Cloudflare's page by the verifier on 2026-11-01, but not for branch-alias
  links), and Mustafa logs in once per Brave session and the link works after that
END

### FIX 12 (Context, line 82)
BEFORE:
  CI workflows, the preview build and deploy job, the generated header sets, the Sentry upload job and how a
AFTER:
  CI workflows, the preview build and deploy job, the generated
  [header sets](../research/GLOSSARY.md#the-production-header-set-and-the-preview-header-set), the Sentry upload job and how a
END

### FIX 13 (part 2, line 145)
BEFORE:
   R5-PREV-REC5, lines 1642–1650). Every multiplayer preview, which talks to [the staging game server](../research/GLOSSARY.md#the-staging-game-server), is behind the
AFTER:
   R5-PREV-REC5, lines 1642–1650). That is how this record reads the foundation's "private preview links": under every
   PREVIEW-WALL option, "previews that reach no game server and carry no unreleased art are public but unlisted"
   (GLOSSARY.md lines 623–624, the access wall). Every multiplayer preview, which talks to [the staging game server](../research/GLOSSARY.md#the-staging-game-server), is behind the
END
