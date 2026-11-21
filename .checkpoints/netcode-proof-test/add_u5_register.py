"""Add U5's register lines to u4s-carries.md before U5-EARLY's launch (u5-brief.md section 9, :401-403; the advisor's
U5-EARLY ruling, advisor-u5-early.md condition 10 and section 6): entries 102-109 at the end of U5's section (U5-Q1 to Q3
and Q5 to Q7 with their terms, T2-L2, S2), a new U6 section with entries 110-111 (-35's counted run, the swap-form
carry), and entry 48's cite moved from ADR 0002:264-265 to :269. Each quote is copied from its source, which the entry
names. Refuses to run twice. Run from the repo root: python3 -I .checkpoints/netcode-proof-test/add_u5_register.py"""
import textwrap

P = ".checkpoints/netcode-proof-test/u4s-carries.md"
FILED = "filed 2026-11-21 before U5-EARLY's launch, advisor-u5-early.md condition 10"
ENTRIES_U5 = [
    (102, "U5-Q1, the three other fuzz targets (the advisor's U5 brief check, advisor-u5-brief.md:159-165; " + FILED + "):",
     '"(1) Same settings and numbers as -34\'s parts (10,000,000 inputs, 0 panics, 0 over-limit by the bar his U5-ALLOC '
     'answer picks; the scanner\'s bar 0); a panic or an over-limit allocation stops U5\'s close like a failed test and '
     'comes to me. (2) Their logs name no mark (`marks: []`), so npt-judge never reads them into -34\'s checks; the runner '
     'prints their verdict beside -34\'s. (3) "After the close" gains, as his marks.json item: -34\'s notes name "plan '
     '§4.7\'s npt-proto messages", which include SessionStatus, while its parts do not."'),
    (103, "U5-Q2, -35's counted run (advisor-u5-brief.md:166-170; " + FILED + "):",
     '"(1) The tool counts every changed path outside crates/net, manifests and Cargo.lock included, with ADV-U5B-5\'s '
     'fixture and ADV-U5B-10\'s details. (2) The one counted run follows U6\'s merge, "once, after the swap" '
     '(marks.json:7321; ADR 0001:1627), on the run\'s checks unchanged; plan §12.1\'s U5 and U6 rows gain one line each. '
     '(3) U6\'s register section gains the swap-form carry (ADV-U5B-5). If he picks U5-SIZE (b), this reads as (c): tool '
     'and run both at U6\'s time."'),
    (104, "U5-Q3, the allocator's home (advisor-u5-brief.md:171-183; " + FILED + "):",
     '"A member crate npt-alloc, the fourth exception at deny (ADR 0002:269: a written invariant, a test, an `#[allow]` '
     'with its reason). (1) Its only unsafe is the `unsafe impl GlobalAlloc` over `System`, one `#[allow(unsafe_code, '
     'reason = "...")]` on it, each inner unsafe block one operation with a SAFETY comment under the two clippy denies '
     'npt-tcpinfo\'s lib.rs holds (:37-38); the invariant: it returns exactly System\'s pointers and forwards every '
     'dealloc and realloc with the same layout, and counting never allocates (const-initialised thread-locals without '
     'Drop). (2) No build script, no `include` (DENY_EXCEPTION_WHY\'s rules), std only. (3) check.mjs: `crates/alloc/` in '
     'UNSAFE_EXCEPTIONS and EXCEPTION_LEVELS (deny), a placed-allow list like TCPINFO_UNSAFE_FNS, the table now (allow, '
     'deny, deny, deny), Cargo.toml\'s comment updated with the :269 cite; check.test.mjs plants (an unsafe block outside '
     'the impl, a second allow, an allow with an empty reason) each failing by name. (4) At U5 only dev dependencies and '
     'the fuzz crate depend on it; a normal dependency (U6\'s PM-0006-20) is U6\'s change with its own read. (5) Each '
     '`#[global_allocator]` static (the sim and room test files, the fuzz targets) compiles under its crate\'s lints; if '
     'one does not, it comes to me."'),
    (105, "U5-Q5, the sanitizer (advisor-u5-brief.md:187-191; " + FILED + "):",
     '"(a) if it fits, else (b), with two terms. (1) The rate is measured at Block 3 on Input and on FullState through '
     'the client (the slowest), with and without ASan; the choice and both rates go in the fill form before the first '
     'counted run. (2) A target\'s bite run uses its counted runs\' sanitizer setting; the header records it."'),
    (106, "U5-Q6, the bite's choice (advisor-u5-brief.md:192-196; " + FILED + "):",
     '"(1) `NPT_BITE` is read once at the target\'s start, only in the bites build (the fuzz crate\'s `bites` feature '
     'turning on npt-proto\'s); a measured build that sees it set refuses to start. (2) The runner sets it, or clears it, '
     'explicitly for every run. (3) The header records the bite id (`bite`), `build.features` with `bites`, and the '
     'binary\'s SHA-256 (plan §4.7 :631-632)."'),
    (107, "U5-Q7, the fuzz crate under the checks (advisor-u5-brief.md:197-203; " + FILED + "):",
     '"(1) check.mjs reads fuzz/\'s .rs files and Cargo.toml as it reads crates/ (the unsafe scan, #[path] and include!, '
     'no build script, no cargo-features or rustflags key), and checks that fuzz/\'s `[lints]` equal the workspace\'s key '
     'for key (unsafe_code forbid, the three clippy denies). (2) fmt and clippy of the fuzz crate on stable 1.99.1, '
     '`--locked`, in the shared target (a cargo check of `fuzz_target!` needs no nightly), or the brief says why not. (3) '
     'The read of libfuzzer-sys\'s expansion at Block 0 is the check: rustc does not report most lints inside another '
     'crate\'s macro, so a forbid there can stay silent; any unsafe or `no_mangle` it holds is named in the read, and a '
     'fifth exception comes to me before Block 3 goes on."'),
    (108, "T2-L2 (the U4d round-2 test review, u4d-round2-test.md:173-176; u4d-report.md:63; " + FILED + "):",
     '"Fix: one line in U5\'s carry 48 (u4s-carries.md): decoding a 60 B header that claims 2^20 ticks allocates at most '
     'room x size_of::<TickInput>()." U5 meets it in Block 2 (`Recording::decode`, replay.rs:323-326; u5-brief.md:133-135).'),
    (109, "S2's differential (U3c's review, u3c-review-result.txt:396; " + FILED + "):",
     '"Make U5\'s parser fuzz target (-34) a differential test: DelayStream\'s head length must equal tungstenite\'s for '
     'every input." U5 meets it in Block 4 (u5-brief.md:175-177).'),
]
U6_HEAD = "## U6 (the server, the transports and the host: plan.md §12.1's U6 row)"
ENTRIES_U6 = [
    (110, "-35's one counted run (U5-Q2 (a) term 2, advisor-u5-brief.md:168-169; " + FILED + "):",
     '"The one counted run follows U6\'s merge, "once, after the swap" (marks.json:7321; ADR 0001:1627), on the run\'s '
     'checks unchanged", on the tool U5 builds (U5-Q2 term 1, entry 103).'),
    (111, "The swap-form carry (ADV-U5B-5, advisor-u5-brief.md:108-110; U5-Q2 (a) term 3; " + FILED + "):",
     '"U6 makes the swap a change inside crates/net (npt-net\'s own selection), and a swap that needs a line outside it '
     'comes to the advisor before -35\'s run (to him if it changes what counts)."'),
]


