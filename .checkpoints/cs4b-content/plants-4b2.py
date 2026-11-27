#!/usr/bin/env python3
"""Change-set 4b-2's planted defects: each new check is shown red on the defect it exists to catch, then green again.

From the worktree's root, with no reviewer or other run in it (the script edits source files, one plant at a time, and
puts each back whatever happens; it says at its end whether every file is byte for byte as at the start):
    python3 .checkpoints/cs4b-content/plants-4b2.py <commit id> [names, e.g. P1 P5]
It writes one line per check to .checkpoints/cs4b-content/plants-4b2.txt and the commands' output to
.gate/plants-4b2.log. The commit id is the caller's `git rev-parse HEAD`, printed first; the caller also shows
`git status --porcelain -- crates packages Cargo.toml Cargo.lock` empty before and after (this script runs no git). A
plant counts as caught only when the command fails AND its output holds the check's own words for that defect; "the
build failed" is never enough. Not part of the product: notes.
"""
import hashlib, os, subprocess, sys

ROOT = os.getcwd()
LOG = open(".gate/plants-4b2.log", "a")
OUT = []
ENV = dict(os.environ, SHAMBLE_WASM_BINDGEN=f"{ROOT}/.gate/wb-tool/wasm-bindgen")
BINDINGS, SIM, WORLD = "crates/sim-wasm/src/lib.rs", "crates/sim/src/lib.rs", "crates/sim/src/world.rs"
CHECKS, BUILD, MANIFEST = "packages/client/scripts/build-checks.mjs", "packages/client/scripts/build.mjs", "Cargo.toml"
WORLD_RS = "crates/sim/src/collision.rs"
WALK, HELPER, HANDOVER = "crates/devtools/src/content_files.rs", "crates/devtools/src/files.rs", "packages/client/scripts/testlines.mjs"
PKG = "packages/sim-worker/pkg/sim_wasm_bg.wasm"
STAMP = "packages/sim-worker/pkg/built-file-check.json"

def run(*command, env=None):
    done = subprocess.run(command, capture_output=True, text=True, env=env or ENV)
    text = done.stdout + done.stderr
    LOG.write(f"\n$ {' '.join(command)}\n{text}\n[exit {done.returncode}]\n"); LOG.flush()
    return done.returncode, text

WASM = ("node", "packages/client/scripts/build.mjs", "--wasm-only")
PAGE = ("node", "--input-type=module", "-e", f'import {{ buildPage }} from "{ROOT}/packages/client/scripts/build.mjs"; await buildPage({{ production: true, outDir: "{ROOT}/.gate/dist-plant" }});')
UNIT = ("node", "../../node_modules/vitest/vitest.mjs", "run", "scripts/build-checks.test.mjs")
HAND = ("node", "../../node_modules/vitest/vitest.mjs", "run", "scripts/testlines.test.mjs")
FILES = ("cargo", "test", "-p", "devtools", "--locked", "--test", "content_files")
READERS = ("cargo", "test", "-p", "devtools", "--locked", "--test", "recordings", "the_committed_recordings")
COLLISION = ("cargo", "test", "-p", "sim", "--locked", "--lib", "collision")
PROOF = ("cargo", "test", "-p", "devtools", "--locked", "--test", "static_world")
FUZZ = ("cargo", "test", "-p", "devtools", "--locked", "--test", "content_fuzz", "fifty_thousand")
PROOFS_RS = "crates/devtools/src/world_proofs.rs"
REPLAY_RS, SCRIPTS_RS, RECORDING_TS = "crates/devtools/src/replay.rs", "crates/devtools/src/scripts.rs", "packages/sim-worker/src/recording.ts"
REPLAY_UNIT = ("cargo", "test", "-p", "devtools", "--locked", "--lib", "replay")
RECS = ("cargo", "test", "-p", "devtools", "--locked", "--test", "recordings")
SW_UNIT = ("node", "../../node_modules/vitest/vitest.mjs", "run", "--dir", "src", "recording")
SCAN = ("cargo", "test", "-p", "sim", "--locked", "--test", "rules_scan", "the_simulation_crate")
def native(crate): return ("cargo", "test", "-p", crate, "--locked", "--test", "native_target", "--target-dir", ".gate/target-plant")
# Step 9, cut C (Builder V6): the constructor takes the content, the Worker and the page carry it.
LIB_RS, MESSAGES_TS, PANIC_TS = "crates/sim-wasm/src/lib.rs", "packages/sim-worker/src/messages.ts", "packages/sim-worker/src/panic-message.ts"
CONTENT_TS, VITE_CONFIG = "packages/client/src/app/content.ts", "packages/client/vite.config.mjs"
SW_MESSAGES = ("node", "../../node_modules/vitest/vitest.mjs", "run", "--dir", "src", "messages")
SW_PANIC = ("node", "../../node_modules/vitest/vitest.mjs", "run", "--dir", "src", "panic-message")
PAGE_CONTENT = ("node", "../../node_modules/vitest/vitest.mjs", "run", "src/app/content.test.ts")
def boundary(name): return ("node", "node_modules/vitest/vitest.mjs", "run", "--root", f"{ROOT}/packages/sim-worker", "--dir", f"{ROOT}/packages/sim-worker/boundary", name)
DEV_PLANT = f"{ROOT}/.gate/dist-plant-dev"
DEV_PAGE = ("node", "--input-type=module", "-e", f'import {{ buildPage }} from "{ROOT}/packages/client/scripts/build.mjs"; await buildPage({{ production: false, outDir: "{DEV_PLANT}" }});')
def chromium(spec): return ("node", "node_modules/@playwright/test/cli.js", "test", "-c", "packages/e2e/playwright.config.ts", "--project=chromium", "--workers=1", "--retries=0", "--trace", "off", "--reporter=list", spec)
BROWSER_ENV = lambda dist: dict(ENV, SHAMBLE_DIST=dist, SHAMBLE_CHROMIUM="/opt/pw-browsers/chromium")
def chain(*parts):
    """Commands run in order, each with its environment, until one fails: the plant's output is all of theirs."""
    return ("CHAIN",) + parts
