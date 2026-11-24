// The small server's rules, each tested on a real server bound to this computer (serve.mjs). Requests are sent with
// Node's http client, not fetch: fetch tidies ".." out of an address before it is sent, and these tests must send it.

import { existsSync, mkdirSync, mkdtempSync, readdirSync, rmSync, symlinkSync, writeFileSync } from 'node:fs';
import { realpath } from 'node:fs/promises';
import { request } from 'node:http';
import { connect } from 'node:net';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { afterAll, beforeAll, describe, expect, test, vi } from 'vitest';
import { addressFor, CONTENT_SECURITY_POLICY, createServer, DEFAULT_PORT, HOST, listen } from './serve.mjs';

// The server's question to the file system is watched, never changed: two refusals must come before it is asked.
vi.mock('node:fs/promises', async (importOriginal) => {
  const real = await importOriginal();
  return { ...real, realpath: vi.fn(real.realpath) };
});

const SECRET = 'outside the served folder';
let base;
let root;
let server;
let port;
let linkMade = true;

beforeAll(async () => {
  base = mkdtempSync(path.join(tmpdir(), 'shamble-serve-'));
  root = path.join(base, 'served');
  mkdirSync(path.join(root, 'sub'), { recursive: true });
  writeFileSync(path.join(root, 'index.html'), '<!doctype html><title>root</title>');
  writeFileSync(path.join(root, 'app.js'), 'export const a = 1;\n');
  writeFileSync(path.join(root, 'style.css'), 'body { margin: 0; }\n');
  writeFileSync(path.join(root, 'sim.wasm'), Buffer.from([0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00]));
  writeFileSync(path.join(root, 'sub', 'index.html'), '<!doctype html><title>sub</title>');
  writeFileSync(path.join(root, 'notes.md'), 'a type that is not in the table\n');
  writeFileSync(path.join(root, 'serve.mjs'), '// the server never serves its own source\n');
  writeFileSync(path.join(root, 'big.js'), Buffer.alloc(8 * 1024 * 1024, '/'));
  mkdirSync(path.join(root, 'folder.js'));
  writeFileSync(path.join(base, 'outside.js'), SECRET);
  try {
    symlinkSync(path.join(base, 'outside.js'), path.join(root, 'leak.js'));
    symlinkSync(path.join(root, 'serve.mjs'), path.join(root, 'alias.js'));
  } catch {
    linkMade = false; // a system that does not let this user make links
  }
  server = createServer({ root });
  port = await listen(server, 0);
});

afterAll(async () => {
  await new Promise((resolve) => server.close(resolve));
  rmSync(base, { recursive: true, force: true });
});

/** One request, sent exactly as given. */
function send({ method = 'GET', path: requestPath = '/', host = addressFor(port), toPort = port } = {}) {
  return new Promise((resolve, reject) => {
    const outgoing = request(
      { host: HOST, port: toPort, method, path: requestPath, headers: { Host: host } },
      (incoming) => {
        const chunks = [];
        incoming.on('data', (chunk) => chunks.push(chunk));
        incoming.on('end', () =>
          resolve({ status: incoming.statusCode, headers: incoming.headers, body: Buffer.concat(chunks) }),
        );
      },
    );
    outgoing.on('error', reject);
    outgoing.end();
  });
}

/** The four headers every answer carries, a refusal too: a Worker takes its policy from its own script's response. */
function expectFixedHeaders(answer, what) {
  expect(answer.headers['x-content-type-options'], what).toBe('nosniff');
  expect(answer.headers['cache-control'], what).toBe('no-store');
  expect(answer.headers['cross-origin-resource-policy'], what).toBe('same-origin');
  expect(answer.headers['content-security-policy'], what).toBe(CONTENT_SECURITY_POLICY);
}

/** One request written by hand on a plain connection: what Node's http client would never send. */
function sendRaw(text) {
  return new Promise((resolve, reject) => {
    const socket = connect(port, HOST, () => socket.write(text));
    const chunks = [];
    socket.on('data', (chunk) => chunks.push(chunk));
    socket.on('end', () => resolve(Buffer.concat(chunks).toString()));
    socket.on('error', reject);
  });
}

