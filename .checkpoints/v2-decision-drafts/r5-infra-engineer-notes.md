# r5 infra-engineer notes

- 17:59 started; output file created; next: read advisor-verdict-r5, plan-r5, inputs, e2 F3.
- 18:08 read verdict r5 (conditions 5, 6), inputs, e2 F3, 0006 parts 4-7, 12, 13 (B9), Harder, OQ-15/-20, migration 0; scratch tree base/ + tree/ (0006 + GLOSSARY), glossary baseline exact 0 / in context 101. Next: compose blocks in doc order: P7 (288-291), P12 pointer (497), B9 (520), DNS-down (744), Harder (753-757), OQ-15 (977-984), OQ-20 (1058-1060), OQ-21 (after 1075), MIG0 (1340-1347); NOTEs P4, P5, P6, PM-0006-18.
- 18:12 FIX OQ15-part7 written (0006:288-291). Next: P12 pointer.
- 18:14 FIX OQ21-part12 written (0006:497-498). Next: B9 (F3).
- 18:15 FIX F3-B9 written (0006:520). Next: DNS-down (744).
- 18:16 FIX OQ15-provider-down written (0006:744-745). Next: Harder (755-760).
- 18:18 FIX F3-OQ15-OQ20-harder written (0006:755-760). Next: OQ-15 entry (977-984).
- 18:21 FIX OQ15-entry written (0006:977-984). Next: OQ-20 entry (1058-1060).
- 18:23 FIX OQ20-entry written (0006:1058-1060). Next: OQ-21 new entry after 1075.
- 18:25 FIX OQ21-entry written (after 0006:1075; Carries wording tidied). Next: MIG0 (1340-1347).
- 18:27 FIX F3-OQ15-migration0 written (0006:1345-1347). Next: NOTEs P4, P5, P6, PM-0006-18; then dry-run + checks.
- 18:36 NOTEs OQ21-part4, -part5, -part6, -part12-rest, PM-0006-18, OQ15-OQ20-rest written. Next: dry-run, tree apply, glossary, rules checks.
- 18:44 dry-run 9/9 ok on disk copy; tree applied; glossary exact 0 (in context +2: 'Cloudflare' as the registrar, not a client-host product); rules.py clean. Next: read the diff once, final checks, DONE.
- 18:48 bug found and fixed: OQ21-entry BEFORE was 1074-1075 (dropped OQ-20's 'What decides it'); now 1075-1076. Next: re-run dry-run, tree, glossary, rules.