FOLD = [(LIB_RS, "            start_tick: 0,\n", "            start_tick: u32::from(bundle.first().copied().unwrap_or(0)),\n")]

USE = "use wasm_bindgen::prelude::*;\n"
MODS = "pub mod rng;\n"
def export(body, more=""): return [(BINDINGS, USE, USE + more + "\n#[wasm_bindgen]\npub fn planted(a: f32, b: f32, c: f32) -> f32 { " + body + " }\n")]
VECTOR = "parry3d::math::Vector::new"
def through_sim(body): return [(SIM, MODS, "pub fn planted(a: f32, b: f32, c: f32) -> f32 { " + body + " }\n" + MODS)] + export("sim::planted(a, b, c)")

# Step 8's second part (Builder V6, 2026-11-27): plants on the repository's own content. Every planted number is
# read from its file and changed by script, never typed (his rule: no value of his is typed by hand).
SOLDIER, TABLE_FILE, MAP_FILE = "content/movement/soldier.json", "content/source/testlines-table.json", "content/maps/test-scene.json"
def content_line(path, key):
    """The one line of a content file that holds `key`, and its number's text, both as they stand."""
    found = [line for line in open(path).read().split("\n") if line.startswith(f'  "{key}": ')]
    if len(found) != 1: raise SystemExit(f"{path}: {len(found)} lines hold {key}")
    return found[0], found[0].split(": ", 1)[1].rstrip(",")
def ulp_up(path, key):
    """`key`'s line, the same line with its number one step of a double up, and the two numbers' texts."""
    import math
    line, text = content_line(path, key)
    up = repr(math.nextafter(float(text), math.inf))
    return line, line.replace(f": {text}", f": {up}", 1), text, up
def numbers_swapped(path, first, second):
    """Two adjacent lines of a content file, the same two with their numbers exchanged, and the two numbers' texts."""
    (a, x), (b, y) = content_line(path, first), content_line(path, second)
    if f"{a}\n{b}\n" not in open(path).read(): raise SystemExit(f"{path}: {first} and {second} are not adjacent lines")
    return f"{a}\n{b}\n", f"{a.replace(f': {x}', f': {y}', 1)}\n{b.replace(f': {y}', f': {x}', 1)}\n", x, y
def fields_swapped(path, first, second):
    """The table's text from `first`'s field line through `second`'s, and the same with the two field names exchanged."""
    text = open(path).read()
    start, end = text.index(f'"field": "{first}"'), text.index(f'"field": "{second}"') + len(f'"field": "{second}"')
    old = text[start:end]
    return old, old.replace(f'"{first}"', "\0").replace(f'"{second}"', f'"{first}"').replace("\0", f'"{second}"')
def millimetre_up(path, shape_id):
    """`shape_id`'s top (its max's second number) and the same one millimetre higher, in decimal text."""
    from decimal import Decimal
    text = open(path).read()
    head = f'"id": "{shape_id}",\n      "max": [\n'
    lines = text[text.index(head) + len(head):].split("\n")
    top = lines[1].strip().rstrip(",")
    return head + lines[0] + "\n" + lines[1] + "\n", head + lines[0] + "\n" + lines[1].replace(top, str(Decimal(top) + Decimal("0.001")), 1) + "\n"
WALK_LINE, WALK_UP, WALK_TEXT, WALK_NEW = ulp_up(SOLDIER, "walk_speed_m_per_s")
STAMINA_OLD, STAMINA_NEW, SPRINT_TEXT, REFILL_TEXT = numbers_swapped(SOLDIER, "sprint_time_from_full_s", "stamina_refill_time_s")
ROWS_OLD, ROWS_NEW = fields_swapped(TABLE_FILE, "sprint_time_from_full_s", "stamina_refill_time_s")
POST_OLD, POST_NEW = millimetre_up(MAP_FILE, "yard_lane_post_50")