def block(entries):
    out = []
    for num, title, text in entries:
        para = f"{num}. **{title}** {text}"
        out.extend(textwrap.wrap(para, width=120, subsequent_indent="     ", break_on_hyphens=False,
                                 break_long_words=False))
    return out


lines = open(P, encoding="utf-8").read().split("\n")
assert not any(l.startswith("102. **") for l in lines), "already added"
old_cite = [i for i, l in enumerate(lines) if "lives under ADR 0002:264-265.)" in l]
assert len(old_cite) == 1, old_cite
lines[old_cite[0]] = lines[old_cite[0]].replace("ADR 0002:264-265.)", "ADR 0002:269.)")
adv = [i for i, l in enumerate(lines) if l == "## For the advisor"]
assert len(adv) == 1, adv
end_u5 = adv[0]
while lines[end_u5 - 1] == "":
    end_u5 -= 1
new = block(ENTRIES_U5) + ["", U6_HEAD] + block(ENTRIES_U6) + [""]
lines = lines[:end_u5] + new + lines[adv[0]:]
open(P, "w", encoding="utf-8").write("\n".join(lines))
print(f"u4s-carries.md now {len(lines)} lines; entries 102-109 at :{end_u5 + 1}-; U6 section and 110-111 after them; "
      f"entry 48's cite at :{old_cite[0] + 1} now ADR 0002:269")
