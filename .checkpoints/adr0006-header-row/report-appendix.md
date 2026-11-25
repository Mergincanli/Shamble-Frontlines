**E1. PM-0006-2, the "number and statistic" cell** (now :1393; was :1370)

Before:
> previews `X-Robots-Tag: noindex`, production none; the preview built with

After:
> previews `X-Robots-Tag: noindex`; production none on its own addresses (the project's `pages.dev` host, and a custom domain if CF has one, as in PM-0006-8), while on production's one-off (hash) addresses `X-Robots-Tag` is recorded, not judged (revised 2026-11-25, Mustafa's ONE-OFF-HEADER (a): the preview hosting check read `noindex` on all 7 paths of production's one-off addresses 0354374e and de1c3e5e, written by no rule of the production build; a one-off address works like a preview address, and the header there keeps old builds out of search engines); the preview built with

**E1b. PM-0006-2, the "what it proves" cell** (now :1393; was :1370)

Before:
> The host serves each header set as generated, on both URL forms (part 1; decides R5-PREV-OQ6)

After:
> The host serves each header set as generated, on both URL forms (production's one-off addresses: the host's own `X-Robots-Tag` recorded, not judged) (part 1; decides R5-PREV-OQ6)

**R3-11. PM-0006-2, the "method and tool" cell** (now :1393; was :1370)

Before:
> each read on its branch alias and on its deployment's hash URL;

After:
> each read on its branch alias and on its deployment's hash URL, and production on a custom domain if CF has one;

**E2. PM-0006-2, the bites in "runs and noise"** (now :1393; was :1370)

Before:
> a `noindex` rule added to production |

After:
> a `noindex` rule added to production (fails on production's own address) |

**E3. part 1, the build count** (now :140; was :140)

Before:
> Pages' Free plan allows 500 builds a
> month (R6-HOST-F3, r6 lines 72–79), and whether a direct upload counts against them is a spike item (part 14). If
> uploads count, only the newest push of each pull request deploys (ADR 0002's concurrency cancel) and the month's
> deploys are counted; a paid plan is Mustafa's after a price read (proposed, not researched).

After:
> Pages' Free plan allows 500 builds a
> month (R6-HOST-F3, r6 lines 72–79). Whether a direct upload counts against them is not answered: the preview
> hosting check saw no build count in the dashboard, before or after its ten uploads (2026-11-18), and Cloudflare's
> docs do not say (read 2026-11-24 and 2026-11-25; PM-0006-7; `docs/reports/2026-11-24-preview-hosting-check.md`). If
> uploads count, only the newest push of each pull request deploys (ADR 0002's concurrency cancel) and the month's
> deploys are counted; until someone shows a count, the previews (foundation step 2.6) work that way anyway (Mustafa's
> STEP4-COUNT (b), 2026-11-25, 00:55 Amsterdam time). A paid plan is Mustafa's after a price read (proposed, not
> researched).

**R3-10. part 1, *Why:*** (now :149; was :145)

Before:
> link with no cap on previews, and GitHub Pages cannot send those headers;

After:
> link with no cap on previews (as read on 2026-10-28; Pages' limits page, read 2026-11-25, no longer says so and
> reads "Preview deployments older than 30 days are deleted, unless they are one of the project's 100 most recent
> deployments" (developers.cloudflare.com/pages/platform/limits/): follow-up ADR6-PAGES-RETENTION), and GitHub Pages
> cannot send those headers;

**E7. part 1, a new paragraph after *Need:* (15 lines)** (now :159)

Added:
> *Workers static assets, weighed again* (2026-11-25, after the header mark failed; Mustafa's ONE-OFF-HEADER (a);
> Cloudflare's docs read 2026-11-25, paths under developers.cloudflare.com/workers/ unless marked): none of the
> check's marks was measured on Workers. The host adds `noindex` to `workers.dev` Preview URLs, "but not to custom
> domain Preview URLs" (previews/custom-domains); the docs are silent on production's own address, its `workers.dev`
> URL, its version links and their aliases (versions-and-deployments/version-urls; configuration/routing/workers-dev),
> so they neither show nor rule out the header the check found on Pages' production one-off links. A pull request
> gets a Preview of its branch's files (previews/configuration), at most 100 per Worker on the Free plan, the oldest
> deleted when the limit is reached, and a command that deletes it (previews; wrangler/commands/workers); on Pages,
> PREV-DELETE was met through the cleanup job, with a departure (the check's report, PM-0006-6). A wall can cover
> previews only (configuration/cloudflare-access); `_headers` takes host rules (static-assets/headers); 25 MiB a
> file, as on Pages (developers.cloudflare.com/pages/platform/limits/); no monthly cap on deploys is named
> (platform/limits; platform/pricing). Cloudflare's Pages overview says "Start new projects with Workers"
> (developers.cloudflare.com/pages/). None of this answers a failed mark: the revised header row is met by the
> readings already taken on Pages (the check's report, Appendix A), so Pages stays; a move would mean running the
> check again on Workers.

**E6. spike 1's list** (now :1236; was :1214)

Before:
> - whether a direct upload counts against Pages' 500 builds a month (R6-HOST-REC1, r6 lines 633–634);

After:
> - whether a direct upload counts against Pages' 500 builds a month (R6-HOST-REC1, r6 lines 633–634); not answered by
> the check, which saw no count (part 1);

**E4. PM-0006-7, the "number and statistic" cell** (now :1398; was :1375)

Before:
> anything else recorded as read; if uploads count, part 1's consequence follows (only the newest push per pull request is deployed, and the month's deploys are counted) |

After:
> anything else recorded as read; if uploads count, part 1's consequence follows (only the newest push per pull request is deployed, and the month's deploys are counted), and until a count is shown it holds anyway. Not answered by the preview hosting check (revised 2026-11-25, Mustafa's STEP4-COUNT (b)): the dashboard showed no count before or after the ten uploads (2026-11-18), and Cloudflare's docs do not say (read 2026-11-24 and 2026-11-25) |

**E9. PM-0006-7, the "runs and noise" cell (the advisor agent's words)** (now :1398; was :1375)

Before:
> | one pair of readings; passes when both are recorded |

After:
> | one pair of readings; passes when both are recorded, each as the count the page shows or as "no count shown" (revised 2026-11-25: the advisor agent's reading of Mustafa's STEP4-COUNT (b), never his decision; "no count shown" leaves the question unanswered, part 1) |

**E11. the hosting checklist's intro** (now :1466; was :1443)

Before:
> or the numbers it asks for.

After:
> or the readings it asks for.

**E10. the hosting checklist, step 4 (the advisor agent's words)** (now :1475; was :1452)

Before:
> read the count again. Expected: two numbers recorded (PM-0006-7).

After:
> read the count again. Expected: two readings recorded, each a number or
> "no count shown" (PM-0006-7; revised 2026-11-25, the advisor agent's reading of Mustafa's STEP4-COUNT (b)). Read
> on 2026-11-18, before and after the ten uploads: no count shown, so the question is not answered (part 1).

<!-- entries: 12 -->
