# infra-engineer: Workers static assets weighed against Pages (ONE-OFF-HEADER (a), addition 2)

Task from the main session, 2026-11-25. Notes only; the main session places the text. No commit.

## Plan
1. Context read (done 2026-11-25): ADR 0006 part 1 :128-151, options row :609, trade-off :725-732, Revisit :795-797,
   Rollback :1556-1558, PM-0006-2 :1370, "When a mark fails" :1404-1416; r6 R6-HOST-F5/F6, r5 R5-PREV-F14 (earlier
   reads 2026-10-28, not reused as evidence: every claim below is from a page read today).
2. Read Cloudflare's official docs today, one block per source, saved at once:
   - S1 Workers Preview URLs (/workers/configuration/previews/) -> Q1 (noindex on preview URLs), Q3, Q5
   - S2 workers.dev routing (/workers/configuration/routing/workers-dev/) -> Q1 (workers.dev), Q5
   - S3 Static assets Headers (/workers/static-assets/headers/) -> Q2 (_headers, host match, Worker responses)
   - S4 Static assets billing and limitations (/workers/static-assets/billing-and-limitations/) -> Q6
   - S5 Workers platform limits (/workers/platform/limits/) -> Q6 (files per version, size, versions kept, deploys)
   - S6 Workers pricing (/workers/platform/pricing/) -> Q6
   - S7 Versions and deployments / wrangler versions commands -> Q3, Q4 (delete a version? alias?)
   - S8 Pages -> Workers migration guide (/workers/static-assets/migration-guides/migrate-from-pages/) and any
     "compatibility matrix" page -> Q7, and Q1 (noindex differences)
   - S9 Pages preview deployments page (noindex wording) for the side-by-side, if needed
   - S10 Access / Workers previews protection, if S1/S2 do not cover Q5
   - Q8 (added by the coordinator, 2026-11-25): do Pages' own docs say whether a direct upload counts against the
     Free plan's 500 builds a month? Pages limits page, direct upload page, pricing page.
   - New after S1: Workers "Previews" (/workers/previews/, compare-workflows, resources) and version retention.
3. Answer Q1-Q8: URL, the page's words, date read, how sure, "not found" where silent.
4. Draft paragraph (max 8 lines x ~118 chars), ADR style; (i) not measured on Workers, (ii) what changes and gains or
   losses, (iii) the outcome under ONE-OFF-HEADER (a). No move written as decided.
5. Advisor before writing the paragraph; then the summary back via SubagentHandback.

## Sources (one block each, written as read)

### S1. Workers: Version URLs (read 2026-11-25)
- URL: https://developers.cloudflare.com/workers/versions-and-deployments/version-urls/ (the old address
  /workers/configuration/previews/ now redirects here; page dateModified 2026-11-19; read as the page's .md export).
- Renamed: "Version URLs, previously called preview URLs, let you access an uploaded version of your Worker before
  deploying it to production."
- NEW since our 2026-10-28 reads: "Version URLs are not Previews ... For branches and pull requests, use
  [Previews](/workers/previews/), which have their own variables, secrets, bindings, and URL." Also for aliases: "For
  branches and pull requests, use Previews instead."
- Forms: "<version-prefix>-<worker-name>.<subdomain>.workers.dev"; aliased: "<alias>-<worker-name>.<subdomain>.workers.dev";
  `npx wrangler versions upload --preview-alias staging`; "Aliases may only be created during version upload."
- Limits: lowercase letters, numbers, dashes; begin with a letter; "The alias and Worker name combined with a dash must
  not exceed 63 characters due to DNS label limits."; "Only the 1000 most recently deployed aliases are retained. When a
  new alias is created beyond this limit, the least recently deployed alias is deleted."
- Deletion: "A Version URL and any aliases pointing to it stop working when the version is deleted under version
  retention. An alias does not prevent its version from being deleted." No per-alias delete command is described.
  "Disabling Version URLs disables routing to Version URLs and aliased Version URLs." (whole-Worker switch).
- Not offered: Workers that implement a Durable Object (incl. Containers, Sandbox); Workers for Platforms user Workers;
  "You cannot configure Version URLs to run on a subdomain other than workers.dev."; "If workers_dev is disabled,
  Version URLs are disabled by default."; no logs for Version URLs (Workers Logs, tail, Logpush).
- Access: "When enabled, Version URLs are publicly available. To require visitors to sign in, use Cloudflare Access.
  Access can protect Version URLs for one Worker or every Worker in an account. You can also protect both production
  and version deployments."
