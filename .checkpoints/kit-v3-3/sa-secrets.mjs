// security-auditor: secret-pattern edge cases. Prints booleans only; keys are synthetic, built at run time.
import { readFileSync } from 'node:fs';
const src = readFileSync(process.argv[2], 'utf8');
const esc = /const ESC = (String\.raw`[^`]*`);/.exec(src)[1];
const ESC = eval(esc);
const API = new RegExp(String.raw`(?:^|[^A-Za-z0-9-]|${ESC})sk-(proj-|svcacct-)?[A-Za-z0-9_-]{32,}`);
const AWS = new RegExp(String.raw`(?:^|[^A-Za-z0-9_]|${ESC})(AKIA|ASIA)[0-9A-Z]{16}\b`);
const body = 'Q'.repeat(20) + '9'.repeat(20);
const k = ['s', 'k', '-'].join('') + 'proj-' + body;
const a = ['AK', 'IA'].join('') + 'Z'.repeat(16);
const cases = [
  ['space', ' ' + k], ['%20', 'q=%20' + k], ['%2520', '%2520' + k], ['%u0020', '%u0020' + k], ['\\u0020', '\\u0020' + k],
  ['\\n', 'a\\n' + k], ['=2C QP', 'x=2C' + k], ['=2c lower QP', 'x=2c' + k], ['hyphen before', 'key-' + k],
  ['sk%2D inner', 'q=sk%2Dproj-' + body], ['html &#32;', '&#32;' + k], ['\\u{20}', '\\u{20}' + k],
  ['aws %2C', '%2C' + a], ['aws _ before', 'x_' + a], ['aws after _', ' ' + a + '_x'], ['aws \\t', '\\t' + a],
];
for (const [n, s] of cases) console.log(n.padEnd(14), 'API', API.test(s), 'AWS', AWS.test(s));
const fp = [['risk- word', 'the risk-' + 'assessment'.repeat(4)], ['C:\\tsk path', 'C:\\tsk-' + body], ['=DE then sk', 'M=DE' + k.slice(0)]];
for (const [n, s] of fp) console.log('FP?', n.padEnd(14), API.test(s));
