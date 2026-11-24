// The zip's small server (build plan, Part 8): it serves the built game in its own folder to this computer only.
// One file, Node's own modules only, Node 18 or newer. It is a development tool: the page never imports it, and it is
// not part of a production build.
//
//   node serve.mjs          serves the folder this file lies in at http://127.0.0.1:4173
//   node serve.mjs 4174     the same on another port
//
// What it refuses, by design: any method but GET and HEAD; any address but its own in the request's Host header (so
// a web page elsewhere cannot reach it through a name that points here); any path that leaves the folder, also
// through an encoded "..", a backslash or a link; any file type that is not in its table.

import { createReadStream, realpathSync } from 'node:fs';
import { realpath, stat } from 'node:fs/promises';
import { createServer as createHttpServer } from 'node:http';
import path from 'node:path';
import { pipeline } from 'node:stream';
import { fileURLToPath } from 'node:url';

/** The only address the server binds: this computer. */
export const HOST = '127.0.0.1';
/** The port `node serve.mjs` uses when none is given. */
export const DEFAULT_PORT = 4173;

/** The address to open, as the server prints it and as it expects it in the Host header. */
export function addressFor(port) {
  return `${HOST}:${port}`;
}

/** The file types the built game is made of. A file of any other type is not served. */
const CONTENT_TYPES = new Map([
  ['.html', 'text/html; charset=utf-8'],
  ['.js', 'text/javascript; charset=utf-8'],
  ['.css', 'text/css; charset=utf-8'],
  ['.json', 'application/json; charset=utf-8'],
  ['.wasm', 'application/wasm'],
]);

/**
 * The strict content security policy the page is built to run under (ADR 0002 part 7): everything comes from this
 * server's own files, the Wasm may be compiled, nothing is inline, and no other page may frame it. Sent with every
 * answer, because a Worker takes its policy from its own script's response.
 */
export const CONTENT_SECURITY_POLICY =
  "default-src 'none'; script-src 'self' 'wasm-unsafe-eval'; worker-src 'self'; connect-src 'self'; " +
  "style-src 'self'; img-src 'self'; base-uri 'none'; form-action 'none'; frame-ancestors 'none'; object-src 'none'";

const COMMON_HEADERS = {
  'X-Content-Type-Options': 'nosniff',
  'Cache-Control': 'no-store',
  'Cross-Origin-Resource-Policy': 'same-origin',
  'Content-Security-Policy': CONTENT_SECURITY_POLICY,
};

function refuse(response, status, text, extra = {}) {
  const body = `${text}\n`;
  response.writeHead(status, {
    ...COMMON_HEADERS,
    'Content-Type': 'text/plain; charset=utf-8',
    'Content-Length': Buffer.byteLength(body),
    ...extra,
  });
  response.end(response.req.method === 'HEAD' ? undefined : body);
}

/**
 * The file a request path names inside `realRoot`, or `null` when it names none that may be served. The path is
 * decoded once, joined to the folder and resolved through every link; the result must still lie inside the folder.
 */