- noindex / X-Robots-Tag: NOT FOUND on this page (silent).
- Static assets on a Version URL: not stated on this page ("uses that Worker version's existing configuration and
  resources").
- Sure: high (page words, current).

### S2. Workers: Previews (read 2026-11-25)
- URL: https://developers.cloudflare.com/workers/previews/ (dateModified 2026-09-24; .md export).
- What: "Previews give each branch an isolated, production-like environment under the same Worker. Each Preview is
  configured with its own variables, secrets, and bindings, served on its own URL, and has its own observability."
  "Worker Previews requires Wrangler 4.135.0 or later."
- Command: "Running `npx wrangler preview` creates or updates a Preview for your current branch under the same Worker."
- URLs: "Preview URL: always serves the latest deployment." "Deployment URL: points to one specific deploy and never
  changes." workers.dev: `<preview-name>-<worker-name>.<subdomain>.workers.dev` and
  `<deployment-id>-<worker-name>.<subdomain>.workers.dev`; custom domain: `<preview-name>.app.example.com` and
  `<deployment-id>-<preview-name>.app.example.com`.
- NOINDEX (Q1): "To prevent search engines from indexing your Preview, use its workers.dev URL, which includes an
  `X-Robots-Tag: noindex` response header. For custom domain Preview URLs, use Cloudflare Access." -> the host adds
  noindex on a Preview's workers.dev URLs; the page does not say whether a Deployment URL on workers.dev also carries it
  (the sentence says "its workers.dev URL"); custom-domain Preview URLs: no noindex stated (Access instead).
- Access (Q5): "Preview URLs are public by default. Use Cloudflare Access to require sign-in. You can protect all
  Previews on an account, one Worker's Previews, or specific hostnames."
- Limits (Q3/Q6): Previews per Worker 100 (Free) / 500 (Paid); Deployments per Preview 100 / 100. "When a limit is
  reached, Cloudflare automatically deletes the oldest to make room" (least recently deployed Preview; oldest deployment).
- Deletion (Q4): "You can also delete Previews yourself with `npx wrangler preview delete --name <preview-name>`. For a
  pull request cleanup example, refer to Delete closed pull request Previews."
- Isolation: "Routes and Cron Triggers target production." Durable Object namespaces provisioned per Preview.
- Sure: high (page words). Gap: static assets per Preview not stated here (see resources page).

### S3. Workers: Previews, Compare workflows (read 2026-11-25)
- URL: https://developers.cloudflare.com/workers/previews/compare-workflows/ (dateModified 2026-11-19).
- "Previews are the recommended way to test changes before production."
- "Do not use Version URLs for branch or pull request testing. Use Previews instead."
- "If you are using aliased Version URLs created with `wrangler versions upload --preview-alias` for branch or pull
  request previews, switch to Previews instead. Aliased Version URLs do not create branch-isolated resources, and they
  stop working when their version is deleted under version retention." Mapping: `--preview-alias staging` ->
  `npx wrangler preview --name staging`.
- Delete with environments: `npx wrangler preview delete --env staging --name <PREVIEW_NAME>`.
- Sure: high. Consequence: R5-PREV-F14's "versions upload --preview-alias" route (r5 lines 852, 1125; the ADR names no
  Workers route) is now the route Cloudflare tells us not to use for pull requests; the current PR route on Workers is `wrangler preview`.

### S4. Workers: Previews, Resources and isolation (read 2026-11-25)
- URL: https://developers.cloudflare.com/workers/previews/resources/ (dateModified 2026-11-21).
- Binding table lists DO, Containers, KV, D1, R2, Queues, etc.; static assets are NOT named in the table or in the
  Limitations section (Service bindings, Workflows, Queue consumers, Cron Triggers, Routes). Not found: whether a
  Preview serves its own uploaded static assets (inferred yes, since a Preview is a deployment of the branch's Worker,
  but the page does not say).
- "Production routes target production. Previews do not take over zone routes, production custom domains ...
  To send HTTP traffic to a Preview, use its Preview URL on workers.dev or a custom domain Preview URL."
- Sure: high on what it says; the static-assets point is "not found".

### S5. Workers: Previews, Custom domains (read 2026-11-25)
- URL: https://developers.cloudflare.com/workers/previews/custom-domains/ (dateModified 2026-11-08).
- Hosts: "Preview URLs can use a custom domain, workers.dev, or both. Enable at least one host to get a Preview URL."
  Custom domain: `<preview-name>.app.example.com` and unique deployment URL `<deployment-id>-<preview-name>.app.example.com`.
  A custom domain can serve "Preview only" (`previews_enabled: true`, `enabled: false`) or "Production and Preview".
  Wildcard DNS and certificate made by Cloudflare (e.g. `*.app.example.com`); "Use a dedicated hostname such as
  previews.example.com if your production domain already has subdomains."
