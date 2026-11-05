// kit-v3-3 step 6 evidence: the new test's strings (copied from checkpoint.test.mjs) against the SECRETS of each given
// checkpoint.mjs copy. Prints counts and the index and kind of each miss; never prints a key.
//   node step6-cases.mjs <label>=<checkpoint.mjs copy>...
import { readFileSync } from 'node:fs';
const load = (file) => {
  const src = readFileSync(file, 'utf8');
  const from = src.search(/^const (ESC|SECRETS) = /m), to = src.indexOf('\n];', src.indexOf('const SECRETS = '));
  return new Function(src.slice(from, to + 3) + '\nreturn SECRETS;')();
};
const SK = 'sk' + '-' + 'K'.repeat(40), AWS = 'AK' + 'IA' + 'Q'.repeat(16), ASIA = 'AS' + 'IA' + 'Q'.repeat(16);
const SLACK = 'xo' + 'xb-' + 'S'.repeat(20);
const refused = [
  [`q=%20${SK}`, 'API key'], [`a%2C${SK}`, 'API key'], [`a%2c${SK}`, 'API key'], [`k%3A${SK}`, 'API key'],
  [`v=%22${SK}`, 'API key'], [`a%26${SK}`, 'API key'], [`p%2F${SK}`, 'API key'], [`l%0A${SK}`, 'API key'],
  [`k%3D${SK}`, 'API key'], [`k%253A${SK}`, 'API key'], [`a%252C${SK}`, 'API key'], [`a%u002C${SK}`, 'API key'],
  [`"line\\n${SK}"`, 'API key'], [`"\\u0022${SK}`, 'API key'], [`"a\\b${SK}"`, 'API key'], [`"a\\f${SK}"`, 'API key'],
  [`a\\v${SK}`, 'API key'], [`a\\0${SK}`, 'API key'], [`a\\x2c${SK}`, 'API key'], [`a=2C${SK}`, 'API key'],
  [`a%2C${AWS}`, 'AWS key'], [`k%3D${AWS}`, 'AWS key'], [`l%0A${ASIA}`, 'AWS key'], [`k=3D${AWS}`, 'AWS key'],
  [`x%20${SLACK}`, 'Slack token'], [`k%3D${SLACK}`, 'Slack token'],
];
const clean = ['risk-' + 'r'.repeat(40), '%20risk-' + 'r'.repeat(40),
  '%2Ctask-legacy-inventory-and-tuning-tables-for-v2-research', '\\ndisk-' + 'd'.repeat(40),
  '%2C' + 'sk' + '-' + 'K'.repeat(31), '%2C' + 'AK' + 'IA' + 'Q'.repeat(15), 'X' + 'AK' + 'IA' + 'Q'.repeat(16),
  'my' + 'AK' + 'IA' + 'Q'.repeat(16), 'foo' + 'xo' + 'xb-' + 'S'.repeat(20), '%ZZ' + 'sk' + '-' + 'K'.repeat(40),
  'https%3A%2F%2Fexample.invalid%2Fpath%3Fa%3D1', '100%25 done'];
for (const arg of process.argv.slice(2)) {
  const [label, file] = arg.split('='), S = load(file);
  const kinds = (t) => S.filter(([, re]) => re.test(t)).map(([k]) => k);
  const missed = refused.map(([t, k], i) => [i, k, kinds(t).includes(k)]).filter(([, , ok]) => !ok);
  const fp = clean.map((t, i) => [i, kinds(t)]).filter(([, k]) => k.length);
  console.log(`[${label}] refused caught ${refused.length - missed.length}/${refused.length}; clean with a hit ${fp.length}/${clean.length}`);
  for (const [i, k] of missed) console.log(`  missed refused #${i} (${k})`);
  for (const [i, k] of fp) console.log(`  hit on clean #${i} (${k.join(', ')})`);
}