# name, kind, what the defect is, the edits (file, text there once, its replacement), the command, the words its output must hold
PLANTS = [
    ("P1", "defect", "a fused multiply-add exported from our own bindings", export("a.mul_add(b, c)"), WASM,
     ["the built-file check refuses", "the release twin: fmaf: a fused multiply-add (fmaf)", "the dev-profile twin: fmaf: a fused multiply-add (fmaf)"]),
    ("P2", "defect", "the same through the vector library the collision library brings, reached from the simulation",
     through_sim(f"{VECTOR}(a, a, a).mul_add({VECTOR}(b, b, b), {VECTOR}(c, c, c)).x"), WASM,
     ["the built-file check refuses", "a fused multiply-add (", "4libm"]),
    ("P3", "defect", "a relaxed-SIMD fused multiply-add instruction",
     export("relaxed(a, b, c)", '\n#[target_feature(enable = "relaxed-simd")]\nfn relaxed(a: f32, b: f32, c: f32) -> f32 {\n    use core::arch::wasm32::{f32x4_extract_lane, f32x4_relaxed_madd, f32x4_splat};\n    f32x4_extract_lane::<0>(f32x4_relaxed_madd(f32x4_splat(a), f32x4_splat(b), f32x4_splat(c)))\n}\n'), WASM,
     ["the built-file check refuses", "holds vector instruction 0xFD", "the target feature +relaxed-simd is not one the players' file is built with"]),
    ("P4a", "defect", "a function of the host named fma, imported",
     export("fma(f64::from(a), f64::from(b), f64::from(c)) as f32", '\n#[wasm_bindgen]\nextern "C" {\n    fn fma(a: f64, b: f64, c: f64) -> f64;\n}\n'), WASM,
     ["the built-file check refuses", "a fused multiply-add (fma)"]),
    ("P4b", "defect", "JavaScript's sine, imported",
     export("sin(f64::from(a + b + c)) as f32", '\n#[wasm_bindgen]\nextern "C" {\n    #[wasm_bindgen(js_namespace = Math)]\n    fn sin(x: f64) -> f64;\n}\n'), WASM,
     ["the built-file check refuses", "the players' WebAssembly imports the function ./sim_wasm_bg.js __wbg_sin_", "which is not one of the bindings' own 3"]),
    ("P4c", "defect", "the host's clock, imported",
     export("now() as f32 + a + b + c", '\n#[wasm_bindgen]\nextern "C" {\n    #[wasm_bindgen(js_namespace = Date)]\n    fn now() -> f64;\n}\n'), WASM,
     ["the built-file check refuses", "the players' WebAssembly imports the function ./sim_wasm_bg.js __wbg_now_"]),
    ("P5", "defect", "a sine of the standard library in the bindings crate, where the source scan does not read", export("(a + b + c).sin()"), WASM,
     ["the built-file check refuses", "sinf: a maths function whose result is not the same on every target"]),
    # The test list named an arc cosine through `angle_between`; the vector library's own polynomial serves that call
    # (tried: it brings libm's square root and absolute value only), so the plant is its exponential.
    ("P6", "defect", "an exponential of the vector library, which arrives as the libm crate's and under no C name",
     through_sim(f"{VECTOR}(a, b, c).exp().x"), WASM,
     ["the built-file check refuses", "is a maths routine outside the allow-list", "libm::math::exp"]),
    ("P6b", "defect", "the vector library's angle between two vectors: the libm crate's square root",
     through_sim(f"{VECTOR}(a, b, c).angle_between({VECTOR}(c, b, a))"), WASM,
     ["the built-file check refuses", "libm::math::sqrt::sqrtf is a maths routine outside the allow-list"]),
    ("P7", "control", "plain arithmetic, a * b + c, exported the same way: never refused", export("a * b + c"), WASM, []),
    ("P8", "defect", "a rounding of the standard library (what step 9's f64::round would bring, said here for it)", export("(a + b + c).round()"), WASM,
     ["the built-file check refuses", "is a maths routine outside the allow-list", "round"]),
    ("P9", "defect", "the names stripped from the release twin", [(MANIFEST, "[profile.release]\noverflow-checks = true\n", "[profile.release]\noverflow-checks = true\nstrip = true\n")], WASM,
     ["the release twin: 0 name sections, not one: the name rules have nothing to read"]),
    ("P10", "defect", "the path remap dropped: the registry's place on this machine reaches the file",
     [(BUILD, 'CARGO_ENCODED_RUSTFLAGS: registryRemapFlag(process.env["CARGO_HOME"], os.homedir(), root) });', 'CARGO_ENCODED_RUSTFLAGS: "" });')], WASM,
     ["the built-file check refuses", "the players' WebAssembly holds a path of the machine that built it (/root/"]),
    ("P11", "defect", "the library named outside its one file", [(WORLD, "use crate::state_hash::Fnv1a64;\n", "use crate::state_hash::Fnv1a64;\nuse parry3d::query::Ray;\n")], SCAN,
     ["world.rs:", "`parry3d`: the collision library is named outside `collision.rs`"]),
    ("P12", "defect", "the fused rule matching a name's last step only (the unit tests' generic name must turn red)",
     [(CHECKS, "    const found = FUSED_STEPS.filter((step) => steps.has(step));", "    const found = FUSED_STEPS.filter((step) => name === step || name.endsWith(`${step.length}${step}`));")], UNIT,
     ["a fused multiply-add is refused under every name it comes by"]),
    ("P13", "defect", "the walker stepping over a vector instruction instead of naming it",
     [(CHECKS, "          vector = true;\n          result.vector.push({ body: place, instruction: body.u32() });", "          body.u32();")], UNIT,
     ["a vector instruction is named with its body and its number"]),
    ("P14", "defect", "the imports rule turned into a list of banned words",
     [(CHECKS, "...imports.filter((entry) => !PLAYERS_IMPORTS.some((known) => same(known, entry)))", "...imports.filter((entry) => /fma|sin|now/.test(entry.name))")], UNIT,
     ["the players' file passes with the bindings' own imports"]),
    ("U1", "defect", "the rule on crates that may not be linked finds nothing", [(CHECKS, "    return UNLINKED_CRATES.filter((crate) => steps.has(crate)).map(", "    return UNLINKED_CRATES.filter(() => false).map(")], UNIT,
     ["a function of a crate the players' file is built without is refused"]),
    ("U2", "defect", "a twin's names in the older scheme are read as if the rules saw them whole", [(CHECKS, "    ...(older.length === 0 ? [] : [", "    ...(older.length >= 0 ? [] : [")], UNIT,
     ["a clean twin passes, and each rule refuses its own planted module by name"]),
    ("U3", "defect", "a second asker of the library is not seen when its line is indented or is a table of its own", [("crates/devtools/tests/dependency_rules.rs", ".any(|line| !line.trim_start().starts_with('#') && line.contains(\"parry3d\"))", ".any(|line| line.starts_with(\"parry3d\"))")], ("cargo", "test", "-p", "devtools", "--locked", "--test", "dependency_rules"),
     ["the_rule_reads_what_it_should_refuse"]),
    # Step 10, the static world.
    ("W1", "defect", "of two shapes at one distance the first visited wins", [(WORLD_RS, "t < hit.t || (t == hit.t && shape < hit.shape)", "t < hit.t")], COLLISION,
     ["a_ray_meets_the_first_shape_on_its_way_and_the_lower_index_of_two_at_one_distance"]),
    # No W2. Tried: with the zero-direction branch taken out, the division by zero gives the same answers on every
    # ray of the tests (IEEE division: infinities for an origin beside the faces, a NaN on a face, and every
    # comparison with a NaN is false, which lets the ray through as the inclusive check does). So that change is
    # no defect a test could find; the branch stays because it says the rule in words and leans on no NaN.
    ("W3", "defect", "a ray that only touches a face misses the box", [(WORLD_RS, "            if origin[axis] < min[axis] || origin[axis] > max[axis] {", "            if origin[axis] <= min[axis] || origin[axis] >= max[axis] {")], COLLISION,
     ["a_ray_meets_the_first_shape_on_its_way_and_the_lower_index_of_two_at_one_distance"]),
    ("W4", "defect", "a minus zero keeps its sign into the world", [(WORLD_RS, "        if value == 0.0 { 0.0 } else { value }", "        value")], COLLISION,
     ["a_minus_zero_in_a_document_leaves_no_sign_in_the_world"]),
    ("W5", "defect", "the shapes keep the document's order", [(WORLD_RS, "        order.sort_by(|a, b| {", "        order.sort_by(|a, b| { if true { return a.cmp(b); }")], COLLISION,
     ["a_shapes_index_is_its_rank_by_id_whatever_the_documents_order"]),
    ("W6", "defect", "a tree one deeper than the walks' stack is taken", [(WORLD_RS, "pub const MAX_TREE_DEPTH: u32 = 31;", "pub const MAX_TREE_DEPTH: u32 = 32;")], COLLISION,
     ["a_document_that_gives_no_world_is_refused_by_name"]),
    ("W7", "defect", "a ramp is high on the side it rises from", [(WORLD_RS, "        Toward::PlusX => (0, true),", "        Toward::PlusX => (0, false),")], COLLISION,
     ["a_ramp_is_met_on_its_slope_its_faces_and_its_floor_toward_each_of_the_four_sides"]),
    ("W8", "defect", "a ramp is met where its box is, slope or no slope", [(WORLD_RS, "        if rate < 0.0 {", "        if rate < 0.0 && false {")], COLLISION,
     ["a_ramp_is_met_on_its_slope_its_faces_and_its_floor_toward_each_of_the_four_sides"]),
    ("W9", "defect", "one ray's distance moved by one unit in its last place: the committed lines differ", [(WORLD_RS, "    Some((enter, leave))\n}", "    Some((if enter > 17.0 && enter < 17.01 { f32::from_bits(enter.to_bits() + 1) } else { enter }, leave))\n}")], PROOF,
     ["this build gives", "and the file has"]),
    # Step 9, the page folder's plain files and the scan of the sources for markup sinks.
    ("P15", "defect", "the page's entry shows a text as markup", [("packages/client/src/main.ts", "void boot();", "void boot();\ndocument.body.innerHTML = location.hash;")], PAGE,
     ["the page's sources are refused", "packages/client/src/main.ts: innerHTML (text is shown as text: MARKUP_SINKS in build-checks.mjs)"]),
    ("P16", "defect", "the Worker makes code from a text", [("packages/sim-worker/src/worker.ts", "let starting = false;", "let starting = new Function(\"return false\")() as boolean;")], PAGE,
     ["the page's sources are refused", "packages/sim-worker/src/worker.ts: new Function (text is shown as text: MARKUP_SINKS in build-checks.mjs)"]),
    ("P17", "defect", "the folder reader takes a link for a file", [(CHECKS, "      if (!entry.isFile()) throw", "      if (entry.isFIFO()) throw")], UNIT,
     ["a link to a file and a link to a folder are refused by name, wherever they lie"]),
    ("P18", "defect", "the scan of the sources no longer knows innerHTML", [(CHECKS, '  ["innerHTML", /innerHTML/i],\n', "")], UNIT,
     ["each word is refused by its own name, in code, in a text and in a comment alike"]),
    ("P19", "control", "the production page builds from the clean sources", [], PAGE, []),
    # Step 10, the judge of a taken map's world, the hostile maps and the fuzz run's worlds.
    ("W10", "defect", "of 512 boxes met at one distance the first visited answers a ray, not the lowest index (the hostile row names it; the same box 512 times does not)", [(WORLD_RS, "t < hit.t || (t == hit.t && shape < hit.shape)", "t < hit.t")], PROOF,
     ["every_hostile_map_is_one_the_judge_passes", "a search of every shape says shape 0"]),
    ("W11", "defect", "a node entered exactly as far as the best hit is pruned, so a lower index behind it is lost", [(WORLD_RS, "            if enter_box(min, max, origin, direction, reach).is_none() {", "            if enter_box(min, max, origin, direction, reach).is_none_or(|(enter, _)| best.is_some_and(|hit| enter >= hit.t)) {")], PROOF,
     ["a search of every shape says shape"]),
    ("W12", "defect", "the builder leaves out one of the map's shapes", [(WORLD_RS, "        for place in order {", "        for place in order.into_iter().skip(1) {")], PROOF,
     ["the world's shapes are not the map's own"]),
    ("W13", "defect", "the fuzz run sends no map on to the world's builder", [(PROOFS_RS, "            Ok(Document::Map(map)) => vec![map],", "            Ok(Document::Map(_)) => Vec::new(),"), (PROOFS_RS, "            .map(|set| set.maps().to_vec())", "            .map(|_| Vec::new())")], FUZZ,
     ["worlds built in 10,000 inputs, under 250"]),
    # Step 8, the tools.
    ("T1", "defect", "the content walk takes a link, a pipe or a folder for a file", [(WALK, "            if !kind.is_file() {", "            if kind.is_dir() && kind.is_file() {")], FILES,
     ["whatever_is_not_a_kinds_folder_with_plain_id_files_is_refused_before_it_is_read"]),
    ("T2", "defect", "the content walk reads a file of any name", [(WALK, 'if !name.strip_suffix(".json").is_some_and(content::json::is_id) {', 'if !name.strip_suffix(".json").is_some_and(|id| !id.is_empty()) {')], FILES,
     ["whatever_is_not_a_kinds_folder_with_plain_id_files_is_refused_before_it_is_read"]),
    ("T3", "defect", "the bundle tool writes without asking the loader", [(WALK, "        content::load_file(path, text.as_bytes()).map_err(|error| error.message())?;", "        let _ = content::load_file(path, text.as_bytes());"), (WALK, "    content::load(text.as_bytes()).map_err(|error| error.message())?;", "    let _ = content::load(text.as_bytes());")], FILES,
     ["a_file_that_is_not_in_canonical_form_is_refused_in_the_loaders_words"]),
    ("T4", "defect", "the bundle may be written inside the content folder when the path is spelled with ..", [(WALK, "    if real(out)?.starts_with(real(content)?) {", "    if out.starts_with(content) {")], FILES,
     ["the_bundle_is_written_outside_the_content_folder_and_never_through_a_link"]),
    ("T5", "defect", "the file helper makes a file through whatever stands in its place", [(HELPER, "        .create_new(true)\n", "        .create(true)\n")], READERS,
     ["a file made any other way than new"]),
    ("T6", "defect", "the hand-over compares numbers loosely (a minus zero passes for zero)", [(HANDOVER, "    else if (!Object.is(content[field], value)) lines.push(", "    else if (content[field] !== value) lines.push(")], HAND,
     ["the check finds a value one step of a double away"]),
    ("T7", "defect", "a table row may name a path as its field", [(HANDOVER, "!/^[a-z][a-z0-9_]{0,63}$/.test(row.field)", "!/^[^ ]+$/.test(row.field)")], HAND,
     ["a row is refused for anything but a field's name"]),
    ("T8", "defect", "the hand-over writes through a link in the target's place", [(HANDOVER, "  if (there !== undefined && !there.isFile()) throw", "  if (there !== undefined && there.isDirectory()) throw")], HAND,
     ["no file is read or written through a link"]),
    ("T9", "defect", "the Export in use is whatever the manifest beside it says (no pin in code)", [(HANDOVER, "  if (found !== pinned) throw", "  if (found !== pinned && false) throw")], HAND,
     ["the Export is the pinned one"]),
    ("T10", "defect", "an Export made at 60 steps a second is handed over", [(HANDOVER, "  if (rate !== 30) throw", "  if (rate === undefined) throw")], HAND,
     ["the Export is the pinned one"]),
    ("C1", "defect", "his movement table holds a value one step of a double from his Export's (the hand-over's check on the real files)", [(SOLDIER, WALK_LINE, WALK_UP)], HAND,
     [f"walk_speed_m_per_s is {WALK_NEW}, and his Export has {WALK_TEXT}"]),
    ("C2", "defect", "the same value, caught by the second route", [(SOLDIER, WALK_LINE, WALK_UP)], FILES,
     [f"values.movement.walk_speed_m_per_s is {WALK_TEXT} in his Export, and the loader's walk_speed_m_per_s is {WALK_NEW}"]),
    ("C3", "defect", "two rows of the table send their values to each other's field, and the movement table is written from that table (the second route)", [(TABLE_FILE, ROWS_OLD, ROWS_NEW), (SOLDIER, STAMINA_OLD, STAMINA_NEW)], FILES,
     [f"values.stamina.refill_time_from_empty_s is {REFILL_TEXT} in his Export, and the loader's stamina_refill_time_s is {SPRINT_TEXT}"]),
    ("C4", "control", "the same mis-mapped pair passes the hand-over's own check, which is what the second route is for", [(TABLE_FILE, ROWS_OLD, ROWS_NEW), (SOLDIER, STAMINA_OLD, STAMINA_NEW)], HAND, []),
    ("C5", "defect", "the test scene's last lane post one millimetre taller: its world is not the committed one", [(MAP_FILE, POST_OLD, POST_NEW)], PROOF,
     ["the test scene's world is not the committed one"]),
    ("C6", "control", "the clean content: the content folder's tests", [], FILES, []),
    ("C7", "control", "the clean content: the static world's tests", [], PROOF, []),
    ("C8", "control", "the clean content: the hand-over's tests", [], HAND, []),
    # Step 9, cut A (Builder V6): the page carries the content bundle.
    ("C9", "defect", "the build writes no content bundle into the page", [(BUILD, "  writeContentBundle(outDir);\n", "")], PAGE,
     ["the built page is refused", "content.json: the page holds no content bundle"]),
    ("C10", "defect", "the page's bundle cap is one byte above the loader's", [(CHECKS, "export const MAX_CONTENT_BYTES = 8 * 1024 * 1024;", "export const MAX_CONTENT_BYTES = 8 * 1024 * 1024 + 1;")], UNIT,
     ["the bundle's cap is the game's loader's own, read from its source"]),
    ("C11", "defect", "the check takes a content.json anywhere in the page's folder", [(CHECKS, "  const bundle = files.find(({ name }) => name === CONTENT_FILE);", "  const bundle = files.find(({ name }) => name.endsWith(CONTENT_FILE));")], UNIT,
     ["a page folder without content.json beside index.html, or with one larger than the loader takes, is refused"]),
    # Step 9, cut B (Builder V6): a recording names its frozen content, and a replay runs on that content alone.
    ("C12", "defect", "a replay takes any bundle: its sha256 is not held to the header", [(REPLAY_RS, "    if sha256(bundle) != header.content_hash {", "    if bundle.is_empty() && sha256(bundle) != header.content_hash {")], REPLAY_UNIT,
     ["content_other_than_the_header_names_is_refused_by_name_before_the_match_is_made"]),
    ("C13", "defect", "a header that names no content is replayed", [(REPLAY_RS, "            header.content_hash == [0; 32] || header.map_id.is_empty(),", "            header.content_hash == [0; 32] && header.map_id.is_empty(),")], REPLAY_UNIT,
     ["what_this_build_cannot_replay_is_refused_by_name_before_any_call"]),
    ("C14", "defect", "the scripts leave the content out of the header", [(SCRIPTS_RS, "            content_hash: content.sha256,", "            content_hash: [0; 32],")], RECS,
     ["content: its header names none"]),
    ("C15", "defect", "the TypeScript check takes any bundle", [(RECORDING_TS, "  if (digest.length !== recording.contentHash.length || digest.some((byte, at) => byte !== recording.contentHash[at])) {", "  if (digest.length !== recording.contentHash.length) {")], SW_UNIT,
     ["the content a replay is given is the one its recording's header names"]),
    ("C16", "control", "the clean recordings: the recordings' tests", [], RECS, []),
    # Step 9, cut C (Builder V6): loading content changes no state (natively, in Node, in Chromium), and the Worker's
    # and the page's new checks each catch their own defect. The fold is never run with make_recordings writing.
    ("C17", "defect", "a content byte folded into the shared state at construction: natively", FOLD, RECS,
     ["first-loop, Shared", "Difference { step: 1,"]),
    ("C18", "defect", "a content byte folded into the shared state at construction: in Node, on the players' file", FOLD,
     chain((ENV, WASM), (ENV, boundary("replay"))),
     ["first-loop replays on the players' file to its hashes file, step for step", "expected 1 to be null"]),
    ("C19", "defect", "a content byte folded into the shared state at construction: in Chromium, on the players' file", FOLD,
     chain((ENV, WASM), (ENV, DEV_PAGE), (BROWSER_ENV(DEV_PLANT), chromium("tests/replay/determinism.spec.ts"))),
     ["every recording replays in the browser's WebAssembly", "toBeNull", "Received: 1"]),
    ("C20", "defect", "the Worker's start takes a bundle one byte over the loader's cap", [(MESSAGES_TS, "  return value instanceof ArrayBuffer && value.byteLength >= 1 && value.byteLength <= MAX_BUNDLE_BYTES;", "  return value instanceof ArrayBuffer && value.byteLength >= 1 && value.byteLength <= MAX_BUNDLE_BYTES + 1;")], SW_MESSAGES,
     ["anything else is refused"]),
    ("C21", "defect", "the Worker's cap is one byte above the loader's own", [(MESSAGES_TS, "export const MAX_BUNDLE_BYTES = 8 * 1024 * 1024;", "export const MAX_BUNDLE_BYTES = 8 * 1024 * 1024 + 1;")], boundary("sim-wasm"),
     ["the Worker's cap on a bundle is the loader's own"]),
    ("C22", "defect", "the Worker tells a thrown text uncut", [(PANIC_TS, "  return cutText(thrown instanceof Error ? thrown.message : String(thrown));", "  return thrown instanceof Error ? thrown.message : String(thrown);")], SW_PANIC,
     ["a thrown text is cut to its bound with a mark"]),
    ("C23", "defect", "the page takes an error text of any length", [(MESSAGES_TS, '      return typeof value["message"] === "string" && value["message"].length <= ERROR_TEXT_UNITS;', '      return typeof value["message"] === "string";')], SW_MESSAGES,
     ["anything else from the Worker is refused"]),
    ("C24", "defect", "the page reads its copy before the Worker's answer", [(CONTENT_TS, '    if (!link.ready) return "";', '    if (!link.ready && link.failure !== "") return "";')], PAGE_CONTENT,
     ["a Worker that answers ready: the copy is read once"]),
    ("C25", "defect", "the page's sentence for a failed fetch carries the server's own status text", [(CONTENT_TS, "the server answered ${response.status}` };", "the server answered ${response.status} ${response.statusText}` };")], PAGE_CONTENT,
     ["what the page says of a failed fetch is fixed text"]),
    ("C26", "defect", "the development server has no /content.json of its own: its page-for-any-address stands in",
     [(VITE_CONFIG, "  plugins: [devContent],\n", "  plugins: [],\n")],
     chromium("tests/skeleton/isolation.spec.ts"),
     ["under Vite's dev server", "application/json"]),
    ("C27", "control", "the clean Worker and page: the messages' and the content's tests", [], SW_MESSAGES, []),
]