- workers.dev: "Production and Preview workers.dev URLs are configured separately." Switch: `preview_urls` (the same
  Wrangler field the Version URLs page names; its "initial value depends on workers_dev").
- NOINDEX (Q1), the key sentence: "To keep a Preview public while discouraging search engine indexing, add an
  `X-Robots-Tag: noindex` response header. Cloudflare adds this header automatically to workers.dev Preview URLs, but
  not to custom domain Preview URLs. For static assets, add the header through a `_headers` file. For Worker-generated
  responses, add it directly in your Worker code."
- Access (Q5): "To restrict access to custom domain and workers.dev Preview URLs, use Cloudflare Access. This prevents
  unauthenticated users and crawlers from viewing your Preview."
- Not found: whether the automatic noindex also covers the workers.dev unique deployment URL, a Version URL, or the
  production workers.dev URL (the sentence names "workers.dev Preview URLs" only). Can it be turned off on workers.dev?
  Not found. Controlled by choice of host: custom-domain Previews get none unless `_headers` adds it.
- Sure: high on the quoted sentences; the deployment-URL and Version-URL cases are "not found".

### S6. Workers: Previews, Examples (read 2026-11-25)
- URL: https://developers.cloudflare.com/workers/previews/examples/ (dateModified 2026-10-01).
- GitHub Actions: one Preview per PR, `npx wrangler preview --name "pr-<number>" --json`, URL from
  `.preview.urls[0]`; triggers `[opened, synchronize, reopened, closed]`; token "needs permission to edit Workers
  Scripts and the resources used by the Preview"; "Repository secrets are unavailable to forked pull requests by
  default. Before exposing Cloudflare credentials, verify that github.event.pull_request.head.repo.full_name ==
  github.repository."
- Deletion (Q4): "Delete closed pull request Previews": cleanup job on `closed`, `npx wrangler preview delete --name
  "pr-<number>" --skip-confirmation`. The page does not say whether deleting a Preview also stops its unique deployment
  URLs (inferred: they belong to the Preview; not stated). Not measured by us.
- Sure: high (page words); the deployment-URL fate after delete is "not found".

