// Feed real numstat (M^1..M) of merged PRs into the copy's classify/pick, and the words check on DECISIONS (M^1 -> M).
import { execFileSync } from 'node:child_process';
const R = '/home/user/Shamble-Frontlines';
const { classify, wordsCheck } = await import(process.argv[2]);
const git = (a) => execFileSync('git', a, { cwd: R, encoding: 'utf8', maxBuffer: 1 << 28 });
for (const [pr, m] of [['#65', '28254a73'], ['#68', '3ab9eaec'], ['#70', '2483e9a7'], ['#75', 'b9aad019'], ['#76', '2671595f'], ['#66', '09faefa7'], ['#67', 'c1da1bd2']]) {
  const files = git(['diff', '--numstat', '--no-renames', `${m}^1`, m]).trim().split('\n').map((l) => { const [a, d, path] = l.split('\t'); return { path, lines: a === '-' ? 1 : +a + +d, binary: a === '-' }; });
  const c = classify(files);
  let words = '-';
  if (files.some((f) => f.path === 'docs/DECISIONS.md')) {
    const read = (cm, p) => { try { return git(['show', `${cm}:${p}`]); } catch { return null; } };
    words = JSON.stringify(wordsCheck(git(['show', `${m}:docs/DECISIONS.md`]), git(['show', `${m}^1:docs/DECISIONS.md`]), read));
  }
  console.log(pr, c.track, JSON.stringify(c.reviewers.score ?? {}), c.reviewers.picks.join(','), 'floors:', c.goFloors.length, 'words:', words);
}
