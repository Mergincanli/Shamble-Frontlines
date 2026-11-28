#!/usr/bin/env python3
"""Change-set 4c's planted defects, step S (the page's scene): each new check is shown red on the defect it exists to
catch, then green again. 4b-2's script (`plants-4b2.py`, readable at 59f6cc7) in its form, with step S's plants.

From the worktree's root, with no other run in it (the script edits source files, one plant at a time, and puts each
back whatever happens; it says at its end whether every file is byte for byte as at the start):
    python3 -I -B .checkpoints/cs4c-soldier/plants-4c.py <commit id> [names, e.g. T1 S5]
    python3 -I -B .checkpoints/cs4c-soldier/plants-4c.py --anchors
It writes one line per check to .checkpoints/cs4c-soldier/plants-4c.txt (a whole run only) and the commands' output to
/home/user/Shamble-Frontlines/.gate/cs4b-2/.gate/plants-4c.log. The commit id is the caller's `git rev-parse HEAD`;
the caller also shows `git status --porcelain -- crates packages` empty before and after (this script runs no git). A
plant counts as caught only when the command fails AND its output holds the check's own words for that defect.
Not part of the product: notes.
"""
import hashlib, os, signal, subprocess, sys

ROOT = "/home/user/Shamble-Frontlines/.gate/cs4b-2"
os.chdir(ROOT)
LOG = open("/home/user/Shamble-Frontlines/.gate/cs4b-2/.gate/plants-4c.log", "a")
OUT = []
ENV = dict(os.environ, SHAMBLE_WASM_BINDGEN=f"{ROOT}/.gate/wb-tool/wasm-bindgen")
CONTENT, BOOT, REPORT = "packages/client/src/app/content.ts", "packages/client/src/app/boot.ts", "packages/client/src/overlay/report.ts"
SCENE, INVENTORY, PICTURE = "packages/client/src/render/scene.ts", "packages/client/src/render/inventory.ts", "packages/client/src/render/three-picture.ts"
SCHEMA_TEST = "packages/types/src/content-schema.test.mjs"
DEV_PLANT = f"{ROOT}/.gate/dist-plant-dev-4c"
PROD_PLANT = f"{ROOT}/.gate/dist-plant-prod-4c"

def run(*command, env=None, cwd=None, timeout=None):
    # Its own process group, so that a timeout ends Vitest's workers too (test-engineer, step S's review: killing only
    # the vitest process left its worker spinning). A run stopped by its timeout reads as exit 124 with a note.
    process = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, env=env or ENV, cwd=cwd or ROOT, start_new_session=True)
    try:
        text, _ = process.communicate(timeout=timeout)
        code = process.returncode
    except subprocess.TimeoutExpired:
        os.killpg(process.pid, signal.SIGKILL)
        text, _ = process.communicate()
        code, text = 124, (text or "") + f"\nTIMED OUT after {timeout} s: the run's whole process group was ended\n"
    LOG.write(f"\n$ {' '.join(command)}\n{text}\n[exit {code}]\n"); LOG.flush()
    return code, text

def vitest(package, *files, timeout=None): return ("VITEST", package, files, timeout)
def page_build(production): return ("node", f"{ROOT}/packages/client/scripts/build.mjs", *(["--production"] if production else []), "--out-dir", PROD_PLANT if production else DEV_PLANT)
def chromium(*specs): return ("node", f"{ROOT}/node_modules/@playwright/test/cli.js", "test", "-c", f"{ROOT}/packages/e2e/playwright.config.ts", "--project=chromium", "--workers=1", "--retries=0", "--trace", "off", "--reporter=list", *specs)
BROWSER_ENV = dict(ENV, SHAMBLE_DIST=DEV_PLANT, SHAMBLE_CHROMIUM="/opt/pw-browsers/chromium")
def browser(*specs): return ("CHAIN", (ENV, page_build(False)), (BROWSER_ENV, chromium(*specs)))
SKELETON, CONTENT_SPEC = "packages/e2e/tests/skeleton/skeleton.spec.ts", "packages/e2e/tests/skeleton/content.spec.ts"

READ_TESTS = vitest("client", "src/app/content.test.ts", "src/app/content-map.test.mjs")
SCENE_TESTS = vitest("client", "src/render/scene.test.ts")