### S7. Workers: Previews, Configuration (read 2026-11-25)
- URL: https://developers.cloudflare.com/workers/previews/configuration/ (dateModified 2026-09-22).
- Static assets in a Preview (Q3): "`assets` | Keep at the top level, even with an `ASSETS` binding. `npx wrangler
  preview` uploads assets from the branch you are previewing." Example config has `"assets": {"directory":
  "./public"}, "previews": {}`. -> a Preview serves its own branch's static assets.
- Sure: high.

### S8. Workers: workers.dev routing (read 2026-11-25)
- URL: https://developers.cloudflare.com/workers/configuration/routing/workers-dev/ (dateModified 2026-11-06).
- "It's recommended to run production Workers on a Workers route or custom domain, rather than on your workers.dev
  subdomain. Your workers.dev subdomain is treated as a Free website and is intended for personal or hobby projects
  that aren't business-critical."
- Access (Q5): "Access can protect one Worker's production workers.dev URL, preview URLs, or both. You can also protect
  all Workers or all Worker previews in an account." Access covers "Version URLs, Preview URLs, and Deployment URLs".
- Off switch: `workers_dev = false` disables the production workers.dev route, but "Disabling your workers.dev route
  does not disable Version URLs, Preview URLs, or Deployment URLs." Adding `routes` infers `workers_dev = false`.
- Browser Integrity Check: "All requests to a workers.dev hostname are evaluated by Browser Integrity Check ... This
  includes Preview URLs and Version URLs. You cannot turn off this check for a workers.dev hostname." Default library
  User-Agents (e.g. Python urllib) "receive error 1010 before your Worker runs". -> PM-0006-2's cookie-less header
  script would need a descriptive User-Agent on workers.dev (relevant to any Workers re-run; not a Pages issue we saw).
- noindex on production workers.dev URL: NOT FOUND (silent).
- Sure: high.

### S9. Workers: Cloudflare Access (read 2026-11-25)
- URL: https://developers.cloudflare.com/workers/configuration/cloudflare-access/ (dateModified 2026-11-02).
- Q5: "A single application: require sign-in on its preview URLs, production URLs, or both." Destinations:
  `preview_worker` ("Preview deployments for one Worker"), `worker` (production and preview), `all_preview_workers`,
  `all_workers`, and hostname-based ("A specific hostname — can be workers.dev, a Custom Domain, or a path").
  Dashboard: "Choose Previews only or All traffic." Worker-level covers "its routes, Custom Domains, workers.dev
  hostname, and previews".
- Static assets: "Workers with Static Assets execute behind an internal router Worker. Access still protects the
  application and its assets."
- Needs "Zero Trust enabled on your account" (Mustafa's act).
- Caveat: "Worker-level Access policies do not currently support WebSocket connections. WebSocket upgrade requests to
  a Worker protected by a worker-level Access policy will fail with a 403 error." (Not our case as drafted: the game's
  WebSocket goes to the game server, not to the client host. Inferred from ADR 0006 parts 1 and 7.)
- Not found: whether "preview deployments" includes the Version URL of the version currently deployed to production
  (Pages' "Restrict previews" walled production's hash links, Appendix A; Workers' docs do not say either way).
- Sure: high on quotes; the production-Version-URL case "not found".

### S10. Workers: Versions & deployments, version retention (read 2026-11-25)
- URL: https://developers.cloudflare.com/workers/versions-and-deployments/ (dateModified 2026-11-19).
- "A version captures the complete state of your Worker at a point in time: its bundled code, static assets, bindings,
  and compatibility settings." -> a Version URL serves that version's own static assets (high, from this sentence).
- Q4: "Versions are kept for 30 days. After 30 days, a version is deleted unless: It is one of the Worker's 100 most
  recent versions. It is currently deployed." "When a version is deleted: ... Its Version URL and any aliased Version
  URLs stop working." No command to delete a version on demand is named on this page or the Version URLs page (not
  found). -> a Version URL / alias cannot be made to stop within 24 h of a PR closing, except by turning Version URLs off
  for the whole Worker (S1). The Previews route has `wrangler preview delete` (S2, S6).
- Wrangler and the dashboard show "the 100 most recent versions and deployments"; deploy/rollback only to the 100
  most recent.
- Version skew: HTML and its assets "can use different versions, even when you deploy to 100% of traffic" (a Workers
  property to note; Pages' atomicity not compared here).
- Sure: high.

### S11. Workers static assets: Headers (read 2026-11-25)
- URL: https://developers.cloudflare.com/workers/static-assets/headers/ (dateModified 2026-11-12).
- Q2 yes: "The default response headers served on static asset responses can be overridden, removed, or added to, by
  creating a plain text file called `_headers` ... in the static asset directory of your project. ... its rules will be
  applied to static asset responses." "Headers defined in the `_headers` file override what Cloudflare ordinarily sends."
- Q2 host match yes: "Using absolute URLs is supported, though be aware that absolute URLs must begin with `https` and
  specifying a port is not supported." Placeholders: "A placeholder can be defined with `:placeholder_name` ...
  Placeholders match all characters apart from the delimiter, which when part of the host, is a period (.)". The page's
  own example: "to prevent your `*.*.workers.dev` URLs from being indexed": `https://:version.:subdomain.workers.dev/*`
  `X-Robots-Tag: noindex`. Same form as Pages' `https://:version.:project.pages.dev/*` (inferred: a preview-only custom
  hostname such as `https://:name.previews.<domain>/*` would match the same way; not tested).
- Q2 Worker code no: "Custom headers defined in the `_headers` file are not applied to responses generated by your
  Worker code, even if the request URL matches a rule defined in `_headers`." (Same as Pages' F2.)
- Limits: "You may define up to 100 header rules. Each line in the `_headers` file has a 2,000 character limit."
  Duplicate headers joined with a comma; rules applied in file order; `! Name` detaches.
- Also: "Cloudflare reserves the right to attach new headers to static asset responses at any time in order to improve
  performance or harden the security of your Worker application." (A standing caveat for "production none".)
- Default `Cache-Control: public, max-age=0, must-revalidate` when no Authorization/Range header.
- Sure: high.

### S12. Workers static assets: Billing and Limitations (read 2026-11-25)
- URL: https://developers.cloudflare.com/workers/static-assets/billing-and-limitations/ (dateModified 2026-11-12).
- Q6: "Requests to static assets are free and unlimited. Requests to the Worker script (for example, in the case of SSR
  content) are billed according to Workers pricing." "There is no additional cost for storing Assets."
- Free-tier caveat: with `run_worker_first`, matching requests "will always invoke your Worker script. If you exceed
  your free tier request limits, these requests will receive a 429".
- Range requests: "Workers returns 200 OK with the full file instead of 206 Partial Content" (not a need of ours today).
- Sure: high.

### S13. Workers: Platform limits (read 2026-11-25)
- URL: https://developers.cloudflare.com/workers/platform/limits/ (dateModified 2026-11-19).
- Q6: "Files per Worker version | 20,000 | 100,000" (Free | Paid); "Individual file size | 25 MiB | 25 MiB";
  "`_headers` rules | 100 | 100"; "`_headers` characters per line | 2,000". Wrangler 4.34.0+ for the higher file count.
