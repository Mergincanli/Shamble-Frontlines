// The hand-over script on made-up values (no value of the game rests on these): what it writes, what its check finds,
// and every file it refuses to read or write. Then the repository's own files (from 4b's step 8): his saved Export,
// the manifest beside it, the table, and the movement table they give.
import { createHash } from "node:crypto";
import { existsSync, lstatSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, renameSync, rmSync, statSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { afterAll, beforeAll, describe, expect, test } from "vitest";
import { checkProblems, compareOld, contentText, EXPORT_SHA256, HEADER, MAX_READ_BYTES, PATHS, readPlain, readRows, readSource, run, tableMarkdown, writePlain } from "./testlines.mjs";

const sha256 = (text) => createHash("sha256").update(text).digest("hex");
/** A made-up Export in the sandbox's own shape: two groups, a yes/no, a rounded number and an old-game list. */
const EXPORT = {
  format: "testlines-export",
  format_version: 1,
  warnings: [],
  values: {
    simulation: { step_rate_hz: 30 },
    movement: { walk_speed_m_per_s: 5.25, sprint_multiplier: 1.5, body_radius_m: 0.1 + 0.2 },
    stamina: { stamina_on: true, refill_time_from_empty_s: 4 },
  },
  old_game_terms: { "T.moveSpeed": 5.25, "T.gravity": 24, "T.pitch (deg)": 41.3, "UNIT_TYPES.conscript.sprintMult": 1.5, "WEAPONS.smg.recoil": 0, "T.gone": 1 },
};
const TABLE = {
  format: "shamble-testlines-table",
  version: 1,
  rows: [
    { export: "stamina.stamina_on", must_be: true, what: "the game has stamina on" },
    { export: "movement.walk_speed_m_per_s", field: "walk_speed_m_per_s", unit: "m/s", old_game: "T.moveSpeed", what: "walking speed" },
    { export: "movement.sprint_multiplier", field: "sprint_multiplier", unit: "times", what: "sprint speed as a share of walking" },
    { export: "movement.body_radius_m", field: "body_radius_m", unit: "m", what: "his body against walls" },
    { export: "stamina.refill_time_from_empty_s", field: "stamina_refill_time_s", unit: "s", what: "time to refill" },
  ],
};
const CONTENT = `{
  "body_radius_m": 0.30000000000000004,
  "format": "shamble-movement",
  "id": "soldier",
  "sprint_multiplier": 1.5,
  "stamina_refill_time_s": 4,
  "version": 2,
  "walk_speed_m_per_s": 5.25
}
`;
const rows = (change) => ({ ...TABLE, rows: change(structuredClone(TABLE.rows)) });
const fills = () => readRows(EXPORT, TABLE).fills;

describe("the hand-over's rules", () => {
  test("the content file is the header and one member per row, sorted, in the game writer's form", () => {
    expect(readRows(EXPORT, TABLE).problems).toEqual([]);
    expect(fills().map(({ field }) => field)).toEqual(["walk_speed_m_per_s", "sprint_multiplier", "body_radius_m", "stamina_refill_time_s"]);
    expect(contentText(fills())).toBe(CONTENT);
    expect(checkProblems(EXPORT, TABLE, CONTENT)).toEqual([]);
    expect(HEADER).toEqual({ format: "shamble-movement", id: "soldier", version: 2 });
  });

  // The converter writes the movement kind's newest version, the one the loader's typed read takes (the advisor
  // agent's VERSION-RULE ruling, 2026-11-28): its HEADER is tied to movement.rs's own VERSION and FORMAT.
  test("the header's format and version are the movement kind's own in the loader's source", () => {
    const source = readFileSync(path.join(import.meta.dirname, "../../../crates/content/src/movement.rs"), "utf8");
    expect(/^pub const FORMAT: &str = "([^"]+)";$/m.exec(source)?.[1]).toBe(HEADER.format);
    expect(Number(/^pub const VERSION: u32 = (\d+);$/m.exec(source)?.[1])).toBe(HEADER.version);
  });

  test("a row with no place in his Export fills its field with its own value and names its place in his sandbox", () => {
    const sandboxRow = { field: "foot_radius_m", value: 0.15, sandbox: "src/movement.ts:59-60", unit: "m", what: "his foot" };
    const table = rows((list) => [...list, sandboxRow]);
    const { fills: made, problems } = readRows(EXPORT, table);
    expect(problems).toEqual([]);
    expect(made.at(-1)).toEqual({ field: "foot_radius_m", place: null, sandbox: "src/movement.ts:59-60", value: 0.15 });
    const text = contentText(made);
    expect(JSON.parse(text).foot_radius_m).toBe(0.15);
    expect(checkProblems(EXPORT, table, text)).toEqual([]);
    expect(checkProblems(EXPORT, table, text.replace('"foot_radius_m": 0.15', '"foot_radius_m": 0.2'))).toEqual([
      `${PATHS.target}: foot_radius_m is 0.2, and his sandbox has 0.15 (src/movement.ts:59-60)`,
    ]);
    expect(checkProblems(EXPORT, table, text.replace('  "foot_radius_m": 0.15,\n', ""))).toEqual([`${PATHS.target} has no foot_radius_m (his sandbox's src/movement.ts:59-60)`]);
    expect(tableMarkdown(EXPORT, table).split("\n").at(-1)).toBe("| his sandbox: src/movement.ts:59-60 | m |  | his foot | `foot_radius_m` | 0.15 |");
  });

  test("a sandbox row without its value, its place or its field, and an Export row with a value of its own, are refused", () => {
    const refused = (row, why) => {
      const { problems } = readRows(EXPORT, rows((list) => [...list, row]));
      expect(problems, JSON.stringify(row)).toHaveLength(1);
      expect(problems[0], JSON.stringify(row)).toContain(why);
    };
    const sandboxRule = "a row with no place in his Export fills a field with its own value and names its place in his sandbox";
    refused({ field: "foot_radius_m", value: 0.15 }, sandboxRule);
    refused({ field: "foot_radius_m", value: 0.15, sandbox: "  " }, sandboxRule);
    refused({ field: "foot_radius_m", value: 0.15, sandbox: 7 }, sandboxRule);
    refused({ value: 0.15, sandbox: "src/movement.ts:59" }, sandboxRule);
    refused({ must_be: 0.15, sandbox: "src/movement.ts:59" }, sandboxRule);
    refused({ field: "foot_radius_m", sandbox: "src/movement.ts:59" }, "it has no value");
    refused({ field: "foot_radius_m", value: "0.15", sandbox: "src/movement.ts:59" }, '"0.15" is not a number a content file can hold');
    refused({ field: "foot_radius_m", value: Number.NaN, sandbox: "src/movement.ts:59" }, "is not a number a content file can hold");
    refused({ field: "walk_speed_m_per_s", value: 6, sandbox: "src/movement.ts:1" }, "the field walk_speed_m_per_s is filled twice");
    const exportRule = "a row with a place in his Export takes its value from there, with no value or sandbox line of its own";
    refused({ export: "movement.walk_speed_m_per_s", field: "speed", value: 6 }, exportRule);
    refused({ export: "movement.walk_speed_m_per_s", field: "speed", sandbox: "src/movement.ts:1" }, exportRule);
  });

  test("his Export's measured block is read by a measured. place, and by none while the values hold a group of that name", () => {
    const exp = { ...structuredClone(EXPORT), measured: { soldier_height_m: 1.8 } };
    const table = rows((list) => [...list, { export: "measured.soldier_height_m", field: "body_height_m", unit: "m", what: "his height" }]);
    const { fills: made, problems } = readRows(exp, table);
    expect(problems).toEqual([]);
    expect(made.at(-1)).toEqual({ field: "body_height_m", place: "measured.soldier_height_m", sandbox: null, value: 1.8 });
    const text = contentText(made);
    expect(checkProblems(exp, table, text.replace('"body_height_m": 1.8', '"body_height_m": 1.9'))).toEqual([
      `${PATHS.target}: body_height_m is 1.9, and his Export has 1.8 (measured.soldier_height_m)`,
    ]);
    expect(readRows(EXPORT, table).problems).toEqual(['row 6 ("measured.soldier_height_m"): his Export has no such value']);
    const clash = { ...exp, values: { ...exp.values, measured: { soldier_height_m: 1.7 } } };
    expect(readRows(clash, table).problems).toEqual(['row 6 ("measured.soldier_height_m"): his Export has no such value']);
  });

  test("the check finds a value one step of a double away, a field nobody fills, a field that is gone, and a form that differs", () => {
    const off = CONTENT.replace("0.30000000000000004", "0.3");
    expect(checkProblems(EXPORT, TABLE, off)).toEqual([`${PATHS.target}: body_radius_m is 0.3, and his Export has 0.30000000000000004 (values.movement.body_radius_m)`]);
    expect(checkProblems(EXPORT, TABLE, CONTENT.replace('"sprint_multiplier": 1.5', '"sprint_multiplier": 1.501'))).toEqual([
      `${PATHS.target}: sprint_multiplier is 1.501, and his Export has 1.5 (values.movement.sprint_multiplier)`,
    ]);
    expect(checkProblems(EXPORT, TABLE, CONTENT.replace('  "sprint_multiplier": 1.5,\n', '  "jump_height_m": 2,\n'))).toEqual([
      `${PATHS.target} has no sprint_multiplier (his Export's values.movement.sprint_multiplier)`,
      `${PATHS.target}: no row of the table fills jump_height_m`,
    ]);
    expect(checkProblems(EXPORT, TABLE, CONTENT.replace('"id": "soldier"', '"id": "medic"'))).toEqual([`${PATHS.target}: id is "medic", not "soldier"`]);
    // The same values in another form: a zero spelled -0 is another number; spacing and order are another text.
    const zero = { ...EXPORT, values: { ...EXPORT.values, stamina: { stamina_on: true, refill_time_from_empty_s: 0 } } };
    expect(checkProblems(zero, TABLE, CONTENT.replace('"stamina_refill_time_s": 4', '"stamina_refill_time_s": -0'))).toEqual([
      `${PATHS.target}: stamina_refill_time_s is -0, and his Export has 0 (values.stamina.refill_time_from_empty_s)`,
    ]);
    for (const form of [CONTENT.trimEnd(), CONTENT.replaceAll("\n", "\r\n"), CONTENT.replace("5.25", "5.250"), JSON.stringify(JSON.parse(CONTENT))]) {
      expect(checkProblems(EXPORT, TABLE, form)).toEqual([`${PATHS.target} holds his values and is not the text \`write\` gives (its form differs)`]);
    }
    expect(checkProblems(EXPORT, TABLE, "not json")[0]).toContain("is not JSON");
    expect(checkProblems(EXPORT, TABLE, "[]\n")).toEqual([`${PATHS.target} is not an object`]);
  });

  test("a row is refused for anything but a field's name and a finite number of his Export", () => {
    const refused = (change, why) => {
      const { fills: made, problems } = readRows(EXPORT, rows(change));
      expect(problems).toHaveLength(1);
      expect(problems[0]).toContain(why);
      return made;
    };
    // A row that names a file instead of a field: no row can send a write anywhere.
    for (const field of ["../.claude/settings.json", "/etc/passwd", "content/source/x.json", "soldier.json", "", "Walk", "__proto__", "format", "id", "version", 7]) {
      refused((list) => [...list.slice(0, 4), { ...list[4], field }], `${JSON.stringify(field)} is not a field's name`);
    }
    refused((list) => [...list, { export: "movement.walk_speed_m_per_s", field: "speed", file: "../x" }], "a row holds export, field, must_be, old_game, sandbox, unit, value, what and nothing else, not file");
    refused((list) => [...list, "movement.walk_speed_m_per_s"], "not an object");
    // A place his Export does not have, in each way a place can be missing.
    for (const place of ["movement.jump_m", "weapons.walk_speed_m_per_s", "movement", "movement.walk_speed_m_per_s.x", "__proto__.constructor", "toString.name", 5]) {
      refused((list) => [...list, { export: place, field: "extra" }], "his Export has no such value");
    }
    // A yes/no, a text, a number no file can hold; a field filled twice; a row that does both or neither.
    refused((list) => [...list, { export: "stamina.stamina_on", field: "stamina_on" }], "true is not a number a content file can hold");
    refused((list) => [...list, { export: "movement.walk_speed_m_per_s", field: "walk_speed_m_per_s" }], "the field walk_speed_m_per_s is filled twice");
    refused((list) => [...list, { export: "movement.walk_speed_m_per_s" }], "a row fills a field or holds the Export to a value, one of the two");
    refused((list) => [...list, { export: "movement.walk_speed_m_per_s", field: "speed", must_be: 5.25 }], "one of the two");
    const exp = structuredClone(EXPORT);
    exp.values.movement.walk_speed_m_per_s = "auto";
    expect(readRows(exp, TABLE).problems).toEqual(['row 2 ("movement.walk_speed_m_per_s"): "auto" is not a number a content file can hold']);
    // A value the game is built for, and his Export has another.
    exp.values.movement.walk_speed_m_per_s = 5.25;
    exp.values.stamina.stamina_on = false;
    expect(readRows(exp, TABLE).problems).toEqual(['row 1 ("stamina.stamina_on"): his Export has false, and the game is built for true']);
    // A refused table stops the check before any content is read.
    expect(checkProblems(exp, TABLE, "not json")).toEqual(readRows(exp, TABLE).problems);
  });

  test("the Export is the pinned one, the manifest's, a Testlines Export at 30 steps with no warning", () => {
    const exportText = JSON.stringify(EXPORT);
    const source = (change = {}) => ({ exportText, manifestText: `file: x\nsha256: ${sha256(exportText)}\n`, tableText: JSON.stringify(TABLE), ...change });
    expect(readSource(source(), sha256(exportText))).toEqual({ exp: EXPORT, table: TABLE });
    expect(() => readSource(source())).toThrow(`the saved Export has sha256 ${sha256(exportText)}, and this script hands over ${EXPORT_SHA256}`);
    expect(() => readSource(source({ manifestText: `sha256: ${EXPORT_SHA256}\n` }), sha256(exportText))).toThrow("the manifest beside the Export does not name the Export's sha256");
    const other = (change) => {
      const text = JSON.stringify(change(structuredClone(EXPORT)));
      return () => readSource(source({ exportText: text, manifestText: `sha256: ${sha256(text)}\n` }), sha256(text));
    };
    expect(other((exp) => ({ ...exp, format: "something-else" }))).toThrow("not a Testlines Export of version 1");
    expect(other((exp) => ({ ...exp, format_version: 2 }))).toThrow("not a Testlines Export of version 1");
    expect(other((exp) => ({ ...exp, values: { ...exp.values, simulation: { step_rate_hz: 60 } } }))).toThrow("made at 60 steps a second, and the game runs at 30");
    expect(other((exp) => ({ ...exp, warnings: ["The step rate is 60 steps a second: the game runs at 30."] }))).toThrow("carries a warning of its own");
    expect(other((exp) => ({ ...exp, warnings: undefined }))).toThrow("carries a warning of its own");
    for (const table of [{ ...TABLE, format: "x" }, { ...TABLE, version: 2 }, { ...TABLE, rows: {} }, []]) {
      expect(() => readSource(source({ tableText: JSON.stringify(table) }), sha256(exportText))).toThrow("the table is not a shamble-testlines-table of version 1 with rows");
    }
    expect(EXPORT_SHA256).toMatch(/^[0-9a-f]{64}$/);
  });

  test("the comparison with the old game's tables sorts every name: equal, different, without a counterpart", () => {
    const legacy = { tables: { T: { values: { moveSpeed: 5.25, gravity: 24.0004, pitch: 0.72 } }, UNIT_TYPES: { values: [{ id: "conscript", sprintMult: 1.214 }] }, WEAPONS: { values: [{ id: "smg", recoil: 0.3 }] } } };
    expect(compareOld(EXPORT, legacy)).toEqual({
      // 24.0004 rounds to 24 at three decimals: equal. An Export value of 0 is not equal to an old 0.3.
      equal: ["T.moveSpeed = 5.25", "T.gravity = 24"],
      different: ["T.pitch (deg): the old game 0.72 rad = 41.253 deg; his Export 41.3", "UNIT_TYPES.conscript.sprintMult: the old game 1.214; his Export 1.5", "WEAPONS.smg.recoil: the old game 0.3; his Export 0"],
      noCounterpart: ["T.gone = 1"],
    });
    expect(compareOld({}, legacy)).toEqual({ equal: [], different: [], noCounterpart: [] });
  });

  test("the table people read is printed from the rows and the Export's own values", () => {
    expect(tableMarkdown(EXPORT, TABLE).split("\n")).toEqual([
      "| His Export (`values.`... or `measured.`...), or his sandbox | Unit | The old game's name | What it is | In `movement/soldier.json` | His value |",
      "|---|---|---|---|---|---|",
      "| `stamina.stamina_on` |  |  | the game has stamina on | must be true | true |",
      "| `movement.walk_speed_m_per_s` | m/s | T.moveSpeed | walking speed | `walk_speed_m_per_s` | 5.25 |",
      "| `movement.sprint_multiplier` | times |  | sprint speed as a share of walking | `sprint_multiplier` | 1.5 |",
      "| `movement.body_radius_m` | m |  | his body against walls | `body_radius_m` | 0.30000000000000004 |",
      "| `stamina.refill_time_from_empty_s` | s |  | time to refill | `stamina_refill_time_s` | 4 |",
    ]);
  });
});

