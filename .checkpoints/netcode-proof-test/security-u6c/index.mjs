// security-auditor scratch (read-only): reads crates.io's sparse index for each named crate and prints the newest
// non-yanked versions (and the one asked for) with checksum, rust-version, publish order and requirement list.
// Use: NODE_USE_ENV_PROXY=1 node index.mjs name@version [name@version ...]
function indexPath(name) {
  const n = name.toLowerCase();
  if (n.length <= 2) return `${n.length}/${n}`;
  if (n.length === 3) return `3/${n[0]}/${n}`;
  return `${n.slice(0, 2)}/${n.slice(2, 4)}/${n}`;
}
for (const arg of process.argv.slice(2)) {
  const [name, want] = arg.split("@");
  const res = await fetch(`https://index.crates.io/${indexPath(name)}`);
  if (!res.ok) {
    console.log(`${name}: HTTP ${res.status}`);
    continue;
  }
  const rows = (await res.text()).split("\n").filter(Boolean).map((l) => JSON.parse(l));
  const live = rows.filter((r) => !r.yanked && !/-/.test(r.vers));
  const newest = live.slice(-3).map((r) => r.vers);
  const w = rows.find((r) => r.vers === want);
  const line = w
    ? `${name} ${want}: cksum ${w.cksum} rust-version ${w.rust_version ?? "-"} yanked ${w.yanked} | newest stable (index order) ${newest.join(", ")}`
    : `${name}: ${want} NOT FOUND | newest ${newest.join(", ")}`;
  console.log(line);
  if (w && process.env.DEPS) {
    for (const d of w.deps) {
      console.log(
        `   ${d.kind ?? "normal"} ${d.package ?? d.name}${d.package ? ` (as ${d.name})` : ""} ${d.req} opt=${d.optional} df=${d.default_features} feats=${JSON.stringify(d.features)}${d.target ? ` target=${d.target}` : ""}`,
      );
    }
    console.log(`   features ${JSON.stringify(w.features)} ${w.features2 ? `features2 ${JSON.stringify(w.features2)}` : ""}`);
  }
}