describe('what it serves', () => {
  test('the page at the folder itself, with the fixed headers', async () => {
    const answer = await send({ path: '/' });
    expect(answer.status).toBe(200);
    expect(answer.body.toString()).toBe('<!doctype html><title>root</title>');
    expect(answer.headers['content-type']).toBe('text/html; charset=utf-8');
    expectFixedHeaders(answer, '/');
    expect(CONTENT_SECURITY_POLICY).toContain("default-src 'none'");
    expect(CONTENT_SECURITY_POLICY).not.toMatch(/'unsafe-inline'|'unsafe-eval'|blob:|data:/);
    expect(answer.headers['content-length']).toBe(String(answer.body.length));
  });

  test('each type of the table under its own content type', async () => {
    const types = {
      '/index.html': 'text/html; charset=utf-8',
      '/app.js': 'text/javascript; charset=utf-8',
      '/style.css': 'text/css; charset=utf-8',
      '/sim.wasm': 'application/wasm',
      '/sub/': 'text/html; charset=utf-8',
    };
    for (const [requestPath, type] of Object.entries(types)) {
      const answer = await send({ path: requestPath });
      expect(answer.status, requestPath).toBe(200);
      expect(answer.headers['content-type'], requestPath).toBe(type);
      expectFixedHeaders(answer, requestPath);
    }
    expect((await send({ path: '/sub/' })).body.toString()).toContain('<title>sub</title>');
    expect((await send({ path: '/sim.wasm' })).body).toEqual(Buffer.from([0, 0x61, 0x73, 0x6d, 1, 0, 0, 0]));
  });

  test('a query does not change which file is meant', async () => {
    const answer = await send({ path: '/index.html?backend=webgl' });
    expect(answer.status).toBe(200);
    expect(answer.body.toString()).toContain('<title>root</title>');
  });

  test('HEAD gives the headers and no body', async () => {
    const answer = await send({ method: 'HEAD', path: '/app.js' });
    expect(answer.status).toBe(200);
    expect(answer.headers['content-type']).toBe('text/javascript; charset=utf-8');
    expect(answer.headers['content-length']).toBe(String('export const a = 1;\n'.length));
    expect(answer.body.length).toBe(0);
  });

  test("the browser's own request for an icon gets \"no content\", not an error", async () => {
    for (const requestPath of ['/favicon.ico', '/favicon.ico?v=2']) {
      for (const method of ['GET', 'HEAD']) {
        const answer = await send({ method, path: requestPath });
        expect(answer.status, `${method} ${requestPath}`).toBe(204);
        expect(answer.body.length, `${method} ${requestPath}`).toBe(0);
        expectFixedHeaders(answer, `${method} ${requestPath}`);
      }
    }
    // Only that one address: an icon elsewhere is a file type the table does not have.
    expect((await send({ path: '/sub/favicon.ico' })).status).toBe(404);
  });
});

