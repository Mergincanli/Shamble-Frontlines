# U8 last round (round 3): security seat, SEC-R2-1 and SEC-R2-2 fixes only, 2026-11-23

Plan: (1) check the snapshot's sha256 (0e7fbab2…) and that W equals it; read advisor-u8-r2.md section 3 items 2-3 and
:126-139; u8-build.md "## Pass 3" items 3.2, 3.3. (2) Interdiff rs.yaml, lg.yaml, cloudinit.test.mjs (2.7), r1.sh
(bringup only), r1sh.test.mjs (2.19) against .gate/u8-r3-old. (3) Read the container record u8-p3-sudo-l-file.txt.
(4) Verdicts SEC-R2-1, SEC-R2-2; new findings on these lines only; what I did not verify. Last line DONE.

Status: part 1 started.