def note(name, kind, ok, what, detail):
    words = {"defect": ("CAUGHT", "NOT CAUGHT"), "control": ("HELD", "DID NOT HOLD")}[kind]
    line = f"{name} {words[0] if ok else words[1]} ({kind}): {what}: {detail}"
    OUT.append((kind, ok, line)); print(line, flush=True)

def digest(path): return hashlib.sha256(open(path, "rb").read()).hexdigest() if os.path.exists(path) else "absent"

def planted(name, kind, what, edits, command, words, cwd=None):
    originals = {path: open(path).read() for path, _, _ in edits}
    try:
        for path, old, new in edits:
            text = open(path).read()
            count = text.count(old)
            if count != 1: return note(name, kind, False, what, f"the text to replace stands {count} times in {path}")
            open(path, "w").write(text.replace(old, new, 1))
        os.chdir(cwd or ROOT)
        if command[0] == "CHAIN":
            code, text = 0, ""
            for env, part in command[1:]:
                code, more = run(*part, env=env)
                text += more
                if code != 0: break
        else:
            code, text = run(*command, env=BROWSER_ENV(f"{ROOT}/.gate/dist-development") if command[0:2] == ("node", "node_modules/@playwright/test/cli.js") else None)
        os.chdir(ROOT)
        found = [word for word in words if word in text]
        ok = (code == 0) if kind == "control" else (code != 0 and len(found) == len(words))
        note(name, kind, ok, what, f"exit {code}, found {len(found)} of {len(words)} wanted texts" + ("" if ok or kind == "control" else f"; missing {[w for w in words if w not in found]}"))
    finally:
        os.chdir(ROOT)
        for path, original in originals.items(): open(path, "w").write(original)