# name, kind, what the defect is, the edits (file, text there once, its replacement), the command, the words its output must hold
PLANTS = [
    ("T1", "defect", "the map's reading leaves its patches of ground out",
     [(CONTENT, "{ id, bounds: { min: boundsMin, max: boundsMax }, spawns, shapes, regions } };", "{ id, bounds: { min: boundsMin, max: boundsMax }, spawns, shapes, regions: [] } };")],
     READ_TESTS, ["the committed test scene reads field by field as the file itself", "the map reads field by field from own properties"]),
    ("T2", "defect", "no key-set check: a key more is let through",
     [(CONTENT, "  return Object.keys(object).length === keys.length && keys.every((key) => Object.hasOwn(object, key));", "  return keys.every((key) => Object.hasOwn(object, key));")],
     READ_TESTS, ["one key more in any object of the map gives the map's fixed sentence"]),
    ("T3", "defect", "the parsed tree cast to the generated type, read no further",
     [(CONTENT, "export function readMap(tree: unknown): Outcome<MapDocument> {\n", "export function readMap(tree: unknown): Outcome<MapDocument> {\n  if (tree !== null && typeof tree === \"object\") return { ok: true, value: tree as MapDocument };\n")],
     READ_TESTS, ["a member of the wrong kind gives the map's fixed sentence"]),
    ("T4", "defect", "rises_toward let through on a box",
     [(CONTENT, '      if (!closed(entry, ["id", "min", "max", "type"])) return fault;', '      if (!closed(entry, ["id", "min", "max", "type"]) && !closed(entry, ["id", "min", "max", "type", "rises_toward"])) return fault;')],
     READ_TESTS, ["rises_toward belongs to a ramp alone"]),
    ("T5", "defect", "no key-set check, and fields read through the prototype",
     [(CONTENT, "  return Object.keys(object).length === keys.length && keys.every((key) => Object.hasOwn(object, key));", "  return keys.length >= 0;"),
      (CONTENT, "  return Object.hasOwn(object, key) ? (object as Readonly<Record<string, unknown>>)[key] : undefined;", "  return (object as Readonly<Record<string, unknown>>)[key];")],
     READ_TESTS, ["nothing is read through a prototype"]),
    ("T6", "defect", "a refusal that names what it met",
     [(CONTENT, '  if (id === null || !isObject(boundsTree) || !closed(boundsTree, ["min", "max"])) return fault;', '  if (id === null || !isObject(boundsTree) || !closed(boundsTree, ["min", "max"])) return { ok: false, problem: `${NOT_THE_MAP}: ${String(own(tree, "id"))}` };')],
     READ_TESTS, ["every refusal is one of the two fixed sentences"]),
    ("T7", "defect", "the bundle's first map taken, not the local match's",
     [(CONTENT, "    if (path === `maps/${LOCAL_MAP}.json`) {", '    if (path.startsWith("maps/") && map === null) {')],
     READ_TESTS, ["the bundle's files: each folder's kind, format and version"]),
    ("T8", "defect", "the copy kept after its reading: a second message reads and delivers again",
     [(CONTENT, "    const kept = this.#kept;\n    this.#kept = null;\n    const read = this.#read(kept);", "    const kept = this.#kept;\n    const read = this.#read(kept);")],
     READ_TESTS, ["on the Worker's ready the page reads its copy once and lets it go"]),
    ("T9", "defect", "the map's format word not the loader's",
     [(CONTENT, 'export const MAP_FORMAT = "shamble-map";', 'export const MAP_FORMAT = "shamble-maps";')],
     vitest("client", "src/app/content-ties.test.mjs"), ["each kind's format and newest version"]),
    ("T10", "defect", "the member-kind compare that finds nothing",
     [(SCHEMA_TEST, "    .filter(([, , typedKind, schemaKindOf]) => typedKind !== schemaKindOf || typedKind === \"unknown\")", "    .filter(() => false)")],
     vitest("types", "src/content-schema.test.mjs"), ["each member of the generated types is of the kind its schema says"]),
    ("S1", "defect", "the -x ramp built with the +x rule",
     [(SCENE, '    case "-x":\n      return [...floor, [x0, y1, z0], [x0, y1, z1]];', '    case "-x":\n      return [...floor, [x1, y1, z0], [x1, y1, z1]];')],
     SCENE_TESTS, ["each solid is drawn at its own corners"]),
    ("S1b", "defect", "the faces left as listed, never turned outward",
     [(SCENE, "      if (dot(n, sub(p, centre)) < 0) {", "      if (dot(n, sub(p, centre)) < -Infinity) {")],
     SCENE_TESTS, ["every triangle of every solid faces outward"]),
    # Run 1's S1c put the +z ramp's list on the -z ramp: the same faces under the other names (the high wall and the
    # slope trade places), so no defect at all, and rightly not caught. Its place: the +x ramp's list, whose faces cut
    # through the -z wedge.
    ("S1c", "defect", "the -z ramp's faces drawn with the +x ramp's list",
     [(SCENE, ': shape.rises_toward === "+z" ? [3, 2, 0, 1] : [0, 1, 3, 2];', ': shape.rises_toward === "+z" ? [3, 2, 0, 1] : [1, 2, 0, 3];')],
     SCENE_TESTS, ["every triangle of every solid faces outward", "ramp_mz volume"]),
    ("S2", "defect", "the last shape skipped",
     [(SCENE, "  for (const shape of map.shapes) {\n    const mesh", "  for (const shape of map.shapes.slice(0, -1)) {\n    const mesh")],
     SCENE_TESTS, ["each shape and patch is drawn once"]),
    ("S3", "defect", "the greys turned upside down",
     [(SCENE, "  const share = tallest > lowest ? (top - lowest) / (tallest - lowest) : 1;", "  const share = tallest > lowest ? 1 - (top - lowest) / (tallest - lowest) : 1;")],
     SCENE_TESTS, ["the greys: neutral, a higher top never darker"]),
    ("S3b", "defect", "the empty floor's blue grey kept for the map's ground",
     [(SCENE, "const MAP_GROUND = 0x3c3c3c;", "const MAP_GROUND = 0x6b7480;")],
     SCENE_TESTS, ["the greys: neutral, a higher top never darker"]),
    ("S4", "defect", "the empty floor's 30 m grid kept for the map",
     [(SCENE, "  for (let x = Math.ceil(minX); x <= maxX; x += 1) points.push(x, y, minZ, x, y, maxZ);", "  for (let x = -15; x <= 15; x += 1) points.push(x, y, minZ, x, y, maxZ);")],
     SCENE_TESTS, ["the grid: one line per whole metre over the bounds"]),
    ("S5", "defect", "the old floor's materials not freed",
     [(SCENE, "  for (const material of materials) material.dispose();", "  void materials;")],
     SCENE_TESTS, ["the swap frees each geometry and material of the old floor once"]),
    ("S5b", "defect", "one material cache shared by every floor",
     [(SCENE, "  const materials = new Map<number, THREE.MeshStandardNodeMaterial>();\n  const material = (grey", "  const material = (grey"),
      (SCENE, "export function buildMapFloor(map: MapDocument): Floor {", "const materials = new Map<number, THREE.MeshStandardNodeMaterial>();\nexport function buildMapFloor(map: MapDocument): Floor {")],
     SCENE_TESTS, ["the swap frees each geometry and material of the old floor once"]),
    ("S6", "defect", "today's fixed camera kept",
     [(SCENE, "  const position = target.clone().addScaledVector(VIEW_DIRECTION, far);", "  const position = new THREE.Vector3(0, 14, 16);")],
     SCENE_TESTS, ["the overview shows the whole floor"]),
    ("S7", "defect", "an inventory that lists the corners the builder wrote beside the meshes",
     [(SCENE, "    floor.add(mark(mesh, shape.id, shape.type));", "    floor.add(mark(mesh, shape.id, shape.type));\n    mesh.userData[\"points\"] = solidPoints(shape).map((p) => p.map(Math.fround));"),
      (INVENTORY, "    const points = [...seen.values()]", "    const listed = object.userData[\"points\"] as [number, number, number][] | undefined;\n    const points = (listed ?? [...seen.values()])")],
     SCENE_TESTS, ["the inventory reads what is drawn, not the map"]),
    ("S7b", "defect", "an unknown object skipped, never said",
     [(INVENTORY, "throw new Error(INVENTORY_UNKNOWN);", "continue;")],
     SCENE_TESTS, ["the inventory reads what is drawn, not the map"]),
    ("R1", "defect", "every file's version printed from the first file",
     [(REPORT, "  return content.map(({ path, format, version }) => `${path} ${format} ${version}`).join(\" · \");", "  return content.map(({ path, format }) => `${path} ${format} ${content[0]?.version}`).join(\" · \");")],
     vitest("client", "src/overlay/report.test.ts"), ["the content and the scene: each file with its own format and version"]),
    ("E1", "defect", "boot taking the Worker's messages through the link alone: no map is drawn (4B-BOOT-CALLS)",
     [(BOOT, "      const problem = takeWorkerMessage(link, content, event);", "      const problem = link.receive(event);")],
     browser(SKELETON, "-g", "the first page on the default backend"), ["the first page on the default backend"]),
    ("E1b", "defect", "the swap without freeing the empty floor (its geometries stay counted)",
     [(PICTURE, "      disposeFloor(floor);\n", "")],
     browser(SKELETON, "-g", "the first page on the default backend"), ["the first page on the default backend", '"geometries": 32']),
    ("E2", "defect", "the map read and drawn before the Worker's ready",
     [(CONTENT, "    this.#kept = fetched.kept;\n    link.start(padBytes, fetched.bundle);", "    this.#kept = fetched.kept;\n    const early = this.#read(fetched.kept);\n    if (early.ok) this.#deliver(early.value);\n    link.start(padBytes, fetched.bundle);")],
     browser(CONTENT_SPEC, "-g", "a duplicate key"), ["a bundle with a duplicate key is refused by the loader, said as text, and the scene stays empty"]),
    ("E3", "defect", "boot taking the Worker's error through the link alone: the copy stays held (4B-BOOT-CALLS)",
     [(BOOT, "    const problem = takeWorkerError(link, content, event.message);", "    const problem = link.workerError(event.message);")],
     browser(SKELETON, "-g", "a Worker that cannot be loaded"), ["a Worker that cannot be loaded stops the match once"]),
    ("E4", "defect", "the inventory called outside the build flag: development code in the players' file",
     [(PICTURE, "      return __SHAMBLE_DEVTOOLS__ ? readInventory(floor) : null;", "      return readInventory(floor);")],
     page_build(True), ["development code in a production build", "the scene inventory met an object it does not know"]),
    # The fix pass after step S's review round (2026-11-28; `review-s-round.md`): each new test shown red on its defect.
    ("F1", "defect", "the overview's search with no limit (GE-F1): at aspect 0 it never ends, so the run hangs until its outer timeout",
     [(SCENE, "    if (doubled === MOST_DOUBLINGS) return null;\n", "")],
     vitest("client", "src/render/scene.test.ts", timeout=90), ["TIMED OUT"]),
    ("F2", "defect", "the framing without its no-view check (TE-7)",
     [(SCENE, "  if (view === null) return false;\n", "")],
     SCENE_TESTS, ["the camera takes the overview"]),
    ("F3", "defect", "the far plane left at 500 m whatever the distance (TE-7)",
     [(SCENE, "  camera.far = Math.max(500, 4 * camera.position.distanceTo(new THREE.Vector3(...view.target)));", "  camera.far = 500;")],
     SCENE_TESTS, ["the camera takes the overview"]),
    ("F4", "defect", "every floor piece facing down, unseen from above (TE-1)",
     [(SCENE, "  const positions = [x0, y, z0, x0, y, z1, x1, y, z1, x0, y, z0, x1, y, z1, x1, y, z0];", "  const positions = [x0, y, z0, x1, y, z1, x0, y, z1, x0, y, z0, x1, y, z0, x1, y, z1];")],
     SCENE_TESTS, ["the ground and each patch lie flat"]),
    ("F5", "defect", "the ground drawn at y 0, not at the bounds' bottom (G-3)",
     [(SCENE, '  floor.add(mark(new THREE.Mesh(floorQuad(minX, minZ, maxX, maxZ, floorY), material(MAP_GROUND, 2)), "ground", "ground"));', '  floor.add(mark(new THREE.Mesh(floorQuad(minX, minZ, maxX, maxZ, 0), material(MAP_GROUND, 2)), "ground", "ground"));')],
     SCENE_TESTS, ["the ground and each patch lie flat"]),
    ("F6", "defect", "each patch over half its depth (TE-1)",
     [(SCENE, "  const [x1, z1] = region.max;", "  const [x1, z1full] = region.max;\n  const z1 = (z0 + z1full) / 2;")],
     SCENE_TESTS, ["the ground and each patch lie flat"]),
    ("F7", "defect", "the map's ground the empty floor's 30 m, not its bounds (TE-1)",
     [(SCENE, '  floor.add(mark(new THREE.Mesh(floorQuad(minX, minZ, maxX, maxZ, floorY), material(MAP_GROUND, 2)), "ground", "ground"));', '  floor.add(mark(new THREE.Mesh(floorQuad(-15, -15, 15, 15, floorY), material(MAP_GROUND, 2)), "ground", "ground"));')],
     SCENE_TESTS, ["the ground and each patch lie flat"]),
    ("F8", "defect", "a box's floor drawn as one triangle twice, which keeps the volume (TE-2, GE-F5)",
     [(SCENE, "      [0, 1, 2, 3],\n      [4, 5, 6, 7],", "      [0, 1, 2, 1],\n      [4, 5, 6, 7],")],
     SCENE_TESTS, ["every triangle of every solid faces outward", "drawn 2 times"]),
    ("F9", "defect", "the grid's lines along z starting at z 0 (TE-4)",
     [(SCENE, "  for (let x = Math.ceil(minX); x <= maxX; x += 1) points.push(x, y, minZ, x, y, maxZ);", "  for (let x = Math.ceil(minX); x <= maxX; x += 1) points.push(x, y, 0, x, y, maxZ);")],
     SCENE_TESTS, ["the grid: one line per whole metre"]),
    ("F10", "defect", "four greys in place of every level (TE-5)",
     [(SCENE, "  const level = Math.round(BLOCK_LOW + share * (BLOCK_HIGH - BLOCK_LOW));", "  const level = Math.round(BLOCK_LOW + (Math.round(share * 3) / 3) * (BLOCK_HIGH - BLOCK_LOW));")],
     SCENE_TESTS, ["the greys: neutral, a higher top never darker"]),
    ("F11", "defect", "the camera ten times farther than fits (TE-3)",
     [(SCENE, "  const position = target.clone().addScaledVector(VIEW_DIRECTION, far);", "  const position = target.clone().addScaledVector(VIEW_DIRECTION, far * 10);")],
     SCENE_TESTS, ["the overview shows the whole floor at 16:9"]),
    ("F12", "defect", "a copy through Object.assign before the key check drops an own __proto__ key (S-2)",
     [(CONTENT, "  return Object.keys(object).length === keys.length && keys.every((key) => Object.hasOwn(object, key));", "  const copy = Object.assign({}, object);\n  return Object.keys(copy).length === keys.length && keys.every((key) => Object.hasOwn(copy, key));")],
     READ_TESTS, ["an own key named __proto__"]),
    ("F13", "defect", "the page's own refusal not remembered: the readout would say nothing was read (EA-5)",
     [(CONTENT, "      this.#refused = true;\n", "")],
     READ_TESTS, ["on the Worker's ready the page reads its copy once"]),
    ("F14", "defect", "the readout's words for the page's refusal missing (EA-5)",
     [(REPORT, '  if (content === "refused") return "read and refused by the page, the notice says why";\n', "")],
     vitest("client", "src/overlay/report.test.ts"), ["the content and the scene: each file with its own format and version"]),
    ("F15", "defect", "a ground word the page knows and the loader does not (EA-3)",
     [(CONTENT, 'export const MAP_GROUNDS = ["street", "grass"] as const;', 'export const MAP_GROUNDS = ["street", "grass", "water"] as const;')],
     vitest("client", "src/app/content-ties.test.mjs"), ["the map's fixed words the page reads are the loader's own"]),
    ("F16", "defect", "the inventory reading any material, not the floor's own (EA-4)",
     [(INVENTORY, " || !(object.material instanceof THREE.MeshStandardNodeMaterial)", "")],
     SCENE_TESTS, ["the inventory reads what is drawn, not the map"]),
    ("F17", "defect", "the +z ramps' tops on the -z side: the browser's every-corner compare (TE-6)",
     [(SCENE, '    case "+z":\n      return [...floor, [x0, y1, z1], [x1, y1, z1]];', '    case "+z":\n      return [...floor, [x0, y1, z0], [x1, y1, z0]];')],
     browser(SKELETON, "-g", "the first page on the default backend"), ["the first page on the default backend", "street_house_a_roof_north"]),
    ("C1", "control", "the clean tree: the page's reading and scene tests", [], vitest("client", "src/app/content.test.ts", "src/app/content-map.test.mjs", "src/app/content-ties.test.mjs", "src/render/scene.test.ts", "src/overlay/report.test.ts"), []),
    ("C2", "control", "the clean tree: a development page built and its first page shown on the default backend", [], browser(SKELETON, "-g", "the first page on the default backend"), []),
    ("C3", "control", "the clean tree: the production page with its marker check", [], page_build(True), []),
]

