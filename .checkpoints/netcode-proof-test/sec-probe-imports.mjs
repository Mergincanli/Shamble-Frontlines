// The security seat's read-only probe (u4d2-review-security.md): does the loader's empty import object refuse every
// module that imports? It builds a module with the seven listed function exports and `memory` (as check.test.mjs's
// wasmModule does) plus one imported function whose module and field names are properties reachable from `{}`
// (Object.prototype's), and calls the copy's loader.mjs `instantiate` on it. It writes nothing.
import { join } from "node:path";
import { pathToFileURL } from "node:url";

const { instantiate, moduleProblems, HASH_WASM_EXPORTS } = await import(
  pathToFileURL(join(process.cwd(), "crates", "hash-wasm", "loader.mjs")).href
);

function moduleWithImport(mod, field) {
  const leb = (n) => {
    const out = [];
    do {
      let b = n & 0x7f;
      n >>>= 7;
      if (n) b |= 0x80;
      out.push(b);
    } while (n);
    return out;
  };
  const str = (s) => [...leb(s.length), ...Buffer.from(s)];
  const section = (id, body) => [id, ...leb(body.length), ...body];
  const fns = HASH_WASM_EXPORTS;
  // type 0: () -> i32. Function 0 is the import; each export's body calls it and returns its result.
  const body = [0, 0x10, 0, 0x0b];
  return new Uint8Array([
    0x00, 0x61, 0x73, 0x6d, 1, 0, 0, 0,
    ...section(1, [1, 0x60, 0, 1, 0x7f]),
    ...section(2, [1, ...str(mod), ...str(field), 0x00, 0]),
    ...section(3, [...leb(fns.length), ...fns.map(() => 0)]),
    ...section(5, [1, 0x00, 1]),
    ...section(7, [...leb(fns.length + 1), ...fns.flatMap((x, i) => [...str(x), 0x00, ...leb(1 + i)]), ...str("memory"), 0x02, 0]),
    ...section(10, [...leb(fns.length), ...fns.flatMap(() => [...leb(body.length), ...body])]),
  ]);
}

for (const [mod, field] of [
  ["env", "now"],
  ["constructor", "keys"],
  ["__proto__", "valueOf"],
  ["constructor", "constructor"],
]) {
  const m = new WebAssembly.Module(moduleWithImport(mod, field));
  const listed = WebAssembly.Module.imports(m).map((i) => `${i.module}.${i.name} (${i.kind})`);
  let said;
  try {
    const ex = instantiate(m);
    let r;
    try {
      r = ex.step();
    } catch (e) {
      r = `step() threw ${e.constructor.name}`;
    }
    said = `INSTANTIATED (moduleProblems ${JSON.stringify(moduleProblems(m))}); step() -> ${r}`;
  } catch (e) {
    said = `refused: ${e.message.slice(0, 160)}`;
  }
  console.log(`import ${listed.join(", ")}: ${said}`);
}
