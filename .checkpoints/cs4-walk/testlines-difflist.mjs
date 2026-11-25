// Prints the list Mustafa confirms under question DIFF-LIST (his point 8c), as Markdown, from the saved Export and
// from the comparison's own output: no value is typed by hand. From the worktree's root:
//   node .checkpoints/cs4-walk/testlines-difflist.mjs            writes testlines-difflist.md
//   node .checkpoints/cs4-walk/testlines-difflist.mjs --into     also replaces the block between the two
//                                                                "difflist" marker lines in plan.md
// Three parts. A: the names where his Export differs from docs/legacy-tuning.json (read from testlines-compare.txt,
// which testlines-compare.mjs wrote; this script refuses a comparison made on another Export). B: the Export's own
// "changed_from_start" list, group by group, for the groups the game takes. C: values of his Export that have no
// number in the old game's tables; the NAMES below are the main session's reading of the Export against its
// "old_game_terms" (the kept tool's table makes that reading exact in 4b), the VALUES are read from the Export here.
// Where the old game holds such a number in its code, the script finds that code by its text and prints the line.
// Node's built-in modules only.
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
const NOTES = '.checkpoints/cs4-walk/';
const exportText = readFileSync(NOTES + 'testlines-export.json', 'utf8');
const digest = createHash('sha256').update(exportText).digest('hex');
if (!readFileSync(NOTES + 'testlines-export.manifest.txt', 'utf8').includes(`sha256: ${digest}`)) {
  console.error('the saved Export does not match its manifest'); process.exit(1);
}
const compare = readFileSync(NOTES + 'testlines-compare.txt', 'utf8');
if (!compare.includes(`sha256 ${digest}`)) { console.error('testlines-compare.txt was made on another Export'); process.exit(1); }
const exp = JSON.parse(exportText);
const show = (v) => (typeof v === 'string' ? `"${v}"` : JSON.stringify(v));

// Part A: the comparison's DIFFERENT block.
const lines = compare.split('\n');
const from = lines.findIndex((l) => l.startsWith('DIFFERENT ('));
const count = Number((lines[from].match(/^DIFFERENT \((\d+)\)/) || [])[1]);
const partA = [];
for (let i = from + 1; i < lines.length && lines[i].startsWith('- '); i++) {
  const m = lines[i].match(/^- (.+?): the old game (.+); his Export (.+)$/);
  if (!m) { console.error(`cannot read the comparison's line: ${lines[i]}`); process.exit(1); }
  partA.push(m.slice(1));
}
if (partA.length !== count) { console.error(`the comparison says ${count} differences, ${partA.length} were read`); process.exit(1); }

// Part B: what he changed from the sandbox's start values, for the groups the game takes.
const NOT_TAKEN = new Set(['hornet', 'measuring_grid', 'metrics_room']);
const partB = exp.changed_from_start.filter((c) => !NOT_TAKEN.has(c.group));
const leftOutGroups = [...new Set(exp.changed_from_start.filter((c) => NOT_TAKEN.has(c.group)).map((c) => c.group))];
const leftOut = exp.changed_from_start.length - partB.length;
// Rows of part B that the game does not carry either, each said in its row, so that a yes to the list takes neither.
const NOT_CARRIED = {
  'simulation.step_rate_hz': 'not carried: the game runs at 30 steps a second (question STEP-60)',
  'soldier.faction': 'not carried: the sandbox session\'s own choice',
};