def note(name, kind, ok, what, detail):
    words = {"defect": ("CAUGHT", "NOT CAUGHT"), "control": ("HELD", "DID NOT HOLD")}[kind]
    line = f"{name} {words[0] if ok else words[1]} ({kind}): {what}: {detail}"
    OUT.append((kind, ok, line)); print(line, flush=True)

def digest(path): return hashlib.sha256(open(path, "rb").read()).hexdigest() if os.path.exists(path) else "absent"

def execute(command):
    if command[0] == "VITEST":
        _, package, files, timeout = command
        return run("node", f"{ROOT}/node_modules/vitest/vitest.mjs", "run", *files, cwd=f"{ROOT}/packages/{package}", timeout=timeout)
    if command[0] == "CHAIN":
        code, text = 0, ""
        for env, part in command[1:]:
            code, more = run(*part, env=env)
            text += more
            if code != 0: break
        return code, text
    return run(*command)

def planted(name, kind, what, edits, command, words):
    originals = {path: open(path).read() for path, _, _ in edits}
    try:
        for path, old, new in edits:
            text = open(path).read()
            count = text.count(old)
            if count != 1: return note(name, kind, False, what, f"the text to replace stands {count} times in {path}")
            open(path, "w").write(text.replace(old, new, 1))
        code, text = execute(command)
        found = [word for word in words if word in text]
        ok = (code == 0 if kind == "control" else code != 0) and len(found) == len(words)
        note(name, kind, ok, what, f"exit {code}, found {len(found)} of {len(words)} wanted texts" + ("" if ok else f"; missing {[w for w in words if w not in found]}"))
    finally:
        for path, original in originals.items(): open(path, "w").write(original)

