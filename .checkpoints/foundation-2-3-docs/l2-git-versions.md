# L2 evidence: the runner's git version on 2026-11-12 (foundation-2-3-docs)
Read 2026-11-13 ~19:35 and ~20:20 UTC with the GitHub API (get_job_logs, return_content), toolchain jobs, the line
`git version ...` (the checkout step's "Getting Git version info", or its post step in the log's last 20 lines).
| Commit | Event | Run | Job | Time (UTC) | git |
|---|---|---|---|---|---|
| c901187 | push | 43249204290 | 131290067542 | 15:34:14 (checkout) and 15:34:31 (post) | 2.56.0 |
| c901187 | pull_request | 43249208734 | 131290084003 | 15:34:57 (post) | **2.56.1** |
| fd14855 | push | 43259689336 | 131327320460 | 16:32:05 (checkout) and 16:32:23 (post) | 2.56.0 |
| fd14855 | pull_request | 43259708110 | 131327382574 | 16:32:31 (post) | 2.56.0 |
| 45f1bb5 (M5) | push | 43281730739 | 131404460318 | 18:37:29 (post) | 2.56.0 |
| 45f1bb5 (M5) | pull_request | 43281743102 | 131404506899 | 18:37:37 (post) | 2.56.0 |
| a5823cf | push | 43282089237 | 131405738690 | 18:39:44 (post) | 2.56.1 |
Finding: on 2026-11-12 the runners differed. 2.56.1 was already on c901187's pull_request run at 15:34 UTC, while
2.56.0 was still on fd14855's and 45f1bb5's runs up to 18:37 UTC. So neither the report's "2.56.1 since 2026-11-12" nor
the advisor agent's L2 wording ("2.56.1 appears from a5823cf (18:39 UTC) on") holds exactly; the fix is worded as read
(put to the advisor agent with the plan's §5 change, 2026-11-13). DONE
