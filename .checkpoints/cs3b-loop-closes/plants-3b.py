#!/usr/bin/env python3
"""Planted defects for change-set 3b: each plant breaks one thing in a source file, runs the check that should catch
it, and puts the file back. A plant counts as caught only when the run fails AND its output holds the plant's `must`
pattern on a line that marks a failure (cargo's FAILED line of the named test or its panic text, vitest's FAIL line
of the named test, or Playwright's code frame line with its '>' mark), never because some other check failed.

Run from the repository's root, after a clean build:  python3 -I -B .checkpoints/cs3b-loop-closes/plants-3b.py [ids...]
It restores every file by content. A browser plant builds the planted tree into .gate/dist-plant; build the true
tree again afterwards. (Change-set 3a's own 96 plants: plants.py beside this file.)
"""
import pathlib
import re
import subprocess
import sys
import time

ROOT = pathlib.Path.cwd()
WB = str(ROOT / ".gate/wb-tool/x/wasm-bindgen")
OUT = ROOT / ".gate/cs3b"
VITEST = ["node", "node_modules/vitest/vitest.mjs", "run"]
CLIENT = VITEST + ["--root", "packages/client"]
WORKER = VITEST + ["--root", "packages/sim-worker", "--dir", "packages/sim-worker/src"]


def cargo(package, *target):
    return ["cargo", "test", "-p", package, "--locked", *(target or ["--lib"])]


PROTOCOL, SIM, ROOM, ENDPOINT, LOCAL = (cargo(name) for name in ("protocol", "sim", "room", "client-endpoint", "sim-wasm"))
DECODERS = cargo("protocol", "--test", "decoder_inputs")
STEP_ALLOC = cargo("sim-wasm", "--test", "step_alloc")
ROOM_ANY = cargo("room", "--test", "any_bytes")
ENDPOINT_ANY = cargo("client-endpoint", "--test", "any_bytes")
BUILD = ["node", "packages/client/scripts/build.mjs", "--out-dir", str(ROOT / ".gate/dist-plant")]
E2E = ["node", "node_modules/@playwright/test/cli.js", "test", "-c", "packages/e2e/playwright.config.ts", "--project=chromium",
       "--workers=1", "--retries=0", "--reporter=list", "--global-timeout=400000", "--trace", "off"]
PRESSES = [BUILD, E2E + ["-g", "10,000 presses"]]
ALLOCATION = [BUILD, E2E + ["-g", "sampling heap profile"]]
PLANTED_ARRAY = "(globalThis as { planted?: Uint8Array }).planted = new Uint8Array(64);"


def failed(test):
    """cargo prints 'test <path>::<name> ... FAILED' for a failed test only."""
    return re.escape(test) + r" \.\.\. FAILED"


def vitest_fail(title):
    """vitest prints ' FAIL  <file> > <title>' (and a line with a cross) for a failed test only."""
    return r"(FAIL|×).*" + re.escape(title)


def frame(source):
    """Playwright's code frame marks only the failing line with '>'."""
    return r"^\s*>\s*\d+ \|.*" + re.escape(source)


CODEC, WIRE, WORLD = "crates/protocol/src/codec.rs", "crates/protocol/src/wire.rs", "crates/sim/src/world.rs"
ROOM_RS, QUEUE, PRESS = "crates/room/src/lib.rs", "crates/room/src/queue.rs", "crates/client-endpoint/src/press.rs"
ENDPOINT_RS, LOCAL_RS, BLOCK = "crates/client-endpoint/src/endpoint.rs", "crates/sim-wasm/src/local_match.rs", "crates/client-endpoint/src/block.rs"
HOST, PACING, MESSAGES = "packages/sim-worker/src/host.ts", "packages/sim-worker/src/pacing.ts", "packages/sim-worker/src/messages.ts"
LINK, KEYS, BOOT = "packages/client/src/app/worker-link.ts", "packages/client/src/overlay/keys.ts", "packages/client/src/app/boot.ts"
SEND = "        if endpoint.build_input(outgoing) {\n            sent(link.end(Side::Client).send_unreliable(outgoing));\n        }\n"
NEWEST = "if (!this.#posted || (ahead !== 0 && ahead < 0x8000_0000)) {"
INPUT_CALL = "sim.input(this.#post, this.#held, this.#p0, this.#p1, this.#p2, this.#p3);"