def stamp_plants(names):
    """After a clean walk: no stamp, a stale stamp, another module in the players' place. Each must stop the page."""
    want = lambda name: not names or name in names
    if not any(want(name) for name in ("S1", "S2", "S3", "S4")): return
    code, _ = run(*WASM)
    stamp, wasm = open(STAMP).read(), open(PKG, "rb").read()
    try:
        if want("S1"):
            code, text = run(*PAGE)
            note("S1", "control", code == 0, "a page built from the walked file, by the page's step alone", f"exit {code}")
        if want("S2"):
            os.remove(STAMP)
            code, text = run(*PAGE)
            note("S2", "defect", code != 0 and "the built-file check left no stamp" in text, "no stamp", f"exit {code}")
            open(STAMP, "w").write(stamp)
        if want("S3"):
            open(STAMP, "w").write(stamp.replace('"production":"', '"production":"0'))
            code, text = run(*PAGE)
            note("S3", "defect", code != 0 and "is not the file the built-file check walked" in text, "a stale stamp (another file's)", f"exit {code}")
            open(STAMP, "w").write(stamp)
        if want("S4"):
            # The test-only module's file stands in for "a module with other code": same tool, same glue shape, never walked.
            open(PKG, "wb").write(open(".gate/rules-module/release/devtools_bg.wasm", "rb").read())
            code, text = run(*PAGE)
            note("S4", "defect", code != 0 and "is not the file the built-file check walked" in text, "another module put in the players' place after a clean walk", f"exit {code}")
    finally:
        open(STAMP, "w").write(stamp); open(PKG, "wb").write(wasm)

