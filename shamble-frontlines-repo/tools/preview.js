// SETUP (fresh container):  mkdir -p /home/claude/tools && cd /home/claude/tools &&
//   npm i three@0.128.0 gl @napi-rs/canvas pngjs acorn@8 acorn-walk@8
// RUN under a virtual display:  xvfb-run -a -s "-screen 0 1280x1024x24" node preview.js <buildDir> <cmd> ...
//   cmds: list | measure | time | shot <out.png> <x> <z> [fov=15] [W] [H]
// Headless preview of the REAL build: three r128 + shamble-core.js + shamble-game.js, bootGame(), gameplay camera.
// usage: xvfb-run -a node preview.js <buildDir> list
//        xvfb-run -a node preview.js <buildDir> shot <out.png> <x> <z> [fov=15] [W=1280] [H=720]
// Limits: WebGL1 (ANGLE) not WebGL2; no DOM/CSS layers (HUD, halftone overlay); canvas text uses Skia fonts.
const fs = require('fs'), vm = require('vm');
const TOOLS = '/home/claude/tools/node_modules/';
const createGL = require(TOOLS + 'gl');
const { createCanvas } = require(TOOLS + '@napi-rs/canvas');
const { PNG } = require(TOOLS + 'pngjs');
const THREE = require(TOOLS + 'three');

const [dir, cmd, outFile, xArg, zArg, fovArg, wArg, hArg] = process.argv.slice(2);
const W = +(wArg || 1280), H = +(hArg || 720), SS = 2;           // render at 2x, box-downsample -> smooth lines
const BW = W * SS, BH = H * SS;
const gl = createGL(BW, BH, { preserveDrawingBuffer: true, antialias: false });
if (!gl) throw new Error('headless-gl context failed');

