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
- Sure: high. Consequence: R5-PREV-F14 / the ADR's "versions upload --preview-alias" route (r5 line 852, 1125) is now
  the route Cloudflare tells us not to use for pull requests; the current PR route on Workers is `wrangler preview`.

### S4. Workers: Previews, Resources and isolation (read 2026-11-25)
- URL: https://developers.cloudflare.com/workers/previews/resources/ (dateModified 2026-11-21).
- Binding table lists DO, Containers, KV, D1, R2, Queues, etc.; static assets are NOT named in the table or in the
  Limitations section (Service bindings, Workflows, Queue consumers, Cron Triggers, Routes). Not found: whether a
  Preview serves its own uploaded static assets (inferred yes, since a Preview is a deployment of the branch's Worker,
  but the page does not say).
- "Production routes target production. Previews do not take over zone routes, production custom domains ...
  To send HTTP traffic to a Preview, use its Preview URL on workers.dev or a custom domain Preview URL."
- Sure: high on what it says; the static-assets point is "not found".