def native_plants(names):
    for name, crate, flags, words in [
        ("N1", "content", "-Ctarget-feature=+fma", ['the content crate is built with ["fma"]']),
        ("N2", "sim", "-Ctarget-feature=+fma", ["the simulation is built without the fma and avx2 target features"]),
        ("N3", "sim", "-Ctarget-cpu=haswell", ["the simulation is built without the fma and avx2 target features"]),
        ("N4", "devtools", "-Ctarget-feature=+avx2", ["the simulation is built without the fma and avx2 target features"]),
    ]:
        if names and name not in names: continue
        code, text = run(*native(crate), env=dict(ENV, RUSTFLAGS=flags))
        found = [word for word in words if word in text]
        note(name, "defect", code != 0 and len(found) == len(words), f"the native tests of {crate} built with {flags}", f"exit {code}, found {len(found)} of {len(words)} wanted texts")
    if not names or "N6" in names:
        # A cargo config file in the repository, with a setting that changes no build: the rule is on the file's being there.
        os.makedirs(".cargo", exist_ok=False)
        try:
            open(".cargo/config.toml", "w").write("[term]\nverbose = false\n")
            code, text = run("cargo", "test", "-p", "devtools", "--locked", "--test", "native_target")
            note("N6", "defect", code != 0 and "holds a .cargo folder" in text, "a cargo config file committed beside the workspace", f"exit {code}")
        finally:
            os.remove(".cargo/config.toml"); os.rmdir(".cargo")
    if not names or "N5" in names:
        code, _ = run(*native("content"), env=ENV)
        note("N5", "control", code == 0, "the native tests of content built with no flag", f"exit {code}")

