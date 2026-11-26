// The few Node names the browser specs use, declared by hand so that the specs can be type-checked (the toolchain
// job's last type-check line) without Node's own type package, which would be two new packages from the internet.
// A spec that uses another Node name fails that check until this file names it: loud, never silent. That one check
// runs without the lib check, because Playwright's and Vite's own type files need Node's types.
declare module "node:fs" {
  export function readFileSync(path: string | URL, encoding: "utf8"): string;
  export function readFileSync(path: string | URL): Uint8Array;
  export function readdirSync(path: string | URL): string[];
}
declare module "node:path" {
  const path: { join(...parts: string[]): string };
  export default path;
}
declare module "node:url" {
  export function pathToFileURL(path: string): URL;
  export function fileURLToPath(url: string | URL): string;
}
declare const process: { readonly env: Record<string, string | undefined> };
