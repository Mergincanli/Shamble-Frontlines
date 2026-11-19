// Scratch (test-engineer, CI-WASM followups, block 4): the few Node names the browser specs use, declared by hand,
// to see what a type check of the specs finds once the missing @types/node is out of the way. Not for the repository
// as is: block 4 says which option the pull request takes.
declare module "node:fs" {
  export function readFileSync(path: string | URL, encoding: "utf8"): string;
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