# id, what is planted, [(file, old, new)], commands (each a list; a build first for the browser ones), must
PLANTS = [
    # --- the message code
    ("wrap-half", "a sequence exactly half the circle ahead counts as newer",
     [("crates/protocol/src/wrap.rs", "if steps != 0 && steps < 0x8000 {", "if steps != 0 && steps <= 0x8000 {")],
     [PROTOCOL], failed("a_sequence_is_newer_up_to_half_way_round_and_never_at_half_way")),
    ("wire-unchecked", "the real reader takes bytes without asking whether they are there: the mark's unchecked index, found by the decoder run",
     [(WIRE, "            .split_at_checked(count)\n            .ok_or(DecodeError::Truncated)?;", "            .split_at(count);")],
     [DECODERS], r"stopped at input \d+: the decoder panicked"),
    ("codec-trailing", "the snapshot decoder takes a message with bytes left after it",
     [(CODEC, "    reader.finish()?;\n    Ok(Snapshot {", "    Ok(Snapshot {")],
     [DECODERS], r"stopped at input \d+: a message that was taken does not encode back to its bytes"),
    ("codec-alloc-early", "the join decoder copies the token before it has read the rest of the message",
     [(CODEC, "            let token = reader.bytes(len)?;\n", "            let token = reader.bytes(len)?.to_vec();\n"),
      (CODEC, "            Ok(Control::Join(Join {\n                token: token.to_vec(),", "            Ok(Control::Join(Join {\n                token,")],
     [DECODERS], r"stopped at input \d+: a message holding 0 bytes took memory"),
    ("codec-cap", "the join decoder takes a token of any length",
     [(CODEC, "within(reader.u16()?, MAX_TOKEN_BYTES, DecodeError::TooLong)?", "within(reader.u16()?, usize::MAX, DecodeError::TooLong)?")],
     [PROTOCOL], failed("what_lies_outside_a_cap_or_a_range_is_refused_by_name")),
    ("loopback-stuck", "the loopback hands the same message out again and again",
     [("crates/protocol/src/loopback.rs", "        self.first = self.after(self.first, 1);\n", "")],
     [PROTOCOL], failed("each_direction_delivers_in_order_with_how_it_travelled")),
    # --- the simulation, the room, the endpoint
    ("sim-oldest", "the world keeps the oldest command of a tick, not the newest",
     [(WORLD, "commands.last()", "commands.first()")],
     [SIM], failed("the_newest_command_of_a_tick_becomes_the_record_and_none_holds_it")),
    ("sim-seed-first", "the seed is written before the player's counters: the shared state is no prefix",
     [(WORLD, "        out.extend_from_slice(presses);\n        out.extend_from_slice(seed);\n", "        out.extend_from_slice(seed);\n        out.extend_from_slice(presses);\n")],
     [SIM], failed("state_hash_matches_known_answers")),
    ("room-presses", "the room lets a press counter move two presses a tick",
     [(ROOM_RS, "u32::from(press_edges(*old, new)) <= covered;", "u32::from(press_edges(*old, new)) <= covered.saturating_mul(2);")],
     [ROOM], failed("each_check_drops_its_command_or_message_and_counts_it")),
    ("room-not-newer", "the room runs a command for the same tick as the command before it",
     [(ROOM_RS, ".filter(|covered| *covered > 0) else {", " else {")],
     [ROOM], failed("each_check_drops_its_command_or_message_and_counts_it")),
    ("room-ack", "the room takes an ack of a snapshot it never sent",
     [(ROOM_RS, ".all(|ack| ack.group == 0 && sent(ack.tick))", ".all(|ack| ack.group == 0 || sent(ack.tick))")],
     [ROOM], failed("each_check_drops_its_command_or_message_and_counts_it")),
    ("queue-twice", "the queue keeps a command that arrives twice, twice",
     [(QUEUE, ".is_some_and(|waits| waits.sequence == new.sequence)", ".is_some_and(|waits| waits.sequence == new.sequence && waits.tick == u32::MAX)")],
     [ROOM], failed("commands_come_out_in_sequence_whatever_order_they_came_in")),
    ("room-run-order", "the queue's window follows every command taken out, also a refused one: a refused command far ahead shuts out the honest ones, and a sequence can come round behind it",
     [(QUEUE, "        self.len = self.len.saturating_sub(1);\n        Some(first)\n", "        self.len = self.len.saturating_sub(1);\n        self.newest_run = Some(first.sequence);\n        Some(first)\n")],
     [ROOM], failed("a_command_taken_out_and_refused_does_not_move_the_window")),
    ("room-run-order-any", "the same, found by the random-message run of the room",
     [(QUEUE, "        self.len = self.len.saturating_sub(1);\n        Some(first)\n", "        self.len = self.len.saturating_sub(1);\n        self.newest_run = Some(first.sequence);\n        Some(first)\n")],
     [ROOM_ANY], failed("the_room_keeps_its_rules_whatever_arrives")),
    ("room-three", "three commands of one player run in one tick",
     [(ROOM_RS, "pub const CATCH_UP_CAP: usize = 2;", "pub const CATCH_UP_CAP: usize = 3;")],
     [ROOM], failed("at_most_two_commands_run_in_one_tick_and_the_rest_the_next")),
    ("pacer-two", "a button's counter moves by two per command",
     [(PRESS, "                button.counter = button.counter.wrapping_add(1);", "                button.counter = button.counter.wrapping_add(2);")],
     [ENDPOINT], failed("each_button_moves_by_at_most_one_per_command_and_the_rest_waits")),
    ("pacer-two-any", "the same, found by the random-message run of the endpoint",
     [(PRESS, "                button.counter = button.counter.wrapping_add(1);", "                button.counter = button.counter.wrapping_add(2);")],
     [ENDPOINT_ANY], r"call \d+: a counter moved by Some\(2\)"),
    ("endpoint-newest-only", "the endpoint never sends a command again",
     [(ENDPOINT_RS, "let unconfirmed = self.confirmed.is_none_or(|seen| seq_newer(earlier, seen));",
       "let unconfirmed = self.confirmed.is_none_or(|seen| seq_newer(earlier, seen)) && sequence == earlier;")],
     [ENDPOINT], failed("each_command_is_for_the_next_tick_and_unconfirmed_ones_are_sent_again")),
    ("endpoint-echo-again", "the echo of one command is counted again with every snapshot",
     [(ENDPOINT_RS, "        if processed == self.checked {\n            return;\n        }\n", "")],
     [ENDPOINT], failed("the_echo_is_checked_once_against_the_command_the_room_ran")),
    ("endpoint-late-refused", "a snapshot that comes late or twice is counted as refused",
     [(ENDPOINT_RS, "            self.readout.late = self.readout.late.wrapping_add(1);", "            self.readout.refused = self.readout.refused.wrapping_add(1);")],
     [ENDPOINT], failed("a_snapshot_that_is_not_newer_and_bytes_that_are_none_are_counted_and_dropped")),
    ("endpoint-index", "the endpoint looks a sent command up without keeping to its ring: a panic on a room's echo, found by the random-message run",
     [(ENDPOINT_RS, "        let place = self.sent.get(usize::from(sequence & RING_MASK))?;", "        let place = &self.sent[usize::from(sequence)];")],
     [ENDPOINT_ANY], failed("the_endpoint_keeps_its_rules_whatever_arrives")),
    ("queue-window", "before the first command runs, the queue takes any spread of sequences",
     [(QUEUE, "        let newest_run = *self.newest_run.get_or_insert(new.sequence.wrapping_sub(1));", "        let newest_run = self.newest_run.unwrap_or(new.sequence.wrapping_sub(1));")],
     [ROOM], failed("a_spread_of_sequences_before_the_first_command_runs_nothing_twice")),
    ("room-presses-any", "the room lets a press counter move two presses a tick, found by the random-message run of the room",
     [(ROOM_RS, "u32::from(press_edges(*old, new)) <= covered;", "u32::from(press_edges(*old, new)) <= covered.saturating_mul(2);")],
     [ROOM_ANY], r"call \d+: \d+ presses in \d+ ticks"),
    ("block-held-place", "the block writer puts the held buttons where the first counters go",
     [(BLOCK, "        put(HELD, u32::from(readout.echo_buttons));", "        put(PRESSES[0], u32::from(readout.echo_buttons));")],
     [ENDPOINT], failed("every_value_is_written_at_the_table_s_place")),
    # --- the local match's step
    ("local-late-command", "the command reaches the server after the server has run its tick",
     [(LOCAL_RS, SEND + "        {\n", "        {\n"),
      (LOCAL_RS, "        let mut client = link.end(Side::Client);\n", SEND + "        let mut client = link.end(Side::Client);\n")],
     [LOCAL], failed("the_join_crosses_in_the_first_step_and_a_post_reaches_the_server_within_its_step")),
    ("local-unsent-hidden", "what the link did not take is left out of the block's count of refused messages",
     [(LOCAL_RS, "        endpoint.write_block(room.counters().refused().wrapping_add(*unsent));", "        endpoint.write_block(room.counters().refused());")],
     [LOCAL], failed("a_link_that_takes_nothing_is_counted_in_the_block")),
    ("step-allocates", "a step of the local match takes a little memory",
     [(LOCAL_RS, "        let Self {\n            room,\n            endpoint,", "        let _planted = std::hint::black_box(vec![0u8; 16]);\n        let Self {\n            room,\n            endpoint,")],
     [STEP_ALLOC], r"a later step took memory"),
    # --- the Worker and the page
    ("host-last-post", "the Worker takes the last post to arrive, not the newest (the mark's second bite, in the host's own test)",
     [(HOST, NEWEST, "if (!this.#posted || ahead === ahead) {")],
     [WORKER + ["host.test.ts"]], vitest_fail("the newest post by its number is kept")),
    ("host-held-stays", "a hidden tab leaves the keys held",
     [(HOST, "          this.#held = 0;\n", "")],
     [WORKER + ["host.test.ts"]], vitest_fail("a hidden tab runs no tick and sets no timer")),
    ("pacer-nan", "a due time that is no number is taken from a step",
     [(PACING, "this.#nextDueMs = due(this.#sim.step(now));", "this.#nextDueMs = this.#sim.step(now);")],
     [WORKER + ["pacing.test.ts"]], vitest_fail("a due time that is not a finite number stops the pacer")),
    ("post-range", "an input post with held bits above the sixteenth is taken",
     [(MESSAGES, '        value["held"] <= 0xffff &&\n', "")],
     [WORKER + ["messages.test.ts"]], vitest_fail("anything else is refused")),
    ("tally-releases", "the tally counts no release for a tap",
     [("packages/client/src/app/press-tally.ts", "this.#releases += presses + Number(", "this.#releases += Number(")],
     [CLIENT + ["src/app/press-tally.test.ts"]], vitest_fail("presses are the counters' steps")),
    ("keys-repeat", "the repeat of a key that never went down here counts as a press",
     [("packages/client/src/input/keyboard.ts", "    if (!repeat) this.#settle(", "    this.#settle(")],
     [CLIENT + ["src/input/keyboard.test.ts"]], vitest_fail("a held key's repeats count nothing")),
    ("keys-typing-up", "a key that comes up while the focus is in a text box is not let go",
     [("packages/client/src/input/keyboard.ts", "    if (key < 0 || (typing && down)) return false;", "    if (key < 0 || typing) return false;")],
     [CLIENT + ["src/input/keyboard.test.ts"]], vitest_fail("in a text box a key that goes down is typing")),
    ("link-short-block", "the link reads a block that is shorter than its sections",
     [(LINK, "    if (buffer.byteLength < BLOCK_BYTES) return", "    if (buffer.byteLength < 0) return")],
     [CLIENT + ["src/app/worker-link.test.ts"]], vitest_fail("the echo and the link's counts are read from every block")),
    ("link-restart-posts", "a Worker that starts again is measured against the old one's last post",
     [(LINK, "        this.#postUsed = 0;\n", "")],
     [CLIENT + ["src/app/worker-link.test.ts"]], vitest_fail("a Worker that starts again has used no post yet")),
    ("link-key-time", "a key change is timed to a tick that still ran an older post",
     [(LINK, "(this.#postUsed - this.#keyPost) >>> 0 < 0x8000_0000) {", "this.#postUsed >= 0) {")],
     [CLIENT + ["src/app/worker-link.test.ts"]], vitest_fail("a key change is timed to the start of the first tick")),
    ("lights-forever", "a box stays lit after its press",
     [(KEYS, "if ((this.#until[button] ?? 0) > now) lit |= 1 << button;", "if ((this.#until[button] ?? 0) > 0) lit |= 1 << button;")],
     [CLIENT + ["src/overlay/keys.test.ts"]], vitest_fail("a box is lit while the echo holds its button")),
    ("diagram-production", "the key diagram is not on the production check's list",
     [("packages/client/scripts/build-checks.mjs", ', "key-diagram"];', "];")],
     [CLIENT + ["scripts/build-checks.test.mjs"]], vitest_fail("the glue stays in the Worker's chunk, and development code stays out of a production build")),
    # --- in the browser: a build of the planted tree, then one spec
    ("bite-held-only", "PM-0002-12's first bite: the Worker passes only the held buttons on",
     [(HOST, INPUT_CALL, "sim.input(this.#post, this.#held, 0, 0, 0, 0);")],
     PRESSES, frame("expect(end.presses).toEqual({ made: PRESSES, echoed: PRESSES, lost: 0 });")),
    ("bite-last-post", "PM-0002-12's second bite: the Worker takes the last post to arrive, not the newest",
     [(HOST, NEWEST, "if (!this.#posted || ahead === ahead) {")],
     PRESSES, frame("expect(end.olderPosts).toBe(0);")),
    ("bite-counted-taken", "the second bite, whole: the Worker counts a late post as ignored and takes it all the same (the test reviewer's plant)",
     [(HOST, "        } else {\n          this.#stalePosts += 1;\n        }\n", "        } else {\n          this.#stalePosts += 1;\n          this.#held = data.held;\n          this.#post = data.post;\n          this.#p0 = data.p0;\n          this.#p1 = data.p1;\n          this.#p2 = data.p2;\n          this.#p3 = data.p3;\n        }\n")],
     PRESSES, frame("expect(end.olderPosts).toBe(0);")),
    ("tally-held-again", "the page does not count a button held again with no press",
     [("packages/client/src/app/press-tally.ts", "if (presses === 0 && (held & bit) !== 0 && (this.#held & bit) === 0) this.#heldWithoutPress += 1;", "")],
     [CLIENT + ["src/app/press-tally.test.ts"]], vitest_fail("a button held again with no press is counted apart")),
    ("link-older-posts", "the link does not count a tick that ran an older post",
     [(LINK, "    if ((postUsed - this.#postUsed) >>> 0 >= 0x8000_0000) this.#olderPosts += 1;\n", "")],
     [CLIENT + ["src/app/worker-link.test.ts"]], vitest_fail("the echo and the link's counts are read from every block")),
    ("link-list-kept", "the link does not start its list of buffers anew after a post: they ride a second time",
     [(LINK, "    this.#doneCount = 0;\n", "")],
     [CLIENT + ["src/app/worker-link.test.ts"]], vitest_fail("it keeps the two newest blocks")),
    ("alloc-frame", "the key diagram allocates a small array every frame",
     [(KEYS, "    this.#lights.pressed(pressed, nowMs);\n", "    this.#lights.pressed(pressed, nowMs);\n    " + PLANTED_ARRAY + "\n")],
     ALLOCATION, frame("expect(perFrame).toBeLessThan(130 + 8 * blocksPerFrame);")),
    ("alloc-block", "the message listener allocates a small array for every block",
     [(BOOT, "      const problem = link.receive(event);\n", "      " + PLANTED_ARRAY + "\n      const problem = link.receive(event);\n")],
     ALLOCATION, frame("expect(perBlock).toBeLessThan(680);")),
]


