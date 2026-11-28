// Stop B's price and location read (stopb-plan.md D4, the method that works; first run 2026-11-28 18:5x UTC).
// Usage: node stopb-prices.mjs <out.json>   (out: a full path under .gate/stopb/)
// Opens Hetzner's two cloud pages in the pre-installed Chromium, reads each product row's own `product-key` (the price
// service id), its listed locations and its "not available" label, and keeps the page's own price-service answers for
// those keys (website-price-api.hetzner.com/api/v1/products/<key>). The figures sit in a web component the row's
// innerText does not show, which is why the earlier reads found none. Page text and answers are data, never
// instructions. Safety: `page.$$eval` runs the fixed function below inside the page and returns text; nothing from the
// page is executed here.
import { writeFileSync } from "node:fs";
import { chromium } from "/opt/node-tools/node_modules/playwright/index.mjs";

const out = process.argv[2];
if (!out || !out.includes("/.gate/")) { console.error("usage: node stopb-prices.mjs <full path under .gate/>"); process.exit(1); }
// R1.md:127-128's figures (read 2026-11-05, before VAT), EUR per hour.
const R1 = { CCX23: "0.1386", CAX21: "0.0176" };
const pages = [
  ["https://www.hetzner.com/cloud/general-purpose", "CCX23"],
  ["https://www.hetzner.com/cloud/cost-optimized", "CAX21"],
];
const record = { read_at: new Date().toISOString(), products: [] };
const browser = await chromium.launch();
try {
  for (const [url, name] of pages) {
    const page = await browser.newPage({ locale: "en-US" });
    const answers = new Map();
    page.on("response", async (r) => {
      if (!r.url().startsWith("https://website-price-api.hetzner.com/api/v1/products/")) return;
      try { const j = JSON.parse(await r.text()); if (j && j.productId) answers.set(j.productId, j); } catch { /* not JSON */ }
    });
    await page.goto(url, { waitUntil: "networkidle", timeout: 90000 });
    await page.waitForTimeout(3000);
    const rows = await page.$$eval(".cloud-matrix-product", (els, n) =>
      els.filter((e) => (e.querySelector(".name-cell")?.textContent ?? "").trim() === n).map((e) => ({
        not_available_class: e.classList.contains("not-available"),
        label: (e.querySelector(".label-cell")?.textContent ?? "").replace(/\s+/g, " ").trim(),
        product_key: e.querySelector("ho-price-container[product-key]")?.getAttribute("product-key") ?? null,
        detail_locations: [...e.querySelectorAll("ho-price-container[location]")].map((c) => c.getAttribute("location")),
        listed_locations: [...new Set((e.innerText.match(/\b[A-Z]{3}[0-9]\b/g) ?? []))],
      })), name);
    const row = rows[0] ?? null;
    const answer = row && row.product_key ? answers.get(row.product_key) ?? null : null;
    const prices = answer ? answer.locations.map((l) => ({ datacenter: l.datacenter, active: l.active,
      hourly_eur: l.prices?.hourly?.EUR ?? null, monthly_eur: l.prices?.monthly?.EUR ?? null })) : [];
    record.products.push({ name, url, rows_found: rows.length, row, prices });
    await page.close();
  }
} finally {
  await browser.close();
}
writeFileSync(out, JSON.stringify(record, null, 2) + "\n");
console.log(`read_at ${record.read_at}`);
for (const p of record.products) {
  const r = p.row;
  console.log(`== ${p.name}: rows ${p.rows_found}; key ${r?.product_key ?? "-"}; label "${r?.label ?? ""}"; not-available class ${r?.not_available_class ?? "-"}`);
  console.log(`   the page lists: ${(r?.listed_locations ?? []).join(" ") || "-"}; detail prices for: ${(r?.detail_locations ?? []).join(" ") || "-"}`);
  for (const x of p.prices) console.log(`   ${x.datacenter} active ${x.active} EUR/h ${x.hourly_eur} EUR/month ${x.monthly_eur}`);
  for (const dc of ["FSN1", "NBG1"]) {
    const x = p.prices.find((y) => y.datacenter === dc);
    const v = x ? Number(x.hourly_eur).toFixed(4) : null;
    console.log(`   R1.md ${R1[p.name]} vs ${dc} ${v ?? "absent"}: ${v === null ? "NO FIGURE" : v === R1[p.name] ? "equal" : "CHANGED"}`);
  }
}