describe("the hand-over's files", () => {
  let base;
  const exportText = `${JSON.stringify(EXPORT, null, 2)}\n`;
  const pinned = sha256(exportText);
  /** A scratch repository: the three source files, the old game's tables, and the content folders. */
  const tree = (name) => {
    const folder = path.join(base, name);
    for (const sub of ["content/source", "content/movement", "docs", "outside"]) mkdirSync(path.join(folder, sub), { recursive: true });
    writeFileSync(path.join(folder, PATHS.export), exportText);
    writeFileSync(path.join(folder, PATHS.manifest), `file: ${PATHS.export}\nsha256: ${pinned}\n`);
    writeFileSync(path.join(folder, PATHS.table), JSON.stringify(TABLE));
    writeFileSync(path.join(folder, PATHS.legacy), JSON.stringify({ tables: { T: { values: { moveSpeed: 5.25 } } } }));
    return folder;
  };
  /** Every entry under a folder with its type, size, time of change and bytes: what "nothing written" means. */
  const snapshot = (folder) =>
    readdirSync(folder, { recursive: true }).sort().map((name) => {
      const info = lstatSync(path.join(folder, name));
      return [name, info.isFile(), info.isDirectory(), info.isSymbolicLink(), info.size, info.mtimeMs, info.isFile() ? readFileSync(path.join(folder, name), "hex") : ""].join(" ");
    });
  beforeAll(() => {
    base = mkdtempSync(path.join(tmpdir(), "shamble-testlines-"));
  });
  afterAll(() => rmSync(base, { recursive: true, force: true }));

  test("write, then the check: the file holds his values; a second write gives the same bytes", () => {
    const folder = tree("clean");
    expect(() => run("--check", folder, PATHS, pinned)).toThrow("ENOENT");
    expect(run("write", folder, PATHS, pinned)).toBe(`${PATHS.target}: 4 values, 4 from his Export (sha256 ${pinned.slice(0, 8)}...) and 0 from his sandbox, written`);
    expect(readFileSync(path.join(folder, PATHS.target), "utf8")).toBe(CONTENT);
    expect(readdirSync(path.join(folder, "content/movement"))).toEqual(["soldier.json"]);
    expect(run("--check", folder, PATHS, pinned)).toBe(`${PATHS.target}: 4 values, 4 from his Export (sha256 ${pinned.slice(0, 8)}...) and 0 from his sandbox, each as the table names it`);
    run("write", folder, PATHS, pinned);
    expect(readFileSync(path.join(folder, PATHS.target), "utf8")).toBe(CONTENT);
  });

  test("the check, the comparison and the table write nothing, and the check names what differs", () => {
    const folder = tree("readers");
    run("write", folder, PATHS, pinned);
    const before = snapshot(folder);
    expect(run("compare-old", folder, PATHS, pinned)).toContain("EQUAL (1)\nT.moveSpeed = 5.25");
    expect(run("table", folder, PATHS, pinned)).toContain("| `movement.walk_speed_m_per_s` | m/s | T.moveSpeed | walking speed | `walk_speed_m_per_s` | 5.25 |");
    run("--check", folder, PATHS, pinned);
    // One value changed in the third decimal; one byte changed in the saved Export.
    writeFileSync(path.join(folder, PATHS.target), CONTENT.replace("5.25", "5.251"));
    expect(() => run("--check", folder, PATHS, pinned)).toThrow("walk_speed_m_per_s is 5.251, and his Export has 5.25");
    writeFileSync(path.join(folder, PATHS.target), CONTENT);
    writeFileSync(path.join(folder, PATHS.export), exportText.replace("5.25", "5.26"));
    for (const mode of ["--check", "write", "compare-old", "table"]) expect(() => run(mode, folder, PATHS, pinned)).toThrow("the saved Export has sha256");
    writeFileSync(path.join(folder, PATHS.export), exportText);
    for (const mode of ["", "save", "--write", "check"]) expect(() => run(mode, folder, PATHS, pinned)).toThrow("usage: testlines.mjs --check | write | compare-old | table");
    expect(snapshot(folder).map((line) => line.replace(/ \d+(\.\d+)? (?=[0-9a-f]*$)/, " "))).toEqual(before.map((line) => line.replace(/ \d+(\.\d+)? (?=[0-9a-f]*$)/, " ")));
  });

  test("a refused table writes nothing, wherever its rows point", () => {
    const folder = tree("table");
    run("write", folder, PATHS, pinned);
    const before = snapshot(folder);
    for (const field of ["../.claude/settings.json", path.join(folder, "outside/x.json"), "content/source/x.json"]) {
      writeFileSync(path.join(folder, PATHS.table), JSON.stringify(rows((list) => [...list, { export: "movement.sprint_multiplier", field }])));
      expect(() => run("write", folder, PATHS, pinned)).toThrow("is not a field's name");
      expect(() => run("--check", folder, PATHS, pinned)).toThrow("is not a field's name");
    }
    writeFileSync(path.join(folder, PATHS.table), JSON.stringify(TABLE));
    expect(readdirSync(path.join(folder, "outside"))).toEqual([]);
    expect(existsSync(path.join(folder, "content/source/x.json"))).toBe(false);
    expect(readFileSync(path.join(folder, PATHS.target), "utf8")).toBe(CONTENT);
    expect(snapshot(folder).filter((line) => !line.startsWith(PATHS.table))).toEqual(before.filter((line) => !line.startsWith(PATHS.table)));
  });

  test("no file is read or written through a link, and no read is larger than its cap", () => {
    const folder = tree("links");
    const secret = path.join(folder, "outside/secret.json");
    writeFileSync(secret, exportText);
    // The Export, the manifest and the table as links to real files: each is refused before it is read.
    for (const name of ["export", "manifest", "table"]) {
      const file = path.join(folder, PATHS[name]);
      const kept = readFileSync(file);
      rmSync(file);
      symlinkSync(name === "export" ? secret : path.join(folder, PATHS.legacy), file);
      expect(() => run("write", folder, PATHS, pinned)).toThrow(`${file} is a link or not a plain file: it is not read`);
      rmSync(file);
      writeFileSync(file, kept);
    }
    // The target as a link, and its folder as a link: nothing is written through either.
    const target = path.join(folder, PATHS.target);
    symlinkSync(secret, target);
    expect(() => run("write", folder, PATHS, pinned)).toThrow(`${target} is a link or not a plain file: nothing is written`);
    expect(() => run("--check", folder, PATHS, pinned)).toThrow(`${target} is a link or not a plain file: it is not read`);
    rmSync(target);
    rmSync(path.join(folder, "content/movement"), { recursive: true });
    symlinkSync(path.join(folder, "outside"), path.join(folder, "content/movement"));
    expect(() => run("write", folder, PATHS, pinned)).toThrow("is a link or not a plain folder: nothing is written");
    expect(readdirSync(path.join(folder, "outside"))).toEqual(["secret.json"]);
    expect(readFileSync(secret, "utf8")).toBe(exportText);
    // A leftover of an earlier write stands in the new file's place: refused, not written through.
    rmSync(path.join(folder, "content/movement"));
    mkdirSync(path.join(folder, "content/movement"));
    symlinkSync(secret, path.join(folder, "content/movement/.soldier.json.new"));
    expect(() => writePlain(target, "x")).toThrow("EEXIST");
    expect(readFileSync(secret, "utf8")).toBe(exportText);
    // A link higher up, the content folder itself (security-auditor's round of 2026-11-27): nothing is read through
    // it and nothing written, though every folder and file below it is plain.
    const real = path.join(folder, "outside/content");
    renameSync(path.join(folder, "content"), real);
    symlinkSync(real, path.join(folder, "content"));
    expect(() => run("--check", folder, PATHS, pinned)).toThrow(`${path.join(folder, "content")} is a link or not a plain folder: it is not read`);
    expect(() => writePlain(target, "x", folder)).toThrow(`${path.join(folder, "content")} is a link or not a plain folder: nothing is written`);
    // And a file that lies outside the folder the reads start from is not read at all.
    expect(() => readPlain(secret, path.join(folder, "content/movement"))).toThrow("is not under");
    rmSync(path.join(folder, "content"));
    renameSync(real, path.join(folder, "content"));
    expect(readFileSync(secret, "utf8")).toBe(exportText);
    // The cap: a file of exactly the cap is read; one byte more is refused.
    const large = path.join(folder, "outside/large.txt");
    writeFileSync(large, "x".repeat(MAX_READ_BYTES));
    expect(readPlain(large)).toHaveLength(MAX_READ_BYTES);
    writeFileSync(large, "x".repeat(MAX_READ_BYTES + 1));
    expect(() => readPlain(large)).toThrow(`is larger than the ${MAX_READ_BYTES} bytes a read takes`);
    expect(statSync(large).size).toBe(MAX_READ_BYTES + 1);
    // A path that climbs out through a link (`lnk/..`) is read as it reads: the folders checked are the file's.
    mkdirSync(path.join(folder, "outside/deep"));
    mkdirSync(path.join(folder, "outside/movement"));
    writeFileSync(path.join(folder, "outside/movement/inside.json"), "OUTSIDE");
    writeFileSync(path.join(folder, "content/movement/inside.json"), "INSIDE");
    symlinkSync(path.join(folder, "outside/deep"), path.join(folder, "content/lnk"));
    expect(readPlain(`${folder}/content/lnk/../movement/inside.json`, folder), "a path that climbs out through a link").toBe("INSIDE");
    writePlain(`${folder}/content/lnk/../movement/inside.json`, "WRITTEN", folder);
    expect(readFileSync(path.join(folder, "outside/movement/inside.json"), "utf8"), "a write that climbs out through a link").toBe("OUTSIDE");
  });
});

