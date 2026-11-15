// LOCAL PROOF ONLY: preloaded with `node --import <this file>` so the unchanged check scripts reach the local server.
// A request for https://<host>.pages.dev/<path> goes to http://127.0.0.1:<LOCAL_PROOF_PORT>/<path> and names its host in
// x-test-host; the scripts' own headers (the named User-Agent) are kept.
const port = process.env.LOCAL_PROOF_PORT;
const realFetch = globalThis.fetch;
globalThis.fetch = (input, init = {}) => {
  const url = new URL(typeof input === 'string' ? input : input.url);
  if (!port || !url.host.endsWith('.pages.dev')) return realFetch(input, init);
  const headers = new Headers(init.headers || {});
  headers.set('x-test-host', url.host);
  return realFetch(`http://127.0.0.1:${port}${url.pathname}${url.search}`, { ...init, headers });
};