- Versions: "Version retention | 30 days"; "Versions always kept | 100 most recent, plus any deployed version".
- Free plan: "Requests | 100,000/day" (Worker invocations; static asset requests are free and unlimited, S12/S14);
  "Number of Workers | 100 | 500".
- Monthly cap on deploys or version uploads: NOT FOUND on this page (no row like Pages' 500 builds a month). The Previews
  limits (100 / 500 Previews per Worker, 100 deployments per Preview, S2) are counts kept, not a monthly cap.
- Sure: high on the table; "no monthly cap" = not found (silence, not a statement).

### S14. Workers: Pricing (read 2026-11-25)
- URL: https://developers.cloudflare.com/workers/platform/pricing/ (dateModified 2026-11-18; prices as read 2026-11-25).
- "The Workers Paid plan includes Workers, Pages Functions, Workers KV, Hyperdrive, and Durable Objects usage for a
  minimum charge of $5 USD per month for an account." "There are no additional charges for data transfer (egress) or
  throughput (bandwidth)."
- Footnote 3: "Requests to static assets are free and unlimited." Example 2: 12 million static-asset requests,
  "Requests to static assets | $0".
- Footnote 4 caveat: "When Workers Caching is enabled, requests served from the Worker's cache are billed at the same
  per-request rate as requests that invoke the Worker. This includes requests to static assets". (An opt-in feature;
  keep it off.)
- Free plan: "100,000 per day" requests (Worker invocations).
- Previews / deploys: no price or monthly cap named (not found).
- Sure: high.

### S15. Workers: Migrate from Pages to Workers (read 2026-11-25)
- URL: https://developers.cloudflare.com/workers/static-assets/migration-guides/migrate-from-pages/ (dateModified
  2026-11-12).
- Q7 (cite only): "Like Pages, requests for static assets on Workers are free, and Pages Functions invocations are
  charged at the same rate as Workers, so you can expect a similar cost structure." "Unlike Pages, Workers has a
  distinctly broader set of features available to it, (including Durable Objects, Cron Triggers, and more comprehensive
  Observability)." "Migrating from Cloudflare Pages to Cloudflare Workers is often a straightforward process." No
  sentence on this page says new projects should choose Workers (not found here).
- Preview environment: "1. Add a `previews` block and enable `workers.dev` Preview URLs"; "Optionally, you can also
  protect these Preview URLs with Cloudflare Access."; "For new Workers created with Workers Builds, preview builds use
  `wrangler preview` by default."
- Asset availability: "Each Worker version has its own set of static assets. HTML and asset requests can use different
  versions while a deployment change spreads across the network".
- Matrix: "Custom HTTP headers for static assets | ✅ | ✅"; "Preview URLs | ✅ | ✅"; "Custom domains outside
  Cloudflare zones | ❌ | ✅" (Workers needs the domain in a Cloudflare zone; he holds shamblefrontlines.com at
  Cloudflare, ADR 0006 OQ-0006-15); "Custom Branch Aliases | ✅ (5)": "Unlike Pages, where you can point any hostname at
  a branch, Workers serves each Preview at `<preview-name>.<your-domain>`"; "Early Hints | 🟡".
- Sure: high.

### S16. Pages: overview page (read 2026-11-25)
- URL: https://developers.cloudflare.com/pages/ (dateModified 2026-08-25).
- Q7, the direction statement, word for word: "Are you sure you want to use Pages? Workers supports most Pages use cases
  and offers a broader feature set. It is Cloudflare's primary platform for building applications. Start new projects
  with Workers." (the first link goes to the migration guide, S15).
- Sure: high (page words). Cite only; not a decision.

### S17. Pages: Limits (read 2026-11-25) (Q8)
- URL: https://developers.cloudflare.com/pages/platform/limits/ (dateModified 2026-11-19).
- Builds: "Each time you push new code to your Git repository, Pages will build and deploy your site. Build limits
  depend on your plan:" table "Builds per month | 500 | 5,000 | 20,000" (Free | Pro | Business); "Builds | 1 build at a
  time"; "Builds will timeout after 20 minutes. Concurrent builds are counted per account."
- Direct upload: NOT FOUND. The page's only description of a build is a Git push; it does not say whether a
  `wrangler pages deploy` direct upload counts against "Builds per month".
- Also (new or newly read): "Deployment retention | 30 days"; "Deployments always kept | 100 most recent, plus your
  current production deployment"; "When a deployment is deleted: ... Its unique preview URL, such as
  `<hash>.<project>.pages.dev`, stops working." Files: 20,000 Free, 100,000 paid; "The maximum file size for a single
  Cloudflare Pages site asset is 25 MiB."
- Sure: high on what it says; Q8 = not found (silence).

