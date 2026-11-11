// Preload: make the optional packages unloadable, as with npm ci --omit=optional.
const Module = require('module');
const orig = Module._load;
Module._load = function (req, ...rest) {
  if (/^(playwright|playwright-core|gl|@napi-rs\/canvas)(\/|$)/.test(req)) {
    process.stderr.write('BLOCKED optional require: ' + req + '\n');
    const e = new Error("Cannot find module '" + req + "'"); e.code = 'MODULE_NOT_FOUND'; throw e;
  }
  return orig.call(this, req, ...rest);
};