def run(command):
    env = dict(__import__("os").environ)
    env["SHAMBLE_WASM_BINDGEN"] = WB
    env["SHAMBLE_DIST"] = str(ROOT / ".gate/dist-plant")
    env["SHAMBLE_CHROMIUM"] = "/opt/pw-browsers/chromium"
    done = subprocess.run(command, cwd=ROOT, env=env, capture_output=True, text=True, timeout=600)
    return done.returncode, done.stdout + done.stderr


def main():
    wanted = set(sys.argv[1:])
    OUT.mkdir(parents=True, exist_ok=True)
    rows = []
    for plant_id, what, edits, commands, must in PLANTS:
        if wanted and plant_id not in wanted:
            continue
        originals = {}
        for name, old, new in edits:
            text = originals.get(name) or (ROOT / name).read_text()
            originals.setdefault(name, text)
        try:
            current = dict(originals)
            for name, old, new in edits:
                assert current[name].count(old) == 1, f"{plant_id}: the text to replace is not there exactly once in {name}"
                current[name] = current[name].replace(old, new)
                (ROOT / name).write_text(current[name])
            output, code = "", 0
            for command in commands:
                code, text = run(command)
                output += f"$ {' '.join(command)}\n{text}\n"
                if code != 0:
                    break
        finally:
            for name, text in originals.items():
                (ROOT / name).write_text(text)
        (OUT / f"plant-{plant_id}.out").write_text(output)
        hit = [line for line in output.splitlines() if re.search(must, line)]
        caught = code != 0 and bool(hit)
        rows.append((plant_id, what, "CAUGHT" if caught else "NOT CAUGHT", (hit[0].strip() if hit else f"exit {code}, no line matches")[:150]))
        print(f"{rows[-1][2]:10} {plant_id:18} {rows[-1][3]}", flush=True)
    stamp = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())
    print(f"{stamp}: {sum(1 for row in rows if row[2] == 'CAUGHT')} of {len(rows)} caught")
    return 0 if all(row[2] == "CAUGHT" for row in rows) else 1


if __name__ == "__main__":
    sys.exit(main())