### S18. Pages: Direct Upload (read 2026-11-25) (Q8)
- URL: https://developers.cloudflare.com/pages/get-started/direct-upload/ (dateModified 2026-04-21).
- "Direct Upload enables you to upload your prebuilt assets to Pages and deploy them to the Cloudflare global network."
  `npx wrangler pages deploy <BUILD_OUTPUT_DIRECTORY>`. Limits table: "Wrangler | 20,000 files | 25 MiB"; "Drag and drop
  | 1,000 files | 25 MiB". "If you choose Direct Upload, you cannot switch to Git integration later."
- Builds per month / whether an upload counts as a build: NOT FOUND (the page uses "build" only for the user's own
  build command).
- Also read: https://developers.cloudflare.com/pages/how-to/use-direct-upload-with-continuous-integration/
  (dateModified 2026-09-18): GitHub Actions / CircleCI / Travis examples with `wrangler pages deploy`; no word on the
  monthly build count (NOT FOUND). https://developers.cloudflare.com/pages/platform/known-issues/ (dateModified
  2026-11-06): direct upload named for own build platforms; no word on the build count (NOT FOUND).
- Sure: high (silence on all three).

### S19. Pages: Functions pricing (read 2026-11-25) (Q8, the docs' only Pages pricing page)
- URL: https://developers.cloudflare.com/pages/functions/pricing/ (dateModified 2026-09-08). No
  /pages/platform/pricing/ page exists (404 on 2026-11-25).
- "On both free and paid plans, requests to static assets are free and unlimited. A request is considered static when
  it does not invoke Functions." Functions requests count toward the Workers quota (100,000 a day on Free).
- Builds: NOT FOUND (the page does not mention builds).
- Sure: high.

### S20. Pages: Preview deployments (read 2026-11-25) (side-by-side for Q1)
- URL: https://developers.cloudflare.com/pages/configuration/preview-deployments/ (dateModified 2026-11-19).
- "By default, every preview deployment generated by Cloudflare Pages includes the X-Robots-Tag: noindex HTTP response
  header." No switch to turn it off is described; production's hash URLs are not mentioned (the check measured that the
  host adds it there too: Appendix A, 0354374e and de1c3e5e, 2026-11-18).
- Access: "Enabling the access policy will restrict viewing project deployments to your Cloudflare account."
- Hash URLs "keep pointing to the same deployment until it is deleted. Deployments older than 30 days are deleted,
  except the project's 100 most recent and your current production deployment."
- Delete: `npx wrangler pages deployment delete <DEPLOYMENT_ID> --project-name <PROJECT_NAME>`; "Use the --force ... to
  force deletion of aliased deployments."; Note: "The latest deployment for a branch cannot be deleted."
- Sure: high.

### S21. Cloudflare price and product pages (read 2026-11-25) (Q8)
- https://www.cloudflare.com/plans/developer-platform/ and https://www.cloudflare.com/products/pages/ (the latter via
  the 301 from pages.cloudflare.com): the fetched text names no builds per month, no direct-upload rule, no preview
  limits. Products page: "Cloudflare Pages is part of Cloudflare's compute offerings on the Workers platform."
- Sure: medium (marketing pages may render tables by script that the fetch does not see); the docs pages S17-S19 are
  the authority and are silent too.

### S22. Wrangler commands reference, Workers (read 2026-11-25) (Q4)
- URL: https://developers.cloudflare.com/workers/wrangler/commands/workers/ (dateModified 2026-10-26).
- `preview`: "Create a Preview deployment of the current Worker for branch and pull request work. Use wrangler deploy
  for production." `--name`: "Name of the Preview. Defaults to the current git branch."
- `preview delete`: "Delete a Preview and all its deployments" (`--name`, `--skip-confirmation`, `--worker-name`).
  -> the deployment URLs go with the Preview (deletion stated; that the URLs then stop serving is inferred, not stated,
  not measured).
- `versions` subcommands listed: upload, deploy, list, view, secret put / delete / bulk. NO `versions delete` (not
  found): a version, its Version URL and its alias cannot be deleted on demand from Wrangler; they go under retention
  (S10) or all at once with `preview_urls = false` (S1).
- Sure: high.

## Answers Q1-Q8 (all pages read 2026-11-25; S-numbers are the blocks above)