describe('what it refuses', () => {
  test('every method but GET and HEAD', async () => {
    for (const method of ['POST', 'PUT', 'DELETE', 'OPTIONS', 'PATCH']) {
      const answer = await send({ method, path: '/' });
      expect(answer.status, method).toBe(405);
      expect(answer.headers.allow, method).toBe('GET, HEAD');
      expectFixedHeaders(answer, method);
    }
  });

  test('a path that leaves the folder, however it is written', async () => {
    const attempts = [
      '/../outside.js',
      '/sub/../../outside.js',
      '/%2e%2e/outside.js',
      '/%2E%2E/outside.js',
      '/sub/%2e%2e/%2e%2e/outside.js',
      '/..%2foutside.js',
      '/sub/..%2f..%2foutside.js',
      '/..%5coutside.js',
      '/..\\outside.js',
      '/%252e%252e/outside.js',
      '//outside.js',
      `/${path.join(base, 'outside.js')}`,
    ];
    for (const attempt of attempts) {
      const answer = await send({ path: attempt });
      expect([400, 404], attempt).toContain(answer.status);
      expect(answer.body.toString(), attempt).not.toContain(SECRET);
      expectFixedHeaders(answer, attempt);
    }
  });

  test('a path that leaves the folder, or names the folder itself, before the file system is asked', async () => {
    // Each of these is also refused later, by the check of the real path: these two checks keep the server from
    // asking the file system about a place outside its folder at all.
    for (const requestPath of ['/../outside.js', '/sub/../../outside.js', '/../missing.js', '/.', '/sub/..']) {
      vi.mocked(realpath).mockClear();
      const answer = await send({ path: requestPath });
      expect(answer.status, requestPath).toBe(404);
      expect(realpath, requestPath).not.toHaveBeenCalled();
    }
    // The control: for a path inside the folder the file system is asked, once.
    vi.mocked(realpath).mockClear();
    expect((await send({ path: '/app.js' })).status).toBe(200);
    expect(realpath).toHaveBeenCalledTimes(1);
  });

  test('an address that does not start with a slash', async () => {
    // "*" is the one such address Node's own parser lets through (the form of an OPTIONS request).
    const answer = await sendRaw(`GET * HTTP/1.1\r\nHost: ${addressFor(port)}\r\nConnection: close\r\n\r\n`);
    expect(answer.split('\r\n', 1)[0]).toContain(' 400 ');
    expect(answer).toContain('That address is not valid.');
    expect(answer.toLowerCase()).toContain(`content-security-policy: ${CONTENT_SECURITY_POLICY}`.toLowerCase());
  });

  test('a folder whose name has a served ending', async () => {
    // Only the check that the real path is a plain file refuses it: its name passes the table of types.
    for (const requestPath of ['/folder.js', '/folder.js?x=1']) {
      const answer = await send({ path: requestPath });
      expect(answer.status, requestPath).toBe(404);
      expectFixedHeaders(answer, requestPath);
    }
  });

  test('a link inside the folder that leads outside it', async ({ skip }) => {
    skip(!linkMade, 'this system does not let this user make links');
    const answer = await send({ path: '/leak.js' });
    expect(answer.status).toBe(404);
    expect(answer.body.toString()).not.toContain(SECRET);
    expectFixedHeaders(answer, '/leak.js');
  });

  test('a file type that is not in the table, its own source among them', async () => {
    for (const requestPath of ['/notes.md', '/serve.mjs']) {
      const answer = await send({ path: requestPath });
      expect(answer.status, requestPath).toBe(404);
      expectFixedHeaders(answer, requestPath);
    }
  });

  test('a link with a served ending to a file of another type', async ({ skip }) => {
    skip(!linkMade, 'this system does not let this user make links');
    const answer = await send({ path: '/alias.js' });
    expect(answer.status).toBe(404);
    expect(answer.body.toString()).not.toContain('never serves its own source');
  });

  test('a download that is broken off leaves no file open', async () => {
    if (!existsSync('/proc/self/fd')) return; // the count of open files is read where the system shows it
    const open = () => readdirSync('/proc/self/fd').length;
    const before = open();
    for (let i = 0; i < 20; i += 1) {
      await new Promise((resolve) => {
        const outgoing = request({ host: HOST, port, path: '/big.js', headers: { Host: addressFor(port) } }, (incoming) => {
          incoming.once('data', () => incoming.destroy());
          incoming.once('close', resolve);
        });
        outgoing.end();
      });
    }
    await new Promise((resolve) => setTimeout(resolve, 200));
    expect(open() - before).toBeLessThan(5);
  });

  test('a folder without its closing slash, and a file that is not there', async () => {
    expect((await send({ path: '/sub' })).status).toBe(404);
    expect((await send({ path: '/missing.js' })).status).toBe(404);
    expect((await send({ path: '/sub/missing/' })).status).toBe(404);
  });

  test('an address that cannot be decoded, or holds a zero byte, a backslash or a colon', async () => {
    for (const requestPath of ['/%E0%A4%A', '/%', '/a%00.js', '/app.js%00', '/a%5Cb.js', '/c%3A/app.js', '/app.js::$DATA']) {
      const answer = await send({ path: requestPath });
      expect(answer.status, requestPath).toBe(400);
      expectFixedHeaders(answer, requestPath);
    }
  });

  test('a Host header that is not its own address', async () => {
    for (const host of ['evil.example', `evil.example:${port}`, `localhost:${port}`, HOST, `${HOST}:${port + 1}`]) {
      const answer = await send({ path: '/', host });
      expect(answer.status, host).toBe(421);
      expect(answer.body.toString(), host).toContain(`http://${addressFor(port)}`);
      expect(answer.body.toString(), host).not.toContain('<title>');
      expectFixedHeaders(answer, host);
    }
    expect((await send({ path: '/', host: addressFor(port) })).status).toBe(200);
  });

  test('a request that names no host at all', async () => {
    // Written by hand on a plain connection: Node's http client always adds a Host header.
    const answer = await sendRaw('GET / HTTP/1.0\r\n\r\n');
    expect(answer.split('\r\n', 1)[0]).toContain(' 421 ');
    expect(answer).not.toContain('<title>');
  });
});

describe('where it listens', () => {
  test('on this computer only, at the address it names', () => {
    expect(server.address().address).toBe('127.0.0.1');
    expect(HOST).toBe('127.0.0.1');
    expect(DEFAULT_PORT).toBe(4173);
    expect(addressFor(DEFAULT_PORT)).toBe('127.0.0.1:4173');
  });

  test('a port that is taken stops it with a plain message', async () => {
    const second = createServer({ root });
    await expect(listen(second, port)).rejects.toThrow(`Port ${port} is already in use on this computer`);
    expect(second.listening).toBe(false);
  });
});
