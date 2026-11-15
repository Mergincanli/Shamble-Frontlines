// LOCAL PROOF ONLY (never shipped to the test repository): a tiny static server that applies a build's _headers the way
// Cloudflare's headers page (2026-11-12) describes it, so the check scripts can be shown to pass the good modes and
// fail the planted ones. It proves the scripts, not Cloudflare: the rule engine below is this chat's reading of the
// page. Requests carry the host they stand for in x-test-host (set by fetch-shim.mjs).
//   rules: a line not starting with whitespace is a URL pattern (https://host/path, or /path); indented lines are
//   "Name: value" or "! Name"; all matching rules apply in file order; a header set twice is joined with ", ";
//   "! Name" removes what earlier rules added; ":name" matches all but "." in a host and all but "/" in a path; "*"
//   matches anything. Rules override the defaults. emulateHostNoindex adds X-Robots-Tag: noindex on every link with a
//   label before the project (Pages: "if requesting the asset over a preview URL"), unless a rule set it.
//   Like Pages (serving-pages page, 2026-11-11), a path ending in .html answers a 308 to its extension-less form, and an
//   extension-less path serves its .html file (the review round's finding F1/TE-1).
import { createServer } from 'node:http';
import { existsSync, readFileSync, statSync } from 'node:fs';
import { extname, join } from 'node:path';

const TYPES = { '.html': 'text/html; charset=utf-8', '.js': 'text/javascript; charset=utf-8', '.css': 'text/css; charset=utf-8',
  '.json': 'application/json', '.wasm': 'application/wasm', '.bin': 'application/octet-stream' };

function toRegExp(s, delim) {
  let re = '';
  for (let i = 0; i < s.length;) {
    if (s[i] === '*') { re += '(.*)'; i += 1; continue; }
    const ph = /^:[A-Za-z_][A-Za-z0-9_]*/.exec(s.slice(i));
    if (ph) { re += delim === '.' ? '([^.]+)' : '([^/]+)'; i += ph[0].length; continue; }
    re += s[i].replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    i += 1;
  }
  return new RegExp(`^${re}$`);
}

export function parseHeaders(text) {
  const rules = [];
  let current = null;
  for (const raw of text.split('\n')) {
    if (!raw.trim() || raw.trim().startsWith('#')) continue;
    if (!/^\s/.test(raw)) {
      const m = /^https:\/\/([^/]+)(\/.*)$/.exec(raw.trim());
      current = { pattern: raw.trim(), hostRe: m ? toRegExp(m[1], '.') : null, pathRe: toRegExp(m ? m[2] : raw.trim(), '/'), ops: [] };
      rules.push(current);
    } else if (current) {
      const line = raw.trim();
      if (line.startsWith('! ')) current.ops.push({ detach: true, name: line.slice(2).trim() });
      else { const i = line.indexOf(':'); current.ops.push({ name: line.slice(0, i).trim(), value: line.slice(i + 1).trim() }); }
    }
  }
  return rules;
}

export function applyRules(rules, host, path) {
  const out = new Map();
  for (const r of rules) {
    if (r.hostRe && !r.hostRe.test(host)) continue;
    if (!r.pathRe.test(path)) continue;
    for (const op of r.ops) {
      const k = op.name.toLowerCase();
      if (op.detach) out.delete(k);
      else { const prev = out.get(k); out.set(k, { name: op.name, value: prev ? `${prev.value}, ${op.value}` : op.value }); }
    }
  }
  return out;
}

// routes: { [host]: { kind: 'build', dir } | { kind: '404' | 'tombstone' | 'wall' | 'block' | 'error' | '429' | 'challenge' | '401' } }
export function startServer(routes, { emulateHostNoindex = false, projectHost = 'shamble-preview-check.pages.dev' } = {}) {
  const cache = new Map();
  const server = createServer((req, res) => {
    const host = req.headers['x-test-host'] || req.headers.host;
    const path = new URL(req.url, 'http://local').pathname;
    const route = routes[host];
    if (!route || route.kind === '404') { res.writeHead(404, { 'content-type': 'text/plain' }); res.end('not found'); return; }
    if (route.kind === 'wall') { res.writeHead(302, { location: 'https://test-team.cloudflareaccess.com/cdn-cgi/access/login/x' }); res.end(); return; }
    if (route.kind === 'block') { res.writeHead(403, { 'content-type': 'text/html' }); res.end('<html>error code: 1010</html>'); return; }
    if (route.kind === 'error') { res.writeHead(503, { 'content-type': 'text/plain' }); res.end('unavailable'); return; }
    if (route.kind === '429') { res.writeHead(429, { 'content-type': 'text/plain' }); res.end('slow down'); return; }
    if (route.kind === 'challenge') { res.writeHead(403, { 'content-type': 'text/html', 'cf-mitigated': 'challenge' }); res.end('<html>Just a moment</html>'); return; }
    if (route.kind === '401') { res.writeHead(401, { 'content-type': 'text/plain' }); res.end('who are you'); return; }
    if (path.endsWith('.html')) {
      const to = path.endsWith('/index.html') ? path.slice(0, -'index.html'.length) : path.slice(0, -'.html'.length);
      res.writeHead(308, { location: to }); res.end(); return;
    }
    if (route.kind === 'tombstone') { res.writeHead(200, { 'content-type': 'text/html' }); res.end('<!doctype html><p>This preview is gone.</p>'); return; }
    if (!cache.has(route.dir)) cache.set(route.dir, parseHeaders(readFileSync(join(route.dir, '_headers'), 'utf8')));
    const rules = cache.get(route.dir);
    let file = path === '/' ? 'index.html' : path.slice(1);
    if (!extname(file) && existsSync(join(route.dir, `${file}.html`))) file = `${file}.html`;
    let status = 200;
    if (file === '_headers' || !existsSync(join(route.dir, file)) || statSync(join(route.dir, file)).isDirectory()) { file = '404.html'; status = 404; }
    const headers = { 'content-type': TYPES[extname(file)] || 'application/octet-stream' };
    if (emulateHostNoindex && host !== projectHost && host.endsWith(`.${projectHost}`)) headers['x-robots-tag'] = 'noindex';
    for (const { name, value } of applyRules(rules, host, path).values()) headers[name.toLowerCase()] = value;
    res.writeHead(status, headers);
    res.end(readFileSync(join(route.dir, file)));
  });
  return new Promise((resolve) => server.listen(0, '127.0.0.1', () => resolve(server)));
}