async function fileFor(realRoot, requestUrl) {
  const rawPath = requestUrl.split(/[?#]/, 1)[0];
  if (!rawPath.startsWith('/')) return { status: 400 };
  let decoded;
  try {
    decoded = decodeURIComponent(rawPath);
  } catch {
    return { status: 400 };
  }
  // None of these is in any name the build writes; on Windows a backslash is a separator and a colon names a drive
  // or a stream.
  if (/[\0\\:]/.test(decoded)) return { status: 400 };
  const relative = decoded.endsWith('/') ? `${decoded}index.html` : decoded;
  const joined = path.join(realRoot, relative);
  const fromRoot = path.relative(realRoot, joined);
  if (fromRoot === '' || fromRoot.startsWith('..') || path.isAbsolute(fromRoot)) return { status: 404 };
  let real;
  let info;
  try {
    real = await realpath(joined);
    info = await stat(real);
  } catch {
    return { status: 404 };
  }
  const realFromRoot = path.relative(realRoot, real);
  if (realFromRoot.startsWith('..') || path.isAbsolute(realFromRoot) || !info.isFile()) return { status: 404 };
  // The type is the real file's, so a link with a served ending cannot hand out a file of another kind.
  const type = CONTENT_TYPES.get(path.extname(real).toLowerCase());
  if (type === undefined) return { status: 404 };
  return { status: 200, file: real, size: info.size, type };
}

/**
 * A server for the files of `root`, not yet listening; `listen` starts it. `root` must exist.
 * @param {{ root: string }} options
 */
export function createServer({ root }) {
  // The system's own resolution, as each request's path gets (on Windows the two can differ for a short name or a
  // mapped drive, and every request would then miss).
  const realRoot = realpathSync.native(root);
  const server = createHttpServer((request, response) => {
    handle(server, realRoot, request, response).catch(() => {
      if (!response.headersSent) refuse(response, 500, 'The server could not answer this request.');
      else response.destroy();
    });
  });
  return server;
}

async function handle(server, realRoot, request, response) {
  if (request.method !== 'GET' && request.method !== 'HEAD') {
    refuse(response, 405, 'This server answers GET and HEAD only.', { Allow: 'GET, HEAD' });
    return;
  }
  const own = addressFor(server.address().port);
  if (request.headers.host !== own) {
    refuse(response, 421, `This server answers only at http://${own} . Open that address.`);
    return;
  }
  // A browser asks for this by itself. The game has no icon yet: "no content" keeps a 404 out of its console.
  if ((request.url ?? '').split(/[?#]/, 1)[0] === '/favicon.ico') {
    response.writeHead(204, COMMON_HEADERS);
    response.end();
    return;
  }
  const found = await fileFor(realRoot, request.url ?? '');
  if (found.status !== 200) {
    refuse(response, found.status, found.status === 400 ? 'That address is not valid.' : 'Not found.');
    return;
  }
  response.writeHead(200, { ...COMMON_HEADERS, 'Content-Type': found.type, 'Content-Length': found.size });
  if (request.method === 'HEAD') {
    response.end();
    return;
  }
  // pipeline closes the file when the answer ends for any reason, a download the client broke off among them.
  pipeline(createReadStream(found.file), response, () => {});
}

/**
 * Starts `server` on this computer's own address. Resolves with the port it got (pass 0 for any free port); rejects
 * with a plain message when the port is taken.
 */
export function listen(server, port) {
  return new Promise((resolve, reject) => {
    server.once('error', (error) => {
      reject(
        error.code === 'EADDRINUSE'
          ? new Error(
              `Port ${port} is already in use on this computer. Close the program that uses it, or start this ` +
                `server on another port, for example: node serve.mjs ${port + 1}`,
            )
          : error,
      );
    });
    server.listen(port, HOST, () => resolve(server.address().port));
  });
}

async function main() {
  const wanted = process.argv[2] === undefined ? DEFAULT_PORT : Number(process.argv[2]);
  if (!Number.isInteger(wanted) || wanted < 1 || wanted > 65535) {
    console.error('The port must be a whole number from 1 to 65535, for example: node serve.mjs 4174');
    process.exit(1);
  }
  const root = path.dirname(fileURLToPath(import.meta.url));
  try {
    const port = await listen(createServer({ root }), wanted);
    console.log(`Shamble Frontlines: serving this folder at http://${addressFor(port)}`);
    console.log('Open that address in your browser. Press Ctrl+C here to stop.');
  } catch (error) {
    console.error(error.message);
    process.exit(1);
  }
}

// Only when this file is the program that was started, never when a test or a tool imports it.
if (process.argv[1] !== undefined && realpathSync(process.argv[1]) === realpathSync(fileURLToPath(import.meta.url))) {
  await main();
}
