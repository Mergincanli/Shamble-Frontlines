// security-auditor scratch (read-only): OSV querybatch for crates.io packages at exact versions, then the details of
// every hit. Use: NODE_USE_ENV_PROXY=1 node osv.mjs name@version [...]
const pkgs = process.argv.slice(2).map((a) => a.split("@"));
const res = await fetch("https://api.osv.dev/v1/querybatch", {
  method: "POST",
  headers: { "content-type": "application/json" },
  body: JSON.stringify({ queries: pkgs.map(([name, version]) => ({ package: { name, ecosystem: "crates.io" }, version })) }),
});
if (!res.ok) throw new Error(`OSV HTTP ${res.status}`);
const { results } = await res.json();
let hits = 0;
for (let i = 0; i < pkgs.length; i++) {
  const vulns = results[i].vulns ?? [];
  hits += vulns.length;
  console.log(`${pkgs[i][0]} ${pkgs[i][1]}: ${vulns.length ? vulns.map((v) => v.id).join(", ") : "none"}`);
}
console.log(`queried ${pkgs.length} at ${new Date().toISOString()}, ${hits} hit(s)`);