describe("his Export and the game's first movement table, the repository's own files", () => {
  test("the check holds content/movement/soldier.json to his saved Export and his sandbox through the table: every value, bit for bit, in the writer's form", () => {
    expect(run("--check")).toBe(`${PATHS.target}: 28 values, 16 from his Export (sha256 ${EXPORT_SHA256.slice(0, 8)}...) and 12 from his sandbox, each as the table names it`);
  });

  // The check above reads the values through the table, so a row sent to the wrong field would pass it, whatever
  // the values (test-engineer's round of 2026-11-27). The table's own names hold the rows in place: each row's field
  // is the last step of its Export name, but for the one rename named here, so no two rows can trade fields. And
  // its group is one of the soldier's, so no row reads another unit's entry of the same name (his Export also holds
  // the hornet's body radius and step: test-engineer's round 2).
  test("every row of the table sends its Export value to the field of the same name, but for two named renames", () => {
    const RENAMED = { "stamina.refill_time_from_empty_s": "stamina_refill_time_s", "measured.soldier_height_m": "body_height_m" };
    const GROUPS = ["movement", "jump", "stamina", "pushing", "aim_down_sights", "measured"];
    const { rows } = JSON.parse(readFileSync(path.join(import.meta.dirname, "../../..", PATHS.table), "utf8"));
    const filled = rows.filter((row) => row.field !== undefined && row.export !== undefined);
    expect(filled).toHaveLength(16);
    for (const { export: name, field } of filled) {
      expect(field, name).toBe(RENAMED[name] ?? name.split(".").at(-1));
      expect(GROUPS, name).toContain(name.split(".")[0]);
    }
    // Each rename stands in the table once, and every field once: the rule above leaves no row out.
    expect(filled.filter(({ export: name }) => name in RENAMED)).toHaveLength(2);
    expect(new Set(filled.map(({ field }) => field)).size).toBe(16);
  });

  // His NO-SOURCE (b) of 2026-11-28: the numbers his sandbox keeps in its code fill the twelve fields named here, each
  // with its place in his sandbox, and no other; together with the sixteen Export rows they are the movement kind's
  // 28 fields, each filled once.
  test("the twelve sandbox rows fill the twelve fields his sandbox keeps in its code, each with its place there", () => {
    const SANDBOX_FIELDS = [
      "foot_radius_m", "pull_share_per_step", "pull_done_under_m", "pull_to_foot_pct", "step_allowance_m", "snap_down_m",
      "push_height_gap_m", "push_sideways_share", "push_keep_going_share", "push_most_m_per_step", "push_head_on_gate_m_per_s",
      "push_keep_going_gate_m_per_s",
    ];
    const { rows } = JSON.parse(readFileSync(path.join(import.meta.dirname, "../../..", PATHS.table), "utf8"));
    const fromSandbox = rows.filter((row) => row.export === undefined);
    expect(fromSandbox.map(({ field }) => field)).toEqual(SANDBOX_FIELDS);
    for (const row of fromSandbox) {
      expect(Number.isFinite(row.value), row.field).toBe(true);
      expect(row.sandbox, row.field).toMatch(/^src\/[a-z]+\.ts:\d+/);
    }
    expect(new Set(rows.map(({ field }) => field).filter(Boolean)).size).toBe(28);
  });
});