// ---------- DOM / window stubs ----------
function makeCanvasEl() {
  const cv = createCanvas(300, 150);
  const wrapCtx = ctx => {
    if (ctx && !ctx.__w) {
      const di = ctx.drawImage.bind(ctx), cp = ctx.createPattern.bind(ctx);
      ctx.drawImage = (img, ...a) => di(img && img._cv ? img._cv : img, ...a);
      ctx.createPattern = (img, rep) => cp(img && img._cv ? img._cv : img, rep);
      ctx.__w = true;
    }
    return ctx;
  };
  return {
    _cv: cv, style: {}, dataset: {}, classList: { add() {}, remove() {}, toggle() {}, contains() { return false; } },
    get width() { return cv.width; }, set width(v) { cv.width = v; },
    get height() { return cv.height; }, set height(v) { cv.height = v; },
    getContext(t) { return t === '2d' ? wrapCtx(cv.getContext('2d')) : null; },
    toDataURL(...a) { return cv.toDataURL(...a); },
    addEventListener() {}, removeEventListener() {}, setAttribute() {}, getAttribute() { return null; },
    getBoundingClientRect() { return { left: 0, top: 0, right: W, bottom: H, width: W, height: H, x: 0, y: 0 }; },
    appendChild(c) { return c; }, remove() {}, focus() {},
  };
}
function makeEl(tag) {
  if (tag === 'canvas') return makeCanvasEl();
  const set = new Set();
  const el = {
    tagName: String(tag || 'div').toUpperCase(), style: {}, dataset: {}, children: [], childNodes: [], options: [],
    classList: { add(...c) { c.forEach(x => set.add(x)); }, remove(...c) { c.forEach(x => set.delete(x)); },
      toggle(c, f) { const on = f === undefined ? !set.has(c) : !!f; on ? set.add(c) : set.delete(c); return on; }, contains(c) { return set.has(c); } },
    appendChild(c) { this.children.push(c); return c; }, append() {}, prepend() {}, removeChild(c) { return c; }, insertBefore(c) { return c; },
    remove() {}, replaceChildren() {}, addEventListener() {}, removeEventListener() {}, dispatchEvent() { return true; },
    setAttribute() {}, getAttribute() { return null; }, removeAttribute() {}, hasAttribute() { return false; },
    querySelector() { return makeEl('div'); }, querySelectorAll() { return []; }, getElementsByTagName() { return []; }, getElementsByClassName() { return []; },
    getBoundingClientRect() { return { left: 0, top: 0, right: W, bottom: H, width: W, height: H, x: 0, y: 0 }; },
    focus() {}, blur() {}, click() {}, cloneNode() { return makeEl(tag); }, closest() { return null; }, contains() { return false; },
    requestPointerLock() {}, scrollIntoView() {}, animate() { return { finished: Promise.resolve(), cancel() {} }; },
    textContent: '', innerHTML: '', innerText: '', value: '', checked: false, disabled: false, hidden: false,
    offsetWidth: W, offsetHeight: H, clientWidth: W, clientHeight: H, scrollTop: 0, parentNode: null, firstChild: null,
  };
  el.parentNode = el; el.parentElement = el;
  return el;
}
const byId = new Map();
globalThis.window = globalThis; globalThis.self = globalThis;
globalThis.innerWidth = BW; globalThis.innerHeight = BH; globalThis.devicePixelRatio = 1;
globalThis.addEventListener = () => {}; globalThis.removeEventListener = () => {};
globalThis.requestAnimationFrame = () => 0; globalThis.cancelAnimationFrame = () => {};
globalThis.matchMedia = () => ({ matches: false, addEventListener() {}, addListener() {} });
globalThis.getComputedStyle = () => ({ getPropertyValue: () => '' });
globalThis.document = {
  getElementById: id => { if (!byId.has(id)) byId.set(id, makeEl('div')); return byId.get(id); },
  createElement: makeEl, createElementNS: (ns, t) => makeEl(t), createTextNode: () => makeEl('span'),
  body: makeEl('body'), head: makeEl('head'), documentElement: makeEl('html'),
  addEventListener() {}, removeEventListener() {}, querySelector: () => makeEl('div'), querySelectorAll: () => [],
  hidden: false, visibilityState: 'visible', pointerLockElement: null, exitPointerLock() {}, fonts: { ready: Promise.resolve() },
};
try { Object.defineProperty(globalThis, 'navigator', { value: { userAgent: 'node', clipboard: { writeText: () => Promise.resolve() }, getGamepads: () => [] }, configurable: true }); } catch (e) {}
const permissive = () => new Proxy(function () {}, { get: (t, p) => (p === Symbol.toPrimitive ? () => 0 : permissive()), apply: () => permissive(), construct: () => permissive(), set: () => true });
globalThis.AudioContext = function () { return permissive(); }; globalThis.webkitAudioContext = globalThis.AudioContext;
globalThis.Image = function () { return makeCanvasEl(); };

// ---------- three r128 with the renderer bound to the headless context ----------
const RealRenderer = THREE.WebGLRenderer;
THREE.WebGLRenderer = function (params) { const c = makeCanvasEl(); c.width = BW; c.height = BH; try { gl.canvas = c; } catch (e) {} if (gl.canvas !== c) Object.defineProperty(gl, 'canvas', { value: c, configurable: true }); return new RealRenderer(Object.assign({}, params, { canvas: c, context: gl })); };
globalThis.THREE = THREE;

const warn = console.warn; console.warn = (...a) => { const s = String(a[0] || ''); if (!/THREE\.WebGLRenderer|extension|Image is not power of two/.test(s)) warn(...a); };
const run = (code, name) => vm.runInThisContext(code, { filename: name || 'inline' });
run(fs.readFileSync(dir + '/shamble-core.js', 'utf8'), 'shamble-core.js');
run(fs.readFileSync(dir + '/shamble-game.js', 'utf8'), 'shamble-game.js');
const _t0 = process.hrtime.bigint(); run('bootGame();', 'boot'); if (cmd === 'time') { const ms = Number(process.hrtime.bigint() - _t0) / 1e6; const t1 = process.hrtime.bigint(); run(`for(let i=0;i<24;i++){ const g=buildSoldier?buildSoldier('Soviets','conscript'):null; }`, 'rebuilds'); console.log('bootGame ms', ms.toFixed(0), '| 24 soldier rebuilds ms', (Number(process.hrtime.bigint() - t1) / 1e6).toFixed(0)); process.exit(0); }

