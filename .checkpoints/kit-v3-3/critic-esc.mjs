// Critic scratch (A3): the plan's ESC fragment (A) and pctDecode (B) on extra cases. Synthetic tokens built at run time;
// prints only case labels and true/false, never a token.
const ESC = String.raw`%(?:25)*[0-9A-Fa-f]{2}|\\(?:[nrt]|x[0-9A-Fa-f]{2}|u[0-9A-Fa-f]{4})`;
const API = new RegExp(String.raw`(?:^|[^A-Za-z0-9-]|${ESC})sk-(proj-|svcacct-)?[A-Za-z0-9_-]{32,}`);
const AWS = new RegExp(String.raw`(?:^|[^A-Za-z0-9_]|${ESC})(AKIA|ASIA)[0-9A-Z]{16}\b`);
const URLC = /[a-z][a-z0-9+.-]*:\/\/[^\s/:@]+:[^\s/@]+@/i;
const dec = (s) => { for (let i = 0; i < 3; i++) { const t = s.replace(/%([0-9A-Fa-f]{2})/g, (_, h) => String.fromCharCode(parseInt(h, 16))); if (t === s) break; s = t; } return s; };
const sk = 's' + 'k-' + 'K'.repeat(40), ak = 'AK' + 'IA' + 'Q'.repeat(16);
const hit = (re, s) => re.test(s) || re.test(dec(s));
const cases = [
  ['FN? \\f before sk', '"a\\f' + sk + '"', API], ['FN? \\b before sk', '"a\\b' + sk + '"', API],
  ['FN? \\0 before sk', 'a\\0' + sk, API], ['FN? \\v before sk', 'a\\v' + sk, API],
  ['FN? QP =2C before sk', 'a=2C' + sk, API], ['FN? QP =3D before AWS', 'k=3D' + ak, AWS],
  ['FN? %u002C before sk', 'a%u002C' + sk, API], ['FN? \\x2c lower', 'a\\x2c' + sk, API],
  ['FN? %2C%2C double', 'a%2C%2C' + sk, API], ['FN? URL creds \\u0040', 'https://u' + ':' + 'p'.repeat(8) + '\\u0040example.invalid', URLC],
  ['FN? URL creds %40 only', 'https://u' + ':' + 'p'.repeat(8) + '%40example.invalid', URLC],
  ['FP? Windows \\tsk- path', 'C:\\temp\\tsk-' + 'a'.repeat(34), API],
  ['FP? %2F slug (old pattern already fires on /sk-slug)', 'https%3A%2F%2Fexample.invalid%2Fsk-' + 'learn-'.repeat(6), API],
  ['FP? decoded mailto-ish', 'next=mailto%3Auser%40example.invalid', URLC],
  ['FP? decoded ssh git url', 'u=ssh%3A%2F%2Fgit%40github.com%2Fx', URLC],
];
for (const [label, s, re] of cases) console.log(label.padEnd(52), hit(re, s));