def anchors():
    """Reads only: every plant's text to replace stands exactly once in its file (a plant whose anchor moved is no plant)."""
    wrong = [(name, path, open(path).read().count(old)) for name, _, _, edits, _, _ in PLANTS for path, old, _ in edits if open(path).read().count(old) != 1]
    for name, path, count in wrong: print(f"{name}: its text stands {count} times in {path}")
    print(f"ANCHORS: {sum(len(plant[3]) for plant in PLANTS)} edits of {len(PLANTS)} plants read, {len(wrong)} wrong")

def main():
    if sys.argv[1] == "--anchors": return anchors()
    commit, names = sys.argv[1], set(sys.argv[2:])
    print(f"plants on {commit}", flush=True)
    # The players' file as the clean tree builds it, before any plant: what the run must leave behind.
    global PLAYERS_BEFORE
    run(*WASM)
    PLAYERS_BEFORE = digest(PKG)
    touched = sorted({path for plant in PLANTS for path, _, _ in plant[3]})
    before = {path: digest(path) for path in touched}
    for name, kind, what, edits, command, words in PLANTS:
        if names and name not in names: continue
        # A chain runs from the root; it holds environments, which are no keys.
        cwd = None if command[0] == "CHAIN" else {UNIT: "packages/client", HAND: "packages/client", PAGE_CONTENT: "packages/client", SW_UNIT: "packages/sim-worker", SW_MESSAGES: "packages/sim-worker", SW_PANIC: "packages/sim-worker"}.get(command)
        planted(name, kind, what, edits, command, words, cwd=cwd)
    stamp_plants(names)
    native_plants(names)
    back = {path: digest(path) for path in touched} == before
    # A plant built into the players' file (C18, C19) leaves it there: the clean build puts it back, checked by digest.
    players_before = PLAYERS_BEFORE
    players_back = run(*WASM)[0] == 0 and digest(PKG) == players_before
    defects = [ok for kind, ok, _ in OUT if kind == "defect"]
    controls = [ok for kind, ok, _ in OUT if kind == "control"]
    lines = [line for _, _, line in OUT]
    lines.append(f"DEFECT CHECKS: {sum(defects)} caught, {len(defects) - sum(defects)} not caught. CONTROLS: {sum(controls)} held, {len(controls) - sum(controls)} did not hold.")
    lines.append(f"EVERY FILE PUT BACK (sha256 of {len(touched)} files as at the start): {'yes' if back else 'NO'}")
    lines.append(f"THE PLAYERS' FILE REBUILT TO ITS SHA256 FROM BEFORE THE PLANTS ({players_before[:12]}...): {'yes' if players_back else 'NO'}")
    if not names:
        os.chdir("packages/client"); unit = run(*UNIT)[0]; os.chdir(ROOT)
        exits = [run(*WASM)[0], unit, run(*SCAN)[0]]
        lines.append(f"CLEAN TREE GREEN: the players' build with its built-file check, the build checks' tests, the source scan: exits {exits}")
        open(".checkpoints/cs4b-content/plants-4b2.txt", "w").write(f"plants on {commit}\n" + "\n".join(lines) + "\n")
    print("\n".join(lines[-3:]))

main()