if (cmd === 'evalshot') {   // evalshot <snippet.js> <out.png>: the snippet renders (e.g. a multi-viewport contact sheet); we read back + save
  const r = run(fs.readFileSync(outFile, 'utf8'), 'evalshot-snippet'); if (r !== undefined) console.log(JSON.stringify(r));
  const px2 = new Uint8Array(BW * BH * 4); gl.readPixels(0, 0, BW, BH, gl.RGBA, gl.UNSIGNED_BYTE, px2);
  const png2 = new PNG({ width: W, height: H });
  for (let y = 0; y < H; y++) for (let xx = 0; xx < W; xx++) { let r0 = 0, g0 = 0, b0 = 0;
    for (let sy = 0; sy < SS; sy++) for (let sx = 0; sx < SS; sx++) { const i = (((BH - 1 - (y * SS + sy)) * BW) + (xx * SS + sx)) * 4; r0 += px2[i]; g0 += px2[i + 1]; b0 += px2[i + 2]; }
    const o = (y * W + xx) * 4; png2.data[o] = r0 / 4; png2.data[o + 1] = g0 / 4; png2.data[o + 2] = b0 / 4; png2.data[o + 3] = 255; }
  fs.writeFileSync(xArg, PNG.sync.write(png2)); console.log('wrote', xArg); process.exit(0);
}
if (cmd === 'eval') {   // eval <file.js>: run a snippet inside the booted game and print its (JSON) result
  const r = run(fs.readFileSync(outFile, 'utf8'), 'eval-snippet'); console.log(JSON.stringify(r)); process.exit(0);
}
if (cmd === 'measure') {   // effective ink width (world units): old = oThick*|A*n| (object-space offset), new = oThick*|oNormal| (world-space)
  const res = run(`(function(){ const out={char:[],world:[]}; const A=new THREE.Matrix3(), v=new THREE.Vector3();
    scene.updateMatrixWorld(true);
    scene.traverse(o=>{ if(!o.userData||!o.userData.isOutline||!o.parent||!o.material.uniforms) return;
      const g=o.geometry, N=g.attributes.normal, O=g.attributes.oNormal, t=o.material.uniforms.oThick.value; if(!N) return;
      A.setFromMatrix4(o.matrixWorld); const step=Math.max(1,Math.floor(N.count/12)); let so=0,sn=0,c=0;
      for(let i=0;i<N.count;i+=step){ v.set(N.getX(i),N.getY(i),N.getZ(i)).applyMatrix3(A); so+=v.length()*t; sn+=(O?(O.getX(i)*N.getX(i)+O.getY(i)*N.getY(i)+O.getZ(i)*N.getZ(i)):1)*t; c++; }
      (o.parent.userData.part?out.char:out.world).push({old:so/c,now:sn/c});
    }); return out; })()`);
  const stat = arr => { if (!arr.length) return 'n/a'; const r = arr.map(e => e.now / e.old).sort((a, b) => a - b), q = f => r[Math.min(r.length - 1, Math.floor(f * r.length))].toFixed(2);
    const mo = arr.map(e => e.old).sort((a, b) => a - b), mn = arr.map(e => e.now).sort((a, b) => a - b), med = a => a[Math.floor(a.length / 2)].toFixed(4);
    return `${arr.length} hulls | width old median ${med(mo)} -> new median ${med(mn)} | new/old ratio p10 ${q(0.1)} median ${q(0.5)} p90 ${q(0.9)}`; };
  console.log('CHARACTER parts:', stat(res.char)); console.log('WORLD objects  :', stat(res.world));
  process.exit(0);
}
if (cmd === 'list') {
  const info = run(`({ houses:(typeof houses!=='undefined'?houses:[]).map(h=>{const g=h.grp||h.group||h.mesh||h; const p=g&&g.position; return p?{bt:h.bt||(g.userData&&g.userData.bt)||'?',x:+p.x.toFixed(1),z:+p.z.toFixed(1)}:null;}).filter(Boolean),
     buildings:(typeof buildings!=='undefined'?buildings:[]).map(b=>{const g=b.mesh||b.group||b.grp; const p=g&&g.position; return p?{kind:b.kind||b.type||b.name||'?',x:+p.x.toFixed(1),z:+p.z.toFixed(1)}:null;}).filter(Boolean),
     trees:(typeof trees!=='undefined'?trees.length:0), props:(typeof props!=='undefined'?props.length:0), obstacles:(typeof obstacles!=='undefined'?obstacles.length:0),
     spawn:(typeof mapSpawn!=='undefined'&&mapSpawn)?{x:+mapSpawn.x.toFixed(1),z:+mapSpawn.z.toFixed(1)}:null })`);
  console.log(JSON.stringify(info));
  process.exit(0);
}

