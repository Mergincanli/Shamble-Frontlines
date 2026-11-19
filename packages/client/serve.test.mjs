// The small server's rules, each tested on a real server bound to this computer (serve.mjs). Requests are sent with
// Node's http client, not fetch: fetch tidies ".." out of an address before it is sent, and these tests must send it.

import { existsSync, mkdirSync, mkdtempSync, readdirSync, rmSync, symlinkSync, writeFileSync } from 'node:fs';
import { request } from 'node:http';
import { connect } from 'node:net';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { afterAll, beforeAll, describe, expect, test } from 'vitest';
import { addressFor, CONTENT_SECURITY_POLICY, createServer, DEFAULT_PORT, HOST, listen } from './serve.mjs';

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

describe('what it serves', () => {
  test('the page at the folder itself, with the fixed headers', async () => {
    const answer = await send({ path: '/' });
    expect(answer.status).toBe(200);
    expect(answer.body.toString()).toBe('<!doctype html><title>root</title>');
    expect(answer.headers['content-type']).toBe('text/html; charset=utf-8');
    expect(answer.headers['x-content-type-options']).toBe('nosniff');
    expect(answer.headers['cache-control']).toBe('no-store');
    expect(answer.headers['content-security-policy']).toBe(CONTENT_SECURITY_POLICY);
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
      expect(answer.headers['x-content-type-options'], requestPath).toBe('nosniff');
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
});

describe('what it refuses', () => {
  test('every method but GET and HEAD', async () => {
    for (const method of ['POST', 'PUT', 'DELETE', 'OPTIONS', 'PATCH']) {
      const answer = await send({ method, path: '/' });
      expect(answer.status, method).toBe(405);
      expect(answer.headers.allow, method).toBe('GET, HEAD');
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
      expect(answer.headers['x-content-type-options'], attempt).toBe('nosniff');
    }
  });

  test('a link inside the folder that leads outside it', async () => {
    if (!linkMade) return;
    const answer = await send({ path: '/leak.js' });
    expect(answer.status).toBe(404);
    expect(answer.body.toString()).not.toContain(SECRET);
  });

  test('a file type that is not in the table, its own source among them', async () => {
    for (const requestPath of ['/notes.md', '/serve.mjs']) {
      expect((await send({ path: requestPath })).status, requestPath).toBe(404);
    }
  });

  test('a link with a served ending to a file of another type', async () => {
    if (!linkMade) return;
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
      expect((await send({ path: requestPath })).status, requestPath).toBe(400);
    }
  });

  test('a Host header that is not its own address', async () => {
    for (const host of ['evil.example', `evil.example:${port}`, `localhost:${port}`, HOST, `${HOST}:${port + 1}`]) {
      const answer = await send({ path: '/', host });
      expect(answer.status, host).toBe(421);
      expect(answer.body.toString(), host).toContain(`http://${addressFor(port)}`);
      expect(answer.body.toString(), host).not.toContain('<title>');
    }
    expect((await send({ path: '/', host: addressFor(port) })).status).toBe(200);
  });

  test('a request that names no host at all', async () => {
    // Written by hand on a plain connection: Node's http client always adds a Host header.
    const answer = await new Promise((resolve, reject) => {
      const socket = connect(port, HOST, () => socket.write('GET / HTTP/1.0\r\n\r\n'));
      const chunks = [];
      socket.on('data', (chunk) => chunks.push(chunk));
      socket.on('end', () => resolve(Buffer.concat(chunks).toString()));
      socket.on('error', reject);
    });
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
