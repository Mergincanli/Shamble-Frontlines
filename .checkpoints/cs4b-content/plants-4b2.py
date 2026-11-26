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
SCAN = ("cargo", "test", "-p", "sim", "--locked", "--test", "rules_scan", "the_simulation_crate")
def native(crate): return ("cargo", "test", "-p", crate, "--locked", "--test", "native_target", "--target-dir", ".gate/target-plant")

USE = "use wasm_bindgen::prelude::*;\n"
MODS = "pub mod rng;\n"
def export(body, more=""): return [(BINDINGS, USE, USE + more + "\n#[wasm_bindgen]\npub fn planted(a: f32, b: f32, c: f32) -> f32 { " + body + " }\n")]
VECTOR = "parry3d::math::Vector::new"
def through_sim(body): return [(SIM, MODS, "pub fn planted(a: f32, b: f32, c: f32) -> f32 { " + body + " }\n" + MODS)] + export("sim::planted(a, b, c)")

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
        code, text = run(*command)
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

def main():
    commit, names = sys.argv[1], set(sys.argv[2:])
    print(f"plants on {commit}", flush=True)
    touched = sorted({path for plant in PLANTS for path, _, _ in plant[3]})
    before = {path: digest(path) for path in touched}
    for name, kind, what, edits, command, words in PLANTS:
        if names and name not in names: continue
        planted(name, kind, what, edits, command, words, cwd="packages/client" if command in (UNIT, HAND) else None)
    stamp_plants(names)
    native_plants(names)
    back = {path: digest(path) for path in touched} == before
    defects = [ok for kind, ok, _ in OUT if kind == "defect"]
    controls = [ok for kind, ok, _ in OUT if kind == "control"]
    lines = [line for _, _, line in OUT]
    lines.append(f"DEFECT CHECKS: {sum(defects)} caught, {len(defects) - sum(defects)} not caught. CONTROLS: {sum(controls)} held, {len(controls) - sum(controls)} did not hold.")
    lines.append(f"EVERY FILE PUT BACK (sha256 of {len(touched)} files as at the start): {'yes' if back else 'NO'}")
    if not names:
        os.chdir("packages/client"); unit = run(*UNIT)[0]; os.chdir(ROOT)
        exits = [run(*WASM)[0], unit, run(*SCAN)[0]]
        lines.append(f"CLEAN TREE GREEN: the players' build with its built-file check, the build checks' tests, the source scan: exits {exits}")
        open(".checkpoints/cs4b-content/plants-4b2.txt", "w").write(f"plants on {commit}\n" + "\n".join(lines) + "\n")
    print("\n".join(lines[-3:]))

main()