// Part C: names with no number in the old game's tables. [path, what the old game has, optional code text to find]
const legacy = readFileSync('legacy/shamble-game.js', 'utf8').split('\n');
const codeLine = (text) => {
  const hits = legacy.map((l, i) => (l.includes(text) ? i + 1 : 0)).filter(Boolean);
  if (hits.length !== 1) { console.error(`the old game's code has "${text}" ${hits.length} times, not once`); process.exit(1); }
  return `\`${text}\` (legacy/shamble-game.js:${hits[0]})`;
};
const WALK = [
  ['movement.body_radius_m', 'a number in its code, not in a table: ' + codeLine('resolveObstacles(player.position,0.7)')],
  ['movement.air_control_pct', 'no number: full steering in the air'],
  ['pushing.push_strength_per_s', 'no such rule: soldiers pass through each other'],
  ['pushing.push_radius_m', 'no such rule'],
  ['aim_down_sights.ads_flatten_floor_deg', 'a number in its code: ' + codeLine('const _tgt=Math.max(28.5,Math.min(_base,_toAim))')],
  ['aim_down_sights.ads_dead_zone_scale', 'a number in its code: ' + codeLine('const dzx=ps.ads?T.camDeadX*0.45:T.camDeadX')],
  ['aim_down_sights.ads_camera_ease_per_s', 'a number in its code: ' + codeLine('adsPitchK+=(_t-adsPitchK)*(1-Math.exp(-7*dt))')],
];
const LATER = [
  ...['smg', 'rifle', 'shotgun'].flatMap((g) => [
    [`${g}.sprint_to_fire_delay_s`, 'no such rule (SLICE-FEEL adds it)'],
    [`${g}.active_reload_window_start_pct`, 'no such rule (SCOPE-RELOAD adds it)'],
    [`${g}.active_reload_window_width_pct`, 'no such rule'],
  ]),
  ['shotgun.falloff_start_m', 'no falloff on this gun'],
  ['shotgun.falloff_end_m', 'no falloff on this gun'],
  ['shotgun.falloff_min_share', 'no falloff on this gun'],
  ['grenade.grenade_refill_s', 'not among the Export\'s old-game names'],
  ['grenade.bounce_kept', 'not among the Export\'s old-game names'],
  ['grenade.slide_kept', 'not among the Export\'s old-game names'],
  ['grenade.fuse_s', 'not among the Export\'s old-game names'],
];
const value = (path) => {
  const [group, name] = path.split('.');
  const v = exp.values[group] && exp.values[group][name];
  if (v === undefined) { console.error(`the Export has no value ${path}`); process.exit(1); }
  return show(v);
};
const rowsC = (list) => list.map(([path, old]) => `   | \`${path}\` | ${value(path)} | ${old} |`);

const out = [
  `   _Printed by \`testlines-difflist.mjs\` from the saved Export (sha256 ${digest.slice(0, 8)}...${digest.slice(-4)}) and from`,
  '   `testlines-compare.txt`; no value is typed by hand._',
  '',
  `   **A. The ${partA.length} names where his Export differs from the old game's tables** (\`docs/legacy-tuning.json\`):`,
  '   | The old game\'s name | The old game | His Export |',
  '   |---|---|---|',
  ...partA.map(([name, old, his]) => `   | \`${name}\` | ${old} | ${his} |`),
  '',
  `   **B. What he changed from the sandbox's start values** (the Export's own list: ${exp.changed_from_start.length} entries,`,
  `   ${leftOut} of them left out here because the game does not take their group: ${leftOutGroups.map((g) => `\`${g}\``).join(', ')}):`,
  '   | Group and name | Start | His value | |',
  '   |---|---|---|---|',
  ...partB.map((c) => `   | \`${c.group}.${c.name}\` | ${show(c.start)} | ${show(c.current)} | ${NOT_CARRIED[`${c.group}.${c.name}`] || ''} |`),
  '',
  '   **C. Values with no number in the old game\'s tables, used by Walk:**',
  '   | His Export\'s name | His value | The old game |',
  '   |---|---|---|',
  ...rowsC(WALK),
  '',
  '   **The same kind, used from change-sets 8 and 9 on** (their plans ask again; shown so the list is whole):',
  '   | His Export\'s name | His value | The old game |',
  '   |---|---|---|',
  ...rowsC(LATER),
];
writeFileSync(NOTES + 'testlines-difflist.md', out.map((l) => l.replace(/^ {3}/, '')).join('\n') + '\n');
if (process.argv.includes('--into')) {
  const planPath = NOTES + 'plan.md';
  const plan = readFileSync(planPath, 'utf8').split('\n');
  const a = plan.findIndex((l) => l.includes('<!-- difflist:begin -->'));
  const b = plan.findIndex((l) => l.includes('<!-- difflist:end -->'));
  if (a < 0 || b <= a) { console.error('plan.md has no difflist markers'); process.exit(1); }
  writeFileSync(planPath, [...plan.slice(0, a + 1), ...out, ...plan.slice(b)].join('\n'));
  console.log(`plan.md: ${out.length} lines between the markers`);
}
console.log(`A ${partA.length}, B ${partB.length} of ${exp.changed_from_start.length}, C ${WALK.length} + ${LATER.length}`);
