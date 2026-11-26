// serve.mjs: T5's page server on http://127.0.0.1 (a secure context; u7-brief.md rev 1.1 section 3's browser line;
// advisor-u6c-brief.md C1 (e)). It matches the request target's path, up to the first `?`, exactly against a fixed
// table and never reads or keeps the query; it serves each room's hash file `/npt/cert-hash-NN.json` from the file the
// caller names for that room; every answer carries plan.md:1123-1126's CSP word for word with https://127.0.0.1:* and
// wss://127.0.0.1:* added to connect-src, `nosniff` and `no-referrer`; GET and HEAD only.
import { existsSync, readFileSync } from "node:fs";
import { createServer } from "node:http";
import { extname } from "node:path";

export const CSP = "default-src 'none'; script-src 'self' 'wasm-unsafe-eval'; worker-src 'self'; connect-src 'self' "
  + "https://rs.nt.shamblefrontlines.com:* wss://rs.nt.shamblefrontlines.com:* https://rs-b.nt.shamblefrontlines.com:* "
  + "wss://rs-b.nt.shamblefrontlines.com:* https://127.0.0.1:* wss://127.0.0.1:*; style-src 'self'; img-src 'self'; "
  + "base-uri 'none'; form-action 'none'; frame-ancestors 'none'";
const TYPES = {
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".wasm": "application/wasm",
  ".json": "application/json",
};
const HASH_PATH = /^\/npt\/cert-hash-(0[0-9]|[12][0-9]|3[01])\.json$/;

/**
 * `files`: request path to a file on disk; `hash(nn)`: that room's hash file on disk, or null. Resolves to
 * { port, served, close }, `served` listing [path, status] per answer (paths only).
 */
export function serve({ files, hash }) {
  const served = [];
  const server = createServer((req, res) => {
    const path = (req.url ?? "").split("?")[0];
    const head = { "Content-Security-Policy": CSP, "X-Content-Type-Options": "nosniff", "Referrer-Policy": "no-referrer" };
    const answer = (status, extra = {}, body = undefined) => {
      served.push([path, status]);
      res.writeHead(status, { ...head, ...extra }).end(body);
    };
    if (req.method !== "GET" && req.method !== "HEAD") return answer(405);
    const room = HASH_PATH.exec(path);
    const file = room ? hash(room[1]) : Object.hasOwn(files, path) ? files[path] : null;
    if (file === null || !existsSync(file)) return answer(404);
    const type = TYPES[extname(file)] ?? "application/octet-stream";
    const body = readFileSync(file);
    return answer(200, { "Content-Type": type, "Cache-Control": "no-store" }, req.method === "HEAD" ? undefined : body);
  });
  return new Promise((ok, fail) => {
    server.once("error", fail);
    server.listen(0, "127.0.0.1", () => ok({
      port: server.address().port,
      served,
      close: () => new Promise((done) => {
        server.closeAllConnections();
        server.close(done);
      }),
    }));
  });
}
