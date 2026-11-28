"""U7 brief rev 1: check each word-for-word quote against its source and against the brief (read only).

Whitespace runs (line wraps, indents) are collapsed to one space on both sides before comparing; a patch file's
leading '+' on each line is dropped first. Prints one line per quote and a total; exits 1 on any miss or long line.
"""
import re
import sys
from pathlib import Path

ROOT = Path("/home/user/Shamble-Frontlines")
CP = ROOT / ".checkpoints/netcode-proof-test"
BRIEF = CP / "u7-brief.md"


def norm(text):
    return re.sub(r"\s+", " ", text).strip()


def load(path):
    text = path.read_text(encoding="utf-8")
    if path.suffix == ".patch":
        text = "\n".join(l[1:] if l.startswith("+") else l for l in text.splitlines())
    return norm(text)


QUOTES = [
    (CP / "history.md", "U7, cut to a plain browser client: it connects over WebTransport and WebSocket, sends presses, "
     "receives snapshots and shows its counters. No sim in Wasm, no prediction, no drawing, no hash check: the real "
     "game runs the sim as Wasm since change-set 2 and checks its determinism in change-set 3."),
    (ROOT / "spikes/netcode-proof/crates/client-wasm/Cargo.toml", "wasm-bindgen joins at U7"),
    (CP / "wip/u6a-build.patch", "U7's page and U6b's clients take the same"),
    (ROOT / "spikes/netcode-proof/marks/marks.json", "session ready and one echo"),
    (ROOT / "spikes/netcode-proof/marks/marks.json", "negotiated draft"),
    (CP / "plan.md", "WebTransport · Playing"),
    (CP / "plan.md", "WebSocket · Playing"),
    (ROOT / "spikes/netcode-proof/marks/SERIES.md", "starting to connect"),
    (ROOT / "spikes/netcode-proof/marks/SERIES.md", "datagrams + one bidirectional stream"),
    (CP / "replan.md", "a scripted page on the test laptop, after U7"),
    (CP / "replan.md", "The hash checks (C-U7-1, -2) go with the sim."),
    (CP / "replan-marks.md", "R7's; U7 draws nothing"),
    (CP / "u4s-carries.md", "U7's web harness"),
    (CP / "u6b-brief.md", "U7's page turns `value_hex` into `serverCertificateHashes`' bytes."),
    (CP / "advisor-u6c-brief.md", "U7's drafter takes C1 (e): the page fetches `/npt/cert-hash-<room>.json` with the "
     "`room` of its own query, and the host ignores queries."),
    (CP / "advisor-u6c-brief.md", "The page host matches the request target's path (up to the first `?`) exactly and "
     "ignores any query string, which it never reads or logs"),
    (CP / "advisor-u6c-brief.md", "**Q-U6C-3 (the hash URL): (a), per room only, no plain name.**"),
    (CP / "advisor-u6c-brief.md", "U7's lock will be the next"),
    (CP / "advisor-u6c-brief.md", "nothing else"),
    (CP / "plan.md", "the endpoint takes the previous step's send time, and ClockSync stamps each `Ping` with it "
     "(NC-U4F-4). The controller's cap gets a source the browser client can reach. My lean: a `JoinAccepted` field, "
     "since the bound is room data (U4F-Q8 (i) (b)); a wire change goes through the catalogue first."),
    (CP / "advisor-u6a-brief.md", "U7's brief puts it where both sides reach it (npt-client does not depend on "
     "npt-room) with a test that the client's cap equals the room's bound; if the bound turns out to depend on "
     "`cmd_timing`, the page's pinned config names the option and the header records it. U7's brief check confirms "
     "the form."),
    (CP / "plan.md", "ClockSync's hardening when it becomes the Worker's: the offset clamped, not mapped to i64::MAX "
     "(RR-U4F-2), with a unit case at server_ns = u64::MAX; the next ping time in closed form (RR-U4F-5 (3)); "
     "`period_ns.max(1)` or a clamped tick rate (RR-U4F-5 (4)); its public surface trimmed: `ClockSync::new`, "
     "`period_ns`, `unknown_pongs`, `next_tick_at` and `one_way_hi_ns` each made crate-private, removed or given a "
     "consumer, and `ping_every_ns` removed (EA-U4F-1)."),
    (CP / "advisor-u6a-brief.md", "Until U6d merges, U6a's, U6b's, U6c's, U7's and U8's briefs and R1's run plan "
     "each state in one line that their tests or runs hold no throwing or rejoining hostile client and no client "
     "restarted mid-run, or bring C-U6-19 first. In R1 each browser leg (each browser on each transport, the "
     "QUIC-blocked fallback, -36's three) joins its own slot once, or the room restarts between legs; the token file "
     "holds a slot for every leg. SA-F1-1 stays open until U6d merges."),
    (CP / "advisor-u8-brief.md", "No `.npmrc` anywhere under spikes/"),
    (CP / "advisor-u8-brief.md", "U7's brief takes the same rule for `web/`."),
    (CP / "security-r1-deps.md", "wasm-bindgen =0.2.130 (U7, npt-client-wasm only): CLEAR WITH CONDITIONS: default "
     "features (std) only, no serde-serialize; no web-sys or wasm-bindgen-futures without another read; js-sys "
     "=0.3.107 allowed (locked)."),
    (CP / "security-r1-deps.md", "typescript 6.0.3 exact (U7): CLEAR."),
    (CP / "security-r1-deps.md", "U7: the lock gains no package from wasm-bindgen; web/package-lock.json holds "
     "typescript 6.0.3 (+ @types/node, vite's 19 if kept) and nothing more."),
    (CP / "u6a-brief.md", "Write files with the Write and Edit tools only (no sed, no shell redirect, no python writes; "
     "never `cargo fmt` writing files: apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes "
     "file whole with Write."),
]


def main():
    brief_raw = BRIEF.read_text(encoding="utf-8")
    brief = norm(brief_raw)
    bad = 0
    cache = {}
    for i, (src, q) in enumerate(QUOTES, 1):
        if src not in cache:
            cache[src] = load(src)
        qn = norm(q)
        in_src = qn in cache[src]
        in_brief = qn in brief
        ok = in_src and in_brief
        bad += 0 if ok else 1
        print(f"{i:2d} {'OK ' if ok else 'MISS'} src={in_src} brief={in_brief} {src.name}: {qn[:70]}")
    long_lines = [(n, len(l)) for n, l in enumerate(brief_raw.splitlines(), 1) if len(l) > 120]
    for n, w in long_lines:
        print(f"long line {n}: {w} characters")
    lines = len(brief_raw.splitlines())
    print(f"quotes checked: {len(QUOTES)}; misses: {bad}; brief lines: {lines}; lines over 120: {len(long_lines)}")
    sys.exit(1 if bad or long_lines else 0)


main()
