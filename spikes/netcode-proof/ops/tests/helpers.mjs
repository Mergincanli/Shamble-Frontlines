// Shared helpers for U8's container tests (u8-brief.md rev 1.1 section 3). Read only: no test writes outside a
// temporary folder from mkdtemp.
import { execFileSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { scanText } from "../operator/redact-check.mjs";

export const OPS = dirname(dirname(fileURLToPath(import.meta.url)));
export const SPIKE = dirname(OPS);

export function text(rel) {
  return readFileSync(join(OPS, rel), "utf8");
}

/** Parses a YAML file with PyYAML in isolated mode (no new tool: the container's python3). */
export function yamlLoad(rel) {
  const out = execFileSync(
    "python3",
    ["-I", "-c", "import json,sys,yaml; print(json.dumps(yaml.safe_load(open(sys.argv[1], encoding='utf-8'))))", join(OPS, rel)],
    { encoding: "utf8" },
  );
  return JSON.parse(out);
}

/** redact-check's findings for one text (the built-in allow-list, no VM list). */
export function leaks(body) {
  return scanText(body);
}

/** Runs `fn(dir)` with a fresh temporary folder, removed afterwards. */
export function withTemp(fn) {
  const dir = mkdtempSync(join(tmpdir(), "npt-u8-"));
  try {
    return fn(dir);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
}

/** `nft -c -f` on a ruleset text; returns { ok, err }. */
export function nftCheck(ruleset) {
  return withTemp((dir) => {
    const f = join(dir, "rules.nft");
    writeFileSync(f, ruleset);
    try {
      execFileSync("nft", ["-c", "-f", f], { stdio: ["ignore", "pipe", "pipe"] });
      return { ok: true, err: "" };
    } catch (e) {
      return { ok: false, err: String(e.stderr || e.message) };
    }
  });
}

/** The heredoc body between `<<'TAG'` and a line `TAG` in a shell text. */
export function heredoc(body, tag) {
  const m = body.match(new RegExp(`<<'${tag}'\\n([\\s\\S]*?)\\n${tag}\\n`));
  return m ? m[1] : null;
}