def anchors():
    """Reads only: every plant's text to replace stands exactly once in its file (a plant whose anchor moved is no plant)."""
    wrong = []
    for name, _, _, edits, _, _ in PLANTS:
        texts = {}
        for path, old, new in edits:
            text = texts.get(path, open(path).read())
            if text.count(old) != 1: wrong.append((name, path, text.count(old)))
            texts[path] = text.replace(old, new, 1)
    for name, path, count in wrong: print(f"{name}: its text stands {count} times in {path}")
    print(f"ANCHORS: {sum(len(plant[3]) for plant in PLANTS)} edits of {len(PLANTS)} plants read, {len(wrong)} wrong")

def main():
    if sys.argv[1] == "--anchors": return anchors()
    commit, names = sys.argv[1], set(sys.argv[2:])
    print(f"plants on {commit}", flush=True)
    touched = sorted({path for plant in PLANTS for path, _, _ in plant[3]})
    before = {path: digest(path) for path in touched}
    for name, kind, what, edits, command, words in PLANTS:
        if names and name not in names: continue
        planted(name, kind, what, edits, command, words)
    back = {path: digest(path) for path in touched} == before
    defects = [ok for kind, ok, _ in OUT if kind == "defect"]
    controls = [ok for kind, ok, _ in OUT if kind == "control"]
    lines = [line for _, _, line in OUT]
    lines.append(f"DEFECT CHECKS: {sum(defects)} caught, {len(defects) - sum(defects)} not caught. CONTROLS: {sum(controls)} held, {len(controls) - sum(controls)} did not hold.")
    lines.append(f"EVERY FILE PUT BACK (sha256 of {len(touched)} files as at the start): {'yes' if back else 'NO'}")
    if not names:
        open(f"{ROOT}/.checkpoints/cs4c-soldier/plants-4c.txt", "w").write(f"plants on {commit}\n" + "\n".join(lines) + "\n")
    print("\n".join(lines[-2:]))

main()