if (cmd === 'menu' || cmd === 'top') {   // menu camera (first impression) | orthographic top-down overview for layout work
  const cx = +(xArg || 0), cz = +(zArg || 0), span = +(fovArg || 420);
  run(cmd === 'menu'
    ? `(function(){ setMenuCam(); camera.aspect=${W}/${H}; camera.updateProjectionMatrix(); camera.updateMatrixWorld(); renderer.setSize(${BW},${BH},false); renderer.render(scene,camera); })()`
    : `(function(){ const hw=${span}/2, hh=hw*${H}/${W}; const oc=new THREE.OrthographicCamera(-hw,hw,hh,-hh,1,3000);
        oc.position.set(${cx},220,${cz}); oc.up.set(0,0,-1); oc.lookAt(${cx},0,${cz}); oc.updateProjectionMatrix(); oc.updateMatrixWorld();
        if(typeof player!=='undefined'&&player) player.position.set(${cx},0,${cz}); if(typeof updateSunFollow==='function') updateSunFollow();
        renderer.setSize(${BW},${BH},false); renderer.render(scene,oc); })()`, cmd);
} else {
// ---------- gameplay framing: camera at target + (sinY*camBack, camHeight, cosY*camBack), look at target+1.2 ----------
const x = +xArg, z = +zArg, fov = +(fovArg || 15);
run(`(function(){
  const yaw=T.camYawBase||0.7853981633974483, sY=Math.sin(yaw), cY=Math.cos(yaw);
  if(typeof player!=='undefined'&&player){ player.position.set(${x},0,${z}); player.visible=true; }
  if(typeof updateSunFollow==='function') updateSunFollow();
  camera.up.set(0,1,0); camera.fov=${fov}; camera.aspect=${W}/${H};
  camera.position.set(${x}+sY*T.camBack, T.camHeight, ${z}+cY*T.camBack); camera.lookAt(${x},1.2,${z});
  camera.updateProjectionMatrix(); camera.updateMatrixWorld();
  renderer.setSize(${BW},${BH},false); renderer.render(scene,camera);
})()`, 'shot');
}

const px = new Uint8Array(BW * BH * 4);
gl.readPixels(0, 0, BW, BH, gl.RGBA, gl.UNSIGNED_BYTE, px);
const png = new PNG({ width: W, height: H });
for (let y = 0; y < H; y++) for (let xx = 0; xx < W; xx++) {
  let r = 0, g = 0, b = 0;
  for (let sy = 0; sy < SS; sy++) for (let sx = 0; sx < SS; sx++) {
    const i = (((BH - 1 - (y * SS + sy)) * BW) + (xx * SS + sx)) * 4;   // flip Y
    r += px[i]; g += px[i + 1]; b += px[i + 2];
  }
  const o = (y * W + xx) * 4, n = SS * SS;
  png.data[o] = r / n; png.data[o + 1] = g / n; png.data[o + 2] = b / n; png.data[o + 3] = 255;
}
fs.writeFileSync(outFile, PNG.sync.write(png));
console.log('wrote', outFile, W + 'x' + H, cmd, xArg || '', zArg || '', fovArg || '');
process.exit(0);