- **Q1 (noindex on Workers), per address type** (revised 2026-11-25 on the coordinator's condition: "silent" where a
  page says nothing; no inference from another page):
  - Production Worker's own per-version URLs (Version URLs of versions made by `wrangler deploy`, S1: "New versions are
    created when you run: wrangler deploy ..."), the counterpart of Pages' production hash links: SILENT (S1, S10, S11
    say nothing about noindex or X-Robots-Tag for them). Not measured.
  - Aliased Version URLs (`--preview-alias`): SILENT (S1). Not measured.
  - Previews, `workers.dev` Preview URL: the host adds it: "Cloudflare adds this header automatically to workers.dev
    Preview URLs, but not to custom domain Preview URLs." (S5, /workers/previews/custom-domains/; also S2). High.
  - Previews, custom-domain Preview URL: the host does not (same sentence, S5); "For static assets, add the header
    through a `_headers` file." High.
  - Previews, deployment URL (`<deployment-id>-...`, workers.dev or custom domain): SILENT (S2 and S5 name "Preview
    URLs" only).
  - Production `workers.dev` URL (`<worker>.<subdomain>.workers.dev`): SILENT (S8, S11).
  - Production route or custom domain: SILENT (S8, S11, S13).
  - Control: for Previews, by host choice (workers.dev: on; custom domain: off unless `_headers` adds it). `_headers`
    can add it by host rule (S11). The headers page says defaults can be "overridden, removed, or added to" (S11), but
    does not say whether that reaches the host's Preview `noindex`: not found. `preview_urls = false` turns Version URLs
    off (S1), and S5 names the same field for workers.dev Preview URLs. Standing caveat: "Cloudflare reserves the right
    to attach new headers to static asset responses at any time" (S11).
- **Q2 (`_headers`):** yes, applies to static assets (S11). Host match yes: "Using absolute URLs is supported" with
  host placeholders, e.g. `https://:version.:subdomain.workers.dev/*` (S11). Worker code no: "not applied to responses
  generated by your Worker code" (S11). 100 rules, 2,000 characters a line (S11, S13). High.
- **Q3 (link per PR):** Cloudflare's PR route is now Previews: "Do not use Version URLs for branch or pull request
  testing. Use Previews instead." (S3). `npx wrangler preview --name pr-<n>`; URLs
  `<preview-name>-<worker-name>.<subdomain>.workers.dev` (latest) and `<deployment-id>-<worker-name>...` (one deploy);
  or `<preview-name>.<custom domain>` (S2, S5, S6). Own assets: "`npx wrangler preview` uploads assets from the branch
  you are previewing" (S7). Limits: 100 Previews per Worker Free, 500 Paid; 100 deployments per Preview; oldest
  auto-deleted; Wrangler 4.135.0+ (S2). Version URLs / `--preview-alias`: forms
  `<version-prefix>-` / `<alias>-<worker-name>.<subdomain>.workers.dev`; alias plus name at most 63 characters; 1000
  aliases kept; only on workers.dev; not generated for Durable Object Workers (incl. Containers, Sandbox) or Workers for
  Platforms user Workers; off by default if `workers_dev` is off (S1). A version captures its static assets (S10), so a
  Version URL serves its own version's assets: one step inferred. High on the quotes.
- **Q4 (deletion):** Previews: `wrangler preview delete --name <n>`: "Delete a Preview and all its deployments" (S22);
  a PR cleanup job on `closed` is Cloudflare's own example (S6). That its URLs then stop serving is inferred, not
  stated. Version URLs and aliases: no `versions delete` command (S22); they stop only under retention (30 days, 100
  most recent and deployed kept, S10) or all at once with `preview_urls = false` (S1). High.
- **Q5 (the wall):** yes. Access "Previews only" for one Worker (`preview_worker`) or all (`all_preview_workers`), or by
  hostname; covers workers.dev, custom-domain Preview URLs and Version URLs; production stays open (S9, S8). Needs Zero
  Trust on the account. Not found: whether production's current version's Version URL counts as a "preview" (on Pages
  the wall covered production's hash links, Appendix A). Worker-level Access fails WebSocket upgrades with 403 (S9; not
  our path as drafted). High; the production-Version-URL case not found.
- **Q6 (limits, price):** 20,000 files per version Free, 100,000 Paid; 25 MiB per file (S13). "Requests to static
  assets are free and unlimited" (S12, S14), except with Workers Caching on (S14, footnote 4). Workers Paid "minimum
  charge of $5 USD per month" (S14, price read 2026-11-25). Monthly cap on deploys or version uploads: the limits and
  pricing pages name none (S13, S14): not found, not "there is none". High.
- **Q7 (direction):** Pages' overview: "Workers supports most Pages use cases and offers a broader feature set. It is
  Cloudflare's primary platform for building applications. Start new projects with Workers." (S16,
  /pages/, dateModified 2026-08-25). Migration guide (S15) and "Previews are the recommended way to test changes before
  production." (S3). High (cite only).
