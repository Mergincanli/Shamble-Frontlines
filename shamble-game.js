/* ============================================================================
 * shamble-game.js — Shamble Frontlines gameplay + shared dispatch.  See MODULE_MAP.md.
 * PURPOSE:   All gameplay, plus the shared control/dispatch layer both pages use.
 *            Loaded by BOTH builds (player + authoring).
 * PROVIDES:  SIM<->view bridge (syncSimWorld / castShot); world
 *            placement glue (placeLegacyHouse / spawnKitBuilding / addProp /
 *            addTree / addCover / makeBuilding / makeBarracks / makeEnemySpawn);
 *            the world-registry arrays (obstacles / houses / covers / trees /
 *            props / paths / platforms / ramps / buildings / barracksList /
 *            enemies / harvesters / enemySpawns) whose lifecycle
 *            buildMap / clearMap / serializeMap own; movement collision
 *            (resolveObstacles); rules & economy (applyRules / RULES_*);
 *            Juggernaut vehicles; enemies / AI; harvesters; HUD incl. the combat
 *            reticle / hitmarker / reload-ring (placeCursor); FX / audio; the
 *            war-room menu + pause UI (wr*: dossier, clipboard, field manual); per-unit
 *            progression; per-frame update(dt); AND the shared dispatch: loop(),
 *            setMenuCam(), backToMenu(), deploy(), startAmbient(), the input
 *            listeners, and bootGame() (consolidates all game-boot side-effects;
 *            both pages call it).
 * REQUIRES:  shamble-core.js (THREE + all core globals/helpers). Editor refs here
 *            are dead-branch-safe (behind mode==='editor' / 'charlab') or typeof-
 *            guarded, so none fire in the editor-less index.html.
 * LOAD ORDER: THREE -> core -> shamble-game.js -> [editor].
 * SIDE-EFFECTS ON LOAD: none — declarations only; the page calls bootGame().
 * ========================================================================== */
const PLAYER_FACTION='Allies';   // the player's faction: uniform + vehicle livery
const PLAYER_TEAM=(TEAMS.find(t=>t.faction===PLAYER_FACTION)||TEAMS[0]).id;   // the player's team id (D3: allies); TEAMS loads with core, so no TDZ
let unitOwned=UNIT_TYPES.map((u,i)=>i===0);  // you start as Conscript; the others unlock in the armory (free T1, Officer 1250 @ T2)
// ── VEHICLES: enterable, bought separately from infantry. Registry only for now; the model + enter/drive logic lands in a later pass (spawnVehicle stub below). Stats are placeholders. ──
function updateSunFollow(){ const snap=(2*SUN_SD)/2048; const cx=Math.round(player.position.x/snap)*snap, cz=Math.round(player.position.z/snap)*snap; sun.position.set(cx+_sunDir.x*SUN_R, _sunDir.y*SUN_R, cz+_sunDir.z*SUN_R); sun.target.position.set(cx,0,cz); sun.target.updateMatrixWorld(); }

let sightLine=null, heightLine=null, adsMode='hold'; // ADS sight + Z-height line meshes; aim toggle setting

/* ============ ENVIRONMENT MODEL (ground look + lighting + sky/fog) ============
 * One source of truth for the scene's mood. Persisted per-map (see serializeMap.env).
 * applyEnv() mutates the live scene; the editor World panel drives it. Ground is a
 * MeshToonMaterial at render time, so the meaningful knobs are colour/detail/cel-cap,
 * NOT roughness/metalness. A dedicated ground cel ramp lets us cap LIT-grass brightness
 * without touching characters/buildings. */
const RULES_DEFAULT={ silo:{capture:3.5}, tbox:{active:120,capture:7,first:8,gap:6}, eco:{resupply:50}, harv:{load:3.5,dock:3.0,respawn:12} };
function rulesMerge(src){ const d=JSON.parse(JSON.stringify(RULES_DEFAULT)); if(src){ for(const k in d){ if(d[k]&&typeof d[k]==='object'&&src[k]&&typeof src[k]==='object') Object.assign(d[k],src[k]); else if(src[k]!==undefined) d[k]=src[k]; } } return d; }
let _rules=rulesMerge(null), _rulesReady=false;
function applyRules(r){ SILO.captureTime=r.silo.capture; TBOX.activeTime=r.tbox.active; TBOX.captureTime=r.tbox.capture; TBOX.firstDelay=r.tbox.first; TBOX.gap=r.tbox.gap; T.resupplyCost=r.eco.resupply; HARV.loadTime=r.harv.load; HARV.dockTime=r.harv.dock; HARV.respawn=r.harv.respawn; }
function currentRulesJSON(){ return JSON.parse(JSON.stringify(_rules)); }
const GUN_WORDS=['BLAM','POW','BANG','TRA','DAKKA','RAT-A-TAT','BRRT','THUNK','BRRR..'];
const KILL_WORDS=['POW!','KO!','BAM!','BOOM!','ZAP!'];
let _gw=0, _comicCd=0; const _comicCache={};
function makeComicTex(word, fill){
  const c=document.createElement('canvas'); c.width=320; c.height=170; const x=c.getContext('2d');
  x.translate(160,85); x.rotate(Math.random()*0.22-0.11);
  x.font='900 84px Impact, "Arial Black", system-ui, sans-serif'; x.textAlign='center'; x.textBaseline='middle'; x.lineJoin='round'; x.miterLimit=2;
  x.lineWidth=16; x.strokeStyle='#1d3557'; x.strokeText(word,0,0);
  x.lineWidth=6; x.strokeStyle='#0c1424'; x.strokeText(word,0,0);
  x.fillStyle=fill||'#FFD166'; x.fillText(word,0,0);
  const t=new THREE.CanvasTexture(c); t.needsUpdate=true; return t;
}
function comicTex(word, fill){ const k=word+'|'+(fill||''); return _comicCache[k]||(_comicCache[k]=makeComicTex(word,fill)); }
function spawnComic(word, pos, scale, fill){
  if(!COMIC) return;
  const cs=FXPool.acquire('comic'); const s=cs.mesh;
  s.material.map=comicTex(word,fill); s.material.opacity=1;
  s.position.copy(pos); s.position.y+=0.5; const sc=scale||2.2; s.scale.set(sc*1.7, sc, 1);
  fx.push({ mesh:s, t:0, life:0.42, kind:'comic', sc, slot:cs });
}
function comicGun(pos){ if(!COMIC || performance.now()<_comicCd) return; _comicCd=performance.now()+150; spawnComic(GUN_WORDS[(_gw++)%GUN_WORDS.length], pos, 1.9, '#FFD166'); }
(function(){ try{ for(const w of GUN_WORDS) comicTex(w,'#FFD166'); for(const w of KILL_WORDS) comicTex(w,'#FFD166');
  for(const w of ['BANZAI!','CHARGE!','BOOM!','IMPALE!','SLASH!']) comicTex(w,'#FF7B00'); }catch(e){} })();   // pre-warm comic textures
function comicKill(pos){ spawnComic(KILL_WORDS[(Math.random()*KILL_WORDS.length)|0], pos, 2.7, '#FFD166'); }
const grid=new THREE.GridHelper(440,176,0x8a7440,0x6f5d2e); grid.position.y=0.02; grid.material.opacity=0.15; grid.material.transparent=true; noRay(grid); scene.add(grid);
// map border — red boundary at the play-area edge (visible in game + editor)
const _borderMat=new THREE.MeshBasicMaterial({color:0xff4d4d,transparent:true,opacity:0.8}); const _borderBoxes=[];
function rebuildBorder(){ const ax=T.arenaX, az=T.arenaZ, t=0.5, h=0.14;
  for(const b of _borderBoxes){ scene.remove(b); b.geometry.dispose(); } _borderBoxes.length=0;
  const mk=(w,d,x,z)=>{const b=new THREE.Mesh(new THREE.BoxGeometry(w,h,d),_borderMat);b.position.set(x,h/2+0.02,z);noRay(b);scene.add(b);_borderBoxes.push(b);};
  mk(ax*2+t,t,0,-az); mk(ax*2+t,t,0,az); mk(t,az*2+t,-ax,0); mk(t,az*2+t,ax,0);
}
function setMapSize(half){ const v=Math.max(24,Math.min(220,Math.round(half))); T.arenaX=v; T.arenaZ=v; rebuildBorder(); } // dev square override; map drives arenaX/arenaZ from the walkable extent
const obstacles=[];
const paths=[];  // flat decal 'roads' (visual only, no collision)
const covers=[]; // linear cover pieces (hedgerow / drystone wall / fence) -- own compound collider + serialize
const trees=[]; // point-placed vegetation (apple / poplar / oak) -- single AABB collider + serialize
const props=[]; // farmyard dressing (haystack / cart / well / ...) -- rotatable, per-prop AABB collider + serialize
const crateMat=new THREE.MeshStandardMaterial({color:0x2c3a3b,roughness:0.85});
const crateMatWarm=new THREE.MeshStandardMaterial({color:0x3a3330,roughness:0.85});
function addBox(x,z,w,h,d,mat,isCrate){const m=new THREE.Mesh(new THREE.BoxGeometry(w,h,d),mat||crateMat);m.position.set(x,h/2,z);m.castShadow=true;m.receiveShadow=true;scene.add(m);if(typeof stylizeOne==='function')stylizeOne(m);const o={mesh:m,box:new THREE.Box3().setFromObject(m),w,h,d,warm:mat===crateMatWarm,crate:isCrate!==false};m.userData.editSel={kind:o.crate?'crate':'wall',ref:o};obstacles.push(o);return o;}
// ---- Phase 5: walkable elevated platforms (solid box from ground to top; sides are cliffs, top is a walkable surface) ----
const platMat=new THREE.MeshStandardMaterial({color:0x6f7358,roughness:0.96});
const platforms=[];
function makePlatform(x,z,w,d,h){ const m=new THREE.Mesh(new THREE.BoxGeometry(w,h,d),platMat); m.position.set(x,h/2,z); m.castShadow=true; m.receiveShadow=true; scene.add(m); if(typeof stylizeOne==='function')stylizeOne(m); const o={mesh:m,box:new THREE.Box3().setFromObject(m),w,h,d}; m.userData.editSel={kind:'platform',ref:o}; platforms.push(o); return o; }
// ---- Phase 5b: walkable ramps (sloped surface rising along local +X from h0 at -w/2 to h1 at +w/2; rot orients it) ----
function rampGeometry(w,d,h0,h1){ const hw=w/2, hd=d/2;
  const pos=new Float32Array([ -hw,0,-hd, hw,0,-hd, hw,0,hd, -hw,0,hd,  -hw,h0,-hd, hw,h1,-hd, hw,h1,hd, -hw,h0,hd ]);
  const idx=[ 0,1,2, 0,2,3,  4,6,5, 4,7,6,  0,3,7, 0,7,4,  1,5,6, 1,6,2,  0,4,5, 0,5,1,  3,2,6, 3,6,7 ];
  const g=new THREE.BufferGeometry(); g.setAttribute('position', new THREE.BufferAttribute(pos,3)); g.setIndex(idx); g.computeVertexNormals(); return g; }
const ramps=[];
function makeRamp(x,z,w,d,h0,h1,rot){ const m=new THREE.Mesh(rampGeometry(w,d,h0||0,h1), platMat); m.position.set(x,0,z); m.rotation.y=rot||0; m.castShadow=true; m.receiveShadow=true; scene.add(m); if(typeof stylizeOne==='function')stylizeOne(m); const o={mesh:m,box:new THREE.Box3().setFromObject(m),w,d,h0:h0||0,h1,rot:rot||0,x,z}; m.userData.editSel={kind:'ramp',ref:o}; ramps.push(o); return o; }
function rampHeightAt(o,x,z){ const dx=x-o.x, dz=z-o.z, c=Math.cos(o.rot), s=Math.sin(o.rot); const lx=c*dx - s*dz, lz=s*dx + c*dz; if(lx< -o.w/2 || lx> o.w/2 || lz< -o.d/2 || lz> o.d/2) return null; const t=(lx + o.w/2)/o.w; return o.h0 + (o.h1-o.h0)*t; }
function addPath(cx,cz,w,d,rotY){ const m=new THREE.Mesh(new THREE.BoxGeometry(w,0.06,d), new THREE.MeshStandardMaterial({color:0x8f8c86,roughness:1.0})); m.position.set(cx,0.03,cz); if(rotY)m.rotation.y=rotY; m.receiveShadow=true; scene.add(m); if(typeof stylizeOne==='function')stylizeOne(m); const o={mesh:m,w,d}; m.userData.editSel={kind:'path',ref:o}; paths.push(o); return o; }
/* crates, buildings, barracks, spawn + resupply are now data-driven — see DM_Romania + buildMap() below */

/* labels */
function makeLabel(text,color){
  const c=document.createElement('canvas'); c.width=256; c.height=64; const x=c.getContext('2d');
  x.font='bold 42px monospace'; x.fillStyle=color; x.textAlign='center'; x.textBaseline='middle';
  x.shadowColor='rgba(0,0,0,0.8)'; x.shadowBlur=6; x.fillText(text,128,32);
  const tex=new THREE.CanvasTexture(c);
  const s=new THREE.Sprite(new THREE.SpriteMaterial({map:tex,transparent:true,depthTest:false})); s.scale.set(5,1.25,1); noRay(s); return s;
}

/* soldier — with weapon pivot (for barrel pitch), muzzle node, and tagged hitboxes */
/* ---- PROCEDURAL CHARACTER RIG (rigid primitives + code-driven animation) ----
   Same userData hooks as makeSoldier (wpn/muzzle/head) so weapons, tint and aim
   are unchanged. Gun + arms parent to the chest; legs+pelvis carry the gait. */
/* ============================================================================
 * ENTITY RECIPE SYSTEM — data-driven procedural rigs (the "parts list").
 * An entity is a flat list of parts (groups + primitive meshes) parented by id,
 * plus a joints map (which groups animate), a weapon mount, and a hit-shape.
 * buildEntity() walks ANY such def, so a soldier is kind:'soldier' and vehicles
 * (tanks/mechs) will be added later as other kinds — same engine, new recipes.
 * The hit-shape lives in the SAME def (def.hit) as the visual, so look + hitbox
 * read from one source and can never drift (no phantom hits). Verified node-for-
 * node against the old hand-built rig via the rig harness.
 * ========================================================================== */
const SOLDIER_KIT={headgear:'brodie', torso:'greatcoat', legs:'trousers', feet:'boots', webbing:'webbing', hair:'default', face:'mustache'};
const ELITE_KIT={headgear:'none', torso:'greatcoat', legs:'trousers', feet:'boots', webbing:'webbing', hair:'crop', face:'none'};   // Empire elite villain (no helmet -> visor reads)
function buildSoldier(faction, sex, kit){
  const def=assembleChar(Object.assign({sex:sex||'m'}, kit||SOLDIER_KIT));
  def.weapon=CHAR_DEFAULT.weapon;   // rifle mount (parented to 'chest'; rides the body)
  def.armMode='rifle';              // hold the rifle (CHAR_MALE defaults to 'relaxed')
  return buildEntity(def, {faction});
}
const ANIM = {
  // ---- gait: feet PLANT in the world; cadence comes from leg reach, so stride always fits the legs ----
  idleSpeed: 0.35,               // below this = idle (feet planted; re-step when they stray = turn-in-place)
  walkRef: 6.3, sprintRef: 9.0,  // SIM speeds (walk = T.moveSpeed)
  duty: [0.60, 0.30, 0.26],      // share of a cycle each foot is planted: slow -> run (walkRef) -> sprint (flight phase when running)
  drop: [0.03, 0.11, 0.14],      // pelvis lowers into a running crouch: slow -> run -> sprint (m). Stride length is then
                                 // computed from the legs' real reach at that height, so feet always land within reach
  lift: [0.13, 0.26],            // swing-foot lift (m)
  toeOut: 0.10, toePitch: 1.5,   // feet splay (rad); toe-off / heel-strike roll during the swing (x lift, rad)
  cycleMin: 0.26, cycleMax: 1.10,// gait cycle bounds (s)
  stanceW: 0.95,                 // feet under the hips x this
  idleStagger: 0.11,             // idle shooter's stance: lead foot forward, rear foot back (m)
  restepDist: 0.26, idleStepT: 0.2,   // idle re-step trigger (m) + step time (s)
  pelvisBob: [0.012, 0.024], pelvisSway: 0.02, pelvisRoll: 0.035,
  twistMax: 0.95,                // hips turn toward movement at most this far from the aim (rad); backpedal keeps hips to the aim
  walkLean: 0.06, sprintLean: 0.30,
  torsoRoll: 0.03, breath: 0.02, breathRate: 1.7,
  // ---- springs: critically damped; k = stiffness (settle ~4/sqrt(k) s) ----
  spr: { gait:{k:60}, legYaw:{k:110}, lean:{k:70}, drop:{k:90}, stance:{k:40}, sway:{k:130}, coat:{k:70} },
  // ---- weapon holds (per GUN_KIT hold): where the gun's STOCK sits (chest-joint relative, aim-aligned axes),
  //      its [pitch, yaw, roll] (YXZ; 'aim' = follow the aim pitch), and how far the chest blades toward the gun ----
  hold: {
    rifle:  { carry:{ at:[0.52,-0.34,-0.10], rot:[0.24,-0.28,0.22], blade:0.48 },
              ads:  { at:[0.52,-0.04,-0.06], rot:['aim',-0.05,0.0], blade:0.52 },
              sprint:{ at:[0.40,-0.52,0.30], rot:[-0.80,-1.00,-0.40], blade:0.16 } },
    smg:    { carry:{ at:[0.50,-0.28,0.00], rot:[0.18,-0.26,0.18], blade:0.44 },
              ads:  { at:[0.50,0.00,0.02], rot:['aim',-0.06,0.0], blade:0.46 },
              sprint:{ at:[0.42,-0.46,0.30], rot:[-0.85,-1.00,-0.40], blade:0.16 } },
    scatter:{ carry:{ at:[0.52,-0.36,-0.02], rot:[0.18,-0.26,0.18], blade:0.48 },
              ads:  { at:[0.54,-0.24,-0.02], rot:['aim',-0.06,0.0], blade:0.48 },
              sprint:{ at:[0.40,-0.52,0.30], rot:[-0.80,-1.00,-0.40], blade:0.16 } },
  },
  poleR: [0.55,-0.80,-0.25], poleL: [-0.55,-0.80,-0.25],   // elbow bend directions (chest space)
  reachFrac: 0.97,               // IK uses this share of full arm length (arms never lock straight)
  recoil: { back:0.12, pitch:0.22, chest:0.15 },           // scaled by the gun's mass
  sway: { turn:0.045, max:0.22, bob:0.6 },                 // gun lags quick aim turns + the body bob
  coat: { front:0.95, back:0.75, flare:0.12, min:-0.8, max:0.6 },   // greatcoat panels follow the thighs
  // ---- reloads: choreography over the reload's progress (0..1), per GUN_KIT reload style. keys = [p, pouch w, socket w,
  //      stroke m]: the support hand blends rail -> pouch/quiver -> the gun's reload point (+ a stroke along `dir`, gun space);
  //      pose = [pitch, yaw, roll, dx, dy, dz] added to the hold while reloading. Points are chest space. ----
  reload: { pouch:[-0.30,-0.80,0.40], quiver:[0.30,0.40,-0.35], styles:{
    mag:  { pose:[0.35,-0.25,-0.70,-0.08,0.10,0.05], dir:[0,-1,0], keys:[[0,0,0,0],[0.12,0,1,0],[0.22,0,1,0.12],[0.40,1,0,0],[0.52,1,0,0],[0.68,0,1,0.10],[0.76,0,1,-0.02],[0.86,0,0,0],[1,0,0,0]] },
    drum: { pose:[0.30,-0.30,-0.80,-0.08,0.10,0.05], dir:[0,-1,0], keys:[[0,0,0,0],[0.12,0,1,0],[0.24,0,1,0.14],[0.42,1,0,0],[0.54,1,0,0],[0.70,0,1,0.12],[0.78,0,1,-0.02],[0.88,0,0,0],[1,0,0,0]] },
    ram:  { pose:[-1.05,-0.20,0.0,-0.25,-0.90,0.35], dir:[0,0,-1], keys:[[0,0,0,0],[0.14,1,0,0],[0.24,1,0,0],[0.36,0,1,0],[0.44,0,1,0.28],[0.52,0,1,0],[0.60,0,1,0.28],[0.68,0,1,0],[0.80,0,0,0],[1,0,0,0]] },
    break:{ pose:[0.70,-0.20,-0.30,-0.05,0.05,0.05], dir:[0,-1,0], keys:[[0,0,0,0],[0.16,1,0,0],[0.26,1,0,0],[0.38,0,1,0],[0.46,0,1,0.06],[0.56,1,0,0],[0.64,0,1,0],[0.72,0,1,0.06],[0.84,0,1,0],[0.96,0,0,0],[1,0,0,0]] },
    nock: { pose:[0.15,-0.10,-0.20,0,0,0], dir:[0,0,-1], keys:[[0,0,0,0],[0.18,1,0,0],[0.30,1,0,0],[0.46,0,1,0],[0.54,0,1,0],[0.72,0,1,0.30],[0.86,0,0,0],[1,0,0,0]] } } },
  // ---- actions ----
  flinch: { k:120, kick:9, pitch:0.6, roll:1.0, head:0.5 },            // hit reaction: upper body snaps away from the hit (spring impulse)
  death:  { buckleT:0.22, kneelY:0.62, lieY:0.47, lie:1.40, push:1.0, g:11, bounce:0.3 },   // toy-soldier fall (m, rad, 1/s^2)
  draw:   { pitch:1.1, roll:0.4, at:[0.05,-0.35,-0.10] },               // weapon swap: the new gun comes up from low
  charge: { at:[-0.12,-0.10,0.30], pitch:0.12, yaw:0.20 },             // bayonet charge: gun thrust forward, bayonet first
  throw:  { wind:[-0.45,0.55,-0.30], release:[-0.10,0.15,0.95], t:0.45 },   // the support hand throws (chest space, s)
  shield: { at:[-0.10,-0.08,0.78], pitch:0.0, yaw:-0.12 },           // shield centre ahead of the chest (aim frame), face toward the aim
  katana: { at:[0.16,-0.42,0.58], pitch:-0.70, yaw:-0.10, sweep:0.30, hilt:[0,0,-0.12] },   // two-hand guard; hands travel with the slash
  // ---- aim layer: rifle hold blended carry <-> ADS by ps.ads (authored in the Character Lab pose tool) ----
  aim: {
    adsK: 45,   // ADS blend spring stiffness
    carry: { gun:[0.13,-0.13,0.70], rot:[0.72,-0.44,0.58], gripL:[-0.20,-0.14,0.30], gripR:[0.17,-0.04,0.03], poleL:[-0.60,-0.85,-0.30], poleR:[0.60,-0.85,-0.30] },
    ads:   { gun:[0.33,0.16,0.70],  rot:[-0.04,0.00,0.10], gripL:[0.06,-0.20,0.44], gripR:[0.18,-0.14,-0.07], poleL:[-0.60,-0.85,-0.30], poleR:[0.60,-0.85,-0.30] },
  },
  // ---- (action / foot-IK knobs are added with their stages) ----
};

// Critically-damped spring integrator — analytic, unconditionally stable at any dt.
// s = {x, v}; advances x toward `target` with stiffness k (damping is critical).
function springStep(s, target, k, dt){
  const w=Math.sqrt(k), e=Math.exp(-w*dt), A=s.x-target, B=s.v+w*A;
  s.x = target + (A + B*dt)*e;
  s.v = (B - w*(A + B*dt))*e;
  return s.x;
}
// reused per-frame state struct for updateSoldierAnim (zero per-frame allocation)
const _animSt = { aimYaw:0, t:0, kick:0, dip:0, lean:0, grounded:true, ads:0, swap:0, rel:-1, throwC:-1 };
const _blendPose = { gun:[0,0,0], rot:[0,0,0], gripL:[0,0,0], gripR:[0,0,0], poleL:[0,0,0], poleR:[0,0,0] };

const _ikD=new THREE.Vector3(),_ikDir=new THREE.Vector3(),_ikAx=new THREE.Vector3(),_ikUA=new THREE.Vector3(),_ikLA=new THREE.Vector3(),_ikQ=new THREE.Quaternion(),_ikQ2=new THREE.Quaternion(),_ikDown=new THREE.Vector3(0,-1,0);
const _ikS=new THREE.Vector3(),_ikT=new THREE.Vector3(),_ikPole=new THREE.Vector3();
const ARM_L1=0.46, ARM_L2=0.42;   // upper arm, forearm+hand (CHAR_MALE gameplay body)
function solveArmIK(sh, el, S, T, pole, sign, L1, L2){
  if(L1===undefined)L1=ARM_L1; if(L2===undefined)L2=ARM_L2;   // per-character bone lengths (arm-length aware); fall back to defaults
  _ikD.subVectors(T,S); let dist=_ikD.length();
  const maxR=L1+L2-0.02, minR=Math.abs(L1-L2)+0.04;
  if(dist>maxR) dist=maxR; else if(dist<minR) dist=minR;
  _ikDir.copy(_ikD).setLength(1);
  const cS=Math.max(-1,Math.min(1,(L1*L1+dist*dist-L2*L2)/(2*L1*dist))), aS=Math.acos(cS);
  const cE=Math.max(-1,Math.min(1,(L1*L1+L2*L2-dist*dist)/(2*L1*L2))), bend=Math.PI-Math.acos(cE);
  _ikAx.copy(_ikDir).cross(pole); if(_ikAx.lengthSq()<1e-6)_ikAx.set(1,0,0); else _ikAx.setLength(1);
  _ikUA.copy(_ikDir).applyAxisAngle(_ikAx, aS*sign);
  _ikQ.setFromUnitVectors(_ikDown,_ikUA); sh.quaternion.copy(_ikQ);
  _ikQ2.copy(_ikQ).invert(); _ikLA.copy(_ikAx).applyQuaternion(_ikQ2);
  el.quaternion.setFromAxisAngle(_ikLA, -bend*sign);   // elbow folds opposite the shoulder's swing so the hand closes onto the target
}
// ================================ ANIM v2 ================================
// Weapon-first, foot-planted procedural animation. COSMETIC ONLY: the player fires from playerFireOrigin()
// and bots from a fixed chest point — never from the animated muzzle — so nothing here can change a hit.
// Per frame: (1) locomotion — feet planted in the world, swinging on arcs to the next foothold, leg IK;
// (2) upper body — twist budget, lean, breath, chest bladed toward the gun; (3) weapon stance — per-gun hold
// from GUN_KIT sockets, reach-aware support hand; (4) arm IK onto the sockets; (5) secondary — gun sway and
// recoil by mass, greatcoat panels following the thighs. Zero per-frame allocation.
const _aV=[], _aQ=[]; for(let i=0;i<10;i++){ _aV.push(new THREE.Vector3()); _aQ.push(new THREE.Quaternion()); }
const _aE=new THREE.Euler(0,0,0,'YXZ'), _aY=new THREE.Vector3(0,1,0), _aDn=new THREE.Vector3(0,-1,0), _aKnee=new THREE.Vector3(0,0,1);
const _aPoleR=new THREE.Vector3(), _aPoleL=new THREE.Vector3(), _aHold={ at:[0,0,0], rot:[0,0,0] };
function _aLerp(a,b,t){ return a+(b-a)*t; }
function _aWrap(a){ while(a>Math.PI)a-=6.283185307; while(a<-Math.PI)a+=6.283185307; return a; }
function _aSmooth(t){ t=t<0?0:t>1?1:t; return t*t*(3-2*t); }
const _aRk=[0,0,0];
function _aKeys(K, p, out){ let i=0; while(i<K.length-2&&K[i+1][0]<p) i++; const a=K[i], b=K[i+1], t=_aSmooth((p-a[0])/Math.max(1e-4,b[0]-a[0]));
  out[0]=a[1]+(b[1]-a[1])*t; out[1]=a[2]+(b[2]-a[2])*t; out[2]=a[3]+(b[3]-a[3])*t; return out; }
function _aNeutral(out, g, i, bS, bC, hipX, stag){ const sx=i?hipX:-hipX; return out.set(g.position.x+bC*sx+bS*stag, g.position.y, g.position.z-bS*sx+bC*stag); }
function _animInit(g){
  const j=g.userData.joints, pelY=g.userData.pelvisY||0.98, thigh=j.kneeL.position.length();
  let palm=0.08;   // fist centre beyond the wrist joint (so the fist, not the wrist, lands on a socket)
  { g.updateMatrixWorld(true); const hr=j.handR, hw=_aV[0]; hr.getWorldPosition(hw); let n=0, sum=0;
    hr.traverse(o=>{ if(!o.isMesh||o.userData.isOutline) return; let q=o, skip=false; while(q&&q!==hr){ if(q===g.userData.sword||q===g.userData.axe||q===g.userData.shield) skip=true; q=q.parent; }
      if(skip) return; o.getWorldPosition(_aV[1]); sum+=_aV[1].distanceTo(hw); n++; });
    if(n) palm=Math.max(0.03,Math.min(0.2,sum/n)); }
  const A={ v2:true, phase:Math.random(), prevX:g.position.x, prevZ:g.position.z, prevRy:g.rotation.y, vel:new THREE.Vector3(),
    sSpeed:{x:0,v:0}, sPY:{x:0,v:0}, sLean:{x:0,v:0}, sAds:{x:0,v:0}, sSpr:{x:0,v:0}, sSwY:{x:0,v:0}, sBob:{x:0,v:0},
    thigh, shin:Math.max(0.2,pelY+j.hipL.position.y-thigh), palm, armU:-j.elR.position.y, armF:-j.handR.position.y, pelY,
    feet:[0,1].map(()=>({ p:new THREE.Vector3(), s:new THREE.Vector3(), cur:new THREE.Vector3(), mode:0, t:0, dur:0.2, yaw:0, yaw0:0, pitch:0 })),
    leg:[null,null], sPW:{x:0,v:0}, sDrop:{x:0,v:0}, coat:[], aimPitch:0, wrx:NaN, init:false, supT:new THREE.Vector3(), hqR:j.handR.quaternion.clone(), hqL:j.handL.quaternion.clone(), fP:{x:0,v:0}, fR:{x:0,v:0}, thrW:0, thrF:-1, death:{ active:false, t:0, th:0, w:0, lx:0, lz:1, py:0 } };
  // procedural ANKLE: the rig has none (boots rode the shin, so they tilted into the ground). Re-hang the foot/boot parts
  // from a pivot at the top of the foot; each frame it is held flat, turned with the foot, rolled toe-off/heel-strike.
  for(let i=0;i<2;i++){ const kn=i?j.kneeR:j.kneeL, parts=[]; let top=-Infinity, bot=Infinity;
    let prev=null; for(let c=0;c<kn.children.length;c++) if(kn.children[c].userData.ankle) prev=kn.children[c];   // re-init: reuse the pivot
    if(prev){ A.leg[i]={ an:prev, h:prev.userData.ankle.h, len:prev.userData.ankle.len }; continue; }
    for(let c=0;c<kn.children.length;c++){ const o=kn.children[c], id=(o.userData&&o.userData.pid)||''; if(!o.isMesh||!/^(foot|bt_foot|bt_cuff)/.test(id)) continue;
      if(/^(foot|bt_foot)/.test(id)){ o.geometry.computeBoundingBox(); const bb=o.geometry.boundingBox; top=Math.max(top,o.position.y+bb.max.y*o.scale.y); bot=Math.min(bot,o.position.y+bb.min.y*o.scale.y); }
      parts.push(o); }
    if(!parts.length||!isFinite(top)) continue;
    const an=new THREE.Object3D(); an.position.set(0,top,0); kn.add(an);
    for(let c=0;c<parts.length;c++){ parts[c].position.y-=top; an.add(parts[c]); }
    an.userData.ankle={ h:top-bot, len:-top }; A.leg[i]={ an, h:top-bot, len:-top }; }
  const ids=[['gc_skFL',0,1],['gc_skFR',1,1],['gc_skBL',0,0],['gc_skBR',1,0]];
  for(let k=0;k<ids.length;k++){ let m=null; g.traverse(o=>{ if(!m&&o.userData&&o.userData.pid===ids[k][0]) m=o; });
    if(m){ if(m.material&&!m.material._coatDS){ m.material=m.material.clone(); m.material.side=THREE.DoubleSide; m.material._coatDS=true; }
      A.coat.push({ m, side:ids[k][1], front:ids[k][2], s:{x:0,v:0} }); } }
  g.userData.anim=A; return A;
}
function updateSoldierAnim(g, st, dt){
  const j=g.userData.joints; if(!j) return;
  const dtc=Math.max(1e-4, Math.min(0.05, dt)), v=_aV, Pl=j.pelvis;
  let A=g.userData.anim; if(!A||!A.v2) A=_animInit(g);
  if(A.death.active){ A.death.active=false; A.init=false; }   // alive again (respawn): re-plant the feet
  // ---- velocity (position delta, smoothed; teleports/respawns re-plant the feet) ----
  let dx=g.position.x-A.prevX, dz=g.position.z-A.prevZ; A.prevX=g.position.x; A.prevZ=g.position.z;
  const tele=Math.abs(dx)>25*dtc||Math.abs(dz)>25*dtc; if(tele){ dx=0; dz=0; }
  const kv=1-Math.exp(-14*dtc); A.vel.x+=(dx/dtc-A.vel.x)*kv; A.vel.z+=(dz/dtc-A.vel.z)*kv; A.vel.y=0;
  const rawSp=hyp2(A.vel.x,A.vel.z), sp=springStep(A.sSpeed, rawSp, ANIM.spr.gait.k, dtc);
  const ry=(st.aimYaw!=null)?st.aimYaw:g.rotation.y, fS=Math.sin(ry), fC=Math.cos(ry);
  const vF=A.vel.x*fS+A.vel.z*fC, vR=A.vel.x*fC-A.vel.z*fS;
  const grounded=st.grounded!==false, moving=rawSp>ANIM.idleSpeed&&grounded;
  const wRun=Math.max(0,Math.min(1,(rawSp-2.0)/Math.max(0.1,ANIM.walkRef-2.0)));   // gait weights track the REAL speed (a lagging
  const wSprint=Math.max(0,Math.min(1,(rawSp-ANIM.walkRef)/Math.max(0.1,ANIM.sprintRef-ANIM.walkRef)));   // cadence left feet behind on a run start)
  const wWalk=Math.min(1,sp/2.0), mv01=moving?1:0;
  // ---- stance weights (guns): aim (ADS) / sprint carry / low-ready carry ----
  const wp=g.userData.wpn, armM=g.userData.armMode, oneHand=armM==='shield', hasGun=(armM==='rifle'||oneHand)&&wp&&wp.visible!==false;
  const kit=hasGun?(GUN_KIT[g.userData.gunId]||GUN_KIT.amk):null, H=hasGun?(ANIM.hold[kit.hold]||ANIM.hold.rifle):null;
  const adsW=springStep(A.sAds, st.ads?1:0, ANIM.aim.adsK, dtc);
  const sprW=springStep(A.sSpr, (hasGun&&!st.ads&&(st.kick||0)<0.05)?wSprint:0, ANIM.spr.stance.k, dtc);
  const tsp=sprW*(1-adsW);
  const blade=H?_aLerp(_aLerp(H.carry.blade,H.ads.blade,adsW),H.sprint.blade,tsp):0;
  // ---- lower-body yaw: hips toward movement within the twist budget; backpedal keeps them to the aim ----
  //   continuous in the move angle (forward-facing fades out before the backpedal-facing fades in, so the hips never flip)
  //   and sprung in WORLD space (an aim flick doesn't snap the hips; they follow within 1.4 rad)
  let pyT=blade*0.45;                                    // standing: the hips take part of the shooter's blade
  if(moving){ const mvA=Math.atan2(vR,vF), am=Math.abs(mvA), tm=ANIM.twistMax;
    pyT=Math.max(-tm,Math.min(tm,mvA))*(1-_aSmooth((am-1.45)/0.75))+Math.max(-tm,Math.min(tm,_aWrap(mvA-Math.PI)))*_aSmooth((am-2.2)/0.6); }
  if(!A.init||tele){ A.sPW.x=g.rotation.y+pyT; A.sPW.v=0; }
  const over=Math.max(0,Math.abs(_aWrap(A.sPW.x-g.rotation.y))-ANIM.twistMax);                 // big twist (aim flick) = stiffer follow,
  springStep(A.sPW, A.sPW.x+_aWrap(g.rotation.y+pyT-A.sPW.x), ANIM.spr.legYaw.k*(1+4*over), dtc);   // so the hips swing round, never pop
  let pYaw=_aWrap(A.sPW.x-g.rotation.y); if(Math.abs(pYaw)>3.0){ pYaw=Math.sign(pYaw)*3.0; A.sPW.x=g.rotation.y+pYaw; }
  // ---- gait timing from leg reach (stride fits the legs at every speed) ----
  const Lg0=A.leg[0], ankH=Lg0?Lg0.h:0, L2=Lg0?Lg0.len:A.shin, chain=A.thigh+L2;
  const drop=springStep(A.sDrop, (_aLerp(ANIM.drop[0],ANIM.drop[1],wRun)+(ANIM.drop[2]-ANIM.drop[1])*wSprint)*mv01, ANIM.spr.drop.k, dtc);
  const Hv=A.pelY+j.hipL.position.y-drop-ankH, Lc=chain*0.97, R=Math.sqrt(Math.max(0.0025,Lc*Lc-Hv*Hv))*0.95;   // reach at this height
  const duty=_aLerp(ANIM.duty[0],ANIM.duty[1],wRun)+(ANIM.duty[2]-ANIM.duty[1])*wSprint;
  const cycle=Math.max(ANIM.cycleMin,Math.min(ANIM.cycleMax,2*R/(Math.max(rawSp,0.6)*duty)));
  if(moving) A.phase=(A.phase+dtc/cycle)%1;
  const lift=_aLerp(ANIM.lift[0],ANIM.lift[1],wRun), by=g.rotation.y+pYaw, bS=Math.sin(by), bC=Math.cos(by), baseY=g.position.y;
  const hipX=Math.abs(j.hipL.position.x)*ANIM.stanceW, stagA=moving?0:ANIM.idleStagger*(hasGun?1:0.4);
  if(!A.init||tele){ for(let i=0;i<2;i++){ const F=A.feet[i]; _aNeutral(F.p,g,i,bS,bC,hipX,(i?-1:1)*stagA); F.cur.copy(F.p); F.mode=0; F.yaw=by; F.pitch=0; } A.init=true; }
  // ---- feet: 0 planted, 1 gait swing (phase-driven), 2 timed step (idle re-step / catch-up) ----
  // A swing moves RELATIVE to the foot's moving stance spot (take-off offset -> landing offset), so a swinging foot
  // travels with the body and never trails behind it; a planted foot stays fixed in the world.
  const tp=ANIM.toePitch*lift, om=R*1.05; let lx=0, lz=0;
  if(moving){ const lt=0.5*duty*cycle; lx=A.vel.x*lt; lz=A.vel.z*lt; const ll=hyp2(lx,lz); if(ll>om){ lx*=om/ll; lz*=om/ll; } }   // landing offset
  for(let i=0;i<2;i++){ const F=A.feet[i]; _aNeutral(v[0],g,i,bS,bC,hipX,(i?-1:1)*stagA);
    if(!grounded){ F.mode=0; F.cur.copy(v[0]); F.cur.y=baseY+0.18; F.p.set(F.cur.x,baseY,F.cur.z); F.yaw=by; F.pitch=0.25; continue; }
    if(F.mode===2){ F.t+=dtc/F.dur; const s=Math.min(1,F.t), e=_aSmooth(s);
      F.cur.set(v[0].x+F.s.x+(lx-F.s.x)*e, baseY+(moving?lift:0.09)*Math.sin(Math.PI*s), v[0].z+F.s.z+(lz-F.s.z)*e);
      F.yaw=F.yaw0+_aWrap(by-F.yaw0)*e; F.pitch=0.6*tp*Math.sin(2*Math.PI*s);
      if(s>=1){ F.mode=0; F.p.set(F.cur.x,baseY,F.cur.z); F.yaw=by; F.pitch=0; } continue; }
    if(moving){
      const ph=(A.phase+(i?0.5:0))%1, swEnd=1-duty;
      if(ph<swEnd){ const s=ph/swEnd, e=_aSmooth(s);
        if(F.mode!==1){ F.mode=1; F.s.set(F.p.x-v[0].x,0,F.p.z-v[0].z); F.yaw0=F.yaw; }          // take-off offset
        F.cur.set(v[0].x+F.s.x+(lx-F.s.x)*e, baseY+lift*Math.sin(Math.PI*s), v[0].z+F.s.z+(lz-F.s.z)*e);
        F.yaw=F.yaw0+_aWrap(by-F.yaw0)*e; F.pitch=tp*Math.sin(2*Math.PI*s); }
      else { if(F.mode===1){ F.mode=0; F.p.set(F.cur.x,baseY,F.cur.z); F.yaw=by; }
        F.cur.copy(F.p); F.pitch=0;
        // catch-up: a planted foot left beyond reach (sharp turn, stop-start, knock-back) steps now instead of stretching the leg
        const sx=i?hipX:-hipX, hx=g.position.x+bC*sx, hz=g.position.z-bS*sx;
        if(hyp2(F.p.x-hx,F.p.z-hz)>Math.max(0.35,R*1.5)){ F.mode=2; F.t=0; F.dur=0.14; F.s.set(F.p.x-v[0].x,0,F.p.z-v[0].z); F.yaw0=F.yaw; } }
    } else {
      if(F.mode===1){ F.mode=2; F.t=0; F.dur=ANIM.idleStepT; F.s.set(F.cur.x-v[0].x,0,F.cur.z-v[0].z); F.yaw0=F.yaw; }
      else { F.cur.copy(F.p); F.pitch=0; }
    } }
  if(!moving&&grounded&&A.feet[0].mode===0&&A.feet[1].mode===0){   // the foot that strays furthest (or turned most) steps back under the body
    let worst=-1, wd=ANIM.restepDist;
    for(let i=0;i<2;i++){ _aNeutral(v[0],g,i,bS,bC,hipX,(i?-1:1)*stagA);
      const d=hyp2(A.feet[i].p.x-v[0].x,A.feet[i].p.z-v[0].z)+Math.max(0,Math.abs(_aWrap(A.feet[i].yaw-by))-0.6)*0.5; if(d>wd){ wd=d; worst=i; } }
    if(worst>=0){ const F=A.feet[worst]; _aNeutral(v[0],g,worst,bS,bC,hipX,(worst?-1:1)*stagA); F.mode=2; F.t=0; F.dur=ANIM.idleStepT; F.s.set(F.p.x-v[0].x,0,F.p.z-v[0].z); F.yaw0=F.yaw; } }
  // ---- pelvis: bob (low at mid-stance), drop at speed, sway + roll; capped so both feet stay reachable ----
  const bob=-_aLerp(ANIM.pelvisBob[0],ANIM.pelvisBob[1],wRun)*mv01*Math.cos(4*Math.PI*(A.phase-(1-duty*0.5)));
  let pY=A.pelY-drop+bob; const Lcap=chain*0.99;
  for(let i=0;i<2;i++){ const F=A.feet[i], sx=i?hipX:-hipX, hx=g.position.x+bC*sx, hz=g.position.z-bS*sx, dh=hyp2(F.cur.x-hx,F.cur.z-hz);
    const mh=(F.cur.y-baseY)+ankH+Math.sqrt(Math.max(0.01,Lcap*Lcap-dh*dh))-j.hipL.position.y; if(mh<pY) pY=mh; }
  pY=Math.max(A.pelY-0.18,pY);                            // safety net only: the stride is sized so this rarely engages
  Pl.position.set(ANIM.pelvisSway*Math.sin(2*Math.PI*A.phase)*wWalk*mv01, pY, 0);
  Pl.rotation.set(0, pYaw, ANIM.pelvisRoll*Math.sin(2*Math.PI*A.phase)*wWalk*mv01);
  // ---- leg IK: plant each foot (feet in pelvis space; knees bend forward) ----
  _aQ[0].copy(Pl.quaternion).invert();
  for(let i=0;i<2;i++){ const F=A.feet[i], hip=i?j.hipR:j.hipL, knee=i?j.kneeR:j.kneeL, Lg=A.leg[i];
    v[2].copy(F.cur); if(Lg) v[2].y+=Lg.h;                                                     // IK target = the ankle above the sole
    v[2].sub(g.position).applyAxisAngle(_aY,-g.rotation.y).sub(Pl.position).applyQuaternion(_aQ[0]);
    solveArmIK(hip, knee, hip.position, v[2], _aKnee, 1, A.thigh, Lg?Lg.len:A.shin);
    if(Lg){ _aQ[3].setFromAxisAngle(_aY,g.rotation.y).multiply(Pl.quaternion).multiply(hip.quaternion).multiply(knee.quaternion).invert();
      _aE.set(F.pitch, F.yaw+(i?1:-1)*ANIM.toeOut, 0, 'YXZ'); _aQ[4].setFromEuler(_aE); Lg.an.quaternion.copy(_aQ[3]).multiply(_aQ[4]); } }
  // ---- upper body: counter-twist to the aim (+ blade), lean, breath, stride roll; head faces the aim ----
  const lean=springStep(A.sLean,(ANIM.walkLean*wWalk+ANIM.sprintLean*wSprint)*mv01,ANIM.spr.lean.k,dtc);
  const breath=(1-mv01)*Math.sin((st.t||0)*ANIM.breathRate)*ANIM.breath, kick=st.kick||0, dip=st.dip||0, twist=blade-pYaw;
  j.spine.rotation.set(lean+(st.lean||0)*0.4, twist*0.45, 0);
  j.chest.rotation.set(breath-kick*ANIM.recoil.chest+dip*0.13, twist*0.55, -Math.sin(2*Math.PI*A.phase)*ANIM.torsoRoll*wWalk*mv01);
  j.neck.rotation.set(-breath*0.6-lean*0.5+kick*0.07+dip*0.15, -blade*0.85, 0);
  // ---- weapon stance (gun holds): place the GUN first, then both hands solve onto its sockets ----
  if(hasGun){
    if(wp.rotation.x!==A.wrx) A.aimPitch=wp.rotation.x;   // pitchWeapon() wrote a fresh aim pitch this frame
    const TW=ANIM.throw; A.thrW+=(((st.throwC!=null&&st.throwC>=0)?1:0)-A.thrW)*(1-Math.exp(-12*dtc));        // grenade wind-up
    if(A.thrF>=0){ A.thrF+=dtc/TW.t; if(A.thrF>=1) A.thrF=-1; }                                                // throw follow-through
    const tw=Math.max(A.thrW,A.thrF>=0?1:0), ch=Math.min(1,st.lean||0), dw=_aSmooth(st.swap||0), Dr=ANIM.draw, Cg=ANIM.charge;
    const C=H.carry, D=H.ads, S=H.sprint, h=_aHold, ta=adsW, aimP=A.aimPitch;
    for(let k=0;k<3;k++) h.at[k]=_aLerp(_aLerp(C.at[k],D.at[k],ta),S.at[k],tsp);
    h.rot[0]=_aLerp(_aLerp(C.rot[0]+aimP*0.35,(D.rot[0]==='aim'?aimP:D.rot[0]),ta),S.rot[0],tsp);
    h.rot[1]=_aLerp(_aLerp(C.rot[1],D.rot[1],ta),S.rot[1],tsp); h.rot[2]=_aLerp(_aLerp(C.rot[2],D.rot[2],ta),S.rot[2],tsp);
    if(oneHand){ h.rot[0]+=0.45; h.at[0]+=0.04; h.at[1]-=0.14; }                                               // shield up: gun carried one-handed, low
    const RL=kit.reload, RS=(RL&&st.rel!=null&&st.rel>=0)?ANIM.reload.styles[RL.style]:null;
    if(RS){ const rp=st.rel, gw=_aSmooth(rp/0.12)*(1-_aSmooth((rp-0.80)/0.16)), P=RS.pose; _aKeys(RS.keys,rp,_aRk);   // reload choreography
      h.rot[0]+=P[0]*gw; h.rot[1]+=P[1]*gw; h.rot[2]+=P[2]*gw; h.at[0]+=P[3]*gw; h.at[1]+=P[4]*gw; h.at[2]+=P[5]*gw; }
    const dipG=RS?0:dip;                                                                                     // (replaces the simple reload dip)
    const mass=kit.mass, dYaw=_aWrap(ry-A.prevRy)/dtc; A.prevRy=ry;
    const swY=springStep(A.sSwY, Math.max(-ANIM.sway.max,Math.min(ANIM.sway.max,-dYaw*ANIM.sway.turn*mass)), ANIM.spr.sway.k/mass, dtc);
    const swB=springStep(A.sBob, -bob*ANIM.sway.bob, ANIM.spr.sway.k/mass, dtc);
    h.rot[0]+=-kick*ANIM.recoil.pitch*mass+dipG*0.55+dw*Dr.pitch+ch*Cg.pitch+tw*0.25; h.rot[1]+=swY+ch*Cg.yaw; h.rot[2]+=dipG*0.45+dw*Dr.roll;
    _aE.set(h.rot[0],h.rot[1],h.rot[2],'YXZ'); _aQ[1].setFromEuler(_aE);                                   // gun orientation, aim frame
    _aQ[2].copy(Pl.quaternion).multiply(j.spine.quaternion).multiply(j.chest.quaternion).invert();        // aim frame -> chest space
    wp.quaternion.copy(_aQ[2]).multiply(_aQ[1]);
    v[3].set(h.at[0]+dw*Dr.at[0]+ch*Cg.at[0], h.at[1]+swB-dipG*0.08+dw*Dr.at[1]+ch*Cg.at[1]-tw*0.08, h.at[2]+dw*Dr.at[2]+ch*Cg.at[2]).applyQuaternion(_aQ[2]);                                // stock anchor, chest space
    v[4].set(0,0,-kick*ANIM.recoil.back*mass).applyQuaternion(wp.quaternion); v[3].add(v[4]);              // recoil along the barrel
    v[5].fromArray(kit.stock).applyQuaternion(wp.quaternion); wp.position.copy(v[3]).sub(v[5]);
    const reach=ANIM.reachFrac*(A.armU+A.armF+A.palm);
    v[6].fromArray(kit.grip).applyQuaternion(wp.quaternion).add(wp.position);                              // trigger hand target
    const dG=v[6].distanceTo(j.shR.position);
    if(dG>reach){ v[7].copy(j.shR.position).sub(v[6]).multiplyScalar((dG-reach)/dG); wp.position.add(v[7]); v[6].add(v[7]); }
    v[8].set(0,0,1).applyQuaternion(wp.quaternion); v[9].set(0,kit.rail[2],0).applyQuaternion(wp.quaternion).add(wp.position);
    const Sx=j.shL.position.x-v[9].x, Sy=j.shL.position.y-v[9].y, Sz=j.shL.position.z-v[9].z;
    const b0=Sx*v[8].x+Sy*v[8].y+Sz*v[8].z, disc=b0*b0-(Sx*Sx+Sy*Sy+Sz*Sz-reach*reach), r0=kit.rail[0], r1=kit.rail[1];
    let zs=r0;                                                                                             // furthest rail point the left arm reaches
    if(disc>=0){ const sq=Math.sqrt(disc), zLo=b0-sq, zHi=b0+sq; zs=zHi<r0?r0:(zLo>r1?r1:Math.min(r1,zHi)); if(zs<r0) zs=r0; }
    else zs=Math.max(r0,Math.min(r1,b0));
    v[9].addScaledVector(v[8],zs);
    if(RS){ const wP=_aRk[0], wS=_aRk[1];                                                                   // support hand: rail / pouch / reload point
      v[4].fromArray(RL.style==='nock'?ANIM.reload.quiver:ANIM.reload.pouch);
      v[5].fromArray(RL.at).applyQuaternion(wp.quaternion).add(wp.position);
      v[7].fromArray(RS.dir).applyQuaternion(wp.quaternion); v[5].addScaledVector(v[7],_aRk[2]);
      v[9].multiplyScalar(Math.max(0,1-wP-wS)).addScaledVector(v[4],wP).addScaledVector(v[5],wS); }
    if(A.thrW>0.001||A.thrF>=0){ const f=A.thrF; v[4].fromArray(TW.wind); let w=_aSmooth(A.thrW);                  // support hand throws:
      if(f>=0){ if(f>=0.25){ v[5].fromArray(TW.release); v[4].lerp(v[5],_aSmooth((f-0.25)/0.3)); }                // wind -> release -> back
        w=f<0.25?Math.max(w,_aSmooth(f/0.25)):(f<0.55?1:1-_aSmooth((f-0.55)/0.45)); }
      v[9].lerp(v[4],w); }
    A.supT.copy(v[9]);                                                                                      // (support target, chest space: tools/metrics)
    j.handR.quaternion.copy(A.hqR); if(!oneHand) j.handL.quaternion.copy(A.hqL);                                  // hands at rest on the gun
    _aPoleR.fromArray(ANIM.poleR).normalize(); _aPoleL.fromArray(ANIM.poleL).normalize();
    solveArmIK(j.shR, j.elR, j.shR.position, v[6], _aPoleR, 1, A.armU, A.armF+A.palm);
    if(!oneHand) solveArmIK(j.shL, j.elL, j.shL.position, v[9], _aPoleL, 1, A.armU, A.armF+A.palm);
    A.wrx=wp.rotation.x;
  } else A.prevRy=ry;
  // ---- shield (left arm) + katana (both hands): hold the prop by setting the HAND's orientation, then IK the arm ----
  const Ud=g.userData, mel=(armM==='melee')?((Ud.sword&&Ud.sword.visible)?Ud.sword:((Ud.axe&&Ud.axe.visible)?Ud.axe:null)):null;
  if((oneHand&&Ud.shield)||mel){
    _aQ[7].copy(Pl.quaternion).multiply(j.spine.quaternion).multiply(j.chest.quaternion).invert();             // aim frame -> chest space
    _aPoleR.fromArray(ANIM.poleR).normalize(); _aPoleL.fromArray(ANIM.poleL).normalize();
    if(oneHand&&Ud.shield){ const Sd=ANIM.shield;                                                                // shield: face the aim, level
      _aE.set(Sd.pitch,Sd.yaw,0,'YXZ'); _aQ[8].setFromEuler(_aE); _aQ[9].copy(_aQ[7]).multiply(_aQ[8]);             // hand orientation (chest)
      v[0].fromArray(Sd.at).applyQuaternion(_aQ[7]).sub(v[1].copy(Ud.shield.position).applyQuaternion(_aQ[9]));    // wrist = centre - offset
      solveArmIK(j.shL, j.elL, j.shL.position, v[0], _aPoleL, 1, A.armU, A.armF);
      j.handL.quaternion.copy(j.shL.quaternion).multiply(j.elL.quaternion).invert().multiply(_aQ[9]); }
    if(mel){ const Kt=ANIM.katana;                                                                                // katana: guard + slash
      v[0].set(Kt.at[0]+Math.sin(mel.rotation.y)*Kt.sweep, Kt.at[1], Kt.at[2]).applyQuaternion(_aQ[7]);             // right wrist travels with the swing
      _aE.set(Kt.pitch,Kt.yaw,0,'YXZ'); _aQ[8].setFromEuler(_aE); _aQ[9].copy(_aQ[7]).multiply(_aQ[8]);
      solveArmIK(j.shR, j.elR, j.shR.position, v[0], _aPoleR, 1, A.armU, A.armF);
      j.handR.quaternion.copy(j.shR.quaternion).multiply(j.elR.quaternion).invert().multiply(_aQ[9]);
      v[1].fromArray(Kt.hilt).applyQuaternion(mel.quaternion).add(mel.position).applyQuaternion(_aQ[9]).add(v[0]);  // left fist on the hilt
      j.handL.quaternion.copy(A.hqL); solveArmIK(j.shL, j.elL, j.shL.position, v[1], _aPoleL, 1, A.armU, A.armF+A.palm); } }
  // ---- hit flinch: the upper body (gun and hands with it) snaps away from the hit, the head whips ----
  const fp=springStep(A.fP,0,ANIM.flinch.k,dtc), fr=springStep(A.fR,0,ANIM.flinch.k,dtc);
  if(fp!==0||fr!==0){ j.spine.rotation.x+=fp*ANIM.flinch.pitch; j.chest.rotation.x+=fp*(1-ANIM.flinch.pitch); j.chest.rotation.z+=fr*ANIM.flinch.roll; j.neck.rotation.x-=fp*ANIM.flinch.head; }
  // ---- greatcoat panels: pushed by the thigh on their side (front/back), a touch of flare at speed ----
  if(A.coat.length){ const K=ANIM.coat, flare=K.flare*wRun*mv01;
    for(let k=0;k<A.coat.length;k++){ const Cp=A.coat[k]; v[0].copy(_aDn).applyQuaternion((Cp.side?j.hipR:j.hipL).quaternion);
      const a=Math.atan2(v[0].z,-v[0].y), tgt=Cp.front?-(Math.max(0,a)*K.front+flare):(Math.max(0,-a)*K.back+flare);
      Cp.m.rotation.x=springStep(Cp.s, Math.max(K.min,Math.min(K.max,tgt)), ANIM.spr.coat.k, dtc); } }
}

// ---- action hooks (cosmetic) --------------------------------------------------------------------------------------
// Hit reaction: push the upper body away from the source; strength ~ damage/40, clamped 0.2..1.
function animHit(g, fromX, fromZ, strength){ const A=g.userData.anim; if(!A||!A.v2) return;
  let dx=g.position.x-fromX, dz=g.position.z-fromZ; const l=hyp2(dx,dz); if(!(l>1e-3)) return; dx/=l; dz/=l;
  const ry=g.rotation.y, sn=Math.sin(ry), cs=Math.cos(ry), k=ANIM.flinch.kick*Math.max(0.2,Math.min(1,strength));
  A.fP.v+=(dx*sn+dz*cs)*k; A.fR.v+=-(dx*cs-dz*sn)*k; }
// Throw follow-through (grenade release, molotov): the support hand winds (if not already) -> releases -> returns.
function animThrow(g){ const A=g.userData.anim; if(A&&A.v2) A.thrF=(A.thrW>0.5)?0.25:0; }
// Death: a toy-soldier fall away from the attacker. Knees buckle into a kneel (feet stay put), the upper body tips over
// from the pelvis under gravity, bounces once and settles on the ground. deathStart() on the kill, animDeath() every dead
// frame; the next regular anim tick (respawn) clears it. The root never tilts or sinks, so platforms stay correct.
function deathStart(g, fromX, fromZ){ const A=g.userData.anim; if(!A||!A.v2) return; const D=A.death;
  let dx=g.position.x-fromX, dz=g.position.z-fromZ, l=hyp2(dx,dz); if(!(l>1e-3)){ const a=Math.random()*6.283; dx=Math.sin(a); dz=Math.cos(a); l=1; }
  dx/=l; dz/=l; const ry=g.rotation.y, sn=Math.sin(ry), cs=Math.cos(ry);
  D.lx=dx*cs-dz*sn; D.lz=dx*sn+dz*cs; D.t=0; D.th=0.04; D.w=ANIM.death.push; D.py=g.userData.joints.pelvis.rotation.y; D.active=true; }
function animDeath(g, dt){ const A=g.userData.anim, j=g.userData.joints; if(!A||!j||!A.death.active) return;
  const D=A.death, K=ANIM.death, Pl=j.pelvis, v=_aV, dtc=Math.max(1e-4,Math.min(0.05,dt)); D.t+=dtc;
  const b=_aSmooth(D.t/K.buckleT);
  if(D.t>K.buckleT*0.5){ D.w+=K.g*Math.sin(D.th+0.1)*dtc; D.th+=D.w*dtc;                              // inverted pendulum
    if(D.th>=K.lie){ D.th=K.lie; D.w=Math.abs(D.w)>0.4?-Math.abs(D.w)*K.bounce:0; } if(D.th<0.02){ D.th=0.02; if(D.w<0) D.w=0; } }
  const tl=D.th/K.lie;
  Pl.position.set(0, A.pelY-(A.pelY-K.kneelY)*b-(K.kneelY-K.lieY)*Math.sin(Math.min(D.th,1.5707)), 0);
  v[0].set(D.lz,0,-D.lx); _aQ[5].setFromAxisAngle(v[0],D.th); _aQ[6].setFromAxisAngle(_aY,D.py); Pl.quaternion.copy(_aQ[5]).multiply(_aQ[6]);
  _aQ[0].copy(Pl.quaternion).invert(); const hipX=Math.abs(j.hipL.position.x)*ANIM.stanceW;
  for(let i=0;i<2;i++){ const hip=i?j.hipR:j.hipL, knee=i?j.kneeR:j.kneeL, Lg=A.leg[i];
    v[2].set(i?hipX:-hipX, Lg?Lg.h:0, -0.34*b).sub(Pl.position).applyQuaternion(_aQ[0]);               // feet tucked behind the kneel (root space)
    solveArmIK(hip, knee, hip.position, v[2], _aKnee, 1, A.thigh, Lg?Lg.len:A.shin);
    if(Lg){ _aQ[3].copy(Pl.quaternion).multiply(hip.quaternion).multiply(knee.quaternion).invert(); _aE.set(0.9*b,(i?1:-1)*ANIM.toeOut,0,'YXZ'); _aQ[4].setFromEuler(_aE); Lg.an.quaternion.copy(_aQ[3]).multiply(_aQ[4]); } }
  j.spine.rotation.x=0.25*tl; j.neck.rotation.x=-0.35*tl;
  for(let k=0;k<A.coat.length;k++){ const Cp=A.coat[k]; Cp.m.rotation.x=springStep(Cp.s, Cp.front?-0.35*tl:0.25*tl, ANIM.spr.coat.k, dtc); } }

/* Apply a rifle HOLD pose (gun transform + both-hand IK) from a pose object {gun,rot,gripL,gripR,poleL,poleR}.
   Used by the pose-authoring tool now, and by the aim layer's carry<->ADS blend later. Cosmetic. */
function applyRifleHold(group, pose){
  const j=group.userData.joints, wp=group.userData.wpn; if(!j||!wp||!pose) return;
  wp.position.set(pose.gun[0],pose.gun[1],pose.gun[2]);
  wp.rotation.set(pose.rot[0],pose.rot[1],pose.rot[2]);
  const th=wp.rotation.x, ct=Math.cos(th), stt=Math.sin(th), px=wp.position.x, py=wp.position.y, pz=wp.position.z;
  const _L1=-j.elR.position.y, _L2=-j.handR.position.y;
  const gR=pose.gripR; _ikT.set(px+gR[0], py+gR[1]*ct-gR[2]*stt, pz+gR[1]*stt+gR[2]*ct);
  _ikS.set(0.6,0.06,0.04); _ikPole.set(pose.poleR[0],pose.poleR[1],pose.poleR[2]); solveArmIK(j.shR,j.elR,_ikS,_ikT,_ikPole,1,_L1,_L2);
  const gL=pose.gripL; _ikT.set(px+gL[0], py+gL[1]*ct-gL[2]*stt, pz+gL[1]*stt+gL[2]*ct);
  _ikS.set(-0.6,0.06,0.04); _ikPole.set(pose.poleL[0],pose.poleL[1],pose.poleL[2]); solveArmIK(j.shL,j.elL,_ikS,_ikT,_ikPole,1,_L1,_L2);
}

/* buildings */
const buildings=[];
function makeBuilding(team,type,x,z){
  team=teamOf(team); const isF=team!==null&&team===PLAYER_TEAM; const accent=isF?0x38e1b0:0xff4d4d;   // canonical id; teamless stays red (N3)
  const dims=type==='hq'?[8,9,8]:type==='refinery'?[10,4,6]:type==='power'?[6,8,5]:[7,5,7];
  const base=isF?0x1f3633:0x361f1f;
  const m=new THREE.Mesh(new THREE.BoxGeometry(dims[0],dims[1],dims[2]),new THREE.MeshStandardMaterial({color:base,roughness:0.85,metalness:0.2}));
  m.position.set(x,dims[1]/2,z); m.castShadow=true; m.receiveShadow=true; scene.add(m);
  const trim=new THREE.Mesh(new THREE.BoxGeometry(dims[0]+0.12,0.55,dims[2]+0.12),new THREE.MeshStandardMaterial({color:accent,emissive:accent,emissiveIntensity:0.65,roughness:0.4}));
  trim.position.set(x,dims[1]-0.7,z); noRay(trim); scene.add(trim); if(typeof stylizeOne==='function'){ stylizeOne(m); stylizeOne(trim); }
  const label=makeLabel(type.toUpperCase(),isF?'#7af0d2':'#ff8a8a'); label.position.set(x,dims[1]+1.5,z); scene.add(label);
  const bg=new THREE.Mesh(new THREE.PlaneGeometry(dims[0]*0.9,0.4),new THREE.MeshBasicMaterial({color:0x101616})); noRay(bg);
  const fg=new THREE.Mesh(new THREE.PlaneGeometry(dims[0]*0.9,0.4),new THREE.MeshBasicMaterial({color:accent})); fg.position.z=0.01; noRay(fg);
  const bar=new THREE.Group(); bar.add(bg,fg); bar.position.set(x,dims[1]+0.7,z); scene.add(bar);
  const hp=type==='hq'?T.hqHealth:type==='refinery'?T.refHealth:type==='power'?T.powerHealth:T.barHealth;
  const b={team,type,mesh:m,trim,label,bar:{grp:bar,fg,w:dims[0]*0.9},health:hp,maxHealth:hp,alive:true,sink:0,box:new THREE.Box3().setFromObject(m)};
  m.userData.buildingRef=b; m.userData.editSel={kind:'building',ref:b}; buildings.push(b); return b;
}
/* buildings created by buildMap() */
function findB(team,type){return buildings.find(b=>b.team===team&&b.type===type);}
// Team identity, one choke point (three-teams item 2): canonical ids from TEAMS; legacy 'friend' / 'hostile' and faction
// names map to them; 'neutral', empty and unknown values mean no team (null). Explicit comparisons only: no `in` test or
// object lookup, so prototype keys such as 'toString' can never pass.
function teamOf(x){ if(x==null||x===''||x==='neutral')return null;
  for(let i=0;i<TEAMS.length;i++){ const t=TEAMS[i]; if(x===t.id||x===t.faction)return t.id; }
  if(x==='friend')return 'allies'; if(x==='hostile')return 'soviets';
  if(DEV.enabled)console.error('teamOf: unknown team',typeof x==='string'?x.slice(0,40):typeof x); return null; }   // map data: log a capped string, never the raw value
function sameTeam(a,b){ const ta=teamOf(a); return ta!==null&&ta===teamOf(b); }   // both have a team and it is the same one
function teamFilter(team){ return TUNE.ff?null:team; }   // resolveShot's shooterTeam: teammates are skipped unless friendly fire is on

let terminalPos=new THREE.Vector3(0,0,34), mapSpawn=new THREE.Vector3(0,0,30), termMesh=null, resupplyLabel=null;

/* enterable structure — The Barracks (Renegade-style interior: roof hides while you're inside) */
const wallMat=new THREE.MeshStandardMaterial({color:0x32413f,roughness:0.9,metalness:0.1});
const bRoofMat=new THREE.MeshStandardMaterial({color:0x28332f,roughness:0.95,metalness:0.05,transparent:true,opacity:1});
const bFloorMat=new THREE.MeshStandardMaterial({color:0x1b2523,roughness:1});
const barracksList=[];
function makeBarracks(cx,cz,team){
  const W=11,D=11,H=5,t=0.6,dw=3.4,seg=(W-dw)/2;
  team=teamOf(team); const rel=team===null?0:team===PLAYER_TEAM?1:2;   // canonical id or null; teamless stays grey
  const accent=rel===2?0xff4d4d:rel===1?0x38e1b0:0x9fb0ad;
  const txtCol=rel===2?'#ff8a8a':rel===1?'#7af0d2':'#cfe6e2';
  const walls=[ addBox(cx,cz-D/2,W,H,t,wallMat,false), addBox(cx+W/2,cz,t,H,D,wallMat,false), addBox(cx-W/2,cz,t,H,D,wallMat,false), addBox(cx-(dw/2+seg/2),cz+D/2,seg,H,t,wallMat,false), addBox(cx+(dw/2+seg/2),cz+D/2,seg,H,t,wallMat,false) ];
  const lintel=new THREE.Mesh(new THREE.BoxGeometry(dw,1.1,t),wallMat); lintel.position.set(cx,H-0.55,cz+D/2); lintel.castShadow=true; noRay(lintel); scene.add(lintel);
  const floor=new THREE.Mesh(new THREE.BoxGeometry(W-t,0.08,D-t),bFloorMat); floor.position.set(cx,0.04,cz); floor.receiveShadow=true; noRay(floor); scene.add(floor);
  const roofM=bRoofMat.clone();
  const roof=new THREE.Mesh(new THREE.BoxGeometry(W+0.4,t,D+0.4),roofM); roof.position.set(cx,H+t/2,cz); roof.castShadow=true; noRay(roof); scene.add(roof);
  const label=makeLabel('BARRACKS',txtCol); label.position.set(cx,H+1.6,cz); label.scale.set(4.4,1.05,1); scene.add(label);
  // Renegade-style purchase panel mounted on the back wall (faces +z into the room)
  const panel=new THREE.Group();
  const frame=new THREE.Mesh(new THREE.BoxGeometry(2.0,1.5,0.14),new THREE.MeshStandardMaterial({color:0x10201d,roughness:0.6,metalness:0.3}));
  const screen=new THREE.Mesh(new THREE.BoxGeometry(1.7,1.2,0.05),new THREE.MeshStandardMaterial({color:accent,emissive:accent,emissiveIntensity:0.75,roughness:0.3})); screen.position.z=0.08;
  panel.add(frame,screen); panel.position.set(cx,1.55,cz-D/2+t/2+0.12); panel.traverse(o=>noRay(o)); scene.add(panel);
  const plabel=makeLabel('ARMORY',txtCol); plabel.position.set(cx,2.62,cz-D/2+t/2+0.2); plabel.scale.set(2.6,0.7,1); scene.add(plabel);
  const panelPos=new THREE.Vector3(cx,0,cz-D/2+1.0);
  if(typeof stylizeOne==='function'){ stylizeOne(lintel); stylizeOne(floor); stylizeOne(roof); stylizeOne(panel); }   // runtime builder: stylizeScene only ran at boot
  const ent={cx,cz,team,iw:W/2-t,id:D/2-t,walls,roof,roofM,label,lintel,floor,panel,plabel,panelPos,op:1};
  walls.forEach(w=>{ w.mesh.userData.editSel={kind:'barracks',ref:ent}; }); // click a wall to select the barracks
  barracksList.push(ent); return ent;
}
// Collision: resolveObstacles(pos,r) handles crates/walls (zero-alloc); buildings/platforms/ramps are handled by the vertical step-guard in tickVertical (walk-on-top). Shared by player + enemies.
let losMeshes=[]; function rebuildLOS(){ losMeshes=obstacles.map(o=>o.mesh).concat(buildings.map(b=>b.mesh)).concat(houses.map(h=>h.body)).concat(covers.filter(c=>c.los&&c.body).map(c=>c.body)); }

const _sharedMapMats=new Set([crateMat,crateMatWarm,platMat,wallMat,bFloorMat]);
function _disposeObj3D(root,seen){ if(!root)return; root.traverse(o=>{
  const g=o.geometry; if(g&&!seen.has(g)){ seen.add(g); g.dispose(); }
  const m=o.material; if(!m)return; const arr=Array.isArray(m)?m:[m];
  for(const mm of arr){ if(mm&&!_sharedMapMats.has(mm)&&!seen.has(mm)){ seen.add(mm); if(mm.map)mm.map.dispose(); mm.dispose(); } } }); }
function clearMap(){ NAV.ok=false;
  const seen=new Set();
  obstacles.forEach(o=>{ scene.remove(o.mesh); _disposeObj3D(o.mesh,seen); }); obstacles.length=0;
  buildings.forEach(b=>{ [b.mesh,b.trim,b.label,b.bar&&b.bar.grp].forEach(x=>{ if(x){ scene.remove(x); _disposeObj3D(x,seen); } }); }); buildings.length=0;
  barracksList.forEach(b=>{ [b.roof,b.floor,b.label,b.lintel,b.panel,b.plabel].forEach(x=>{ if(x){ scene.remove(x); _disposeObj3D(x,seen); } }); }); barracksList.length=0;
  paths.forEach(o=>{ scene.remove(o.mesh); _disposeObj3D(o.mesh,seen); }); paths.length=0;
  platforms.forEach(p=>{ scene.remove(p.mesh); _disposeObj3D(p.mesh,seen); }); platforms.length=0;
  ramps.forEach(r=>{ scene.remove(r.mesh); _disposeObj3D(r.mesh,seen); }); ramps.length=0;
  houses.forEach(h=>{ scene.remove(h.grp); _disposeObj3D(h.grp,seen); }); houses.length=0;
  covers.forEach(o=>{ scene.remove(o.grp); _disposeObj3D(o.grp,seen); }); covers.length=0;
  trees.forEach(o=>{ scene.remove(o.grp); _disposeObj3D(o.grp,seen); }); trees.length=0;   // were never cleared: every rebuild (undo, redo,
  props.forEach(o=>{ scene.remove(o.grp); _disposeObj3D(o.grp,seen); }); props.length=0;   // reset, load) stacked another copy of them all
  enemySpawns.forEach(sp=>{ enemySpawnLayer.remove(sp.grp); _disposeObj3D(sp.grp,seen); }); enemySpawns.length=0;
  if(termMesh){ scene.remove(termMesh); _disposeObj3D(termMesh,seen); termMesh=null; } if(resupplyLabel){ scene.remove(resupplyLabel); _disposeObj3D(resupplyLabel,seen); resupplyLabel=null; }
}
// ===== Living Comic architecture (ART_DIRECTION §6) — first building: half-timbered house (Fachwerk) =====
const houses=[];   // placeable decorative buildings (own collider + serialize; mirrors the obstacle/platform pattern)
// Canvas texture: cream plaster + dark near-black Fachwerk timber frame (border + rails + verticals + diagonal braces).
function placeLegacyHouse(o, doLOS){ scene.add(o.grp); houses.push(o); stylizeOne(o.grp); if(o.body) o.body.userData.noMerge=true; mergeStaticGroup(o.grp); if(doLOS && typeof rebuildLOS==='function') rebuildLOS(); return o; }
function spawnKitBuilding(spec, x, z, rot){
  const built=BK.build(spec), g=built.group; g.position.set(x,0,z); if(rot)g.rotation.y=rot; scene.add(g); g.updateMatrixWorld(true);
  const o={ grp:g, body:built.body, localColliders:built.colliders, boxes:[], x, z, rot:rot||0, kind:'kit', spec:Object.assign({},spec) };
  _bkWorldBoxes(o); g.userData.editSel={kind:'house',ref:o}; houses.push(o);
  if(typeof stylizeOne==='function') stylizeOne(g);   // toon + BOLD world outline (runtime object)
  if(built.body) built.body.userData.noMerge=true; mergeStaticGroup(g);   // fewer draw calls (static piece)
  if(typeof rebuildLOS==='function') rebuildLOS();     // include in blast/occlusion mesh LOS
  return o;
}

// ---- Cover glue: gap-free chain of small AABBs along the run (tight even on diagonals; reuses the compound-box path) ----
function _coverWorldBoxes(o){ const L=o.len, t=o.thickness, h=o.height, step=Math.max(0.6,t);
  let n=Math.max(1,Math.round(L/step)); n=Math.min(n,48); const seg=L/n, half=Math.max(t*0.5+0.05, seg*0.5+0.02);
  const c=Math.cos(o.rot), s=Math.sin(o.rot); o.boxes=o.boxes||[];
  for(let i=0;i<n;i++){ const tt=-L/2+seg*(i+0.5), wx=o.cx+tt*c, wz=o.cz-tt*s; let b=o.boxes[i]||(o.boxes[i]=new THREE.Box3()); b.min.set(wx-half,0,wz-half); b.max.set(wx+half,h,wz+half); }
  o.boxes.length=n; }
function addCover(type, cx, cz, len, rot){ const S=BK.COVER[type]||BK.COVER.hedge; const built=BK.buildCover(type,len), g=built.group;
  g.position.set(cx,0,cz); if(rot)g.rotation.y=rot; scene.add(g); g.updateMatrixWorld(true);
  const o={ grp:g, body:built.body, type, cx, cz, len, rot:rot||0, thickness:S.thickness, height:S.height, los:S.los, kind:'cover', boxes:[] };
  _coverWorldBoxes(o); g.userData.editSel={kind:'cover',ref:o}; covers.push(o);
  if(typeof stylizeOne==='function') stylizeOne(g); if(built.body) built.body.userData.noMerge=true; mergeStaticGroup(g); if(typeof rebuildLOS==='function') rebuildLOS(); return o; }
function _bkIsCover(t){ return t==='hedge'||t==='drystone'||t==='fence'; }
// ---- vegetation: point-placed trees (cel-shaded via stylizeOne; deterministic canopy so reloads are identical) ----
function _propWorldBox(o){ if(!o.solid){ o.box=null; return; } o.grp.updateMatrixWorld(true); o.box=new THREE.Box3().setFromObject(o.grp); }
function addProp(kind, cx, cz, seed, rot){ if(seed==null) seed=(Math.floor(Math.random()*1e9))>>>0; const built=BK.buildProp(kind, seed), g=built.group; g.position.set(cx,0,cz); g.rotation.y=rot||0; scene.add(g);
  const o={ grp:g, kind, x:cx, z:cz, seed, rot:rot||0, solid:built.solid, los:built.los, height:built.height }; _propWorldBox(o);
  g.userData.editSel={kind:'prop',ref:o}; props.push(o); stylizeOne(g); mergeStaticGroup(g); return o; }
function _treeWorldBox(o){ const r=o.colR; o.box=new THREE.Box3(new THREE.Vector3(o.x-r,0,o.z-r), new THREE.Vector3(o.x+r,o.height,o.z+r)); }
function addTree(kind, cx, cz, seed){ if(seed==null) seed=(Math.floor(Math.random()*1e9))>>>0; const built=BK.buildTree(kind, seed), g=built.group; g.position.set(cx,0,cz); scene.add(g);
  const o={ grp:g, kind, x:cx, z:cz, seed, colR:built.colR, height:built.height, los:true }; _treeWorldBox(o);
  g.userData.editSel={kind:'tree',ref:o}; trees.push(o); stylizeOne(g); mergeStaticGroup(g); return o; }
// ---- orchard: ONE shared cell layout used by both the drag preview and the plant, so what you see is what you get ----
const enemySpawns=[];
const enemySpawnLayer=new THREE.Group(); scene.add(enemySpawnLayer);
const _spawnFacCol={Soviets:0xE11020, Allies:0x1F6FF0, Empire:0x7B2FD6};
function makeEnemySpawn(x,z,faction){
  faction=Object.prototype.hasOwnProperty.call(_spawnFacCol,faction)?faction:'Soviets'; const col=_spawnFacCol[faction];
  const grp=new THREE.Group(); grp.position.set(x,0,z);
  const ring=new THREE.Mesh(new THREE.RingGeometry(1.5,1.95,30), new THREE.MeshBasicMaterial({color:col,transparent:true,opacity:0.85,side:THREE.DoubleSide,depthWrite:false})); ring.rotation.x=-Math.PI/2; ring.position.y=0.06; grp.add(ring);
  const disc=new THREE.Mesh(new THREE.CircleGeometry(1.45,30), new THREE.MeshBasicMaterial({color:col,transparent:true,opacity:0.22,side:THREE.DoubleSide,depthWrite:false})); disc.rotation.x=-Math.PI/2; disc.position.y=0.05; grp.add(disc);
  const pole=new THREE.Mesh(new THREE.CylinderGeometry(0.09,0.09,3.4,8), new THREE.MeshBasicMaterial({color:0x1a1a20})); pole.position.y=1.7; grp.add(pole);
  const flag=new THREE.Mesh(new THREE.PlaneGeometry(1.5,0.9), new THREE.MeshBasicMaterial({color:col,side:THREE.DoubleSide})); flag.position.set(0.75,2.95,0); grp.add(flag);
  const rec={grp,x,z,faction}; grp.userData.editSel={kind:'enemyspawn',ref:rec};
  enemySpawns.push(rec); enemySpawnLayer.add(grp); return rec;
}
const curMap={ name:'Cabana Republic' };   // B6: the built map's name, so saves, exports and undo snapshots keep it
function buildMap(def){
  clearMap(); curMap.name=(typeof def.name==='string'&&def.name)?def.name.slice(0,80):'Untitled map';
  (def.obstacles||[]).forEach(o=>{ const ob=addBox(o.x,o.z,o.w,o.h,o.d,o.warm?crateMatWarm:crateMat,true); if(o.rot){ ob.mesh.rotation.y=o.rot; ob.box.setFromObject(ob.mesh); } if(o.y!=null){ ob.mesh.position.y=o.y; ob.box.setFromObject(ob.mesh); } });
  (def.paths||[]).forEach(pp=>addPath(pp.x,pp.z,pp.w,pp.d,pp.rot||0));
  (def.buildings||[]).forEach(b=>{ const e=makeBuilding(b.team,b.type,b.x,b.z); if(b.hp){ e.maxHealth=b.hp; e.health=b.hp; } if(b.s&&b.s!==1)_scaleBuilding(e,b.s); if(b.rot){ e.mesh.rotation.y=b.rot; if(e.trim)e.trim.rotation.y=b.rot; e.box.setFromObject(e.mesh); } if(b.y!=null){ const rdy=b.y-e.mesh.position.y; [e.mesh,e.trim,e.label,e.bar&&e.bar.grp].forEach(o=>{if(o)o.position.y+=rdy;}); e.box.setFromObject(e.mesh); } });
  (def.platforms||[]).forEach(p=>makePlatform(p.x,p.z,p.w,p.d,p.h));
  (def.ramps||[]).forEach(r=>{ const rm=makeRamp(r.x,r.z,r.w,r.d,r.h0||0,r.h1,r.rot||0); if(r.y!=null){ rm.mesh.position.y=r.y; rm.box.setFromObject(rm.mesh); } });
  (def.houses||[]).forEach(h=>{ if(h.k==='kit'||h.spec){ spawnKitBuilding(h.spec, h.x, h.z, h.rot||0); } else { placeLegacyHouse((({thatchCottage:makeThatchCottage,brickFarm:makeBrickFlintFarmhouse,manor:makeManorHouse})[h.bt]||makeTimberHouse)(h.x,h.z,h.rot||0), h.bt==='manor'); } });
  (def.covers||[]).forEach(o=>{ addCover(o.type, o.x, o.z, o.len, o.rot||0); });
  (def.trees||[]).forEach(t=>addTree(t.kind, t.x, t.z, t.seed));
  (def.props||[]).forEach(pp=>addProp(pp.kind, pp.x, pp.z, pp.seed, pp.rot||0));
  (def.enemySpawns||[]).forEach(sp=>makeEnemySpawn(sp.x,sp.z,sp.faction));
  (def.barracks||[]).forEach(b=>{ const e=makeBarracks(b.x,b.z,b.team); if(b.s&&b.s!==1)_scaleBarracks(e,b.s); if(b.rot)_rotateBarracksBy(e,b.rot); });
  const rs=def.resupply||{x:0,z:34}; terminalPos.set(rs.x,0,rs.z);
  termMesh=new THREE.Mesh(new THREE.BoxGeometry(1.4,1.6,1),new THREE.MeshStandardMaterial({color:0x183a36,emissive:0x0e2a26,emissiveIntensity:0.5,roughness:0.5}));
  termMesh.position.set(rs.x,0.8,rs.z); termMesh.castShadow=true; termMesh.userData.editSel={kind:'resupply'}; scene.add(termMesh); if(typeof stylizeOne==='function')stylizeOne(termMesh);
  if(rs.rot)termMesh.rotation.y=rs.rot; if(rs.w||rs.h||rs.d){ const rw=rs.w||1.4,rh=rs.h||1.6,rd=rs.d||1; termMesh.scale.set(rw/1.4,rh/1.6,rd); termMesh.position.y=rh/2; }
  resupplyLabel=makeLabel('RESUPPLY','#7af0d2'); resupplyLabel.position.set(rs.x,2.6,rs.z); resupplyLabel.scale.set(4,1,1); scene.add(resupplyLabel);
  const sp=def.spawn||{x:0,z:30}; mapSpawn.set(sp.x,0,sp.z);
  rebuildLOS();
  _env=envMerge(def.env); applyEnv(_env,{regen:true});
  _rules=rulesMerge(def.rules); if(_rulesReady)applyRules(_rules);
  buildTeamIndex(); activateRoster();
}
function serializeMap(){ return {
  name:curMap.name,
  spawn:{x:+mapSpawn.x.toFixed(1),z:+mapSpawn.z.toFixed(1)},
  resupply:{x:+terminalPos.x.toFixed(1),z:+terminalPos.z.toFixed(1),rot:+((termMesh?termMesh.rotation.y:0)).toFixed(3),w:+((termMesh?termMesh.scale.x*1.4:1.4)).toFixed(2),h:+((termMesh?termMesh.scale.y*1.6:1.6)).toFixed(2),d:+((termMesh?termMesh.scale.z:1)).toFixed(2)},
  obstacles:obstacles.filter(o=>o.crate).map(o=>({x:+o.mesh.position.x.toFixed(1),z:+o.mesh.position.z.toFixed(1),w:o.w,h:o.h,d:o.d,warm:o.warm,rot:+(o.mesh.rotation.y).toFixed(3),y:+o.mesh.position.y.toFixed(2)})),
  paths:paths.map(o=>({x:+o.mesh.position.x.toFixed(1),z:+o.mesh.position.z.toFixed(1),w:o.w,d:o.d,rot:+(o.mesh.rotation.y).toFixed(3)})),
  platforms:platforms.map(p=>({x:+p.mesh.position.x.toFixed(1),z:+p.mesh.position.z.toFixed(1),w:p.w,d:p.d,h:p.h})),
  ramps:ramps.map(r=>({x:+r.mesh.position.x.toFixed(1),z:+r.mesh.position.z.toFixed(1),w:r.w,d:r.d,h0:r.h0,h1:r.h1,rot:+(r.mesh.rotation.y).toFixed(3),y:+r.mesh.position.y.toFixed(2)})),
  houses:houses.map(h=>(h.spec?{k:'kit',spec:h.spec,x:+h.grp.position.x.toFixed(1),z:+h.grp.position.z.toFixed(1),rot:+(h.grp.rotation.y).toFixed(3)}:{bt:h.type||'timberHouse',x:+h.grp.position.x.toFixed(1),z:+h.grp.position.z.toFixed(1),rot:+(h.grp.rotation.y).toFixed(3)})),
  covers:covers.map(o=>({type:o.type,x:+o.cx.toFixed(1),z:+o.cz.toFixed(1),len:+o.len.toFixed(2),rot:+o.rot.toFixed(3)})),
  trees:trees.map(o=>({kind:o.kind,x:+o.x.toFixed(1),z:+o.z.toFixed(1),seed:o.seed})),
  props:props.map(o=>({kind:o.kind,x:+o.x.toFixed(1),z:+o.z.toFixed(1),seed:o.seed,rot:+o.rot.toFixed(3)})),
  enemySpawns:enemySpawns.map(sp=>({x:+sp.x.toFixed(1),z:+sp.z.toFixed(1),faction:sp.faction})),
  buildings:buildings.map(b=>({team:b.team,type:b.type,x:+b.mesh.position.x.toFixed(1),z:+b.mesh.position.z.toFixed(1),rot:+(b.mesh.rotation.y).toFixed(3),s:+(b.scl||1).toFixed(3),hp:Math.round(b.maxHealth),y:+b.mesh.position.y.toFixed(2)})),
  barracks:barracksList.map(b=>({x:+(b.cx).toFixed(1),z:+(b.cz).toFixed(1),team:b.team,rot:+(b.rot||0).toFixed(3),s:+(b.scl||1).toFixed(3)})),
  env:currentEnvJSON(),
  rules:currentRulesJSON()
};}
const player=buildSoldier(PLAYER_FACTION); player.position.set(0,0,30); scene.add(player);
// --- melee + shield held models (player only; built once, hidden by default) ---
const ps={vel:new THREE.Vector3(),health:T.playerHealth,lastHitT:99,dead:false,deadT:0,curWeapon:0,spread:WEAPONS[0].spreadHip,fireCd:0,reloading:false,reloadT:0,ads:false,shieldOwned:false,shieldOn:false,shieldFlash:0,swinging:false,swingT:0,swingDur:0,swingImpact:0,swingHit:false,swingMesh:null,swingW:null,stamina:1,staminaLock:false,wpnSwap:0,wpnReloadDip:0,channeling:false,unit:0,maxHealth:100,molotovMode:false,swapCd:0,banzai:false,banzaiT:0,banzaiCd:0,burstLeft:0,bayonet:false,bayonetT:0,bayonetCd:0,_bayHit:null,fireKick:0,toppleT:0,vy:0,grounded:true,lastAttacker:null,_tx:-1,_tz:0,_stepT:0,id:0,team:PLAYER_TEAM,alive:true,group:player,active:true,frags:0,downs:0,lastAttackerId:-1,targetId:-1,retargetT:0,scanCursor:0,role:null};   // id 0 = SIM_PLAYER = SOLDIERS[0]
const loadout=WEAPONS.map((w)=>({ammo:w.mag,reserve:w.reserve,owned:true}));
const curW=()=>WEAPONS[ps.curWeapon], curL=()=>loadout[ps.curWeapon];
// ─── Phase 0: two-pool economy. Pay = personal cash (buys gear); Tech = shared team meter (advances era T1→T4). All values tunable. ───
const ECO={
  payTrickle:1,        // credits/sec/player while refinery alive
  payTrickleDead:0.25, // credits/sec/player when refinery destroyed (income near-cutoff)
  paySilo:1,           // credits/sec/player per captured silo
  payHarvester:600,    // credits/player per harvester dock
  techHarvester:5,     // team tech per harvester dock (half a Tech Box — the cash/tech tradeoff)
  techBox:10,          // team tech per Tech Box held to expiry
  techPerKill:1,       // small team-tech slice per kill (combat feeds tech on top of cash)
  techPerHeadshot:2,
  eraNames:['T1','T2','T3','T4'],
  eraCost:[60,90,120], // tech to advance T1→T2, T2→T3, T3→T4
};
const teamEco={}; for(let i=0;i<TEAMS.length;i++) teamEco[TEAMS[i].id]={era:1,tech:0,silos:0};
teamEco.friend=teamEco.allies; teamEco.hostile=teamEco.soviets;   // legacy aliases are the SAME objects until 3b: iterate TEAMS, never Object.keys(teamEco)
// ─── Phase 2: harvester / supply convoy (tunable) ───
const HARV={ speed:7, loadTime:3.5, dockTime:3.0, hp:900, respawn:12, fieldX:DM_Cabana.harvField.x, fieldZ:DM_Cabana.harvField.z };
// ─── Phase 3: Tech Boxes (tunable) ───
const TBOX={ points:DM_Cabana.techPoints, activeTime:120, captureTime:7, range:5.5, gap:6, firstDelay:8 };
// Team colours: allies teal, both enemy teams today's red until 3b's colour pass (D4); friend/hostile stay as aliases.
const TEAM_COL={ neutral:0xb5a642, friend:0x38e1b0, hostile:0xff4d4d, allies:0x38e1b0, soviets:0xff4d4d, empire:0xff4d4d };
const TEAM_HEX={ neutral:'#cdb24a', friend:'#7af0d2', hostile:'#ff8a8a', allies:'#7af0d2', soviets:'#ff8a8a', empire:'#ff8a8a' };
// Per-team building index (three-teams item 7): per-frame code reads these fields instead of searching buildings[] with
// a closure (findB). Rebuilt by buildTeamIndex() at the end of buildMap, at the start of resetRound and once at boot after
// the trucks exist; the editor adds and removes buildings without buildMap, and its Play button runs resetRound.
const TEAM_INDEX={ firstEnemy:null, rosterVer:0 };
for(let i=0;i<TEAMS.length;i++){ const id=TEAMS[i].id; TEAM_INDEX[id]={ active:false, blds:[], hqs:[], refinery:null, power:null, barracksB:null, barracksE:null, markers:[], harv:null, dockX:0, dockZ:0, laneX:0, laneZ:0 };
  if(TEAM_INDEX.firstEnemy===null&&id!==PLAYER_TEAM)TEAM_INDEX.firstEnemy=id; }   // set here too: HUD code may read it before the first buildMap
function buildTeamIndex(){
  for(let i=0;i<TEAMS.length;i++){ const e=TEAM_INDEX[TEAMS[i].id]; e.blds.length=0; e.hqs.length=0; e.refinery=null; e.power=null; e.barracksB=null; e.barracksE=null; e.markers.length=0; e.harv=null; }
  for(let i=0;i<buildings.length;i++){ const b=buildings[i], id=teamOf(b.team); if(!id)continue; const e=TEAM_INDEX[id]; e.blds.push(b);   // first of each type wins, as findB did
    if(b.type==='hq')e.hqs.push(b); else if(b.type==='refinery'){ if(!e.refinery)e.refinery=b; } else if(b.type==='power'){ if(!e.power)e.power=b; } else if(b.type==='barracks'){ if(!e.barracksB)e.barracksB=b; } }
  for(let i=0;i<barracksList.length;i++){ const bk=barracksList[i], id=teamOf(bk.team); if(id&&!TEAM_INDEX[id].barracksE)TEAM_INDEX[id].barracksE=bk; }
  for(let i=0;i<enemySpawns.length;i++){ const sp=enemySpawns[i], id=teamOf(sp.faction); if(id)TEAM_INDEX[id].markers.push(sp); }
  for(let i=0;i<harvesters.length;i++){ const hv=harvesters[i], id=teamOf(hv.team); if(id&&!TEAM_INDEX[id].harv)TEAM_INDEX[id].harv=hv; }
  TEAM_INDEX.firstEnemy=null;
  for(let i=0;i<TEAMS.length;i++){ const id=TEAMS[i].id, e=TEAM_INDEX[id], r=e.refinery;
    if(TEAM_INDEX.firstEnemy===null&&id!==PLAYER_TEAM)TEAM_INDEX.firstEnemy=id;
    // dock and field lane: today's two-team numbers for allies (dock -z, lane -4) and soviets (+z, +4); any other team
    // docks 7 m from its refinery toward the field. Map data (harvLanes) replaces these in a later step.
    if(id==='allies'||id==='soviets'){ const sg=(id==='allies')?-1:1;
      e.dockX=r?r.mesh.position.x:0; e.dockZ=r?r.mesh.position.z+7*sg:-35*sg; e.laneX=HARV.fieldX+4*sg; e.laneZ=HARV.fieldZ; }
    else { const rx=r?r.mesh.position.x:0, rz=r?r.mesh.position.z:0, dx=HARV.fieldX-rx, dz=HARV.fieldZ-rz, d=Math.sqrt(dx*dx+dz*dz)||1;
      e.dockX=rx+dx/d*7; e.dockZ=rz+dz/d*7; e.laneX=HARV.fieldX; e.laneZ=HARV.fieldZ; } }
}
// Roster activation (three-teams item 4): a team plays when it has an HQ; the player's team always does; a map where no
// other team has an HQ gets the Soviets on the legacy north band (N2). Slots of a team that doesn't play are parked:
// active=false, alive=false, hidden, and an alive=false SIM entry. Called after buildTeamIndex (buildMap, resetRound, boot).
function activateRoster(){
  let other=false;
  for(let i=0;i<TEAMS.length;i++){ const id=TEAMS[i].id, e=TEAM_INDEX[id]; e.active=(id===PLAYER_TEAM)||e.hqs.length>0; if(e.active&&id!==PLAYER_TEAM)other=true; }
  if(!other) TEAM_INDEX.soviets.active=true;
  for(let s=1;s<SOLDIERS.length;s++){ const en=SOLDIERS[s], g=en.group;
    // INTERIM (3a steps 6-9): the player's own team's bots stay parked until bots target across teams (step 10 deletes this)
    const on=TEAM_INDEX[en.team].active&&en.team!==PLAYER_TEAM;
    en.active=on;
    // A parked slot leaves the scene graph: r128 updates the matrices of hidden objects too. Back in the scene it goes
    // through stylizeOne, which skips meshes that are already toon and styles a slot that was parked when stylizeScene ran.
    if(on){ if(!g.parent){ scene.add(g); if(typeof stylizeOne==='function')stylizeOne(g); } g.visible=mode!=='editor'; }
    else { en.alive=false; en.respawnT=0; g.visible=false; g.userData.bar.grp.visible=false; if(g.parent)g.parent.remove(g); } }
  for(let i=0;i<harvesters.length;i++){ const hv=harvesters[i], ix=TEAM_INDEX[teamOf(hv.team)]; hv.parked=!(ix&&ix.active);
    if(hv.parked){ hv.alive=false; hv.group.visible=false; hv.bar.grp.visible=false; } }
  TEAM_INDEX.rosterVer++; _simFrame=-1;   // re-sync SIM on its next call, even within this frame, so a slot parked now is dead there now
}
// ─── Phase 4: economy-as-weapon — power plant down throttles tech (×0.5) and doubles prices ───
function powerDead(team){ for(let i=0;i<buildings.length;i++){ const b=buildings[i]; if(b.type==='power'&&sameTeam(b.team,team)) return !b.alive; } return false; }
function priceMult(team=PLAYER_TEAM){ return powerDead(team) ? 2 : 1; }
let credits=0, grenadeCount=0, meleeCd=0, kills=0, roundDeaths=0, roundAssists=0, gameOver=false;
let nearPanel=false, shopOpen=false;
// DEV/admin tools — set DEV.enabled=false (or wire to a real auth check) to hide entirely from players.
const DEV={enabled:true,god:false,infAmmo:false,freezeAI:false,allGuns:false};
let rifleKills=0, rifleLevel=1; // gun-game progression (rifleKills = total kills)
const RIFLE_TIERS=[ {name:'AMK Rifle I',reload:5.0,span:5}, {name:'AMK Rifle II',reload:3.5,span:10}, {name:'AMK Rifle III',reload:2.5,span:0} ];
const tierBase=L=>{let s=0;for(let i=0;i<L-1;i++)s+=RIFLE_TIERS[i].span;return s;};      // total kills at which level L begins
const advanceAt=L=>tierBase(L)+RIFLE_TIERS[L-1].span;                                       // total kills needed to leave level L

/* enemies */
const enemies=[];
const SOLDIERS=[];   // fixed slot table: SOLDIERS[0] is ps (the player), SOLDIERS[s] is enemies[s-1]; en.id===s is its SIM id
function spawnEnemyMesh(faction,elite){
  const e=elite?buildSoldier('Empire','m',ELITE_KIT):buildSoldier(faction||'Soviets');
  const bg=new THREE.Mesh(new THREE.PlaneGeometry(1.4,0.16),new THREE.MeshBasicMaterial({color:0x300808})); noRay(bg);
  const fg=new THREE.Mesh(new THREE.PlaneGeometry(1.4,0.16),new THREE.MeshBasicMaterial({color:0xff4d4d})); fg.position.z=0.001; noRay(fg);
  const grp=new THREE.Group(); grp.add(bg,fg); grp.position.y=2.9; e.add(grp); e.userData.bar={grp,fg}; return e;
}
function randSpawn(){return new THREE.Vector3((Math.random()*2-1)*(T.arenaX-8),0,-T.arenaZ+10+Math.random()*22);}
function respawnPos(en){ if(enemySpawns.length){ const fac=en&&en.faction; const mine=fac?enemySpawns.filter(sp=>sp.faction===fac):null; const pool=(mine&&mine.length)?mine:enemySpawns; const m=pool[(Math.random()*pool.length)|0]; const a=Math.random()*6.283, r=Math.random()*5; return new THREE.Vector3(m.x+Math.cos(a)*r,0,m.z+Math.sin(a)*r); } return randSpawn(); }
function setEnemyType(en, idx){ en.unit=idx; en.maxHealth=en.elite?T.eliteHealth:unitOf(idx).maxHealth; setHeldGun(en.group, WEAPONS[unitOf(idx).wpn].id); if(en.elite)return; const u=unitOf(idx); const hm=en.group.userData.head; if(hm&&hm.material&&hm.material.color) hm.material.color.set(u.accent); }
const harvesters=[]; const _hvDir=new THREE.Vector3();
function makeHarvester(team){
  team=teamOf(team); const isF=team!==null&&team===PLAYER_TEAM; const accent=isF?0x38e1b0:0xff4d4d;
  const g=new THREE.Group();
  const body=new THREE.Mesh(new THREE.BoxGeometry(3.0,1.5,2.0), new THREE.MeshStandardMaterial({color:isF?0x32423c:0x42322f,roughness:0.82,metalness:0.3})); body.position.set(-0.3,1.05,0); body.castShadow=true; g.add(body);
  const cab=new THREE.Mesh(new THREE.BoxGeometry(1.3,1.35,1.85), new THREE.MeshStandardMaterial({color:isF?0x283732:0x352826,roughness:0.82,metalness:0.3})); cab.position.set(1.55,0.95,0); cab.castShadow=true; g.add(cab);
  const stripe=new THREE.Mesh(new THREE.BoxGeometry(3.06,0.34,2.06), new THREE.MeshStandardMaterial({color:accent,emissive:accent,emissiveIntensity:0.5,roughness:0.4})); stripe.position.set(-0.3,1.72,0); g.add(stripe);
  const wmat=new THREE.MeshStandardMaterial({color:0x141418,roughness:0.95}); const wg=new THREE.CylinderGeometry(0.52,0.52,0.42,12);
  [[-1.2,1.05],[0.2,1.05],[-1.2,-1.05],[0.2,-1.05],[1.7,1.0],[1.7,-1.0]].forEach(([x,z])=>{ const wh=new THREE.Mesh(wg,wmat); wh.rotation.z=Math.PI/2; wh.position.set(x,0.52,z); g.add(wh); });
  g.visible=false; scene.add(g);
  const bg=new THREE.Mesh(new THREE.PlaneGeometry(3.4,0.34), new THREE.MeshBasicMaterial({color:0x101010,transparent:true,opacity:0.85})); noRay(bg);
  const fg=new THREE.Mesh(new THREE.PlaneGeometry(3.4,0.34), new THREE.MeshBasicMaterial({color:accent})); fg.position.z=0.01; noRay(fg);
  const bar=new THREE.Group(); bar.add(bg,fg); bar.visible=false; scene.add(bar);
  const hv={ team, group:g, accent, health:HARV.hp, maxHealth:HARV.hp, alive:true, refDead:false, state:'toField', t:0, hitFlash:0, respawnT:0, bar:{grp:bar,fg,w:3.4}, hx:1.7, hy:2.0, hz:1.1, parked:false, docks:0 };
  harvesters.push(hv); return hv;
}
function makeSupplyField(){
  const x=HARV.fieldX, z=HARV.fieldZ;
  const pad=new THREE.Mesh(new THREE.CircleGeometry(4.2,40), new THREE.MeshStandardMaterial({color:0x3a3826,roughness:0.95})); pad.rotation.x=-Math.PI/2; pad.position.set(x,0.02,z); pad.receiveShadow=true; noRay(pad); scene.add(pad);
  const ring=new THREE.Mesh(new THREE.RingGeometry(4.2,4.9,44), new THREE.MeshBasicMaterial({color:0xb5a642,transparent:true,opacity:0.4,side:THREE.DoubleSide})); ring.rotation.x=-Math.PI/2; ring.position.set(x,0.03,z); noRay(ring); scene.add(ring);
  const cmat=new THREE.MeshStandardMaterial({color:0x6b5a36,roughness:0.85,metalness:0.1});
  [[0,0,1.0],[1.1,0.5,0.8],[-0.9,-0.6,0.8],[0.3,-1.1,0.7],[-0.6,0.9,0.7]].forEach(([dx,dz,s])=>{ const c=new THREE.Mesh(new THREE.BoxGeometry(s,s,s), cmat); c.position.set(x+dx,s/2,z+dz); c.castShadow=true; noRay(c); scene.add(c); });
  const lbl=makeLabel('SUPPLY CACHE','#cdb24a'); lbl.position.set(x,3.4,z); scene.add(lbl);
}
function harvDock(team){ const e=TEAM_INDEX[teamOf(team)]; return e?{x:e.dockX,z:e.dockZ}:{x:0,z:0}; }
function updateHarvBar(hv){ const f=Math.max(0,hv.health/hv.maxHealth); hv.bar.fg.scale.x=f; hv.bar.fg.position.x=-(hv.bar.w*(1-f))/2; }
function harvesterPayout(hv){ hv.docks++; if(sameTeam(hv.team,PLAYER_TEAM)){ credits+=ECO.payHarvester; updateCreditHUD(); showLevelToast('Supply Dock','+'+ECO.payHarvester+' cr  ·  +'+ECO.techHarvester+' tech'); } addTech(hv.team, ECO.techHarvester); }
function damageHarvester(hv,d){ if(!hv.alive)return; hv.health-=d; hv.hitFlash=0.1; if(hv.health<=0) destroyHarvester(hv,false); else { updateHarvBar(hv); updateBaseHUD(); } }
function destroyHarvester(hv,refDead){ if(!hv.alive)return; hv.alive=false; hv.group.visible=false; hv.bar.grp.visible=false; addExplosion(hv.group.position.clone().setY(1.0)); camShake=Math.min(1.3,camShake+0.5); hv.refDead=!!refDead; hv.respawnT=HARV.respawn; if(sameTeam(hv.team,PLAYER_TEAM)) showLevelToast('Supply Truck Lost', refDead?'Refinery down — no resupply':'Respawning in '+HARV.respawn+'s'); updateBaseHUD(); }
function respawnHarvester(hv){ if(hv.parked)return; const dk=harvDock(hv.team); hv.group.position.set(dk.x,0,dk.z); hv.group.rotation.y=0; hv.health=hv.maxHealth; hv.alive=true; hv.refDead=false; hv.state='toField'; hv.t=0; hv.hitFlash=0; hv.group.visible=true; hv.bar.grp.visible=true; updateHarvBar(hv); updateBaseHUD(); }
function resetHarvesters(){ for(const hv of harvesters){ if(hv.parked)continue; hv.docks=0; const dk=harvDock(hv.team); hv.group.position.set(dk.x,0,dk.z); hv.group.rotation.y=0; hv.health=hv.maxHealth; hv.alive=true; hv.refDead=false; hv.state='toField'; hv.t=0; hv.hitFlash=0; hv.group.visible=true; hv.bar.grp.visible=true; updateHarvBar(hv); } updateBaseHUD(); }
function siegeFireHarvester(en,hv){ _eF.copy(en.group.position);_eF.y=1.45; _eT.copy(hv.group.position);_eT.y=1.4; const dir=_v.copy(_eT).sub(_eF).normalize(); eM.copy(_eF).addScaledVector(dir,0.9); addMuzzle(eM,0xff6a4a); sfxEnemyShot(eM); addTracer(eM,_eT,0xff5a3a); addImpact(_eT,_zAxis,'metal'); damageHarvester(hv,T.enemySiegeDamage); }
function updateHarvesters(dt){
  if(gameOver)return;
  for(const hv of harvesters){
    if(hv.parked)continue;   // its team doesn't play on this map
    const ix=TEAM_INDEX[teamOf(hv.team)], ref=ix?ix.refinery:null;
    if(hv.alive && (!ref || !ref.alive)){ destroyHarvester(hv,true); continue; }     // refinery dead → self-destruct, no respawn
    if(!hv.alive){ if(ref && ref.alive && !hv.refDead){ hv.respawnT-=dt; if(hv.respawnT<=0) respawnHarvester(hv); } continue; }
    if(hv.hitFlash>0) hv.hitFlash-=dt;
    const p=hv.group.position;
    if(hv.state==='toField' || hv.state==='toBase'){
      let destX,destZ;
      if(hv.state==='toField'){ destX=ix.laneX; destZ=ix.laneZ; }
      else { destX=ix.dockX; destZ=ix.dockZ; }
      _hvDir.set(destX-p.x,0,destZ-p.z); const d=_hvDir.length();
      if(d>1.2){ _hvDir.multiplyScalar(1/d); p.x+=_hvDir.x*HARV.speed*dt; p.z+=_hvDir.z*HARV.speed*dt; hv.group.rotation.y=Math.atan2(_hvDir.x,_hvDir.z); }
      else if(hv.state==='toField'){ hv.state='loading'; hv.t=HARV.loadTime; }
      else { hv.state='docking'; hv.t=HARV.dockTime; }
    } else { hv.t-=dt; if(hv.t<=0){ if(hv.state==='loading') hv.state='toBase'; else { harvesterPayout(hv); hv.state='toField'; } } }
    p.y=surfaceHeightAt(p.x,p.z); hv.bar.grp.position.set(p.x,p.y+3.2,p.z);
  }
}
const techPts=[];
function makeTechPoint(pt){
  const g=new THREE.Group(); g.position.set(pt.x,0,pt.z);
  const pad=new THREE.Mesh(new THREE.CircleGeometry(TBOX.range,40), new THREE.MeshStandardMaterial({color:0x26261c,roughness:0.96,transparent:true,opacity:0.55})); pad.rotation.x=-Math.PI/2; pad.position.y=0.02; noRay(pad); g.add(pad);
  const ring=new THREE.Mesh(new THREE.RingGeometry(TBOX.range-0.28,TBOX.range,52), new THREE.MeshBasicMaterial({color:0x6b6038,transparent:true,opacity:0.5,side:THREE.DoubleSide})); ring.rotation.x=-Math.PI/2; ring.position.y=0.05; noRay(ring); g.add(ring);
  const post=new THREE.Mesh(new THREE.CylinderGeometry(0.13,0.13,4.6,10), new THREE.MeshStandardMaterial({color:0x44402c,emissive:0x161510,emissiveIntensity:0.3,roughness:0.6})); post.position.y=2.3; noRay(post); g.add(post);
  const tip=new THREE.Mesh(new THREE.SphereGeometry(0.32,12,10), new THREE.MeshBasicMaterial({color:0x6b6038})); tip.position.y=4.7; noRay(tip); g.add(tip);
  const lbl=makeLabel(pt.name.toUpperCase(),'#cdb24a'); lbl.position.y=5.5; g.add(lbl);
  scene.add(g); return {pt, group:g, ring, post, tip, lbl};
}
let tboxMesh=null, tboxBar=null;
function makeTechBoxMesh(){
  const g=new THREE.Group();
  const crate=new THREE.Mesh(new THREE.BoxGeometry(1.5,1.5,1.5), new THREE.MeshStandardMaterial({color:TEAM_COL.neutral,emissive:TEAM_COL.neutral,emissiveIntensity:0.45,roughness:0.5,metalness:0.3})); crate.castShadow=true; g.add(crate);
  const band=new THREE.Mesh(new THREE.BoxGeometry(1.64,0.34,1.64), new THREE.MeshStandardMaterial({color:0x2a2718,roughness:0.55,metalness:0.4})); g.add(band);
  g.userData.crate=crate; noRay(g); g.visible=false; scene.add(g);
  const bg=new THREE.Mesh(new THREE.PlaneGeometry(2.0,0.26), new THREE.MeshBasicMaterial({color:0x101010,transparent:true,opacity:0.85})); noRay(bg);
  const fg=new THREE.Mesh(new THREE.PlaneGeometry(2.0,0.26), new THREE.MeshBasicMaterial({color:TEAM_COL[PLAYER_TEAM]})); fg.position.z=0.01; noRay(fg);
  const bar=new THREE.Group(); bar.add(bg,fg); bar.visible=false; scene.add(bar);
  tboxMesh=g; tboxBar={grp:bar,fg,w:2.0};
}
const tbox={ active:false, idx:-1, owner:'neutral', prog:0, progTeam:null, timer:0, gap:TBOX.firstDelay, nextIdx:Math.floor(Math.random()*TBOX.points.length) };
const tboxHudEl=document.getElementById('tboxHud'), tbNameEl=document.getElementById('tbName'), tbTimerEl=document.getElementById('tbTimer'), tbBarWrap=document.getElementById('tbBar'), tbBarFill=tbBarWrap?tbBarWrap.querySelector('i'):null;
let _tbSec=-1;
function tbRGB(team){ const t=teamOf(team); return t===null?'181,166,66':(t===PLAYER_TEAM?'56,225,176':'255,77,77'); }
function dressTechPoints(){
  techPts.forEach((tp,i)=>{
    let col=0x6b6038, emi=0.3, op=0.5;
    if(tbox.active && i===tbox.idx){ col=TEAM_COL[tbox.owner]; emi=0.9; op=0.85; }
    else if(!tbox.active && i===tbox.nextIdx){ col=0xc9b878; emi=0.6; op=0.7; }
    tp.ring.material.color.setHex(col); tp.ring.material.opacity=op; tp.tip.material.color.setHex(col);
    tp.post.material.emissive.setHex(col); tp.post.material.emissiveIntensity=emi;
  });
}
function spawnTechBox(idx){ tbox.active=true; tbox.idx=idx; tbox.owner='neutral'; tbox.prog=0; tbox.progTeam=null; tbox.timer=TBOX.activeTime; const p=TBOX.points[idx]; tboxMesh.position.set(p.x,1.5,p.z); tboxMesh.userData.crate.material.color.setHex(TEAM_COL.neutral); tboxMesh.userData.crate.material.emissive.setHex(TEAM_COL.neutral); tboxMesh.visible=true; tboxBar.grp.visible=false; dressTechPoints(); _tbSec=-1; updateTboxHUD(); showLevelToast('Tech Box Live', p.name+' — capture it'); }
function onTechCapture(team){ const nm=TBOX.points[tbox.idx].name; const mine=sameTeam(team,PLAYER_TEAM); showLevelToast(mine?'Tech Box Captured':'Tech Box Taken', mine?nm+' — defend it':'Hostiles took '+nm); dressTechPoints(); updateTboxHUD(); }
function cashOutTechBox(){
  const nm=TBOX.points[tbox.idx].name;
  if(tbox.owner!=='neutral'){ addTech(tbox.owner, ECO.techBox); const mine=sameTeam(tbox.owner,PLAYER_TEAM); showLevelToast(mine?'Tech Box Secured':'Tech Box Lost', mine?'+'+ECO.techBox+' tech banked':'Hostiles banked +'+ECO.techBox+' tech'); }
  else showLevelToast('Tech Box Expired', nm+' uncaptured');
  tbox.active=false; tboxMesh.visible=false; tboxBar.grp.visible=false; tbox.owner='neutral'; tbox.prog=0; tbox.progTeam=null; ps.channeling=false;
  let n=tbox.idx; if(TBOX.points.length>1){ while(n===tbox.idx) n=Math.floor(Math.random()*TBOX.points.length); } tbox.nextIdx=n; tbox.gap=TBOX.gap;
  dressTechPoints(); _tbSec=-1; updateTboxHUD();
}
function resetTechBox(){ tbox.active=false; tbox.idx=-1; tbox.owner='neutral'; tbox.prog=0; tbox.progTeam=null; tbox.timer=0; tbox.gap=TBOX.firstDelay; tbox.nextIdx=Math.floor(Math.random()*TBOX.points.length); if(tboxMesh)tboxMesh.visible=false; if(tboxBar)tboxBar.grp.visible=false; ps.channeling=false; dressTechPoints(); _tbSec=-1; updateTboxHUD(); }
function updateTboxHUD(){
  if(!tboxHudEl)return;
  if(tbox.active){ tboxHudEl.style.borderColor='rgba('+tbRGB(tbox.owner)+',.5)'; if(tbNameEl){ tbNameEl.textContent=TBOX.points[tbox.idx].name; tbNameEl.style.color=TEAM_HEX[tbox.owner]; } }
  else { tboxHudEl.style.borderColor='rgba(181,166,66,.25)'; if(tbNameEl){ tbNameEl.textContent='Next · '+TBOX.points[tbox.nextIdx].name; tbNameEl.style.color='#cdb24a'; } }
}
function updateTboxHUDTick(){
  if(!tboxHudEl)return;
  if(tbox.active){
    const s=Math.max(0,Math.ceil(tbox.timer)); if(s!==_tbSec){ _tbSec=s; if(tbTimerEl) tbTimerEl.textContent=Math.floor(s/60)+':'+((s%60)<10?'0':'')+(s%60); }
    if(tbBarWrap){ if(tbox.prog>0){ tbBarWrap.style.display='block'; if(tbBarFill){ tbBarFill.style.width=(tbox.prog*100).toFixed(0)+'%'; tbBarFill.style.background='rgba('+tbRGB(tbox.progTeam)+',.95)'; } } else tbBarWrap.style.display='none'; }
  } else { const g=Math.max(0,Math.ceil(tbox.gap)); if(g!==_tbSec){ _tbSec=g; if(tbTimerEl) tbTimerEl.textContent='in '+g+'s'; } if(tbBarWrap) tbBarWrap.style.display='none'; }
}
function updateTechBox(dt){
  if(gameOver){ ps.channeling=false; return; }
  if(!tbox.active){ tbox.gap-=dt; if(tbox.gap<=0) spawnTechBox(tbox.nextIdx); updateTboxHUDTick(); return; }
  tbox.timer-=dt;
  const p=TBOX.points[tbox.idx];
  const pdx=player.position.x-p.x, pdz=player.position.z-p.z; const pIn=(pdx*pdx+pdz*pdz)<=TBOX.range*TBOX.range;
  const playerCh = pIn && !ps.dead && keys['KeyE'];
  let enemyCh=false; for(const en of enemies){ if(!en.alive)continue; const ex=en.group.position.x-p.x, ez=en.group.position.z-p.z; if(ex*ex+ez*ez<=TBOX.range*TBOX.range){ enemyCh=true; break; } }
  ps.channeling = playerCh && tbox.owner!=='friend';
  if(ps.channeling && charge.active) endCharge();
  let chTeam=null;
  if(playerCh && !enemyCh) chTeam='friend'; else if(enemyCh && !playerCh) chTeam='hostile';
  if(chTeam && tbox.owner!==chTeam){
    if(tbox.progTeam!==chTeam){ tbox.prog=0; tbox.progTeam=chTeam; }
    tbox.prog += dt/TBOX.captureTime;
    if(tbox.prog>=1){ tbox.owner=chTeam; tbox.prog=0; tbox.progTeam=null; onTechCapture(chTeam); }
  } else if(tbox.prog>0){ tbox.prog=Math.max(0,tbox.prog-dt/TBOX.captureTime); if(tbox.prog<=0) tbox.progTeam=null; }
  const tt=performance.now()*0.001;
  tboxMesh.position.y=1.5+Math.sin(tt*2.2)*0.16; tboxMesh.rotation.y+=dt*0.8;
  tboxMesh.userData.crate.material.color.setHex(TEAM_COL[tbox.owner]); tboxMesh.userData.crate.material.emissive.setHex(TEAM_COL[tbox.owner]);
  const tp=techPts[tbox.idx]; tp.ring.material.color.setHex(TEAM_COL[tbox.owner]); tp.tip.material.color.setHex(TEAM_COL[tbox.owner]); tp.post.material.emissive.setHex(TEAM_COL[tbox.owner]);
  if(tbox.prog>0 && tbox.progTeam){ tboxBar.grp.visible=true; tboxBar.grp.position.set(p.x,3.5,p.z); const f=tbox.prog; tboxBar.fg.scale.x=f; tboxBar.fg.position.x=-(tboxBar.w*(1-f))/2; tboxBar.fg.material.color.setHex(TEAM_COL[tbox.progTeam]); }
  else tboxBar.grp.visible=false;
  if(tbox.timer<=0){ cashOutTechBox(); return; }
  updateTboxHUDTick();
}

/* ============ SILOS (Phase 4) ============
 * Neutral fuel silos captured by PRESENCE (no channel; weapons stay live — softer than Tech Boxes).
 * Each silo a team holds adds ECO.paySilo cr/sec to that team's trickle (the player's cash). An
 * enemy on the pad contests (freezes/decays); clear them to flip it. Bots capture incidentally as
 * the front line crosses the pads. teamEco.<team>.silos (recounted on every flip) drives income. */
const SILO={ points:DM_Cabana.silos, range:5, captureTime:3.5 };
const silos=[]; const siloRateEl=document.getElementById('siloRate');
function makeSilo(pt){
  const g=new THREE.Group(); g.position.set(pt.x,0,pt.z);
  const pad=new THREE.Mesh(new THREE.CircleGeometry(SILO.range,36), new THREE.MeshStandardMaterial({color:0x242018,roughness:0.96,transparent:true,opacity:0.5})); pad.rotation.x=-Math.PI/2; pad.position.y=0.02; noRay(pad); g.add(pad);
  const ring=new THREE.Mesh(new THREE.RingGeometry(SILO.range-0.25,SILO.range,44), new THREE.MeshBasicMaterial({color:0x6b6038,transparent:true,opacity:0.45,side:THREE.DoubleSide})); ring.rotation.x=-Math.PI/2; ring.position.y=0.05; noRay(ring); g.add(ring);
  const tank=new THREE.Mesh(new THREE.CylinderGeometry(1.5,1.5,3.6,18), new THREE.MeshStandardMaterial({color:0x3a3a40,roughness:0.7,metalness:0.4})); tank.position.y=1.9; tank.castShadow=true; g.add(tank);
  const cap=new THREE.Mesh(new THREE.CylinderGeometry(1.62,1.62,0.4,18), new THREE.MeshStandardMaterial({color:0x2a2a30,roughness:0.6,metalness:0.5})); cap.position.y=3.8; g.add(cap);
  const band=new THREE.Mesh(new THREE.CylinderGeometry(1.56,1.56,0.6,18), new THREE.MeshStandardMaterial({color:0x6b6038,emissive:0x6b6038,emissiveIntensity:0.5,roughness:0.5})); band.position.y=2.7; g.add(band);
  const lbl=makeLabel('SILO','#cdb24a'); lbl.position.y=4.6; g.add(lbl);
  scene.add(g);
  const bg=new THREE.Mesh(new THREE.PlaneGeometry(1.8,0.22), new THREE.MeshBasicMaterial({color:0x101010,transparent:true,opacity:0.85})); noRay(bg);
  const fg=new THREE.Mesh(new THREE.PlaneGeometry(1.8,0.22), new THREE.MeshBasicMaterial({color:TEAM_COL[PLAYER_TEAM]})); fg.position.z=0.01; noRay(fg);
  const bar=new THREE.Group(); bar.add(bg,fg); bar.position.set(pt.x,4.0,pt.z); bar.visible=false; scene.add(bar);
  return {pt, group:g, ring, band, owner:'neutral', prog:0, progTeam:null, bar:{grp:bar,fg,w:1.8}};
}
function dressSilo(s){ const c=TEAM_COL[s.owner]; s.band.material.color.setHex(c); s.band.material.emissive.setHex(c); s.ring.material.color.setHex(c); s.ring.material.opacity=s.owner==='neutral'?0.45:0.8; }
function updateSiloHUD(){ silos.forEach((s,i)=>{ const d=document.querySelector('.silo-dot[data-s="'+i+'"]'); if(d) d.style.background='rgba('+tbRGB(s.owner)+',.95)'; }); if(siloRateEl) siloRateEl.textContent='+'+(ECO.paySilo*teamEco[PLAYER_TEAM].silos)+' cr/s'; }
function recountSilos(){ for(let i=0;i<TEAMS.length;i++) teamEco[TEAMS[i].id].silos=0; for(const s of silos){ const id=teamOf(s.owner); if(id)teamEco[id].silos++; } updateSiloHUD(); }
function resetSilos(){ for(const s of silos){ s.owner='neutral'; s.prog=0; s.progTeam=null; s.bar.grp.visible=false; dressSilo(s); } for(let i=0;i<TEAMS.length;i++) teamEco[TEAMS[i].id].silos=0; updateSiloHUD(); }
function updateSilos(dt){
  if(gameOver)return;
  for(const s of silos){
    const p=s.pt;
    const pdx=player.position.x-p.x, pdz=player.position.z-p.z; const fIn=(pdx*pdx+pdz*pdz)<=SILO.range*SILO.range && !ps.dead;
    let hIn=false; for(const en of enemies){ if(!en.alive)continue; const ex=en.group.position.x-p.x, ez=en.group.position.z-p.z; if(ex*ex+ez*ez<=SILO.range*SILO.range){ hIn=true; break; } }
    let chTeam=null; if(fIn && !hIn) chTeam='friend'; else if(hIn && !fIn) chTeam='hostile';
    if(chTeam && s.owner!==chTeam){
      if(s.progTeam!==chTeam){ s.prog=0; s.progTeam=chTeam; }
      s.prog += dt/SILO.captureTime;
      if(s.prog>=1){ s.owner=chTeam; s.prog=0; s.progTeam=null; dressSilo(s); recountSilos(); if(sameTeam(chTeam,PLAYER_TEAM)) showLevelToast('Silo Captured','+'+ECO.paySilo+' cr/s'); }
    } else if(s.prog>0){ s.prog=Math.max(0,s.prog-dt/SILO.captureTime); if(s.prog<=0) s.progTeam=null; }
    if(s.prog>0 && s.progTeam){ s.bar.grp.visible=true; const f=s.prog; s.bar.fg.scale.x=f; s.bar.fg.position.x=-(s.bar.w*(1-f))/2; s.bar.fg.material.color.setHex(TEAM_COL[s.progTeam]); }
    else s.bar.grp.visible=false;
  }
}

/* FX */
const fx=[];
// ---- FX object pool: reuse meshes per kind (kills per-event geometry/material alloc + GC churn). ----
// Each slot owns its geometry+material (built once). Pooled meshes stay in the scene, toggled visible;
// the FX loop RELEASES a slot (see f.slot at expiry) instead of disposing it. Lights + rare grenade-only
// kinds (dust/shock/scorch/fade/frag/comic) stay on the old create/dispose path.
const FXPool={ _p:{}, _def(k,make,cap){this._p[k]={free:[],make,cap:cap||0,n:0};},   // cap: max slots ever made (0 = unbounded); acquire -> null when full
  acquire(k){ const p=this._p[k]; let s=p.free.pop(); if(!s){ if(p.cap&&p.n>=p.cap) return null; s=p.make(); s._kind=k; p.n++; } s.mesh.visible=true; return s; },
  release(s){ s.mesh.visible=false; this._p[s._kind].free.push(s); } };
const _fxUp=new THREE.Vector3(0,1,0), _fxDir=new THREE.Vector3(); const FX_SPARK_MAX=24; const FX_FRAG_MAX=64;
function acquireSpark(p,ct,col,size,life){ ct=Math.min(ct,FX_SPARK_MAX); const s=FXPool.acquire('spark'); const m=s.mesh; m.position.copy(p); const pos=m.geometry.attributes.position; for(let i=0;i<ct*3;i++)pos.array[i]=0; pos.needsUpdate=true; m.geometry.setDrawRange(0,ct); m.material.color.setHex(col); m.material.size=size; m.material.opacity=1; fx.push({mesh:m,t:0,life:life,kind:'spark',ct:ct,vels:s.vels,slot:s}); return s.vels; }
function addTracer(a,b,c){const d=a.distanceTo(b);const s=FXPool.acquire('tracer');const m=s.mesh;m.position.copy(a).add(b).multiplyScalar(0.5);m.quaternion.setFromUnitVectors(_fxUp,_fxDir.copy(b).sub(a).normalize());m.scale.set(1,d,1);m.material.color.setHex(c||0xfff0b0);m.material.opacity=0.95;fx.push({mesh:m,t:0,life:0.13,kind:'tracer',slot:s});}
function addMuzzle(p,c){const s=FXPool.acquire('flash');const m=s.mesh;m.position.copy(p);m.scale.setScalar(0.32);m.material.color.setHex(c||0xffd27a);m.material.opacity=0.95;fx.push({mesh:m,t:0,life:0.06,kind:'flash',r0:0.32,slot:s});const sl=FXPool.acquire('light');const l=sl.mesh;l.color.setHex(c||0xffcf80);l.intensity=4;l.distance=8;l.position.copy(p);fx.push({mesh:l,t:0,life:0.06,kind:'light',slot:sl});}
function addImpact(p,n,surf){ // surface-aware: 'flesh' blood / 'metal' bright sparks / 'dirt' dusty chip (bool back-compat: true=flesh,false=dirt)
  if(surf===true)surf='flesh'; else if(surf===false||!surf)surf='dirt'; if(surf!=='flesh')sfxImpact(p); // positional thwack for world hits; enemy hits already get the centered hitmarker tick
  let col,ct,size,sMin,sMax,life,up;
  if(surf==='flesh'){ col=0xc81e1e; ct=10; size=0.17; sMin=2.2; sMax=5.0; life=0.42; up=1.4; }
  else if(surf==='metal'){ col=0xffcf86; ct=11; size=0.13; sMin=4.0; sMax=8.5; life=0.30; up=2.6; }
  else { col=0x8c7c60; ct=8; size=0.22; sMin=1.4; sMax=3.4; life=0.50; up=1.7; }
  const vels=acquireSpark(p,ct,col,size,life);
  for(let i=0;i<ct;i++)vels[i].set(Math.random()*2-1,Math.random(),Math.random()*2-1).normalize().multiplyScalar(sMin+Math.random()*(sMax-sMin)).addScaledVector(n,up);
}
function addCasing(){ const s=FXPool.acquire('casing'); const m=s.mesh; m.position.set(player.position.x,1.32,player.position.z); m.rotation.set(0,0,0); m.material.opacity=1;
  const rt=Math.random()*6.28; s.vel.set(Math.cos(rt)*1.8,3.0+Math.random()*1.3,Math.sin(rt)*1.8); s.av.set((Math.random()*2-1)*16,(Math.random()*2-1)*8,(Math.random()*2-1)*16);
  fx.push({mesh:m,t:0,life:1.1,kind:'casing',vel:s.vel,av:s.av,slot:s}); }
function addExplosion(p){
  sfxBoom(p);
  const sb=FXPool.acquire('boom'); const bm=sb.mesh; bm.position.copy(p); bm.scale.setScalar(1); bm.material.opacity=0.9; fx.push({mesh:bm,t:0,life:0.4,kind:'boom',slot:sb});
  const sl=FXPool.acquire('light'); const l=sl.mesh; l.color.setHex(0xffa040); l.intensity=8; l.distance=T.grenadeRadius*3; l.position.copy(p); fx.push({mesh:l,t:0,life:0.35,kind:'light',slot:sl});
  const vels=acquireSpark(p,22,0xffcaa0,0.3,0.5);
  for(let i=0;i<22;i++)vels[i].set(Math.random()*2-1,Math.random()*1.3,Math.random()*2-1).normalize().multiplyScalar(5+Math.random()*7);
}

/* ---------- grounded & gritty grenade blast (Tarkov-style: blast > fireball) ---------- */
const concussionEl=document.getElementById('concussion'); let concussionTO=0;
function concussion(prox){
  const s=Math.min(1,prox*T.concussionStrength); if(s<0.03)return;
  concussionEl.style.transition='opacity .05s linear'; concussionEl.style.opacity=s;
  clearTimeout(concussionTO); concussionTO=setTimeout(()=>{concussionEl.style.transition='opacity .55s ease-out';concussionEl.style.opacity='0';},70);
}
function scuffTick(x,z){ // tiny dirt scuff where a fragment skips off the ground
  const sl=FXPool.acquire('scuff'); if(!sl) return; const m=sl.mesh;   // cosmetic: skipped when 40 are already live m.scale.setScalar(0.1+Math.random()*0.12); m.rotation.set(-Math.PI/2,0,0); m.position.set(x,0.025,z);
  m.material.opacity=0.5; fx.push({mesh:m,t:0,life:0.45,kind:'fade',op:0.5,slot:sl});
}
function spawnShock(p){ // flat pressure rings on the ground
  const mk=(kind,r1,op,life)=>{ const sl=FXPool.acquire(kind), ring=sl.mesh; ring.scale.setScalar(1); ring.rotation.set(-Math.PI/2,0,0); ring.position.set(p.x,0.06,p.z);
    ring.material.opacity=op; fx.push({mesh:ring,t:0,life:life,kind:'shock',op:op,baseR:r1,slot:sl}); };
  mk('shockA',0.62,0.6,0.22); mk('shockB',0.55,0.3,0.36);
}
function spawnDust(p){ // low dirt cloud, lingers and thins
  for(let i=0;i<5;i++){const r=0.55+Math.random()*0.55;
    const sl=FXPool.acquire('dust'), m=sl.mesh; m.scale.setScalar(r); m.material.opacity=0.3;
    m.position.set(p.x+(Math.random()*2-1)*1.3,Math.max(0.3,p.y)+Math.random()*0.4,p.z+(Math.random()*2-1)*1.3);
    fx.push({mesh:m,t:0,life:T.dustLife*(0.7+Math.random()*0.5),kind:'dust',op:0.3,grow:1.7+Math.random()*1.3,rise:0.4+Math.random()*0.5,r0:r,slot:sl});}
}
function fragBurst(p,n,size,spMult,elMin,elSpan,life){ // one shrapnel layer: a POOLED Points cloud flung out under gravity (kind:'frag' tick handles bounce/cool/fade) (D3)
  n=Math.min(n,FX_FRAG_MAX); const s=FXPool.acquire('frag'); const pts=s.mesh; pts.position.copy(p);
  const pos=pts.geometry.attributes.position; for(let i=0;i<n*3;i++)pos.array[i]=0; pos.needsUpdate=true; pts.geometry.setDrawRange(0,n);
  pts.material.color.setHex(0xfff0d8); pts.material.size=size; pts.material.opacity=1;
  const vels=s.vels; for(let i=0;i<n;i++){const a=Math.random()*Math.PI*2,el=elMin+Math.random()*elSpan,sp=T.fragSpeed*spMult*(0.6+Math.random()*1.1),ch=Math.cos(el);vels[i].set(Math.cos(a)*ch,Math.sin(el),Math.sin(a)*ch).multiplyScalar(sp);}
  fx.push({mesh:pts,t:0,life:life,kind:'frag',ct:n,vels:s.vels,baseX:p.x,baseY:p.y,baseZ:p.z,slot:s});
}
function spawnShrapnel(p){ // Tarkov-style: a dense fast spray + a few heavier chunks that arc out and skitter
  fragBurst(p, T.fragCount, 0.42, 1.0, T.fragLaunchAngle*0.25, T.fragLaunchAngle*1.05, T.fragLife);          // fine fast spray
  fragBurst(p, T.fragChunks||14, 0.72, 0.55, T.fragLaunchAngle*0.8, T.fragLaunchAngle*1.7, T.fragLife*1.3);  // heavier arcing chunks that linger
}
function spawnScorch(p){ // dark scuff that lingers then fades
  const sl=FXPool.acquire('scorch'), m=sl.mesh; m.scale.setScalar(T.scorchRadius*(0.8+Math.random()*0.3)); m.rotation.set(-Math.PI/2,0,Math.random()*6.28);
  m.position.set(p.x,0.03,p.z); m.material.opacity=0.72; fx.push({mesh:m,t:0,life:T.scorchLife,kind:'scorch',op:0.72,slot:sl});
}
function grenadeBlast(p){
  sfxBoom(p);
  // hard, brief flash + light pop — NOT a fireball
  const fl=new THREE.Mesh(new THREE.SphereGeometry(0.55,10,8),new THREE.MeshBasicMaterial({color:0xfff1d2,transparent:true,opacity:0.95})); fl.position.copy(p); noRay(fl); scene.add(fl); fx.push({mesh:fl,t:0,life:0.07,kind:'flash'});
  const _fls=FXPool.acquire('light'); const fll=_fls.mesh; fll.color.setHex(0xffe1b0); fll.intensity=9; fll.distance=T.shockRadius*2.2; fll.position.copy(p); fx.push({mesh:fll,t:0,life:0.1,kind:'light',i0:9,slot:_fls});
  spawnShock(p); spawnDust(p); spawnShrapnel(p); spawnScorch(p);
  // directional concussion: kick the camera AWAY from the blast, scaled by proximity
  const away=_v.copy(player.position).sub(p); away.y=0; const d=away.length();
  const prox=Math.max(0,1-d/T.camKickRange);
  if(prox>0){ if(d>0.6) away.normalize(); else away.set(Math.random()*2-1,0,Math.random()*2-1).normalize();
    const imp=T.camKickImpulse*prox; camKickVel.addScaledVector(away,imp); camKickVel.y+=imp*0.4; camShake=Math.max(camShake,0.4*prox); }
  concussion(prox);
}

/* input */
const keys={}; const mouseNDC=new THREE.Vector2(); let mouseDown=false,semiQueued=false,started=false,paused=false,mode='menu',rotating=false,camYaw=0,camYawTarget=0,camPitchOverride=null,adsPitchK=0,adsPanK=0;
let mx=innerWidth/2, my=innerHeight/2;
const reticleEl=document.getElementById('reticle'), hitmarkEl=document.getElementById('hitmark'), rotDotEl=document.getElementById('rotDot');
const _rUp=reticleEl.querySelector('.up'),_rDown=reticleEl.querySelector('.down'),_rLeft=reticleEl.querySelector('.left'),_rRight=reticleEl.querySelector('.right');
const reloadRing=document.getElementById('reloadRing'), reloadFg=reloadRing.querySelector('.rr-fg'), reloadLabel=document.getElementById('reloadLabel');
const RR_C=2*Math.PI*16; reloadFg.style.strokeDasharray=RR_C;
let _cvR=renderer.domElement.getBoundingClientRect();
function refreshRect(){ _cvR=renderer.domElement.getBoundingClientRect(); }
const pauseMenuEl=document.getElementById('pauseMenu');
const scoreboardEl=document.getElementById('scoreboard');
function setPause(p){ paused=p; mouseDown=false; semiQueued=false; ps.ads=false; rotating=false; renderer.domElement.style.cursor=p?'default':'none'; pauseMenuEl.classList.remove('wr-manual'); pauseMenuEl.style.display=p?'flex':'none'; if(p){endCharge();reticleEl.style.display='none';hitmarkEl.style.display='none';if(scoreboardEl)scoreboardEl.style.display='none';if(sightLine)sightLine.visible=false;if(heightLine)heightLine.visible=false;} }
function togglePause(){ if(!started||gameOver) return; setPause(!paused); }
const titleEl=document.getElementById('title'), endEl=document.getElementById('endScreen'), hudEl=document.getElementById('hud');
function deploy(){ started=true; mode='game'; paused=false; mouseDown=false; semiQueued=false; renderer.domElement.style.cursor='none'; if(typeof edGhost!=='undefined'&&edGhost)edGhost.visible=false; resetRound(); hudEl.style.display=''; titleEl.style.opacity='0'; setTimeout(()=>{if(mode==='game')titleEl.style.display='none';},500); maybeShowTutorial();  navBuild();   // bot pathfinding grid for this map
}
const raycaster=new THREE.Raycaster(), losRay=new THREE.Raycaster();
const groundPlane=new THREE.Plane(new THREE.Vector3(0,1,0),0), _v=new THREE.Vector3(), _p1=new THREE.Vector3(), _p2=new THREE.Vector3();
const aimPoint=new THREE.Vector3(0,1,0), aimGround=new THREE.Vector3(0,0,0); let aimHoverEnemy=false, aimPart=null, aimTarget=false, aimBlocked=false;
// reusable scratch + camera dead-zone state — keeps the per-frame path allocation-free
const _eEyes=new THREE.Vector3(),_ePc=new THREE.Vector3(),_eDir=new THREE.Vector3(),_eMv=new THREE.Vector3(),_eStr=new THREE.Vector3();
const _camLookT=new THREE.Vector3(),_camAnchor=new THREE.Vector3(),_camDesired=new THREE.Vector3(),_camLF=new THREE.Vector3();
const _wish=new THREE.Vector3();   // scratch: player movement wish-dir / bayonet charge-dir (was a per-frame alloc, A1)
let smPushX=0,smPushY=0,_lastStam=-1;
// signed 0..±1: zero inside the dead zone, smoothstep ramp to ±1 at the screen edge
function dzPush(n,dz){ const a=(Math.abs(n)-dz)/(1-dz); if(a<=0)return 0; const s=a<1?a:1; return (n<0?-1:1)*(s*s*(3-2*s)); }
function updateAim(){
  raycaster.setFromCamera(mouseNDC,camera); raycaster.far=400;
  const o=raycaster.ray.origin, d=raycaster.ray.direction;
  // stable ground point under the crosshair (drives camera framing fallback)
  if(raycaster.ray.intersectPlane(groundPlane,_v)) aimGround.copy(_v);
  // hit resolution through the HEADLESS math core — no per-frame array/raycast allocation
  syncSimWorld();
  const r=resolveShot(SIM,SIM_PLAYER, o.x,o.y,o.z, d.x,d.y,d.z, 400, teamFilter(PLAYER_TEAM));
  aimHoverEnemy=false; aimPart=null; aimTarget=false; aimBlocked=false;
  if(r.kind==='none'){ aimPoint.copy(aimGround); }
  else { aimPoint.set(r.px,r.py,r.pz);
    if(r.kind==='enemy'){ const so=SOLDIERS[r.id]; if(so&&so!==ps&&so.alive&&!sameTeam(so.team,PLAYER_TEAM)){ aimHoverEnemy=true; aimPart=r.part; aimTarget=true; } }   // with friendly fire on, an ally can be hit but never reads as a target
    else if(r.kind==='building'&&buildings[r.id]&&buildings[r.id].alive&&teamOf(buildings[r.id].team)!==null&&!sameTeam(buildings[r.id].team,PLAYER_TEAM)){ aimTarget=true; }
    else if(r.kind==='harvester'&&harvesters[r.id]&&harvesters[r.id].alive&&teamOf(harvesters[r.id].team)!==null&&!sameTeam(harvesters[r.id].team,PLAYER_TEAM)){ aimTarget=true; }
    else if(r.kind==='building'||r.kind==='block'||r.kind==='harvester'){ aimBlocked=true; } }
}
const _ms=new THREE.Vector3(),_adir=new THREE.Vector3(),_aq=new THREE.Quaternion(),_aup=new THREE.Vector3(0,1,0);
function updateAimVisuals(){
  if(!sightLine)return;
  const show=ps.ads&&!ps.dead&&!gameOver&&!paused&&!(curW()&&curW().charge);   // torque bow suppresses the classic aim line -- its charge arc IS the aim visual (no doubled line)
  sightLine.visible=show; heightLine.visible=show;
  if(!show)return;
  // sight line: white over open ground, dark grey on any solid structure (wall/cover/building/harvester), red ONLY on an enemy soldier
  getMuzzle(player,_ms);
  _adir.copy(aimPoint).sub(_ms); const L=_adir.length();
  if(L>0.01){ _adir.multiplyScalar(1/L); _aq.setFromUnitVectors(_aup,_adir);
    sightLine.position.copy(_ms); sightLine.quaternion.copy(_aq); sightLine.scale.set(1,L,1);
    sightLine.material.color.setHex(aimHoverEnemy?0xff4d4d:(aimTarget||aimBlocked)?0x33373d:0xffffff); }
  // yellow Z-height line: stands from the ground up to the exact shot height (tall=head, short=legs)
  const h=Math.max(0.001,aimPoint.y);
  heightLine.position.set(aimPoint.x,0,aimPoint.z); heightLine.quaternion.set(0,0,0,1); heightLine.scale.set(1,h,1);
}

/* weapons */
function switchWeapon(i){if(i===ps.curWeapon||ps.dead||gameOver||mode!=='game'||!loadout[i]||!loadout[i].owned)return;endCharge();ps.curWeapon=i;ps.wpnSwap=1;ps.reloading=false;ps.fireCd=0.12;ps.spread=curW().spreadHip;ps.swinging=false;document.getElementById('reloadBar').style.opacity='0';updateHeldModel();updateWeaponHUD();}

// ===== UNIT-TYPE SYSTEM (player) — select a class, each with a primary weapon + one ability =====
function applyUnit(i){
  const u=unitOf(i); ps.unit=i; endCharge();
  ps.maxHealth=u.maxHealth; if(ps.health>ps.maxHealth) ps.health=ps.maxHealth;
  ps.molotovMode=false; ps.swapCd=0; ps.banzai=false; ps.banzaiT=0; ps.banzaiCd=0; ps.bayonet=false; ps.bayonetT=0; ps.bayonetCd=0; ps.burstLeft=0;
  ps.shieldOwned=(u.ability==='shield'); ps.shieldOn=false;
  ps.curWeapon=u.wpn; ps.spread=curW().spreadHip; ps.reloading=false; ps.fireCd=0.12; ps.swinging=false; ps.wpnSwap=1;
  document.getElementById('reloadBar').style.opacity='0';
  updateHeldModel(); updateWeaponHUD(); updateHealthHUD(); updateAbilityHUD();
}
function selectUnit(i){ if(ps.dead||gameOver||mode!=='game'||i===ps.unit) return; applyUnit(i); showLevelToast('Unit',unitOf(i).name); }
// number-key weapon slots: 1 = current class weapon, 3 = bought special weapon (class abilities stay on F)
// DEV: after "Unlock All Weapons", the mouse wheel / key 2 cycle every gun on any class (normal play stays class-locked).
let _devWheelT=0;
function devCycleGun(dir){ if(!DEV.allGuns||mode!=='game'||ps.dead||ps.banzai||ps.shieldOn||pilotJug) return;
  const guns=[]; for(let i=0;i<WEAPONS.length;i++){ const w=WEAPONS[i]; if(GUN_KIT[w.id]&&!w.melee&&!w.thrown&&loadout[i]&&loadout[i].owned) guns.push(i); }
  if(!guns.length) return; let k=guns.indexOf(ps.curWeapon); k=(k<0)?0:(k+dir+guns.length)%guns.length;
  ps.molotovMode=false; switchWeapon(guns[k]); updateAbilityHUD(); }
function equipPrimary(){ const u=unitOf(ps.unit); ps.molotovMode=false; switchWeapon(u.wpn); updateAbilityHUD(); }
function equipSpecial(){ const si=WEAPONS.findIndex(w=>w.special); if(si<0)return; if(!loadout[si]||!loadout[si].owned){ showLevelToast('Special Weapon','Buy it in the Armory (E)'); return; } ps.molotovMode=false; switchWeapon(si); }
function unitStatus(i){ const u=UNIT_TYPES[i]; if(!u) return {state:'era'};
  if(ps.unit===i) return {state:'active'};
  if(unitOwned[i]) return {state:'equip'};
  if((u.tier||1) > teamEco[PLAYER_TEAM].era) return {state:'era'};
  if((u.cost||0)>0 && credits < u.cost*priceMult()) return {state:'poor'};
  return {state:(u.cost||0)>0 ? 'buy' : 'equip'}; }
function pickClass(i){ const u=UNIT_TYPES[i]; const st=unitStatus(i);
  if(st.state==='active'||st.state==='era'||st.state==='poor') return;
  if(!unitOwned[i]){ if((u.cost||0)>0){ credits-=Math.round(u.cost*priceMult()); updateCreditHUD(); showLevelToast('Class unlocked',u.name); } unitOwned[i]=true; }
  selectUnit(i); renderShop(); }
/* ==================== JUGGERNAUT — walking tri-cannon artillery mech (MoesTools port; native primitive rebuild) ==================== */
const juggernauts=[];
let pilotJug=null, nearJugObj=null;
const JUG={ targetH:5.6, maxSpeed:6, speed:3.2, range:34, fireInterval:3.4, rippleMs:80 };   // [dial] scale/speed/cadence (targetH ~3.1x soldier)
const JUG_PILOT={ moveSpeed:5.5, turn:3.2, aimTurn:9 }, JUG_ENTER_R=5;   // [dial] drive feel + enter radius
const JUG_CANNON={ damage:130, range:130, tracer:0xffd27a, headMult:1 };                      // siege shell
const _jugV=new THREE.Vector3(), _jugV2=new THREE.Vector3(), _jugV3=new THREE.Vector3(), _JUP=new THREE.Vector3(0,1,0);
function buildJuggernaut(teamColor, accent){
  const HALF=Math.PI/2;
  const tc=new THREE.Color(teamColor!==undefined?teamColor:0xD41414), ac=new THREE.Color(accent!==undefined?accent:0xffd24a);
  const SLOT={ primary:{c:tc.clone(),r:0.82,m:0.06}, secondary:{c:tc.clone().multiplyScalar(0.7),r:0.8,m:0.08},
    metal:{c:new THREE.Color(0x6b7280),r:0.5,m:0.55}, dark:{c:new THREE.Color(0x1b1e24),r:0.6,m:0.3},
    glass:{c:new THREE.Color(0x141821).lerp(tc,0.22),r:0.3,m:0.1}, accent:{c:ac.clone(),r:0.45,m:0.1} };
  function part(geo, slot, o){ o=o||{}; const sd=SLOT[slot]||SLOT.primary;
    const mesh=new THREE.Mesh(geo,new THREE.MeshStandardMaterial({color:sd.c.clone(),roughness:sd.r,metalness:sd.m}));
    mesh.castShadow=true; mesh.receiveShadow=true; mesh.userData.part='jug';
    if(o.pos)mesh.position.set(o.pos[0],o.pos[1],o.pos[2]);
    if(o.rot)mesh.rotation.set(o.rot[0],o.rot[1],o.rot[2]);
    if(o.scale)mesh.scale.set(o.scale[0],o.scale[1],o.scale[2]); return mesh; }
  const box=(w,h,d)=>new THREE.BoxGeometry(w,h,d), cyl=(a,b,h,se)=>new THREE.CylinderGeometry(a,b,h,se);
  const g=new THREE.Group(), lift=new THREE.Group(); g.add(lift);
  const body=new THREE.Group(), upper=new THREE.Group(), cannonGroup=new THREE.Group();
  body.add(upper); body.add(cannonGroup); lift.add(body);
  // pelvis (root base)
  upper.add(part(box(1.78,0.98,1.4),'primary',{pos:[0,0.02,0]}));
  upper.add(part(box(1.5,0.3,1.2),'dark',{pos:[0,-0.5,0]}));
  upper.add(part(box(1.9,0.2,1.0),'metal',{pos:[0,0.45,0]}));
  // upper torso
  upper.add(part(box(2.3,0.5,1.7),'primary',{pos:[0,0.78,0]}));
  upper.add(part(box(2.4,0.14,1.8),'secondary',{pos:[0,1.04,0]}));
  upper.add(part(box(2.0,0.7,0.9),'primary',{pos:[0,1.35,-0.5]}));
  upper.add(part(box(0.6,0.5,0.7),'secondary',{pos:[-0.73,1.7,-0.45]}));
  upper.add(part(box(0.6,0.5,0.7),'secondary',{pos:[0.73,1.7,-0.45]}));
  upper.add(part(box(0.5,0.3,0.5),'dark',{pos:[0,1.78,-0.5]}));
  // cockpit pods
  for(const sd of [-1,1]){
    upper.add(part(box(0.5,0.6,0.85),'metal',{pos:[sd*1.05,0.7,0.25]}));
    upper.add(part(cyl(0.46,0.46,1.15,6),'glass',{pos:[sd*1.3,0.74,0.35],rot:[HALF,0,0],scale:[1,1,0.6]}));
    upper.add(part(box(0.3,0.12,0.5),'accent',{pos:[sd*1.3,0.45,0.35]}));
  }
  // three cannons + muzzle nodes
  const muzzles=[];
  for(const cx of [-0.52,0,0.52]){
    cannonGroup.add(part(box(0.42,0.56,0.95),'primary',{pos:[cx,1.12,-0.35],rot:[0.12,0,0]}));
    cannonGroup.add(part(cyl(0.135,0.135,2.7,18),'metal',{pos:[cx,1.18,1.0],rot:[HALF,0,0]}));
    cannonGroup.add(part(cyl(0.175,0.175,0.34,18),'dark',{pos:[cx,1.18,2.34],rot:[HALF,0,0]}));
    cannonGroup.add(part(cyl(0.15,0.15,0.16,18),'accent',{pos:[cx,1.18,0.5],rot:[HALF,0,0]}));
    const mz=new THREE.Object3D(); mz.position.set(cx,1.18,2.5); cannonGroup.add(mz); muzzles.push(mz);
  }
  // legs: jointed hip->knee->ankle. Rest pose (reverse-joint) is BAKED into mesh tilts, so pivots start at 0 (deterministic, no re-parent math).
  function buildLeg(side){
    const leg=new THREE.Group(); leg.position.set(side*0.84,-0.12,0);
    leg.add(part(cyl(0.36,0.36,0.54,22),'metal',{pos:[0,0,0],rot:[0,0,HALF]}));               // fixed hip axle
    const hip=new THREE.Object3D(); leg.add(hip);
    hip.add(part(box(0.5,1.0,0.58),'primary',{pos:[0,-0.48,-0.21],rot:[0.42,0,0]}));           // thigh (baked back-rake)
    const knee=new THREE.Object3D(); knee.position.set(0,-0.95,-0.42); hip.add(knee);
    knee.add(part(cyl(0.4,0.4,0.58,22),'metal',{pos:[0,0,0],rot:[0,0,HALF]}));                 // knee axle
    knee.add(part(box(0.56,1.0,0.64),'primary',{pos:[0,-0.425,0.28],rot:[-0.58,0,0]}));        // shin (baked forward-down)
    knee.add(part(box(0.48,0.74,0.12),'secondary',{pos:[0,-0.4,0.58],rot:[-0.58,0,0]}));       // shin plate
    const ankle=new THREE.Object3D(); ankle.position.set(0,-0.87,0.56); knee.add(ankle);
    ankle.add(part(box(0.52,0.42,0.5),'metal',{pos:[0,0,0]}));                                 // ankle block
    ankle.add(part(cyl(0.64,0.5,0.26,4),'primary',{pos:[0,-0.18,0.08],rot:[0,HALF/2,0]}));     // foot pad (4-sided wedge)
    ankle.add(part(box(0.22,0.18,0.32),'metal',{pos:[-0.15,-0.22,0.41]}));                     // toe L
    ankle.add(part(box(0.22,0.18,0.32),'metal',{pos:[0.15,-0.22,0.41]}));                      // toe R
    return {leg,hip,knee,ankle};
  }
  const L=buildLeg(-1), R=buildLeg(1); lift.add(L.leg); lift.add(R.leg);
  // ground feet to y=0 + scale to ~2.3x soldier
  g.updateMatrixWorld(true);
  const bb=new THREE.Box3().setFromObject(lift); lift.position.y=-bb.min.y;
  const hh=Math.max(0.001,bb.max.y-bb.min.y), scl=JUG.targetH/hh; g.scale.setScalar(scl);
  // toon + ink outline by hand (stylizeScene only runs once at init; this unit spawns at runtime)
  if(typeof stylizeOne==='function') stylizeOne(g);
  // expose (mirrors buildCharacter)
  g.userData.joints={ hipL:L.hip, kneeL:L.knee, ankleL:L.ankle, hipR:R.hip, kneeR:R.knee, ankleR:R.ankle };
  g.userData.body=body; g.userData.cannonGroup=cannonGroup; g.userData.muzzles=muzzles;
  g.userData.baseY=body.position.y; g.userData.phase=0; g.userData.recoil=0;
  g.userData.rest={hip:0,knee:0,ankle:0};   // rest pose baked into the mesh tilts
  g.userData.isJuggernaut=true;
  return g;
}
function _jugFootfall(g,ph,side){
  const A=g.userData._foot||(g.userData._foot={L:1,R:1});
  const s=Math.sin(ph), prev=A[side]; A[side]=s;
  if(prev>0 && s<=0){ const ank=(side==='L')?g.userData.joints.ankleL:g.userData.joints.ankleR; if(!ank)return;
    ank.getWorldPosition(_jugV); _jugV.y=0.12;
    if(typeof addImpact==='function') addImpact(_jugV,_JUP,'dirt');
    if(typeof camera!=='undefined' && typeof camShake!=='undefined'){ const cd=camera.position.distanceTo(_jugV); const sh=(cd<60?(1-cd/60):0)*0.26; if(sh>0)camShake=Math.min(1.3,camShake+sh); }
    if(typeof sfxThunk==='function') sfxThunk();
  }
}
function animateJuggernaut(g, speed, dt, t, fireKick){
  const J=g.userData.joints, rest=g.userData.rest; if(!J)return;
  dt=Math.max(1e-4,Math.min(0.05,dt));
  let gs=(speed-0.02)/Math.max(0.1,JUG.maxSpeed-0.02); gs=gs<0?0:gs>1?1:gs; const gait=gs*gs*(3-2*gs);
  const stepHz=0.55+0.9*gait; g.userData.phase+=dt*stepHz*Math.PI*2;
  const pL=g.userData.phase, pR=pL+Math.PI;
  const A_hip=0.34*gait, A_knee=0.55*gait, A_toe=0.14*gait;
  function driveLeg(hip,knee,ankle,ph){ const s=Math.sin(ph), swing=s>0?s:0;
    const hipD=A_hip*s, kneeD=-A_knee*swing, toeOff=A_toe*Math.max(0,Math.sin(ph-0.6));
    const ankleD=-(0.5*hipD)-(0.6*kneeD)+toeOff;
    hip.rotation.x=rest.hip+hipD; knee.rotation.x=rest.knee+kneeD; ankle.rotation.x=rest.ankle+ankleD; }
  driveLeg(J.hipL,J.kneeL,J.ankleL,pL); driveLeg(J.hipR,J.kneeR,J.ankleR,pR);
  const body=g.userData.body, baseY=g.userData.baseY;
  const bob=0.10*gait*(0.5-0.5*Math.cos(2*pL));
  body.position.y=baseY-bob+(gait<0.05?0.012*Math.sin(t*1.6):0);
  body.position.x=0.06*gait*Math.sin(pL);
  body.rotation.x=0.05*gait; body.rotation.z=0.02*gait*Math.sin(pL);
  g.userData.recoil=Math.max(0,g.userData.recoil+(fireKick||0)-dt*6);
  const cg=g.userData.cannonGroup; cg.position.z=-0.12*g.userData.recoil; cg.rotation.x=0.05*g.userData.recoil;
  _jugFootfall(g,pL,'L'); _jugFootfall(g,pR,'R');
}
function enterJug(jug){ if(!jug||!jug.alive)return; pilotJug=jug; const cg=jug.group.userData.cannonGroup; jug._aimYaw=cg?cg.rotation.y:0; player.visible=false; ps.vel.set(0,0,0); ps.ads=false; mouseDown=false; if(typeof sfxThunk==='function')sfxThunk(); showLevelToast('Piloting Juggernaut','WASD drive \u00b7 aim + hold to fire'); }
function exitJug(){ const jug=pilotJug; pilotJug=null; player.visible=true; mouseDown=false; if(jug&&jug.group){ const g=jug.group, ry=g.rotation.y; player.position.set(g.position.x+Math.cos(ry)*3.4,0,g.position.z-Math.sin(ry)*3.4); player.rotation.y=ry+Math.PI; ps.vel.set(0,0,0); } }
function _jugFirePilot(jug, idx){
  const g=jug.group, mz=g.userData.muzzles[idx]; if(!mz)return;
  mz.getWorldPosition(_jugV);
  if(typeof addMuzzle==='function') addMuzzle(_jugV, 0xffe0a0);
  g.userData.recoil=Math.min(2.6, g.userData.recoil+1);
  _jugV2.copy(aimGround); _jugV2.y+=0.6; _jugV2.sub(_jugV);
  if(_jugV2.lengthSq()<1e-6){ const ry=g.rotation.y; _jugV2.set(Math.sin(ry),0,Math.cos(ry)); }
  _jugV2.normalize();
  if(typeof syncSimWorld==='function' && typeof resolveShot==='function'){
    syncSimWorld();
    const r=resolveShot(SIM,SIM_PLAYER,_jugV.x,_jugV.y,_jugV.z,_jugV2.x,_jugV2.y,_jugV2.z,JUG_CANNON.range,teamFilter(PLAYER_TEAM));
    _jugV3.set(r.px,r.py,r.pz);
    if(typeof addTracer==='function') addTracer(_jugV,_jugV3,JUG_CANNON.tracer);
    if(r.kind==='enemy'){ const en=SOLDIERS[r.id]; if(en&&en!==ps&&en.alive) damageEnemy(en,JUG_CANNON.damage,r.part,_jugV3,true); }
    else if(r.kind==='building'){ const b=buildings[r.id]; if(b&&b.alive&&teamOf(b.team)!==null&&!sameTeam(b.team,PLAYER_TEAM)&&typeof damageBuilding==='function') damageBuilding(b,JUG_CANNON.damage); }
    if(typeof addExplosion==='function') addExplosion(_jugV3);
  }
}
function tickPilot(dt, tt){
  const jug=pilotJug; if(!jug||!jug.alive||ps.dead||gameOver){ exitJug(); return; }
  const g=jug.group;
  const _cyaw=camYaw+T.camYawBase, cy=Math.cos(_cyaw), sy=Math.sin(_cyaw);
  const fX=-sy, fZ=-cy, rX=cy, rZ=-sy;
  const wish=_jugV.set(0,0,0);
  if(keys['KeyW']){wish.x+=fX;wish.z+=fZ;} if(keys['KeyS']){wish.x-=fX;wish.z-=fZ;}
  if(keys['KeyD']){wish.x+=rX;wish.z+=rZ;} if(keys['KeyA']){wish.x-=rX;wish.z-=rZ;}
  let sp=0;
  if(wish.lengthSq()>1e-4){ wish.normalize();
    const mYaw=Math.atan2(wish.x,wish.z);
    let dy=mYaw-g.rotation.y; while(dy>Math.PI)dy-=6.283185307; while(dy<-Math.PI)dy+=6.283185307;
    g.rotation.y+=dy*Math.min(1,dt*JUG_PILOT.turn);
    sp=JUG_PILOT.moveSpeed;
    g.position.x+=wish.x*sp*dt; g.position.z+=wish.z*sp*dt;
    const lX=T.arenaX-2, lZ=T.arenaZ-2; g.position.x=Math.max(-lX,Math.min(lX,g.position.x)); g.position.z=Math.max(-lZ,Math.min(lZ,g.position.z));
  }
  const cg=g.userData.cannonGroup;
  if(cg){ const aimYaw=Math.atan2(aimGround.x-g.position.x, aimGround.z-g.position.z); let local=aimYaw-g.rotation.y;
    while(local>Math.PI)local-=6.283185307; while(local<-Math.PI)local+=6.283185307;
    local=Math.max(-2.6,Math.min(2.6,local));
    if(jug._aimYaw==null)jug._aimYaw=local;
    let da=local-jug._aimYaw; while(da>Math.PI)da-=6.283185307; while(da<-Math.PI)da+=6.283185307;
    jug._aimYaw+=da*Math.min(1,dt*JUG_PILOT.aimTurn);
    cg.rotation.y=jug._aimYaw;
  }
  if(typeof animateJuggernaut==='function') animateJuggernaut(g, sp, dt, tt, 0);
  jug.fireCd-=dt;
  if(jug.rippleActive){ jug.rippleT-=dt; if(jug.rippleT<=0){ _jugFirePilot(jug,jug.rippleIdx); jug.rippleIdx++; jug.rippleT=JUG.rippleMs/1000; if(jug.rippleIdx>=3){ jug.rippleActive=false; jug.fireCd=JUG.fireInterval; } } }
  else if(mouseDown && jug.fireCd<=0){ jug.rippleActive=true; jug.rippleIdx=0; jug.rippleT=0; }
  player.position.set(g.position.x,0,g.position.z); player.rotation.y=g.rotation.y;
}
function tickJuggernauts(dt, tt){
  if(!juggernauts.length)return;
  // Deployed vehicle: idle in place (hydraulic hum) until entered + driven. It does NOT hunt, auto-fire, or farm XP.
  // The walk cycle + _jugFireBarrel stay ready for the player-drive pass; a driver will set jug.speed (>0) and trigger fire.
  for(let i=0;i<juggernauts.length;i++){ const jug=juggernauts[i]; if(!jug.alive||jug===pilotJug)continue;
    if(typeof animateJuggernaut==='function') animateJuggernaut(jug.group, jug.speed||0, dt, tt, 0);
  }
}
function vehicleStatus(id){ const v=VEHICLE_TYPES.find(x=>x.id===id); if(!v) return {state:'na'};
  if((v.tier||1) > teamEco[PLAYER_TEAM].era) return {state:'era'};
  if((v.cost||0)>0 && credits < v.cost*priceMult()) return {state:'poor'};
  return {state:'buy'}; }
function buyVehicle(id){ const v=VEHICLE_TYPES.find(x=>x.id===id); if(!v) return; if(vehicleStatus(id).state!=='buy') return;
  credits-=Math.round((v.cost||0)*priceMult()); updateCreditHUD(); showLevelToast('Vehicle purchased', v.name);
  closeShop(); if(typeof spawnVehicle==='function') spawnVehicle(id); }
// Vehicle deployment: builds the Juggernaut beside the player in the player's faction livery (hull = faction primary,
// accents = highlight trim); enterJug / tickPilot / exitJug drive it.
function spawnVehicle(id){ const v=VEHICLE_TYPES.find(x=>x.id===id); if(!v)return;
  if(id==='juggernaut'){ if(typeof buildJuggernaut!=='function'){ showLevelToast('Vehicle', v.name+' unavailable'); return; }
    const pal=FACTION_PALETTES[PLAYER_FACTION]||FACTION_PALETTES.Allies; const g=buildJuggernaut(pal.FactionPrimary, pal.HighlightTrim);
    const ry=player.rotation.y, bx=player.position.x-Math.sin(ry)*7, bz=player.position.z-Math.cos(ry)*7;
    g.position.set(bx,0,bz); g.rotation.y=ry; scene.add(g);
    juggernauts.push({ group:g, alive:true, health:v.maxHealth||1500, target:null, fireCd:1.2, rippleActive:false, rippleIdx:0, rippleT:0 });
    showLevelToast('Juggernaut deployed', 'Heavy artillery online');
  } else { showLevelToast('Deploying', v.name+' \u2014 coming soon'); } }
function useAbility(){
  if(ps.dead||gameOver||!started||ps.channeling) return; const u=unitOf(ps.unit);
  if(u.ability==='molotov'){
    if(ps.swapCd>0) return; ps.molotovMode=!ps.molotovMode; ps.swapCd=u.swapCd;
    ps.curWeapon = ps.molotovMode ? u.altWpn : u.wpn; ps.spread=curW().spreadHip; ps.reloading=false; ps.fireCd=0.15; ps.swinging=false; ps.wpnSwap=1;
    document.getElementById('reloadBar').style.opacity='0'; updateHeldModel(); updateWeaponHUD(); updateAbilityHUD();
    showLevelToast(ps.molotovMode?'Molotov ready':'Rifle ready', u.name);
  } else if(u.ability==='shield'){
    if(ps.swapCd>0) return; ps.shieldOn=!ps.shieldOn; ps.swapCd=u.swapCd; updateHeldModel(); updateAbilityHUD(); if(typeof sfxClank==='function')sfxClank();
    showLevelToast(ps.shieldOn?'Shield up':'Shield down', u.name);
  } else if(u.ability==='banzai'){
    if(ps.banzai||ps.banzaiCd>0) return; ps.banzai=true; ps.banzaiT=u.banzaiDur;
    ps.curWeapon=u.altWpn; ps.spread=0; ps.reloading=false; ps.fireCd=0.1; ps.swinging=false; ps.wpnSwap=1;
    document.getElementById('reloadBar').style.opacity='0'; updateHeldModel(); updateWeaponHUD(); updateAbilityHUD();
    if(typeof spawnComic==='function') spawnComic('BANZAI!', player.position.clone().setY(2.4), 2.8, '#FF7B00');
    if(typeof sfxThunk==='function') sfxThunk();
  } else if(u.ability==='bayonet'){
    if(ps.bayonet||ps.bayonetCd>0) return; ps.bayonet=true; ps.bayonetT=u.bayonetDur; ps._bayHit=new Set();
    ps.reloading=false; ps.fireCd=0.1; ps.swinging=false; ps.burstLeft=0;
    updateHeldModel(); updateWeaponHUD(); updateAbilityHUD();
    if(typeof spawnComic==='function') spawnComic('CHARGE!', player.position.clone().setY(2.4), 2.6, '#FF7B00');
    if(typeof sfxThunk==='function') sfxThunk();
  }
}
function endBanzai(){ if(!ps.banzai) return; const u=unitOf(ps.unit);
  ps.banzai=false; ps.banzaiT=0; ps.banzaiCd=u.banzaiCd;
  ps.curWeapon=u.wpn; ps.spread=curW().spreadHip; ps.fireCd=0.15; ps.swinging=false; ps.wpnSwap=1;
  updateHeldModel(); updateWeaponHUD(); updateAbilityHUD(); }
function endBayonet(){ if(!ps.bayonet) return; const u=unitOf(ps.unit);
  ps.bayonet=false; ps.bayonetT=0; ps.bayonetCd=u.bayonetCd||10; ps.fireCd=0.2;
  updateWeaponHUD(); updateAbilityHUD(); }
function throwMolotov(){ animThrow(player);
  getMuzzle(player,_p2); throwVelocity(_thrV,_p2,'nade');
  const m=new THREE.Mesh(new THREE.SphereGeometry(0.24,10,8), new THREE.MeshStandardMaterial({ color:0x5a2f16, roughness:0.5, emissive:0xff5a1a, emissiveIntensity:0.6 }));
  m.position.copy(_p2); m.castShadow=true; noRay(m); stylizeOne(m,0.035); scene.add(m);
  grenades.push({ mesh:m, vel:_thrV.clone(), fuse:1.4, molotov:true });
  camRecoil+=0.6; camShake+=0.1;
}
function startReload(){const w=curW(),l=curL();if(ps.reloading||l.ammo===w.mag||l.reserve<=0||ps.dead)return;ps.reloading=true;ps.reloadT=0;sfxReload(0);document.getElementById('reloadBar').style.opacity='1';}
const muzzleW=new THREE.Vector3(),aimDir=new THREE.Vector3(); let camRecoil=0,camShake=0;
const _sRight=new THREE.Vector3(),_sUpv=new THREE.Vector3(),_sDir=new THREE.Vector3();   // reusable shot-spread basis (B1: was 4 Vector3/pellet)
function getMuzzle(s,out){ return s.userData.muzzle.getWorldPosition(out); }
// Gameplay fire origin (player): a stable chest-height point just ahead of the body on the gun side, along the
// facing — independent of animation and never inside a wall. The SIM ray starts here; muzzle FX + tracers still
// start at the animated muzzle. FIRE_H matches the old aim-pose muzzle height, so firing over crates is unchanged.
const FIRE_H=1.9, FIRE_R=0.3, FIRE_F=0.35, _fireO=new THREE.Vector3();
function playerFireOrigin(out){ const ry=player.rotation.y, s=Math.sin(ry), c=Math.cos(ry);
  return out.set(player.position.x+s*FIRE_F+c*FIRE_R, player.position.y+FIRE_H, player.position.z+c*FIRE_F-s*FIRE_R); }
// Held gun model per soldier (GUN_KIT): built once per soldier, others hidden; the muzzle marker follows. Cosmetic.
function setHeldGun(g, id){ const u=g.userData, wpn=u.wpn; if(!wpn) return; const k=GUN_KIT[id]?id:'amk';
  if(!u.guns){ u.guns={}; for(let i=0;i<wpn.children.length;i++){ const c=wpn.children[i]; if(c!==u.muzzle&&c.isGroup){ u.guns.amk=c; break; } } }
  if(!u.guns[k]){ const m=buildGunModel(k); stylizeOne(m); wpn.add(m); u.guns[k]=m; }
  for(const key in u.guns) u.guns[key].visible=(key===k);
  const mz=GUN_KIT[k].muzzle; if(u.muzzle) u.muzzle.position.set(mz[0],mz[1],mz[2]); u.gunId=k; }
function pitchWeapon(s,target){
  const wpn=s.userData.wpn; wpn.getWorldPosition(_p1);
  const dy=target.y-_p1.y, horiz=hyp2(target.x-_p1.x,target.z-_p1.z);
  let e=Math.atan2(dy,Math.max(0.001,horiz)); e=Math.max(-T.weaponPitchMax,Math.min(T.weaponPitchMax,e));
  wpn.rotation.x=-e;
}
/* ============================================================================
 * HEADLESS SIM CORE — ZERO THREE.js / DOM. Identical to sim-core.js; portable
 * verbatim to a Node server. Hit detection is pure capsule/sphere/AABB math, so
 * it runs server-side and can be replayed against rewound lag-comp snapshots.
 * (Stage 1: decouple simulation from the view layer. Tick = 60 Hz authoritative.)
 * ========================================================================== */
let _frameId=0, _simFrame=-1;   // once-per-frame guard for syncSimWorld (F1): all in-frame callers capture identical positions
const SIM_PLAYER=0;   // the player's SIM id and SOLDIERS slot; a bot's SIM id is its slot en.id (= enemies index + 1); every player-side shot passes it, so it never self-hits
const _hpV=new THREE.Vector3(), _hnV=new THREE.Vector3(), _dmgPt=new THREE.Vector3();
function syncSimWorld(force){
  if(!force && _simFrame===_frameId) return;   // F1: already synced this frame; positions dont change between in-frame callers
  _simFrame=_frameId;
  const P=SIM.players; P.length=SOLDIERS.length;   // one entry per slot: P[s] is SOLDIERS[s] (P[0] the player, id SIM_PLAYER); parked slots are alive=false
  { let e=P[0]||(P[0]={id:0,x:0,y:0,z:0,alive:false,stance:0,team:null}); const pos=player.position; e.id=SIM_PLAYER; e.x=pos.x; e.y=pos.y; e.z=pos.z; e.alive=!ps.dead; e.stance=0; e.team=PLAYER_TEAM; }
  for(let s=1;s<SOLDIERS.length;s++){ const en=SOLDIERS[s]; let e=P[s]||(P[s]={id:0,x:0,y:0,z:0,alive:false,stance:0,team:null}); const pos=en.group.position; e.id=en.id; e.x=pos.x; e.y=pos.y; e.z=pos.z; e.alive=en.active&&en.alive; e.stance=0; e.team=en.team; }
  const S=SIM.statics; let n=0; for(let i=0;i<S.length;i++) S[i].rot=false;   // slots are reused: only oriented obstacles set rot below
  for(let i=0;i<buildings.length;i++){ const b=buildings[i],bx=b.box; let s=S[n]||(S[n]={}); s.kind='building'; s.id=i; s.alive=b.alive; s.minx=bx.min.x;s.miny=bx.min.y;s.minz=bx.min.z;s.maxx=bx.max.x;s.maxy=bx.max.y;s.maxz=bx.max.z; n++; }
  for(let i=0;i<obstacles.length;i++){ const o=obstacles[i],bx=o.box; let s=S[n]||(S[n]={}); s.kind='block'; s.id=-1; s.alive=true; s.minx=bx.min.x;s.miny=bx.min.y;s.minz=bx.min.z;s.maxx=bx.max.x;s.maxy=bx.max.y;s.maxz=bx.max.z;
    const th=o.mesh.rotation.y; if(Math.abs(Math.sin(2*th))>1e-3&&o.w>0&&o.d>0){ s.rot=true; s.cx=o.mesh.position.x; s.cz=o.mesh.position.z; s.rc=Math.cos(th); s.rs=Math.sin(th); s.hw=o.w*0.5; s.hd=o.d*0.5; } n++; }   // oriented box for 45° pieces
  for(let i=0;i<harvesters.length;i++){ const hv=harvesters[i]; if(!hv.alive)continue; const hp=hv.group.position; let s=S[n]||(S[n]={}); s.kind='harvester'; s.id=i; s.alive=true; s.minx=hp.x-hv.hx;s.miny=0;s.minz=hp.z-hv.hz;s.maxx=hp.x+hv.hx;s.maxy=hv.hy;s.maxz=hp.z+hv.hz; n++; }
  for(let i=0;i<platforms.length;i++){ const bx=platforms[i].box; let s=S[n]||(S[n]={}); s.kind='block'; s.id=-1; s.alive=true; s.minx=bx.min.x;s.miny=bx.min.y;s.minz=bx.min.z;s.maxx=bx.max.x;s.maxy=bx.max.y;s.maxz=bx.max.z; n++; }
  const rampLo=n;   // ramp slots: their boxes stand in for slopes, so bot fire skips them (main's losMeshes never held ramps)
  for(let i=0;i<ramps.length;i++){ const bx=ramps[i].box; let s=S[n]||(S[n]={}); s.kind='block'; s.id=-1; s.alive=true; s.minx=bx.min.x;s.miny=bx.min.y;s.minz=bx.min.z;s.maxx=bx.max.x;s.maxy=bx.max.y;s.maxz=bx.max.z; n++; }
  const rampHi=n;
  for(let i=0;i<houses.length;i++){ const bs=houses[i].boxes||[houses[i].box]; for(let k=0;k<bs.length;k++){ const bx=bs[k]; let s=S[n]||(S[n]={}); s.kind='block'; s.id=-1; s.alive=true; s.minx=bx.min.x;s.miny=bx.min.y;s.minz=bx.min.z;s.maxx=bx.max.x;s.maxy=bx.max.y;s.maxz=bx.max.z; n++; } }
  for(let i=0;i<covers.length;i++){ if(!covers[i].los||!covers[i].boxes)continue; const bs=covers[i].boxes; for(let k=0;k<bs.length;k++){ const bx=bs[k]; let s=S[n]||(S[n]={}); s.kind='block'; s.id=-1; s.alive=true; s.minx=bx.min.x;s.miny=bx.min.y;s.minz=bx.min.z;s.maxx=bx.max.x;s.maxy=bx.max.y;s.maxz=bx.max.z; n++; } }
  for(let i=0;i<trees.length;i++){ if(!trees[i].los||!trees[i].box)continue; const bx=trees[i].box; let s=S[n]||(S[n]={}); s.kind='block'; s.id=-1; s.alive=true; s.minx=bx.min.x;s.miny=bx.min.y;s.minz=bx.min.z;s.maxx=bx.max.x;s.maxy=bx.max.y;s.maxz=bx.max.z; n++; }for(let i=0;i<props.length;i++){ if(!props[i].los||!props[i].box)continue; const bx=props[i].box; let s=S[n]||(S[n]={}); s.kind='block'; s.id=-1; s.alive=true; s.minx=bx.min.x;s.miny=bx.min.y;s.minz=bx.min.z;s.maxx=bx.max.x;s.maxy=bx.max.y;s.maxz=bx.max.z; n++; }
  S.length=n;
  const B=_botFireWorld.statics; let m=0; for(let i=0;i<n;i++){ if(i<rampLo||i>=rampHi) B[m++]=S[i]; } B.length=m;
}

function fireOnce(){
  const w=curW(),l=curL(); if(ps.reloading||ps.fireCd>0||ps.dead||gameOver||ps.channeling||ps.shieldOn)return;
  if(w.thrown) return; // thrown weapons charge on hold and launch on release (see charge system)
  if(w.melee){ startSwing(w); ps.fireCd=w.fireInterval; return; }
  if(w.id==='molotov'){ throwMolotov(); ps.fireCd=w.fireInterval; updateWeaponHUD(); return; }
  if(l.ammo<=0){ if(l.reserve>0) startReload(); else dryFire(); return; }
  l.ammo--; ps.fireCd=w.burst?w.burstInterval:w.fireInterval; ps.fireKick=Math.min(1.1, ps.fireKick + 0.4 + w.recoil*0.05);
  getMuzzle(player,muzzleW); playerFireOrigin(_fireO);
  aimDir.copy(aimPoint).sub(_fireO).normalize(); addMuzzle(muzzleW,w.tracer); comicGun(muzzleW); sfxShot(w); addCasing();
  for(let p=0;p<w.pellets;p++){
    _sRight.crossVectors(aimDir,_aup).normalize();
    _sUpv.crossVectors(_sRight,aimDir).normalize();
    const ax=(Math.random()*2-1)*ps.spread,ay=(Math.random()*2-1)*ps.spread;
    _sDir.copy(aimDir).addScaledVector(_sRight,Math.tan(ax)).addScaledVector(_sUpv,Math.tan(ay)).normalize();
    castShot(_fireO,_sDir,w,muzzleW);
  }
  ps.spread=Math.min(w.spreadMax,ps.spread+w.spreadPerShot); camRecoil+=w.recoil; camShake+=w.recoil*0.187; const _fovK=fovBase/52; /* scale FOV recoil-kick to the active base fov (ref 52deg): keeps the tuned punch on the follow cams, stops the iso fov-15 nausea-zoom — the RELATIVE zoom stays constant on every camera style */ fovKick=Math.min(7*_fovK,fovKick+(1.85+w.recoil*0.68)*_fovK); updateWeaponHUD();
  if(l.ammo<=0) startReload(); // auto-reload on empty (no dead-click gap)
}
function falloffMult(w,dist){ if(!w.falloffStart||dist<=w.falloffStart) return 1; const end=w.falloffEnd||w.range; if(dist>=end) return (w.falloffMin||0.25); const t=(dist-w.falloffStart)/(end-w.falloffStart); return 1-(1-(w.falloffMin||0.25))*t; }
function castShot(origin,dir,w,vis){
  // Hit detection now runs through the HEADLESS pure-math core (resolveShot),
  // NOT THREE.Raycaster. The raycaster remains only for the cosmetic aim dot.
  syncSimWorld();
  const r=resolveShot(SIM,SIM_PLAYER, origin.x,origin.y,origin.z, dir.x,dir.y,dir.z, w.range, teamFilter(PLAYER_TEAM));
  _hpV.set(r.px,r.py,r.pz); _hnV.set(r.nx,r.ny,r.nz);
  const dmg=w.damage*falloffMult(w,origin.distanceTo(_hpV));
  addTracer(vis||origin,_hpV,w.tracer);
  if(r.kind==='enemy'){ const en=SOLDIERS[r.id]; if(en&&en!==ps&&en.alive){ damageEnemy(en,dmg,r.part,_hpV,false,w.headMult); addImpact(_hpV,_hnV,'flesh'); } else addImpact(_hpV,_hnV,'dirt'); }
  else if(r.kind==='building'){ const b=buildings[r.id]; if(b&&b.alive&&teamOf(b.team)!==null&&!sameTeam(b.team,PLAYER_TEAM)){ damageBuilding(b,dmg); showHitmarker(false,false); } addImpact(_hpV,_hnV,'metal'); }
  else if(r.kind==='harvester'){ const hv=harvesters[r.id]; if(hv&&hv.alive&&teamOf(hv.team)!==null&&!sameTeam(hv.team,PLAYER_TEAM)){ damageHarvester(hv,dmg); showHitmarker(false,false); } addImpact(_hpV,_hnV,'metal'); }
  else addImpact(_hpV,_hnV,'dirt');
}
function damageEnemy(en,dmg,part,point,flat,hsMult){
  part=part||'body';
  const mult=part==='head'?(hsMult||T.headMult):part==='legs'?T.legMult:T.bodyMult;
  const final=flat?dmg:dmg*mult; en.health-=final; en.hitFlash=0.12; en.state='engage';
  const pt=point?_dmgPt.copy(point):_dmgPt.copy(en.group.position).setY(1.6);
  const killed=en.health<=0&&en.alive;
  spawnDamageNumber(pt, Math.round(final), part==='head', killed);
  showHitmarker(killed, part==='head');
  animHit(en.group,player.position.x,player.position.z,dmg/40);if(killed){en.alive=false;{ const sp=SIM.players[en.id]; if(sp)sp.alive=false; }en.topple=0;deathStart(en.group,player.position.x,player.position.z);en.respawnT=T.enemyRespawn;kills++;en.downs=(en.downs||0)+1;credits+=T.killReward;addTech(PLAYER_TEAM,part==='head'?ECO.techPerHeadshot:ECO.techPerKill);onKill(curW().id,part==='head');if(part==='head')onHeadshotKill();else sfxKill();if(part==='head'||curW().melee)triggerVignette(en,curW().melee?'melee':'headshot');updateHostileCount();updateCreditHUD();comicKill(en.group.position.clone().setY(1.8));}
}

/* buildings dmg */
function damageBuilding(b,d){
  if(!b.alive)return; b.health-=d;
  if(b.health<=0){b.alive=false;b.sink=0;addExplosion(b.mesh.position.clone().setY(b.mesh.geometry.parameters.height/2));camShake+=0.6;b.trim.material.emissiveIntensity=0;b.label.visible=false;b.bar.grp.visible=false;updateBaseHUD();checkEnd();}
  else updateBaseHUD();
}
function nearestFriendlyBuilding(p){const bl=TEAM_INDEX[PLAYER_TEAM].blds;let best=null,bd=1e9;for(let i=0;i<bl.length;i++){const b=bl[i];if(b.alive){const d=b.mesh.position.distanceTo(p);if(d<bd){bd=d;best=b;}}}return best;}
// ---- Phase 5A: enemy verticality helpers ----
const EJUMP_REACH = T.jumpV*T.jumpV/(2*T.gravity) + T.stepUp;   // ~3.07m: highest surface an agent can gain by jumping
function rampLowEnd(o){ const c=Math.cos(o.rot), s=Math.sin(o.rot), lx=-o.w/2; return {x:o.x+c*lx, z:o.z-s*lx}; }   // world XZ of a ramp's low edge
// When the player is higher than an enemy can jump to, steer it toward the nearest ramp's base until it starts climbing; returns true if redirected.
function enemyClimbSteer(en, mv){ const eh=en.group.position.y; if(player.position.y<=eh+EJUMP_REACH) return false;
  let best=null,bd=1e9; for(let i=0;i<ramps.length;i++){ const r=ramps[i], dx=r.x-en.group.position.x, dz=r.z-en.group.position.z, d=dx*dx+dz*dz; if(d<bd){bd=d;best=r;} }
  if(!best) return false; const le=rampLowEnd(best), dLE=hyp2(le.x-en.group.position.x, le.z-en.group.position.z);
  if(dLE>3.5 && eh<0.8){ mv.set(le.x-en.group.position.x,0,le.z-en.group.position.z); if(mv.lengthSq()>0)mv.normalize(); return true; }
  return false; }

/* enemy fire */
const eM=new THREE.Vector3(), _eF=new THREE.Vector3(), _eT=new THREE.Vector3(), _eEnd=new THREE.Vector3(), _zAxis=new THREE.Vector3(0,0,1);   // enemy-fire scratch + siege impact normal (B2a)
const _eHitN=new THREE.Vector3(), _botFireWorld={ players:SIM.players, statics:[], groundY:0, _res:{}, _nrm:[0,0,0,0] };   // bot-fire impact normal + its SIM view: the same soldiers, statics without ramps (K1)
function enemyFire(en){
  en._aimT=1.4;   // raise the gun to the aim stance while shooting (cosmetic)
  const eu=unitOf(en.unit);
  const ep=en.group.position; syncSimWorld(); const P0=SIM.players[0];   // aim at the same player snapshot the hit test uses
  _eF.set(ep.x,ep.y+1.45,ep.z);_eT.set(P0.x,P0.y+1.45,P0.z);   // chest to chest, following both heights
  const dir=_v.copy(_eT).sub(_eF).normalize();eM.copy(_eF).addScaledVector(dir,0.9);addMuzzle(eM,0xff6a4a); sfxEnemyShot(eM);   // muzzle: looks only
  _sRight.crossVectors(dir,_aup).normalize();_sUpv.crossVectors(_sRight,dir).normalize();
  const ax=(Math.random()*2-1)*T.enemySpread,ay=(Math.random()*2-1)*T.enemySpread;
  const sdv=_sDir.copy(dir).addScaledVector(_sRight,Math.tan(ax)).addScaledVector(_sUpv,Math.tan(ay)).normalize();
  // B1: resolved in the SIM hit world the player's bullets use (the same soldier capsules), never the animated meshes.
  // K1: the soldiers are SIM.players, minus the bot itself and its own team; the statics leave out ramp slots, whose boxes
  // stand in for slopes. One hit world for every shooter is a multiplayer invariant (ADR-001).
  const W=_botFireWorld; W.groundY=SIM.groundY;   // W.statics: SIM.statics minus ramps, filled by syncSimWorld
  const r=resolveShot(W,en.id,_eF.x,_eF.y,_eF.z,sdv.x,sdv.y,sdv.z,eu.enemyRange,teamFilter(en.team)); let end;
  if(r.kind==='enemy'){end=_eEnd.set(r.px,r.py,r.pz);if(r.id===SIM_PLAYER)damagePlayer(eu.enemyDmg,ep,false,en);addImpact(end,_aup,'flesh');}   // a bot hit by a bot takes no damage until applyDamage (step 8)
  else if(r.kind!=='none'){end=_eEnd.set(r.px,r.py,r.pz);addImpact(end,_eHitN.set(r.nx,r.ny,r.nz),'dirt');}
  else end=_eEnd.copy(eM).addScaledVector(sdv,T.enemyRange);
  addTracer(r.t<0.9?_eF:eM,end,0xff5a3a);   // a hit closer than the muzzle draws from the chest
}
function siegeFire(en,b){
  _eF.copy(en.group.position);_eF.y=1.45;_eT.copy(b.mesh.position);_eT.y=Math.min(3,b.mesh.geometry.parameters.height*0.5);
  const dir=_v.copy(_eT).sub(_eF).normalize();eM.copy(_eF).addScaledVector(dir,0.9);addMuzzle(eM,0xff6a4a); sfxEnemyShot(eM);
  addTracer(eM,_eT,0xff5a3a); addImpact(_eT,_zAxis,'metal'); damageBuilding(b,T.enemySiegeDamage);
}

/* grenades + melee */
const grenades=[];

/* ===== melee swing + shield ===== */
const _smooth=t=>t*t*(3-2*t);
// swing pose as a function of normalized progress p (0..1): windup -> strike -> recover, returns to idle.
function swingPose(p){
  const W={x:-0.15,y:1.35}, S={x:0.1,y:-1.35}, I=MELEE_IDLE; let x,y;
  if(p<0.22){ const t=_smooth(p/0.22); x=I.x+(W.x-I.x)*t; y=I.y+(W.y-I.y)*t; }
  else if(p<0.5){ const t=_smooth((p-0.22)/0.28); x=W.x+(S.x-W.x)*t; y=W.y+(S.y-W.y)*t; }
  else { const t=_smooth((p-0.5)/0.5); x=S.x+(I.x-S.x)*t; y=S.y+(I.y-S.y)*t; }
  return {x,y};
}
function startSwing(w){
  ps.swinging=true; ps.swingT=0; ps.swingDur=w.swingDur; ps.swingImpact=w.impact; ps.swingHit=false; ps.swingW=w;
  ps.swingMesh = w.id==='axe'?player.userData.axe:player.userData.sword;
  camShake+=w.recoil*0.06;
}
const _melTo=new THREE.Vector3();
function meleeHitCheck(w){
  const fwd=new THREE.Vector3(Math.sin(player.rotation.y),0,Math.cos(player.rotation.y));
  const cosArc=Math.cos(w.arcDeg*0.5*Math.PI/180); let hitAny=false;
  enemies.forEach(en=>{ if(!en.alive)return;
    _melTo.copy(en.group.position).sub(player.position); _melTo.y=0; const d=_melTo.length();
    if(d>w.range||d<0.001)return; _melTo.multiplyScalar(1/d);
    if(_melTo.dot(fwd)<cosArc)return;
    damageEnemy(en, w.damage, 'body', en.group.position.clone().setY(1.5), true);
    hitAny=true;
  });
  if(hitAny) camShake+=0.12;
}
// show the correct held model for the current weapon + shield state
function updateHeldModel(){
  const w=curW(), u=player.userData;
  const isSword=w.id==='sword'||w.id==='katana', isAxe=w.id==='axe';
  if(u.wpn) u.wpn.visible = !w.melee;
  if(!w.melee && !w.thrown && GUN_KIT[w.id]) setHeldGun(player, w.id);   // each gun shows its own model
  if(u.sword){ u.sword.visible=isSword; if(isSword&&!ps.swinging) u.sword.rotation.set(MELEE_IDLE.x,MELEE_IDLE.y,0); }
  if(u.axe){ u.axe.visible=isAxe; if(isAxe&&!ps.swinging) u.axe.rotation.set(MELEE_IDLE.x,MELEE_IDLE.y,0); }
  if(u.shield) u.shield.visible = ps.shieldOn;
  poseArms(player, w.melee?'melee':(ps.shieldOn?'shield':'rifle'));
}

/* ===== charged throw — hold to charge (bow = LMB, grenade = G); longer hold throws farther,
        with a live ballistic arc + landing ring as the feedback system ===== */
const T_CHARGE={ max:1.0,
  jav:{ pitch:0.05, sMin:32, sMax:64, grav:14, floor:0.05, col:0x38e1b0 },
  nade:{ pitch:0.72, sMin:13, sMax:26, grav:20, floor:0.26, col:0xffb060 },
  bow:{ pitch:0.04, sMin:30, sMax:94, grav:14, floor:0.05, col:0xff5a2a, capApex:0.7, time:1.35 } };   // flat trajectory: apex ~head height so bolts hit enemies along the aim line, not lob over them
const charge={ active:false, kind:null, t:0 };
const AIM_ARC_N=48; const ARC_DOTS_MAX=300;   // physics steps (coarse) vs render dots (dense, uniform arc-length spacing)
const _arcGeo=new THREE.BufferGeometry(); _arcGeo.setAttribute('position', new THREE.BufferAttribute(new Float32Array(ARC_DOTS_MAX*3),3));
const _arcDots=new THREE.Points(_arcGeo, new THREE.PointsMaterial({ color:0x38e1b0, size:0.3, sizeAttenuation:true, transparent:true, opacity:0.92, depthWrite:false })); _arcDots.frustumCulled=false; noRay(_arcDots); _arcDots.visible=false; scene.add(_arcDots);
const _arcRing=new THREE.Mesh(new THREE.RingGeometry(0.46,0.64,32), new THREE.MeshBasicMaterial({ color:0x38e1b0, transparent:true, opacity:0.85, side:THREE.DoubleSide, depthWrite:false })); _arcRing.rotation.x=-Math.PI/2; noRay(_arcRing); _arcRing.visible=false; scene.add(_arcRing);
const _arcHub=new THREE.Mesh(new THREE.CircleGeometry(0.13,18), new THREE.MeshBasicMaterial({ color:0x38e1b0, transparent:true, opacity:0.9, side:THREE.DoubleSide, depthWrite:false })); _arcHub.rotation.x=-Math.PI/2; noRay(_arcHub); _arcHub.visible=false; scene.add(_arcHub);
const _thrV=new THREE.Vector3(), _simP=new THREE.Vector3(), _simV=new THREE.Vector3(), _simPv=new THREE.Vector3();
function chargeFrac(){ const ct=(charge.kind&&T_CHARGE[charge.kind]&&T_CHARGE[charge.kind].time)||T_CHARGE.max; return Math.min(charge.t/ct,1); }
// fixed launch pitch, charge-scaled speed, aimed along the crosshair's ground heading
function throwVelocity(out, launch, kind){
  const cfg=T_CHARGE[kind], sp=cfg.sMin+(cfg.sMax-cfg.sMin)*chargeFrac();
  let hx=aimGround.x-launch.x, hz=aimGround.z-launch.z; const hl=hyp2(hx,hz);
  if(hl<1e-3){ hx=Math.sin(player.rotation.y); hz=Math.cos(player.rotation.y); } else { hx/=hl; hz/=hl; }
  const ch=Math.cos(cfg.pitch); let vy=sp*Math.sin(cfg.pitch);
  if(cfg.capApex!==undefined){ const mr=Math.max(0.05,(player.position.y+cfg.capApex)-launch.y); const vc=Math.sqrt(2*cfg.grav*mr); if(vy>vc)vy=vc; } // apex = launchY + vy^2/(2g) <= head-top
  out.set(hx*sp*ch, vy, hz*sp*ch); return out;
}
function startCharge(kind){
  if(charge.active||ps.dead||gameOver||paused||shopOpen||!started||ps.channeling) return;
  if(kind==='nade'){ if(grenadeCount<=0) return; }
  else if(kind==='bow'){ const w=curW(),l=curL(); if(!w.charge||ps.reloading||ps.fireCd>0||l.ammo<=0) return; }
  charge.active=true; charge.kind=kind; charge.t=0;
}
function endCharge(){ charge.active=false; charge.kind=null; charge.t=0; _arcDots.visible=false; _arcRing.visible=false; _arcHub.visible=false; }
function releaseCharge(){ if(!charge.active) return; const k=charge.kind; if(k==='nade') throwGrenadeCharged(); else if(k==='bow') fireBolt(); endCharge(); }
function tickCharge(dt){
  if(!charge.active) return;
  const ok = !ps.dead && !gameOver && (charge.kind==='nade' ? grenadeCount>0 : charge.kind==='bow' ? (curW().charge && !ps.reloading && curL().ammo>0) : (curW().thrown && !ps.reloading));
  if(!ok){ endCharge(); return; }
  charge.t=Math.min(charge.t+dt, (charge.kind&&T_CHARGE[charge.kind]&&T_CHARGE[charge.kind].time)||T_CHARGE.max); updateArcPreview();
}
// integrate the projectile under gravity until it meets the ground or a wall; draw dots + landing ring
function updateArcPreview(){
  const cfg=T_CHARGE[charge.kind]; getMuzzle(player,_simP); throwVelocity(_simV,_simP,charge.kind);
  const arr=_arcGeo.attributes.position.array, g=cfg.grav, h=0.045, SP=0.45, MAXD=ARC_DOTS_MAX;   // coarse physics step h; dense render dots every SP units of arc length
  let n=0, lx=_simP.x, ly=cfg.floor, lz=_simP.z, done=false, acc=0, nextD=SP;
  arr[0]=_simP.x; arr[1]=_simP.y; arr[2]=_simP.z; n=1;                                  // seed dot at the muzzle
  for(let s=0;s<AIM_ARC_N && n<MAXD;s++){
    _simPv.copy(_simP); _simV.y-=g*h; _simP.addScaledVector(_simV,h);
    let dx=_simP.x-_simPv.x, dy=_simP.y-_simPv.y, dz=_simP.z-_simPv.z; const sl=hyp3(dx,dy,dz);
    let hitT=-1;                                                                          // fraction [0..1] of this step where the arc stops (ground or wall)
    if(_simP.y<=cfg.floor) hitT=(_simPv.y-cfg.floor)/Math.max(1e-4,_simPv.y-_simP.y);
    if(sl>1e-5){ const idl=1/sl, S=SIM.statics; for(let k=0;k<S.length;k++){ const st=S[k]; if(st.alive===false) continue; const tt=rayStatic(_simPv.x,_simPv.y,_simPv.z, dx*idl,dy*idl,dz*idl,st,null); if(tt>=0&&tt<=sl){ const ft=tt/sl; if(hitT<0||ft<hitT)hitT=ft; } } }
    const segDist=(hitT>=0?hitT:1)*sl;                                                    // emit dense dots at global arc-length multiples of SP (uniform spacing, no gaps at step boundaries)
    while(nextD<=acc+segDist && n<MAXD){ const t=(nextD-acc)/Math.max(1e-6,sl); arr[n*3]=_simPv.x+dx*t; arr[n*3+1]=_simPv.y+dy*t; arr[n*3+2]=_simPv.z+dz*t; n++; nextD+=SP; }
    acc+=segDist;
    if(hitT>=0){ lx=_simPv.x+dx*hitT; ly=Math.max(cfg.floor,_simPv.y+dy*hitT); lz=_simPv.z+dz*hitT; if(n<MAXD){arr[n*3]=lx;arr[n*3+1]=ly;arr[n*3+2]=lz;n++;} done=true; break; }
  }
  if(!done){ lx=_simP.x; ly=Math.max(cfg.floor,_simP.y); lz=_simP.z; }
  _arcGeo.attributes.position.needsUpdate=true; _arcGeo.setDrawRange(0,n);
  const sc=1+chargeFrac()*0.35;
  _arcRing.position.set(lx,0.06,lz); _arcRing.scale.set(sc,sc,sc); _arcHub.position.set(lx,0.07,lz);
  const _bowArm=(charge.kind==='bow'&&chargeFrac()>=BOW_ARM);                          // >=BOW_ARM: arc telegraph turns orange (near-max power); the bolt always explodes anyway
  const _arcCol=(charge.kind==='bow')?(_bowArm?0xff7a1e:0x66f2ff):cfg.col;              // bow: BRIGHT cyan while charging, hot orange once armed (Gears-style high-contrast beam)
  _arcDots.material.color.setHex(_arcCol); _arcRing.material.color.setHex(_arcCol); _arcHub.material.color.setHex(_arcCol);
  _arcDots.material.size=(charge.kind==='bow')?0.5:0.3; _arcDots.material.opacity=(charge.kind==='bow')?1:0.92;
  _arcDots.visible=_arcRing.visible=_arcHub.visible=true;
}
function throwGrenadeCharged(){ animThrow(player);
  if(grenadeCount<=0) return; grenadeCount--; updateGrenadeHUD();
  getMuzzle(player,_p2); throwVelocity(_thrV,_p2,'nade');
  const m=new THREE.Mesh(new THREE.SphereGeometry(0.26,10,8), new THREE.MeshStandardMaterial({ color:0x39482f, roughness:0.6, emissive:0x223315, emissiveIntensity:0.3 }));
  m.position.copy(_p2); m.castShadow=true; noRay(m); stylizeOne(m,0.035); scene.add(m);
  grenades.push({ mesh:m, vel:_thrV.clone(), fuse:1.5 });
}
function explodeAt(p,bMult){
  grenadeBlast(p);
  if(COMIC)spawnComic('BOOM!',p.clone(),3.0,'#FF7B00');
  const from=p.clone(); from.y=Math.max(p.y,0.5);
  enemies.forEach(en=>{if(!en.alive)return;const d=en.group.position.distanceTo(p);if(d<T.grenadeRadius&&blastReaches(from,en.group.position.clone().setY(1.1),null))damageEnemy(en,T.grenadeDamage*(1-d/T.grenadeRadius),'body',en.group.position.clone().setY(1.4));});
  buildings.forEach(b=>{if(!b.alive||teamOf(b.team)===null||sameTeam(b.team,PLAYER_TEAM))return;const d=b.mesh.position.distanceTo(p);if(d<T.grenadeRadius+4&&blastReaches(from,b.mesh.position.clone().setY(Math.min(3,b.mesh.geometry.parameters.height*0.5)),b.mesh))damageBuilding(b,T.grenadeBuildingDamage*(bMult||1)*(1-d/(T.grenadeRadius+4)));});
  harvesters.forEach(hv=>{if(!hv.alive||teamOf(hv.team)===null||sameTeam(hv.team,PLAYER_TEAM))return;const d=hv.group.position.distanceTo(p);if(d<T.grenadeRadius+3&&blastReaches(from,hv.group.position.clone().setY(1.2),null))damageHarvester(hv,T.grenadeBuildingDamage*0.6*(1-d/(T.grenadeRadius+3)));});
  const dp=player.position.distanceTo(p); if(dp<T.grenadeRadius&&!ps.dead&&blastReaches(from,player.position.clone().setY(1.1),null))damagePlayer(T.grenadeDamage*0.5*(1-dp/T.grenadeRadius),p,true);
}

// ===== TORQUE BOW (Gears-style) — charge LMB to scale launch velocity; the kind-agnostic arc
// preview (T_CHARGE.bow) flattens automatically as speed rises. A (near-)full charge arms a
// sticky bolt with a short delayed blast; every bolt explodes now (charge only scales speed/range). =====
const bolts=[]; const _bSeg=new THREE.Vector3(), _bDir=new THREE.Vector3(), _bHit=new THREE.Vector3(); const BOLT_FUSE=0.55; const BOW_ARM=0.9;   // BOW_ARM: charge fraction where the arc telegraph turns orange (max power); bolts always explode.
function makeBolt(armed){
  const g=new THREE.Group();
  const shaft=new THREE.Mesh(new THREE.CylinderGeometry(0.05,0.05,1.5,6), new THREE.MeshStandardMaterial({color:armed?0x3a2418:0x4a4038,roughness:0.7,metalness:0.3}));
  const tip=new THREE.Mesh(new THREE.ConeGeometry(0.14,0.5,6), new THREE.MeshStandardMaterial({color:armed?0xff6a2a:0xcfd6d2,emissive:armed?0xff3a10:0x000000,emissiveIntensity:armed?0.8:0,roughness:0.4,metalness:0.55})); tip.position.y=0.95;
  const fins=new THREE.Mesh(new THREE.ConeGeometry(0.13,0.34,4), new THREE.MeshStandardMaterial({color:0x8f3636,roughness:0.7})); fins.position.y=-0.8; fins.rotation.x=Math.PI;
  g.add(shaft,tip,fins); g.traverse(o=>noRay(o)); stylizeOne(g,0.03); g.userData.tipMat=tip.material; return g;
}
function removeBolt(b,i){ scene.remove(b.mesh); b.mesh.traverse(o=>{ if(o.geometry)o.geometry.dispose(); if(o.material)o.material.dispose(); }); bolts.splice(i,1); }
function clearBolts(){ for(const b of bolts){ scene.remove(b.mesh); b.mesh.traverse(o=>{ if(o.geometry)o.geometry.dispose(); if(o.material)o.material.dispose(); }); } bolts.length=0; }
function fireBolt(){
  const w=curW(),l=curL(); if(l.ammo<=0) return;
  const armed = true;                 // bolts ALWAYS explode/stick now (Mustafa) -- charge only scales speed/range (dud path removed)
  playerFireOrigin(muzzleW); throwVelocity(_thrV,muzzleW,'bow'); _bDir.copy(_thrV).normalize();
  const m=makeBolt(armed); m.position.copy(muzzleW); m.quaternion.setFromUnitVectors(_aup,_bDir); scene.add(m);
  bolts.push({ mesh:m, vel:_thrV.clone(), prev:muzzleW.clone(), armed, state:'fly', t:0, life:0, stuckTo:null, stuckOff:new THREE.Vector3() });
  l.ammo--; ps.fireCd=w.fireInterval; ps.fireKick=Math.min(1.1,ps.fireKick+0.5+w.recoil*0.05);
  camRecoil+=w.recoil; camShake+=w.recoil*0.12; const _fk=fovBase/52; fovKick=Math.min(7*_fk,fovKick+(1.2+w.recoil*0.5)*_fk);
  addMuzzle(muzzleW,0xffb060); updateWeaponHUD(); if(l.ammo<=0){ if(l.reserve>0) startReload(); else equipPrimary(); }
}

/* economy */
function tryResupply(){
  if(ps.dead||gameOver||!started)return;
  if(player.position.distanceTo(terminalPos)>6)return;
  if(credits<T.resupplyCost*priceMult())return;
  credits-=T.resupplyCost*priceMult(); ps.health=ps.maxHealth;
  loadout.forEach((l,i)=>{l.ammo=WEAPONS[i].mag;l.reserve=WEAPONS[i].reserve;});
  ps.lastHitT=99;
  updateHealthHUD();updateWeaponHUD();updateGrenadeHUD();updateCreditHUD();
}

/* damage to player */
const vignette=document.getElementById('vignette'),dmgDirEl=document.getElementById('dmgDir'),wedgeEl=dmgDirEl.querySelector('.wedge'); let dmgDirT=0;
function damagePlayer(amount,fromPos,ignoreShield,srcEnemy){
  if(ps.dead||gameOver)return;
  if(pilotJug)return;   // piloting a vehicle: the mech takes the hits, not the soldier
  if(DEV.god)return;
  if(ps.shieldOn&&fromPos&&!ignoreShield){
    const fwd=new THREE.Vector3(Math.sin(player.rotation.y),0,Math.cos(player.rotation.y));
    const to=new THREE.Vector3(fromPos.x-player.position.x,0,fromPos.z-player.position.z);
    if(to.lengthSq()>1e-4){ to.normalize(); if(to.dot(fwd)>T.shieldArcCos){ amount*=(1-T.shieldBlock); ps.shieldFlash=1; camShake+=0.08; } }
  }
  ps.health-=amount; ps.lastHitT=0; camShake+=0.4; if(srcEnemy)ps.lastAttacker=srcEnemy; if(fromPos) animHit(player,fromPos.x,fromPos.z,amount/40);
  const inten=Math.min(1,amount/14+(1-ps.health/ps.maxHealth)*0.5);
  vignette.style.boxShadow=`inset 0 0 160px rgba(255,40,40,${0.22+inten*0.35})`;
  setTimeout(()=>{vignette.style.boxShadow=ps.health<30?'inset 0 0 160px rgba(255,40,40,0.16)':'inset 0 0 160px rgba(255,40,40,0)';},120);
  const toA=_v.copy(fromPos).sub(player.position);toA.y=0;const cf=new THREE.Vector3();camera.getWorldDirection(cf);cf.y=0;cf.normalize();
  const ang=Math.atan2(toA.x,toA.z)-Math.atan2(cf.x,cf.z); wedgeEl.style.transform=`rotate(${-ang}rad)`; dmgDirEl.style.opacity='1'; dmgDirT=0.8; if(toA.lengthSq()>1e-4){ _od.copy(toA).normalize(); camKickVel.addScaledVector(_od,-Math.min(7,amount*0.5)); camKickVel.y+=1.2; }
  updateHealthHUD(); if(ps.health<=0)playerDie();
}
function playerDie(){{ const _la=ps.lastAttacker&&ps.lastAttacker.group; deathStart(player,_la?_la.position.x:player.position.x+Math.random()-0.5,_la?_la.position.z:player.position.z+Math.random()-0.5); }ps.dead=true;ps.alive=false;if(SIM.players[0])SIM.players[0].alive=false;ps.deadT=T.respawnDelay;ps.toppleT=0;ps._tx=(Math.random()<0.5?-1:1);ps._tz=(Math.random()*2-1)*0.55;mouseDown=false;endCharge();onDeath();if(ps.lastAttacker){ps.lastAttacker.frags=(ps.lastAttacker.frags||0)+1;ps.lastAttacker=null;}document.getElementById('deathScreen').style.display='flex';}
function playerRespawn(){ps.dead=false;ps.alive=true;ps.lastAttacker=null;player.rotation.x=0;player.rotation.z=0;ps.toppleT=0;ps.fireKick=0;applyUnit(ps.unit);ps.health=ps.maxHealth;ps.lastHitT=99;player.position.set(mapSpawn.x+(Math.random()*2-1)*4,0,mapSpawn.z);ps.vel.set(0,0,0);ps.reloading=false;ps.fireCd=0;ps.swinging=false;loadout.forEach((l,i)=>{l.ammo=WEAPONS[i].mag;});document.getElementById('reloadBar').style.opacity='0';document.getElementById('deathScreen').style.display='none';vignette.style.boxShadow='inset 0 0 160px rgba(255,40,40,0)';updateHeldModel();updateHealthHUD();updateWeaponHUD();}

/* end / reset */
function checkEnd(){const eHQ=TEAM_INDEX.soviets.hqs[0],fHQ=TEAM_INDEX[PLAYER_TEAM].hqs[0];if(!gameOver&&eHQ&&!eHQ.alive)endGame(true);if(!gameOver&&fHQ&&!fHQ.alive)endGame(false);}
function endGame(win){gameOver=true;mouseDown=false;endEl.className=win?'win':'lose';endEl.style.display='flex';document.getElementById('endTitle').textContent=win?'VICTORY':'DEFEAT';document.getElementById('endSub').textContent=win?'enemy HQ destroyed':'your HQ has fallen';}
function resetRound(){
  buildTeamIndex(); activateRoster();   // the editor edits buildings without buildMap
  gameOver=false; endEl.style.display='none'; document.getElementById('deathScreen').style.display='none';
  buildings.forEach(b=>{b.alive=true;b.health=b.maxHealth;b.sink=0;b.mesh.position.y=b.mesh.geometry.parameters.height/2;b.mesh.visible=true;b.trim.visible=true;b.trim.material.emissiveIntensity=0.65;b.trim.position.y=b.mesh.geometry.parameters.height-0.7;b.label.visible=true;b.bar.grp.visible=true;b.box.setFromObject(b.mesh);});
  enemies.forEach(en=>{if(!en.active)return;en.alive=true;en.frags=0;en.downs=0;setEnemyType(en,en.elite?2:(Math.random()*3)|0);en.health=en.maxHealth;en.lungeCd=0;en.topple=0;en._tx=undefined;en.state='patrol';en.group.rotation.x=0;en.group.rotation.z=0;en.group.position.copy(respawnPos(en));en.group.position.y=0;en.group.visible=true;en.fireCd=1+Math.random();en.group.userData.bar.grp.visible=true;en.group.userData.wpn.rotation.x=0;});
  ps.health=T.playerHealth;ps.dead=false;ps.alive=true;ps.lastHitT=99;ps.lastAttacker=null;player.position.set(mapSpawn.x,0,mapSpawn.z);ps.vel.set(0,0,0); startAmbient();
  loadout.forEach((l,i)=>{l.ammo=WEAPONS[i].mag;l.reserve=WEAPONS[i].reserve;l.owned=(i===0);});
  for(let i=0;i<TEAMS.length;i++){ const e=teamEco[TEAMS[i].id]; e.era=1; e.tech=0; e.silos=0; }
  credits=T.startCredits;grenadeCount=0;kills=0;roundDeaths=0;roundAssists=0;ps.curWeapon=0;ps.spread=curW().spreadHip;ps.reloading=false;ps.fireCd=0;ps.ads=false;ps.shieldOwned=false;ps.shieldOn=false;ps.swinging=false;ps.shieldFlash=0;document.getElementById('reloadBar').style.opacity='0';unitOwned=UNIT_TYPES.map((u,i)=>i===0);ps.unit=0;applyUnit(0);ps.health=ps.maxHealth;
  clearBolts(); shopOpen=false; document.getElementById('shopModal').style.display='none'; updateHeldModel(); updateRank();
  vignette.style.boxShadow='inset 0 0 160px rgba(255,40,40,0)';
  camKick.set(0,0,0);camKickVel.set(0,0,0);concussionEl.style.opacity='0';camYaw=0;camYawTarget=0;rotating=false;smPushX=0;smPushY=0;
  resetHarvesters();resetTechBox();resetSilos();grantFreeWeapons(true);updateHealthHUD();updateWeaponHUD();updateGrenadeHUD();updateCreditHUD();updateHostileCount();updateBaseHUD();updateEraHUD();
}

/* HUD */
const ammoEl=document.getElementById('ammo'),ammoVal=document.getElementById('ammoVal'),reserveVal=document.getElementById('reserveVal'),weaponNameEl=document.getElementById('weaponName');
const hpBar=document.getElementById('healthBar'),hpBarI=hpBar.querySelector('i'),hpNum=document.getElementById('hpNum');
const slotsEl=document.getElementById('slots'),dmgNumbersEl=document.getElementById('dmgNumbers');
function showReadyFlash(){ammoEl.classList.add('ready');setTimeout(()=>ammoEl.classList.remove('ready'),420);}
function dryFire(){ammoEl.classList.remove('dry');void ammoEl.offsetWidth;ammoEl.classList.add('dry');setTimeout(()=>ammoEl.classList.remove('dry'),340);}
// gun-game rank display (replaces weapon slots)
const ROMAN=['I','II','III','IV','V','VI','VII','VIII'];
const rankLabel=document.createElement('div'); rankLabel.id='rankLabel';
const pipWrap=document.createElement('div'); pipWrap.id='pipWrap';
const pips=[]; function buildPips(n){pipWrap.innerHTML='';pips.length=0;for(let i=0;i<n;i++){const pp=document.createElement('div');pp.className='pip2';pipWrap.appendChild(pp);pips.push(pp);}} buildPips(RIFLE_TIERS[0].span);
const levelToastEl=document.getElementById('levelToast'); let levelToastTO=0;
function updateRank(){ const w=curW(); rankLabel.textContent=(w.tier||'').toUpperCase(); pipWrap.style.display='none'; }
function showLevelToast(t1,t2){ levelToastEl.innerHTML='<b>'+t1+'</b><span>'+t2+'</span>'; levelToastEl.classList.add('show'); clearTimeout(levelToastTO); levelToastTO=setTimeout(()=>levelToastEl.classList.remove('show'),2600); }
/* ===== Armory shop (buy / equip weapons) ===== */
const shopModalEl=document.getElementById('shopModal'), shopListEl=document.getElementById('shopList'), shopCreditsEl=document.getElementById('shopCredits');
function openShop(){ if(ps.dead||gameOver)return; shopOpen=true; endCharge(); mouseDown=false; semiQueued=false; ps.ads=false; rotating=false; renderer.domElement.style.cursor='default'; reticleEl.style.display='none'; rotDotEl.style.display='none'; hitmarkEl.style.display='none'; if(sightLine)sightLine.visible=false; if(heightLine)heightLine.visible=false; shopModalEl.style.display='flex'; renderShop(); }
function closeShop(){ shopOpen=false; shopModalEl.style.display='none'; renderer.domElement.style.cursor='none'; }
// ─── Phase 1: tier (era) + player-level gating ───
const TIER_MODE='cumulative';   // 'cumulative' = past-tier weapons stay buyable · 'current' = only the current era's tier
function weaponStatus(i){
  const w=WEAPONS[i];
  if(loadout[i].owned) return {state:'owned'};
  const eraOk = TIER_MODE==='cumulative' ? w.era<=teamEco[PLAYER_TEAM].era : w.era===teamEco[PLAYER_TEAM].era;
  if(!eraOk) return {state:'era'};
  if(Prog.level < w.minLevel) return {state:'level'};
  if(w.cost>0 && credits<w.cost*priceMult()) return {state:'poor'};
  return {state:'buy'};
}
function grantFreeWeapons(silent){   // auto-issue each tier's free standard (cost 0) once its era + level gates are met
  const names=[];
  WEAPONS.forEach((w,i)=>{
    if(w.cost===0 && !loadout[i].owned){
      const eraOk = TIER_MODE==='cumulative' ? w.era<=teamEco[PLAYER_TEAM].era : w.era===teamEco[PLAYER_TEAM].era;
      if(eraOk && Prog.level>=w.minLevel){ loadout[i].owned=true; loadout[i].ammo=w.mag; loadout[i].reserve=w.reserve; names.push(w.name); }
    }
  });
  if(names.length && !silent) showLevelToast('Tier Weapon', names.join(', ')+' issued');
  return names.join(', ');
}
function buyWeapon(i){ const w=WEAPONS[i]; if(weaponStatus(i).state!=='buy')return; credits-=w.cost*priceMult(); loadout[i].owned=true; loadout[i].ammo=w.mag; loadout[i].reserve=w.reserve; updateCreditHUD(); showLevelToast('Purchased',w.name); switchWeapon(i); renderShop(); }
function renderShop(){
  shopCreditsEl.textContent=Math.floor(credits); shopListEl.innerHTML='';
  if(powerDead(PLAYER_TEAM)){ const warn=document.createElement('div'); warn.className='shopWarn'; warn.textContent='⚠ Power plant offline — prices doubled'; shopListEl.appendChild(warn); }
  { const note=document.createElement('div'); note.className='shopWarn'; note.style.opacity='0.85'; note.textContent='Pick your class below. Keys: 1 class weapon, 3 special, F ability, G frag'; shopListEl.appendChild(note); }
  // --- SQUAD (spawn Conscript; others free to equip; no instant 1-4 switch) ---
  { const h=document.createElement('div'); h.className='shopTierHead'; h.textContent='Squad'; shopListEl.appendChild(h);
    [0,1,2,3].forEach(ui=>{ const u=UNIT_TYPES[ui]; if(!u)return; const st=unitStatus(ui); const cost=Math.round((u.cost||0)*priceMult());
      const row=document.createElement('div'); row.className='shopRow';
      const info=document.createElement('div');
      const tt=unitOwned[ui]?('T'+(u.tier||1)):((u.cost||0)>0?('T'+(u.tier||1)+' · '+cost+' CR'+(priceMult()>1?' ⚠':'')):('T'+(u.tier||1)+' · FREE')); info.innerHTML='<div class="shopName">'+u.name+'</div><div class="shopTier">'+tt+' · '+u.maxHealth+' HP</div>';
      const btn=document.createElement('div'); btn.className='shopBtn';
      if(st.state==='active'){ btn.textContent='Active'; btn.classList.add('cant'); }
      else if(st.state==='buy'){ btn.textContent='Buy'; btn.onclick=()=>pickClass(ui); }
      else if(st.state==='equip'){ btn.textContent='Equip'; btn.onclick=()=>pickClass(ui); }
      else if(st.state==='poor'){ btn.textContent='Buy'; btn.classList.add('cant'); }
      else { btn.textContent='Tier '+(u.tier||1); btn.classList.add('cant'); }
      row.appendChild(info); row.appendChild(btn); shopListEl.appendChild(row);
    });
  }
  // --- VEHICLES (enterable; bought separately from infantry) ---
  if(typeof VEHICLE_TYPES!=='undefined' && VEHICLE_TYPES.length){ const h=document.createElement('div'); h.className='shopTierHead'; h.textContent='Vehicles'; shopListEl.appendChild(h);
    VEHICLE_TYPES.forEach(v=>{ const st=vehicleStatus(v.id); const cost=Math.round((v.cost||0)*priceMult());
      const row=document.createElement('div'); row.className='shopRow';
      const info=document.createElement('div');
      info.innerHTML='<div class="shopName">'+v.name+'</div><div class="shopTier">VEHICLE \u00b7 T'+(v.tier||1)+' \u00b7 '+v.maxHealth+' HP \u00b7 '+(st.state==='era'?('TIER '+(v.tier||1)):(cost+' CR'+(priceMult()>1?' \u26a0':'')))+'</div>';
      const btn=document.createElement('div'); btn.className='shopBtn';
      if(st.state==='buy'){ btn.textContent='Buy'; btn.onclick=()=>buyVehicle(v.id); }
      else if(st.state==='poor'){ btn.textContent='Buy'; btn.classList.add('cant'); }
      else { btn.textContent='Tier '+(v.tier||1); btn.classList.add('cant'); }
      row.appendChild(info); row.appendChild(btn); shopListEl.appendChild(row);
    });
  }
  // --- SPECIAL WEAPONS (bought, finite ammo, key 3 to equip) ---
  { const h=document.createElement('div'); h.className='shopTierHead'; h.textContent='Special Weapons'; shopListEl.appendChild(h);
    WEAPONS.forEach((w,i)=>{ if(!w.special)return; const st=weaponStatus(i); const cost=Math.round(w.cost*priceMult());
      const row=document.createElement('div'); row.className='shopRow';
      const info=document.createElement('div');
      info.innerHTML='<div class="shopName">'+w.name+'</div><div class="shopTier">SPECIAL · '+w.mag+'/'+w.reserve+' AMMO · '+(st.state==='owned'?'OWNED — PRESS 3':(cost+' CR'+(priceMult()>1?' ⚠':'')))+'</div>';
      const btn=document.createElement('div'); btn.className='shopBtn';
      if(st.state==='owned'){ btn.textContent='Equip'; btn.onclick=()=>{ equipSpecial(); closeShop(); }; }
      else if(st.state==='buy'){ btn.textContent='Buy'; btn.onclick=()=>{ buyWeapon(i); renderShop(); }; }
      else { btn.textContent=(st.state==='poor'?'Buy':'Locked'); btn.classList.add('cant'); }
      row.appendChild(info); row.appendChild(btn); shopListEl.appendChild(row);
    });
  }

  // frag grenade (buyable consumable — thrown with G; you do NOT spawn with any)
  { const row=document.createElement('div'); row.className='shopRow';
    const info=document.createElement('div'); const cost=50*priceMult();
    info.innerHTML='<div class="shopName">Frag Grenade</div><div class="shopTier">THROWABLE (G) · '+cost+' cr'+(priceMult()>1?' ⚠':'')+' · '+grenadeCount+'/'+T.grenadeMax+' held</div>';
    const btn=document.createElement('div'); btn.className='shopBtn';
    if(grenadeCount>=T.grenadeMax){ btn.textContent='Full'; btn.classList.add('cant'); }
    else if(credits<cost){ btn.textContent='Buy'; btn.classList.add('cant'); }
    else { btn.textContent='Buy'; btn.onclick=()=>{ buyGrenade(); renderShop(); }; }
    row.appendChild(info); row.appendChild(btn); shopListEl.appendChild(row);
  }
}
const Prog={xp:0,level:1,prestige:0,kills:0,deaths:0,headshots:0,weapon:{}};
const ProgStore={ persist:KVStore.persist,
  async load(){ if(this.persist){ for(const k of ['shamble_prog','vanguard_prog']){ try{ const r=await KVStore.get(k); if(r&&r.value) return JSON.parse(r.value); }catch(e){} } } return null; },
  async save(){ if(this.persist){try{await KVStore.set('shamble_prog',JSON.stringify(Prog));}catch(e){}} } };
let _progSaveTO=0; function saveProg(){ clearTimeout(_progSaveTO); _progSaveTO=setTimeout(()=>ProgStore.save(),400); }
// XP to advance FROM level n -> n+1. lvl1->2 = 500 (~10 kills); ramps quadratically (grindy by 100).
function xpForLevel(n){ return Math.round(500 + (n-1)*110 + (n-1)*(n-1)*5); }

const RANK_TIERS=[
  {name:'Recruit',upTo:3,metal:'bronze',motif:'plain'},
  {name:'Private',upTo:8,metal:'bronze',motif:'chevron',n:1},
  {name:'Private First Class',upTo:14,metal:'bronze',motif:'chevron',n:1,rockers:1},
  {name:'Corporal',upTo:21,metal:'silver',motif:'chevron',n:2},
  {name:'Sergeant',upTo:29,metal:'silver',motif:'chevron',n:3},
  {name:'Staff Sergeant',upTo:37,metal:'silver',motif:'chevron',n:3,rockers:1},
  {name:'Technical Sergeant',upTo:45,metal:'silver',motif:'chevron',n:3,rockers:2},
  {name:'Master Sergeant',upTo:53,metal:'gold',motif:'chevron',n:3,rockers:3},
  {name:'First Sergeant',upTo:60,metal:'gold',motif:'chevron',n:3,rockers:3,star:1},
  {name:'Sergeant Major',upTo:67,metal:'gold',motif:'chevron',n:3,rockers:3,star:2},
  {name:'Second Lieutenant',upTo:73,metal:'gold',motif:'bar',n:1},
  {name:'First Lieutenant',upTo:79,metal:'silver',motif:'bar',n:1},
  {name:'Captain',upTo:85,metal:'silver',motif:'bar',n:2},
  {name:'Major',upTo:90,metal:'gold',motif:'oak',n:1},
  {name:'Lieutenant Colonel',upTo:94,metal:'silver',motif:'oak',n:1},
  {name:'Colonel',upTo:97,metal:'gold',motif:'eagle'},
  {name:'Brigadier General',upTo:99,metal:'gold',motif:'stars',n:1},
  {name:'General of the Army',upTo:100,metal:'gold',motif:'stars',n:5},
];
function rankForLevel(level){ for(const t of RANK_TIERS) if(level<=t.upTo) return t; return RANK_TIERS[RANK_TIERS.length-1]; }

/* ---- shared SVG helpers ---- */
let _svgU=0;
function metalCols(m){ return m==='gold'?['#ffe9a8','#e3ad33','#7a5410','#fff7d6']:m==='silver'?['#f7fafc','#c4ccd4','#6f7a85','#ffffff']:m==='platinum'?['#f1f9ff','#d2deec','#828ea0','#ffffff']:['#eaad6d','#b87333','#5b3413','#f4cb93']; }
function _star(cx,cy,rO,rI,pts){ pts=pts||5; let p='',n=pts*2; for(let i=0;i<n;i++){ const r=i%2?rI:rO,a=-Math.PI/2+i*Math.PI/pts; p+=(i?'L':'M')+(cx+r*Math.cos(a)).toFixed(2)+' '+(cy+r*Math.sin(a)).toFixed(2); } return p+'Z'; }
function _chev(cx,y,hw,ri,th){ return 'M'+(cx-hw)+' '+y+'L'+cx+' '+(y-ri)+'L'+(cx+hw)+' '+y+'L'+(cx+hw)+' '+(y+th)+'L'+cx+' '+(y-ri+th)+'L'+(cx-hw)+' '+(y+th)+'Z'; }
function _rocker(cx,y,rad,th){ const x0=cx-rad,x1=cx+rad,ry=rad*0.5; return 'M'+x0+' '+y+'A'+rad+' '+ry+' 0 0 0 '+x1+' '+y+'L'+x1+' '+(y+th)+'A'+rad+' '+ry+' 0 0 1 '+x0+' '+(y+th)+'Z'; }
function _wreathSVG(c){ let s='<g fill="'+c[1]+'" stroke="'+c[2]+'" stroke-width=".4">'; const cx=50,cy=53,R=22; for(let a=205;a<=515;a+=15){ if(a>338&&a<382) continue; const rad=a*Math.PI/180,x=cx+Math.cos(rad)*R,y=cy+Math.sin(rad)*R,rot=a+90; s+='<ellipse cx="'+x.toFixed(1)+'" cy="'+y.toFixed(1)+'" rx="1.8" ry="4.2" transform="rotate('+rot.toFixed(0)+' '+x.toFixed(1)+' '+y.toFixed(1)+')"/>'; } s+='<circle cx="'+cx+'" cy="'+cy+'" r="'+R+'" fill="none" stroke="'+c[2]+'" stroke-width="1" opacity=".4"/></g>'; return s; }

/* ---- rank insignia: WW2 rank slides. Oxblood cloth + gold embroidery (NCO) -> gunmetal + polished silver/gold pins (officers) -> brass-edged gunmetal + gold eagle & stars (generals) ---- */
function rankInsigniaSVG(level,size){
  const t=rankForLevel(level), motif=t.motif, u='r'+(_svgU++);
  const style=(motif==='chevron'||motif==='plain')?'cloth':(motif==='bar'||motif==='oak')?'metal':(motif==='eagle')?'eagle':'star';
  const GOLD=['#ffe9a8','#e8b73e','#7a5212'], SILV=['#f2f6fa','#bcc6d0','#6c747e'], CHEVG=['#ffe39a','#dcb247','#9c7a1e'];
  const EAGLE='<path d="M50 38 L50 60 L45 60 L45 42 Z"/><path d="M50 42 C39 35 26 33 13 40 C24 42 29 47 29 52 C36 47 44 47 50 52 Z"/><path d="M50 42 C61 35 74 33 87 40 C76 42 71 47 71 52 C64 47 56 47 50 52 Z"/><circle cx="50" cy="36" r="4.6"/><path d="M50 60 L45 70 L55 70 Z"/>';
  const isCloth=(style==='cloth');
  // vertical ribbon field + metal edge by band; width adapts to motif, height constant
  const fld=isCloth?['#5a2526','#3f1819','#280e0e']:['#373a42','#272a32','#15171c'];
  const edgeC=(style==='star')?'#c9a23a':isCloth?'#7c5e2b':'#99a0ac';
  let RW=42; if(motif==='bar') RW=20+(t.n-1)*17; else if(motif==='oak') RW=30; else if(style==='eagle') RW=50; else if(style==='star') RW=48;
  const RH=86, RX=50-RW/2, RY=7, rx=6;
  let defs='<filter id="'+u+'sh" x="-45%" y="-45%" width="190%" height="190%"><feDropShadow dx="0" dy="2" stdDeviation="1.9" flood-color="#160f07" flood-opacity=".55"/></filter>'
    +'<linearGradient id="'+u+'bk" x1="0" y1="0" x2=".32" y2="1"><stop offset="0" stop-color="'+fld[0]+'"/><stop offset=".5" stop-color="'+fld[1]+'"/><stop offset="1" stop-color="'+fld[2]+'"/></linearGradient>';
  const back='<g filter="url(#'+u+'sh)">'
    +'<rect x="'+(RX-1.7).toFixed(1)+'" y="'+(RY-1.7).toFixed(1)+'" width="'+(RW+3.4).toFixed(1)+'" height="'+(RH+3.4).toFixed(1)+'" rx="'+(rx+1.5)+'" fill="'+edgeC+'"/>'
    +'<rect x="'+RX.toFixed(1)+'" y="'+RY+'" width="'+RW.toFixed(1)+'" height="'+RH+'" rx="'+rx+'" fill="url(#'+u+'bk)"/>'
    +'<rect x="'+(RX+2).toFixed(1)+'" y="'+(RY+2)+'" width="'+(RW-4).toFixed(1)+'" height="'+(RH-4)+'" rx="'+(rx-1.4)+'" fill="none" stroke="#fff" stroke-opacity=".08" stroke-width="1"/>'
    +'</g>';
  let body='';
  if(isCloth){
    defs+='<linearGradient id="'+u+'c" x1="0" y1="0" x2=".2" y2="1"><stop offset="0" stop-color="'+CHEVG[0]+'"/><stop offset=".5" stop-color="'+CHEVG[1]+'"/><stop offset="1" stop-color="'+CHEVG[2]+'"/></linearGradient>';
    const cf='fill="url(#'+u+'c)" stroke="#6f5414" stroke-width=".55" stroke-linejoin="round"';
    if(motif==='chevron'){
      const hw=15.5,ri=9,th=5.2;
      if(t.rockers||t.star){
        const sp=8; for(let i=0;i<t.n;i++) body+='<path d="'+_chev(50,42-i*sp,hw,ri,th)+'" '+cf+'/>';
        if(t.star) for(let k=0;k<t.star;k++){ const sx=50+(k-(t.star-1)/2)*9; body+='<path d="'+_star(sx,53,4.6,2,5)+'" '+cf+'/>'; }
        for(let j=0;j<(t.rockers||0);j++) body+='<path d="'+_rocker(50,61+j*6,15.5-j*1.3,4.6)+'" '+cf+'/>';
      } else {
        const sp=8.4,y0=46+(t.n-1)*sp/2; for(let i=0;i<t.n;i++) body+='<path d="'+_chev(50,y0-i*sp,hw,ri,th)+'" '+cf+'/>';
      }
    } else { body+='<circle cx="50" cy="50" r="6" fill="none" stroke="url(#'+u+'c)" stroke-width="1.8" stroke-dasharray="2.6 2.2"/><circle cx="50" cy="50" r="1.8" fill="url(#'+u+'c)"/>'; }
  } else if(style==='metal'){
    const c=(t.metal==='gold')?GOLD:SILV;
    defs+='<linearGradient id="'+u+'m" x1="0" y1="0" x2=".25" y2="1"><stop offset="0" stop-color="'+c[0]+'"/><stop offset=".45" stop-color="'+c[1]+'"/><stop offset="1" stop-color="'+c[2]+'"/></linearGradient>';
    const fillM='fill="url(#'+u+'m)" stroke="'+c[2]+'" stroke-width=".8" stroke-linejoin="round"';
    if(motif==='bar'){
      const bw=10,bh=48,gap=7,tot=t.n*bw+(t.n-1)*gap,x0=50-tot/2;
      for(let i=0;i<t.n;i++){ const x=x0+i*(bw+gap); body+='<rect x="'+x+'" y="'+(50-bh/2)+'" width="'+bw+'" height="'+bh+'" rx="2.3" '+fillM+'/><rect x="'+(x+1.8)+'" y="'+(50-bh/2+3)+'" width="2" height="'+(bh-6)+'" rx="1" fill="#fff" opacity=".42"/>'; }
    } else {
      body+='<g transform="translate(50 50) scale(1.04) translate(-50 -50)"><path d="M50 76 C46 67 43 60 46 54 C40 56 35 51 38 46 C43 48 45 44 43 40 C38 42 33 37 36 32 C42 34 45 30 44 24 C48 28 50 25 50 20 C50 25 52 28 56 24 C55 30 58 34 64 32 C67 37 62 42 57 40 C55 44 57 48 62 46 C65 51 60 56 54 54 C57 60 54 67 50 76 Z" '+fillM+'/><path d="M50 71 L50 26" fill="none" stroke="'+c[2]+'" stroke-width="1.1" opacity=".5"/></g>';
    }
  } else if(style==='eagle'){
    const c=SILV;
    defs+='<linearGradient id="'+u+'e" x1="0" y1="0" x2=".2" y2="1"><stop offset="0" stop-color="'+c[0]+'"/><stop offset=".45" stop-color="'+c[1]+'"/><stop offset="1" stop-color="'+c[2]+'"/></linearGradient>';
    body+='<g transform="translate(50 50) scale(.62) translate(-50 -50)" fill="url(#'+u+'e)" stroke="'+c[2]+'" stroke-width=".7" stroke-linejoin="round">'+EAGLE+'</g>';
  } else {
    const c=GOLD;
    defs+='<linearGradient id="'+u+'g" x1="0" y1="0" x2=".2" y2="1"><stop offset="0" stop-color="'+c[0]+'"/><stop offset=".42" stop-color="'+c[1]+'"/><stop offset="1" stop-color="'+c[2]+'"/></linearGradient>';
    const fillG='fill="url(#'+u+'g)" stroke="'+c[2]+'" stroke-width=".7" stroke-linejoin="round"';
    const n=t.n||1;
    if(n>=4) body+=_wreathSVG(c);
    body+='<g transform="translate(50 35) scale(.5) translate(-50 -50)" '+fillG+'>'+EAGLE+'</g>';
    const ss=(n>=4)?4.8:6, dxs=(n>=4)?8.6:12;
    for(let i=0;i<n;i++){ const dx=(i-(n-1)/2)*dxs; body+='<path d="'+_star(50+dx,72,ss,ss*0.44,5)+'" '+fillG+'/>'; }
  }
  return '<svg viewBox="0 0 100 100" width="'+size+'" height="'+size+'" xmlns="http://www.w3.org/2000/svg"><defs>'+defs+'</defs>'+back+'<g>'+body+'</g></svg>';
}

/* ---- prestige medals (ornate, shiny) ---- */
const PRESTIGE=[
  {points:5,metal:'silver',gem:'#cfd6dd',center:'star',label:'First Honor'},
  {points:8,metal:'silver',gem:'#ff5a5a',center:'gem',label:'Iron Star'},
  {points:6,metal:'gold',gem:'#ffd24d',center:'star',label:'Gilded Cross'},
  {points:8,metal:'gold',gem:'#ff4d4d',center:'gem',label:'Crimson Sunburst'},
  {points:12,metal:'silver',gem:'#9fd3ff',center:'orb',label:'Azure Order'},
  {points:6,metal:'gold',gem:'#5fe06a',center:'gem',label:'Emerald Cross'},
  {points:10,metal:'gold',gem:'#ffe08a',center:'eagle',label:'Eagle Standard'},
  {points:8,metal:'platinum',gem:'#eaf4ff',center:'gear',label:'Platinum Aegis'},
  {points:12,metal:'gold',gem:'#ff5a5a',center:'star',wreath:true,label:'Marshal Wreath'},
  {points:16,metal:'platinum',gem:'#ffd24d',center:'star',wreath:true,label:'Supreme Command'},
];
function prestigeCfg(p){ return PRESTIGE[Math.max(0,Math.min(PRESTIGE.length-1,p-1))]; }
function prestigeMedalSVG(p,size){
  const cfg=prestigeCfg(p), c=metalCols(cfg.metal), u='p'+(_svgU++), N=cfg.points, rO=46, rI=19;
  let spikes='';
  for(let i=0;i<N;i++){ const a=-Math.PI/2+i*2*Math.PI/N, wa=0.16;
    const x1=(50+rO*Math.cos(a)).toFixed(1),y1=(50+rO*Math.sin(a)).toFixed(1),x2=(50+rI*Math.cos(a-wa)).toFixed(1),y2=(50+rI*Math.sin(a-wa)).toFixed(1),x3=(50+rI*Math.cos(a+wa)).toFixed(1),y3=(50+rI*Math.sin(a+wa)).toFixed(1);
    spikes+='<path d="M'+x1+' '+y1+'L'+x2+' '+y2+'L'+x3+' '+y3+'Z" fill="url(#'+u+'sp)" stroke="'+c[2]+'" stroke-width=".4"/>'; }
  for(let i=0;i<N;i++){ const a=-Math.PI/2+(i+0.5)*2*Math.PI/N, rs=rO*0.6, wa=0.12;
    const x1=(50+rs*Math.cos(a)).toFixed(1),y1=(50+rs*Math.sin(a)).toFixed(1),x2=(50+rI*Math.cos(a-wa)).toFixed(1),y2=(50+rI*Math.sin(a-wa)).toFixed(1),x3=(50+rI*Math.cos(a+wa)).toFixed(1),y3=(50+rI*Math.sin(a+wa)).toFixed(1);
    spikes+='<path d="M'+x1+' '+y1+'L'+x2+' '+y2+'L'+x3+' '+y3+'Z" fill="url(#'+u+'sp2)" stroke="'+c[2]+'" stroke-width=".3"/>'; }
  let wreath='';
  if(cfg.wreath){ let lp=''; const steps=8; for(let s=0;s<steps;s++){ const tt=s/(steps-1), ang=Math.PI*0.58+tt*Math.PI*0.84, lr=rO*0.82, lx=(50+lr*Math.cos(ang)).toFixed(1), ly=(50+lr*Math.sin(ang)).toFixed(1), rot=(ang*180/Math.PI+90).toFixed(0); lp+='<ellipse cx="'+lx+'" cy="'+ly+'" rx="3.6" ry="1.9" transform="rotate('+rot+' '+lx+' '+ly+')" fill="url(#'+u+'ring)" stroke="'+c[2]+'" stroke-width=".3"/>'; } wreath='<g>'+lp+'</g><g transform="translate(100,0) scale(-1,1)">'+lp+'</g>'; }
  let emblem='';
  if(cfg.center==='star') emblem='<path d="'+_star(50,50,12,5,5)+'" fill="url(#'+u+'gem)" stroke="'+c[0]+'" stroke-width=".6"/><circle cx="46.5" cy="46" r="1.8" fill="#fff" opacity=".7"/>';
  else if(cfg.center==='gem') emblem='<circle cx="50" cy="50" r="10.5" fill="url(#'+u+'gem)"/><ellipse cx="46.4" cy="46" rx="3" ry="2" fill="#fff" opacity=".8"/>';
  else if(cfg.center==='orb') emblem='<circle cx="50" cy="50" r="11" fill="url(#'+u+'gem)"/><ellipse cx="46" cy="45.5" rx="3.6" ry="2.3" fill="#fff" opacity=".75"/>';
  else if(cfg.center==='eagle') emblem='<g fill="url(#'+u+'ring)" stroke="'+c[2]+'" stroke-width=".4"><path d="M50 44 L50 56 L47 56 L47 47 Z"/><path d="M50 45 C43 41 35 40 28 44 C34 45 36 48 36 51 C40 48 46 48 50 51 Z"/><path d="M50 45 C57 41 65 40 72 44 C66 45 64 48 64 51 C60 48 54 48 50 51 Z"/><circle cx="50" cy="41" r="3"/></g>';
  else if(cfg.center==='gear'){ let cog=''; for(let i=0;i<10;i++) cog+='<rect x="48.6" y="37" width="2.8" height="4.6" transform="rotate('+(i*36)+' 50 50)" fill="url(#'+u+'ring)"/>'; emblem=cog+'<circle cx="50" cy="50" r="8.5" fill="url(#'+u+'gem)"/><ellipse cx="47" cy="47" rx="2.4" ry="1.6" fill="#fff" opacity=".7"/>'; }
  const center='<circle cx="50" cy="50" r="19" fill="url(#'+u+'ring)" stroke="'+c[2]+'" stroke-width="1"/><circle cx="50" cy="50" r="15.5" fill="#14110d"/>'+emblem;
  const defs='<radialGradient id="'+u+'sp" cx=".5" cy=".42" r=".7"><stop offset="0" stop-color="'+c[0]+'"/><stop offset=".55" stop-color="'+c[1]+'"/><stop offset="1" stop-color="'+c[2]+'"/></radialGradient>'
    +'<radialGradient id="'+u+'sp2" cx=".5" cy=".5" r=".7"><stop offset="0" stop-color="'+c[1]+'"/><stop offset="1" stop-color="'+c[2]+'"/></radialGradient>'
    +'<radialGradient id="'+u+'ring" cx=".4" cy=".34" r=".8"><stop offset="0" stop-color="'+c[0]+'"/><stop offset=".5" stop-color="'+c[1]+'"/><stop offset="1" stop-color="'+c[2]+'"/></radialGradient>'
    +'<radialGradient id="'+u+'gem" cx=".38" cy=".3" r=".75"><stop offset="0" stop-color="#ffffff"/><stop offset=".2" stop-color="'+cfg.gem+'"/><stop offset="1" stop-color="#1a0e0e"/></radialGradient>'
    +'<filter id="'+u+'gl" x="-30%" y="-30%" width="160%" height="160%"><feGaussianBlur stdDeviation="1.8" result="b"/><feMerge><feMergeNode in="b"/><feMergeNode in="SourceGraphic"/></feMerge></filter>';
  const shine='<path d="M26 30 A30 30 0 0 1 64 22" fill="none" stroke="#fff" stroke-width="2.5" stroke-linecap="round" opacity=".3"/>';
  return '<svg viewBox="0 0 100 100" width="'+size+'" height="'+size+'" xmlns="http://www.w3.org/2000/svg"><defs>'+defs+'</defs><g filter="url(#'+u+'gl)">'+wreath+spikes+center+'</g>'+shine+'</svg>';
}

/* ---- stats helpers ---- */
function weaponName(id){ const w=WEAPONS.find(x=>x.id===id); return w?w.name:(id||'—'); }
function mostUsedWeapon(){ let best=null,bn=0; for(const k in Prog.weapon){ if(Prog.weapon[k]>bn){bn=Prog.weapon[k];best=k;} } return best?weaponName(best):'—'; }

/* ---- kill chain + XP ---- */
let chainTimer=0, chainCount=0;
function onKill(weaponId, headshot){
  Prog.kills++; if(headshot)Prog.headshots++;
  if(weaponId) Prog.weapon[weaponId]=(Prog.weapon[weaponId]||0)+1;
  let xp; if(chainTimer>0){ chainCount++; xp=150; } else { chainCount=1; xp=50; }
  chainTimer=5; // each kill (re)opens/extends the 5s bonus window
  if(chainCount>=2) showMultiKill(chainCount, xp);
  addXP(xp); saveProg();
}
function onDeath(){ Prog.deaths++; roundDeaths++; saveProg(); }
function addXP(amount){
  if(Prog.level>=100){ showXpEarn(amount); updateCareerChip(); return; } // max level: prestige to keep ranking up
  Prog.xp+=amount; let leveled=false;
  while(Prog.level<100 && Prog.xp>=xpForLevel(Prog.level)){ Prog.xp-=xpForLevel(Prog.level); Prog.level++; leveled=true; }
  if(Prog.level>=100) Prog.xp=0;
  showXpEarn(amount);
  if(leveled) showRankUp(Prog.level, rankForLevel(Prog.level));
  if(leveled){ grantFreeWeapons(true); if(shopOpen) renderShop(); }
  updateCareerChip();
}
function doPrestige(){
  if(Prog.level<100) return;
  Prog.prestige++; Prog.level=1; Prog.xp=0; saveProg();
  updateCareerChip(); renderCareer();
  document.querySelector('#rankBanner .rb-t').textContent='Prestige';
  showBanner(prestigeMedalSVG(Prog.prestige,84), 'Prestige '+Prog.prestige, prestigeCfg(Prog.prestige).label);
}

/* ---- career UI ---- */
const ccIconEl=document.getElementById('ccIcon'), ccLvlEl=document.getElementById('ccLvl'), ccPrestEl=document.getElementById('ccPrest'), ccXpEl=document.getElementById('ccXp'), ccPopEl=document.getElementById('ccPop');
const careerModalEl=document.getElementById('careerModal'), mkBannerEl=document.getElementById('mkBanner'), rankBannerEl=document.getElementById('rankBanner'), rbIconEl=document.getElementById('rbIcon');
function updateCareerChip(){
  ccIconEl.innerHTML=rankInsigniaSVG(Prog.level,38);
  ccLvlEl.textContent='LVL '+Prog.level;
  if(Prog.prestige>0){ ccPrestEl.style.display=''; ccPrestEl.textContent='✦'+Prog.prestige; } else ccPrestEl.style.display='none';
  const need=xpForLevel(Prog.level), frac=Prog.level>=100?1:Math.max(0,Math.min(1,Prog.xp/need));
  ccXpEl.style.width=(frac*100)+'%';
}
let _xpPopTO=0;
let _mkTO=0;
function showMultiKill(count, xp){
  const names=['','','DOUBLE KILL','TRIPLE KILL','QUAD KILL','MEGA KILL']; const t=names[Math.min(count,5)]||'UNSTOPPABLE';
  document.getElementById('mkText').textContent=t; document.getElementById('mkSub').textContent='+'+xp+' XP';
  mkBannerEl.classList.remove('show'); void mkBannerEl.offsetWidth; mkBannerEl.classList.add('show'); clearTimeout(_mkTO); _mkTO=setTimeout(()=>mkBannerEl.classList.remove('show'),1100);
}
let _rbTO=0;
function showBanner(iconSVG, title, line){ rbIconEl.innerHTML=iconSVG; document.getElementById('rbRank').textContent=title; document.getElementById('rbLvl').textContent=line; rankBannerEl.classList.add('show'); clearTimeout(_rbTO); _rbTO=setTimeout(()=>rankBannerEl.classList.remove('show'),2400); }
function showRankUp(level, rank){ document.querySelector('#rankBanner .rb-t').textContent='Rank Up'; showBanner(rankInsigniaSVG(level,84), rank.name, 'Level '+level); }
function statRow(k,v){ return '<div class="cpStat"><span>'+k+'</span><b>'+v+'</b></div>'; }
function renderCareer(){
  const r=rankForLevel(Prog.level), need=xpForLevel(Prog.level);
  document.getElementById('cpIcon').innerHTML=rankInsigniaSVG(Prog.level,150);
  document.getElementById('cpRank').textContent=r.name;
  document.getElementById('cpLvl').textContent='Level '+Prog.level+(Prog.prestige>0?' · Prestige '+Prog.prestige:'')+(Prog.level>=100?' · MAX':'');
  document.getElementById('cpXpFill').style.width=(Prog.level>=100?100:(Prog.xp/need*100))+'%';
  document.getElementById('cpXp').textContent=Prog.level>=100?'Max rank reached — prestige to begin again':(Prog.xp+' / '+need+' XP');
  const pm=document.getElementById('cpMedal');
  if(Prog.prestige>0){ pm.style.display=''; pm.innerHTML=prestigeMedalSVG(Prog.prestige,120)+'<div class="cpMedalLabel">'+prestigeCfg(Prog.prestige).label.toUpperCase()+'</div>'; } else pm.style.display='none';
  const kd=(Prog.kills/Math.max(1,Prog.deaths)).toFixed(2), hsP=Prog.kills?Math.round(Prog.headshots/Prog.kills*100):0;
  document.getElementById('cpStats').innerHTML=statRow('Kills',Prog.kills)+statRow('Deaths',Prog.deaths)+statRow('K / D',kd)+statRow('Headshots',Prog.headshots+' · '+hsP+'%')+statRow('Favorite Weapon',mostUsedWeapon())+statRow('Rank',r.name);
  document.getElementById('cpPrestige').style.display=Prog.level>=100?'':'none';
}
function openCareer(){ renderCareer(); careerModalEl.style.display='flex'; }
function closeCareer(){ careerModalEl.style.display='none'; }
const devPanelEl=document.getElementById('devPanel');
function devSync(){ document.getElementById('dvGod').classList.toggle('on',DEV.god); document.getElementById('dvAmmo').classList.toggle('on',DEV.infAmmo); document.getElementById('dvFreeze').classList.toggle('on',DEV.freezeAI); }
function devToggle(){ if(!DEV.enabled)return; const open=devPanelEl.style.display!=='block'; devPanelEl.style.display=open?'block':'none'; if(open){devSync();devSyncCam();devCamLive();} }
function devCamLive(){ if(!devPanelEl||devPanelEl.style.display!=='block')return;
  const d=camera.position.distanceTo(player.position); camera.getWorldDirection(_v);
  const pitch=Math.atan2(-_v.y,hyp2(_v.x,_v.z))*180/Math.PI;
  const e1=document.getElementById('dvCamDist'); if(e1)e1.textContent=d.toFixed(1)+' u';
  const e2=document.getElementById('dvCamPitch'); if(e2)e2.textContent=pitch.toFixed(1)+'° below horizontal';
  const e3=document.getElementById('dvCamHF'); if(e3)e3.textContent=camera.position.y.toFixed(1)+' u · '+T.fov+'° fov'; }
// ---- perf overlay (dev-panel, live): frame time + renderer.info draw/geo/tex/program counts + JS heap.
// Samples raw performance.now() deltas EVERY frame (so hitches show, unlike the clamped sim dt); DOM write throttled to ~4/s; fully idle while the panel is closed. ----
let _perfLast=0,_perfAccum=0,_perfCount=0,_perfMax=0,_perfNext=0,_perfEls=null;
function perfLive(){
  if(!devPanelEl||devPanelEl.style.display!=='block'){ if(_perfLast){_perfLast=0;_perfAccum=0;_perfCount=0;_perfMax=0;_perfNext=0;} return; }
  const now=performance.now();
  if(_perfLast){ const ft=now-_perfLast; _perfAccum+=ft; _perfCount++; if(ft>_perfMax)_perfMax=ft; }
  _perfLast=now;
  if(_perfCount<1||now<_perfNext) return;                 // hold the write until we have at least one real inter-frame sample
  _perfNext=now+250;
  const avg=_perfAccum/_perfCount, fps=avg>0?1000/avg:0, mx=_perfMax;
  _perfAccum=0; _perfCount=0; _perfMax=0;
  if(!_perfEls){ const g=id=>document.getElementById(id); _perfEls={fps:g('dvFps'),frame:g('dvFrame'),calls:g('dvCalls'),tris:g('dvTris'),geo:g('dvGeo'),tex:g('dvTex'),prog:g('dvProg'),heap:g('dvHeap')}; }
  const E=_perfEls, ri=renderer.info;
  if(E.fps) E.fps.textContent=fps.toFixed(0);
  if(E.frame) E.frame.textContent=avg.toFixed(1)+' / '+mx.toFixed(1)+' ms';
  if(E.calls) E.calls.textContent=ri.render.calls;
  if(E.tris) E.tris.textContent=ri.render.triangles.toLocaleString('en-US');
  if(E.geo) E.geo.textContent=ri.memory.geometries;
  if(E.tex) E.tex.textContent=ri.memory.textures;
  if(E.prog) E.prog.textContent=(ri.programs?ri.programs.length:0);
  if(E.heap) E.heap.textContent=(performance.memory?(performance.memory.usedJSHeapSize/1048576).toFixed(1)+' MB':'n/a');
}
const _CAMDEF={camHeight:T.camHeight, camBack:T.camBack, fov:T.fov, fovADS:T.fovADS};
// ---- camera style presets (dev experiment). yaw is a BASE azimuth added to live camYaw.
// Iso = true dimetric: yaw 45 deg, pitch atan(camHeight/camBack)=atan(40/56.6)=35.3deg ~ atan(1/sqrt2).
let _camStyle=3;
const CAM_STYLES={
  1:{id:1,camHeight:24,   camBack:16,   fov:52, fovADS:52, yaw:0,                  adsCam:'pitch'}, // Classic  (pre-bake follow cam)
  2:{id:2,camHeight:15.5, camBack:10.5, fov:52, fovADS:52, yaw:0,                  adsCam:'pitch'}, // Tactical (recent follow cam)
  3:{id:3,camHeight:40,   camBack:56.6, fov:15, fovADS:15, yaw:0.7853981633974483, adsCam:'pitch'}, // Isometric (Project Zomboid style)
  4:{id:4,camHeight:41.5, camBack:65.5, fov:12, fovADS:12, yaw:0.7853981633974483, adsCam:'pitch'}, // Iso+ (tuned default)
  5:{id:5,camHeight:52,   camBack:42,   fov:18, fovADS:18, yaw:0.7853981633974483, adsCam:'pan'}    // Foxhole (steep top-down + distance-pan ADS)
};
function applyCamStyle(s){ T.camHeight=s.camHeight; T.camBack=s.camBack; T.fov=s.fov; T.fovADS=s.fovADS; T.camYawBase=s.yaw; T.adsCam=s.adsCam||'pitch'; fovBase=s.fov; camYaw=0; camYawTarget=0; _camStyle=s.id; camPitchOverride=null; adsPitchK=0; adsPanK=0; devSyncCam(); }
function dvPitchSync(){ const pt=document.getElementById('dvPitch'); if(pt){ const pv=(camPitchOverride!=null?camPitchOverride:Math.atan2(T.camHeight,Math.max(0.001,T.camBack))*180/Math.PI); pt.value=pv; const v=document.getElementById('dvPitchV'); if(v)v.textContent=pv.toFixed(0)+'°'; } }
function devSyncCam(){ const b=document.getElementById('dvBoom'), h=document.getElementById('dvHeight'), f=document.getElementById('dvFov');
  if(b){ b.value=T.camBack; document.getElementById('dvBoomV').textContent=(+T.camBack).toFixed(1); }
  if(h){ h.value=T.camHeight; document.getElementById('dvHeightV').textContent=(+T.camHeight).toFixed(1); }
  if(f){ f.value=T.fov; document.getElementById('dvFovV').textContent=T.fov+'°'; }
  ['dvCam1','dvCam2','dvCam3','dvCam4','dvCam5'].forEach((id,i)=>{const e=document.getElementById(id); if(e)e.classList.toggle('on',i+1===_camStyle);});
  { const ac=document.getElementById('dvAdsCam'); if(ac) ac.textContent='ADS Cam: '+(T.adsCam==='pitch'?'Pitch':T.adsCam==='pan'?'Pan':'Off'); }
  dvPitchSync();
  { const mp=document.getElementById('dvMap'); if(mp){ mp.value=T.arenaX; document.getElementById('dvMapV').textContent=T.arenaX; } } }
function devSetLevel(n){ Prog.level=Math.max(1,Math.min(100,n)); Prog.xp=0; updateCareerChip(); saveProg(); showRankUp(Prog.level, rankForLevel(Prog.level)); }
function devPrestige(){ Prog.prestige++; Prog.level=1; Prog.xp=0; updateCareerChip(); saveProg(); document.querySelector('#rankBanner .rb-t').textContent='Prestige'; showBanner(prestigeMedalSVG(Prog.prestige,84),'Prestige '+Prog.prestige,prestigeCfg(Prog.prestige).label); }
function devReset(){ Prog.xp=0;Prog.level=1;Prog.prestige=0;Prog.kills=0;Prog.deaths=0;Prog.headshots=0;Prog.weapon={}; updateCareerChip(); saveProg(); showLevelToast('Dev','Progression reset'); }
let tutorialOpen=false;
const STAM={max:7, regen:0.2, lockUntil:0.3}; // 7s full sprint; regen 0.2/s; after exhaustion must refill to 30%

// --- WebAudio (synthesized; no external files) ---
const SFX={ctx:null,muted:false,master:0.85};
function audioKick(){ if(!SFX.ctx){ try{ SFX.ctx=new (window.AudioContext||window.webkitAudioContext)(); }catch(e){} } if(SFX.ctx&&SFX.ctx.state==='suspended') SFX.ctx.resume(); }
function _ramp(g,t,a,d,peak){ g.gain.setValueAtTime(0.0001,t); g.gain.exponentialRampToValueAtTime(peak,t+a); g.gain.exponentialRampToValueAtTime(0.0001,t+a+d); }
function sfxThunk(){ if(!SFX.ctx||SFX.muted)return; const c=SFX.ctx,t=c.currentTime,bus=c.createGain(); bus.gain.value=SFX.master; bus.connect(c.destination);
  const o=c.createOscillator(),g=c.createGain(); o.type='sine'; o.frequency.setValueAtTime(190,t); o.frequency.exponentialRampToValueAtTime(58,t+0.13); _ramp(g,t,0.004,0.17,0.95); o.connect(g).connect(bus); o.start(t); o.stop(t+0.22);
  const len=Math.floor(c.sampleRate*0.05),buf=c.createBuffer(1,len,c.sampleRate),dat=buf.getChannelData(0); for(let i=0;i<len;i++)dat[i]=(Math.random()*2-1)*Math.pow(1-i/len,2.6);
  const n=c.createBufferSource(); n.buffer=buf; const ng=c.createGain(),bp=c.createBiquadFilter(); bp.type='bandpass'; bp.frequency.value=1700; bp.Q.value=0.8; _ramp(ng,t,0.001,0.05,0.55); n.connect(bp).connect(ng).connect(bus); n.start(t); n.stop(t+0.06); }
function sfxKill(){ if(!SFX.ctx||SFX.muted)return; const c=SFX.ctx,t=c.currentTime,o=c.createOscillator(),g=c.createGain(); o.type='triangle'; o.frequency.setValueAtTime(440,t); o.frequency.exponentialRampToValueAtTime(150,t+0.09); _ramp(g,t,0.003,0.1,0.3*SFX.master); o.connect(g).connect(c.destination); o.start(t); o.stop(t+0.13); }
function sfxShot(w){ if(!SFX.ctx||SFX.muted)return; if(w&&w.id==='amk'){ sfxAMK(); return; } const c=SFX.ctx,t=c.currentTime;
  const big=(w.id==='amk'), shot=(w.id==='bagpipe'), fast=!!w.auto;
  const dur=big?0.26:(shot?0.2:(fast?0.07:0.11));
  const len=Math.max(1,Math.floor(c.sampleRate*dur)); const buf=c.createBuffer(1,len,c.sampleRate); const d=buf.getChannelData(0);
  for(let i=0;i<len;i++) d[i]=(Math.random()*2-1)*Math.pow(1-i/len, big?1.4:2.2);
  const src=c.createBufferSource(); src.buffer=buf;
  const bp=c.createBiquadFilter(); bp.type=shot?'lowpass':'bandpass'; bp.frequency.value=big?720:(shot?1100:1700); bp.Q.value=shot?0.6:0.9;
  const ng=c.createGain(); _ramp(ng,t,0.002,dur,(big?0.6:(shot?0.55:0.42))*SFX.master);
  src.connect(bp).connect(ng).connect(c.destination); src.start(t); src.stop(t+dur+0.06);
  const o=c.createOscillator(),g=c.createGain(); o.type='sine'; const f0=big?130:(shot?115:190);
  o.frequency.setValueAtTime(f0,t); o.frequency.exponentialRampToValueAtTime(f0*0.45,t+dur*0.9);
  _ramp(g,t,0.003,dur*0.8,(big?0.5:0.32)*SFX.master); o.connect(g).connect(c.destination); o.start(t); o.stop(t+dur+0.06); }
function sfxClank(){ if(!SFX.ctx||SFX.muted)return; const c=SFX.ctx,t=c.currentTime,o=c.createOscillator(),g=c.createGain(); o.type='square'; o.frequency.setValueAtTime(330,t); o.frequency.exponentialRampToValueAtTime(150,t+0.09); _ramp(g,t,0.002,0.1,0.22*SFX.master); o.connect(g).connect(c.destination); o.start(t); o.stop(t+0.13); }
function sfxStep(){ if(!SFX.ctx||SFX.muted)return; const c=SFX.ctx,t=c.currentTime; const len=Math.max(1,Math.floor(c.sampleRate*0.05)); const buf=c.createBuffer(1,len,c.sampleRate); const d=buf.getChannelData(0); for(let i=0;i<len;i++)d[i]=(Math.random()*2-1)*Math.pow(1-i/len,3); const src=c.createBufferSource(); src.buffer=buf; const lp=c.createBiquadFilter(); lp.type='lowpass'; lp.frequency.value=280; const g=c.createGain(); _ramp(g,t,0.003,0.05,0.1*SFX.master); src.connect(lp).connect(g).connect(c.destination); src.start(t); src.stop(t+0.09); }

// --- headshot-kill skull (Rust-style) ---
const skullEl=document.getElementById('skull'); let _skullTO=null;
function popSkull(){ skullEl.classList.remove('show'); void skullEl.offsetWidth; skullEl.classList.add('show'); clearTimeout(_skullTO); _skullTO=setTimeout(()=>skullEl.classList.remove('show'),760); }
const _audV=new THREE.Vector3(), _audR=new THREE.Vector3(); let _impSndUntil=0, _amb=null;
function sfxBus(pos){ const c=SFX.ctx, g=c.createGain(); if(!pos){ g.gain.value=1; g.connect(c.destination); return g; }
  _audV.copy(pos).sub(player.position); _audV.y=0; const dist=_audV.length(); g.gain.value=1/(1+dist/22); // inverse-distance: ~0.5 @22u, ~0.25 @66u
  if(c.createStereoPanner){ _audR.setFromMatrixColumn(camera.matrixWorld,0); _audR.y=0; const rl=_audR.length()||1; const px=Math.max(-1,Math.min(1,(_audV.x*_audR.x+_audV.z*_audR.z)/rl/20)); const pan=c.createStereoPanner(); pan.pan.value=px; g.connect(pan).connect(c.destination); } else { g.connect(c.destination); }
  return g; }
function sfxBoom(pos){ if(!SFX.ctx||SFX.muted)return; const c=SFX.ctx,t=c.currentTime,bus=sfxBus(pos);
  const o=c.createOscillator(),g=c.createGain(); o.type='sine'; o.frequency.setValueAtTime(120,t); o.frequency.exponentialRampToValueAtTime(38,t+0.32); _ramp(g,t,0.005,0.4,0.85*SFX.master); o.connect(g).connect(bus); o.start(t); o.stop(t+0.5);
  const len=Math.floor(c.sampleRate*0.4),buf=c.createBuffer(1,len,c.sampleRate),d=buf.getChannelData(0); for(let i=0;i<len;i++)d[i]=(Math.random()*2-1)*Math.pow(1-i/len,1.6); const src=c.createBufferSource(); src.buffer=buf; const lp=c.createBiquadFilter(); lp.type='lowpass'; lp.frequency.value=900; const ng=c.createGain(); _ramp(ng,t,0.002,0.34,0.7*SFX.master); src.connect(lp).connect(ng).connect(bus); src.start(t); src.stop(t+0.44); }
function sfxImpact(pos){ if(!SFX.ctx||SFX.muted)return; const now=(typeof performance!=='undefined'&&performance.now)?performance.now():Date.now(); if(now<_impSndUntil)return; _impSndUntil=now+55; const c=SFX.ctx,t=c.currentTime,bus=sfxBus(pos);
  const len=Math.floor(c.sampleRate*0.04),buf=c.createBuffer(1,len,c.sampleRate),d=buf.getChannelData(0); for(let i=0;i<len;i++)d[i]=(Math.random()*2-1)*Math.pow(1-i/len,3); const src=c.createBufferSource(); src.buffer=buf; const hp=c.createBiquadFilter(); hp.type='highpass'; hp.frequency.value=2200; const g=c.createGain(); _ramp(g,t,0.001,0.035,0.16*SFX.master); src.connect(hp).connect(g).connect(bus); src.start(t); src.stop(t+0.06); }
function sfxEnemyShot(pos){ if(!SFX.ctx||SFX.muted)return; const c=SFX.ctx,t=c.currentTime,bus=sfxBus(pos);
  const len=Math.floor(c.sampleRate*0.09),buf=c.createBuffer(1,len,c.sampleRate),d=buf.getChannelData(0); for(let i=0;i<len;i++)d[i]=(Math.random()*2-1)*Math.pow(1-i/len,2.2); const src=c.createBufferSource(); src.buffer=buf; const bp=c.createBiquadFilter(); bp.type='bandpass'; bp.frequency.value=1500; bp.Q.value=0.8; const ng=c.createGain(); _ramp(ng,t,0.002,0.09,0.4*SFX.master); src.connect(bp).connect(ng).connect(bus); src.start(t); src.stop(t+0.14);
  const o=c.createOscillator(),g=c.createGain(); o.type='sine'; o.frequency.setValueAtTime(150,t); o.frequency.exponentialRampToValueAtTime(70,t+0.08); _ramp(g,t,0.003,0.08,0.28*SFX.master); o.connect(g).connect(bus); o.start(t); o.stop(t+0.13); }
function sfxTick(){ if(!SFX.ctx||SFX.muted)return; const c=SFX.ctx,t=c.currentTime,o=c.createOscillator(),g=c.createGain(); o.type='square'; o.frequency.setValueAtTime(1400,t); o.frequency.exponentialRampToValueAtTime(900,t+0.03); _ramp(g,t,0.001,0.035,0.12*SFX.master); o.connect(g).connect(c.destination); o.start(t); o.stop(t+0.05); }
function sfxReload(stage){ if(!SFX.ctx||SFX.muted)return; if(typeof curW==='function'&&curW()&&curW().id==='amk'){ sfxAMKReload(stage); return; } const c=SFX.ctx,t=c.currentTime,f=stage?520:360; const o=c.createOscillator(),g=c.createGain(); o.type='square'; o.frequency.setValueAtTime(f,t); o.frequency.exponentialRampToValueAtTime(f*0.6,t+0.05); _ramp(g,t,0.001,0.06,0.18*SFX.master); o.connect(g).connect(c.destination); o.start(t); o.stop(t+0.09);
  const len=Math.floor(c.sampleRate*0.03),buf=c.createBuffer(1,len,c.sampleRate),dd=buf.getChannelData(0); for(let i=0;i<len;i++)dd[i]=(Math.random()*2-1)*Math.pow(1-i/len,2); const src=c.createBufferSource(); src.buffer=buf; const hp=c.createBiquadFilter(); hp.type='highpass'; hp.frequency.value=1200; const ng=c.createGain(); _ramp(ng,t,0.001,0.03,0.12*SFX.master); src.connect(hp).connect(ng).connect(c.destination); src.start(t); src.stop(t+0.05); }
// ---- AMK Rifle: Springfield M1903 (WW2 sniper) -- heavy saturated-noise blast + convolution "rolling street" echo ----
// The report is broadband noise (no tonal oscillator -> not 8-bit): a bright CRACK + a heavy low BODY that darkens as it
// opens, a deep SUB thump for weight, all saturated for punch. The echo is a generated CONVOLUTION reverb -- irregular,
// decorrelated early slaps (off buildings at different distances) feeding a dense dark decaying tail. No feedback loop,
// so no comb resonance -> no metallic "tin-can" ping/flutter; it rolls like a shot down a war-torn street/canyon.
let _amkNoiseBuf=null, _amkShaper=null, _amkEcho=null;
function _amkEnsure(c){
  if(!_amkNoiseBuf){ const len=Math.floor(c.sampleRate*2.0); _amkNoiseBuf=c.createBuffer(1,len,c.sampleRate); const d=_amkNoiseBuf.getChannelData(0); for(let i=0;i<len;i++)d[i]=Math.random()*2-1; }
  if(!_amkShaper){ const n=1024; _amkShaper=new Float32Array(n); for(let i=0;i<n;i++){ const x=i/(n-1)*2-1; _amkShaper[i]=Math.tanh(2.2*x); } }
  if(!_amkEcho){
    const dur=1.5, len=Math.floor(c.sampleRate*dur), ir=c.createBuffer(2,len,c.sampleRate);
    for(let ch=0;ch<2;ch++){ const d=ir.getChannelData(ch); let lp=0;
      for(let i=0;i<len;i++){ const frac=i/len, env=Math.pow(1-frac,2.0); const s=Math.random()*2-1; lp+=(s-lp)*0.13; d[i]=lp*env*0.55; }
      const taps=(ch===0)?[[0.063,0.9],[0.117,0.62],[0.189,0.44],[0.271,0.3],[0.36,0.2]]:[[0.081,0.85],[0.143,0.58],[0.221,0.42],[0.304,0.28],[0.39,0.19]];
      for(let k=0;k<taps.length;k++){ const idx=Math.floor(taps[k][0]*c.sampleRate); if(idx<len) d[idx]+=(Math.random()<0.5?-1:1)*taps[k][1]; }
    }
    const conv=c.createConvolver(); conv.normalize=false; conv.buffer=ir;
    const inp=c.createGain(); inp.gain.value=1;
    const wetHP=c.createBiquadFilter(); wetHP.type='highpass'; wetHP.frequency.value=95;   // keep sub out of the wet (no mud)
    const wetLP=c.createBiquadFilter(); wetLP.type='lowpass'; wetLP.frequency.value=650;    // muffled / distant, not tinny (darker)
    const out=c.createGain(); out.gain.value=0.45*SFX.master;
    inp.connect(conv); conv.connect(wetHP); wetHP.connect(wetLP); wetLP.connect(out); out.connect(c.destination);
    _amkEcho=inp;
  }
}
function sfxAMK(){ if(!SFX.ctx||SFX.muted)return; const c=SFX.ctx,t=c.currentTime; _amkEnsure(c);
  const master=c.createGain(); master.gain.value=0.5*SFX.master;
  const comp=c.createDynamicsCompressor(); comp.threshold.value=-8; comp.knee.value=6; comp.ratio.value=3; comp.attack.value=0.003; comp.release.value=0.15;
  master.connect(comp); comp.connect(c.destination);   // limiter: tames the transient (less punchy) + prevents clipping
  const offA=Math.random()*1.4, offB=Math.random()*1.4;   // random buffer offsets -> each shot's noise differs
  // Layer A -- CRACK: bright saturated noise, near-instant attack (the supersonic snap)
  const nA=c.createBufferSource(); nA.buffer=_amkNoiseBuf;
  const hpA=c.createBiquadFilter(); hpA.type='highpass'; hpA.frequency.value=1300;
  const pkA=c.createBiquadFilter(); pkA.type='peaking'; pkA.frequency.value=2600; pkA.Q.value=0.7; pkA.gain.value=3;
  const wsA=c.createWaveShaper(); wsA.curve=_amkShaper; wsA.oversample='4x';
  const gA=c.createGain(); gA.gain.setValueAtTime(0.0001,t); gA.gain.exponentialRampToValueAtTime(0.5,t+0.002); gA.gain.exponentialRampToValueAtTime(0.001,t+0.05);
  nA.connect(hpA); hpA.connect(pkA); pkA.connect(wsA); wsA.connect(gA); gA.connect(master); gA.connect(_amkEcho);
  // Layer B -- BODY: heavy low boom; lowpass sweeps bright->dark, low-shelf lift adds weight, saturated for grit
  const nB=c.createBufferSource(); nB.buffer=_amkNoiseBuf;
  const lpB=c.createBiquadFilter(); lpB.type='lowpass'; lpB.Q.value=0.8; lpB.frequency.setValueAtTime(1600,t); lpB.frequency.exponentialRampToValueAtTime(140,t+0.28);
  const lsB=c.createBiquadFilter(); lsB.type='lowshelf'; lsB.frequency.value=165; lsB.gain.value=6;
  const wsB=c.createWaveShaper(); wsB.curve=_amkShaper; wsB.oversample='4x';
  const gB=c.createGain(); gB.gain.setValueAtTime(0.0001,t); gB.gain.exponentialRampToValueAtTime(1.0,t+0.004); gB.gain.exponentialRampToValueAtTime(0.001,t+0.3);
  nB.connect(lpB); lpB.connect(lsB); lsB.connect(wsB); wsB.connect(gB); gB.connect(master); gB.connect(_amkEcho);
  // Layer C -- SUB: deep thump for chest weight (short, felt not heard as a tone)
  const sub=c.createOscillator(); sub.type='sine'; sub.frequency.setValueAtTime(64,t); sub.frequency.exponentialRampToValueAtTime(34,t+0.1);
  const gS=c.createGain(); gS.gain.setValueAtTime(0.0001,t); gS.gain.exponentialRampToValueAtTime(0.8,t+0.004); gS.gain.exponentialRampToValueAtTime(0.001,t+0.18);
  sub.connect(gS); gS.connect(master);
  nA.start(t,offA); nB.start(t,offB); sub.start(t);
  nA.stop(t+0.3); nB.stop(t+0.42); sub.stop(t+0.22);
}
function sfxAMKReload(stage){ if(!SFX.ctx||SFX.muted)return; const c=SFX.ctx,t=c.currentTime;
  // metallic bolt clack (filtered noise transient) — stage 0 = bolt back, stage 1 = bolt forward
  const len=Math.floor(c.sampleRate*0.09),buf=c.createBuffer(1,len,c.sampleRate),d=buf.getChannelData(0); for(let i=0;i<len;i++)d[i]=(Math.random()*2-1)*Math.pow(1-i/len,2.4);
  const src=c.createBufferSource(); src.buffer=buf; const bp=c.createBiquadFilter(); bp.type='bandpass'; bp.frequency.value=stage?2600:1900; bp.Q.value=1.3; const ng=c.createGain(); _ramp(ng,t,0.001,0.09,0.24*SFX.master); src.connect(bp).connect(ng).connect(c.destination); src.start(t); src.stop(t+0.11);
  // metallic resonance ping (receiver / bolt ring)
  const o=c.createOscillator(),g=c.createGain(); o.type='triangle'; const f=stage?300:220; o.frequency.setValueAtTime(f*2.2,t); o.frequency.exponentialRampToValueAtTime(f,t+0.06); _ramp(g,t,0.001,0.1,0.15*SFX.master); o.connect(g).connect(c.destination); o.start(t); o.stop(t+0.13);
  // bolt-close (stage 1): a low seating thunk = the satisfying lock-up
  if(stage){ const o2=c.createOscillator(),g2=c.createGain(); o2.type='sine'; o2.frequency.setValueAtTime(150,t); o2.frequency.exponentialRampToValueAtTime(72,t+0.08); _ramp(g2,t,0.002,0.11,0.3*SFX.master); o2.connect(g2).connect(c.destination); o2.start(t); o2.stop(t+0.15); }
}
function startAmbient(){ if(!SFX.ctx||SFX.muted||_amb)return; const c=SFX.ctx; const len=Math.floor(c.sampleRate*2),buf=c.createBuffer(1,len,c.sampleRate),d=buf.getChannelData(0); let last=0; for(let i=0;i<len;i++){ last=last*0.96+(Math.random()*2-1)*0.04; d[i]=last; }
  const src=c.createBufferSource(); src.buffer=buf; src.loop=true; const lp=c.createBiquadFilter(); lp.type='lowpass'; lp.frequency.value=420; const g=c.createGain(); g.gain.value=0.0001; g.gain.setTargetAtTime(0.05*SFX.master,c.currentTime,1.5); src.connect(lp).connect(g).connect(c.destination); src.start(); _amb={src:src,g:g}; }
function stopAmbient(){ if(!_amb||!SFX.ctx)return; const c=SFX.ctx; try{ _amb.g.gain.setTargetAtTime(0.0001,c.currentTime,0.4); const s=_amb.src; setTimeout(function(){try{s.stop();}catch(e){}},800); }catch(e){} _amb=null; }
function onHeadshotKill(){ popSkull(); sfxThunk(); }

// --- MW3-style XP earned banner (accumulates, then fades fast) ---
const xpEarnEl=document.getElementById('xpEarn'), xpEarnNumEl=document.getElementById('xpEarnNum');
let _xpAccum=0,_xpHideTO=null,_xpClearTO=null;
function showXpEarn(n){ _xpAccum+=n; xpEarnNumEl.textContent='+'+_xpAccum; xpEarnEl.classList.add('show');
  xpEarnNumEl.classList.remove('bump'); void xpEarnNumEl.offsetWidth; xpEarnNumEl.classList.add('bump');
  clearTimeout(_xpHideTO); _xpHideTO=setTimeout(()=>xpEarnEl.classList.remove('show'),1050);
  clearTimeout(_xpClearTO); _xpClearTO=setTimeout(()=>{ _xpAccum=0; },1500); }

// --- onboarding overlay (first deploy; persisted) ---
const tutorialEl=document.getElementById('tutorial'); const TUT_KEY='shamble_tutorial_seen';
const TutStore={ persist:KVStore.persist, // ungated on purpose: KVStore's memory fallback still remembers a dismissal for the session
  async load(){ for(const k of [TUT_KEY,'vanguard_tutorial_seen']){ try{ const r=await KVStore.get(k); if(r&&r.value==='1') return true; }catch(e){} } return false; },
  save(){ KVStore.set(TUT_KEY,'1'); } };
function openTutorial(){ tutorialOpen=true; tutorialEl.style.display='flex'; }
function closeTutorial(){ if(!tutorialOpen)return; tutorialOpen=false; tutorialEl.style.display='none'; TutStore.save(); }
function maybeShowTutorial(){ TutStore.load().then(seen=>{ if(!seen && mode==='game') openTutorial(); }); }
const staminaBarEl=document.getElementById('staminaBar'), staminaFillEl=document.getElementById('staminaFill');

/* ===== XIII-style KILL VIGNETTE — hit-stop + comic picture-in-picture panels (headshots & melee) ===== */
let hitStop=0; const HITSTOP=0.12;
let vignetteActive=false, _vigRT=null, _vigCam=null, _vigOutTO=null, _vigDoneTO=null, _vigQueue=[];
const VIG_W=300, VIG_H=200;
const vigEl=document.getElementById('killStrip');
const vigCanvases=vigEl?[...vigEl.querySelectorAll('canvas')]:[];
const vigOnomEl=vigEl?vigEl.querySelector('.vig-onom'):null;
function vigEnsure(){ if(_vigRT)return; for(let i=0;i<vigCanvases.length;i++) vigCanvases[i].style.filter='contrast(1.12) saturate(1.18)'; _vigRT=new THREE.WebGLRenderTarget(VIG_W,VIG_H,{minFilter:THREE.LinearFilter,magFilter:THREE.LinearFilter,format:THREE.RGBAFormat}); _vigCam=new THREE.PerspectiveCamera(40,VIG_W/VIG_H,0.1,400); }
// posterize + Sobel ink + (CSS halftone on top) → cel/comic look, one-time on a small canvas
function comicPost(ctx){ const w=VIG_W,h=VIG_H,src=ctx.getImageData(0,0,w,h),d=src.data,lum=new Float32Array(w*h);
  for(let i=0,p=0;i<d.length;i+=4,p++) lum[p]=0.299*d[i]+0.587*d[i+1]+0.114*d[i+2];
  const levels=5, step=255/(levels-1);
  for(let i=0;i<d.length;i+=4){ for(let k=0;k<3;k++){ let v=(d[i+k]-128)*1.22+128; v=Math.round(Math.max(0,Math.min(255,v))/step)*step; d[i+k]=v; } }
  for(let y=1;y<h-1;y++){ for(let x=1;x<w-1;x++){
      const a=lum[(y-1)*w+x-1],b=lum[(y-1)*w+x],c=lum[(y-1)*w+x+1],e=lum[y*w+x-1],f=lum[y*w+x+1],g=lum[(y+1)*w+x-1],hh=lum[(y+1)*w+x],ii=lum[(y+1)*w+x+1];
      const gx=-a-2*e-g+c+2*f+ii, gy=-a-2*b-c+g+2*hh+ii, mag=Math.sqrt(gx*gx+gy*gy);
      if(mag>88){ const di=(y*w+x)*4, q=Math.max(0,1-(mag-88)/110); d[di]*=q; d[di+1]*=q; d[di+2]*=q; } } }
  ctx.putImageData(src,0,0); }
const _vigSz=new THREE.Vector2();
// Capture one vignette shot. GPU path: render the shot into the bottom-left corner of the main canvas and drawImage that
// corner into the panel canvas — a GPU-side copy on accelerated canvases, so no readPixels stall and no per-pixel JS
// (the old path stalled the GPU twice per shot and ran two full-image JS loops). vigStep runs BEFORE the main render,
// which paints over the corner, so nothing is ever seen there. The readback path stays as the automatic fallback.
function vigShot(camPos, look, fov, canvas){
  _vigCam.fov=fov; _vigCam.position.copy(camPos); _vigCam.up.set(0,1,0); _vigCam.lookAt(look); _vigCam.updateProjectionMatrix();
  const ctx=canvas.getContext('2d'); if(!ctx) return;
  const el=renderer.domElement, pr=renderer.getPixelRatio(), sz=renderer.getSize(_vigSz);
  try{ renderer.setRenderTarget(null); renderer.setScissorTest(true); renderer.setViewport(0,0,VIG_W,VIG_H); renderer.setScissor(0,0,VIG_W,VIG_H);
    renderer.render(scene,_vigCam);
    renderer.setScissorTest(false); renderer.setViewport(0,0,sz.x,sz.y);
    ctx.drawImage(el, 0, el.height-Math.round(VIG_H*pr), Math.round(VIG_W*pr), Math.round(VIG_H*pr), 0, 0, canvas.width, canvas.height);
  }catch(err){ renderer.setScissorTest(false); renderer.setViewport(0,0,sz.x,sz.y); vigShotReadback(camPos, look, fov, canvas); } }
function vigShotReadback(camPos, look, fov, canvas){
  _vigCam.fov=fov; _vigCam.position.copy(camPos); _vigCam.up.set(0,1,0); _vigCam.lookAt(look); _vigCam.updateProjectionMatrix();
  renderer.setRenderTarget(_vigRT); renderer.render(scene,_vigCam); renderer.setRenderTarget(null);
  const buf=new Uint8Array(VIG_W*VIG_H*4); renderer.readRenderTargetPixels(_vigRT,0,0,VIG_W,VIG_H,buf);
  const ctx=canvas.getContext('2d'), img=ctx.createImageData(VIG_W,VIG_H), o=img.data;
  for(let y=0;y<VIG_H;y++){ const sy=VIG_H-1-y; for(let x=0;x<VIG_W;x++){ const si=(sy*VIG_W+x)*4, di=(y*VIG_W+x)*4; o[di]=buf[si];o[di+1]=buf[si+1];o[di+2]=buf[si+2];o[di+3]=255; } }
  ctx.putImageData(img,0,0); comicPost(ctx);
}
function triggerVignette(victim, kind){
  if(vignetteActive || !vigEl || vigCanvases.length<3) return;
  try{
    vigEnsure();
    const vp=victim.group.position.clone();
    const head=vp.clone().setY(2.0), chest=vp.clone().setY(1.35);
    let toV=vp.clone().sub(player.position); toV.y=0; if(toV.lengthSq()<0.0001)toV.set(0,0,1); toV.normalize();
    const side=new THREE.Vector3(-toV.z,0,toV.x);
    const wide=chest.clone().addScaledVector(toV,-5.0).addScaledVector(side,2.7); wide.y=4.4;   // establishing
    const close=head.clone().addScaledVector(toV,-1.7).addScaledVector(side,0.85); close.y=2.55; // impact
    const hero=chest.clone().addScaledVector(toV,-2.3).addScaledVector(side,-2.1); hero.y=0.75;  // low hero / result
    // enqueue the 3 shots; vigStep() drains ONE per frame during the hit-stop, so the three
    // GPU readbacks spread across successive frozen frames instead of stalling the kill frame
    _vigQueue.length=0;
    _vigQueue.push({pos:wide,  look:chest, fov:42, canvas:vigCanvases[0]});
    _vigQueue.push({pos:close, look:head,  fov:33, canvas:vigCanvases[1]});
    _vigQueue.push({pos:hero,  look:head,  fov:48, canvas:vigCanvases[2]});
    const words = kind==='melee' ? ['WHACK!','THWAK!','SHK!','KRRK!'] : ['CRACK!','BLAM!','POW!','KRAK!'];
    if(vigOnomEl) vigOnomEl.textContent = words[Math.floor(Math.random()*words.length)];
    vignetteActive=true; hitStop=Math.max(hitStop,HITSTOP);
    vigEl.classList.remove('out'); vigEl.classList.add('show');   // (no forced reflow: the previous vignette already cleared 'show')
    clearTimeout(_vigOutTO); clearTimeout(_vigDoneTO);
    _vigOutTO=setTimeout(()=>vigEl.classList.add('out'),1550);
    _vigDoneTO=setTimeout(()=>{ vigEl.classList.remove('show','out'); vignetteActive=false; },2150);
  }catch(err){ renderer.setRenderTarget(null); vignetteActive=false; _vigQueue.length=0; }
}
// Drain ONE queued vignette capture per frame (called from loop() during the hit-stop). The
// scene is frozen while hitStop>0, so the three freeze-frames are identical to a same-frame capture.
function vigStep(){ if(!_vigQueue.length) return; const s=_vigQueue.shift(); try{ vigShot(s.pos, s.look, s.fov, s.canvas); }catch(err){ renderer.setRenderTarget(null); } }
function updateWeaponHUD(){const w=curW(),l=curL();weaponNameEl.textContent=w.name;if(w.melee){ammoVal.textContent='—';reserveVal.textContent='';ammoEl.classList.remove('low','empty');}else{ammoVal.textContent=l.ammo;reserveVal.textContent=w.special?('/ '+l.reserve):'∞';ammoEl.classList.toggle('low',w.special&&l.reserve<=2);ammoEl.classList.toggle('empty',l.ammo<=0);}updateRank();}
function updateHealthHUD(){const h=Math.max(0,ps.health);hpBarI.style.width=(h/ps.maxHealth*100)+'%';hpNum.textContent=Math.round(h);hpBar.classList.toggle('hurt',h<60&&h>=30);hpBar.classList.toggle('crit',h<30);}
const creditValEl=document.getElementById('creditVal'); let _lastCred=-1;
function updateCreditHUD(){ const v=Math.floor(credits); if(v!==_lastCred){ _lastCred=v; creditValEl.textContent=v; } }
// ─── Phase 0: team tech / era meter ───
const eraUsEl=document.getElementById('eraUs'), eraFillUsEl=document.getElementById('eraFillUs'), eraNumUsEl=document.getElementById('eraNumUs'), eraThemEl=document.getElementById('eraThem'), eraHudEl=document.getElementById('eraHud');
let _eraF=-1,_eraTF=-1,_eraH=-1;
function updateEraHUD(){
  const f=teamEco[PLAYER_TEAM],h=teamEco[TEAM_INDEX.firstEnemy],maxE=ECO.eraNames.length,tf=Math.floor(f.tech);
  if(f.era===_eraF&&tf===_eraTF&&h.era===_eraH) return;            // change-detected: allocation-free when idle
  _eraF=f.era;_eraTF=tf;_eraH=h.era;
  eraUsEl.textContent=ECO.eraNames[f.era-1]; eraThemEl.textContent=ECO.eraNames[h.era-1];
  if(f.era>=maxE){ eraHudEl.classList.add('max'); eraNumUsEl.textContent='MAX'; eraFillUsEl.style.width='100%'; }
  else { eraHudEl.classList.remove('max'); const need=ECO.eraCost[f.era-1]; eraFillUsEl.style.width=Math.min(100,tf/need*100).toFixed(1)+'%'; eraNumUsEl.textContent=tf+' / '+need; }
}
function onEraUp(team){
  if(sameTeam(team,PLAYER_TEAM)){ const got=grantFreeWeapons(true); showLevelToast('Tech Advanced', ECO.eraNames[teamEco[PLAYER_TEAM].era-1]+(got?' · '+got+' issued':' online')); if(shopOpen) renderShop(); }
  else { const e=teamEco[teamOf(team)]; if(e) showLevelToast('Enemy Tech','Hostiles reached '+ECO.eraNames[e.era-1]); }
}
function addTech(team,amt){
  const id=teamOf(team); if(!id)return;
  amt = amt * (powerDead(id)?0.5:1);   // power plant down -> tech income halved
  team=id; const e=teamEco[id];
  if(e.era<ECO.eraNames.length){
    e.tech+=amt;
    let need=ECO.eraCost[e.era-1];
    while(e.era<ECO.eraNames.length&&e.tech>=need){ e.tech-=need; e.era++; onEraUp(team); need=(e.era<ECO.eraNames.length)?ECO.eraCost[e.era-1]:Infinity; }
    if(e.era>=ECO.eraNames.length) e.tech=0;
  }
  updateEraHUD();
}
const _abLabEl=document.getElementById('abilityLab'), _abValEl=document.getElementById('nadeVal');
function updateAbilityHUD(){
  if(!_abValEl) return; const u=unitOf(ps.unit); const lab=u.name.toUpperCase(); let txt='—';
  if(u.ability==='molotov') txt = ps.molotovMode?'MOLOTOV':'RIFLE';
  else if(u.ability==='shield') txt = ps.shieldOn?'SHIELD':'READY';
  else if(u.ability==='banzai') txt = ps.banzai?('BANZAI '+Math.ceil(Math.max(0,ps.banzaiT))+'s') : (ps.banzaiCd>0?('CD '+Math.ceil(ps.banzaiCd)+'s'):'READY');
  else if(u.ability==='bayonet') txt = ps.bayonet?'IMPALE!' : (ps.bayonetCd>0?('CD '+Math.ceil(ps.bayonetCd)+'s'):'READY');
  if(_abLabEl && _abLabEl.textContent!==lab) _abLabEl.textContent=lab;
  if(_abValEl.textContent!==txt) _abValEl.textContent=txt;
}
function updateGrenadeHUD(){ updateAbilityHUD(); updateFragHUD(); }
function updateFragHUD(){ const el=document.getElementById('fragVal'); if(el) el.textContent=grenadeCount; }
function buyGrenade(){ const cost=50*priceMult(); if(grenadeCount>=T.grenadeMax||credits<cost) return; credits-=cost; grenadeCount++; updateCreditHUD(); updateGrenadeHUD(); }
function updateHostileCount(){const el=document.getElementById('hostileCount'); if(el)el.textContent=String(enemies.filter(e=>e.alive).length).padStart(2,'0');}
let _sbLvl=-1, _sbBuilt=-1, _sbRowsA=[], _sbRowsB=[], _sbR=null;
function _sbMakeRow(team){ const row=document.createElement('div'); row.className='sb-prow';
  const rank=document.createElement('span'); rank.className=team==='h'?'sb-rank h-rank':'sb-rank';
  const name=document.createElement('span'); name.className='sb-pname';
  const k=document.createElement('span'); k.className='sb-pk';
  const d=document.createElement('span'); d.className='sb-pd';
  const a=document.createElement('span'); a.className='sb-pa';
  row.appendChild(rank); row.appendChild(name); row.appendChild(k); row.appendChild(d); row.appendChild(a);
  return {root:row,rank:rank,name:name,k:k,d:d,a:a,src:null}; }
function sbBuildRosters(){ const listA=document.getElementById('sbListA'), listB=document.getElementById('sbListB'); if(!listA||!listB)return;
  listA.innerHTML=''; _sbRowsA=[]; const you=_sbMakeRow('a'); you.root.classList.add('you'); you.src='you'; you.rank.innerHTML=rankInsigniaSVG(Prog.level,22); you.name.textContent='You'; listA.appendChild(you.root); _sbRowsA.push(you);
  listB.innerHTML=''; _sbRowsB=[]; for(let i=0;i<enemies.length;i++){ const en=enemies[i]; if(!en.active||sameTeam(en.team,PLAYER_TEAM))continue; const r=_sbMakeRow('h'); r.src=en; r.name.textContent='Hostile '+String(_sbRowsB.length+1).padStart(2,'0'); listB.appendChild(r.root); _sbRowsB.push(r); }
  _sbBuilt=TEAM_INDEX.rosterVer; _sbLvl=-1; }
function sbSync(){ const el=scoreboardEl; if(!el||el.style.display!=='flex')return; if(mode!=='game'){ el.style.display='none'; return; }
  if(_sbBuilt!==TEAM_INDEX.rosterVer) sbBuildRosters();
  if(!_sbR){ const g=id=>document.getElementById(id); _sbR={aK:g('sbAKills'),bK:g('sbBKills'),aAlive:g('sbAAlive'),bAlive:g('sbBAlive'),aTier:g('sbATier'),bTier:g('sbBTier'),atK:g('sbATotK'),atD:g('sbATotD'),atA:g('sbATotA'),btK:g('sbBTotK'),btD:g('sbBTotD'),btA:g('sbBTotA'),mIns:g('sbMeIns'),mRank:g('sbMeRank'),mLvl:g('sbMeLvl'),mPrest:g('sbMePrest'),mXp:g('sbMeXp'),mXn:g('sbMeXpTxt'),mK:g('sbMeK'),mD:g('sbMeD'),mA:g('sbMeA')}; }
  const lvlChanged=(Prog.level!==_sbLvl);
  let aK=0,aD=0,aA=0,aAlive=0;
  for(const r of _sbRowsA){ let k,d,a,alive; if(r.src==='you'){ k=kills; d=roundDeaths; a=roundAssists; alive=!ps.dead; if(lvlChanged)r.rank.innerHTML=rankInsigniaSVG(Prog.level,22); } else { k=r.src.k||0; d=r.src.d||0; a=r.src.a||0; alive=r.src.alive!==false; }
    _st(r.k,k); _st(r.d,d); _st(r.a,a); r.root.classList.toggle('down',!alive); aK+=k; aD+=d; aA+=a; if(alive)aAlive++; }
  let bK=0,bD=0,bAlive=0;
  for(const r of _sbRowsB){ const en=r.src, k=en.frags||0, d=en.downs||0; _st(r.k,k); _st(r.d,d); _st(r.a,'—'); r.root.classList.toggle('down',!en.alive); bK+=k; bD+=d; if(en.alive)bAlive++; }
  _st(_sbR.aK,aK); _st(_sbR.bK,bK); _st(_sbR.aAlive,aAlive); _st(_sbR.bAlive,bAlive);   // B5: write only on change
  _st(_sbR.aTier,ECO.eraNames[teamEco[PLAYER_TEAM].era-1]); _st(_sbR.bTier,ECO.eraNames[teamEco[TEAM_INDEX.firstEnemy].era-1]);
  _st(_sbR.atK,aK); _st(_sbR.atD,aD); _st(_sbR.atA,aA); _st(_sbR.btK,bK); _st(_sbR.btD,bD); _st(_sbR.btA,'—');
  if(lvlChanged){ _sbR.mIns.innerHTML=rankInsigniaSVG(Prog.level,40); _sbR.mRank.textContent=rankForLevel(Prog.level).name; _sbLvl=Prog.level; }
  _st(_sbR.mLvl,'Level '+Prog.level);
  if(Prog.prestige>0){ _sd(_sbR.mPrest,''); _st(_sbR.mPrest,'✦'+Prog.prestige); } else _sd(_sbR.mPrest,'none');
  const _need=xpForLevel(Prog.level), _frac=Prog.level>=100?1:Math.max(0,Math.min(1,Prog.xp/_need));
  { const w=(_frac*100)+'%'; if(_sbR.mXp.style.width!==w)_sbR.mXp.style.width=w; } _st(_sbR.mXn,Prog.level>=100?'MAX RANK':Prog.xp+' / '+_need+' XP');
  _st(_sbR.mK,kills); _st(_sbR.mD,roundDeaths); _st(_sbR.mA,roundAssists); }
function sbShow(on){ if(!scoreboardEl)return; if(on&&_sbBuilt!==TEAM_INDEX.rosterVer)sbBuildRosters(); scoreboardEl.style.display=on?'flex':'none'; if(on)sbSync(); }
function updateBaseHUD(){document.querySelectorAll('.pip').forEach(p=>{const[t,ty]=p.dataset.b.split('-');const ix=TEAM_INDEX[t==='f'?PLAYER_TEAM:TEAM_INDEX.firstEnemy]; if(!ix)return;
  if(ty==='harv'){ const hv=ix.harv; if(!hv)return; p.classList.toggle('dead',!hv.alive); p.querySelector('i').style.width=(hv.alive?Math.max(0,hv.health/hv.maxHealth)*100:0)+'%'; return; }
  const b=ty==='hq'?ix.hqs[0]:ty==='refinery'?ix.refinery:ty==='power'?ix.power:ty==='barracks'?ix.barracksB:null;if(!b)return;p.classList.toggle('dead',!b.alive);p.querySelector('i').style.width=(Math.max(0,b.health/b.maxHealth)*100)+'%';});}
let hitmarkT=0;
function showHitmarker(kill,head){hitmarkT=0.2;hitmarkEl.style.display='block';hitmarkEl.style.opacity='1';hitmarkEl.querySelectorAll('span').forEach(s=>{s.classList.toggle('kill',!!kill&&!head);s.classList.toggle('head',!!head);}); if(!kill&&!head)sfxTick();
  const s0=kill?2.1:head?1.75:1.5; if(hitmarkEl.animate) hitmarkEl.animate([{transform:'translate(-50%,-50%) rotate(45deg) scale('+s0+')'},{transform:'translate(-50%,-50%) rotate(45deg) scale(1)'}],{duration:190,easing:'cubic-bezier(.2,.8,.3,1)'}); }
// Damage numbers from a fixed pool of 40 DOM nodes (was createElement + appendChild + a timed remove per hit). A reused
// node restarts with its transition off, then the fade is re-enabled next frame, so every number starts from its start state.
const _dnPool=[]; let _dnNext=0;
function spawnDamageNumber(worldPos,val,head,kill){
  _p1.copy(worldPos); _p1.y+=0.3; _p1.project(camera);
  if(_p1.z>1) return;
  const x=(_p1.x*0.5+0.5)*innerWidth, y=(-_p1.y*0.5+0.5)*innerHeight;
  if(!_dnPool.length) for(let i=0;i<40;i++){ const e=document.createElement('div'); e.style.opacity='0'; _dnPool.push({el:e,dx:0}); }
  const P=_dnPool[_dnNext]; _dnNext=(_dnNext+1)%_dnPool.length; const d=P.el;
  if(d.parentNode!==dmgNumbersEl) for(let i=0;i<_dnPool.length;i++) dmgNumbersEl.appendChild(_dnPool[i].el);   // (re)attach if the layer was emptied
  d.className='dn'+(head?' head':'')+(kill&&!head?' kill':''); d.textContent=head?('\u2738 '+val):(''+val);
  d.style.transition='none'; d.style.transform=''; d.style.left=x+'px'; d.style.top=y+'px'; d.style.opacity='1';
  P.dx=(Math.random()*2-1)*16;
  requestAnimationFrame(()=>{ d.style.transition=''; d.style.transform='translate(calc(-50% + '+P.dx+'px),-180%)'; d.style.opacity='0'; });
}
let _lastGap=-1;
function updateReticle(){
  const gap=5+ps.spread*150+camRecoil*7;
  if(Math.abs(gap-_lastGap)>0.25){ _lastGap=gap; const g=gap+4;
    _rUp.style.transform='translate(-50%,-50%) translateY('+(-g)+'px)';
    _rDown.style.transform='translate(-50%,-50%) translateY('+g+'px)';
    _rLeft.style.transform='translate(-50%,-50%) translateX('+(-g)+'px)';
    _rRight.style.transform='translate(-50%,-50%) translateX('+g+'px)';
  }
  reticleEl.classList.toggle('hostile',aimTarget);
  reticleEl.classList.toggle('blocked',aimBlocked&&!aimTarget);
  reticleEl.classList.toggle('head',aimHoverEnemy&&aimPart==='head');
}
const termPrompt=document.getElementById('terminalPrompt'),termText=document.getElementById('termText');
function resolveObstacles(pos,r){for(let i=0;i<obstacles.length;i++){const o=obstacles[i],b=o.box,px=pos.x,pz=pos.z,th=o.mesh.rotation.y;if(th>-1e-3&&th<1e-3){if(px>b.min.x-r&&px<b.max.x+r&&pz>b.min.z-r&&pz<b.max.z+r){const dl=Math.abs(px-(b.min.x-r)),dr=Math.abs((b.max.x+r)-px),db=Math.abs(pz-(b.min.z-r)),df=Math.abs((b.max.z+r)-pz);const m=Math.min(dl,dr,db,df);if(m===dl)pos.x=b.min.x-r;else if(m===dr)pos.x=b.max.x+r;else if(m===db)pos.z=b.min.z-r;else pos.z=b.max.z+r;}}else{const cx=o.mesh.position.x,cz=o.mesh.position.z,hw=o.w*0.5,hd=o.d*0.5,c=Math.cos(th),sn=Math.sin(th),dx=px-cx,dz=pz-cz;let lx=c*dx-sn*dz,lz=sn*dx+c*dz;if(lx>-hw-r&&lx<hw+r&&lz>-hd-r&&lz<hd+r){const dl=lx+hw+r,dr=hw+r-lx,db=lz+hd+r,df=hd+r-lz,m=Math.min(dl,dr,db,df);if(m===dl)lx=-hw-r;else if(m===dr)lx=hw+r;else if(m===db)lz=-hd-r;else lz=hd+r;pos.x=cx+(c*lx+sn*lz);pos.z=cz+(-sn*lx+c*lz);}}}for(let i=0;i<houses.length;i++){const h=houses[i],th=h.grp?h.grp.rotation.y:0;if(th>-1e-3&&th<1e-3){const bs=h.boxes||[h.box];for(let k=0;k<bs.length;k++){const b=bs[k],px=pos.x,pz=pos.z;if(px>b.min.x-r&&px<b.max.x+r&&pz>b.min.z-r&&pz<b.max.z+r){const dl=Math.abs(px-(b.min.x-r)),dr=Math.abs((b.max.x+r)-px),db=Math.abs(pz-(b.min.z-r)),df=Math.abs((b.max.z+r)-pz);const m=Math.min(dl,dr,db,df);if(m===dl)pos.x=b.min.x-r;else if(m===dr)pos.x=b.max.x+r;else if(m===db)pos.z=b.min.z-r;else pos.z=b.max.z+r;}}}else{let L=h.localColliders;if(!L){if(!h._lc){let bb=(h.body&&h.body.geometry)?h.body.geometry.boundingBox:null;if(h.body&&h.body.geometry&&!bb){h.body.geometry.computeBoundingBox();bb=h.body.geometry.boundingBox;}h._lc=bb?[{minx:bb.min.x+h.body.position.x,minz:bb.min.z+h.body.position.z,maxx:bb.max.x+h.body.position.x,maxz:bb.max.z+h.body.position.z}]:[];}L=h._lc;}const wx=h.grp.position.x,wz=h.grp.position.z,c=Math.cos(th),sn=Math.sin(th);for(let k=0;k<L.length;k++){const cc=L[k],dx=pos.x-wx,dz=pos.z-wz;let lx=c*dx-sn*dz,lz=sn*dx+c*dz;if(lx>cc.minx-r&&lx<cc.maxx+r&&lz>cc.minz-r&&lz<cc.maxz+r){const dl=lx-(cc.minx-r),dr=(cc.maxx+r)-lx,db=lz-(cc.minz-r),df=(cc.maxz+r)-lz,m=Math.min(dl,dr,db,df);if(m===dl)lx=cc.minx-r;else if(m===dr)lx=cc.maxx+r;else if(m===db)lz=cc.minz-r;else lz=cc.maxz+r;pos.x=wx+(c*lx+sn*lz);pos.z=wz+(-sn*lx+c*lz);}}}}for(let i=0;i<covers.length;i++){const bs=covers[i].boxes;if(!bs)continue;for(let k=0;k<bs.length;k++){const b=bs[k];if(pos.y>b.max.y-0.2)continue;const px=pos.x,pz=pos.z;if(px>b.min.x-r&&px<b.max.x+r&&pz>b.min.z-r&&pz<b.max.z+r){const dl=Math.abs(px-(b.min.x-r)),dr=Math.abs((b.max.x+r)-px),db=Math.abs(pz-(b.min.z-r)),df=Math.abs((b.max.z+r)-pz);const m=Math.min(dl,dr,db,df);if(m===dl)pos.x=b.min.x-r;else if(m===dr)pos.x=b.max.x+r;else if(m===db)pos.z=b.min.z-r;else pos.z=b.max.z+r;}}}for(let i=0;i<trees.length;i++){const b=trees[i].box;if(!b)continue;const px=pos.x,pz=pos.z;if(px>b.min.x-r&&px<b.max.x+r&&pz>b.min.z-r&&pz<b.max.z+r){const dl=Math.abs(px-(b.min.x-r)),dr=Math.abs((b.max.x+r)-px),db=Math.abs(pz-(b.min.z-r)),df=Math.abs((b.max.z+r)-pz);const m=Math.min(dl,dr,db,df);if(m===dl)pos.x=b.min.x-r;else if(m===dr)pos.x=b.max.x+r;else if(m===db)pos.z=b.min.z-r;else pos.z=b.max.z+r;}}for(let i=0;i<props.length;i++){const b=props[i].box;if(!b)continue;const px=pos.x,pz=pos.z;if(px>b.min.x-r&&px<b.max.x+r&&pz>b.min.z-r&&pz<b.max.z+r){const dl=Math.abs(px-(b.min.x-r)),dr=Math.abs((b.max.x+r)-px),db=Math.abs(pz-(b.min.z-r)),df=Math.abs((b.max.z+r)-pz);const m=Math.min(dl,dr,db,df);if(m===dl)pos.x=b.min.x-r;else if(m===dr)pos.x=b.max.x+r;else if(m===db)pos.z=b.min.z-r;else pos.z=b.max.z+r;}}}
// ---- Phase 5: vertical surface query + conditional cliff collision + gravity/jump integration ----
function surfaceHeightAt(x,z,withCover){ let h=0; for(let i=0;i<platforms.length;i++){ const b=platforms[i].box; if(x>=b.min.x&&x<=b.max.x&&z>=b.min.z&&z<=b.max.z&&b.max.y>h)h=b.max.y; } for(let i=0;i<ramps.length;i++){ const rh=rampHeightAt(ramps[i],x,z); if(rh!==null&&rh>h)h=rh; } for(let i=0;i<buildings.length;i++){ const b=buildings[i]; if(!b.alive)continue; const bx=b.box; if(x>=bx.min.x&&x<=bx.max.x&&z>=bx.min.z&&z<=bx.max.z&&bx.max.y>h)h=bx.max.y; } if(withCover){ for(let i=0;i<covers.length;i++){ const bs=covers[i].boxes; if(!bs)continue; for(let k=0;k<bs.length;k++){ const cb=bs[k]; if(x>=cb.min.x&&x<=cb.max.x&&z>=cb.min.z&&z<=cb.max.z&&cb.max.y>h)h=cb.max.y; } } } return h; }
// Shared vertical solver for any grounded agent (player + enemies). st carries {vy,grounded,px,pz}.
// Step-up guard blocks moving onto a surface >stepUp above the feet (cliffs, ramp sides) with a
// radius-ahead sample + per-axis slide; then gravity, optional jump, and surface landing. Zero allocation.
function tickVertical(pos, st, dt, jumpHeld, r, standOnCover){
  if(st.px!==undefined){ const foot=pos.y, up=foot+T.stepUp; let mvx=pos.x-st.px, mvz=pos.z-st.pz; const ml=hyp2(mvx,mvz);
    if(ml>1e-6 && ml<4){ const ux=mvx/ml, uz=mvz/ml;   // ml<4 skips teleports (respawns) so the guard never yanks an agent back to a stale spot
      if(surfaceHeightAt(pos.x+ux*r, pos.z+uz*r) > up){ const okX=surfaceHeightAt(pos.x+(mvx>=0?r:-r), st.pz)<=up, okZ=surfaceHeightAt(st.px, pos.z+(mvz>=0?r:-r))<=up;
        if(okX && Math.abs(mvx)>=Math.abs(mvz)) pos.z=st.pz; else if(okZ) pos.x=st.px; else if(okX) pos.z=st.pz; else { pos.x=st.px; pos.z=st.pz; } } } }
  if(jumpHeld && st.grounded){ st.vy=T.jumpV; st.grounded=false; }
  st.vy=(st.vy||0)-T.gravity*dt; pos.y+=st.vy*dt;
  const surf=surfaceHeightAt(pos.x, pos.z, standOnCover);
  if(pos.y<=surf){ pos.y=surf; st.vy=0; st.grounded=true; } else st.grounded=false;
  st.px=pos.x; st.pz=pos.z;
}
function blastReaches(from,to,ignore){const dir=_v.copy(to).sub(from);const dist=dir.length();if(dist<0.001)return true;dir.normalize();losRay.set(from,dir);losRay.far=dist-0.3;const hits=losRay.intersectObjects(losMeshes,false);for(const h of hits){if(h.object!==ignore)return false;}return true;}

/* loop */
const clock=new THREE.Clock(); let camLookAt=new THREE.Vector3(0,0,20); const respawnCountEl=document.getElementById('respawnCount');
const camBase=new THREE.Vector3(0,T.camHeight,T.camBack); const camKick=new THREE.Vector3(), camKickVel=new THREE.Vector3(); let fovBase=T.fov, fovKick=0;
const _panCam=new THREE.PerspectiveCamera(40,1,1,2000); // temp cam: projects the player to NDC so the Foxhole distance-pan never pushes them off-screen
const _ov=new THREE.Vector3(), _od=new THREE.Vector3(), _ohit=new THREE.Vector3(); // building-occlusion ray temps
// ============ UNIFIED OCCLUSION (Foxhole-style cutaway) ============
// One global effect: any structure whose AABB sits between the iso camera and the player is faded (fill AND its
// inverted-hull ink outline) so the player is never hidden behind geometry. Driven purely by occlusion of ~5 points
// across the player's silhouette, so it needs no per-building setup and treats buildings/barracks/platforms/ramps/
// crates identically. The surface the player STANDS on is excluded via a distance margin, so roofs you walk on stay solid.
const OCC_FADE=0.24;                                   // ghost opacity of an occluding structure
const _occHit=new Set(), _occRec=new Map(), _occBox=new THREE.Box3();
const _occSX=[0,0,0,0,0], _occSY=[1.1,1.95,0.35,1.1,1.1], _occSZ=[0,0,0,0,0];  // silhouette samples: torso, head, feet, L, R
let _occCx=0,_occCy=0,_occCz=0,_occPx=0,_occPy=0,_occPz=0,_occK=0.2;
function _occTest(box){ if(!box)return false;          // true if a camera->silhouette ray crosses box at least 0.6m before the player
  // Inline slab test, no calls in this hot loop (52+ calls/frame x 5 samples): calling rayAabb here boxed its float
  // arguments on every call (measured ~0.9 MB/s of garbage). Same result: an entry point in [0, dl-0.6).
  const mnx=box.min.x, mny=box.min.y, mnz=box.min.z, mxx=box.max.x, mxy=box.max.y, mxz=box.max.z;
  for(let i=0;i<5;i++){ const dx=(_occPx+_occSX[i])-_occCx, dy=(_occPy+_occSY[i])-_occCy, dz=(_occPz+_occSZ[i])-_occCz;
    const dl=Math.sqrt(dx*dx+dy*dy+dz*dz); if(dl<1e-3)continue; const inv=1/dl, ux=dx*inv, uy=dy*inv, uz=dz*inv;
    let t0=0, t1=dl-0.6; if(t1<=0) continue;
    if(ux>-1e-9&&ux<1e-9){ if(_occCx<mnx||_occCx>mxx) continue; } else { let a=(mnx-_occCx)/ux, b=(mxx-_occCx)/ux; if(a>b){ const q=a; a=b; b=q; } if(a>t0)t0=a; if(b<t1)t1=b; if(t0>t1) continue; }
    if(uy>-1e-9&&uy<1e-9){ if(_occCy<mny||_occCy>mxy) continue; } else { let a=(mny-_occCy)/uy, b=(mxy-_occCy)/uy; if(a>b){ const q=a; a=b; b=q; } if(a>t0)t0=a; if(b<t1)t1=b; if(t0>t1) continue; }
    if(uz>-1e-9&&uz<1e-9){ if(_occCz<mnz||_occCz>mxz) continue; } else { let a=(mnz-_occCz)/uz, b=(mxz-_occCz)/uz; if(a>b){ const q=a; a=b; b=q; } if(a>t0)t0=a; if(b<t1)t1=b; if(t0>t1) continue; }
    return true; }
  return false; }
function _occRecFor(mesh, extras){                     // build+cache a fade record ONCE per mesh (fill material, extra materials, all ink-outline children)
  let rec=_occRec.get(mesh); if(rec) return rec;
  const outlines=[]; const ch=mesh.children; for(let i=0;i<ch.length;i++){ if(ch[i].userData&&ch[i].userData.isOutline){ outlines.push(ch[i]); break; } }
  if(mesh.material) mesh.material.transparent=true;
  const exMats=[]; if(extras){ for(let i=0;i<extras.length;i++){ const e=extras[i]; if(e&&e.material){ e.material.transparent=true; exMats.push(e.material); const ec=e.children; for(let j=0;j<ec.length;j++){ if(ec[j].userData&&ec[j].userData.isOutline){ outlines.push(ec[j]); break; } } } } }
  rec={fill:mesh.material, extras:exMats, outlines:outlines, op:1}; _occRec.set(mesh, rec); return rec; }
function _occApply(rec, mesh){                          // lerp one record toward its target opacity; fully restore + release when clear
  const tgt=_occHit.has(mesh)?OCC_FADE:1; rec.op+=(tgt-rec.op)*_occK; const dw=rec.op>0.95, ov=rec.op>0.9;
  if(rec.fill){ rec.fill.opacity=rec.op; rec.fill.depthWrite=dw; }
  for(let i=0;i<rec.extras.length;i++){ rec.extras[i].opacity=rec.op; rec.extras[i].depthWrite=dw; }
  for(let i=0;i<rec.outlines.length;i++) rec.outlines[i].visible=ov;
  if(tgt===1 && rec.op>0.985){ if(rec.fill){ rec.fill.opacity=1; rec.fill.depthWrite=true; rec.fill.transparent=false; } for(let i=0;i<rec.extras.length;i++){ rec.extras[i].opacity=1; rec.extras[i].depthWrite=true; rec.extras[i].transparent=false; } for(let i=0;i<rec.outlines.length;i++) rec.outlines[i].visible=true; _occRec.delete(mesh); } }
function updateOcclusion(dt){ if(mode!=='game') return;
  const camp=camera.position; _occCx=camp.x; _occCy=camp.y; _occCz=camp.z; _occPx=player.position.x; _occPy=player.position.y; _occPz=player.position.z;
  const cdx=_occPx-_occCx, cdz=_occPz-_occCz, cl=hyp2(cdx,cdz)||1, perpX=-cdz/cl, perpZ=cdx/cl;
  _occSX[3]=perpX*0.55; _occSZ[3]=perpZ*0.55; _occSX[4]=-perpX*0.55; _occSZ[4]=-perpZ*0.55;   // L/R samples track the camera azimuth
  _occHit.clear();
  for(let i=0;i<obstacles.length;i++){ const o=obstacles[i]; if(_occTest(o.box)){ if(!_occRec.has(o.mesh))_occRecFor(o.mesh,null); _occHit.add(o.mesh); } }
  for(let i=0;i<buildings.length;i++){ const b=buildings[i]; if(b.alive && _occTest(b.box)){ if(!_occRec.has(b.mesh))_occRecFor(b.mesh, b.trim?[b.trim]:null); _occHit.add(b.mesh); } }
  for(let i=0;i<platforms.length;i++){ const p=platforms[i]; if(_occTest(p.box)){ if(!_occRec.has(p.mesh))_occRecFor(p.mesh,null); _occHit.add(p.mesh); } }
  for(let i=0;i<ramps.length;i++){ const r=ramps[i]; if(_occTest(r.box)){ if(!_occRec.has(r.mesh))_occRecFor(r.mesh,null); _occHit.add(r.mesh); } }
  for(let i=0;i<barracksList.length;i++){ const bk=barracksList[i];
    for(let w=0;w<bk.walls.length;w++){ const wl=bk.walls[w]; if(_occTest(wl.box)){ if(!_occRec.has(wl.mesh))_occRecFor(wl.mesh,null); _occHit.add(wl.mesh); } }
    if(bk.roof){ _occBox.setFromObject(bk.roof);
      // roof ALWAYS fades while the player is UNDER it (inside footprint AND below the slab); stays solid when standing ON it (multi-floor-ready).
      const underRoof = _occPx>_occBox.min.x && _occPx<_occBox.max.x && _occPz>_occBox.min.z && _occPz<_occBox.max.z && _occPy<_occBox.min.y-0.3;
      if(underRoof || _occTest(_occBox)){ if(!_occRec.has(bk.roof))_occRecFor(bk.roof,null); _occHit.add(bk.roof); } }
    if(bk.lintel){ _occBox.setFromObject(bk.lintel); if(_occTest(_occBox)){ if(!_occRec.has(bk.lintel))_occRecFor(bk.lintel,null); _occHit.add(bk.lintel); } }
  }
  _occK=1-Math.exp(-14*dt); _occRec.forEach(_occApply);
}
function occRestoreAll(){ _occRec.forEach((rec,mesh)=>{ if(rec.fill){ rec.fill.opacity=1; rec.fill.depthWrite=true; rec.fill.transparent=false; } for(let i=0;i<rec.extras.length;i++){ rec.extras[i].opacity=1; rec.extras[i].depthWrite=true; rec.extras[i].transparent=false; } for(let i=0;i<rec.outlines.length;i++) rec.outlines[i].visible=true; }); _occRec.clear(); }
const WORLD_FWD=new THREE.Vector3(0,0,-1), WORLD_RIGHT=new THREE.Vector3(1,0,0); // movement axes (camera never yaws)
// DOM writes only when the value changes (an unchanged write still dirties style; a text write replaces the node).
// Compared against the element's CURRENT value, so a write made elsewhere can never desync them.
function _sd(el,v){ if(el&&el.style.display!==v) el.style.display=v; }
function _so(el,v){ if(!el) return; const s=''+v; if(el.style.opacity!==s) el.style.opacity=s; }
function _st(el,v){ if(!el) return; const s=''+v; if(el.textContent!==s) el.textContent=s; }
// ============================ NAV — bot pathfinding ============================
// A 2 m grid over the arena, built at deploy with the SAME movement collision the bots use (resolveObstacles). A cell is
// blocked when any part of its square is solid for a bot: its centre is tested at the bot radius plus half a cell (exact for
// axis-aligned colliders, whose collision is square). Testing the centre at the bot radius alone left up to half a metre of
// tree inside "free" cells, and bots routed through it pinned themselves on the trunk (bot-poplar-wedge). A* (8-neighbour, octile, no corner
// cutting) on preallocated typed arrays with a search stamp: zero allocation per search. Paths are string-pulled with grid
// line-of-sight. Bots walk straight whenever the way is clear (open-field behaviour unchanged) and follow waypoints only
// when it isn't; stuck detection forces a new route. At most NAV.budget searches per frame across all bots.
const NAV={ ok:false, cs:2, nx:0, nz:0, x0:0, z0:0, blk:null, g:null, f:null, came:null, stamp:null, closed:null, heap:null, tmp:null, hn:0, sid:0, budget:0 };
const _navP=new THREE.Vector3(), _navDir=new THREE.Vector3(), _NDX=[1,-1,0,0,1,1,-1,-1], _NDZ=[0,0,1,-1,1,-1,1,-1];
function navBuild(){ const cs=NAV.cs, lx=T.arenaX-1, lz=T.arenaZ-1, nx=Math.ceil(2*lx/cs), nz=Math.ceil(2*lz/cs), n=nx*nz;
  NAV.nx=nx; NAV.nz=nz; NAV.x0=-lx; NAV.z0=-lz;
  if(!NAV.blk||NAV.blk.length!==n){ NAV.blk=new Uint8Array(n); NAV.g=new Float32Array(n); NAV.f=new Float32Array(n); NAV.came=new Int32Array(n);
    NAV.stamp=new Uint32Array(n); NAV.closed=new Uint32Array(n); NAV.heap=new Int32Array(n*2); NAV.tmp=new Int32Array(n); }
  const r=0.7+cs*0.5;   // 0.7 = the bots' collision radius (update(): resolveObstacles(en.group.position,0.7)), + half a cell
  for(let iz=0;iz<nz;iz++) for(let ix=0;ix<nx;ix++){ const x=NAV.x0+(ix+0.5)*cs, z=NAV.z0+(iz+0.5)*cs; _navP.set(x,0,z); resolveObstacles(_navP,r);
    NAV.blk[iz*nx+ix]=(Math.abs(_navP.x-x)>1e-4||Math.abs(_navP.z-z)>1e-4)?1:0; }
  NAV.ok=true; }
function _navCell(x,z){ const ix=Math.floor((x-NAV.x0)/NAV.cs), iz=Math.floor((z-NAV.z0)/NAV.cs); return (ix<0||iz<0||ix>=NAV.nx||iz>=NAV.nz)?-1:iz*NAV.nx+ix; }
function navClear(x0,z0,x1,z1){ const dx=x1-x0, dz=z1-z0, steps=Math.ceil(Math.sqrt(dx*dx+dz*dz)/(NAV.cs*0.5));
  for(let k=1;k<=steps;k++){ const c=_navCell(x0+dx*k/steps, z0+dz*k/steps); if(c<0||NAV.blk[c]) return false; } return true; }
function _navPush(c){ const H=NAV.heap, F=NAV.f; let i=NAV.hn++; while(i>0){ const q=(i-1)>>1; if(F[H[q]]<=F[c]) break; H[i]=H[q]; i=q; } H[i]=c; }
function _navPop(){ const H=NAV.heap, F=NAV.f, top=H[0], last=H[--NAV.hn], n=NAV.hn; let i=0;
  if(n>0){ for(;;){ let l=2*i+1; if(l>=n) break; if(l+1<n&&F[H[l+1]]<F[H[l]]) l++; if(F[H[l]]>=F[last]) break; H[i]=H[l]; i=l; } H[i]=last; } return top; }
function _navNearestFree(t){ const nx=NAV.nx, nz=NAV.nz, tx=t%nx, tz=(t/nx)|0;
  for(let r=1;r<=10;r++) for(let dz=-r;dz<=r;dz++) for(let dx=-r;dx<=r;dx++){ if(Math.abs(dx)!==r&&Math.abs(dz)!==r) continue; const x=tx+dx, z=tz+dz;
    if(x>=0&&z>=0&&x<nx&&z<nz&&!NAV.blk[z*nx+x]) return z*nx+x; } return -1; }
// A* from (sx,sz) to (gx,gz): writes string-pulled waypoints (x,z pairs) into out; returns their count (0 = no route)
function navPath(sx,sz,gx,gz,out){ const nx=NAV.nx, blk=NAV.blk, G=NAV.g, F=NAV.f, C=NAV.came, ST=NAV.stamp, CL=NAV.closed, cs=NAV.cs;
  const s0=_navCell(sx,sz); let s=s0, t=_navCell(gx,gz); if(s<0||t<0) return 0; if(blk[t]){ t=_navNearestFree(t); if(t<0) return 0; }
  { const x=s%nx, z=(s/nx)|0;   // a bot pressed against a wall can stand in a blocked cell, or a boxed-in one: start from the nearest open cell
    if(blk[s]||!((x>0&&!blk[s-1])||(x<nx-1&&!blk[s+1])||(z>0&&!blk[s-nx])||(z<NAV.nz-1&&!blk[s+nx]))){ const f=_navNearestFree(s); if(f>=0) s=f; } }
  const sid=++NAV.sid, tx=t%nx, tz=(t/nx)|0; NAV.hn=0; ST[s]=sid; G[s]=0; C[s]=-1;
  { const dx=Math.abs(s%nx-tx), dz=Math.abs(((s/nx)|0)-tz); F[s]=dx+dz-0.5858*Math.min(dx,dz); } _navPush(s);
  let found=false, it=0;
  while(NAV.hn>0&&it++<8000){ const c=_navPop(); if(CL[c]===sid) continue; CL[c]=sid; if(c===t){ found=true; break; }
    const cx=c%nx, cz=(c/nx)|0;
    for(let k=0;k<8;k++){ const ox=_NDX[k], oz=_NDZ[k], ex=cx+ox, ez=cz+oz; if(ex<0||ez<0||ex>=nx||ez>=NAV.nz) continue; const e=ez*nx+ex;
      if(blk[e]||CL[e]===sid) continue; if(ox!==0&&oz!==0&&(blk[cz*nx+ex]||blk[ez*nx+cx])) continue;
      const ng=G[c]+((ox!==0&&oz!==0)?1.4142:1);
      if(ST[e]!==sid||ng<G[e]){ ST[e]=sid; G[e]=ng; C[e]=c; const dx=Math.abs(ex-tx), dz=Math.abs(ez-tz); F[e]=ng+dx+dz-0.5858*Math.min(dx,dz); if(NAV.hn<NAV.heap.length) _navPush(e); } } }
  if(!found) return 0;
  let L=0; for(let c=t;c!==-1&&L<NAV.tmp.length;c=C[c]) NAV.tmp[L++]=c;              // reversed: tmp[L-1] = start
  let n=0, ax=sx, az=sz, i=L-1;
  if(s!==s0){ ax=NAV.x0+(s%nx+0.5)*cs; az=NAV.z0+(((s/nx)|0)+0.5)*cs; out[0]=ax; out[1]=az; n=1; }   // first get to the open start cell
  while(i>0&&n<(out.length>>1)){ let j=i-1; for(let k=i-1;k>=Math.max(0,i-24);k--){ const c=NAV.tmp[k]; if(navClear(ax,az,NAV.x0+(c%nx+0.5)*cs,NAV.z0+(((c/nx)|0)+0.5)*cs)) j=k; else break; }
    const c=NAV.tmp[j]; ax=NAV.x0+(c%nx+0.5)*cs; az=NAV.z0+(((c/nx)|0)+0.5)*cs; out[n*2]=ax; out[n*2+1]=az; n++; i=j; }
  return n; }
// Unit direction for a bot walking to (gx,gz): straight when clear, else along its cached route.
function navSteer(en,gx,gz,out,dt){ const p=en.group.position, dx=gx-p.x, dz=gz-p.z, d=Math.sqrt(dx*dx+dz*dz);
  if(d<1e-4) return out.set(0,0,0);
  if(!NAV.ok||navClear(p.x,p.z,gx,gz)){ if(en._nav) en._nav.n=0; return out.set(dx/d,0,dz/d); }
  let N=en._nav; if(!N) N=en._nav={ pts:new Float32Array(128), n:0, i:0, gx:0, gz:0, t:0, chk:0, lx:p.x, lz:p.z, stuck:false };
  N.t+=dt; N.chk+=dt;
  if(N.chk>0.8){ const mx=p.x-N.lx, mz=p.z-N.lz; N.stuck=(mx*mx+mz*mz<0.12); N.lx=p.x; N.lz=p.z; N.chk=0; }
  const gm=(N.gx-gx)*(N.gx-gx)+(N.gz-gz)*(N.gz-gz);
  if((N.n===0||gm>9||N.t>1.5||N.stuck)&&NAV.budget>0){ NAV.budget--; N.n=navPath(p.x,p.z,gx,gz,N.pts); N.i=0; N.t=0; N.gx=gx; N.gz=gz; N.stuck=false; }
  if(N.n===0) return out.set(dx/d,0,dz/d);                                   // no route (yet): the old straight line
  for(let k=0;k<2&&N.i<N.n-1&&navClear(p.x,p.z,N.pts[(N.i+1)*2],N.pts[(N.i+1)*2+1]);k++) N.i++;   // skip ahead when visible
  let ex=N.pts[N.i*2]-p.x, ez=N.pts[N.i*2+1]-p.z, el=Math.sqrt(ex*ex+ez*ez);
  if(el<1.0&&N.i<N.n-1){ N.i++; ex=N.pts[N.i*2]-p.x; ez=N.pts[N.i*2+1]-p.z; el=Math.sqrt(ex*ex+ez*ez); }
  return el>1e-4?out.set(ex/el,0,ez/el):out.set(dx/d,0,dz/d); }
function update(dt){ NAV.budget=2;
  _frameId++;   // a new sim step: syncSimWorld re-syncs once per update(), so headless runs (tools/sim.js) see live positions
  const w=curW(),l=curL();
  const _animClock=performance.now()*0.001;
  if(meleeCd>0)meleeCd-=dt;
  if(ps.swapCd>0)ps.swapCd=Math.max(0,ps.swapCd-dt);
  if(ps.banzaiCd>0)ps.banzaiCd=Math.max(0,ps.banzaiCd-dt);
  if(ps.banzai){ps.banzaiT-=dt; if(ps.banzaiT<=0)endBanzai();}
  if(ps.bayonetCd>0)ps.bayonetCd=Math.max(0,ps.bayonetCd-dt);
  if(ps.bayonet){ps.bayonetT-=dt; if(ps.bayonetT<=0)endBayonet();}
  updateAbilityHUD();

  if(ps.dead){ps.deadT-=dt;ps.toppleT=Math.min(0.5,ps.toppleT+dt);const _tt=ps.toppleT/0.5;animDeath(player,dt);_st(respawnCountEl,Math.ceil(ps.deadT));if(ps.deadT<=0)playerRespawn();}
  if(ps.reloading){
    ps.reloadT+=dt; const p=Math.min(1,ps.reloadT/w.reload);
    reloadFg.style.strokeDashoffset=RR_C*(1-p);
    _sd(reloadRing,'block'); _sd(reloadLabel,'block'); ammoEl.classList.add('reloading');
    if(p>=1){const need=w.mag-l.ammo;if(w.special){const take=Math.min(need,l.reserve);l.ammo+=take;l.reserve-=take;}else{l.ammo+=need;}ps.reloading=false;sfxReload(1);_sd(reloadRing,'none');_sd(reloadLabel,'none');ammoEl.classList.remove('reloading');showReadyFlash();updateWeaponHUD();}
  } else if(reloadRing.style.display!=='none'){ _sd(reloadRing,'none'); _sd(reloadLabel,'none'); ammoEl.classList.remove('reloading'); }
  if(ps.fireCd>0)ps.fireCd-=dt;

  if(!gameOver){
    const fRef=TEAM_INDEX[PLAYER_TEAM].refinery;
    credits+=((fRef&&fRef.alive?ECO.payTrickle:ECO.payTrickleDead)+ECO.paySilo*teamEco[PLAYER_TEAM].silos)*dt; updateCreditHUD();
    updateHarvesters(dt);   // both teams' tech now comes from their harvester docks (+ combat slice)
  }
  updateTechBox(dt);
  updateSilos(dt);
  ps.lastHitT+=dt;
  if(!ps.dead&&ps.lastHitT>T.regenDelay&&ps.health<ps.maxHealth){ps.health=Math.min(ps.maxHealth,ps.health+T.regenRate*dt);updateHealthHUD();}

  // aim (3D) + face + barrel pitch
  // Cast aim through the STABLE camera base (camBase/camLookAt) — exclude shake,
  // recoil, and concussion-kick so cosmetic screen motion never drags the
  // crosshair's world aim. The FX are re-applied to `camera` below for rendering.
  camera.position.copy(camBase); camera.lookAt(camLookAt); camera.updateMatrixWorld();
  updateAim(); updateAimVisuals();
  if(!ps.dead&&!pilotJug){
    const fd=_v.set(aimPoint.x-player.position.x,0,aimPoint.z-player.position.z);
    if(fd.lengthSq()>0.0004) player.rotation.y=Math.atan2(fd.x,fd.z);
    pitchWeapon(player,aimPoint);
    // reload / weapon-swap micro-animation (additive tilt on the held gun; recomputed each frame so it never accumulates)
    { const wm=player.userData.wpn;
      ps.fireKick=Math.max(0, ps.fireKick - dt*5);
      if(wm && !curW().melee){
        if(ps.wpnSwap>0) ps.wpnSwap=Math.max(0,ps.wpnSwap-dt/0.26);
        ps.wpnReloadDip += ((ps.reloading?1:0)-ps.wpnReloadDip)*Math.min(1,dt*9);
      } }  // gun pose (recoil/reload/swap tilt + carry<->ADS) now owned by updateSoldierAnim's aim layer
  }
  const baseSpread=ps.ads?w.spreadADS:w.spreadHip; ps.spread=Math.max(baseSpread,ps.spread-w.spreadDecay*dt);

  if(!ps.dead&&!gameOver&&!pilotJug){
    if(ps.bayonet){ semiQueued=false; }
    else if(w.burst){
      if(ps.burstLeft>0){ if(ps.fireCd<=0){ fireOnce(); ps.burstLeft--; if(ps.burstLeft<=0) ps.fireCd=Math.max(ps.fireCd,w.fireInterval); } }
      else if(semiQueued){ semiQueued=false; if(ps.fireCd<=0) ps.burstLeft=w.burst; }
    }
    else if(w.auto){ if(mouseDown)fireOnce(); }
    else { if(semiQueued){ fireOnce(); semiQueued=false; } }
  }
  tickCharge(dt);

  // kill-chain bonus window
  if(chainTimer>0){ chainTimer=Math.max(0,chainTimer-dt); if(chainTimer<=0) chainCount=0; }
  // dev: infinite ammo
  if(DEV.infAmmo){ const l=curL(),w=curW(); if(!w.melee){ l.ammo=w.mag; if(l.reserve<900)l.reserve=w.reserve; ps.reloading=false; } grenadeCount=T.grenadeMax; }

  // melee swing animation + impact-frame hit registration
  if(ps.swinging&&ps.swingMesh){
    ps.swingT+=dt; const p=Math.min(1,ps.swingT/ps.swingDur); const pose=swingPose(p);
    ps.swingMesh.rotation.set(pose.x,pose.y,0);
    if(!ps.swingHit&&ps.swingT>=ps.swingImpact){ ps.swingHit=true; meleeHitCheck(ps.swingW); }
    if(ps.swingT>=ps.swingDur){ ps.swinging=false; ps.swingMesh.rotation.set(MELEE_IDLE.x,MELEE_IDLE.y,0); }
  }
  // shield block flash decay (brightens the boss on a frontal block)
  if(ps.shieldFlash>0){ ps.shieldFlash=Math.max(0,ps.shieldFlash-dt*4); const u=player.userData; if(u.shield&&u.shield.userData.boss) u.shield.userData.boss.emissiveIntensity=ps.shieldFlash*0.9; }

  // camera yaw (smoothed toward target along the short path) + ground axes — shared by movement & framing
  let dyaw=camYawTarget-camYaw; while(dyaw>Math.PI)dyaw-=6.283185307; while(dyaw<-Math.PI)dyaw+=6.283185307;
  camYaw+=dyaw*(1-Math.exp(-T.camYawLerp*dt));
  const _cyaw=camYaw+T.camYawBase; const cosY=Math.cos(_cyaw), sinY=Math.sin(_cyaw);
  const fwdX=-sinY, fwdZ=-cosY, rgtX=cosY, rgtZ=-sinY;
  // MOVEMENT — camera-relative axes (Foxhole-style): W moves away from the camera, regardless of yaw
  if(!ps.dead&&!gameOver&&!pilotJug){
    if(ps.bayonet){
      const cs=T.moveSpeed*unitOf(2).sprintMult*1.3;
      const bd=_wish.set(aimPoint.x-player.position.x,0,aimPoint.z-player.position.z);
      if(bd.lengthSq()>1e-4){ bd.normalize(); ps.vel.copy(bd).multiplyScalar(cs); player.rotation.y=Math.atan2(bd.x,bd.z); }
      else ps.vel.multiplyScalar(0.5);
      player.position.addScaledVector(ps.vel,dt);
      if(ps._bayHit) enemies.forEach(en=>{ if(en.alive && !ps._bayHit.has(en) && en.group.position.distanceTo(player.position)<2.4){ ps._bayHit.add(en); damageEnemy(en,9999,'body',en.group.position.clone().setY(1.5),true); camShake+=0.16; if(typeof spawnComic==='function')spawnComic('IMPALE!',en.group.position.clone().setY(1.9),2.4,'#FF7B00'); } });
    } else {
    const wish=_wish.set(0,0,0);
    if(keys['KeyW']){wish.x+=fwdX;wish.z+=fwdZ;}
    if(keys['KeyS']){wish.x-=fwdX;wish.z-=fwdZ;}
    if(keys['KeyD']){wish.x+=rgtX;wish.z+=rgtZ;}
    if(keys['KeyA']){wish.x-=rgtX;wish.z-=rgtZ;}
    const moving=wish.lengthSq()>0;
    if(moving)wish.normalize();
    const wantSprint=(keys['ShiftLeft']||keys['ShiftRight'])&&moving&&!ps.ads&&!ps.shieldOn;
    if(wantSprint&&!ps.staminaLock){ ps.stamina=Math.max(0,ps.stamina-dt/STAM.max); if(ps.stamina<=0){ps.stamina=0;ps.staminaLock=true;} }
    else { ps.stamina=Math.min(1,ps.stamina+dt*STAM.regen); if(ps.staminaLock&&ps.stamina>=STAM.lockUntil)ps.staminaLock=false; }
    const sprinting=wantSprint&&!ps.staminaLock&&ps.stamina>0;
    let sp=ps.ads?T.adsMoveMult:(sprinting?unitOf(ps.unit).sprintMult:1); if(ps.banzai) sp=Math.max(sp,1)*1.3; if(ps.shieldOn) sp*=T.shieldMoveMult;
    ps.vel.lerp(wish.multiplyScalar(T.moveSpeed*sp),1-Math.exp(-T.accel*dt));
    player.position.addScaledVector(ps.vel,dt);
    }
  } else ps.vel.multiplyScalar(0.8);
  const limX=T.arenaX-1, limZ=T.arenaZ-1; player.position.x=Math.max(-limX,Math.min(limX,player.position.x)); player.position.z=Math.max(-limZ,Math.min(limZ,player.position.z));
  if(!pilotJug){ resolveObstacles(player.position,0.7); tickVertical(player.position, ps, dt, keys['Space'] && !ps.dead, 0.7, true); }
  if(pilotJug) tickPilot(dt, performance.now()*0.001);
  if(!ps.dead&&!pilotJug){ _animSt.aimYaw=player.rotation.y;_animSt.t=_animClock;_animSt.kick=ps.fireKick;_animSt.dip=ps.wpnReloadDip;_animSt.rel=ps.reloading?Math.min(1,ps.reloadT/curW().reload):-1;_animSt.throwC=(charge.active&&charge.kind==='nade')?charge.t:-1;_animSt.lean=ps.bayonet?1:0;_animSt.grounded=ps.grounded;_animSt.ads=ps.ads?1:0;_animSt.swap=ps.wpnSwap; updateSoldierAnim(player,_animSt,dt); const _psp=hyp2(ps.vel.x,ps.vel.z); if(_psp>1.6){ ps._stepT=(ps._stepT||0)-dt; if(ps._stepT<=0){ if(typeof sfxStep==='function')sfxStep(); ps._stepT=Math.max(0.2,0.46-_psp*0.022); } } else ps._stepT=0; }
  // stamina HUD
  staminaBarEl.classList.toggle('show', ps.stamina<0.999 || ps.staminaLock);
  staminaBarEl.classList.toggle('low', ps.stamina<0.35 && !ps.staminaLock);
  staminaBarEl.classList.toggle('exhausted', ps.staminaLock);
  if(Math.abs(ps.stamina-_lastStam)>0.002){ _lastStam=ps.stamina; staminaFillEl.style.transform='scaleX('+ps.stamina.toFixed(3)+')'; }
  updateOcclusion(dt);   // unified Foxhole-style cutaway (fill + ink outline) for every structure between camera and player

  // vehicle enter/exit prompt (priority over terminal)
  nearJugObj=null; if(!pilotJug&&!ps.dead&&!gameOver){ let _jbd=JUG_ENTER_R*JUG_ENTER_R; for(const jug of juggernauts){ if(!jug.alive)continue; const dx=jug.group.position.x-player.position.x, dz=jug.group.position.z-player.position.z; const d=dx*dx+dz*dz; if(d<_jbd){_jbd=d;nearJugObj=jug;} } }
  // terminal prompt
  nearPanel=false; for(const bk of barracksList){ if(bk.panelPos && sameTeam(bk.team,PLAYER_TEAM) && player.position.distanceTo(bk.panelPos)<2.6){ nearPanel=true; break; } }
  nearPanel = nearPanel && !ps.dead && !gameOver;
  const nearTerm = !nearPanel && !nearJugObj && !pilotJug && player.position.distanceTo(terminalPos)<6 && !ps.dead && !gameOver;
  _so(termPrompt,(pilotJug||nearJugObj||nearPanel||nearTerm)?'1':'0');
  if(pilotJug){ termPrompt.classList.remove('poor'); _st(termText,'Exit — E'); }
  else if(nearJugObj){ termPrompt.classList.remove('poor'); _st(termText,'Enter Juggernaut — E'); }
  else if(nearPanel){ termPrompt.classList.remove('poor'); _st(termText,'Open Armory'); }
  else if(nearTerm){const rc=T.resupplyCost*priceMult();const ok=credits>=rc;termPrompt.classList.toggle('poor',!ok);_st(termText,ok?'Resupply — '+rc+' cr':'Need '+rc+' cr');}

  // camera
  // ---- camera framing ---- computed in CAMERA-RELATIVE axes so the limits behave identically at any yaw.
  // Uses aimGround (crosshair->ground), not aimPoint, so framing never snaps when the crosshair crosses a target.
  // ---- camera framing via a screen-space DEAD ZONE ----
  // Inside a centred box the camera does NOT react to crosshair movement (kills the
  // motion-sickness from constant micro-panning); as the crosshair pushes toward the
  // screen edge the camera eases to pan and look deeper. The aim itself stays instant.
  const dzx=ps.ads?T.camDeadX*0.45:T.camDeadX, dzy=ps.ads?T.camDeadY*0.45:T.camDeadY;
  const tgtPX=dzPush(mouseNDC.x,dzx), tgtPY=dzPush(mouseNDC.y,dzy);
  const ks=1-Math.exp(-T.camPushLerp*dt); smPushX+=(tgtPX-smPushX)*ks; smPushY+=(tgtPY-smPushY)*ks;
  const fwdReach=(smPushY>=0?smPushY*T.lookFwdRange:smPushY*T.lookBackRange);
  const lookFwd=Math.max(T.lookAheadFloor,Math.min(T.lookAheadHip,T.lookAheadBase+fwdReach*T.lookFwdGain));
  const lookSide=Math.max(-T.lookSideMax,Math.min(T.lookSideMax,smPushX*T.lookSideMax));
  const posFwd=Math.max(-T.posBackMax,Math.min(T.posFwdMax,(smPushY>=0?smPushY*T.posFwdMax:smPushY*T.posBackMax)));
  const posSide=Math.max(-T.posSideMax,Math.min(T.posSideMax,smPushX*T.posSideMax));
  _camLookT.set(player.position.x+fwdX*lookFwd+rgtX*lookSide,player.position.y+1.2,player.position.z+fwdZ*lookFwd+rgtZ*lookSide);
  _camAnchor.set(player.position.x+fwdX*posFwd+rgtX*posSide,player.position.y,player.position.z+fwdZ*posFwd+rgtZ*posSide);
  _camDesired.set(_camAnchor.x-fwdX*T.camBack,_camAnchor.y+T.camHeight,_camAnchor.z-fwdZ*T.camBack);
  // ADS camera reaction — per-style (T.adsCam). 'pitch': flatten the look toward the crosshair (cap 28.5deg, camera pos fixed).
  // 'pan': Foxhole-style parallel distance-pan toward the crosshair (pitch fixed; player clamped to stay on-screen). 'off': none.
  if(T.adsCam==='pitch'){ const _t=ps.ads?1:0; adsPitchK+=(_t-adsPitchK)*(1-Math.exp(-7*dt));
    const _ph=_camDesired.y-_camLookT.y, _dx=_camLookT.x-_camDesired.x, _dz=_camLookT.z-_camDesired.z, _dh=hyp2(_dx,_dz);
    if(_dh>0.01&&_ph>0.01){ const _nat=Math.atan2(_ph,_dh)*180/Math.PI, _base=(camPitchOverride!=null?camPitchOverride:_nat);
      const _adh=hyp2(aimGround.x-_camDesired.x,aimGround.z-_camDesired.z), _toAim=(_adh>0.01?Math.atan2(_camDesired.y-aimGround.y,_adh)*180/Math.PI:_base);
      const _tgt=Math.max(28.5,Math.min(_base,_toAim)), _eff=_base+(_tgt-_base)*adsPitchK;
      if(Math.abs(_eff-_nat)>0.05){ const _f=(_ph/Math.tan(_eff*Math.PI/180))/_dh; _camLookT.x=_camDesired.x+_dx*_f; _camLookT.z=_camDesired.z+_dz*_f; } } }
  else if(T.adsCam==='pan'){ const _t=ps.ads?1:0; adsPanK+=(_t-adsPanK)*(1-Math.exp(-7*dt));
    if(adsPanK>0.001){
      let _px=(fwdX*smPushY+rgtX*smPushX)*T.foxPanMax*adsPanK, _pz=(fwdZ*smPushY+rgtZ*smPushX)*T.foxPanMax*adsPanK;
      { const _pm=hyp2(_px,_pz), _cap=T.camBack*0.7; if(_pm>_cap){ _px*=_cap/_pm; _pz*=_cap/_pm; } } // keep player in front of cam -> projection stays monotonic
      const _EDGE=0.9; _panCam.fov=camera.fov; _panCam.aspect=camera.aspect; _panCam.up.set(0,1,0); _panCam.updateProjectionMatrix();
      for(let _i=0;_i<3;_i++){ _panCam.position.set(_camDesired.x+_px,_camDesired.y,_camDesired.z+_pz); _panCam.lookAt(_camLookT.x+_px,_camLookT.y,_camLookT.z+_pz); _panCam.updateMatrixWorld();
        _p1.set(player.position.x,player.position.y+0.2,player.position.z).project(_panCam);
        const _ov=Math.max(Math.abs(_p1.x),Math.abs(_p1.y))/_EDGE; if(_ov>1){ _px/=_ov; _pz/=_ov; } else break; }
      _camDesired.x+=_px; _camDesired.z+=_pz; _camLookT.x+=_px; _camLookT.z+=_pz; } }
  camBase.lerp(_camDesired,1-Math.exp(-T.camLerp*dt)); camLookAt.lerp(_camLookT,1-Math.exp(-T.lookLerp*dt));
  camRecoil=Math.min(2.2,Math.max(0,camRecoil-dt*3.2)); camShake=Math.min(0.6,Math.max(0,camShake-dt*2.5));
  const tf=ps.ads?T.fovADS:T.fov; fovBase+=(tf-fovBase)*(1-Math.exp(-8*dt)); fovKick=Math.max(0,fovKick-dt*15); camera.fov=fovBase+fovKick; camera.updateProjectionMatrix();
  // directional concussion kick (spring-damped): lurch view away from a blast, then settle
  camKickVel.x+=(-T.camKickStiffness*camKick.x-T.camKickDamping*camKickVel.x)*dt;
  camKickVel.y+=(-T.camKickStiffness*camKick.y-T.camKickDamping*camKickVel.y)*dt;
  camKickVel.z+=(-T.camKickStiffness*camKick.z-T.camKickDamping*camKickVel.z)*dt;
  camKick.addScaledVector(camKickVel,dt);
  camera.position.copy(camBase).add(camKick);
  if(camShake>0.001){camera.position.x+=(Math.random()*2-1)*camShake*0.18;camera.position.y+=(Math.random()*2-1)*camShake*0.34;}
  camera.lookAt(camLookAt);
  player.updateMatrixWorld();

  // crosshair visibility (swap to a thick dot while orbiting the camera)
  const showCH = started && !ps.dead && !gameOver;
  _sd(reticleEl,(showCH && !rotating) ? 'block' : 'none');
  _sd(rotDotEl,(showCH && rotating) ? 'block' : 'none');
  if(!showCH){ _sd(hitmarkEl,'none'); }

  // grenades
  for(let i=grenades.length-1;i>=0;i--){const g=grenades[i];g.vel.y-=20*dt;g.mesh.position.addScaledVector(g.vel,dt);g.fuse-=dt;
    if(g.mesh.position.y<=0.26){g.mesh.position.y=0.26;g.vel.y*=-0.4;g.vel.x*=0.6;g.vel.z*=0.6;}
    if(g.fuse<=0){explodeAt(g.mesh.position.clone(), g.molotov?1.8:1);scene.remove(g.mesh);g.mesh.geometry.dispose();g.mesh.material.dispose();grenades.splice(i,1);}}

  // torque-bow bolts: fly under gravity -> stick; armed bolts arm a short fuse then blast
  syncSimWorld(); // refresh SIM hitboxes so bolts hit the same head/torso/leg boxes as bullets
  for(let i=bolts.length-1;i>=0;i--){ const b=bolts[i];
    if(b.state==='fly'){
      b.prev.copy(b.mesh.position); b.vel.y-=T_CHARGE.bow.grav*dt; b.mesh.position.addScaledVector(b.vel,dt);
      const segLen=_bSeg.copy(b.mesh.position).sub(b.prev).length();
      if(segLen>1e-4){
        _bDir.copy(_bSeg).multiplyScalar(1/segLen); b.mesh.quaternion.setFromUnitVectors(_aup,_bDir);
        const r=resolveShot(SIM,SIM_PLAYER, b.prev.x,b.prev.y,b.prev.z, _bDir.x,_bDir.y,_bDir.z, segLen+0.05, teamFilter(PLAYER_TEAM)); // SIM head/torso/leg boxes -> a head-high bolt connects; no per-frame alloc
        if(r.kind!=='none'){
          const en=(r.kind==='enemy'&&SOLDIERS[r.id]!==ps)?SOLDIERS[r.id]:null; _bHit.set(r.px,r.py,r.pz);
          b.state='armed'; b.t=0; addImpact(_bHit,_aup,!!en); b.mesh.position.copy(_bHit);
          if(en&&en.alive){ b.stuckTo=en; b.stuckOff.copy(_bHit).sub(en.group.position); } else b.stuckTo=null; // stick to the body, ride it until the fuse blows
          continue;
        }
      }
      b.life+=dt;
      if(b.mesh.position.y<=0.05){ b.mesh.position.y=0.05; b.state='armed'; b.t=0; b.stuckTo=null; continue; }
      if(b.life>5){ removeBolt(b,i); continue; }
    } else if(b.state==='armed'){
      if(b.stuckTo){ if(b.stuckTo.alive){ b.mesh.position.copy(b.stuckTo.group.position).add(b.stuckOff); } else { explodeAt(b.mesh.position.clone(), 1.5); removeBolt(b,i); continue; } }
      b.t+=dt; const tm=b.mesh.userData.tipMat; if(tm) tm.emissiveIntensity=0.55+Math.abs(Math.sin(b.t*38))*0.6;
      if(b.t>=BOLT_FUSE){ explodeAt(b.mesh.position.clone(), 1.5); removeBolt(b,i); continue; }
    }
  }

  
  // enemies
  const pPos=player.position;
  enemies.forEach((en,ei)=>{
    if(!en.active)return;   // parked slot: its team doesn't play on this map
    if(en.alive){
      const epx=en.group.position.x, epz=en.group.position.z;
      const ddx=pPos.x-epx, ddz=pPos.z-epz; const distP=hyp2(ddx,ddz);
      _eEyes.set(epx,1.45,epz); _ePc.set(pPos.x,1.45,pPos.z);
      const canSee=!ps.dead && !gameOver && !DEV.freezeAI && distP<T.enemyDetect && hasLOS(_eEyes,_ePc);
      if(canSee){
        en.group.rotation.y=Math.atan2(ddx,ddz);
        _p2.set(pPos.x,1.4,pPos.z); pitchWeapon(en.group,_p2);
        en.strafeT-=dt;if(en.strafeT<=0){en.strafeDir*=-1;en.strafeT=1+Math.random()*1.5;}
        const eu=unitOf(en.unit); const pr=eu.preferRange;
        const inv=distP>1e-4?1/distP:0; _eDir.set(ddx*inv,0,ddz*inv); _eMv.set(0,0,0);
        if(distP>pr+3)_eMv.add(navSteer(en,pPos.x,pPos.z,_navDir,dt));else if(distP<pr-3)_eMv.sub(_eDir);
        _eStr.set(-_eDir.z,0,_eDir.x).multiplyScalar(en.strafeDir*0.8); _eMv.add(_eStr);
        if(_eMv.lengthSq()>0)_eMv.normalize(); enemyClimbSteer(en,_eMv); en.group.position.addScaledVector(_eMv,dt*T.enemyMoveSpeed*eu.speedMult);
        if(en.lungeCd>0)en.lungeCd-=dt;
        if(eu.lunge&&distP<3.2&&en.lungeCd<=0){ en.lungeCd=2.5; damagePlayer(eu.lunge,en.group.position,false,en); camShake+=0.25; if(typeof spawnComic==='function')spawnComic('SLASH!',player.position.clone().setY(1.9),2.2,'#FF7B00'); }
        en.fireCd-=dt;
        if(en.fireCd<=0){if(en.burstLeft<=0)en.burstLeft=eu.burst;enemyFire(en);en.burstLeft--;en.fireCd=en.burstLeft>0?eu.fireInterval:T.enemyBurstPause+Math.random()*0.6;}
      } else if(!gameOver && !DEV.freezeAI){
        if(ei%2===0 && tbox.active && tbox.owner!=='hostile'){   // ~half the bots contest an active Tech Box they don't hold
          const bp=TBOX.points[tbox.idx]; const to=_v.set(bp.x-en.group.position.x,0,bp.z-en.group.position.z); const d=to.length(); en.group.rotation.y=Math.atan2(to.x,to.z); en.group.userData.wpn.rotation.x*=0.9;
          if(d>TBOX.range-1.2){ en.group.position.addScaledVector(navSteer(en,bp.x,bp.z,_navDir,dt),dt*T.enemyMoveSpeed); }
        } else {
        const fb=nearestFriendlyBuilding(en.group.position);
        let fh=null,fhD=1e9; for(const h of harvesters){ if(sameTeam(h.team,PLAYER_TEAM)&&h.alive){ const dd=h.group.position.distanceTo(en.group.position); if(dd<fhD){fhD=dd;fh=h;} } }
        const fbD=fb?fb.mesh.position.distanceTo(en.group.position):1e9;
        if(fh && fhD<fbD && fhD<42){ const to=_v.copy(fh.group.position).sub(en.group.position);to.y=0;const d=to.length();to.normalize();en.group.rotation.y=Math.atan2(to.x,to.z);
          _p2.copy(fh.group.position); _p2.y=1.4; pitchWeapon(en.group,_p2);
          if(d>13)en.group.position.addScaledVector(navSteer(en,fh.group.position.x,fh.group.position.z,_navDir,dt),dt*T.enemyMoveSpeed);
          else{en.fireCd-=dt;if(en.fireCd<=0){siegeFireHarvester(en,fh);en.fireCd=T.enemyFireInterval*1.6+Math.random()*0.4;}} }
        else if(fb){const to=_v.copy(fb.mesh.position).sub(en.group.position);to.y=0;const d=to.length();to.normalize();en.group.rotation.y=Math.atan2(to.x,to.z);
          _p2.copy(fb.mesh.position); _p2.y=Math.min(3,fb.mesh.geometry.parameters.height*0.5); pitchWeapon(en.group,_p2);
          if(d>15)en.group.position.addScaledVector(navSteer(en,fb.mesh.position.x,fb.mesh.position.z,_navDir,dt),dt*T.enemyMoveSpeed);
          else{en.fireCd-=dt;if(en.fireCd<=0){siegeFire(en,fb);en.fireCd=T.enemyFireInterval*1.6+Math.random()*0.4;}}}
        else en.group.userData.wpn.rotation.x*=0.9;
        }
      }
      en.group.position.x=Math.max(-limX,Math.min(limX,en.group.position.x));en.group.position.z=Math.max(-limZ,Math.min(limZ,en.group.position.z));
      let _ejump=false; if(en.grounded){ const _jx=en.group.position.x-epx, _jz=en.group.position.z-epz, _jl=hyp2(_jx,_jz); if(_jl>1e-3){ const _f=en.group.position.y, _ah=surfaceHeightAt(en.group.position.x+_jx/_jl*1.5, en.group.position.z+_jz/_jl*1.5); if(_ah>_f+T.stepUp && _ah<=_f+EJUMP_REACH) _ejump=true; } }
      resolveObstacles(en.group.position,0.7); tickVertical(en.group.position, en, dt, _ejump, 0.7);
      { const _fOn=en.hitFlash>0; if(_fOn)en.hitFlash-=dt; if(_fOn!==en._fOn){ en._fOn=_fOn; const _hx=_fOn?0x661010:0x000000; en.group.traverse(o=>{if(o.isMesh&&o.material&&o.material.emissive)o.material.emissive.setHex(_hx);}); } }
      const bar=en.group.userData.bar;bar.grp.visible=true;bar.grp.lookAt(camera.position);const h=Math.max(0,Math.min(1,en.health/en.maxHealth));bar.fg.scale.x=h;bar.fg.position.x=-(1-h)*0.7;
      _animSt.aimYaw=en.group.rotation.y;_animSt.t=_animClock;_animSt.kick=0;_animSt.dip=0;_animSt.rel=-1;_animSt.throwC=-1;_animSt.lean=0;_animSt.grounded=en.grounded;en._aimT=Math.max(0,(en._aimT||0)-dt);_animSt.ads=en._aimT>0?1:0;_animSt.swap=0; updateSoldierAnim(en.group,_animSt,dt);
    } else {
      en.topple+=dt;if(en._tx===undefined){en._tx=(Math.random()<0.5?-1:1);en._tz=(Math.random()*2-1)*0.55;}animDeath(en.group,dt);en.group.userData.bar.grp.visible=false;
      if(!gameOver){const eRef=TEAM_INDEX[en.team].refinery;en.respawnT-=dt*(eRef&&eRef.alive?1.4:1);if(en.respawnT<=0){en.alive=true;setEnemyType(en,en.elite?2:(Math.random()*3)|0);en.health=en.maxHealth;en.lungeCd=0;en.topple=0;en._tx=undefined;en.state='patrol';en.group.rotation.x=0;en.group.rotation.z=0;en.group.position.copy(respawnPos(en));en.group.position.y=0;en.fireCd=1+Math.random();en.group.userData.wpn.rotation.x=0;updateHostileCount();}}
    }
  });

  // building sink animation
  buildings.forEach(b=>{if(!b.alive&&b.sink<1){b.sink=Math.min(1,b.sink+dt*0.7);b.mesh.position.y=b.mesh.geometry.parameters.height/2 - b.sink*b.mesh.geometry.parameters.height*0.85;b.trim.position.y=b.mesh.position.y+b.mesh.geometry.parameters.height/2-0.7;}});

  // FX
  if(typeof tickJuggernauts==='function')tickJuggernauts(dt, performance.now()*0.001);
  for(let i=fx.length-1;i>=0;i--){const f=fx[i];f.t+=dt;const k=f.t/f.life;
    if(f.kind==='tracer'||f.kind==='flash'){f.mesh.material.opacity=Math.max(0,0.9*(1-k));if(f.kind==='flash')f.mesh.scale.setScalar((f.r0||1)*(1+k*1.5));}
    else if(f.kind==='boom'){f.mesh.material.opacity=Math.max(0,0.9*(1-k));f.mesh.scale.setScalar(1+k*T.grenadeRadius);}
    else if(f.kind==='light')f.mesh.intensity=Math.max(0,(f.i0||4)*(1-k));
    else if(f.kind==='spark'){const pos=f.mesh.geometry.attributes.position;for(let j=0;j<f.ct;j++){f.vels[j].y-=9.8*dt;pos.array[j*3]+=f.vels[j].x*dt;pos.array[j*3+1]+=f.vels[j].y*dt;pos.array[j*3+2]+=f.vels[j].z*dt;}pos.needsUpdate=true;f.mesh.material.opacity=Math.max(0,1-k);}
    else if(f.kind==='casing'){ f.vel.y-=15*dt; f.mesh.position.addScaledVector(f.vel,dt); f.mesh.rotation.x+=f.av.x*dt; f.mesh.rotation.y+=f.av.y*dt; f.mesh.rotation.z+=f.av.z*dt; if(f.mesh.position.y<0.06){f.mesh.position.y=0.06; f.vel.y*=-0.35; f.vel.x*=0.55; f.vel.z*=0.55; f.av.multiplyScalar(0.5);} f.mesh.material.opacity = k<0.75?1:Math.max(0,1-(k-0.75)/0.25); }
    else if(f.kind==='shock'){const e=1-Math.pow(1-k,3);const sc=1+(T.shockRadius/f.baseR-1)*e;f.mesh.scale.set(sc,sc,sc);f.mesh.material.opacity=Math.max(0,f.op*(1-k));}
    else if(f.kind==='dust'){const sc=(1+f.grow*k)*(f.r0||1);f.mesh.scale.set(sc,sc*0.7,sc);f.mesh.position.y+=f.rise*dt;f.mesh.material.opacity=Math.max(0,f.op*(1-k*k));}
    else if(f.kind==='scorch'){f.mesh.material.opacity=f.op*(k<0.55?1:Math.max(0,1-(k-0.55)/0.45));}
    else if(f.kind==='fade'){f.mesh.material.opacity=Math.max(0,f.op*(1-k));}
    else if(f.kind==='frag'){const pos=f.mesh.geometry.attributes.position;for(let j=0;j<f.ct;j++){const v=f.vels[j];v.y-=T.fragGravity*dt;pos.array[j*3]+=v.x*dt;pos.array[j*3+1]+=v.y*dt;pos.array[j*3+2]+=v.z*dt;if(f.baseY+pos.array[j*3+1]<=0.04&&v.y<0){pos.array[j*3+1]=0.04-f.baseY;v.y=-v.y*T.fragRestitution;v.x*=T.fragFriction;v.z*=T.fragFriction;if(Math.random()<0.22)scuffTick(f.baseX+pos.array[j*3],f.baseZ+pos.array[j*3+2]);}}pos.needsUpdate=true;f.mesh.material.opacity=Math.max(0,1-k*k);f.mesh.material.color.setRGB(1,0.93-k*0.5,0.78-k*0.55);}
    else if(f.kind==='comic'){ const pop=Math.min(1,k*4); const s=f.sc*(0.45+0.55*pop); f.mesh.scale.set(s*1.7,s,1); f.mesh.material.opacity=k<0.55?1:Math.max(0,1-(k-0.55)/0.45); f.mesh.position.y+=dt*1.7; }
    if(f.t>=f.life){ if(f.slot)FXPool.release(f.slot); else { scene.remove(f.mesh); if(f.mesh.geometry)f.mesh.geometry.dispose(); if(f.mesh.material)f.mesh.material.dispose(); } fx.splice(i,1); }}

  if(hitmarkT>0){hitmarkT-=dt;if(hitmarkT<=0){_so(hitmarkEl,'0');_sd(hitmarkEl,'none');}}
  if(dmgDirT>0){dmgDirT-=dt;if(dmgDirT<=0)_so(dmgDirEl,'0');}
  updateReticle();
}
/* ===== MAIN MENU camera + MAP EDITOR ===== */
function setMenuCam(){ camera.up.set(0,1,0); camera.fov=52; camera.position.set(0,46,60); camera.lookAt(0,0,-2); camera.updateProjectionMatrix(); }

// ---- selection (single OR multi) ----
function _rotAround(o,cx,cz,da){ const dx=o.position.x-cx, dz=o.position.z-cz, c=Math.cos(da), s=Math.sin(da); o.position.x=cx+dx*c-dz*s; o.position.z=cz+dx*s+dz*c; o.rotation.y+=da; }
function _scaleAround(p,cx,cz,r){ p.position.x=cx+(p.position.x-cx)*r; p.position.y=p.position.y*r; p.position.z=cz+(p.position.z-cz)*r; p.scale.multiplyScalar(r); }
function _scaleBuilding(b,target){ const cur=b.scl||1, r=target/cur; if(Math.abs(r-1)<1e-4){ b.scl=target; return; } const cx=b.mesh.position.x, cz=b.mesh.position.z; [b.mesh,b.trim,b.label,(b.bar&&b.bar.grp)].forEach(p=>{ if(p)_scaleAround(p,cx,cz,r); }); b.box.setFromObject(b.mesh); b.scl=target; }
function _scaleBarracks(e,target){ const cur=e.scl||1, r=target/cur; if(Math.abs(r-1)<1e-4){ e.scl=target; return; } const cx=e.cx, cz=e.cz;
  e.walls.forEach(w=>{ _scaleAround(w.mesh,cx,cz,r); w.box.setFromObject(w.mesh); });
  [e.lintel,e.floor,e.roof,e.label,e.panel,e.plabel].forEach(p=>{ if(p)_scaleAround(p,cx,cz,r); });
  if(e.panelPos){ e.panelPos.x=cx+(e.panelPos.x-cx)*r; e.panelPos.z=cz+(e.panelPos.z-cz)*r; }
  e.iw=(e.iw||0)*r; e.id=(e.id||0)*r; e.scl=target; }
function _rotateBarracksBy(e,da){ const cx=e.cx, cz=e.cz;
  e.walls.forEach(w=>{ _rotAround(w.mesh,cx,cz,da); w.box.setFromObject(w.mesh); });
  [e.lintel,e.floor,e.roof,e.label,e.panel,e.plabel].forEach(p=>{ if(p)_rotAround(p,cx,cz,da); });
  if(e.panelPos){ const dx=e.panelPos.x-cx, dz=e.panelPos.z-cz, c=Math.cos(da), s=Math.sin(da); e.panelPos.x=cx+dx*c-dz*s; e.panelPos.z=cz+dx*s+dz*c; }
  e.rot=(e.rot||0)+da; }
function backToMenu(){
  mode='menu'; started=false; paused=false;
  if(typeof edLeaveToMenu==='function') edLeaveToMenu();
  pauseMenuEl.style.display='none'; pauseMenuEl.classList.remove('wr-manual');
  enemies.forEach(en=>{en.group.visible=en.active;});
  hudEl.style.display='none'; renderer.domElement.style.cursor='default';
  titleEl.style.display='flex'; titleEl.style.opacity='1'; wrMenuShow();
  setMenuCam();
}

/* ===================== WAR-ROOM UI — theme from ui_theme_mockup.html (approved); step 1: main menu + pause =====================
 * Behaviour for the dossier (main menu) and the clipboard (pause, and the field manual opened from the menu). Their DOM + CSS
 * live in both pages (identical apart from the editor's Map editor tab). Menu-time only: nothing here runs per frame.
 * Sounds go through the game's SFX context, so the Audio option mutes them; their noise buffers are built once and reused. */
const WR={ typeTok:0, deploying:false, reduce:false, thudBuf:null, tickBuf:null };
// the "Deployed" stamp lands (0.24 s slam) before the match starts and the dossier fades
const WR_DEPLOY_MS=280;
const WR_PEN='<svg class="wr-pen" viewBox="0 0 220 64" preserveAspectRatio="none" aria-hidden="true">'
  +'<path class="v1" pathLength="1" d="M18 34 C 20 8, 190 4, 206 26 C 218 50, 60 62, 22 50 C 8 44, 12 30, 30 22"/>'
  +'<path class="v2" pathLength="1" d="M12 55 C 60 50, 118 59, 168 53 C 186 51, 198 50, 212 42"/>'
  +'<path class="v3" pathLength="1" d="M14 52 C 70 48, 140 56, 208 50 M34 60 C 92 57, 150 62, 198 58"/>'
  +'<path class="v4" pathLength="1" d="M12 13 C 80 8, 150 10, 210 9 C 211 26, 209 42, 207 57 C 140 59, 70 58, 13 58 C 11 42, 12 28, 14 12"/></svg>';
function _wrNoise(c,sec){ const len=Math.max(1,Math.floor(c.sampleRate*sec)), b=c.createBuffer(1,len,c.sampleRate), d=b.getChannelData(0); for(let i=0;i<len;i++) d[i]=(Math.random()*2-1)*(1-i/len); return b; }
// rubber-stamp thud: a falling sine + a short noise slap
function sfxStampThud(){ if(!SFX.ctx||SFX.muted)return; const c=SFX.ctx,t=c.currentTime; if(!WR.thudBuf) WR.thudBuf=_wrNoise(c,0.08);
  const o=c.createOscillator(),g=c.createGain(); o.type='sine'; o.frequency.setValueAtTime(120,t); o.frequency.exponentialRampToValueAtTime(42,t+0.18);
  g.gain.setValueAtTime(0.55*SFX.master,t); g.gain.exponentialRampToValueAtTime(0.001,t+0.25); o.connect(g).connect(c.destination); o.start(t); o.stop(t+0.26);
  const n=c.createBufferSource(),ng=c.createGain(); ng.gain.value=0.35*SFX.master; n.buffer=WR.thudBuf; n.connect(ng).connect(c.destination); n.start(t); }
// typewriter key tick
function sfxTypeTick(){ if(!SFX.ctx||SFX.muted)return; const c=SFX.ctx; if(!WR.tickBuf) WR.tickBuf=_wrNoise(c,0.018);
  const n=c.createBufferSource(),g=c.createGain(); g.gain.value=0.12*SFX.master; n.buffer=WR.tickBuf; n.connect(g).connect(c.destination); n.start(c.currentTime); }
// types el's text (kept in data-text) behind a blinking cursor; a newer call, or leaving the menu, cancels it
function wrType(el){ const text=el.dataset.text||(el.dataset.text=el.textContent.trim()), tok=++WR.typeTok; el.textContent='';
  if(WR.reduce){ el.textContent=text; return; }
  const tn=document.createTextNode(''), cur=document.createElement('span'); cur.className='wr-cur'; cur.textContent='\u00a0'; el.appendChild(tn); el.appendChild(cur); let i=0;
  (function step(){ if(tok!==WR.typeTok||mode!=='menu'||i>=text.length) return; const ch=text[i++]; tn.data=text.slice(0,i); if(ch!==' ') sfxTypeTick(); setTimeout(step,22+Math.random()*30); })(); }
// the folder jolts when a stamp lands (Web Animations: no forced layout)
function wrJolt(el){ if(!el||WR.reduce||!el.animate) return; el.animate([{transform:'none'},{transform:'translate(-5px,3px)'},{transform:'translate(4px,-3px)'},{transform:'translate(-3px,2px)'},{transform:'translate(2px,-1px)'},{transform:'none'}],{duration:200}); }
// menu (re)shown: a fresh brief, no leftover stamps
function wrMenuShow(){ WR.deploying=false; document.querySelectorAll('#title .wr-deployed,#title .wr-splat').forEach(n=>n.remove()); const b=document.getElementById('wrBrief'); if(b) wrType(b); }
// DEPLOY: a red "Deployed" stamp slams where you clicked (keyboard: on the button), ink splats, the folder jolts, then the match starts
function wrDeploy(e){ if(mode!=='menu'||WR.deploying) return; WR.deploying=true; const btn=e&&e.currentTarget, sheet=document.getElementById('wrDossier');
  if(btn&&btn.blur) btn.blur();
  if(sheet){ const r=sheet.getBoundingClientRect(); let cx=e?e.clientX:0, cy=e?e.clientY:0;
    if(!e||e.detail===0){ const b=(btn&&btn.getBoundingClientRect)?btn.getBoundingClientRect():r; cx=b.left+b.width/2; cy=b.top+b.height/2; }
    const x=Math.max(10,Math.min(r.width-230,cx-r.left-110)), y=Math.max(10,Math.min(r.height-70,cy-r.top-80)), rot=(Math.random()*16-8).toFixed(1)+'deg';
    const st=document.createElement('div'); st.className='wr-stamp wr-deployed wr-slam'; st.setAttribute('aria-hidden','true'); st.textContent='Deployed';
    st.style.setProperty('--r',rot); st.style.transform='rotate('+rot+')'; st.style.left=x+'px'; st.style.top=y+'px'; sheet.appendChild(st);
    const sp=document.createElement('div'); sp.className='wr-splat'; sp.style.left=(x+50)+'px'; sp.style.top=(y-20)+'px'; sheet.appendChild(sp); setTimeout(()=>sp.remove(),600);
    sfxStampThud(); wrJolt(document.getElementById('wrFolder')); }
  setTimeout(()=>{ WR.deploying=false; if(mode==='menu') deploy(); }, WR.reduce?0:WR_DEPLOY_MS); }
// FIELD MANUAL: the pause clipboard opened from the menu (its orders swap for "Back to the dossier"); setPause/backToMenu reset it
function openManual(){ if(mode!=='menu'||WR.deploying) return; pauseMenuEl.classList.add('wr-manual'); pauseMenuEl.style.display='flex'; }
function closeManual(){ pauseMenuEl.style.display='none'; pauseMenuEl.classList.remove('wr-manual'); }
function wrPressed(sel,on){ document.querySelectorAll(sel).forEach(b=>b.setAttribute('aria-pressed',on(b)?'true':'false')); }
// boot: pen marks on every tab and order (a new variant is picked on leave), the reduced-motion flag, the first brief
function wrBoot(){ WR.reduce=!!(window.matchMedia&&matchMedia('(prefers-reduced-motion: reduce)').matches);
  document.querySelectorAll('.wr-tab,.wr-order').forEach(el=>{ el.insertAdjacentHTML('beforeend',WR_PEN); el.dataset.v='1';
    const next=()=>{ let v; do{ v=String(1+Math.floor(Math.random()*4)); }while(v===el.dataset.v); el.dataset.v=v; };
    el.addEventListener('pointerleave',next); el.addEventListener('blur',next); });
  wrMenuShow(); }
let _curX=-1,_curY=-1;
function placeCursor(){
  if(mx===_curX&&my===_curY) return; _curX=mx; _curY=my;
  reticleEl.style.transform='translate3d('+mx+'px,'+my+'px,0)';
  rotDotEl.style.left=mx+'px'; rotDotEl.style.top=my+'px';
  hitmarkEl.style.left=mx+'px'; hitmarkEl.style.top=my+'px';
  reloadRing.style.left=mx+'px'; reloadRing.style.top=my+'px';
  reloadLabel.style.left=mx+'px'; reloadLabel.style.top=my+'px';
}
function loop(){_frameId++;enemySpawnLayer.visible=(mode==='editor'&&!edPreview);const dt=Math.min(clock.getDelta(),0.05);if(mode==='game'){if(hitStop>0){hitStop=Math.max(0,hitStop-dt);}else if(!paused&&!shopOpen&&!tutorialOpen)update(dt);}else if(mode==='editor'){updateEditor(dt);}else if(mode==='charlab'){clTick(dt);}else{updateAim();}placeCursor();updateSunFollow();renderer.shadowMap.autoUpdate=!(mode==='game'&&hitStop>0);if(mode==='game')vigStep();renderer.render(scene,(mode==='editor'&&edOrtho&&!edPreview&&edOrthoCam)?edOrthoCam:camera);devCamLive();perfLive();sbSync();requestAnimationFrame(loop);}

function bootGame(){
rebuildBorder();

/* obstacles (cover) */
BK.TREES={
  apple:  { label:'Apple tree',      leaf:0x6a9e3e, trunkBot:0.28, trunkTop:0.22, trunkH:1.8, blobs:6,  canopyH:2.8, spread:1.8,  colR:1.0, seed:3 },
  poplar: { label:'Lombardy poplar', leaf:0x5f8f3a, trunkBot:0.34, trunkTop:0.18, trunkH:2.0, blobs:9,  canopyH:6.0, spread:1.25, colR:0.9, seed:7, columnar:true },
  oak:    { label:'Oak',             leaf:0x4f7f30, trunkBot:0.50, trunkTop:0.36, trunkH:2.6, blobs:8,  canopyH:4.2, spread:2.8,  colR:1.4, seed:11 }
};
BK.buildTree=function(kind, seed){
  const K=this.TREES[kind]||this.TREES.oak, g=new THREE.Group();
  const sd=((seed==null?1:seed)>>>0)||1;
  const H=(a,b)=>{ const v=Math.sin(a*127.1+b*311.7+sd*0.0009171+K.seed*0.37)*43758.5453; return v-Math.floor(v); };   // deterministic 0..1 per (a,b,tree-seed)
  const bark=new THREE.MeshStandardMaterial({ color:0x6b4a2e, roughness:0.95 });
  const lc=new THREE.Color(K.leaf).offsetHSL((H(90,3)-0.5)*0.03, (H(90,4)-0.5)*0.08, (H(90,5)-0.5)*0.10);   // subtle per-tree tint -> variety
  const leaf=new THREE.MeshStandardMaterial({ color:lc, roughness:0.9 }); leaf.userData.nature=true; bark.userData.nature=true;
  const trunk=new THREE.Mesh(new THREE.CylinderGeometry(K.trunkTop,K.trunkBot,K.trunkH,7), bark);
  trunk.position.y=K.trunkH/2; trunk.castShadow=true; trunk.receiveShadow=true; g.add(trunk);
  let topY=K.trunkH;
  if(K.columnar){                                        // cohesive tapering spindle: cumulative overlapping blobs, wide base -> rounded tip (no gaps)
    let y=K.trunkH + K.spread*0.55, r=K.spread*1.4;
    for(let i=0;i<K.blobs;i++){
      const wob=K.spread*0.16;
      const b=new THREE.Mesh(new THREE.IcosahedronGeometry(Math.max(0.32,r),1), leaf);
      b.position.set((H(i,1)-0.5)*wob, y, (H(i,2)-0.5)*wob); b.castShadow=true; b.receiveShadow=true; g.add(b);
      topY=Math.max(topY, y+r);
      const nr=r*(0.82 - H(i,6)*0.05);                   // taper each step (slight variance)
      y += (r+nr)*0.44; r=nr;                             // step < radius -> heavy overlap
    }
  } else {                                               // clustered rounded canopy (apple / oak)
    for(let i=0;i<K.blobs;i++){
      const ang=H(i,1)*6.2832, rad=H(i,2)*K.spread*0.7;
      const bx=Math.cos(ang)*rad, bz=Math.sin(ang)*rad;
      const by=K.trunkH + K.canopyH*0.35 + (H(i,3)-0.4)*K.canopyH*0.5;
      const r=K.spread*(0.5+0.28*H(i,4));
      const b=new THREE.Mesh(new THREE.IcosahedronGeometry(Math.max(0.32,r),1), leaf); b.position.set(bx,by,bz); b.castShadow=true; b.receiveShadow=true; g.add(b);
      topY=Math.max(topY, by+r);
    }
  }
  if(typeof stylizeOne==='function') stylizeOne(g);       // toon + BOLD world outline (runtime object)
  const sc=0.84 + H(90,9)*0.36;                           // per-tree overall scale (0.84..1.20)
  g.scale.set(sc,sc,sc);
  return { group:g, colR:+(K.colR*sc).toFixed(3), height:+(topY*sc).toFixed(2) };
};
// ---- farmyard props: registry + one builder (cel-shaded via stylizeOne; rotatable; per-prop AABB) ----
BK.PROPS={
  haystack:{ label:'Haystack',       h:2.4, solid:true,  los:true            },
  haybales:{ label:'Hay bales',      h:1.5, solid:true,  los:true,  rot:true },
  cart:{     label:'Farm cart',      h:1.3, solid:true,  los:true,  rot:true },
  well:{     label:'Well',           h:2.4, solid:true,  los:true            },
  logpile:{  label:'Log pile',       h:1.1, solid:true,  los:true,  rot:true },
  barrels:{  label:'Barrels',        h:1.0, solid:true,  los:true            },
  vegpatch:{ label:'Vegetable patch',h:0.4, solid:false, los:false, rot:true },
  trough:{   label:'Water trough',   h:0.7, solid:true,  los:false, rot:true }
};
BK.buildProp=function(kind, seed){
  const K=this.PROPS[kind]||this.PROPS.haystack, g=new THREE.Group();
  const sd=((seed==null?1:seed)>>>0)||1;
  const H=(a,b)=>{ const v=Math.sin(a*127.1+b*311.7+sd*0.0009171)*43758.5453; return v-Math.floor(v); };
  const wood=new THREE.MeshStandardMaterial({ color:0x8a5a2b, roughness:0.9 });
  const woodDark=new THREE.MeshStandardMaterial({ color:0x6b4423, roughness:0.9 });
  const hay=new THREE.MeshStandardMaterial({ color:0xd9b44a, roughness:0.98 });
  const hayDark=new THREE.MeshStandardMaterial({ color:0xc39a34, roughness:0.98 });
  const stone=new THREE.MeshStandardMaterial({ color:0x9a968c, roughness:1.0 });
  const metal=new THREE.MeshStandardMaterial({ color:0x555a5e, roughness:0.6, metalness:0.3 });
  const soil=new THREE.MeshStandardMaterial({ color:0x5a3d22, roughness:1.0 });
  const green=new THREE.MeshStandardMaterial({ color:0x5f9e3a, roughness:0.9 });
  const colH=K.h;
  if(kind==='haystack'){
    const body=new THREE.Mesh(new THREE.CylinderGeometry(1.35,1.6,K.h*0.5,12), hay); body.position.y=K.h*0.25; body.castShadow=true; body.receiveShadow=true; g.add(body);
    const top=new THREE.Mesh(new THREE.ConeGeometry(1.45,K.h*0.52,12), hayDark); top.position.y=K.h*0.5+K.h*0.26; top.castShadow=true; g.add(top);
    const r1=new THREE.Mesh(new THREE.TorusGeometry(1.5,0.06,6,16), hayDark); r1.rotation.x=Math.PI/2; r1.position.y=K.h*0.2; g.add(r1);
    const r2=new THREE.Mesh(new THREE.TorusGeometry(1.42,0.06,6,16), hayDark); r2.rotation.x=Math.PI/2; r2.position.y=K.h*0.4; g.add(r2);
    const knot=new THREE.Mesh(new THREE.ConeGeometry(0.26,0.42,7), hay); knot.position.y=K.h*1.02; knot.castShadow=true; g.add(knot);
  } else if(kind==='haybales'){
    const bw=1.3,bh=0.72,bd=0.86; const twMat=new THREE.MeshStandardMaterial({ color:0x7a5a26, roughness:1.0 });
    const bale=(x,y,z,ry,dk)=>{ const b=new THREE.Mesh(new THREE.BoxGeometry(bw,bh,bd),(dk?hayDark:hay)); b.position.set(x,y,z); b.rotation.y=ry; b.castShadow=true; b.receiveShadow=true; g.add(b);
      for(const sx of [-bw*0.26,bw*0.26]){ const t=new THREE.Mesh(new THREE.BoxGeometry(0.05,bh+0.02,bd+0.02), twMat); t.position.set(x+Math.cos(ry)*sx, y, z-Math.sin(ry)*sx); t.rotation.y=ry; g.add(t); } };
    bale(-0.42,bh/2,0.02,0.05,0); bale(0.42,bh/2,-0.04,-0.04,1); bale(0.48,bh/2,-0.9,0.03,0); bale(-0.36,bh/2,-0.92,-0.02,1);
    bale(-0.02,bh+bh/2,-0.28,0.08,1); bale(0.12,bh+bh/2,-0.66,-0.06,0);
  } else if(kind==='cart'){
    const bed=new THREE.Mesh(new THREE.BoxGeometry(2.2,0.28,1.1), wood); bed.position.y=0.72; bed.castShadow=true; bed.receiveShadow=true; g.add(bed);
    const railF=new THREE.Mesh(new THREE.BoxGeometry(2.2,0.42,0.08), woodDark); railF.position.set(0,0.98,0.5); railF.castShadow=true; g.add(railF);
    const railB=new THREE.Mesh(new THREE.BoxGeometry(2.2,0.42,0.08), woodDark); railB.position.set(0,0.98,-0.5); railB.castShadow=true; g.add(railB);
    const wgeo=new THREE.CylinderGeometry(0.52,0.52,0.14,12);
    const hubg=new THREE.CylinderGeometry(0.16,0.16,0.2,8), tyreMat=new THREE.MeshStandardMaterial({ color:0x4a2f18, roughness:0.92 });
    const wheel=(x,z)=>{ const so=(z>=0?0.03:-0.03);
      const tyre=new THREE.Mesh(wgeo, tyreMat); tyre.rotation.x=Math.PI/2; tyre.position.set(x,0.52,z); tyre.castShadow=true; g.add(tyre);
      const disc=new THREE.Mesh(new THREE.CylinderGeometry(0.4,0.4,0.16,12), wood); disc.rotation.x=Math.PI/2; disc.position.set(x,0.52,z); g.add(disc);
      const hub=new THREE.Mesh(hubg, woodDark); hub.rotation.x=Math.PI/2; hub.position.set(x,0.52,z); g.add(hub);
      const sv=new THREE.Mesh(new THREE.BoxGeometry(0.09,0.9,0.05), woodDark); sv.position.set(x,0.52,z+so); g.add(sv);
      const shk=new THREE.Mesh(new THREE.BoxGeometry(0.9,0.09,0.05), woodDark); shk.position.set(x,0.52,z+so); g.add(shk); };
    wheel(0.82,0.62); wheel(0.82,-0.62); wheel(-0.82,0.62); wheel(-0.82,-0.62);
    const sh1=new THREE.Mesh(new THREE.BoxGeometry(1.4,0.1,0.1), wood); sh1.position.set(1.6,0.6,0.34); g.add(sh1);
    const sh2=new THREE.Mesh(new THREE.BoxGeometry(1.4,0.1,0.1), wood); sh2.position.set(1.6,0.6,-0.34); g.add(sh2);
  } else if(kind==='well'){
    const ring=new THREE.Mesh(new THREE.CylinderGeometry(1.15,1.2,0.95,14), stone); ring.position.y=0.48; ring.castShadow=true; ring.receiveShadow=true; g.add(ring);
    const hole=new THREE.Mesh(new THREE.CylinderGeometry(0.82,0.82,0.98,14), new THREE.MeshStandardMaterial({ color:0x18262e, roughness:1.0 })); hole.position.y=0.5; g.add(hole);
    const pg=new THREE.BoxGeometry(0.15,1.5,0.15);
    const p1=new THREE.Mesh(pg, woodDark); p1.position.set(0.95,1.2,0); p1.castShadow=true; g.add(p1);
    const p2=new THREE.Mesh(pg, woodDark); p2.position.set(-0.95,1.2,0); p2.castShadow=true; g.add(p2);
    const beam=new THREE.Mesh(new THREE.CylinderGeometry(0.1,0.1,2.0,8), woodDark); beam.rotation.z=Math.PI/2; beam.position.set(0,1.95,0); g.add(beam);
    const rope=new THREE.Mesh(new THREE.BoxGeometry(0.04,1.05,0.04), new THREE.MeshStandardMaterial({ color:0x6a5a3a, roughness:1.0 })); rope.position.set(0.3,1.42,0); g.add(rope);
    const bucket=new THREE.Mesh(new THREE.CylinderGeometry(0.22,0.18,0.34,10), woodDark); bucket.position.set(0.3,0.9,0); bucket.castShadow=true; g.add(bucket);
    const crank=new THREE.Mesh(new THREE.BoxGeometry(0.06,0.32,0.06), woodDark); crank.position.set(1.02,1.78,0); g.add(crank);
    const roof=new THREE.Mesh(gableRoofGeo(2.7,1.7,0.5), new THREE.MeshStandardMaterial({ map:this.tex('tile'), roughness:0.9 })); roof.position.y=2.0; roof.castShadow=true; g.add(roof);
  } else if(kind==='logpile'){
    const lg=new THREE.CylinderGeometry(0.25,0.25,2.1,8);
    const rows=[[-0.78,-0.26,0.26,0.78],[-0.52,0,0.52],[-0.26,0.26]]; let y=0.26;
    const capg=new THREE.CylinderGeometry(0.25,0.25,0.05,8), capMat=new THREE.MeshStandardMaterial({ color:0xd8b57a, roughness:0.9 });
    for(let ri=0;ri<3;ri++){ for(const zo of rows[ri]){ const l=new THREE.Mesh(lg, (ri&1?wood:woodDark)); l.rotation.z=Math.PI/2; l.position.set(0,y,zo); l.castShadow=true; l.receiveShadow=true; g.add(l);
      const c1=new THREE.Mesh(capg,capMat); c1.rotation.z=Math.PI/2; c1.position.set(1.05,y,zo); g.add(c1); const c2=new THREE.Mesh(capg,capMat); c2.rotation.z=Math.PI/2; c2.position.set(-1.05,y,zo); g.add(c2); } y+=0.44; }
  } else if(kind==='barrels'){
    const bg=new THREE.CylinderGeometry(0.42,0.38,1.0,12), bandg=new THREE.CylinderGeometry(0.45,0.45,0.09,12);
    const spots=[[-0.5,-0.32],[0.52,-0.18],[-0.05,0.5],[0.6,0.56]];
    for(let i=0;i<spots.length;i++){ const bx=spots[i][0],bz=spots[i][1]; const b=new THREE.Mesh(bg,(i&1?woodDark:wood)); b.position.set(bx,0.5,bz); b.rotation.y=H(i,2)*6.2832; b.castShadow=true; b.receiveShadow=true; g.add(b);
      const d1=new THREE.Mesh(bandg,metal); d1.position.set(bx,0.72,bz); g.add(d1); const d2=new THREE.Mesh(bandg,metal); d2.position.set(bx,0.3,bz); g.add(d2); const d3=new THREE.Mesh(bandg,metal); d3.position.set(bx,0.5,bz); g.add(d3); const lid=new THREE.Mesh(new THREE.CylinderGeometry(0.37,0.37,0.06,12), woodDark); lid.position.set(bx,1.0,bz); g.add(lid); }
  } else if(kind==='vegpatch'){
    const bed=new THREE.Mesh(new THREE.BoxGeometry(3.0,0.22,2.0), soil); bed.position.y=0.11; bed.receiveShadow=true; g.add(bed);
    const furMat=new THREE.MeshStandardMaterial({ color:0x6b4a28, roughness:1.0 });
    for(let fr=0;fr<4;fr++){ const fz=-0.75+fr*0.5; const fur=new THREE.Mesh(new THREE.BoxGeometry(2.9,0.1,0.3), furMat); fur.position.set(0,0.24,fz); g.add(fur); }
    const sg=new THREE.ConeGeometry(0.13,0.36,5);
    for(let r=0;r<4;r++){ const rz=-0.75+r*0.5; for(let c=0;c<7;c++){ const cx=-1.32+c*0.44; const spr=new THREE.Mesh(sg, green); spr.position.set(cx,0.34,rz); g.add(spr); } }
  } else if(kind==='trough'){
    const body=new THREE.Mesh(new THREE.BoxGeometry(2.4,0.6,0.8), wood); body.position.y=0.42; body.castShadow=true; body.receiveShadow=true; g.add(body);
    const rim=new THREE.Mesh(new THREE.BoxGeometry(2.5,0.08,0.9), woodDark); rim.position.y=0.72; g.add(rim);
    for(const sx of [-0.8,0,0.8]){ const seam=new THREE.Mesh(new THREE.BoxGeometry(0.04,0.6,0.82), woodDark); seam.position.set(sx,0.42,0); g.add(seam); }
    const water=new THREE.Mesh(new THREE.BoxGeometry(2.2,0.1,0.62), new THREE.MeshStandardMaterial({ color:0x2f6b7a, roughness:0.3 })); water.position.y=0.66; g.add(water);
    const lg2=new THREE.BoxGeometry(0.13,0.38,0.13); const legs=[[-1.05,0.3],[1.05,0.3],[-1.05,-0.3],[1.05,-0.3]];
    for(const L of legs){ const lm=new THREE.Mesh(lg2,woodDark); lm.position.set(L[0],0.19,L[1]); g.add(lm); }
  }
  if(typeof stylizeOne==='function') stylizeOne(g);
  return { group:g, solid:!!K.solid, los:!!K.los, height:colH };
};
buildMap(DM_Cabana);

/* player */
player.userData.sword=makeSword(); player.userData.joints.handR.add(player.userData.sword); player.userData.sword.position.set(0,-0.05,0.05);
player.userData.axe=makeAxe(); player.userData.joints.handR.add(player.userData.axe); player.userData.axe.position.set(0,-0.05,0.05);
player.userData.shield=makeShield(0x38e1b0); player.userData.joints.handL.add(player.userData.shield); player.userData.shield.position.set(0,-0.12,0.05); player.userData.shield.scale.set(0.55,0.55,0.55);
// ADS aim visuals (created once; shown only while aiming down sights). Unit cylinders scaled on Y + oriented like tracers.
{ const slg=new THREE.CylinderGeometry(0.045,0.045,1,6); slg.translate(0,0.5,0);
  sightLine=new THREE.Mesh(slg,new THREE.MeshBasicMaterial({color:0xffffff,transparent:true,opacity:0.95,depthTest:false,depthWrite:false}));
  // Foxhole-style fade: opaque at the muzzle (uv.v=0) -> transparent toward the aim (uv.v=1), via an alpha gradient along the cylinder length
  { const _gN=64, _gd=new Uint8Array(_gN*4); for(let _i=0;_i<_gN;_i++){ const _a=Math.pow(1-_i/(_gN-1),1.25); _gd[_i*4]=255;_gd[_i*4+1]=255;_gd[_i*4+2]=255;_gd[_i*4+3]=Math.round(255*_a); }
    const _gt=new THREE.DataTexture(_gd,1,_gN,THREE.RGBAFormat); _gt.flipY=false; _gt.minFilter=THREE.LinearFilter; _gt.magFilter=THREE.LinearFilter; _gt.wrapS=_gt.wrapT=THREE.ClampToEdgeWrapping; _gt.needsUpdate=true; sightLine.material.map=_gt; sightLine.material.needsUpdate=true; }
  sightLine.renderOrder=994; sightLine.visible=false; noRay(sightLine); scene.add(sightLine);
  const hlg=new THREE.CylinderGeometry(0.055,0.055,1,6); hlg.translate(0,0.5,0);
  heightLine=new THREE.Mesh(hlg,new THREE.MeshBasicMaterial({color:0xffd23f,transparent:true,opacity:0.92,depthTest:false}));
  heightLine.renderOrder=995; heightLine.visible=false; noRay(heightLine); scene.add(heightLine); }
// Fixed slot table (three-teams item 4): TUNE.teams.size slots per team, team-major in TEAMS order. Slot 0 is you, so
// with size 4: 1-3 allies, 4-7 soviets, 8-11 empire (the player's team, TEAMS[0] today, gets size-1 bot slots).
// activateRoster() parks the slots of teams that don't play.
SOLDIERS[0]=ps;
for(let k=0;k<TEAMS.length;k++){ const tm=TEAMS[k];
  for(let j=(tm.id===PLAYER_TEAM?1:0);j<TUNE.teams.size;j++){
  const _fac=tm.faction; const g=spawnEnemyMesh(_fac,false); scene.add(g);   // no elite (D6): Empire bots wear the standard soldier
  const en={group:g,elite:false,faction:_fac,health:T.enemyHealth,alive:true,topple:0,respawnT:0,hitFlash:0,state:'patrol',wander:Math.random()*6.28,wanderT:0,fireCd:Math.random()*2,burstLeft:0,strafeDir:Math.random()<.5?1:-1,strafeT:0,unit:0,lungeCd:0,frags:0,downs:0,vy:0,grounded:true,_aimT:0,_tx:undefined,_tz:0,_fOn:false,_nav:null,
    id:SOLDIERS.length,team:tm.id,maxHealth:T.enemyHealth,active:false,lastAttackerId:-1,targetId:-1,retargetT:0,scanCursor:0,role:null,credits:0,px:undefined,pz:undefined,   // px/pz stay undefined until tickVertical's first step (it tests !==undefined)
    losT:0,targetVis:false,scanBestId:-1,scanBestD2:0,pushTeam:null};
  setEnemyType(en, j%3); en.health=en.maxHealth; g.position.copy(respawnPos(en));
  g.traverse(o=>{ if(o.isMesh) o.userData.enemyRef=en; }); // for aim hover detection
  enemies.push(en); SOLDIERS.push(en);
} }

/* ============ HARVESTERS / SUPPLY CONVOY (Phase 2) ============
 * One AI supply truck per team: refinery dock → supply cache → load → return → dock = cash + tech.
 * Destructible & contestable (shoot the enemy truck to deny their tech). Respawns while the
 * refinery lives; self-destructs (no respawn) if the refinery dies. Movement is straight-line
 * waypoints (open top-down map, no pathfinding). Hot path is allocation-free. */
for(let i=0;i<TEAMS.length;i++) makeHarvester(TEAMS[i].id); makeSupplyField(); buildTeamIndex(); activateRoster();

/* ============ TECH BOXES (Phase 3) ============
 * One box at a time, rotating among 3 capture points. Hold E inside the ring with no enemies
 * present for captureTime to claim it; an enemy on the ring contests (freezes/decays progress) —
 * clear them to capture or steal. Channeling blocks your weapons (you're vulnerable). At expiry
 * the holding team banks ECO.techBox tech; a new box then spawns at a different point after a gap.
 * ~half the bots contest the box they don't hold; the rest keep sieging buildings/harvester. */
TBOX.points.forEach(pt=>techPts.push(makeTechPoint(pt))); makeTechBoxMesh();
SILO.points.forEach(pt=>silos.push(makeSilo(pt)));
_rulesReady=true; applyRules(_rules);
FXPool._def('casing',()=>{ const m=new THREE.Mesh(new THREE.BoxGeometry(0.055,0.055,0.14),new THREE.MeshStandardMaterial({color:0xd6a23a,roughness:0.5,metalness:0.6,emissive:0x3a2a08,emissiveIntensity:0.25,transparent:true,opacity:1})); noRay(m); m.userData.noOutline=true; stylizeOne(m); m.visible=false; scene.add(m); return {mesh:m,vel:new THREE.Vector3(),av:new THREE.Vector3()}; });
FXPool._def('flash',()=>{ const m=new THREE.Mesh(new THREE.SphereGeometry(1,8,6),new THREE.MeshBasicMaterial({color:0xffd27a,transparent:true,opacity:0.95})); noRay(m); m.visible=false; scene.add(m); return {mesh:m}; });
FXPool._def('tracer',()=>{ const m=new THREE.Mesh(new THREE.CylinderGeometry(0.05,0.05,1,6),new THREE.MeshBasicMaterial({color:0xfff0b0,transparent:true,opacity:0.95})); noRay(m); m.visible=false; scene.add(m); return {mesh:m}; });
FXPool._def('boom',()=>{ const m=new THREE.Mesh(new THREE.SphereGeometry(1,16,12),new THREE.MeshBasicMaterial({color:0xffb060,transparent:true,opacity:0.9})); noRay(m); m.visible=false; scene.add(m); return {mesh:m}; });
FXPool._def('spark',()=>{ const g=new THREE.BufferGeometry(); g.setAttribute('position',new THREE.BufferAttribute(new Float32Array(FX_SPARK_MAX*3),3)); const pts=new THREE.Points(g,new THREE.PointsMaterial({color:0xffffff,size:0.2,transparent:true,opacity:1,depthWrite:false})); noRay(pts); pts.visible=false; scene.add(pts); const vels=[]; for(let i=0;i<FX_SPARK_MAX;i++)vels.push(new THREE.Vector3()); return {mesh:pts,vels}; });
FXPool._def('light',()=>{ const l=new THREE.PointLight(0xffffff,4,8); l.visible=false; scene.add(l); return {mesh:l}; });   // pooled transient FX light (D1)
FXPool._def('comic',()=>{ const s=new THREE.Sprite(new THREE.SpriteMaterial({ transparent:true, depthWrite:false })); noRay(s); s.visible=false; scene.add(s); return {mesh:s}; });   // pooled onomatopoeia sprite; map swapped on acquire (D2)
// Pooled one-shot ground/air FX (each used to build a fresh Mesh + Geometry + Material per event): the slots share unit
// geometries (scaled per use) and own their material, so per-instance opacity fades still work.
{ const G={ scuff:new THREE.CircleGeometry(1,7), scorch:new THREE.CircleGeometry(1,28), dust:new THREE.SphereGeometry(1,10,8),
    shockA:new THREE.RingGeometry(0.45,0.62,48), shockB:new THREE.RingGeometry(0.30,0.55,48) };
  const mk=(geo,col,two)=>()=>{ const m=new THREE.Mesh(geo,new THREE.MeshBasicMaterial({color:col,transparent:true,depthWrite:false,side:two?THREE.DoubleSide:THREE.FrontSide})); noRay(m); m.visible=false; scene.add(m); return {mesh:m}; };
  FXPool._def('scuff',mk(G.scuff,0x5e4f3c),40); FXPool._def('scorch',mk(G.scorch,0x0d0a07)); FXPool._def('dust',mk(G.dust,0x877a66));
  FXPool._def('shockA',mk(G.shockA,0xe6efe8,true)); FXPool._def('shockB',mk(G.shockB,0xe6efe8,true)); }
FXPool._def('frag',()=>{ const g=new THREE.BufferGeometry(); g.setAttribute('position',new THREE.BufferAttribute(new Float32Array(FX_FRAG_MAX*3),3)); const pts=new THREE.Points(g,new THREE.PointsMaterial({color:0xfff0d8,size:0.42,transparent:true,opacity:1,sizeAttenuation:true,depthWrite:false})); noRay(pts); pts.visible=false; scene.add(pts); const vels=[]; for(let i=0;i<FX_FRAG_MAX;i++)vels.push(new THREE.Vector3()); return {mesh:pts,vels}; });   // pooled shrapnel cloud (D3)
addEventListener('keydown',e=>{
  if(e.code==='Backquote'&&!e.repeat){ devToggle(); return; }
  if(e.code==='Tab'&&mode==='game'){ e.preventDefault(); if(!e.repeat&&!paused&&!shopOpen&&!gameOver)sbShow(true); return; }
  if(mode==='editor'){ if(e.target&&/INPUT|TEXTAREA/.test(e.target.tagName))return; keys[e.code]=true; if(edPreview){ if((e.code==='Tab'||e.code==='Escape')&&!e.repeat){ e.preventDefault(); edExitPreview(); } return; } if(e.code==='Tab'){ e.preventDefault(); if(!e.repeat)edTogglePreview(); return; } if((e.ctrlKey||e.metaKey)&&e.code==='KeyD'){ e.preventDefault(); if(!e.repeat)edDuplicate(); return; } if((e.ctrlKey||e.metaKey)&&e.code==='KeyZ'&&!e.shiftKey){ e.preventDefault(); if(!e.repeat)edUndoDo(); return; } if((e.ctrlKey||e.metaKey)&&(e.code==='KeyY'||(e.shiftKey&&e.code==='KeyZ'))){ e.preventDefault(); if(!e.repeat)edRedoDo(); return; } if((e.ctrlKey||e.metaKey)&&e.code==='KeyC'){ e.preventDefault(); if(!e.repeat)edCopy(); return; } if((e.ctrlKey||e.metaKey)&&e.code==='KeyV'){ e.preventDefault(); if(!e.repeat)edPaste(); return; } if((e.code==='ArrowUp'||e.code==='ArrowDown'||e.code==='ArrowLeft'||e.code==='ArrowRight')&&edMulti.length){ e.preventDefault(); edNudge(e.code, e.shiftKey); return; } if(e.code==='KeyM'&&edMirror){ e.preventDefault(); if(!e.repeat)edMirrorSel(); return; } if(e.code==='KeyO'){ if(!e.repeat)edToggleOrtho(); return; } if(e.code==='KeyG'){ if(!e.repeat)edToggleSnap(); return; } if(e.code==='Delete'||e.code==='Backspace'){ edDeleteSel(); return; } if(e.code==='KeyQ'){ if(!e.repeat){ edGizmoMode='move'; if(typeof setEdTool==='function')setEdTool('select'); const rl=document.getElementById('edRail'); if(rl)rl.querySelectorAll('.edRailBtn[data-group="tool"]').forEach(b=>b.classList.toggle('on',b.dataset.mode==='select')); if(typeof edUpdateGizmo==='function'){edUpdateGizmo();edUpdateHandles();edUpdateRing();} } return; } if(e.code==='KeyE'){ if(!e.repeat&&typeof edSetGizmoModeKey==='function')edSetGizmoModeKey('move'); return; } if(e.code==='KeyR'){ if(!e.repeat&&typeof edSetGizmoModeKey==='function')edSetGizmoModeKey('rotate'); return; } if(e.code==='KeyT'){ if(!e.repeat&&typeof edSetGizmoModeKey==='function')edSetGizmoModeKey('scale'); return; } if(e.code==='Escape'){ backToMenu(); return; } return; }
  if(mode==='menu'&&e.code==='Escape'&&pauseMenuEl.classList.contains('wr-manual')){ closeManual(); return; }
  if(mode!=='game') return;
  if(tutorialOpen){ if(e.code==='Enter'||e.code==='Escape'||e.code==='Space') closeTutorial(); return; }
  if(e.repeat) return;
  if(shopOpen){ if(e.code==='KeyE'||e.code==='Escape') closeShop(); return; }
  if((e.code==='Escape'||e.code==='KeyP')&&!gameOver){ togglePause(); return; }
  if(paused) return;
  keys[e.code]=true;
  if(e.code==='KeyR'){ if(gameOver) resetRound(); else startReload(); }
  if(e.code==='KeyF') useAbility();
  if(e.code==='KeyG'&&!e.repeat) startCharge('nade');
  if(e.code==='KeyE'){ if(pilotJug){exitJug();} else if(nearJugObj){enterJug(nearJugObj);} else if(nearPanel){openShop();} else {tryResupply();} }
  if(e.code==='Digit1') equipPrimary();
  if(e.code==='Digit3') equipSpecial();
  if(e.code==='Digit2') devCycleGun(1);
  if(e.code==='BracketLeft') camYawTarget=0;
});
addEventListener('keyup',e=>{keys[e.code]=false; if(e.code==='Tab')sbShow(false); if(e.code==='KeyG'&&charge.active&&charge.kind==='nade') releaseCharge();});
// Cache the canvas rect so the hot mousemove path does ZERO forced layout reads.
addEventListener('scroll',refreshRect,true);
// Hot path stores pointer + NDC only. All cursor-element positioning happens once per
// frame in placeCursor() (driven by the render loop) so the crosshair stays locked to the
// rendered frame and can never desync, and we avoid layout-thrash at the mouse poll rate.
renderer.domElement.addEventListener('mousemove',e=>{
  mx=e.clientX; my=e.clientY;
  mouseNDC.x=((mx-_cvR.left)/_cvR.width)*2-1; mouseNDC.y=-((my-_cvR.top)/_cvR.height)*2+1;
  if(rotating&&mode==='game') camYawTarget-=(e.movementX||0)*T.camYawSens;
});
renderer.domElement.addEventListener('mousedown',e=>{if(mode!=='game'||shopOpen)return;e.preventDefault();if(e.button===1){rotating=true;}if(e.button===0){ if(curW().charge){ startCharge('bow'); } else { mouseDown=true; semiQueued=true; } }if(e.button===2){ if(adsMode==='toggle') ps.ads=!ps.ads; else ps.ads=true; }});
addEventListener('mouseup',e=>{if(e.button===1)rotating=false;if(mode!=='game')return;if(e.button===0){ mouseDown=false; if(charge.active&&charge.kind==='bow') releaseCharge(); }if(e.button===2&&adsMode==='hold')ps.ads=false;});
// --- game mode: kill text-selection / drag behaviours that hijack mouse input ---
['selectstart','dragstart','dblclick'].forEach(ev=>document.addEventListener(ev,e=>e.preventDefault()));
document.addEventListener('contextmenu',e=>{ const t=e.target; if(t&&(/INPUT|TEXTAREA/.test(t.tagName)||t.isContentEditable))return; e.preventDefault(); });
// only release inputs when the tab is genuinely hidden (won't fire on normal clicks/focus changes)
document.addEventListener('visibilitychange',()=>{if(document.hidden){mouseDown=false;semiQueued=false;ps.ads=false;rotating=false;endCharge();for(const k in keys)keys[k]=false;}});
renderer.domElement.addEventListener('contextmenu',e=>e.preventDefault());
renderer.domElement.addEventListener('wheel',e=>{e.preventDefault(); if(mode==='editor'){ if(edPreview)return; if(e.shiftKey&&edMulti.length){ edRotateSel(e.deltaY>0?1:-1, e.altKey); } else { edZoom(e.deltaY); } return; } else if(mode==='charlab'){ clDist=Math.max(2.4,Math.min(16,clDist*(1+(e.deltaY>0?0.12:-0.12)))); }
  else if(mode==='game'&&DEV.allGuns){ const _n=performance.now(); if(_n-_devWheelT>160){ _devWheelT=_n; devCycleGun(e.deltaY>0?1:-1); } }},{passive:false});
// pause / main menu
document.getElementById('pmResume').addEventListener('click',()=>setPause(false));
document.getElementById('pmRestart').addEventListener('click',()=>{resetRound();setPause(false);});
document.getElementById('pmMenu').addEventListener('click',()=>{setPause(false);backToMenu();});
document.querySelectorAll('#pauseMenu [data-ads]').forEach(el=>{el.addEventListener('click',()=>{adsMode=el.dataset.ads;wrPressed('#pauseMenu [data-ads]',b=>b.dataset.ads===adsMode);if(adsMode==='hold')ps.ads=false;sfxTypeTick();});});
document.querySelectorAll('#pauseMenu [data-audio]').forEach(el=>{el.addEventListener('click',()=>{SFX.muted=el.dataset.audio==='off'; if(SFX.muted)stopAmbient(); else if(mode==='game')startAmbient(); wrPressed('#pauseMenu [data-audio]',b=>b.dataset.audio===(SFX.muted?'off':'on'));sfxTypeTick();});});

document.getElementById('mPlay').addEventListener('click',wrDeploy); document.getElementById('mDeployTab').addEventListener('click',wrDeploy);
document.getElementById('mManual').addEventListener('click',openManual); document.getElementById('pmBack').addEventListener('click',closeManual);
wrBoot();
renderer.domElement.addEventListener('click',()=>{ if(mode==='game'&&gameOver) resetRound(); });
endEl.addEventListener('click',()=>{ if(gameOver) resetRound(); });

/* raycasting + 3D aim */
slotsEl.appendChild(rankLabel); slotsEl.appendChild(pipWrap);
document.getElementById('shopClose').onclick=closeShop;

/* ===================== PROGRESSION — global, persistent levels + prestige ===================== */
document.getElementById('mCareer').addEventListener('click',()=>{ if(!WR.deploying) openCareer(); });
document.getElementById('pmCareer').addEventListener('click',openCareer);
document.getElementById('cpClose').addEventListener('click',closeCareer);
(function(){ let armed=false,tmr=0; const pb=document.getElementById('cpPrestige'); pb.addEventListener('click',()=>{ if(Prog.level<100)return; if(!armed){ armed=true; pb.classList.add('arm'); pb.textContent='Confirm Prestige?'; clearTimeout(tmr); tmr=setTimeout(()=>{armed=false;pb.classList.remove('arm');pb.textContent='★ Prestige';},2600); return; } armed=false; pb.classList.remove('arm'); pb.textContent='★ Prestige'; doPrestige(); }); })();
ProgStore.load().then(d=>{ if(d&&typeof d==='object') Object.assign(Prog,d); updateCareerChip(); });
updateCareerChip();

/* ===== DEV / admin panel ===== */
(function(){ const $=id=>document.getElementById(id);
  $('dpClose').onclick=()=>{ devPanelEl.style.display='none'; };
  $('dvGod').onclick=()=>{ DEV.god=!DEV.god; devSync(); };
  $('dvAmmo').onclick=()=>{ DEV.infAmmo=!DEV.infAmmo; devSync(); };
  $('dvFreeze').onclick=()=>{ DEV.freezeAI=!DEV.freezeAI; devSync(); };
  $('dvBoom').oninput=()=>{ T.camBack=parseFloat($('dvBoom').value); $('dvBoomV').textContent=T.camBack.toFixed(1); dvPitchSync(); };
  $('dvHeight').oninput=()=>{ T.camHeight=parseFloat($('dvHeight').value); $('dvHeightV').textContent=T.camHeight.toFixed(1); dvPitchSync(); };
  $('dvFov').oninput=()=>{ T.fov=parseFloat($('dvFov').value); T.fovADS=T.fov; $('dvFovV').textContent=T.fov+'°'; };
  $('dvHtStr').oninput=()=>{ htOpacity=parseFloat($('dvHtStr').value); $('dvHtStrV').textContent=htOpacity<=0?'off':htOpacity.toFixed(2); updateHalftone(); };
  $('dvHtSize').oninput=()=>{ htSize=parseFloat($('dvHtSize').value); $('dvHtSizeV').textContent=htSize.toFixed(1); updateHalftone(); };
  updateHalftone();
  $('dvCam1').onclick=()=>applyCamStyle(CAM_STYLES[1]); $('dvCam2').onclick=()=>applyCamStyle(CAM_STYLES[2]); $('dvCam3').onclick=()=>applyCamStyle(CAM_STYLES[3]); $('dvCam4').onclick=()=>applyCamStyle(CAM_STYLES[4]); $('dvCam5').onclick=()=>applyCamStyle(CAM_STYLES[5]);
  $('dvAdsCam').onclick=()=>{ T.adsCam=(T.adsCam==='pitch')?'pan':(T.adsCam==='pan')?'off':'pitch'; adsPitchK=0; adsPanK=0; devSyncCam(); };
  $('dvCamReset').onclick=()=>{ applyCamStyle(CAM_STYLES[3]); };
  $('dvPitch').oninput=()=>{ const p=parseFloat($('dvPitch').value); camPitchOverride=p; $('dvPitchV').textContent=p.toFixed(0)+'°'; };
  { const mp=$('dvMap'); if(mp) mp.oninput=()=>{ setMapSize(+mp.value); $('dvMapV').textContent=T.arenaX; }; }
  $('dvHeal').onclick=()=>{ ps.health=ps.maxHealth; ps.dead=false; ps.alive=true; updateHealthHUD(); };
  $('dvCred').onclick=()=>{ credits+=1000; updateCreditHUD(); };
  $('dvHarv').onclick=()=>{ credits+=ECO.payHarvester; updateCreditHUD(); addTech(PLAYER_TEAM,ECO.techHarvester); showLevelToast('Harvester','+'+ECO.payHarvester+' cr · +'+ECO.techHarvester+' tech'); };
  $('dvBox').onclick=()=>{ addTech(PLAYER_TEAM,ECO.techBox); showLevelToast('Tech Box','+'+ECO.techBox+' tech (captured)'); };
  $('dvTechUs').onclick=()=>addTech(PLAYER_TEAM,20);
  $('dvTechEn').onclick=()=>addTech(TEAM_INDEX.firstEnemy,20);
  $('dvSilo').onclick=()=>{ teamEco[PLAYER_TEAM].silos++; showLevelToast('Silo','+'+ECO.paySilo+'/s · total '+teamEco[PLAYER_TEAM].silos); };
  $('dvEra').onclick=()=>{ const f=teamEco[PLAYER_TEAM]; if(f.era<ECO.eraNames.length) addTech(PLAYER_TEAM, ECO.eraCost[f.era-1]-f.tech); };
  $('dvUnlock').onclick=()=>{ loadout.forEach((l,i)=>{ l.owned=true; const w=WEAPONS[i]; if(!(l.ammo>0)) l.ammo=w.mag||0; if(!(l.reserve>0)) l.reserve=w.reserve||0; }); ps.shieldOwned=true; DEV.allGuns=true;
    updateHeldModel(); updateWeaponHUD(); showLevelToast('Dev','All weapons unlocked \u2014 mouse wheel or 2 cycles every gun'); };
  $('dvXp').onclick=()=>addXP(1000);
  $('dvLvl').onclick=()=>devSetLevel(Prog.level+1);
  $('dvMax').onclick=()=>devSetLevel(100);
  $('dvPrest').onclick=devPrestige;
  $('dvReset').onclick=devReset;
  $('dvKill').onclick=()=>{ if(mode!=='game')return; enemies.forEach(en=>{ if(en.alive) damageEnemy(en,T.enemyHealth+999,'body',en.group.position.clone().setY(1.4),true); }); };
  $('dvWin').onclick=()=>{ if(mode==='game')endGame(true); };
  $('dvLose').onclick=()=>{ if(mode==='game')endGame(false); };
})();

/* ===== FEEL SYSTEMS: stamina cfg, audio, headshot skull, XP banner, onboarding ===== */
addEventListener('pointerdown',audioKick); addEventListener('keydown',audioKick);
document.getElementById('tutGo').onclick=closeTutorial;

// --- stamina/skull/xp HUD element refs (used in update) ---
resetHarvesters();resetTechBox();resetSilos();grantFreeWeapons(true);updateWeaponHUD();updateHealthHUD();updateCreditHUD();updateGrenadeHUD();updateHostileCount();updateBaseHUD();updateEraHUD();
stylizeScene(scene);
enemies.forEach(en=>{ if(en.elite&&typeof applyVillain==='function') applyVillain(en.group); });   // Empire elite: lacquer+visor+purple outline (post-toonify)
applyEnv(_env,{regen:false}); // env + ground cel-cap now that the ground material is toon

/* helpers */
  navBuild();   // warm the pathfinding grid while loading (deploy then rebuilds it warm, ~40 ms)
  // Warm navPath's branches before V8 optimises it (a start in a blocked cell: the poplar; a goal in a blocked cell: the
  // refinery; an open route round the poplar row), or the first real use of each branch deopts it (measured: up to 54 per match).
  { const w=new Float32Array(128); navPath(-80,-81,0,30,w); navPath(0,30,-100,80,w); navPath(-79,-90,-79,-70,w); }
}

// V8 'const tracking let': a top-level let assigned only once is treated as a constant and baked into optimized code;
// its FIRST later reassignment (first kill, first vehicle entry, ...) deopts every function that baked it in (measured
// on update()). One real value change per let at load marks them mutable up front. The change keeps each slot's type
// (numbers +1 then the exact original, booleans flipped twice) so number slots stay unboxed: a sentinel object store
// made every later per-frame number write allocate (measured +2 MB/s).
{ const _S={}, _w=function(x){ _w.v=x; }; if(typeof htOpacity==='number'){ _w.v=htOpacity; htOpacity=htOpacity+1; htOpacity=_w.v; } else if(typeof htOpacity==='boolean'){ htOpacity=!htOpacity; htOpacity=!htOpacity; } else { _w.v=htOpacity; htOpacity=_S; htOpacity=_w.v; } if(typeof htSize==='number'){ _w.v=htSize; htSize=htSize+1; htSize=_w.v; } else if(typeof htSize==='boolean'){ htSize=!htSize; htSize=!htSize; } else { _w.v=htSize; htSize=_S; htSize=_w.v; } if(typeof _natureGain==='number'){ _w.v=_natureGain; _natureGain=_natureGain+1; _natureGain=_w.v; } else if(typeof _natureGain==='boolean'){ _natureGain=!_natureGain; _natureGain=!_natureGain; } else { _w.v=_natureGain; _natureGain=_S; _natureGain=_w.v; } if(typeof _env==='number'){ _w.v=_env; _env=_env+1; _env=_w.v; } else if(typeof _env==='boolean'){ _env=!_env; _env=!_env; } else { _w.v=_env; _env=_S; _env=_w.v; } if(typeof NPR==='number'){ _w.v=NPR; NPR=NPR+1; NPR=_w.v; } else if(typeof NPR==='boolean'){ NPR=!NPR; NPR=!NPR; } else { _w.v=NPR; NPR=_S; NPR=_w.v; } if(typeof COMIC==='number'){ _w.v=COMIC; COMIC=COMIC+1; COMIC=_w.v; } else if(typeof COMIC==='boolean'){ COMIC=!COMIC; COMIC=!COMIC; } else { _w.v=COMIC; COMIC=_S; COMIC=_w.v; } if(typeof CHAR_OUTLINE==='number'){ _w.v=CHAR_OUTLINE; CHAR_OUTLINE=CHAR_OUTLINE+1; CHAR_OUTLINE=_w.v; } else if(typeof CHAR_OUTLINE==='boolean'){ CHAR_OUTLINE=!CHAR_OUTLINE; CHAR_OUTLINE=!CHAR_OUTLINE; } else { _w.v=CHAR_OUTLINE; CHAR_OUTLINE=_S; CHAR_OUTLINE=_w.v; } if(typeof unitOwned==='number'){ _w.v=unitOwned; unitOwned=unitOwned+1; unitOwned=_w.v; } else if(typeof unitOwned==='boolean'){ unitOwned=!unitOwned; unitOwned=!unitOwned; } else { _w.v=unitOwned; unitOwned=_S; unitOwned=_w.v; } if(typeof sightLine==='number'){ _w.v=sightLine; sightLine=sightLine+1; sightLine=_w.v; } else if(typeof sightLine==='boolean'){ sightLine=!sightLine; sightLine=!sightLine; } else { _w.v=sightLine; sightLine=_S; sightLine=_w.v; } if(typeof heightLine==='number'){ _w.v=heightLine; heightLine=heightLine+1; heightLine=_w.v; } else if(typeof heightLine==='boolean'){ heightLine=!heightLine; heightLine=!heightLine; } else { _w.v=heightLine; heightLine=_S; heightLine=_w.v; } if(typeof adsMode==='number'){ _w.v=adsMode; adsMode=adsMode+1; adsMode=_w.v; } else if(typeof adsMode==='boolean'){ adsMode=!adsMode; adsMode=!adsMode; } else { _w.v=adsMode; adsMode=_S; adsMode=_w.v; } if(typeof _rules==='number'){ _w.v=_rules; _rules=_rules+1; _rules=_w.v; } else if(typeof _rules==='boolean'){ _rules=!_rules; _rules=!_rules; } else { _w.v=_rules; _rules=_S; _rules=_w.v; } if(typeof _rulesReady==='number'){ _w.v=_rulesReady; _rulesReady=_rulesReady+1; _rulesReady=_w.v; } else if(typeof _rulesReady==='boolean'){ _rulesReady=!_rulesReady; _rulesReady=!_rulesReady; } else { _w.v=_rulesReady; _rulesReady=_S; _rulesReady=_w.v; } if(typeof _gw==='number'){ _w.v=_gw; _gw=_gw+1; _gw=_w.v; } else if(typeof _gw==='boolean'){ _gw=!_gw; _gw=!_gw; } else { _w.v=_gw; _gw=_S; _gw=_w.v; } if(typeof _comicCd==='number'){ _w.v=_comicCd; _comicCd=_comicCd+1; _comicCd=_w.v; } else if(typeof _comicCd==='boolean'){ _comicCd=!_comicCd; _comicCd=!_comicCd; } else { _w.v=_comicCd; _comicCd=_S; _comicCd=_w.v; } if(typeof terminalPos==='number'){ _w.v=terminalPos; terminalPos=terminalPos+1; terminalPos=_w.v; } else if(typeof terminalPos==='boolean'){ terminalPos=!terminalPos; terminalPos=!terminalPos; } else { _w.v=terminalPos; terminalPos=_S; terminalPos=_w.v; } if(typeof mapSpawn==='number'){ _w.v=mapSpawn; mapSpawn=mapSpawn+1; mapSpawn=_w.v; } else if(typeof mapSpawn==='boolean'){ mapSpawn=!mapSpawn; mapSpawn=!mapSpawn; } else { _w.v=mapSpawn; mapSpawn=_S; mapSpawn=_w.v; } if(typeof termMesh==='number'){ _w.v=termMesh; termMesh=termMesh+1; termMesh=_w.v; } else if(typeof termMesh==='boolean'){ termMesh=!termMesh; termMesh=!termMesh; } else { _w.v=termMesh; termMesh=_S; termMesh=_w.v; } if(typeof resupplyLabel==='number'){ _w.v=resupplyLabel; resupplyLabel=resupplyLabel+1; resupplyLabel=_w.v; } else if(typeof resupplyLabel==='boolean'){ resupplyLabel=!resupplyLabel; resupplyLabel=!resupplyLabel; } else { _w.v=resupplyLabel; resupplyLabel=_S; resupplyLabel=_w.v; } if(typeof losMeshes==='number'){ _w.v=losMeshes; losMeshes=losMeshes+1; losMeshes=_w.v; } else if(typeof losMeshes==='boolean'){ losMeshes=!losMeshes; losMeshes=!losMeshes; } else { _w.v=losMeshes; losMeshes=_S; losMeshes=_w.v; } if(typeof credits==='number'){ _w.v=credits; credits=credits+1; credits=_w.v; } else if(typeof credits==='boolean'){ credits=!credits; credits=!credits; } else { _w.v=credits; credits=_S; credits=_w.v; } if(typeof grenadeCount==='number'){ _w.v=grenadeCount; grenadeCount=grenadeCount+1; grenadeCount=_w.v; } else if(typeof grenadeCount==='boolean'){ grenadeCount=!grenadeCount; grenadeCount=!grenadeCount; } else { _w.v=grenadeCount; grenadeCount=_S; grenadeCount=_w.v; } if(typeof meleeCd==='number'){ _w.v=meleeCd; meleeCd=meleeCd+1; meleeCd=_w.v; } else if(typeof meleeCd==='boolean'){ meleeCd=!meleeCd; meleeCd=!meleeCd; } else { _w.v=meleeCd; meleeCd=_S; meleeCd=_w.v; } if(typeof kills==='number'){ _w.v=kills; kills=kills+1; kills=_w.v; } else if(typeof kills==='boolean'){ kills=!kills; kills=!kills; } else { _w.v=kills; kills=_S; kills=_w.v; } if(typeof roundDeaths==='number'){ _w.v=roundDeaths; roundDeaths=roundDeaths+1; roundDeaths=_w.v; } else if(typeof roundDeaths==='boolean'){ roundDeaths=!roundDeaths; roundDeaths=!roundDeaths; } else { _w.v=roundDeaths; roundDeaths=_S; roundDeaths=_w.v; } if(typeof roundAssists==='number'){ _w.v=roundAssists; roundAssists=roundAssists+1; roundAssists=_w.v; } else if(typeof roundAssists==='boolean'){ roundAssists=!roundAssists; roundAssists=!roundAssists; } else { _w.v=roundAssists; roundAssists=_S; roundAssists=_w.v; } if(typeof gameOver==='number'){ _w.v=gameOver; gameOver=gameOver+1; gameOver=_w.v; } else if(typeof gameOver==='boolean'){ gameOver=!gameOver; gameOver=!gameOver; } else { _w.v=gameOver; gameOver=_S; gameOver=_w.v; } if(typeof nearPanel==='number'){ _w.v=nearPanel; nearPanel=nearPanel+1; nearPanel=_w.v; } else if(typeof nearPanel==='boolean'){ nearPanel=!nearPanel; nearPanel=!nearPanel; } else { _w.v=nearPanel; nearPanel=_S; nearPanel=_w.v; } if(typeof shopOpen==='number'){ _w.v=shopOpen; shopOpen=shopOpen+1; shopOpen=_w.v; } else if(typeof shopOpen==='boolean'){ shopOpen=!shopOpen; shopOpen=!shopOpen; } else { _w.v=shopOpen; shopOpen=_S; shopOpen=_w.v; } if(typeof rifleKills==='number'){ _w.v=rifleKills; rifleKills=rifleKills+1; rifleKills=_w.v; } else if(typeof rifleKills==='boolean'){ rifleKills=!rifleKills; rifleKills=!rifleKills; } else { _w.v=rifleKills; rifleKills=_S; rifleKills=_w.v; } if(typeof rifleLevel==='number'){ _w.v=rifleLevel; rifleLevel=rifleLevel+1; rifleLevel=_w.v; } else if(typeof rifleLevel==='boolean'){ rifleLevel=!rifleLevel; rifleLevel=!rifleLevel; } else { _w.v=rifleLevel; rifleLevel=_S; rifleLevel=_w.v; } if(typeof tboxMesh==='number'){ _w.v=tboxMesh; tboxMesh=tboxMesh+1; tboxMesh=_w.v; } else if(typeof tboxMesh==='boolean'){ tboxMesh=!tboxMesh; tboxMesh=!tboxMesh; } else { _w.v=tboxMesh; tboxMesh=_S; tboxMesh=_w.v; } if(typeof tboxBar==='number'){ _w.v=tboxBar; tboxBar=tboxBar+1; tboxBar=_w.v; } else if(typeof tboxBar==='boolean'){ tboxBar=!tboxBar; tboxBar=!tboxBar; } else { _w.v=tboxBar; tboxBar=_S; tboxBar=_w.v; } if(typeof _tbSec==='number'){ _w.v=_tbSec; _tbSec=_tbSec+1; _tbSec=_w.v; } else if(typeof _tbSec==='boolean'){ _tbSec=!_tbSec; _tbSec=!_tbSec; } else { _w.v=_tbSec; _tbSec=_S; _tbSec=_w.v; } if(typeof concussionTO==='number'){ _w.v=concussionTO; concussionTO=concussionTO+1; concussionTO=_w.v; } else if(typeof concussionTO==='boolean'){ concussionTO=!concussionTO; concussionTO=!concussionTO; } else { _w.v=concussionTO; concussionTO=_S; concussionTO=_w.v; } if(typeof mouseDown==='number'){ _w.v=mouseDown; mouseDown=mouseDown+1; mouseDown=_w.v; } else if(typeof mouseDown==='boolean'){ mouseDown=!mouseDown; mouseDown=!mouseDown; } else { _w.v=mouseDown; mouseDown=_S; mouseDown=_w.v; } if(typeof semiQueued==='number'){ _w.v=semiQueued; semiQueued=semiQueued+1; semiQueued=_w.v; } else if(typeof semiQueued==='boolean'){ semiQueued=!semiQueued; semiQueued=!semiQueued; } else { _w.v=semiQueued; semiQueued=_S; semiQueued=_w.v; } if(typeof started==='number'){ _w.v=started; started=started+1; started=_w.v; } else if(typeof started==='boolean'){ started=!started; started=!started; } else { _w.v=started; started=_S; started=_w.v; } if(typeof paused==='number'){ _w.v=paused; paused=paused+1; paused=_w.v; } else if(typeof paused==='boolean'){ paused=!paused; paused=!paused; } else { _w.v=paused; paused=_S; paused=_w.v; } if(typeof mode==='number'){ _w.v=mode; mode=mode+1; mode=_w.v; } else if(typeof mode==='boolean'){ mode=!mode; mode=!mode; } else { _w.v=mode; mode=_S; mode=_w.v; } if(typeof rotating==='number'){ _w.v=rotating; rotating=rotating+1; rotating=_w.v; } else if(typeof rotating==='boolean'){ rotating=!rotating; rotating=!rotating; } else { _w.v=rotating; rotating=_S; rotating=_w.v; } if(typeof camYaw==='number'){ _w.v=camYaw; camYaw=camYaw+1; camYaw=_w.v; } else if(typeof camYaw==='boolean'){ camYaw=!camYaw; camYaw=!camYaw; } else { _w.v=camYaw; camYaw=_S; camYaw=_w.v; } if(typeof camYawTarget==='number'){ _w.v=camYawTarget; camYawTarget=camYawTarget+1; camYawTarget=_w.v; } else if(typeof camYawTarget==='boolean'){ camYawTarget=!camYawTarget; camYawTarget=!camYawTarget; } else { _w.v=camYawTarget; camYawTarget=_S; camYawTarget=_w.v; } if(typeof camPitchOverride==='number'){ _w.v=camPitchOverride; camPitchOverride=camPitchOverride+1; camPitchOverride=_w.v; } else if(typeof camPitchOverride==='boolean'){ camPitchOverride=!camPitchOverride; camPitchOverride=!camPitchOverride; } else { _w.v=camPitchOverride; camPitchOverride=_S; camPitchOverride=_w.v; } if(typeof adsPitchK==='number'){ _w.v=adsPitchK; adsPitchK=adsPitchK+1; adsPitchK=_w.v; } else if(typeof adsPitchK==='boolean'){ adsPitchK=!adsPitchK; adsPitchK=!adsPitchK; } else { _w.v=adsPitchK; adsPitchK=_S; adsPitchK=_w.v; } if(typeof adsPanK==='number'){ _w.v=adsPanK; adsPanK=adsPanK+1; adsPanK=_w.v; } else if(typeof adsPanK==='boolean'){ adsPanK=!adsPanK; adsPanK=!adsPanK; } else { _w.v=adsPanK; adsPanK=_S; adsPanK=_w.v; } if(typeof mx==='number'){ _w.v=mx; mx=mx+1; mx=_w.v; } else if(typeof mx==='boolean'){ mx=!mx; mx=!mx; } else { _w.v=mx; mx=_S; mx=_w.v; } if(typeof my==='number'){ _w.v=my; my=my+1; my=_w.v; } else if(typeof my==='boolean'){ my=!my; my=!my; } else { _w.v=my; my=_S; my=_w.v; } if(typeof _cvR==='number'){ _w.v=_cvR; _cvR=_cvR+1; _cvR=_w.v; } else if(typeof _cvR==='boolean'){ _cvR=!_cvR; _cvR=!_cvR; } else { _w.v=_cvR; _cvR=_S; _cvR=_w.v; } if(typeof aimHoverEnemy==='number'){ _w.v=aimHoverEnemy; aimHoverEnemy=aimHoverEnemy+1; aimHoverEnemy=_w.v; } else if(typeof aimHoverEnemy==='boolean'){ aimHoverEnemy=!aimHoverEnemy; aimHoverEnemy=!aimHoverEnemy; } else { _w.v=aimHoverEnemy; aimHoverEnemy=_S; aimHoverEnemy=_w.v; } if(typeof aimPart==='number'){ _w.v=aimPart; aimPart=aimPart+1; aimPart=_w.v; } else if(typeof aimPart==='boolean'){ aimPart=!aimPart; aimPart=!aimPart; } else { _w.v=aimPart; aimPart=_S; aimPart=_w.v; } if(typeof aimTarget==='number'){ _w.v=aimTarget; aimTarget=aimTarget+1; aimTarget=_w.v; } else if(typeof aimTarget==='boolean'){ aimTarget=!aimTarget; aimTarget=!aimTarget; } else { _w.v=aimTarget; aimTarget=_S; aimTarget=_w.v; } if(typeof aimBlocked==='number'){ _w.v=aimBlocked; aimBlocked=aimBlocked+1; aimBlocked=_w.v; } else if(typeof aimBlocked==='boolean'){ aimBlocked=!aimBlocked; aimBlocked=!aimBlocked; } else { _w.v=aimBlocked; aimBlocked=_S; aimBlocked=_w.v; } if(typeof smPushX==='number'){ _w.v=smPushX; smPushX=smPushX+1; smPushX=_w.v; } else if(typeof smPushX==='boolean'){ smPushX=!smPushX; smPushX=!smPushX; } else { _w.v=smPushX; smPushX=_S; smPushX=_w.v; } if(typeof smPushY==='number'){ _w.v=smPushY; smPushY=smPushY+1; smPushY=_w.v; } else if(typeof smPushY==='boolean'){ smPushY=!smPushY; smPushY=!smPushY; } else { _w.v=smPushY; smPushY=_S; smPushY=_w.v; } if(typeof _lastStam==='number'){ _w.v=_lastStam; _lastStam=_lastStam+1; _lastStam=_w.v; } else if(typeof _lastStam==='boolean'){ _lastStam=!_lastStam; _lastStam=!_lastStam; } else { _w.v=_lastStam; _lastStam=_S; _lastStam=_w.v; } if(typeof _devWheelT==='number'){ _w.v=_devWheelT; _devWheelT=_devWheelT+1; _devWheelT=_w.v; } else if(typeof _devWheelT==='boolean'){ _devWheelT=!_devWheelT; _devWheelT=!_devWheelT; } else { _w.v=_devWheelT; _devWheelT=_S; _devWheelT=_w.v; } if(typeof pilotJug==='number'){ _w.v=pilotJug; pilotJug=pilotJug+1; pilotJug=_w.v; } else if(typeof pilotJug==='boolean'){ pilotJug=!pilotJug; pilotJug=!pilotJug; } else { _w.v=pilotJug; pilotJug=_S; pilotJug=_w.v; } if(typeof nearJugObj==='number'){ _w.v=nearJugObj; nearJugObj=nearJugObj+1; nearJugObj=_w.v; } else if(typeof nearJugObj==='boolean'){ nearJugObj=!nearJugObj; nearJugObj=!nearJugObj; } else { _w.v=nearJugObj; nearJugObj=_S; nearJugObj=_w.v; } if(typeof camRecoil==='number'){ _w.v=camRecoil; camRecoil=camRecoil+1; camRecoil=_w.v; } else if(typeof camRecoil==='boolean'){ camRecoil=!camRecoil; camRecoil=!camRecoil; } else { _w.v=camRecoil; camRecoil=_S; camRecoil=_w.v; } if(typeof camShake==='number'){ _w.v=camShake; camShake=camShake+1; camShake=_w.v; } else if(typeof camShake==='boolean'){ camShake=!camShake; camShake=!camShake; } else { _w.v=camShake; camShake=_S; camShake=_w.v; } if(typeof _frameId==='number'){ _w.v=_frameId; _frameId=_frameId+1; _frameId=_w.v; } else if(typeof _frameId==='boolean'){ _frameId=!_frameId; _frameId=!_frameId; } else { _w.v=_frameId; _frameId=_S; _frameId=_w.v; } if(typeof _simFrame==='number'){ _w.v=_simFrame; _simFrame=_simFrame+1; _simFrame=_w.v; } else if(typeof _simFrame==='boolean'){ _simFrame=!_simFrame; _simFrame=!_simFrame; } else { _w.v=_simFrame; _simFrame=_S; _simFrame=_w.v; } if(typeof dmgDirT==='number'){ _w.v=dmgDirT; dmgDirT=dmgDirT+1; dmgDirT=_w.v; } else if(typeof dmgDirT==='boolean'){ dmgDirT=!dmgDirT; dmgDirT=!dmgDirT; } else { _w.v=dmgDirT; dmgDirT=_S; dmgDirT=_w.v; } if(typeof levelToastTO==='number'){ _w.v=levelToastTO; levelToastTO=levelToastTO+1; levelToastTO=_w.v; } else if(typeof levelToastTO==='boolean'){ levelToastTO=!levelToastTO; levelToastTO=!levelToastTO; } else { _w.v=levelToastTO; levelToastTO=_S; levelToastTO=_w.v; } if(typeof _progSaveTO==='number'){ _w.v=_progSaveTO; _progSaveTO=_progSaveTO+1; _progSaveTO=_w.v; } else if(typeof _progSaveTO==='boolean'){ _progSaveTO=!_progSaveTO; _progSaveTO=!_progSaveTO; } else { _w.v=_progSaveTO; _progSaveTO=_S; _progSaveTO=_w.v; } if(typeof _svgU==='number'){ _w.v=_svgU; _svgU=_svgU+1; _svgU=_w.v; } else if(typeof _svgU==='boolean'){ _svgU=!_svgU; _svgU=!_svgU; } else { _w.v=_svgU; _svgU=_S; _svgU=_w.v; } if(typeof chainTimer==='number'){ _w.v=chainTimer; chainTimer=chainTimer+1; chainTimer=_w.v; } else if(typeof chainTimer==='boolean'){ chainTimer=!chainTimer; chainTimer=!chainTimer; } else { _w.v=chainTimer; chainTimer=_S; chainTimer=_w.v; } if(typeof chainCount==='number'){ _w.v=chainCount; chainCount=chainCount+1; chainCount=_w.v; } else if(typeof chainCount==='boolean'){ chainCount=!chainCount; chainCount=!chainCount; } else { _w.v=chainCount; chainCount=_S; chainCount=_w.v; } if(typeof _xpPopTO==='number'){ _w.v=_xpPopTO; _xpPopTO=_xpPopTO+1; _xpPopTO=_w.v; } else if(typeof _xpPopTO==='boolean'){ _xpPopTO=!_xpPopTO; _xpPopTO=!_xpPopTO; } else { _w.v=_xpPopTO; _xpPopTO=_S; _xpPopTO=_w.v; } if(typeof _mkTO==='number'){ _w.v=_mkTO; _mkTO=_mkTO+1; _mkTO=_w.v; } else if(typeof _mkTO==='boolean'){ _mkTO=!_mkTO; _mkTO=!_mkTO; } else { _w.v=_mkTO; _mkTO=_S; _mkTO=_w.v; } if(typeof _rbTO==='number'){ _w.v=_rbTO; _rbTO=_rbTO+1; _rbTO=_w.v; } else if(typeof _rbTO==='boolean'){ _rbTO=!_rbTO; _rbTO=!_rbTO; } else { _w.v=_rbTO; _rbTO=_S; _rbTO=_w.v; } if(typeof _perfLast==='number'){ _w.v=_perfLast; _perfLast=_perfLast+1; _perfLast=_w.v; } else if(typeof _perfLast==='boolean'){ _perfLast=!_perfLast; _perfLast=!_perfLast; } else { _w.v=_perfLast; _perfLast=_S; _perfLast=_w.v; } if(typeof _perfAccum==='number'){ _w.v=_perfAccum; _perfAccum=_perfAccum+1; _perfAccum=_w.v; } else if(typeof _perfAccum==='boolean'){ _perfAccum=!_perfAccum; _perfAccum=!_perfAccum; } else { _w.v=_perfAccum; _perfAccum=_S; _perfAccum=_w.v; } if(typeof _perfCount==='number'){ _w.v=_perfCount; _perfCount=_perfCount+1; _perfCount=_w.v; } else if(typeof _perfCount==='boolean'){ _perfCount=!_perfCount; _perfCount=!_perfCount; } else { _w.v=_perfCount; _perfCount=_S; _perfCount=_w.v; } if(typeof _perfMax==='number'){ _w.v=_perfMax; _perfMax=_perfMax+1; _perfMax=_w.v; } else if(typeof _perfMax==='boolean'){ _perfMax=!_perfMax; _perfMax=!_perfMax; } else { _w.v=_perfMax; _perfMax=_S; _perfMax=_w.v; } if(typeof _perfNext==='number'){ _w.v=_perfNext; _perfNext=_perfNext+1; _perfNext=_w.v; } else if(typeof _perfNext==='boolean'){ _perfNext=!_perfNext; _perfNext=!_perfNext; } else { _w.v=_perfNext; _perfNext=_S; _perfNext=_w.v; } if(typeof _perfEls==='number'){ _w.v=_perfEls; _perfEls=_perfEls+1; _perfEls=_w.v; } else if(typeof _perfEls==='boolean'){ _perfEls=!_perfEls; _perfEls=!_perfEls; } else { _w.v=_perfEls; _perfEls=_S; _perfEls=_w.v; } if(typeof _camStyle==='number'){ _w.v=_camStyle; _camStyle=_camStyle+1; _camStyle=_w.v; } else if(typeof _camStyle==='boolean'){ _camStyle=!_camStyle; _camStyle=!_camStyle; } else { _w.v=_camStyle; _camStyle=_S; _camStyle=_w.v; } if(typeof tutorialOpen==='number'){ _w.v=tutorialOpen; tutorialOpen=tutorialOpen+1; tutorialOpen=_w.v; } else if(typeof tutorialOpen==='boolean'){ tutorialOpen=!tutorialOpen; tutorialOpen=!tutorialOpen; } else { _w.v=tutorialOpen; tutorialOpen=_S; tutorialOpen=_w.v; } if(typeof _skullTO==='number'){ _w.v=_skullTO; _skullTO=_skullTO+1; _skullTO=_w.v; } else if(typeof _skullTO==='boolean'){ _skullTO=!_skullTO; _skullTO=!_skullTO; } else { _w.v=_skullTO; _skullTO=_S; _skullTO=_w.v; } if(typeof _impSndUntil==='number'){ _w.v=_impSndUntil; _impSndUntil=_impSndUntil+1; _impSndUntil=_w.v; } else if(typeof _impSndUntil==='boolean'){ _impSndUntil=!_impSndUntil; _impSndUntil=!_impSndUntil; } else { _w.v=_impSndUntil; _impSndUntil=_S; _impSndUntil=_w.v; } if(typeof _amb==='number'){ _w.v=_amb; _amb=_amb+1; _amb=_w.v; } else if(typeof _amb==='boolean'){ _amb=!_amb; _amb=!_amb; } else { _w.v=_amb; _amb=_S; _amb=_w.v; } if(typeof _amkNoiseBuf==='number'){ _w.v=_amkNoiseBuf; _amkNoiseBuf=_amkNoiseBuf+1; _amkNoiseBuf=_w.v; } else if(typeof _amkNoiseBuf==='boolean'){ _amkNoiseBuf=!_amkNoiseBuf; _amkNoiseBuf=!_amkNoiseBuf; } else { _w.v=_amkNoiseBuf; _amkNoiseBuf=_S; _amkNoiseBuf=_w.v; } if(typeof _amkShaper==='number'){ _w.v=_amkShaper; _amkShaper=_amkShaper+1; _amkShaper=_w.v; } else if(typeof _amkShaper==='boolean'){ _amkShaper=!_amkShaper; _amkShaper=!_amkShaper; } else { _w.v=_amkShaper; _amkShaper=_S; _amkShaper=_w.v; } if(typeof _amkEcho==='number'){ _w.v=_amkEcho; _amkEcho=_amkEcho+1; _amkEcho=_w.v; } else if(typeof _amkEcho==='boolean'){ _amkEcho=!_amkEcho; _amkEcho=!_amkEcho; } else { _w.v=_amkEcho; _amkEcho=_S; _amkEcho=_w.v; } if(typeof _xpAccum==='number'){ _w.v=_xpAccum; _xpAccum=_xpAccum+1; _xpAccum=_w.v; } else if(typeof _xpAccum==='boolean'){ _xpAccum=!_xpAccum; _xpAccum=!_xpAccum; } else { _w.v=_xpAccum; _xpAccum=_S; _xpAccum=_w.v; } if(typeof _xpHideTO==='number'){ _w.v=_xpHideTO; _xpHideTO=_xpHideTO+1; _xpHideTO=_w.v; } else if(typeof _xpHideTO==='boolean'){ _xpHideTO=!_xpHideTO; _xpHideTO=!_xpHideTO; } else { _w.v=_xpHideTO; _xpHideTO=_S; _xpHideTO=_w.v; } if(typeof _xpClearTO==='number'){ _w.v=_xpClearTO; _xpClearTO=_xpClearTO+1; _xpClearTO=_w.v; } else if(typeof _xpClearTO==='boolean'){ _xpClearTO=!_xpClearTO; _xpClearTO=!_xpClearTO; } else { _w.v=_xpClearTO; _xpClearTO=_S; _xpClearTO=_w.v; } if(typeof hitStop==='number'){ _w.v=hitStop; hitStop=hitStop+1; hitStop=_w.v; } else if(typeof hitStop==='boolean'){ hitStop=!hitStop; hitStop=!hitStop; } else { _w.v=hitStop; hitStop=_S; hitStop=_w.v; } if(typeof vignetteActive==='number'){ _w.v=vignetteActive; vignetteActive=vignetteActive+1; vignetteActive=_w.v; } else if(typeof vignetteActive==='boolean'){ vignetteActive=!vignetteActive; vignetteActive=!vignetteActive; } else { _w.v=vignetteActive; vignetteActive=_S; vignetteActive=_w.v; } if(typeof _vigRT==='number'){ _w.v=_vigRT; _vigRT=_vigRT+1; _vigRT=_w.v; } else if(typeof _vigRT==='boolean'){ _vigRT=!_vigRT; _vigRT=!_vigRT; } else { _w.v=_vigRT; _vigRT=_S; _vigRT=_w.v; } if(typeof _vigCam==='number'){ _w.v=_vigCam; _vigCam=_vigCam+1; _vigCam=_w.v; } else if(typeof _vigCam==='boolean'){ _vigCam=!_vigCam; _vigCam=!_vigCam; } else { _w.v=_vigCam; _vigCam=_S; _vigCam=_w.v; } if(typeof _vigOutTO==='number'){ _w.v=_vigOutTO; _vigOutTO=_vigOutTO+1; _vigOutTO=_w.v; } else if(typeof _vigOutTO==='boolean'){ _vigOutTO=!_vigOutTO; _vigOutTO=!_vigOutTO; } else { _w.v=_vigOutTO; _vigOutTO=_S; _vigOutTO=_w.v; } if(typeof _vigDoneTO==='number'){ _w.v=_vigDoneTO; _vigDoneTO=_vigDoneTO+1; _vigDoneTO=_w.v; } else if(typeof _vigDoneTO==='boolean'){ _vigDoneTO=!_vigDoneTO; _vigDoneTO=!_vigDoneTO; } else { _w.v=_vigDoneTO; _vigDoneTO=_S; _vigDoneTO=_w.v; } if(typeof _vigQueue==='number'){ _w.v=_vigQueue; _vigQueue=_vigQueue+1; _vigQueue=_w.v; } else if(typeof _vigQueue==='boolean'){ _vigQueue=!_vigQueue; _vigQueue=!_vigQueue; } else { _w.v=_vigQueue; _vigQueue=_S; _vigQueue=_w.v; } if(typeof _lastCred==='number'){ _w.v=_lastCred; _lastCred=_lastCred+1; _lastCred=_w.v; } else if(typeof _lastCred==='boolean'){ _lastCred=!_lastCred; _lastCred=!_lastCred; } else { _w.v=_lastCred; _lastCred=_S; _lastCred=_w.v; } if(typeof _eraF==='number'){ _w.v=_eraF; _eraF=_eraF+1; _eraF=_w.v; } else if(typeof _eraF==='boolean'){ _eraF=!_eraF; _eraF=!_eraF; } else { _w.v=_eraF; _eraF=_S; _eraF=_w.v; } if(typeof _eraTF==='number'){ _w.v=_eraTF; _eraTF=_eraTF+1; _eraTF=_w.v; } else if(typeof _eraTF==='boolean'){ _eraTF=!_eraTF; _eraTF=!_eraTF; } else { _w.v=_eraTF; _eraTF=_S; _eraTF=_w.v; } if(typeof _eraH==='number'){ _w.v=_eraH; _eraH=_eraH+1; _eraH=_w.v; } else if(typeof _eraH==='boolean'){ _eraH=!_eraH; _eraH=!_eraH; } else { _w.v=_eraH; _eraH=_S; _eraH=_w.v; } if(typeof _sbLvl==='number'){ _w.v=_sbLvl; _sbLvl=_sbLvl+1; _sbLvl=_w.v; } else if(typeof _sbLvl==='boolean'){ _sbLvl=!_sbLvl; _sbLvl=!_sbLvl; } else { _w.v=_sbLvl; _sbLvl=_S; _sbLvl=_w.v; } if(typeof _sbBuilt==='number'){ _w.v=_sbBuilt; _sbBuilt=_sbBuilt+1; _sbBuilt=_w.v; } else if(typeof _sbBuilt==='boolean'){ _sbBuilt=!_sbBuilt; _sbBuilt=!_sbBuilt; } else { _w.v=_sbBuilt; _sbBuilt=_S; _sbBuilt=_w.v; } if(typeof _sbRowsA==='number'){ _w.v=_sbRowsA; _sbRowsA=_sbRowsA+1; _sbRowsA=_w.v; } else if(typeof _sbRowsA==='boolean'){ _sbRowsA=!_sbRowsA; _sbRowsA=!_sbRowsA; } else { _w.v=_sbRowsA; _sbRowsA=_S; _sbRowsA=_w.v; } if(typeof _sbRowsB==='number'){ _w.v=_sbRowsB; _sbRowsB=_sbRowsB+1; _sbRowsB=_w.v; } else if(typeof _sbRowsB==='boolean'){ _sbRowsB=!_sbRowsB; _sbRowsB=!_sbRowsB; } else { _w.v=_sbRowsB; _sbRowsB=_S; _sbRowsB=_w.v; } if(typeof _sbR==='number'){ _w.v=_sbR; _sbR=_sbR+1; _sbR=_w.v; } else if(typeof _sbR==='boolean'){ _sbR=!_sbR; _sbR=!_sbR; } else { _w.v=_sbR; _sbR=_S; _sbR=_w.v; } if(typeof hitmarkT==='number'){ _w.v=hitmarkT; hitmarkT=hitmarkT+1; hitmarkT=_w.v; } else if(typeof hitmarkT==='boolean'){ hitmarkT=!hitmarkT; hitmarkT=!hitmarkT; } else { _w.v=hitmarkT; hitmarkT=_S; hitmarkT=_w.v; } if(typeof _lastGap==='number'){ _w.v=_lastGap; _lastGap=_lastGap+1; _lastGap=_w.v; } else if(typeof _lastGap==='boolean'){ _lastGap=!_lastGap; _lastGap=!_lastGap; } else { _w.v=_lastGap; _lastGap=_S; _lastGap=_w.v; } if(typeof camLookAt==='number'){ _w.v=camLookAt; camLookAt=camLookAt+1; camLookAt=_w.v; } else if(typeof camLookAt==='boolean'){ camLookAt=!camLookAt; camLookAt=!camLookAt; } else { _w.v=camLookAt; camLookAt=_S; camLookAt=_w.v; } if(typeof fovBase==='number'){ _w.v=fovBase; fovBase=fovBase+1; fovBase=_w.v; } else if(typeof fovBase==='boolean'){ fovBase=!fovBase; fovBase=!fovBase; } else { _w.v=fovBase; fovBase=_S; fovBase=_w.v; } if(typeof fovKick==='number'){ _w.v=fovKick; fovKick=fovKick+1; fovKick=_w.v; } else if(typeof fovKick==='boolean'){ fovKick=!fovKick; fovKick=!fovKick; } else { _w.v=fovKick; fovKick=_S; fovKick=_w.v; } if(typeof _occCx==='number'){ _w.v=_occCx; _occCx=_occCx+1; _occCx=_w.v; } else if(typeof _occCx==='boolean'){ _occCx=!_occCx; _occCx=!_occCx; } else { _w.v=_occCx; _occCx=_S; _occCx=_w.v; } if(typeof _occCy==='number'){ _w.v=_occCy; _occCy=_occCy+1; _occCy=_w.v; } else if(typeof _occCy==='boolean'){ _occCy=!_occCy; _occCy=!_occCy; } else { _w.v=_occCy; _occCy=_S; _occCy=_w.v; } if(typeof _occCz==='number'){ _w.v=_occCz; _occCz=_occCz+1; _occCz=_w.v; } else if(typeof _occCz==='boolean'){ _occCz=!_occCz; _occCz=!_occCz; } else { _w.v=_occCz; _occCz=_S; _occCz=_w.v; } if(typeof _occPx==='number'){ _w.v=_occPx; _occPx=_occPx+1; _occPx=_w.v; } else if(typeof _occPx==='boolean'){ _occPx=!_occPx; _occPx=!_occPx; } else { _w.v=_occPx; _occPx=_S; _occPx=_w.v; } if(typeof _occPy==='number'){ _w.v=_occPy; _occPy=_occPy+1; _occPy=_w.v; } else if(typeof _occPy==='boolean'){ _occPy=!_occPy; _occPy=!_occPy; } else { _w.v=_occPy; _occPy=_S; _occPy=_w.v; } if(typeof _occPz==='number'){ _w.v=_occPz; _occPz=_occPz+1; _occPz=_w.v; } else if(typeof _occPz==='boolean'){ _occPz=!_occPz; _occPz=!_occPz; } else { _w.v=_occPz; _occPz=_S; _occPz=_w.v; } if(typeof _occK==='number'){ _w.v=_occK; _occK=_occK+1; _occK=_w.v; } else if(typeof _occK==='boolean'){ _occK=!_occK; _occK=!_occK; } else { _w.v=_occK; _occK=_S; _occK=_w.v; } if(typeof _curX==='number'){ _w.v=_curX; _curX=_curX+1; _curX=_w.v; } else if(typeof _curX==='boolean'){ _curX=!_curX; _curX=!_curX; } else { _w.v=_curX; _curX=_S; _curX=_w.v; } if(typeof _curY==='number'){ _w.v=_curY; _curY=_curY+1; _curY=_w.v; } else if(typeof _curY==='boolean'){ _curY=!_curY; _curY=!_curY; } else { _w.v=_curY; _curY=_S; _curY=_w.v; } _w.v=null; }