- **Q8 (direct uploads and the 500 builds):** not found. Pages' limits page: "Each time you push new code to your Git
  repository, Pages will build and deploy your site." and "Builds per month | 500" (S17); the direct upload page, the
  direct-upload-with-CI page, the known issues page and the Functions pricing page are silent (S18, S19); the docs have
  no /pages/platform/pricing/ page (404); Cloudflare's plan and product pages show nothing in the fetched text (S21,
  medium: script-rendered tables may be missed). High that the docs read do not say.

## Draft paragraph for ADR 0006 part 1 (revised; 8 lines, each at most 118 characters, counted in characters)
Cites are paths under developers.cloudflare.com/workers/ (full URLs in S1-S22). The read date is stated once for all
cites, a compression of "source and date after every claim"; the per-clause source map follows. "Appendix A" is the
preview hosting check report's (docs/reports/2026-11-24-preview-hosting-check.md); the main session may name it fully.

*Workers static assets, weighed again (ONE-OFF-HEADER (a)).* None of the check's marks was measured on Workers. By
its docs (developers.cloudflare.com/workers/…, read 2026-11-25) the host adds `noindex` on `workers.dev` Preview URLs,
"not to custom domain Preview URLs" (previews/custom-domains); they are silent on production's own Version URLs and
address, so they neither show nor rule out Pages' one-off finding. A pull request gets a Preview of its branch's
assets, 100 per Worker on Free, and a delete PREV-DELETE no longer needs (previews, previews/configuration); a wall on
previews only (configuration/cloudflare-access); `_headers` with host rules (static-assets/headers); 20,000 files,
25 MiB a file, free static requests, no monthly deploy cap named (platform/limits, platform/pricing). Under (a) the
row judges production's own address, a pass on Pages (Appendix A): Pages stays; a move re-runs the check on Workers.

Sources behind each clause: host noindex on workers.dev Preview URLs, not custom-domain ones S5 (and S2); silent on
production's own Version URLs and address S1, S8, S10, S11, S13; Preview with its branch's assets S7, S2; 100 per Worker
on Free S2; delete S2, S22 (PREV-DELETE passed on Pages: Appendix A, so not a needed gain); wall on previews only S9;
`_headers` host rules S11; 20,000 files, 25 MiB S13; free static requests S14 (and S12); no monthly deploy cap named
S13, S14 (silence). Left out for space: "Start new projects with Workers" (S16); the PR route change (S3); Browser
Integrity Check on workers.dev (S8); version skew (S10).

## Decision for Mustafa (optional; not needed for this PR)
My reading does NOT favour a move to Workers: nothing there is measured, Pages passes the revised row with measured
numbers, and the Workers gains (a delete command, no named monthly cap) answer no failed mark today.
WORKERS-NEXT. Question: should Workers stay only the Rollback route, or should the preview hosting check be run on
Workers now? Why it matters: Cloudflare's Pages overview says "Start new projects with Workers" (S16), and Workers' pull
request route changed since the research (S3). Options: (a) keep Pages; add Cloudflare's direction statement to
"Revisit when"; refresh the Workers route in the research and the Rollback at the next revision; (b) order spike 1 on
Workers Previews now (a throwaway Worker, his account steps, a re-run of PM-0006-2, -6 and -8). Recommendation: (a).

## For the main session
- Stale Workers route (what the lines say): r5 :852 and :1125 (R5-PREV-F14 and the hosts table: `versions upload
  --preview-alias`, "the 1000 most recent kept"); r6 :972 and r5 :1941 (PREV-DELETE: "only the 1000 most recently
  deployed aliases are retained" and "an explicit deletion was not read"). Today Cloudflare says "Do not use Version
  URLs for branch or pull request testing. Use Previews instead." (S3), and `wrangler preview delete` exists (S22).
  Not stale: r6 :93-98 (R6-HOST-F6 already quotes the `previews` block route); ADR :609 (names Workers only as "the
  alternative", no route).
- ADR 0006 Rollback :1556-1558 understates a move ("only ADR 0002's deploy target changes"): the deploy command changes
  (`wrangler preview`, Wrangler 4.135.0+, S2), and a `https://:version.:project.pages.dev/*` rule would not match
  Workers' hosts. A follow-up, not this row.
- A Workers re-run of the header script would meet Browser Integrity Check on workers.dev (error 1010 for default
  library User-Agents; it cannot be turned off there, S8): send a descriptive User-Agent.
- Not found: whether Access "Previews only" walls the Version URL of production's current version (S9).
- Q8: Cloudflare's docs do not say whether a direct upload counts against the 500 builds (S17-S19; S21 medium).
- Aside: Pages' limits page now states deployment retention (30 days; the 100 most recent and current production kept;
  a deleted deployment's hash URL "stops working", S17). Not checked against R5-PREV-F13 here.
