/* ============================================================================
 * shamble-core.js — Shamble Frontlines shared core.  See MODULE_MAP.md.
 * PURPOSE:   Engine + shared definitions used by BOTH builds (player + authoring).
 *            The single source of truth below the gameplay/editor layers.
 * PROVIDES:  T (tuning); WEAPONS / UNIT_TYPES / VEHICLE_TYPES; render globals
 *            (scene / camera / renderer / sun / hemiLight / ambLight / ground /
 *            sea / beach); the NPR pipeline (toonify / makeOutlineMat / addHull /
 *            ensureOutlineNormals / stylizeScene / INK); ENV (applyEnv / generateGrassTexture /
 *            ENV_DEFAULT); the character builder (buildEntity / buildCharacter /
 *            assembleChar / applyFaction / FACTION_PALETTES / CHAR_* / CPARTS /
 *            poseArms); the building kit (BK namespace + pure makers
 *            makeTimberHouse / makeThatchCottage / makeBrickFlintFarmhouse /
 *            makeManorHouse + _bkWorldBoxes); the map schema (cabanaRepublicMap /
 *            parseGridToMapDef / DM_Cabana / DM_Romania); weapon-prop makers
 *            (makeSword / makeAxe / makeShield / makeAMKRifle) + the gun kit (GUN_KIT /
 *            buildGunModel: one model + sockets per gun); the persistence
 *            adapter KVStore (window.storage -> localStorage -> memory, picked
 *            once at load); and the headless
 *            SIM math (FIXED_DT / BTN / BODY / PART / raySphere / rayCapsule /
 *            rayAabb / resolveShot / SIM / hasLOS), fenced at the bottom.
 * REQUIRES:  THREE (r128 UMD global) + DOM #app. References nothing above it.
 * LOAD ORDER: THREE -> shamble-core.js -> shamble-game.js -> [shamble-editor.js].
 * SIDE-EFFECTS ON LOAD: creates renderer / scene / camera / lights + ground /
 *            sea / beach; KVStore makes one read-only storage probe.
 * NOTE: NOT headless-runnable today (owns the render globals); the SIM block is
 *       fenced contiguously for a clean future Phase-6 headless lift-out.
 * ========================================================================== */
// ---- KVStore: the one persistence adapter (rank / tutorial / maps / prefabs / charlab) ----
// Async key-value API shaped like window.storage: get(k) -> {key,value}|null, set(k,v),
// delete(k), list(prefix) -> {keys,prefix}. Backend picked ONCE at load:
//   window.storage (claude.ai artifact runtime) -> localStorage (file:// and normal
//   browsers) -> memory (this session only).
// Never rejects: failures resolve to null, so un-awaited saves can't raise unhandled
// rejections. A localStorage write that fails (e.g. quota) is kept in memory for the session.
const KVStore=(function(){
  const W=(typeof window!=='undefined')?window:null;
  const ws=(W&&W.storage&&typeof W.storage.get==='function'&&typeof W.storage.set==='function')?W.storage:null;
  let ls=null;
  if(!ws){ try{ const L=W&&W.localStorage; if(L){ L.getItem('shamble_kv_probe'); ls=L; } }catch(e){ ls=null; } }
  const mem=Object.create(null);
  const backend=ws?'window.storage':(ls?'localStorage':'memory');
  return { backend:backend, persist:backend!=='memory',
    async get(k){
      if(ws){ try{ return (await ws.get(k))||null; }catch(e){ return null; } }
      if(k in mem) return {key:k,value:mem[k]};
      if(ls){ try{ const v=ls.getItem(k); if(v!==null) return {key:k,value:v}; }catch(e){} }
      return null; },
    async set(k,v){
      if(ws){ try{ return (await ws.set(k,v))||null; }catch(e){ return null; } }
      v=String(v);
      if(ls){ try{ ls.setItem(k,v); delete mem[k]; return {key:k,value:v}; }catch(e){} }
      mem[k]=v; return {key:k,value:v}; },
    async delete(k){
      if(ws){ try{ return (await ws.delete(k))||null; }catch(e){ return null; } }
      delete mem[k];
      if(ls){ try{ ls.removeItem(k); }catch(e){} }
      return {key:k,deleted:true}; },
    async list(prefix){
      if(ws){ try{ return (await ws.list(prefix))||null; }catch(e){ return null; } }
      const p=prefix||'', keys=new Set();
      for(const k in mem) if(k.startsWith(p)) keys.add(k);
      if(ls){ try{ for(let i=0;i<ls.length;i++){ const k=ls.key(i); if(k!==null&&k.startsWith(p)) keys.add(k); } }catch(e){} }
      return {keys:[...keys],prefix:p}; }
  };
})();
const T = {
  moveSpeed:6.3, sprintMult:1.4286, accel:14, arenaX:60, arenaZ:60,
  camHeight:40, camBack:56.6, camPosLean:0.18, camLookLean:0.55, maxLean:26, camLerp:7, lookLerp:9, fov:15, fovADS:15, camYawBase:0.7853981633974483, camYawSens:0.006, camYawLerp:14,
  // camera DEAD ZONE (screen-space): the camera ignores crosshair movement inside this
  // centred box (NDC half-extents) and only eases to pan as the crosshair nears the edge.
  camDeadX:0.60, camDeadY:0.52, camPushLerp:9, adsCam:'pitch', foxPanMax:16,
  // hipfire (no-ADS) framing: player seated below center + forward view CAPPED (matches the screenshot).
  // ADS reuses the params above (camLookLean/camPosLean/maxLean) = today's exact forward reach. Do not invent new ADS distances.
  lookAheadBase:5.5, lookAheadFloor:3.5, lookAheadHip:8.0, lookFwdGain:0.55, lookFwdRange:26, lookBackRange:16,
  lookSideGain:0.55, lookSideMax:6.0, posFwdGain:0.13, posFwdMax:3.2, posBackMax:2.2, posSideGain:0.18, posSideMax:2.0,
  adsMoveMult:0.55,
  playerHealth:100, regenDelay:3.5, regenRate:16, respawnDelay:2.0,
  enemyCount:6, eliteCount:1, eliteHealth:200, enemyHealth:100, enemyRespawn:4.0, enemyDetect:44, enemyPreferRange:22,
  enemyMoveSpeed:4.2, enemyFireInterval:0.22, enemyBurst:1, enemyBurstPause:4.5,
  enemyDamage:34, enemySpread:0.11, enemyRange:60, enemySiegeDamage:11,
  incomeBase:1.5, incomeRefinery:9, killReward:25, resupplyCost:50, startCredits:1000,
  gravity:24, jumpV:11, stepUp:0.55,   // verticality (Phase 5): tune via playtest
  grenadeMax:3, grenadeDamage:140, grenadeRadius:6.5, grenadeBuildingDamage:240,
  meleeDamage:999, meleeRange:3.0, meleeCooldown:0.55,
  shieldCost:350, shieldBlock:0.7, shieldMoveMult:0.72, shieldArcCos:0.30,
  hqHealth:1600, refHealth:1000, barHealth:1000, powerHealth:700,
  // locational damage
  headMult:2.6, bodyMult:1.0, legMult:0.65, weaponPitchMax:0.85,
  // grenade VFX — grounded & gritty: blast + shrapnel + directional concussion
  fragCount:60, fragSpeed:64, fragLaunchAngle:0.42, fragGravity:46, fragRestitution:0.42, fragFriction:0.66, fragLife:1.05, fragChunks:14,
  shockRadius:7.5, dustLife:1.05, scorchRadius:2.4, scorchLife:4.0,
  camKickRange:34, camKickImpulse:24, camKickStiffness:110, camKickDamping:12, concussionStrength:0.85,
};
// Three teams, one per faction (LOOP_V2 §2). Order matters: soviets is the first enemy team of the player's (allies).
const TEAMS = [
  { id:'allies',  faction:'Allies',  name:'Allies',  short:'ALL' },
  { id:'soviets', faction:'Soviets', name:'Soviets', short:'SOV' },
  { id:'empire',  faction:'Empire',  name:'Empire',  short:'EMP' },
];
// The one tunable table for team play (three-teams plan, item 1). Bot sight range stays T.enemyDetect (read directly).
const TUNE = {
  teams: { size:4 },          // soldiers per team, the player included
  ff: false,                  // friendly fire (D5): off = bullets and blasts pass teammates
  ai: { retargetS:0.25, losPerS:480, losRecheckS:0.1, losCapS:0.1, roles:['push','objective','hunt'] },
  spawn: { doorOut:4, lateral:2 },   // bot spawn point outside the own barracks door (D13), metres
  fx: { botCullR:50, botSfxR:90 },   // bot-shot FX and sound culled beyond these camera distances (item 13)
};
const WEAPONS = [
  // 0 — AMK Rifle I (Conscript primary): accurate semi-auto mid-range rifle. The kiting generalist.
  { id:'amk', name:'AMK Rifle I', short:'AMK', era:1, minLevel:1, cost:0, auto:false, fireInterval:0.2, mag:1, reserve:999, pellets:1, spreadHip:0.022, spreadADS:0.006, spreadPerShot:0.0, spreadMax:0.05, spreadDecay:0.3, damage:34, range:300, recoil:2.0, reload:3.5, tracer:0xfff0c0 },
  // 1 — Bagpipe (Peacekeeper primary): close-range scattergun. Pairs with the riot shield.
  { id:'bagpipe', name:'Bagpipe', short:'BAG', era:1, minLevel:1, cost:0, auto:false, fireInterval:0.85, mag:2, reserve:24, pellets:8, spreadHip:0.10, spreadADS:0.05, spreadPerShot:0.0, spreadMax:0.16, spreadDecay:0.5, damage:11, range:34, recoil:4.5, reload:2.6, headMult:1.2, tracer:0xffd9a0 },
  // 2 — Shabzon I (Imperial Warrior primary): high-RoF SMG, big spread — lethal up close, useless at range.
  { id:'shabzon', name:'Shabzon I', short:'SMG', era:1, minLevel:1, cost:0, auto:true, fireInterval:0.072, mag:36, reserve:144, pellets:1, spreadHip:0.075, spreadADS:0.06, spreadPerShot:0.005, spreadMax:0.16, spreadDecay:0.7, damage:8, range:55, recoil:0.3, reload:1.9, falloffStart:15, falloffEnd:35, falloffMin:0.2, tracer:0xfff0c0 },
  // 3 — Molotov (Conscript ability): lobbed incendiary, AoE + bonus vs structures. Unlimited; rifle disabled while equipped.
  { id:'molotov', name:'Molotov', short:'MOLO', era:1, minLevel:1, cost:0, auto:false, fireInterval:1.05, mag:1, reserve:999, pellets:0, spreadHip:0, spreadADS:0, spreadPerShot:0, spreadMax:0, spreadDecay:1, damage:0, range:0, recoil:1.0, reload:0.1, tracer:0 },
  // 4 — Beam Katana (Imperial Warrior Banzai): one-hit melee while charging.
  { id:'katana', name:'Beam Katana', short:'KAT', era:1, minLevel:1, cost:0, melee:true, auto:true, fireInterval:0.45, mag:1, reserve:0, pellets:0, spreadHip:0, spreadADS:0, spreadPerShot:0, spreadMax:0, spreadDecay:1, damage:9999, range:3.6, arcDeg:120, swingDur:0.4, impact:0.13, recoil:1.0 },
  // 5 — Retro Lancer (Officer primary): 3-round burst rifle (FAMAS-like) — accurate, punchy per burst.
  { id:'lancer', name:'Retro Lancer', short:'LAN', era:1, minLevel:1, cost:0, auto:false, burst:3, burstInterval:0.06, fireInterval:0.42, mag:24, reserve:120, pellets:1, spreadHip:0.02, spreadADS:0.008, spreadPerShot:0.003, spreadMax:0.06, spreadDecay:0.5, damage:9, range:95, recoil:1.3, reload:2.2, tracer:0xfff0c0 },
  { id:'torque', name:'Torque Bow', short:'TRQ', era:1, minLevel:1, cost:500, charge:true, special:true, auto:false, fireInterval:0.7, mag:1, reserve:10, pellets:0, spreadHip:0, spreadADS:0, spreadPerShot:0, spreadMax:0, spreadDecay:1, damage:30, range:200, recoil:2.4, reload:1.5, tracer:0 },
];

// ===== UNIT TYPES (Phase 5 — first tier; rock/paper/scissors, RA3-flavoured) =====
// Triangle: Warrior rushes & beats Conscript · Peacekeeper's shield beats Warrior · Conscript kites & beats Peacekeeper.
const UNIT_TYPES=[
  { id:'conscript',  name:'Conscript',        tier:1, cost:0, wpn:0, altWpn:3, ability:'molotov', swapCd:1.5,
    sprintMult:1.214, maxHealth:100, accent:0xff5b5b,
    enemyDmg:8, enemyRange:80, preferRange:24, speedMult:1.0,  fireInterval:0.13, burst:4 },
  { id:'peacekeeper',name:'Peacekeeper',      tier:1, cost:0, wpn:1,            ability:'shield',  swapCd:2.0,
    sprintMult:1.165, maxHealth:150, accent:0x55a8ff,
    enemyDmg:6, enemyRange:24, preferRange:11, speedMult:0.95, fireInterval:0.8,  burst:1 },
  { id:'warrior',    name:'Imperial Warrior', tier:1, cost:0, wpn:2, altWpn:4, ability:'banzai', banzaiDur:10, banzaiCd:45,
    sprintMult:1.62,  maxHealth:120, accent:0xffd24a,
    enemyDmg:5, enemyRange:48, preferRange:9,  speedMult:1.5,  fireInterval:0.09, burst:6, lunge:48 },
  { id:'officer',    name:'Officer',          tier:2, cost:1250, wpn:5,            ability:'bayonet', bayonetCd:10, bayonetDur:1.3,
    sprintMult:1.214, maxHealth:110, accent:0x9b8cff,
    enemyDmg:7, enemyRange:70, preferRange:18, speedMult:1.05, fireInterval:0.35, burst:3 },
];
function unitOf(i){ return UNIT_TYPES[i]||UNIT_TYPES[0]; }
const VEHICLE_TYPES=[
  { id:'juggernaut', name:'Juggernaut', tier:2, cost:2500, maxHealth:1500, seats:1,
    desc:"Enterable heavy assault walker (Kane's Wrath-inspired) \u2014 heavy armour, triple artillery cannons." }
];

const app=document.getElementById('app');
const renderer=new THREE.WebGLRenderer({antialias:true});
let htOpacity=0, htSize=5;   // Living Comic halftone dots -- default OFF (locked)
function updateHalftone(){ let el=document.getElementById('halftone');
  if(!el){ el=document.createElement('div'); el.id='halftone'; document.body.appendChild(el); }
  if(htOpacity<=0){ el.style.display='none'; return; }
  el.style.display='block'; el.style.opacity=htOpacity;
  const r=(htSize*0.18).toFixed(2), r2=(htSize*0.30).toFixed(2);
  el.style.backgroundImage='radial-gradient(circle, rgba(20,20,40,0.9) '+r+'px, transparent '+r2+'px)';
  el.style.backgroundSize=htSize+'px '+htSize+'px'; }
renderer.setSize(innerWidth,innerHeight); renderer.setPixelRatio(Math.min(devicePixelRatio,2));
renderer.shadowMap.enabled=true; renderer.shadowMap.type=THREE.PCFSoftShadowMap; renderer.outputEncoding=THREE.LinearEncoding;   // authored hex = on-screen colour. r128 never converts input colours, so sRGB output brightened everything twice (washed-out look, shadows lifted to grey)
app.appendChild(renderer.domElement);

const scene=new THREE.Scene(); scene.background=new THREE.Color(0x8fcfee); scene.fog=new THREE.Fog(0xb0dcf0,260,760);
const camera=new THREE.PerspectiveCamera(T.fov,innerWidth/innerHeight,0.1,400); camera.position.set(0,T.camHeight,T.camBack);

const hemiLight=new THREE.HemisphereLight(0xffe3b8,0x3a2456,0.78); scene.add(hemiLight); const ambLight=new THREE.AmbientLight(0x3a2456,0.28); scene.add(ambLight);
const sun=new THREE.DirectionalLight(0xfff0cc,1.32); sun.position.set(-34,50,26); sun.castShadow=true;
sun.shadow.mapSize.set(2048,2048); sun.shadow.camera.near=1; sun.shadow.camera.far=160;
const sd=50; sun.shadow.camera.left=-sd; sun.shadow.camera.right=sd; sun.shadow.camera.top=sd; sun.shadow.camera.bottom=-sd; sun.shadow.bias=-0.0004; scene.add(sun); scene.add(sun.target);
const SUN_R=65.8, SUN_SD=sd; const _sunDir=new THREE.Vector3().copy(sun.position).normalize();   // shadow frustum follows the player (crisp shadows across the 404x252 map); _sunDir preserves the env-set light direction
function noRay(o){ o.raycast=function(){}; } // exclude decorative objects from raycasts
const ENV_DEFAULT={
  ground:{ base:'#4aab24', detailDark:'#3f9e1e', detailLight:'#5fbf2e', density:1.25, contrast:1.3, patch:1.0, tiling:20, litCap:195 },
  sun:{ color:'#ffffff', int:0.8, az:-52.6, el:49.4 },
  amb:{ color:'#303055', int:0.12 },
  hemi:{ sky:'#a0c8ff', ground:'#4a3a66', int:0.32 },
  nature:{ gain:1.56 },   // brightens ONLY grass + nature-tagged materials (foliage, hedges): lush environment on top of the richer building/character lighting
  sky:{ bg:'#8fd0f0' },
  fog:{ color:'#bfe4f5', near:260, far:760 },
  _sunR:65.8
};
let _natureGain=ENV_DEFAULT.nature.gain;   // live value; applyEnv updates it, toonify bakes it into nature materials at creation
function envMerge(src){ const d=JSON.parse(JSON.stringify(ENV_DEFAULT)); if(src){ for(const k in d){ if(d[k]&&typeof d[k]==='object'&&src[k]&&typeof src[k]==='object') Object.assign(d[k],src[k]); else if(src[k]!==undefined) d[k]=src[k]; } } return d; }
let _env=envMerge(null);
// ── Match Rules: per-map objective + economy timings (mirrors per-map _env). Writes through to the live SILO/TBOX/HARV/T constants. ──
function currentEnvJSON(){ return JSON.parse(JSON.stringify(_env)); }
// ground-only 3-band cel ramp (so we can cap how bright LIT grass gets independently of the global ramp)
const _groundGrad=(()=>{ const d=new Uint8Array([80,80,80, 195,195,195]); const t=new THREE.DataTexture(d,2,1,THREE.RGBFormat); t.magFilter=THREE.NearestFilter; t.minFilter=THREE.NearestFilter; t.needsUpdate=true; return t; })();
// painterly, seamless, parametric grass: tonal patches + clumps + fine speckle + wrapped blade strokes
function generateGrassTexture(g){
  const _mr=Math.random; let _gs=1337>>>0; Math.random=function(){ _gs=(_gs+0x6D2B79F5)>>>0; let _tt=Math.imul(_gs^(_gs>>>15),1|_gs); _tt=(_tt+Math.imul(_tt^(_tt>>>7),61|_tt))^_tt; return ((_tt^(_tt>>>14))>>>0)/4294967296; };
  g=g||ENV_DEFAULT.ground; const S=512, c=document.createElement('canvas'); c.width=c.height=S; const x=c.getContext('2d');
  x.fillStyle=g.base; x.fillRect(0,0,S,S);
  const dark=g.detailDark, light=g.detailLight, k=g.contrast;
  function blob(px,py,r,col,a){ x.globalAlpha=a; const grd=x.createRadialGradient(px,py,0,px,py,r); grd.addColorStop(0,col); grd.addColorStop(1,'rgba(0,0,0,0)'); x.fillStyle=grd;
    for(let ox=-1;ox<=1;ox++)for(let oy=-1;oy<=1;oy++){ const cx=px+ox*S,cy=py+oy*S; if(cx<-r||cx>S+r||cy<-r||cy>S+r)continue; x.beginPath(); x.arc(cx,cy,r,0,6.283); x.fill(); } }
  function stroke(px,py,len,ang,w,col,a){ x.globalAlpha=a; x.strokeStyle=col; x.lineWidth=w; x.lineCap='round';
    for(let ox=-1;ox<=1;ox++)for(let oy=-1;oy<=1;oy++){ const cx=px+ox*S,cy=py+oy*S; if(cx<-len||cx>S+len||cy<-len||cy>S+len)continue; x.beginPath(); x.moveTo(cx,cy); x.lineTo(cx+Math.sin(ang)*len,cy-Math.cos(ang)*len); x.stroke(); } }
  const nPatch=Math.round(16*g.patch); for(let i=0;i<nPatch;i++) blob(Math.random()*S,Math.random()*S,48+Math.random()*86,(i&1)?dark:light,(0.06+Math.random()*0.08)*k);
  const nClump=Math.round(70*g.density); for(let i=0;i<nClump;i++) blob(Math.random()*S,Math.random()*S,10+Math.random()*22,(i&1)?dark:light,(0.10+Math.random()*0.14)*k);
  const nFine=Math.round(360*g.density); for(let i=0;i<nFine;i++) blob(Math.random()*S,Math.random()*S,3+Math.random()*7,(i&1)?dark:light,(0.12+Math.random()*0.18)*k);
  const nBlade=Math.round(300*g.density); for(let i=0;i<nBlade;i++){ const a2=Math.random()*0.5-0.25; stroke(Math.random()*S,Math.random()*S,5+Math.random()*10,a2,1+Math.random()*1.4,(i&1)?dark:light,(0.05+Math.random()*0.10)*k); }
  x.globalAlpha=1; const t=new THREE.CanvasTexture(c); t.needsUpdate=true; Math.random=_mr; return t;
}
// apply an env block to the live scene. regen:false skips the (heavier) grass-canvas rebuild.
function applyEnv(env, opts){ opts=opts||{}; const regen=(opts.regen!==false);
  try{
    const G=env.ground;
    if(regen){ const tex=generateGrassTexture(G); tex.wrapS=tex.wrapT=THREE.RepeatWrapping; tex.repeat.set(G.tiling,G.tiling*GROUND_D/GROUND_W); tex.anisotropy=4;
      const old=ground.material.map; ground.material.map=tex; ground.material.needsUpdate=true; if(old&&old!==tex)old.dispose(); }
    else if(ground.material.map){ ground.material.map.repeat.set(G.tiling,G.tiling*GROUND_D/GROUND_W); }
    const lc=Math.max(170,Math.min(255,Math.round(G.litCap))); const gd=_groundGrad.image.data; gd[3]=gd[4]=gd[5]=lc; _groundGrad.needsUpdate=true;
    if(ground.material.isMeshToonMaterial) ground.material.gradientMap=_groundGrad;
    if(scene.background&&scene.background.set) scene.background.set(env.sky.bg);
    if(scene.fog){ scene.fog.color.set(env.fog.color); scene.fog.near=env.fog.near; scene.fog.far=env.fog.far; }
    sun.color.set(env.sun.color); sun.intensity=env.sun.int;
    const R=env._sunR||65.8, elr=env.sun.el*Math.PI/180, azr=env.sun.az*Math.PI/180, hh=R*Math.cos(elr);
    sun.position.set(hh*Math.sin(azr), R*Math.sin(elr), hh*Math.cos(azr)); _sunDir.copy(sun.position).normalize();
    ambLight.color.set(env.amb.color); ambLight.intensity=env.amb.int;
    hemiLight.color.set(env.hemi.sky); hemiLight.groundColor.set(env.hemi.ground); hemiLight.intensity=env.hemi.int;
    _natureGain=(env.nature&&env.nature.gain!=null)?env.nature.gain:1;
    ground.material.color.setScalar(_natureGain);
    scene.traverse(o=>{ const ms=Array.isArray(o.material)?o.material:(o.material?[o.material]:null); if(!ms) return;
      for(const m of ms) if(m.userData&&m.userData.nature&&m.userData.baseCol) m.color.copy(m.userData.baseCol).multiplyScalar(_natureGain); });
  }catch(e){ console.warn('[env] applyEnv skipped:', e); }
}
const GROUND_W=372, GROUND_D=252;
T.arenaX=GROUND_W/2; T.arenaZ=GROUND_D/2;   // red border + walk clamp match the walkable island
const _grassTex=generateGrassTexture(); _grassTex.wrapS=_grassTex.wrapT=THREE.RepeatWrapping; _grassTex.repeat.set(20,20*GROUND_D/GROUND_W); _grassTex.anisotropy=4;
const groundMat=new THREE.MeshStandardMaterial({color:0xffffff,roughness:1.0,map:_grassTex});
const ground=new THREE.Mesh(new THREE.PlaneGeometry(GROUND_W,GROUND_D),groundMat); ground.rotation.x=-Math.PI/2; ground.receiveShadow=true; scene.add(ground);
// stylized island: flat sea (MeshBasic => skipped by stylizeScene), a sand ring, grass on top
const seaMat=new THREE.MeshBasicMaterial({color:0x2f6f8f}); seaMat.userData.nature=true; seaMat.userData.baseCol=seaMat.color.clone(); /* environment: gets the nature gain in applyEnv */ const sea=new THREE.Mesh(new THREE.PlaneGeometry(2000,2000),seaMat); sea.rotation.x=-Math.PI/2; sea.position.y=-0.6; sea.raycast=function(){}; scene.add(sea);
const beachMat=new THREE.MeshStandardMaterial({color:0xcdba7e,roughness:1.0}); const beach=new THREE.Mesh(new THREE.PlaneGeometry(404,284),beachMat); beach.rotation.x=-Math.PI/2; beach.position.y=-0.18; beach.receiveShadow=true; beach.raycast=function(){}; scene.add(beach);

/* ============ LUSH COMIC NPR — toon shading + inverted-hull ink + comic SFX sprites ============
 * Cel shading uses the BUILT-IN MeshToonMaterial (+ a 3-band gradient ramp) rather than a hand-written
 * cel shader — it inherits the scene's lights/shadows/fog for free. Colored shadows come from the violet
 * HemisphereLight ground term + violet AmbientLight (scene-wide), not a per-material tint. Saturation is
 * baked into the flat albedos. Outlines are inverted-hull (a tiny normal-push BackSide shader). The whole
 * pass is gated by NPR and wrapped in try/catch so a bad frame degrades gracefully instead of breaking. */
let NPR=true, COMIC=true;
const INK=0x11121a;   // Living Comic bold near-black ink (shared by world + characters; only thickness differs)
const WORLD_OUTLINE=0.09;   // Living Comic bold world ink [dial]
let   CHAR_OUTLINE=0.8;      // characters/units/weapons get the thin Illustrated weight (locked multiplier)
// toon ramp: shadow / mid / lit bands — edit these bytes to move the stepped-lighting thresholds
const _toonGrad=(()=>{ const d=new Uint8Array([80,80,80, 240,240,240]); const t=new THREE.DataTexture(d,2,1,THREE.RGBFormat); t.magFilter=THREE.NearestFilter; t.minFilter=THREE.NearestFilter; t.needsUpdate=true; return t; })();
// Extra cel ramps for the character look: warm SKIN (fake SSS -- shadows bleed warm, not grey), a hair-warm
// CLOTH ramp (keeps saturated faction uniforms from muddying), and a punchier METAL ramp (crisp steel/brass).
function _mkGrad(arr){ const n=arr.length/3; const t=new THREE.DataTexture(new Uint8Array(arr),n,1,THREE.RGBFormat); t.magFilter=THREE.NearestFilter; t.minFilter=THREE.NearestFilter; t.needsUpdate=true; return t; }
const _skinGrad =_mkGrad([150,96,84, 255,244,230]);
const _clothGrad=_mkGrad([96,92,86, 252,249,244]);
const _metalGrad=_mkGrad([48,48,48, 86,86,86, 150,150,150, 255,255,255]);
const _lacquerGrad=_mkGrad([18,18,26, 30,30,42, 120,120,140, 235,235,245]);   // Empire elite lacquer: dark + one glint
// Fresnel rim = fake subsurface warmth on skin, injected into a MeshToonMaterial via onBeforeCompile using
// SELF-CONTAINED varyings (never depends on three internals; a graceful no-op if these tokens ever move).
const RIM_COL=0xff7a52, RIM_STR=0.55, RIM_POW=2.6;
function attachSkinRim(m){
  m.onBeforeCompile=sh=>{
    sh.uniforms.uRimColor={value:new THREE.Color(RIM_COL)}; sh.uniforms.uRimStr={value:RIM_STR}; sh.uniforms.uRimPow={value:RIM_POW};
    sh.vertexShader=sh.vertexShader
      .replace('void main() {','varying vec3 vRimN; varying vec3 vRimV;\nvoid main() {')
      .replace('#include <project_vertex>','#include <project_vertex>\n vRimN=normalize(transformedNormal); vRimV=-mvPosition.xyz;');
    sh.fragmentShader=sh.fragmentShader
      .replace('void main() {','uniform vec3 uRimColor; uniform float uRimStr; uniform float uRimPow;\nvarying vec3 vRimN; varying vec3 vRimV;\nvoid main() {')
      .replace('#include <output_fragment>','float _rf=pow(1.0-clamp(dot(normalize(vRimN),normalize(vRimV)),0.0,1.0),uRimPow);\n outgoingLight+=uRimColor*(_rf*uRimStr);\n#include <output_fragment>');
  };
  m.customProgramCacheKey=()=>'skinRim'; return m;
}
function gradForKind(kind){ return kind==='skin'?_skinGrad:kind==='cloth'?_clothGrad:kind==='metal'?_metalGrad:kind==='lacquer'?_lacquerGrad:_toonGrad; }
function applyVillain(group){   // Empire elite: near-black lacquer armour + emissive violet visor + purple silhouette
  group.traverse(o=>{
    if(!o.isMesh||!o.material) return;
    if(o.userData.isOutline){ if(o.material.uniforms&&o.material.uniforms.ink) o.material.uniforms.ink.value.set(0x7B2FD6); return; }
    if(o.userData.role){ o.material.color.set(0x262636); o.material.gradientMap=_lacquerGrad; o.material.needsUpdate=true; }
  });
  const head=group.userData.head;
  if(head){ const visor=new THREE.Mesh(new THREE.BoxGeometry(0.5,0.12,0.16), new THREE.MeshBasicMaterial({color:0xc24bff}));
    visor.position.set(0,0.05,0.38); visor.userData.noOutline=true; head.add(visor); }
}
function toonify(std, kind){
  const m=new THREE.MeshToonMaterial({ color:std.color?std.color.clone():new THREE.Color(0xffffff), gradientMap:gradForKind(kind) });
  if(std.map) m.map=std.map;
  if(std.emissive){ m.emissive=std.emissive.clone(); m.emissiveIntensity=(std.emissiveIntensity!==undefined)?std.emissiveIntensity:1; }
  m.transparent=std.transparent; m.opacity=std.opacity; m.side=std.side; m.fog=true;
  if(kind==='skin') attachSkinRim(m);
  if(std.userData&&std.userData.nature){ m.userData.nature=true; m.userData.baseCol=m.color.clone(); m.color.multiplyScalar(_natureGain); }
  return m;
}
// Ink hull shader. Offsets each vertex in WORLD units along its smoothed outline direction
// (attribute oNormal, built by ensureOutlineNormals), so line weight ignores object/part scale
// and hard corners stay closed. oNormal's length carries the mitre factor. If oNormal is missing
// (a geometry swapped in without ensureOutlineNormals) WebGL feeds (0,0,0) and the shader falls
// back to the plain normal. The clamp keeps the shell from sinking below the ground: the hull's
// bottom face flattens into an ink rim around every footprint (the contact line). WebGL1-safe:
// vec4*mat4 = transpose(mat4)*vec4, i.e. view->world for the rigid view matrix.
const OUTLINE_GROUND_Y=0.02;   // just above the grass (y=0), below road decal tops (0.06); avoids depth fighting
function makeOutlineMat(thickness){ return new THREE.ShaderMaterial({
  uniforms:{ oThick:{value:thickness}, ink:{value:new THREE.Color(INK)}, oGround:{value:OUTLINE_GROUND_Y} },
  vertexShader:'uniform float oThick; uniform float oGround; attribute vec3 oNormal;\n'+
    'void main(){ vec3 n=oNormal; float s=length(n); if(s<1e-4){ n=normal; s=1.0; }\n'+
    '  vec3 wn=normalize((vec4(normalMatrix*n,0.0)*viewMatrix).xyz);\n'+
    '  vec4 wp=modelMatrix*vec4(position,1.0); wp.xyz+=wn*(oThick*s); wp.y=max(wp.y,oGround);\n'+
    '  gl_Position=projectionMatrix*viewMatrix*wp; }',
  fragmentShader:'uniform vec3 ink; void main(){ gl_FragColor=vec4(ink,1.0); }',
  side:THREE.BackSide, fog:false }); }

// Smoothed + mitred outline directions, cached per geometry CONTENT (soldier rebuilds re-create
// identical part shapes, so each unique shape is solved once). Vertices sharing a position get the
// sum of their DISTINCT normals, scaled by 1/min(cos) so each face is pushed out by ~the full
// thickness (mitre capped at 2x, like a stroke mitre limit). Opposing normals that cancel (thin
// double-sided sheets) keep their own normal. One-time cost per unique geometry; never per frame.
const _oNCache=new Map();
function _outlineDirs(P,N,n){
  const out=new Float32Array(n*3), idx=new Uint32Array(n), qx=new Int32Array(n), qy=new Int32Array(n), qz=new Int32Array(n);
  for(let i=0;i<n;i++){ idx[i]=i; qx[i]=Math.round(P.getX(i)*1e4); qy[i]=Math.round(P.getY(i)*1e4); qz[i]=Math.round(P.getZ(i)*1e4); }
  idx.sort((a,b)=>(qx[a]-qx[b])||(qy[a]-qy[b])||(qz[a]-qz[b]));
  const ux=[],uy=[],uz=[];
  for(let s=0;s<n;){
    const a=idx[s]; let e=s+1;
    while(e<n&&qx[idx[e]]===qx[a]&&qy[idx[e]]===qy[a]&&qz[idx[e]]===qz[a]) e++;
    let k=0,sx=0,sy=0,sz=0;
    for(let t=s;t<e;t++){ const i=idx[t], x=N.getX(i), y=N.getY(i), z=N.getZ(i); let dup=false;
      for(let u=0;u<k;u++){ if(x*ux[u]+y*uy[u]+z*uz[u]>0.999){ dup=true; break; } }
      if(!dup){ ux[k]=x; uy[k]=y; uz[k]=z; k++; sx+=x; sy+=y; sz+=z; } }
    const L=hyp3(sx,sy,sz);
    if(L>1e-4){ const mx=sx/L, my=sy/L, mz=sz/L; let mind=1;
      for(let u=0;u<k;u++){ const d=mx*ux[u]+my*uy[u]+mz*uz[u]; if(d<mind) mind=d; }
      const sc=Math.min(2,1/Math.max(mind,0.5));
      for(let t=s;t<e;t++){ const o=idx[t]*3; out[o]=mx*sc; out[o+1]=my*sc; out[o+2]=mz*sc; } }
    else { for(let t=s;t<e;t++){ const i=idx[t], o=i*3; out[o]=N.getX(i); out[o+1]=N.getY(i); out[o+2]=N.getZ(i); } }
    s=e;
  }
  return out;
}
function ensureOutlineNormals(geo){
  const A=geo&&geo.attributes; if(!A||!A.position||!A.normal) return false;
  const P=A.position, N=A.normal, n=P.count;
  if(A.oNormal&&A.oNormal.count===n) return true;
  let hp=2166136261, hn=2166136261;   // order-sensitive content hash (FNV-style), numeric only
  for(let i=0;i<n;i++){
    hp=Math.imul(hp^Math.round(P.getX(i)*1e4),16777619); hp=Math.imul(hp^Math.round(P.getY(i)*1e4),16777619); hp=Math.imul(hp^Math.round(P.getZ(i)*1e4),16777619);
    hn=Math.imul(hn^Math.round(N.getX(i)*1e4),16777619); hn=Math.imul(hn^Math.round(N.getY(i)*1e4),16777619); hn=Math.imul(hn^Math.round(N.getZ(i)*1e4),16777619);
  }
  const key=n+':'+hp+':'+hn;
  let dirs=_oNCache.get(key);
  if(!dirs){ if(_oNCache.size>4000) _oNCache.clear(); dirs=_outlineDirs(P,N,n); _oNCache.set(key,dirs); }
  geo.setAttribute('oNormal', new THREE.BufferAttribute(dirs,3));
  return true;
}
// Draw-call merge for STATIC kit pieces (houses, covers, props, trees): folds a group's meshes into one mesh per
// (material, outline weight, shadow flags, uv, raycast), each part's transform baked in relative to the group, and gives
// every merged mesh one ink hull at the same weight. The piece stays its own group, so editor picking (editSel on the
// group), colliders (computed before) and re-rolls are unchanged. Skips: transparent materials, meshes with non-hull
// children, and anything tagged userData.noMerge (e.g. a piece's collider body). Returns draw calls saved.
const _mgM=new THREE.Matrix4(), _mgI=new THREE.Matrix4(), _mgN=new THREE.Matrix3(), _mgV=new THREE.Vector3();
// Material CONTENT signature: toonify gives every mesh its own material instance, so equal-looking parts only share by
// value. Custom shader hooks (onBeforeCompile, e.g. the skin rim) are never shared.
function _matSig(m){ if(m.onBeforeCompile!==THREE.Material.prototype.onBeforeCompile) return 'u:'+m.uuid;
  const u=m.userData||{}, bc=u.baseCol, bcs=(bc&&bc.getHex)?bc.getHex():(bc!=null?bc:'');
  return m.type+'|'+(m.color?m.color.getHex():-1)+'|'+(m.emissive?m.emissive.getHex():-1)+'|'+(m.emissiveIntensity||0)+'|'+(m.map?m.map.uuid:'')+'|'+(m.gradientMap?m.gradientMap.uuid:'')
    +'|'+m.side+'|'+(m.vertexColors?1:0)+'|'+(m.flatShading?1:0)+'|'+(m.alphaTest||0)+'|'+(m.depthWrite?1:0)+'|'+(u.nature?'n'+bcs:''); }
function mergeStaticGroup(grp){
  if(!grp||grp.userData._merged) return 0;
  grp.updateMatrixWorld(true); _mgI.copy(grp.matrixWorld).invert();
  const buckets=new Map(), baseRay=THREE.Mesh.prototype.raycast;
  grp.traverse(o=>{ if(!o.isMesh||o.userData.isOutline||o.userData.noMerge||!o.visible||Array.isArray(o.material)||!o.material||o.material.transparent) return;
    const g=o.geometry; if(!g||!g.isBufferGeometry||!g.attributes.position||!g.attributes.normal) return;
    let hull=null, other=0; for(let i=0;i<o.children.length;i++){ const ch=o.children[i]; if(ch.userData.isOutline&&!hull) hull=ch; else other++; }
    if(other) return;
    const th=(hull&&hull.material&&hull.material.uniforms&&hull.material.uniforms.oThick)?hull.material.uniforms.oThick.value:0, nr=(o.raycast!==baseRay);
    const key=_matSig(o.material)+'|'+th.toFixed(5)+'|'+(o.castShadow?1:0)+(o.receiveShadow?1:0)+'|'+(g.attributes.uv?1:0)+'|'+(nr?1:0);
    let b=buckets.get(key); if(!b){ b={mat:o.material,th,cast:o.castShadow,recv:o.receiveShadow,uv:!!g.attributes.uv,ray:nr?o.raycast:null,meshes:[]}; buckets.set(key,b); }
    b.meshes.push(o); });
  let saved=0;
  for(const b of buckets.values()){ if(b.meshes.length<2) continue;
    const parts=[]; let nv=0; for(const o of b.meshes){ const g=o.geometry.index?o.geometry.toNonIndexed():o.geometry; parts.push(o,g); nv+=g.attributes.position.count; }
    const P=new Float32Array(nv*3), N=new Float32Array(nv*3), U=b.uv?new Float32Array(nv*2):null; let off=0;
    for(let k=0;k<parts.length;k+=2){ const o=parts[k], g=parts[k+1], pa=g.attributes.position, na=g.attributes.normal, ua=g.attributes.uv, n=pa.count;
      _mgM.multiplyMatrices(_mgI,o.matrixWorld); _mgN.getNormalMatrix(_mgM);
      for(let i=0;i<n;i++){ const q=(off+i)*3; _mgV.fromBufferAttribute(pa,i).applyMatrix4(_mgM); P[q]=_mgV.x; P[q+1]=_mgV.y; P[q+2]=_mgV.z;
        _mgV.fromBufferAttribute(na,i).applyMatrix3(_mgN).normalize(); N[q]=_mgV.x; N[q+1]=_mgV.y; N[q+2]=_mgV.z;
        if(U&&ua){ U[(off+i)*2]=ua.getX(i); U[(off+i)*2+1]=ua.getY(i); } }
      off+=n; if(g!==o.geometry) g.dispose(); }
    const mg=new THREE.BufferGeometry(); mg.setAttribute('position',new THREE.BufferAttribute(P,3)); mg.setAttribute('normal',new THREE.BufferAttribute(N,3)); if(U) mg.setAttribute('uv',new THREE.BufferAttribute(U,2));
    mg.computeBoundingBox(); mg.computeBoundingSphere();
    const m=new THREE.Mesh(mg,b.mat); m.castShadow=b.cast; m.receiveShadow=b.recv; m.userData._toon=true; m.userData._mergedParts=b.meshes.length; if(b.ray) m.raycast=b.ray;
    grp.add(m); if(b.th>0) addHull(m,b.th);
    for(const o of b.meshes){ for(const ch of o.children){ if(ch.userData.isOutline&&ch.material) ch.material.dispose(); } if(o.parent) o.parent.remove(o); o.geometry.dispose(); if(o.material!==b.mat) o.material.dispose(); }
    saved+=(b.meshes.length-1)*(b.th>0?2:1); }
  grp.userData._merged=true; return saved; }
function addHull(mesh, thickness){
  if(!mesh.geometry||!mesh.geometry.attributes||!mesh.geometry.attributes.normal) return;
  let t=thickness||WORLD_OUTLINE;
  if(mesh.userData&&mesh.userData.part) t*=CHAR_OUTLINE;   // characters/units/weapons -> thin Illustrated weight
  ensureOutlineNormals(mesh.geometry);
  const o=new THREE.Mesh(mesh.geometry, makeOutlineMat(t));
  o.castShadow=false; o.receiveShadow=false; o.userData.isOutline=true; noRay(o); mesh.add(o);
}
function stylizeScene(scene){
  if(!NPR) return;
  try{
    const list=[];
    scene.traverse(o=>{ if(o.isMesh && o.material && o.material.isMeshStandardMaterial && !o.userData.isOutline) list.push(o); });
    for(const m of list){
      const flat = m.geometry && /Plane|Circle|Ring/.test(m.geometry.type);
      m.material = toonify(m.material, m.userData.matKind);
      if(!flat && !m.userData.noOutline) addHull(m, m.userData.outlineW||WORLD_OUTLINE);
    }
  }catch(e){ console.warn('[NPR] stylize skipped:', e); }
}
// toonify + ink-outline a runtime-spawned object (stylizeScene only runs once at init, so dynamic objects bypass it).
// Mirrors stylizeScene per-mesh: skip outlines on flat geo + userData.noOutline; guard against double-apply.
function stylizeOne(obj, thickness){
  if(!NPR||!obj) return obj;
  obj.traverse(o=>{ if(o.isMesh && o.material && o.material.isMeshStandardMaterial && !o.userData.isOutline && !o.userData._toon){
    const flat = o.geometry && /Plane|Circle|Ring/.test(o.geometry.type);
    o.material = toonify(o.material, o.userData.matKind); o.userData._toon=true;
    if(!flat && !o.userData.noOutline) addHull(o, thickness||o.userData.outlineW||WORLD_OUTLINE);
  }});
  return obj;
}
/* --- comic onomatopoeia: canvas-drawn, cached textures, spawned as scaling/fading sprites --- */
const CHAR_DEFAULT={ kind:'soldier', colors:{dark:0x1b2024}, pelvisY:0.95, armMode:'rifle',
  parts:[
    {id:'pelvis',type:'group',parent:null,pos:[0,0.95,0]},
    {id:'hips',type:'sphere',parent:'pelvis',mat:'base',tag:'legs',ow:0.05,r:0.34,wseg:14,hseg:10,scale:[1.0,0.7,0.86]},
    {id:'hipL',type:'group',parent:'pelvis',pos:[-0.2,-0.05,0]},
    {id:'thighL',type:'cyl',parent:'hipL',mat:'base',tag:'legs',ow:0.03,rt:0.18,rb:0.16,h:0.44,rseg:10,pos:[0,-0.22,0]},
    {id:'kneeL',type:'group',parent:'hipL',pos:[0,-0.44,0]},
    {id:'shinL',type:'cyl',parent:'kneeL',mat:'base',tag:'legs',ow:0.03,rt:0.15,rb:0.12,h:0.42,rseg:10,pos:[0,-0.21,0]},
    {id:'footL',type:'box',parent:'kneeL',mat:'dark',tag:'legs',ow:0.03,w:0.2,h:0.14,d:0.4,pos:[0,-0.42,0.1]},
    {id:'hipR',type:'group',parent:'pelvis',pos:[0.2,-0.05,0]},
    {id:'thighR',type:'cyl',parent:'hipR',mat:'base',tag:'legs',ow:0.03,rt:0.18,rb:0.16,h:0.44,rseg:10,pos:[0,-0.22,0]},
    {id:'kneeR',type:'group',parent:'hipR',pos:[0,-0.44,0]},
    {id:'shinR',type:'cyl',parent:'kneeR',mat:'base',tag:'legs',ow:0.03,rt:0.15,rb:0.12,h:0.42,rseg:10,pos:[0,-0.21,0]},
    {id:'footR',type:'box',parent:'kneeR',mat:'dark',tag:'legs',ow:0.03,w:0.2,h:0.14,d:0.4,pos:[0,-0.42,0.1]},
    {id:'spine',type:'group',parent:'pelvis',pos:[0,0.05,0]},
    {id:'torso',type:'cyl',parent:'spine',mat:'base',tag:'body',ow:0.05,rt:0.37,rb:0.29,h:0.98,rseg:16,pos:[0,0.42,0]},
    {id:'chest',type:'group',parent:'spine',pos:[0,0.7,0]},
    {id:'shoulders',type:'cyl',parent:'chest',mat:'base',tag:'body',ow:0.05,rt:0.46,rb:0.41,h:0.26,rseg:16,pos:[0,0.02,0]},
    {id:'neck',type:'group',parent:'chest',pos:[0,0.18,0]},
    {id:'head',type:'sphere',parent:'neck',mat:'accent',tag:'head',ow:0.05,r:0.36,wseg:16,hseg:14,pos:[0,0.28,0]},
    {id:'brim',type:'box',parent:'neck',mat:'base',tag:'head',ow:0.03,w:0.46,h:0.1,d:0.34,pos:[0,0.36,0.26]},
    {id:'shR',type:'group',parent:'chest',pos:[0.56,0.06,0.04]},
    {id:'upR',type:'cyl',parent:'shR',mat:'base',tag:'body',ow:0.03,rt:0.115,rb:0.1,h:0.44,rseg:10,pos:[0,-0.22,0]},
    {id:'elbowR',type:'group',parent:'shR',pos:[0,-0.44,0]},
    {id:'farmR',type:'cyl',parent:'elbowR',mat:'base',tag:'body',ow:0.03,rt:0.1,rb:0.085,h:0.4,rseg:10,pos:[0,-0.2,0]},
    {id:'handR',type:'sphere',parent:'elbowR',mat:'dark',tag:'body',ow:0.03,r:0.105,wseg:10,hseg:8,pos:[0,-0.4,0]},
    {id:'shL',type:'group',parent:'chest',pos:[-0.56,0.06,0.04]},
    {id:'upL',type:'cyl',parent:'shL',mat:'base',tag:'body',ow:0.03,rt:0.115,rb:0.1,h:0.44,rseg:10,pos:[0,-0.22,0]},
    {id:'elbowL',type:'group',parent:'shL',pos:[0,-0.44,0]},
    {id:'farmL',type:'cyl',parent:'elbowL',mat:'base',tag:'body',ow:0.03,rt:0.1,rb:0.085,h:0.4,rseg:10,pos:[0,-0.2,0]},
    {id:'handL',type:'sphere',parent:'elbowL',mat:'dark',tag:'body',ow:0.03,r:0.105,wseg:10,hseg:8,pos:[0,-0.4,0]},
  ],
  weapon:{id:'wpn',parent:'chest',pos:[0.12,-0.02,0.34],muzzle:[0,0.02,1.08],make:'AMKRifle'},
  joints:{pelvis:'pelvis',spine:'spine',chest:'chest',neck:'neck',hipL:'hipL',hipR:'hipR',kneeL:'kneeL',kneeR:'kneeR',shR:'shR',elR:'elbowR',shL:'shL',elL:'elbowL',handR:'handR',handL:'handL'},
  hit:{headY:2.12,headR:0.40,torsoBotY:0.95,torsoTopY:1.50,torsoR:0.52,legBotY:0.12,legTopY:0.95,legR:0.42,crouchDrop:0.55}
};
/* ---- Two BASE BODIES for the Character Lab: skin mannequins + a BaseGear undergarment + hair, no helmet.
 * Identical rig ids to CHAR_DEFAULT (so the Lab proportion sliders keep working) and the SAME hit-shape
 * (_BODY_HIT) so male/female are cosmetically distinct but a fair, identical target. Clothing layers over
 * these in Phase B. Male = broad chest/shoulders, thicker limbs, taller; Female = narrower shoulders,
 * defined waist, wider hip-to-waist, leaner limbs, shorter, longer hair. armMode 'relaxed' = clean A-pose. */
const _BODY_HIT={headY:2.12,headR:0.40,torsoBotY:0.95,torsoTopY:1.50,torsoR:0.52,legBotY:0.12,legTopY:0.95,legR:0.42,crouchDrop:0.55};
const CHAR_MALE={ kind:'soldier', sex:'m', colors:{dark:0x20242a, skin:0xd9a077, hair:0x3a2a1c, eyes:0x243038, steel:0x6a7078}, pelvisY:0.98, armMode:'relaxed',
  parts:[
    {id:'pelvis',type:'group',parent:null,pos:[0,0.98,0]},
    {id:'hips',type:'sphere',parent:'pelvis',mat:'skin',tag:'legs',ow:0.05,r:0.35,wseg:14,hseg:10,scale:[1.05,0.7,0.9]},
    {id:'shorts',type:'cyl',parent:'pelvis',mat:'primary',tag:'legs',ow:0.05,rt:0.37,rb:0.33,h:0.34,rseg:16,pos:[0,-0.12,0]},
    {id:'hipL',type:'group',parent:'pelvis',pos:[-0.21,-0.05,0]},
    {id:'thighL',type:'cyl',parent:'hipL',mat:'skin',tag:'legs',ow:0.03,rt:0.19,rb:0.16,h:0.46,rseg:12,pos:[0,-0.23,0]},
    {id:'kneeL',type:'group',parent:'hipL',pos:[0,-0.46,0]},
    {id:'shinL',type:'cyl',parent:'kneeL',mat:'skin',tag:'legs',ow:0.03,rt:0.15,rb:0.115,h:0.44,rseg:12,pos:[0,-0.22,0]},
    {id:'footL',type:'box',parent:'kneeL',mat:'skin',tag:'legs',ow:0.03,w:0.2,h:0.14,d:0.4,pos:[0,-0.44,0.1]},
    {id:'hipR',type:'group',parent:'pelvis',pos:[0.21,-0.05,0]},
    {id:'thighR',type:'cyl',parent:'hipR',mat:'skin',tag:'legs',ow:0.03,rt:0.19,rb:0.16,h:0.46,rseg:12,pos:[0,-0.23,0]},
    {id:'kneeR',type:'group',parent:'hipR',pos:[0,-0.46,0]},
    {id:'shinR',type:'cyl',parent:'kneeR',mat:'skin',tag:'legs',ow:0.03,rt:0.15,rb:0.115,h:0.44,rseg:12,pos:[0,-0.22,0]},
    {id:'footR',type:'box',parent:'kneeR',mat:'skin',tag:'legs',ow:0.03,w:0.2,h:0.14,d:0.4,pos:[0,-0.44,0.1]},
    {id:'spine',type:'group',parent:'pelvis',pos:[0,0.05,0]},
    {id:'torso',type:'cyl',parent:'spine',mat:'skin',tag:'body',ow:0.05,rt:0.41,rb:0.31,h:1.0,rseg:18,pos:[0,0.43,0]},
    {id:'chest',type:'group',parent:'spine',pos:[0,0.72,0]},
    {id:'shoulders',type:'cyl',parent:'chest',mat:'skin',tag:'body',ow:0.05,rt:0.5,rb:0.44,h:0.28,rseg:18,pos:[0,0.02,0]},
    {id:'pecs',type:'sphere',parent:'chest',mat:'skin',tag:'body',ow:0.03,noOutline:true,r:0.3,wseg:14,hseg:10,scale:[1.35,0.62,0.72],pos:[0,-0.16,0.1]},
    {id:'neck',type:'group',parent:'chest',pos:[0,0.2,0]},
    {id:'neckM',type:'cyl',parent:'neck',mat:'skin',tag:'body',ow:0.03,noOutline:true,rt:0.13,rb:0.16,h:0.18,rseg:12,pos:[0,0.0,0]},
    {id:'head',type:'sphere',parent:'neck',mat:'skin',tag:'head',ow:0.05,r:0.36,wseg:18,hseg:16,pos:[0,0.3,0],scale:[1,1.05,1]},
    {id:'hair',type:'sphere',parent:'neck',mat:'hair',tag:'head',ow:0.04,r:0.375,wseg:16,hseg:12,pos:[0,0.44,-0.08],scale:[1.05,0.82,1.08]},
    {id:'eyeL',type:'sphere',parent:'neck',mat:'eyes',tag:'head',ow:0,noOutline:true,r:0.05,wseg:8,hseg:6,pos:[-0.12,0.31,0.31]},
    {id:'eyeR',type:'sphere',parent:'neck',mat:'eyes',tag:'head',ow:0,noOutline:true,r:0.05,wseg:8,hseg:6,pos:[0.12,0.31,0.31]},
    {id:'browL',type:'box',parent:'neck',mat:'hair',tag:'head',ow:0.02,w:0.11,h:0.028,d:0.05,pos:[-0.12,0.375,0.32],rot:[0,0,0.12]},
    {id:'browR',type:'box',parent:'neck',mat:'hair',tag:'head',ow:0.02,w:0.11,h:0.028,d:0.05,pos:[0.12,0.375,0.32],rot:[0,0,-0.12]},
    {id:'nose',type:'sphere',parent:'neck',mat:'skin',tag:'head',ow:0,noOutline:true,r:0.045,wseg:8,hseg:8,scale:[1.0,1.6,1.3],pos:[0,0.27,0.35]},
    {id:'shR',type:'group',parent:'chest',pos:[0.6,0.06,0.04]},
    {id:'upR',type:'cyl',parent:'shR',mat:'skin',tag:'body',ow:0.03,rt:0.135,rb:0.115,h:0.58,rseg:12,pos:[0,-0.29,0]},
    {id:'padR',type:'sphere',parent:'shR',mat:'pad',tag:'body',ow:0.04,r:0.2,wseg:12,hseg:10,scale:[1.05,0.6,1.1],pos:[0,0.06,0]},
    {id:'elbowR',type:'group',parent:'shR',pos:[0,-0.58,0]},
    {id:'farmR',type:'cyl',parent:'elbowR',mat:'skin',tag:'body',ow:0.03,rt:0.115,rb:0.09,h:0.59,rseg:12,pos:[0,-0.295,0]},
    {id:'handR',type:'sphere',parent:'elbowR',mat:'skin',tag:'body',ow:0.03,r:0.11,wseg:10,hseg:8,pos:[0,-0.59,0]},
    {id:'shL',type:'group',parent:'chest',pos:[-0.6,0.06,0.04]},
    {id:'upL',type:'cyl',parent:'shL',mat:'skin',tag:'body',ow:0.03,rt:0.135,rb:0.115,h:0.58,rseg:12,pos:[0,-0.29,0]},
    {id:'padL',type:'sphere',parent:'shL',mat:'pad',tag:'body',ow:0.04,r:0.2,wseg:12,hseg:10,scale:[1.05,0.6,1.1],pos:[0,0.06,0]},
    {id:'elbowL',type:'group',parent:'shL',pos:[0,-0.58,0]},
    {id:'farmL',type:'cyl',parent:'elbowL',mat:'skin',tag:'body',ow:0.03,rt:0.115,rb:0.09,h:0.59,rseg:12,pos:[0,-0.295,0]},
    {id:'handL',type:'sphere',parent:'elbowL',mat:'skin',tag:'body',ow:0.03,r:0.11,wseg:10,hseg:8,pos:[0,-0.59,0]},
  ],
  joints:{pelvis:'pelvis',spine:'spine',chest:'chest',neck:'neck',hipL:'hipL',hipR:'hipR',kneeL:'kneeL',kneeR:'kneeR',shR:'shR',elR:'elbowR',shL:'shL',elL:'elbowL',handR:'handR',handL:'handL'},
  hit:_BODY_HIT
};
const CHAR_FEMALE={ kind:'soldier', sex:'f', colors:{dark:0x20242a, skin:0xe2ac86, hair:0x2a1c12, eyes:0x33403a, steel:0x6a7078}, pelvisY:0.93, armMode:'relaxed',
  parts:[
    {id:'pelvis',type:'group',parent:null,pos:[0,0.93,0]},
    {id:'hips',type:'sphere',parent:'pelvis',mat:'skin',tag:'legs',ow:0.05,r:0.34,wseg:14,hseg:10,scale:[1.12,0.66,0.9]},
    {id:'shorts',type:'cyl',parent:'pelvis',mat:'primary',tag:'legs',ow:0.05,rt:0.35,rb:0.30,h:0.3,rseg:16,pos:[0,-0.1,0]},
    {id:'hipL',type:'group',parent:'pelvis',pos:[-0.19,-0.05,0]},
    {id:'thighL',type:'cyl',parent:'hipL',mat:'skin',tag:'legs',ow:0.03,rt:0.16,rb:0.13,h:0.46,rseg:12,pos:[0,-0.23,0]},
    {id:'kneeL',type:'group',parent:'hipL',pos:[0,-0.46,0]},
    {id:'shinL',type:'cyl',parent:'kneeL',mat:'skin',tag:'legs',ow:0.03,rt:0.125,rb:0.1,h:0.44,rseg:12,pos:[0,-0.22,0]},
    {id:'footL',type:'box',parent:'kneeL',mat:'skin',tag:'legs',ow:0.03,w:0.17,h:0.12,d:0.36,pos:[0,-0.44,0.09]},
    {id:'hipR',type:'group',parent:'pelvis',pos:[0.19,-0.05,0]},
    {id:'thighR',type:'cyl',parent:'hipR',mat:'skin',tag:'legs',ow:0.03,rt:0.16,rb:0.13,h:0.46,rseg:12,pos:[0,-0.23,0]},
    {id:'kneeR',type:'group',parent:'hipR',pos:[0,-0.46,0]},
    {id:'shinR',type:'cyl',parent:'kneeR',mat:'skin',tag:'legs',ow:0.03,rt:0.125,rb:0.1,h:0.44,rseg:12,pos:[0,-0.22,0]},
    {id:'footR',type:'box',parent:'kneeR',mat:'skin',tag:'legs',ow:0.03,w:0.17,h:0.12,d:0.36,pos:[0,-0.44,0.09]},
    {id:'spine',type:'group',parent:'pelvis',pos:[0,0.05,0]},
    {id:'torso',type:'cyl',parent:'spine',mat:'skin',tag:'body',ow:0.05,rt:0.33,rb:0.25,h:0.98,rseg:18,pos:[0,0.42,0]},
    {id:'chest',type:'group',parent:'spine',pos:[0,0.7,0]},
    {id:'shoulders',type:'cyl',parent:'chest',mat:'skin',tag:'body',ow:0.05,rt:0.4,rb:0.36,h:0.24,rseg:18,pos:[0,0.02,0]},
    {id:'bustL',type:'sphere',parent:'chest',mat:'skin',tag:'body',ow:0.03,noOutline:true,r:0.11,wseg:12,hseg:10,pos:[-0.1,-0.14,0.13]},
    {id:'bustR',type:'sphere',parent:'chest',mat:'skin',tag:'body',ow:0.03,noOutline:true,r:0.11,wseg:12,hseg:10,pos:[0.1,-0.14,0.13]},
    {id:'band',type:'cyl',parent:'chest',mat:'primary',tag:'body',ow:0.04,rt:0.35,rb:0.34,h:0.2,rseg:18,pos:[0,-0.13,0]},
    {id:'neck',type:'group',parent:'chest',pos:[0,0.18,0]},
    {id:'neckM',type:'cyl',parent:'neck',mat:'skin',tag:'body',ow:0.03,noOutline:true,rt:0.1,rb:0.13,h:0.16,rseg:12,pos:[0,0.0,0]},
    {id:'head',type:'sphere',parent:'neck',mat:'skin',tag:'head',ow:0.05,r:0.34,wseg:18,hseg:16,pos:[0,0.28,0],scale:[0.98,1.06,0.98]},
    {id:'hair',type:'sphere',parent:'neck',mat:'hair',tag:'head',ow:0.04,r:0.36,wseg:16,hseg:14,pos:[0,0.44,-0.1],scale:[1.08,1.0,1.18]},
    {id:'ponytail',type:'cyl',parent:'neck',mat:'hair',tag:'head',ow:0.03,rt:0.1,rb:0.05,h:0.5,rseg:8,pos:[0,0.2,-0.34],rot:[0.5,0,0]},
    {id:'eyeL',type:'sphere',parent:'neck',mat:'eyes',tag:'head',ow:0,noOutline:true,r:0.048,wseg:8,hseg:6,pos:[-0.11,0.29,0.29]},
    {id:'eyeR',type:'sphere',parent:'neck',mat:'eyes',tag:'head',ow:0,noOutline:true,r:0.048,wseg:8,hseg:6,pos:[0.11,0.29,0.29]},
    {id:'browL',type:'box',parent:'neck',mat:'hair',tag:'head',ow:0.02,w:0.1,h:0.024,d:0.045,pos:[-0.11,0.355,0.30],rot:[0,0,0.12]},
    {id:'browR',type:'box',parent:'neck',mat:'hair',tag:'head',ow:0.02,w:0.1,h:0.024,d:0.045,pos:[0.11,0.355,0.30],rot:[0,0,-0.12]},
    {id:'nose',type:'sphere',parent:'neck',mat:'skin',tag:'head',ow:0,noOutline:true,r:0.04,wseg:8,hseg:8,scale:[1.0,1.55,1.3],pos:[0,0.25,0.32]},
    {id:'shR',type:'group',parent:'chest',pos:[0.46,0.06,0.04]},
    {id:'upR',type:'cyl',parent:'shR',mat:'skin',tag:'body',ow:0.03,rt:0.1,rb:0.085,h:0.58,rseg:12,pos:[0,-0.29,0]},
    {id:'padR',type:'sphere',parent:'shR',mat:'pad',tag:'body',ow:0.04,r:0.16,wseg:12,hseg:10,scale:[1.05,0.58,1.05],pos:[0,0.05,0]},
    {id:'elbowR',type:'group',parent:'shR',pos:[0,-0.58,0]},
    {id:'farmR',type:'cyl',parent:'elbowR',mat:'skin',tag:'body',ow:0.03,rt:0.085,rb:0.07,h:0.59,rseg:12,pos:[0,-0.295,0]},
    {id:'handR',type:'sphere',parent:'elbowR',mat:'skin',tag:'body',ow:0.03,r:0.095,wseg:10,hseg:8,pos:[0,-0.59,0]},
    {id:'shL',type:'group',parent:'chest',pos:[-0.46,0.06,0.04]},
    {id:'upL',type:'cyl',parent:'shL',mat:'skin',tag:'body',ow:0.03,rt:0.1,rb:0.085,h:0.58,rseg:12,pos:[0,-0.29,0]},
    {id:'padL',type:'sphere',parent:'shL',mat:'pad',tag:'body',ow:0.04,r:0.16,wseg:12,hseg:10,scale:[1.05,0.58,1.05],pos:[0,0.05,0]},
    {id:'elbowL',type:'group',parent:'shL',pos:[0,-0.58,0]},
    {id:'farmL',type:'cyl',parent:'elbowL',mat:'skin',tag:'body',ow:0.03,rt:0.085,rb:0.07,h:0.59,rseg:12,pos:[0,-0.295,0]},
    {id:'handL',type:'sphere',parent:'elbowL',mat:'skin',tag:'body',ow:0.03,r:0.095,wseg:10,hseg:8,pos:[0,-0.59,0]},
  ],
  joints:{pelvis:'pelvis',spine:'spine',chest:'chest',neck:'neck',hipL:'hipL',hipR:'hipR',kneeL:'kneeL',kneeR:'kneeR',shR:'shR',elR:'elbowR',shL:'shL',elL:'elbowL',handR:'handR',handL:'handL'},
  hit:_BODY_HIT
};
/* 70/20/10 FACTION COLOUR SYSTEM. Each faction = three cloth roles: BaseGear (dark grounding under-layers,
 * the ~70 percent), FactionPrimary (bold identity on the big outer pieces, the ~20 percent), and HighlightTrim
 * (punchy piping/edges, the ~10 percent -- and, in this build, the recoloured ink silhouette for max pop).
 * Skin / hair / eyes are per-character and are never faction-locked. Steel (helmets/hardware) stays neutral. */
/* ============================================================================
 * PHASE B -- clothing / gear PARTS REGISTRY. Each slot maps id -> fn(sex) returning recipe-parts that
 * LAYER onto a base body (parented to existing rig node ids, coloured via material roles base/primary/trim
 * so they inherit the faction system for free). Geometries are swappable here without touching buildEntity
 * or the Lab. A saved character stays tiny: {sex, faction, slots, colours}. Helmets are steel with a
 * faction-PRIMARY centre crest (per spec: steel lid + coloured line over the crown).
 * ========================================================================== */
const CPARTS={
  headgear:{
    none:()=>[],
    brodie:(sex)=>{ const f=(sex==='f'), cy=(f?0.42:0.44), s=(f?0.95:1.0); return [
      {id:'hg_dome',type:'lathe',parent:'neck',mat:'steel',tag:'head',ow:0.04,rseg:20,pos:[0,cy,-0.01],scale:[s,s,s],
        profile:[[0.46,-0.03],[0.44,-0.01],[0.4,0.02],[0.37,0.07],[0.33,0.13],[0.27,0.19],[0.19,0.25],[0.1,0.29],[0.0,0.31]]},
      {id:'hg_crest',type:'box',parent:'neck',mat:'primary',tag:'head',ow:0.02,w:0.06,h:0.12,d:0.42,pos:[0,cy+0.27*s,-0.02],scale:[1,1,s]} ]; },
    m1:(sex)=>{ const f=(sex==='f'), cy=(f?0.40:0.42), s=(f?0.95:1.0); return [
      {id:'hg_dome',type:'lathe',parent:'neck',mat:'steel',tag:'head',ow:0.04,rseg:16,pos:[0,cy,-0.01],scale:[s,s,s],
        profile:[[0.44,0.0],[0.42,0.03],[0.40,0.08],[0.36,0.16],[0.30,0.24],[0.22,0.30],[0.12,0.33],[0.0,0.34]]},
      {id:'hg_visor',type:'box',parent:'neck',mat:'steel',tag:'head',ow:0.03,w:0.46,h:0.05,d:0.2,pos:[0,cy+0.02,0.3],scale:[s,1,s]},
      {id:'hg_crest',type:'box',parent:'neck',mat:'primary',tag:'head',ow:0.02,w:0.07,h:0.12,d:0.7,pos:[0,cy+0.3*s,-0.01],scale:[1,1,s]} ]; },
    cap:(sex)=>{ const f=(sex==='f'), cy=(f?0.42:0.44), s=(f?0.95:1.0); return [ {id:'cp_crown',type:'cyl',parent:'neck',mat:'base',tag:'head',ow:0.04,rt:0.35*s,rb:0.4*s,h:0.36,rseg:18,pos:[0,cy+0.14,-0.03]}, {id:'cp_band',type:'cyl',parent:'neck',mat:'primary',tag:'head',ow:0.03,rt:0.41*s,rb:0.41*s,h:0.07,rseg:18,pos:[0,cy-0.04,-0.02]}, {id:'cp_bill',type:'box',parent:'neck',mat:'base',tag:'head',ow:0.03,w:0.42*s,h:0.05,d:0.28,pos:[0,cy-0.04,0.32],rot:[0.18,0,0]} ]; },
  },
  torso:{
    none:()=>[],
    tunic:(sex)=>{ const f=(sex==='f'); return [
      {id:'tn_body',type:'cyl',parent:'spine',mat:'base',tag:'body',ow:0.05,rt:(f?0.42:0.52),rb:(f?0.29:0.35),h:(f?0.95:0.98),rseg:18,pos:[0,0.43,0]},
      {id:'tn_slR',type:'cyl',parent:'shR',mat:'base',tag:'body',ow:0.03,rt:(f?0.125:0.16),rb:(f?0.105:0.13),h:0.5,rseg:12,pos:[0,-0.24,0]},
      {id:'tn_slL',type:'cyl',parent:'shL',mat:'base',tag:'body',ow:0.03,rt:(f?0.125:0.16),rb:(f?0.105:0.13),h:0.5,rseg:12,pos:[0,-0.24,0]},
      {id:'tn_collar',type:'cyl',parent:'chest',mat:'primary',tag:'body',ow:0.03,rt:(f?0.19:0.22),rb:(f?0.23:0.27),h:0.1,rseg:16,pos:[0,0.09,0]},
      {id:'tn_placket',type:'box',parent:'spine',mat:'primary',tag:'body',ow:0.03,w:0.08,h:0.86,d:0.06,pos:[0,0.43,(f?0.31:0.38)]} ]; },
    greatcoat:(sex)=>{ const f=(sex==='f'); return [ {id:'gc_body',type:'cyl',parent:'spine',mat:'primary',tag:'body',ow:0.05,rt:(f?0.4:0.48),rb:(f?0.34:0.42),h:(f?0.95:0.98),rseg:18,pos:[0,0.43,0]}, {id:'gc_chest',type:'cyl',parent:'chest',mat:'base',tag:'body',ow:0.04,rt:(f?0.42:0.52),rb:(f?0.38:0.47),h:(f?0.42:0.46),pos:[0,-0.05,0]}, {id:'gc_skFR',type:'lathe',parent:'pelvis',mat:'base',tag:'legs',ow:0.05,rseg:3,phiStart:0,phiLen:Math.PI/2,pos:[0,0.05,0],profile:[[(f?0.52:0.58),-0.72],[(f?0.5:0.56),-0.58],[(f?0.46:0.52),-0.38],[(f?0.4:0.46),-0.18],[(f?0.34:0.4),0.0]]}, {id:'gc_skBR',type:'lathe',parent:'pelvis',mat:'base',tag:'legs',ow:0.05,rseg:3,phiStart:Math.PI/2,phiLen:Math.PI/2,pos:[0,0.05,0],profile:[[(f?0.52:0.58),-0.72],[(f?0.5:0.56),-0.58],[(f?0.46:0.52),-0.38],[(f?0.4:0.46),-0.18],[(f?0.34:0.4),0.0]]}, {id:'gc_skBL',type:'lathe',parent:'pelvis',mat:'base',tag:'legs',ow:0.05,rseg:3,phiStart:Math.PI,phiLen:Math.PI/2,pos:[0,0.05,0],profile:[[(f?0.52:0.58),-0.72],[(f?0.5:0.56),-0.58],[(f?0.46:0.52),-0.38],[(f?0.4:0.46),-0.18],[(f?0.34:0.4),0.0]]}, {id:'gc_skFL',type:'lathe',parent:'pelvis',mat:'base',tag:'legs',ow:0.05,rseg:3,phiStart:Math.PI*1.5,phiLen:Math.PI/2,pos:[0,0.05,0],profile:[[(f?0.52:0.58),-0.72],[(f?0.5:0.56),-0.58],[(f?0.46:0.52),-0.38],[(f?0.4:0.46),-0.18],[(f?0.34:0.4),0.0]]}, {id:'gc_slR',type:'cyl',parent:'shR',mat:'base',tag:'body',ow:0.03,rt:(f?0.13:0.17),rb:(f?0.11:0.14),h:0.52,rseg:12,pos:[0,-0.25,0]}, {id:'gc_slL',type:'cyl',parent:'shL',mat:'base',tag:'body',ow:0.03,rt:(f?0.13:0.17),rb:(f?0.11:0.14),h:0.52,rseg:12,pos:[0,-0.25,0]}, {id:'gc_collar',type:'cyl',parent:'chest',mat:'primary',tag:'body',ow:0.04,rt:(f?0.26:0.3),rb:(f?0.2:0.23),h:0.16,rseg:16,pos:[0,0.2,0]}, {id:'gc_placket',type:'box',parent:'spine',mat:'trim',tag:'body',ow:0.03,w:0.12,h:0.5,d:0.06,pos:[0,0.55,(f?0.42:0.52)]} ]; },
  },
  legs:{
    none:()=>[],
    trousers:(sex)=>{ const f=(sex==='f'); return [
      {id:'tr_hip',type:'cyl',parent:'pelvis',mat:'base',tag:'legs',ow:0.05,rt:(f?0.37:0.4),rb:(f?0.33:0.36),h:0.34,rseg:16,pos:[0,-0.1,0]},
      {id:'tr_thR',type:'cyl',parent:'hipR',mat:'base',tag:'legs',ow:0.03,rt:(f?0.18:0.21),rb:(f?0.15:0.18),h:0.48,rseg:12,pos:[0,-0.23,0]},
      {id:'tr_shR',type:'cyl',parent:'kneeR',mat:'base',tag:'legs',ow:0.03,rt:(f?0.14:0.17),rb:(f?0.115:0.135),h:0.46,rseg:12,pos:[0,-0.22,0]},
      {id:'tr_thL',type:'cyl',parent:'hipL',mat:'base',tag:'legs',ow:0.03,rt:(f?0.18:0.21),rb:(f?0.15:0.18),h:0.48,rseg:12,pos:[0,-0.23,0]},
      {id:'tr_shL',type:'cyl',parent:'kneeL',mat:'base',tag:'legs',ow:0.03,rt:(f?0.14:0.17),rb:(f?0.115:0.135),h:0.46,rseg:12,pos:[0,-0.22,0]} ]; },
  },
  feet:{
    none:()=>[],
    boots:(sex)=>{ const f=(sex==='f'); return [ {id:'bt_footR',type:'box',parent:'kneeR',mat:'leather',tag:'legs',ow:0.03,w:(f?0.19:0.22),h:(f?0.14:0.16),d:(f?0.38:0.42),pos:[0,-0.44,0.1]}, {id:'bt_footL',type:'box',parent:'kneeL',mat:'leather',tag:'legs',ow:0.03,w:(f?0.19:0.22),h:(f?0.14:0.16),d:(f?0.38:0.42),pos:[0,-0.44,0.1]}, {id:'bt_cuffR',type:'cyl',parent:'kneeR',mat:'leather',tag:'legs',ow:0.03,rt:(f?0.15:0.17),rb:(f?0.16:0.18),h:0.18,rseg:12,pos:[0,-0.36,0.01]}, {id:'bt_cuffL',type:'cyl',parent:'kneeL',mat:'leather',tag:'legs',ow:0.03,rt:(f?0.15:0.17),rb:(f?0.16:0.18),h:0.18,rseg:12,pos:[0,-0.36,0.01]} ]; },
  },
  webbing:{
    none:()=>[],
    webbing:(sex)=>{ const f=(sex==='f'); return [ {id:'wb_belt',type:'cyl',parent:'pelvis',mat:'leather',tag:'body',ow:0.04,rt:(f?0.33:0.38),rb:(f?0.32:0.37),h:0.15,rseg:18,pos:[0,0.12,0]}, {id:'wb_buckle',type:'box',parent:'pelvis',mat:'trim',tag:'body',ow:0.03,w:0.13,h:0.13,d:0.05,pos:[0,0.12,(f?0.37:0.42)]}, {id:'wb_pouchR',type:'box',parent:'pelvis',mat:'leather',tag:'body',ow:0.03,w:0.15,h:0.18,d:0.11,pos:[(f?0.17:0.21),0.05,(f?0.27:0.31)]}, {id:'wb_flapR',type:'box',parent:'pelvis',mat:'leather',tag:'body',ow:0.02,w:0.16,h:0.05,d:0.13,pos:[(f?0.17:0.21),0.145,(f?0.27:0.31)]}, {id:'wb_pouchL',type:'box',parent:'pelvis',mat:'leather',tag:'body',ow:0.03,w:0.15,h:0.18,d:0.11,pos:[(f?-0.17:-0.21),0.05,(f?0.27:0.31)]}, {id:'wb_flapL',type:'box',parent:'pelvis',mat:'leather',tag:'body',ow:0.02,w:0.16,h:0.05,d:0.13,pos:[(f?-0.17:-0.21),0.145,(f?0.27:0.31)]} ]; },
  },
  hair:{
    default:()=>[],
    none:()=>[],
    crop:(sex)=>{ const f=(sex==='f'); return [ {id:'hs_crop',type:'sphere',parent:'neck',mat:'hair',tag:'head',ow:0.04,r:(f?0.36:0.375),wseg:16,hseg:12,pos:[0,0.4,(f?-0.09:-0.07)],scale:[1.05,0.68,1.08]} ]; },
    swept:(sex)=>{ const f=(sex==='f'); return [ {id:'hs_sw_main',type:'sphere',parent:'neck',mat:'hair',tag:'head',ow:0.04,r:(f?0.36:0.375),wseg:16,hseg:12,pos:[0,0.44,(f?-0.09:-0.08)],scale:[1.05,0.9,1.1]}, {id:'hs_sw_front',type:'box',parent:'neck',mat:'hair',tag:'head',ow:0.03,w:(f?0.3:0.34),h:0.14,d:0.14,pos:[0,0.56,(f?0.06:0.08)],rot:[-0.5,0,0]} ]; },
    sidepart:(sex)=>{ const f=(sex==='f'); return [ {id:'hs_sp_main',type:'sphere',parent:'neck',mat:'hair',tag:'head',ow:0.04,r:(f?0.36:0.375),wseg:16,hseg:12,pos:[0.02,0.44,(f?-0.09:-0.08)],scale:[1.06,0.88,1.1]}, {id:'hs_sp_sweep',type:'box',parent:'neck',mat:'hair',tag:'head',ow:0.03,w:0.13,h:0.1,d:0.28,pos:[(f?0.13:0.15),0.5,(f?-0.06:-0.05)],rot:[0,0,-0.18]} ]; },
    bun:(sex)=>{ const f=(sex==='f'); return [ {id:'hs_bun_cap',type:'sphere',parent:'neck',mat:'hair',tag:'head',ow:0.04,r:(f?0.355:0.365),wseg:16,hseg:12,pos:[0,0.42,(f?-0.1:-0.08)],scale:[1.04,0.8,1.08]}, {id:'hs_bun_ball',type:'sphere',parent:'neck',mat:'hair',tag:'head',ow:0.04,r:(f?0.12:0.13),wseg:12,hseg:10,pos:[0,0.5,(f?-0.32:-0.3)]} ]; },
    ponytail:(sex)=>{ const f=(sex==='f'); return [ {id:'hs_pt_cap',type:'sphere',parent:'neck',mat:'hair',tag:'head',ow:0.04,r:(f?0.355:0.365),wseg:16,hseg:12,pos:[0,0.42,(f?-0.1:-0.08)],scale:[1.04,0.82,1.08]}, {id:'hs_pt_tail',type:'cyl',parent:'neck',mat:'hair',tag:'head',ow:0.03,rt:0.1,rb:0.045,h:0.55,rseg:8,pos:[0,(f?0.22:0.24),(f?-0.34:-0.32)],rot:[0.5,0,0]} ]; },
    mohawk:(sex)=>{ const f=(sex==='f'); return [ {id:'hs_moh',type:'sphere',parent:'neck',mat:'hair',tag:'head',ow:0.04,r:0.3,wseg:12,hseg:10,pos:[0,(f?0.56:0.58),(f?-0.06:-0.05)],scale:[0.26,0.78,1.4]} ]; },
  },
  face:{
    none:()=>[],
    stubble:(sex)=>{ const f=(sex==='f'); return [ {id:'fh_stub',type:'sphere',parent:'neck',mat:'hair',tag:'head',ow:0,noOutline:true,r:(f?0.19:0.2),wseg:16,hseg:12,pos:[0,(f?0.15:0.17),(f?0.095:0.115)],scale:[1.8,0.92,1.12]}, {id:'fh_stub_lip',type:'sphere',parent:'neck',mat:'hair',tag:'head',ow:0,noOutline:true,r:(f?0.1:0.11),wseg:12,hseg:8,pos:[0,(f?0.205:0.235),(f?0.285:0.31)],scale:[1.35,0.5,0.55]} ]; },
    mustache:(sex)=>{ const f=(sex==='f'); return [ {id:'fh_must_l',type:'box',parent:'neck',mat:'hair',tag:'head',ow:0.03,w:(f?0.12:0.13),h:0.05,d:0.06,pos:[(f?-0.055:-0.06),(f?0.205:0.225),(f?0.31:0.34)],rot:[0,0.15,0.12]}, {id:'fh_must_r',type:'box',parent:'neck',mat:'hair',tag:'head',ow:0.03,w:(f?0.12:0.13),h:0.05,d:0.06,pos:[(f?0.055:0.06),(f?0.205:0.225),(f?0.31:0.34)],rot:[0,-0.15,-0.12]} ]; },
    goatee:(sex)=>{ const f=(sex==='f'); return [ {id:'fh_go_ml',type:'box',parent:'neck',mat:'hair',tag:'head',ow:0.03,w:(f?0.11:0.12),h:0.045,d:0.055,pos:[(f?-0.05:-0.055),(f?0.205:0.225),(f?0.31:0.34)],rot:[0,0.15,0.12]}, {id:'fh_go_mr',type:'box',parent:'neck',mat:'hair',tag:'head',ow:0.03,w:(f?0.11:0.12),h:0.045,d:0.055,pos:[(f?0.05:0.055),(f?0.205:0.225),(f?0.31:0.34)],rot:[0,-0.15,-0.12]}, {id:'fh_go_chin',type:'sphere',parent:'neck',mat:'hair',tag:'head',ow:0.04,r:(f?0.055:0.06),wseg:12,hseg:10,pos:[0,(f?0.095:0.11),(f?0.255:0.28)],scale:[1.5,1.15,1.4]} ]; },
    beard:(sex)=>{ const f=(sex==='f'); return [ {id:'fh_bd',type:'sphere',parent:'neck',mat:'hair',tag:'head',ow:0.04,r:(f?0.19:0.2),wseg:16,hseg:14,pos:[0,(f?0.13:0.15),(f?0.1:0.12)],scale:[1.88,0.72,1.3]}, {id:'fh_bd_ml',type:'box',parent:'neck',mat:'hair',tag:'head',ow:0.03,w:(f?0.12:0.13),h:0.05,d:0.06,pos:[(f?-0.055:-0.06),(f?0.205:0.225),(f?0.31:0.34)],rot:[0,0.15,0.12]}, {id:'fh_bd_mr',type:'box',parent:'neck',mat:'hair',tag:'head',ow:0.03,w:(f?0.12:0.13),h:0.05,d:0.06,pos:[(f?0.055:0.06),(f?0.205:0.225),(f?0.31:0.34)],rot:[0,-0.15,-0.12]} ]; },
  },
};
// Compose a full recipe from a compact selection {sex, headgear, ...}: clone the base body, append equipped slot parts.
// Reused by the Lab (clAssembled) and available to the game later for spawning kitted units.
function assembleChar(sel){ sel=sel||{};
  const base=(sel.sex==='f')?CHAR_FEMALE:CHAR_MALE; const def=JSON.parse(JSON.stringify(base));
  const hide={}; if(sel.torso&&sel.torso!=='none')hide.band=1; if(sel.legs&&sel.legs!=='none')hide.shorts=1; if(sel.headgear&&sel.headgear!=='none'){hide.hair=1;hide.ponytail=1;} if(sel.hair&&sel.hair!=='default'){hide.hair=1;hide.ponytail=1;}
  if(Object.keys(hide).length)def.parts=def.parts.filter(p=>!hide[p.id]);
  for(const slot in CPARTS){ const id=sel[slot]; if(!id||id==='none')continue; if(slot==='hair'&&sel.headgear&&sel.headgear!=='none')continue; const fn=CPARTS[slot][id]; if(!fn)continue; const ex=fn(sel.sex); if(ex&&ex.length)def.parts=def.parts.concat(ex); }
  return def;
}
const FACTION_PALETTES={
  Soviets:{ BaseGear:0x432818, FactionPrimary:0xD41414, HighlightTrim:0xFDF0D5, CoatAccent:0xD41414, PadColor:0xD41414 },  // [dial] G==B kills the pink/magenta tilt under the blue fill
  Allies: { BaseGear:0x00296B, FactionPrimary:0x1F6FF0, HighlightTrim:0xFDC500, CoatAccent:0x1F6FF0, PadColor:0xFDC500 },
  Empire: { BaseGear:0x240046, FactionPrimary:0x7B2FD6, HighlightTrim:0xFF6D00, CoatAccent:0xFF6D00, PadColor:0xFF6D00 }
};
// Recolour a built character to a faction. Role-tagged cloth meshes -> their role colour; the ink-outline
// silhouette -> HighlightTrim (option-3 full-silhouette pop). Skin/hair/eyes/steel are left untouched.
// Works on MeshStandard (pre-toon) or MeshToon (post-toon) meshes -- both expose .color.
function applyFaction(group, faction){
  const Pf=FACTION_PALETTES[faction]; if(!Pf||!group)return;
  group.traverse(o=>{
    if(o.userData&&o.userData.isOutline){ return; }   // Living Comic: character outlines stay at shared dark INK (Empire elite sets its own purple explicitly)
    if(o.isMesh&&o.userData&&o.userData.role&&o.material&&o.material.color){
      const r=o.userData.role; const c=(r==='base')?Pf.BaseGear:(r==='primary')?Pf.FactionPrimary:(r==='coat')?Pf.CoatAccent:(r==='pad')?Pf.PadColor:Pf.HighlightTrim;
      o.material.color.set(c);
    }
  });
  group.userData.faction=faction;
}
function buildEntity(def,opts){ opts=opts||{};
  const g=new THREE.Group();
  const FP=(opts.faction&&FACTION_PALETTES[opts.faction])?FACTION_PALETTES[opts.faction]:null;
  const mats={
    base:   new THREE.MeshStandardMaterial({color:(FP?FP.BaseGear:(opts.base!==undefined?opts.base:0x6b6f76)),roughness:0.92,metalness:0.04}),
    primary:new THREE.MeshStandardMaterial({color:(FP?FP.FactionPrimary:(def.colors.primary!==undefined?def.colors.primary:0xC1121F)),roughness:0.85,metalness:0.05}),
    trim:   new THREE.MeshStandardMaterial({color:(FP?FP.HighlightTrim:(def.colors.trim!==undefined?def.colors.trim:0xFDF0D5)),roughness:0.6,metalness:0.12}),
    skin:   new THREE.MeshStandardMaterial({color:(opts.skin!==undefined?opts.skin:(def.colors.skin!==undefined?def.colors.skin:0xd9a077)),roughness:0.78,metalness:0.0}),
    hair:   new THREE.MeshStandardMaterial({color:(opts.hair!==undefined?opts.hair:(def.colors.hair!==undefined?def.colors.hair:0x3a2a1c)),roughness:0.8,metalness:0.02}),
    eyes:   new THREE.MeshStandardMaterial({color:(opts.eyes!==undefined?opts.eyes:(def.colors.eyes!==undefined?def.colors.eyes:0x243038)),roughness:0.32,metalness:0.0}),
    steel:  new THREE.MeshStandardMaterial({color:(def.colors.steel!==undefined?def.colors.steel:0x6a7078),roughness:0.5,metalness:0.55,flatShading:true}),
    dark:   new THREE.MeshStandardMaterial({color:def.colors.dark,roughness:0.55,metalness:0.35}),
    leather:new THREE.MeshStandardMaterial({color:0x2e2117,roughness:0.72,metalness:0.08}),
    accent: new THREE.MeshStandardMaterial({color:(opts.accent!==undefined?opts.accent:0xffffff),roughness:0.45}),
    coat:   new THREE.MeshStandardMaterial({color:(FP?FP.CoatAccent:0xC1121F),roughness:0.85,metalness:0.05}),
    pad:    new THREE.MeshStandardMaterial({color:(FP?FP.PadColor:0xC1121F),roughness:0.85,metalness:0.05})
  };
  const geomOf=p=>{
    if(p.type==='sphere')return new THREE.SphereGeometry(p.r,p.wseg||12,p.hseg||10);
    if(p.type==='cyl')return new THREE.CylinderGeometry(p.rt,p.rb,p.h,p.rseg||12);
    if(p.type==='box')return new THREE.BoxGeometry(p.w,p.h,p.d);
    if(p.type==='cone')return new THREE.ConeGeometry(p.r,p.h,p.rseg||12);
    if(p.type==='torus')return new THREE.TorusGeometry(p.r,p.tube,p.rseg||8,p.tseg||16,(p.arc!==undefined?p.arc:Math.PI*2));
    if(p.type==='lathe'){ const pts=(p.profile||[]).map(a=>new THREE.Vector2(a[0],a[1])); return new THREE.LatheGeometry(pts,p.rseg||10,(p.phiStart||0),(p.phiLen!==undefined?p.phiLen:Math.PI*2)); }
    if(p.type==='extrude'){ const s=new THREE.Shape(); const pv=p.shape||[]; if(pv.length){ s.moveTo(pv[0][0],pv[0][1]); for(let i=1;i<pv.length;i++)s.lineTo(pv[i][0],pv[i][1]); } return new THREE.ExtrudeGeometry(s,{depth:(p.depth!==undefined?p.depth:0.1),bevelEnabled:!!p.bevel,bevelThickness:(p.bevelT||0.02),bevelSize:(p.bevelS||0.02),bevelSegments:(p.bevelSeg||1),steps:1}); }
    if(p.type==='tube'){ const cp=(p.pts||[]).map(a=>new THREE.Vector3(a[0],a[1],a[2])); const cv=new THREE.CatmullRomCurve3(cp); return new THREE.TubeGeometry(cv,(p.tubeSeg||16),(p.tubeR||0.05),(p.radSeg||8),!!p.closed); }
    if(p.type==='plane')return new THREE.PlaneGeometry(p.w,p.h);
    return null;
  };
  const N={};
  for(const p of def.parts){ let node;
    if(p.type==='group'){ node=new THREE.Group(); }
    else { node=new THREE.Mesh(geomOf(p),mats[p.mat]||mats.base); node.castShadow=true; node.userData.part=p.tag; node.userData.outlineW=p.ow;
      node.userData.matKind=(p.mat==='skin')?'skin':(p.mat==='steel')?'metal':(p.mat==='base'||p.mat==='primary'||p.mat==='trim'||p.mat==='leather'||p.mat==='coat'||p.mat==='pad')?'cloth':'default';
      if(p.mat==='base'||p.mat==='primary'||p.mat==='trim'||p.mat==='coat'||p.mat==='pad')node.userData.role=p.mat;
      if(p.noOutline)node.userData.noOutline=true; }
    if(p.pos)node.position.set(p.pos[0],p.pos[1],p.pos[2]);
    if(p.scale)node.scale.set(p.scale[0],p.scale[1],p.scale[2]);
    if(p.rot)node.rotation.set(p.rot[0],p.rot[1],p.rot[2]);
    N[p.id]=node; node.userData.pid=p.id; (p.parent?N[p.parent]:g).add(node); }
  if(def.weapon){ const w=def.weapon; const wpn=new THREE.Group(); wpn.position.set(w.pos[0],w.pos[1],w.pos[2]);
    const gun=(w.make==='AMKRifle')?makeAMKRifle():new THREE.Group();
    const muzzle=new THREE.Object3D(); muzzle.position.set(w.muzzle[0],w.muzzle[1],w.muzzle[2]);
    wpn.add(gun,muzzle); (w.parent?N[w.parent]:g).add(wpn); g.userData.wpn=wpn; g.userData.muzzle=muzzle; N[w.id]=wpn; }
  const j={}; for(const k in def.joints)j[k]=N[def.joints[k]]; g.userData.joints=j;
  g.userData.head=N.head||null; g.userData.phase=Math.random()*6.28; g.userData.pelvisY=(def.pelvisY!==undefined?def.pelvisY:0.95);
  g.userData.mats=mats; if(opts.faction)g.userData.faction=opts.faction;
  if(def.armMode)poseArms(g,def.armMode);
  return g;
}
// ===== Gameplay soldiers dressed in the Character-Lab wardrobe, per faction =====
// assembleChar builds off CHAR_MALE/FEMALE, which (unlike CHAR_DEFAULT) carry NO weapon mount and default
// to a 'relaxed' arm pose -> inject the AMK-rifle mount + rifle pose so the soldier holds + fires a gun.
// _BODY_HIT is shared across all bodies, so this is purely cosmetic over a fair, identical target.
function makeAMKRifle(){
  // AMK Rifle I -- flintlock musket with a flared blunderbuss muzzle. Built so the GRIP sits at the
  // group's (0,0,0) (= the right hand) and the barrel runs +Z (= the aim axis). The rig pitches this
  // group on local X for recoil, so we deliberately do NOT rotate the group; only the barrel cylinder
  // is rotated (its default +Y height laid onto +Z).
  const g=new THREE.Group();
  const matWood =new THREE.MeshStandardMaterial({color:0x6e4a2a, roughness:0.85, metalness:0.0});   // warm brown
  const matMetal=new THREE.MeshStandardMaterial({color:0x2a2e33, roughness:0.5, metalness:0.55, flatShading:true}); // dark gunmetal
  const mk=(geo,mat,outline)=>{ const m=new THREE.Mesh(geo,mat); m.castShadow=true; m.userData.part='body'; if(outline===false)m.userData.noOutline=true; else m.userData.outlineW=0.028; return m; };
  // STOCK (wood): butt -> wrist/grip -> action; the wrist straddles the origin so the hand grips there
  const butt  =mk(new THREE.BoxGeometry(0.13,0.25,0.40),matWood); butt.position.set(0,-0.01,-0.42);
  const wrist =mk(new THREE.BoxGeometry(0.10,0.16,0.34),matWood); wrist.position.set(0,-0.04,-0.13);
  const action=mk(new THREE.BoxGeometry(0.10,0.15,0.26),matWood); action.position.set(0,-0.01,0.06);
  // FLINTLOCK + trigger (metal)
  const breech =mk(new THREE.BoxGeometry(0.08,0.10,0.13),matMetal); breech.position.set(0,0.02,0.15);
  const hammer =mk(new THREE.BoxGeometry(0.035,0.10,0.05),matMetal); hammer.position.set(0,0.10,0.03); hammer.rotation.x=0.55;
  const frizzen=mk(new THREE.BoxGeometry(0.045,0.06,0.045),matMetal,false); frizzen.position.set(0,0.07,0.12);
  const guard  =mk(new THREE.BoxGeometry(0.04,0.02,0.11),matMetal,false); guard.position.set(0,-0.12,0.01);
  // BARREL (metal): uniform shaft + flared trumpet bell, both laid along +Z (radiusTop faces the muzzle mouth)
  const shaft=mk(new THREE.CylinderGeometry(0.052,0.052,0.66,12),matMetal); shaft.rotation.x=Math.PI/2; shaft.position.set(0,0.02,0.50);
  const band =mk(new THREE.CylinderGeometry(0.062,0.062,0.035,12),matMetal,false); band.rotation.x=Math.PI/2; band.position.set(0,0.02,0.42);
  const bell =mk(new THREE.CylinderGeometry(0.15,0.06,0.24,14),matMetal); bell.rotation.x=Math.PI/2; bell.position.set(0,0.02,0.95);
  g.add(butt,wrist,action,breech,hammer,frizzen,guard,shaft,band,bell);
  return g;
}

// ---- GUN KIT: one procedural model per gun + its sockets --------------------------------------------
// Model space matches makeAMKRifle: the trigger GRIP sits near the origin and the barrel runs +Z.
// Sockets (model space): grip = trigger-hand point, stock = butt end (shoulder/flank contact),
// rail = [zMin, zMax, y] the support hand may slide along (the hold solver picks the furthest point
// the arm can reach, so hands spread on long guns and bunch on short ones), muzzle = FX point.
// mass scales sway + recoil; hold picks the stance profile (ANIM.hold in game); reload = choreography style
// (ANIM.reload.styles in game) + the point on the gun the support hand works at. Cosmetic only.
// Fast hypotenuse: Math.hypot is a variadic builtin that allocates on every call (measured: the #1 source of per-frame
// garbage via the occlusion, shot and animation hot loops). These are plain sqrt of squares and inline in optimized code.
function hyp2(a,b){ return Math.sqrt(a*a+b*b); }
function hyp3(a,b,c){ return Math.sqrt(a*a+b*b+c*c); }
function _gunParts(){
  const M={ wood:new THREE.MeshStandardMaterial({color:0x6e4a2a,roughness:0.85}),
            dark:new THREE.MeshStandardMaterial({color:0x2a2e33,roughness:0.5,metalness:0.55,flatShading:true}),
            steel:new THREE.MeshStandardMaterial({color:0x9aa3ad,roughness:0.35,metalness:0.7}),
            brass:new THREE.MeshStandardMaterial({color:0xc9962e,roughness:0.4,metalness:0.6}),
            leather:new THREE.MeshStandardMaterial({color:0x5a3a22,roughness:0.9}),
            glow:new THREE.MeshStandardMaterial({color:0xffa23a,emissive:0xff7a1a,emissiveIntensity:1.2,roughness:0.4}) };
  const g=new THREE.Group(), RZ=[Math.PI/2,0,0], RX=[0,0,Math.PI/2];
  const add=(geo,mat,pos,rot,outline)=>{ const m=new THREE.Mesh(geo,M[mat]); m.castShadow=true; m.userData.part='body';
    if(mat!=='wood'&&mat!=='leather'&&mat!=='glow') m.userData.matKind='metal';
    if(outline===false) m.userData.noOutline=true; else m.userData.outlineW=0.028;
    m.position.set(pos[0],pos[1],pos[2]); if(rot) m.rotation.set(rot[0],rot[1],rot[2]); g.add(m); return m; };
  return { g, add, RZ, RX, B:(w,h,d)=>new THREE.BoxGeometry(w,h,d), C:(r1,r2,l,n)=>new THREE.CylinderGeometry(r1,r2,l,n||12) };
}
function makeBagpipeGun(){   // Bagpipe scattergun: stubby twin barrels with brass bells over a leather "bag" drum
  const {g,add,RZ,B,C}=_gunParts();
  add(B(0.12,0.20,0.32),'wood',[0,-0.02,-0.40]); add(B(0.09,0.14,0.22),'wood',[0,-0.05,-0.14]); add(B(0.15,0.13,0.20),'dark',[0,0.0,0.07]);
  for(const sx of [-0.058,0.058]){ add(C(0.052,0.052,0.56),'dark',[sx,0.035,0.44],RZ); add(C(0.085,0.055,0.12,14),'brass',[sx,0.035,0.76],RZ); }
  add(new THREE.SphereGeometry(0.15,14,10),'leather',[0,-0.11,0.34]).scale.set(1.0,0.82,1.45);
  add(C(0.157,0.157,0.035,16),'brass',[0,-0.11,0.34],RZ,false); add(B(0.03,0.02,0.10),'dark',[0,-0.13,-0.01],null,false);
  return g; }
function makeShabzonSMG(){   // Shabzon SMG: short wooden stock, perforated shroud, the drum magazine silhouette
  const {g,add,RZ,RX,B,C}=_gunParts();
  add(B(0.10,0.17,0.26),'wood',[0,-0.03,-0.34]); add(B(0.08,0.12,0.18),'wood',[0,-0.05,-0.13]); add(B(0.10,0.12,0.30),'dark',[0,0.01,0.12]);
  add(C(0.055,0.055,0.30,10),'dark',[0,0.02,0.42],RZ); for(const z of [0.33,0.42,0.51]) add(B(0.118,0.03,0.035),'steel',[0,0.02,z],null,false);
  add(C(0.03,0.03,0.08,8),'dark',[0,0.02,0.60],RZ,false); add(C(0.13,0.13,0.075,18),'dark',[0,-0.13,0.14],RX); add(C(0.05,0.05,0.085,10),'steel',[0,-0.13,0.14],RX,false);
  add(B(0.03,0.05,0.04),'dark',[0,0.09,0.02],null,false);
  return g; }
function makeLancerRifle(){  // Retro Lancer: boxy burst rifle with the long bayonet blade under the barrel
  const {g,add,RZ,B,C}=_gunParts();
  add(B(0.11,0.19,0.32),'dark',[0,-0.03,-0.38]); add(B(0.08,0.13,0.20),'dark',[0,-0.06,-0.12]); add(B(0.12,0.15,0.44),'steel',[0,0.01,0.14]);
  add(B(0.11,0.11,0.30),'wood',[0,-0.01,0.49]); add(C(0.032,0.032,0.34,10),'dark',[0,0.01,0.80],RZ); add(B(0.07,0.22,0.10),'dark',[0,-0.16,0.16],[0.25,0,0]);
  add(B(0.022,0.07,0.46),'steel',[0,-0.075,0.96]); add(new THREE.ConeGeometry(0.035,0.12,4),'steel',[0,-0.075,1.24],RZ,false); add(B(0.05,0.06,0.06),'dark',[0,0.11,0.06],null,false);
  return g; }
function makeTorqueBow(){    // Torque Bow: stocked bow with swept limbs, cams, string and the glowing torque bolt
  const {g,add,RZ,B,C}=_gunParts();
  add(B(0.10,0.15,0.36),'wood',[0,-0.03,-0.32]); add(B(0.08,0.12,0.18),'wood',[0,-0.05,-0.10]); add(B(0.07,0.08,0.72),'dark',[0,0.02,0.30]); add(B(0.16,0.12,0.12),'steel',[0,0.0,0.56]);
  for(const sd of [-1,1]){ add(B(0.46,0.05,0.07),'dark',[sd*0.25,0.02,0.52],[0,sd*0.38,0]); add(C(0.045,0.045,0.05,10),'brass',[sd*0.46,0.02,0.44],null,false); }
  add(B(0.92,0.012,0.012),'steel',[0,0.02,0.44],null,false); add(C(0.018,0.018,0.62,6),'wood',[0,0.07,0.36],RZ,false); add(new THREE.ConeGeometry(0.045,0.1,6),'glow',[0,0.07,0.71],RZ,false);
  return g; }
const GUN_KIT={
  amk:    { make:makeAMKRifle,    grip:[0,-0.06,-0.08], stock:[0,-0.01,-0.62], rail:[0.28,0.60,-0.03], muzzle:[0,0.02,1.08],  mass:1.0,  hold:'rifle', reload:{style:'ram',at:[0,0.02,1.0]} },
  bagpipe:{ make:makeBagpipeGun,  grip:[0,-0.06,-0.10], stock:[0,-0.02,-0.56], rail:[0.24,0.44,-0.12], muzzle:[0,0.035,0.84], mass:1.15, hold:'scatter', reload:{style:'break',at:[0,0.05,0.10]} },
  shabzon:{ make:makeShabzonSMG,  grip:[0,-0.06,-0.08], stock:[0,-0.03,-0.47], rail:[0.10,0.40,-0.04], muzzle:[0,0.02,0.64],  mass:0.6,  hold:'smg', reload:{style:'drum',at:[0,-0.13,0.14]} },
  lancer: { make:makeLancerRifle, grip:[0,-0.07,-0.08], stock:[0,-0.03,-0.54], rail:[0.36,0.62,-0.06], muzzle:[0,0.01,0.98],  mass:0.9,  hold:'rifle', reload:{style:'mag',at:[0,-0.17,0.16]} },
  torque: { make:makeTorqueBow,   grip:[0,-0.06,-0.06], stock:[0,-0.03,-0.50], rail:[0.30,0.52,-0.04], muzzle:[0,0.07,0.76],  mass:0.85, hold:'rifle', reload:{style:'nock',at:[0,0.07,0.08]} },
};
function buildGunModel(id){ const k=GUN_KIT[id]?id:'amk'; const m=GUN_KIT[k].make(); m.userData.gunId=k; return m; }

// ============================================================================
// ANIM — procedural animation config (Phase 2 layered system). ALL tunables for
// locomotion / aim / actions / springs live in this one block. Tune feel here;
// built blind then polished by video. Speeds are SIM units (walk ~6.3 =
// T.moveSpeed; sprint ~7-10 by class).
// ============================================================================
const _ARMP={
  rifle:  { shR:[-1.34,0.10,0.03], elR:-0.72, shL:[-1.30,0.64,0.10], elL:-1.0 },  // two-hand: R hand on the grip/middle, L hand forward on the foregrip
  melee:  { shR:[-1.12,0.25,0.12], elR:-0.22, shL:[-1.10,0.38,0.18], elL:-0.78 },
  shield: { shR:[-1.35,0.05,0.02], elR:-0.62, shL:[-1.45,0.12,-0.06], elL:-1.05 },
  relaxed:{ shR:[-0.12,0.0,-0.14], elR:-0.16, shL:[-0.12,0.0,0.14], elL:-0.16 }
};
function poseArms(g, mode){
  const j=g.userData.joints; if(!j||!j.shR) return;
  const P=_ARMP[mode]||_ARMP.rifle;
  j.shR.rotation.set(P.shR[0],P.shR[1],P.shR[2]); j.elR.rotation.x=P.elR;
  j.shL.rotation.set(P.shL[0],P.shL[1],P.shL[2]); j.elL.rotation.x=P.elL;
  g.userData.armMode=mode;
}
// ---- 2-bone arm IK: plant a hand on a chest-local target (shoulder -> elbow -> hand; bones rest along -Y) ----
const cabanaRepublicMap = [
"~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~",
"~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~",
"~~~~.................................~~~~",
"~~~...................................~~~",
"~~.....................................~~",
"~~....O...........................O....~~",
"~~...C####.....................####C...~~",
"~~....#.#####...............#####.#....~~",
"~~....#......######.X.######......#....~~",
"~~....#...........................#....~~",
"~~....#...........................#....~~",
"~~....#...........................#....~~",
"~~....#...........C...C...........#....~~",
"~~....#...........................#....~~",
"~~....#......X...C..F..C...X......#....~~",
"~~....#..........#######..........#....~~",
"~~....#.......C.#.C...C.#.C.......#....~~",
"~~....#........#.........#........#....~~",
"~~....#.......#...........#.......#....~~",
"~~....#.C....#.............#....C.#....~~",
"~~....#.....#...............#.....#....~~",
"~~.....#...#.................#...#.....~~",
"~~..P...#.RC.................CR.#...P..~~",
"~~.......#.....................#.......~~",
"~~.......E.....................E.......~~",
"~~..H..K.........................K..H..~~",
"~~~~.................................~~~~",
"~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~",
"~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"
];

// ===== MAP: Cabana Republic — data-driven ASCII grid -> existing map-def -> buildMap() =====
// Authoring is the ASCII above (symmetric, human-readable). parseGridToMapDef compiles it into the
// SAME def shape buildMap already consumes, so the SIM colliders, editor round-trip and toon pass all
// come for free. Team is decided by grid half; objectives (O/X/F) feed the silo/techbox/harvester systems.
const GRID_SIZE = 10.0;
function parseGridToMapDef(grid){
  const H=grid.length, W=grid[0].length, cx=(W-1)/2, cz=(H-1)/2;
  const def={ name:'Cabana Republic', obstacles:[], paths:[], buildings:[], barracks:[], platforms:[], ramps:[],
              silos:[], techPoints:[], spawn:{x:0,z:0}, resupply:{x:0,z:0},
              env:{ fog:{near:260,far:760} } };
  const techNames=['Alpha','Bravo','Charlie']; let ti=0, friendE=null, harv=null;
  for(let r=0;r<H;r++){ const row=grid[r];
    for(let c=0;c<W;c++){
      const ch=row[c]; if(ch==='.'||ch==='~') continue;
      const x=(c-cx)*GRID_SIZE, z=(r-cz)*GRID_SIZE, team=(c<cx?'friend':'hostile');
      switch(ch){
        case '#': def.paths.push({x,z,w:GRID_SIZE,d:GRID_SIZE}); break;
        case 'C': def.obstacles.push({x,z,w:2.6,h:1.5,d:2.6,warm:(((r+c)&1)===0)}); break;
        case 'H': def.buildings.push({team,type:'hq',x,z}); break;
        case 'R': def.buildings.push({team,type:'refinery',x,z}); break;
        case 'P': def.buildings.push({team,type:'power',x,z}); break;
        case 'K': def.buildings.push({team,type:'barracks',x,z}); break;
        case 'E': def.barracks.push({x,z,team}); if(team==='friend') friendE={x,z}; break;
        case 'O': def.silos.push({x,z}); break;
        case 'X': def.techPoints.push({name:(techNames[ti++]||('Point'+ti)),x,z}); break;
        case 'F': harv={x,z}; break;
      }
    }
  }
  // spawn + resupply: just outside the friendly enterable barracks, nudged toward map centre
  if(friendE){ const L=hyp2(friendE.x,friendE.z)||1, ux=-friendE.x/L, uz=-friendE.z/L;
    def.resupply={x:+(friendE.x+ux*7).toFixed(2), z:+(friendE.z+uz*7).toFixed(2)};
    def.spawn   ={x:+(friendE.x+ux*15).toFixed(2), z:+(friendE.z+uz*15).toFixed(2)}; }
  def.harvField = harv || {x:0,z:0};
  return def;
}
const DM_Cabana = parseGridToMapDef(cabanaRepublicMap);
(function(){ const _sx=DM_Cabana.spawn.x, _sz=DM_Cabana.spawn.z, _dir=_sx<0?1:-1; DM_Cabana.platforms=[ {x:_sx+_dir*16,z:_sz,w:10,d:10,h:1.2}, {x:_sx+_dir*30,z:_sz,w:12,d:12,h:2.5}, {x:_sx+_dir*46,z:_sz,w:14,d:14,h:4.0} ]; DM_Cabana.ramps=[ {x:_sx+_dir*30,z:_sz+12,w:12,d:8,h0:0,h1:2.5,rot:Math.PI/2}, {x:_sx+_dir*46,z:_sz+14,w:14,d:8,h0:0,h1:4.0,rot:Math.PI/2} ]; const _roofH=t=>t==='hq'?9:t==='refinery'?4:t==='power'?8:5, _zh=t=>t==='refinery'?3:t==='power'?2.5:t==='hq'?4:3.5; const _fb=(DM_Cabana.buildings||[]).filter(b=>b.team==='friend').sort((a,b)=>_roofH(a.type)-_roofH(b.type)); if(_fb[0]) DM_Cabana.ramps.push({x:_fb[0].x, z:_fb[0].z+_zh(_fb[0].type)+7, w:14, d:8, h0:0, h1:_roofH(_fb[0].type), rot:Math.PI/2}); })();

// ===== Cabana Republic village pass: plain map data (round-trips through the editor like any placed piece) =====
// Designed as the LEFT half and mirrored for team symmetry. Every piece was validated against road tiles, existing
// colliders, objective/spawn clearances, bot lanes and harvester routes (tools/village.py). Keys are the kit names
// buildMap already consumes: houses.bt -> legacy makers, trees.kind -> BK.TREES, props.kind -> BK.PROPS, covers.type -> BK.COVER.
DM_Cabana.houses = [
  {bt:'manor',x:0,z:-104,rot:0}, {bt:'thatchCottage',x:-155,z:-44,rot:1.5708}, {bt:'thatchCottage',x:155,z:-44,rot:-1.5708},
  {bt:'timber',x:-155,z:-21,rot:1.5708}, {bt:'timber',x:155,z:-21,rot:-1.5708}, {bt:'brickFarm',x:-155,z:3,rot:1.5708},
  {bt:'brickFarm',x:155,z:3,rot:-1.5708}, {bt:'timber',x:-155,z:27,rot:1.5708}, {bt:'timber',x:155,z:27,rot:-1.5708},
  {bt:'brickFarm',x:-46,z:-104,rot:0}, {bt:'brickFarm',x:46,z:-104,rot:0}
];
DM_Cabana.trees = [
  {kind:'apple',x:-175,z:-39,seed:20}, {kind:'apple',x:175,z:-39,seed:520}, {kind:'apple',x:-175,z:-16,seed:21}, {kind:'apple',x:175,z:-16,seed:521},
  {kind:'apple',x:-175,z:8,seed:22}, {kind:'apple',x:175,z:8,seed:522}, {kind:'apple',x:-175,z:32,seed:23}, {kind:'apple',x:175,z:32,seed:523},
  {kind:'apple',x:-108,z:-113,seed:40}, {kind:'apple',x:108,z:-113,seed:540}, {kind:'apple',x:-108,z:-104,seed:41}, {kind:'apple',x:108,z:-104,seed:541},
  {kind:'apple',x:-108,z:-95,seed:42}, {kind:'apple',x:108,z:-95,seed:542}, {kind:'apple',x:-99,z:-113,seed:43}, {kind:'apple',x:99,z:-113,seed:543},
  {kind:'apple',x:-99,z:-104,seed:44}, {kind:'apple',x:99,z:-104,seed:544}, {kind:'apple',x:-99,z:-95,seed:45}, {kind:'apple',x:99,z:-95,seed:545},
  {kind:'apple',x:-90,z:-113,seed:46}, {kind:'apple',x:90,z:-113,seed:546}, {kind:'apple',x:-90,z:-104,seed:47}, {kind:'apple',x:90,z:-104,seed:547},
  {kind:'apple',x:-90,z:-95,seed:48}, {kind:'apple',x:90,z:-95,seed:548}, {kind:'apple',x:-81,z:-113,seed:49}, {kind:'apple',x:81,z:-113,seed:549},
  {kind:'apple',x:-81,z:-104,seed:50}, {kind:'apple',x:81,z:-104,seed:550}, {kind:'apple',x:-81,z:-95,seed:51}, {kind:'apple',x:81,z:-95,seed:551},
  {kind:'poplar',x:-100,z:-78,seed:60}, {kind:'poplar',x:100,z:-78,seed:560}, {kind:'poplar',x:-90,z:-82.5,seed:61}, {kind:'poplar',x:90,z:-82.5,seed:561},
  {kind:'poplar',x:-80,z:-81,seed:62}, {kind:'poplar',x:80,z:-81,seed:562}, {kind:'poplar',x:-68,z:-71,seed:70}, {kind:'poplar',x:68,z:-71,seed:570},
  {kind:'poplar',x:-54.9,z:-68.2,seed:71}, {kind:'poplar',x:54.9,z:-68.2,seed:571}, {kind:'poplar',x:-46.1,z:-73.1,seed:72}, {kind:'poplar',x:46.1,z:-73.1,seed:572},
  {kind:'poplar',x:-32,z:-71,seed:73}, {kind:'poplar',x:32,z:-71,seed:573}, {kind:'poplar',x:-21,z:-71,seed:74}, {kind:'poplar',x:21,z:-71,seed:574},
  {kind:'poplar',x:-9,z:-93,seed:80}, {kind:'poplar',x:9,z:-93,seed:580}, {kind:'poplar',x:-9,z:-83,seed:81}, {kind:'poplar',x:9,z:-83,seed:581},
  {kind:'oak',x:-100,z:-25,seed:90}, {kind:'oak',x:100,z:-25,seed:590}, {kind:'oak',x:-125,z:5,seed:91}, {kind:'oak',x:125,z:5,seed:591},
  {kind:'oak',x:-56.4,z:-19.4,seed:92}, {kind:'oak',x:56.4,z:-19.4,seed:592}, {kind:'apple',x:-128,z:22,seed:100}, {kind:'apple',x:128,z:22,seed:600},
  {kind:'apple',x:-128,z:31,seed:101}, {kind:'apple',x:128,z:31,seed:601}, {kind:'apple',x:-119,z:22,seed:102}, {kind:'apple',x:119,z:22,seed:602},
  {kind:'apple',x:-119,z:31,seed:103}, {kind:'apple',x:119,z:31,seed:603}, {kind:'apple',x:-110,z:22,seed:104}, {kind:'apple',x:110,z:22,seed:604},
  {kind:'apple',x:-108.5,z:31,seed:105}, {kind:'apple',x:108.5,z:31,seed:605}
];
DM_Cabana.props = [
  {kind:'vegpatch',x:-167,z:-45,seed:10,rot:1.5708}, {kind:'vegpatch',x:167,z:-45,seed:510,rot:-1.5708}, {kind:'vegpatch',x:-167,z:-22,seed:11,rot:1.5708},
  {kind:'vegpatch',x:167,z:-22,seed:511,rot:-1.5708}, {kind:'vegpatch',x:-167,z:2,seed:12,rot:1.5708}, {kind:'vegpatch',x:167,z:2,seed:512,rot:-1.5708},
  {kind:'vegpatch',x:-167,z:26,seed:13,rot:1.5708}, {kind:'vegpatch',x:167,z:26,seed:513,rot:-1.5708}, {kind:'well',x:-152,z:-32.5,seed:3,rot:0},
  {kind:'well',x:152,z:-32.5,seed:503,rot:0}, {kind:'barrels',x:-163,z:-9,seed:4,rot:0}, {kind:'barrels',x:163,z:-9,seed:504,rot:0},
  {kind:'logpile',x:-163,z:15,seed:5,rot:1.5708}, {kind:'logpile',x:163,z:15,seed:505,rot:-1.5708}, {kind:'cart',x:-151,z:39,seed:6,rot:1.5708},
  {kind:'cart',x:151,z:39,seed:506,rot:-1.5708}, {kind:'haystack',x:-60,z:-101,seed:7,rot:0}, {kind:'haystack',x:60,z:-101,seed:507,rot:0},
  {kind:'haybales',x:-55.4,z:-93.6,seed:8,rot:0.3}, {kind:'haybales',x:55.4,z:-93.6,seed:508,rot:-0.3}, {kind:'trough',x:-40,z:-95,seed:9,rot:0},
  {kind:'trough',x:40,z:-95,seed:509,rot:0}, {kind:'cart',x:-33,z:-103,seed:10,rot:1.5708}, {kind:'cart',x:33,z:-103,seed:510,rot:-1.5708},
  {kind:'haystack',x:-88,z:-38,seed:11,rot:0}, {kind:'haystack',x:88,z:-38,seed:511,rot:0}, {kind:'haystack',x:-120.5,z:-28,seed:12,rot:0},
  {kind:'haystack',x:120.5,z:-28,seed:512,rot:0}, {kind:'logpile',x:-101.5,z:57,seed:13,rot:0}, {kind:'logpile',x:101.5,z:57,seed:513,rot:0},
  {kind:'barrels',x:-112,z:60,seed:14,rot:0}, {kind:'barrels',x:112,z:60,seed:514,rot:0}, {kind:'barrels',x:-171,z:96,seed:15,rot:0},
  {kind:'barrels',x:171,z:96,seed:515,rot:0}, {kind:'logpile',x:-171,z:74,seed:16,rot:1.5708}, {kind:'logpile',x:171,z:74,seed:516,rot:-1.5708}
];
DM_Cabana.covers = [
  {type:'fence',x:-171.5,z:-32.5,len:10,rot:1.5708}, {type:'fence',x:171.5,z:-32.5,len:10,rot:-1.5708}, {type:'fence',x:-171.5,z:15,len:10,rot:1.5708},
  {type:'fence',x:171.5,z:15,len:10,rot:-1.5708}, {type:'hedge',x:-97,z:17,len:12,rot:1.5708}, {type:'hedge',x:97,z:17,len:12,rot:-1.5708},
  {type:'hedge',x:-98,z:2,len:12,rot:1.5708}, {type:'hedge',x:98,z:2,len:12,rot:-1.5708}, {type:'hedge',x:-108,z:-20,len:12,rot:1.5708},
  {type:'hedge',x:108,z:-20,len:12,rot:-1.5708}
];

/* ===== MAP: data-driven definition + builder (DM_Romania) ===== */
const DM_Romania = {
  name:'DM_Romania',
  spawn:{x:0,z:30}, resupply:{x:0,z:34},
  obstacles:[
    {x:0,z:0,w:6,h:4,d:6,warm:true},
    {x:-18,z:-10,w:2,h:1.5,d:2,warm:false},{x:16,z:-16,w:3,h:2.5,d:3,warm:true},{x:-22,z:12,w:4,h:1.5,d:4,warm:false},
    {x:20,z:10,w:2,h:2.5,d:2,warm:true},{x:-8,z:22,w:3,h:1.5,d:3,warm:false},{x:10,z:-26,w:4,h:2.5,d:4,warm:true},
    {x:-30,z:-2,w:2,h:1.5,d:2,warm:false},{x:28,z:-4,w:3,h:2.5,d:3,warm:true},{x:-14,z:-30,w:4,h:1.5,d:4,warm:false},
    {x:22,z:26,w:2,h:2.5,d:2,warm:true},{x:-26,z:-22,w:3,h:1.5,d:3,warm:false},{x:26,z:18,w:4,h:2.5,d:4,warm:true}
  ],
  buildings:[
    {team:'friend',type:'hq',x:0,z:42},{team:'friend',type:'refinery',x:-11,z:42},{team:'friend',type:'barracks',x:11,z:42},{team:'friend',type:'power',x:0,z:50},
    {team:'hostile',type:'hq',x:0,z:-42},{team:'hostile',type:'refinery',x:-11,z:-42},{team:'hostile',type:'barracks',x:11,z:-42},{team:'hostile',type:'power',x:0,z:-50}
  ],
  barracks:[ {x:-18,z:22,team:'friend'}, {x:-18,z:-22,team:'hostile'} ]
};
// C1: dispose the OLD map's GPU resources on rebuild (clearMap ran on init, Reset, map-load, and every editor undo/redo -- previously leaked geo/mat/label-textures each time).
// SHARED material templates are skipped; a `seen` set prevents double-dispose (outline hulls share the parent geometry); only .map (per-instance label CanvasTextures) is disposed, never .gradientMap (shared toon ramps).
function genTimberTex(){
  const S=256, c=document.createElement('canvas'); c.width=c.height=S; const x=c.getContext('2d');
  x.fillStyle='#f0e2b6'; x.fillRect(0,0,S,S);                                   // plaster fill
  for(let i=0;i<150;i++){ const g=200+((Math.random()*30)|0); x.fillStyle='rgba('+g+','+(g-15)+','+(g-55)+',0.22)'; const r=4+Math.random()*9; x.beginPath(); x.arc(Math.random()*S,Math.random()*S,r,0,6.283); x.fill(); }
  x.fillStyle='#342414'; const bw=20;                                          // dark timbers
  x.fillRect(0,0,S,bw); x.fillRect(0,S-bw,S,bw); x.fillRect(0,0,bw,S); x.fillRect(S-bw,0,bw,S);   // frame
  x.fillRect(0,(S-bw)/2,S,bw);                                                 // mid rail
  x.fillRect(S/3-bw/2,0,bw,S); x.fillRect(2*S/3-bw/2,0,bw,S);                  // two studs
  x.lineWidth=bw; x.lineCap='square'; x.strokeStyle='#342414';                // diagonal braces (lower panels)
  x.beginPath(); x.moveTo(bw,S-bw); x.lineTo(S/3-bw/2,S/2); x.stroke();
  x.beginPath(); x.moveTo(S-bw,S-bw); x.lineTo(2*S/3+bw/2,S/2); x.stroke();
  const t=new THREE.CanvasTexture(c); t.magFilter=THREE.NearestFilter; t.minFilter=THREE.NearestFilter; t.wrapS=t.wrapT=THREE.RepeatWrapping; return t;
}
// Fat gable roof: triangular prism, ridge along X. Non-indexed -> flat per-face normals (crisp cel + correct hull).
function gableRoofGeo(len, span, peak, ridgeZ){
  const hw=len/2, hd=span/2, rz=ridgeZ||0;                              // rz != 0 -> saltbox/catslide (ridge shifted toward one eave)
  const P=[[-hw,0,-hd],[-hw,0,hd],[-hw,peak,rz],[hw,0,-hd],[hw,0,hd],[hw,peak,rz]];
  const tris=[[0,1,2],[3,5,4],[0,4,1],[0,3,4],[1,4,5],[1,5,2],[0,2,3],[2,5,3]];
  const UV=[[[0,0],[1,0],[0.5,1]],[[0,0],[0.5,1],[1,0]],[[0,0],[0,0],[0,0]],[[0,0],[0,0],[0,0]],[[0,0],[1,0],[1,1]],[[0,0],[1,1],[0,1]],[[0,0],[0,1],[1,0]],[[0,1],[1,1],[1,0]]];
  const pos=[], uvs=[]; for(let ti=0;ti<tris.length;ti++){ const t=tris[ti]; for(let k=0;k<3;k++){ const vi=t[k]; pos.push(P[vi][0],P[vi][1],P[vi][2]); uvs.push(UV[ti][k][0],UV[ti][k][1]); } }
  const g=new THREE.BufferGeometry(); g.setAttribute('position', new THREE.Float32BufferAttribute(pos,3)); g.setAttribute('uv', new THREE.Float32BufferAttribute(uvs,2)); g.computeVertexNormals(); return g;
}
function genTileTex(){
  const S=256, c=document.createElement('canvas'); c.width=c.height=S; const x=c.getContext('2d');
  const rows=5, rh=S/rows, tw=S/4;
  for(let i=0;i<rows;i++){ const y=i*rh, off=(i%2)*(tw/2);
    x.fillStyle=(i%2===0)?'#d5471f':'#bf3a16'; x.fillRect(0,y,S,rh);                    // course base (subtle alternation)
    for(let j=-1;j<=rows;j++){ const tx=j*tw+off;
      if((i+j)%2===0){ x.fillStyle='rgba(255,145,85,0.16)'; x.fillRect(tx,y,tw,rh); }    // per-tile face variation
      x.fillStyle='#5e1c0a'; x.fillRect(tx,y,4,rh);                                      // vertical tile seam
    }
    x.fillStyle='rgba(255,180,120,0.5)'; x.fillRect(0,y+4,S,3);                          // tile-lip highlight (catches light)
    x.fillStyle='#3f1305'; x.fillRect(0,y+rh-7,S,7);                                          // course shadow line
  }
  const t=new THREE.CanvasTexture(c); t.magFilter=THREE.NearestFilter; t.minFilter=THREE.NearestFilter; t.wrapS=t.wrapT=THREE.RepeatWrapping; return t;
}
function makeTimberHouse(x,z,rot){
  const g=new THREE.Group(); g.position.set(x,0,z); if(rot)g.rotation.y=rot;
  const bw=6, bh=3.4, bd=5, peak=2.6, ov=0.5;                                  // [dial] body + roof dims
  const timber=genTimberTex(); timber.repeat.set(2,1);
  const body=new THREE.Mesh(new THREE.BoxGeometry(bw,bh,bd), new THREE.MeshStandardMaterial({map:timber, roughness:0.92, metalness:0.0}));
  body.position.y=bh/2; body.castShadow=true; body.receiveShadow=true; g.add(body);
  const tile=genTileTex(); tile.repeat.set(5.6,4.2);                              // [dial] tile density
  const roof=new THREE.Mesh(gableRoofGeo(bw+ov, bd+ov*2, peak), new THREE.MeshStandardMaterial({map:tile, roughness:0.85, metalness:0.0}));
  roof.position.y=bh; roof.castShadow=true; g.add(roof);
  const chimney=new THREE.Mesh(new THREE.BoxGeometry(0.72,1.7,0.72), new THREE.MeshStandardMaterial({color:0x8a2e18, roughness:0.95}));  // brick, straddles ridge
  chimney.position.set(bw*0.28, bh+peak+0.15, 0); chimney.castShadow=true; g.add(chimney);
  const frameMat=new THREE.MeshStandardMaterial({color:0x5a3a1c, roughness:0.85});   // wood trim
  const paneMat=new THREE.MeshStandardMaterial({color:0x2a3a48, roughness:0.3, metalness:0.1});
  const doorFrame=new THREE.Mesh(new THREE.BoxGeometry(1.18,2.05,0.1), frameMat); doorFrame.position.set(0,1.0,bd/2+0.03); doorFrame.castShadow=true; g.add(doorFrame);
  const door=new THREE.Mesh(new THREE.BoxGeometry(0.92,1.82,0.1), new THREE.MeshStandardMaterial({color:0x8a5a28, roughness:0.8})); door.position.set(0,0.93,bd/2+0.055); door.castShadow=true; g.add(door);
  const mkWin=(px,py,pz,ry)=>{ const wg=new THREE.Group(); wg.position.set(px,py,pz); if(ry)wg.rotation.y=ry;
    const frame=new THREE.Mesh(new THREE.BoxGeometry(1.05,1.2,0.1), frameMat); frame.position.z=0.03; frame.castShadow=true; wg.add(frame);
    const pane=new THREE.Mesh(new THREE.BoxGeometry(0.76,0.9,0.1), paneMat); pane.position.z=0.045; pane.castShadow=true; wg.add(pane);
    g.add(wg); };
  mkWin(-1.8,2.0,bd/2,0); mkWin(1.8,2.0,bd/2,0); mkWin(bw/2,1.9,0,Math.PI/2); mkWin(-bw/2,1.9,0,-Math.PI/2);
  g.updateMatrixWorld(true);
  const o={grp:g, body, box:new THREE.Box3().setFromObject(body), x, z, rot:rot||0, type:'timberHouse'};
  g.userData.editSel={kind:'house',ref:o};
  if(typeof stylizeOne==='function') stylizeOne(g);   // toonify + BOLD world outline (runtime object)
  return o;
}
function genPlasterTex(){
  const S=256, c=document.createElement('canvas'); c.width=c.height=S; const x=c.getContext('2d');
  x.fillStyle='#e6dcc2'; x.fillRect(0,0,S,S);                                   // cream whitewash
  for(let i=0;i<200;i++){ const g=222+((Math.random()*26)|0); x.fillStyle='rgba('+g+','+(g-12)+','+(g-42)+',0.2)'; const r=5+Math.random()*13; x.beginPath(); x.arc(Math.random()*S,Math.random()*S,r,0,6.283); x.fill(); }
  for(let k=0;k<28;k++){ x.strokeStyle='rgba(150,135,105,0.16)'; x.lineWidth=1; const sx=Math.random()*S,sy=Math.random()*S; x.beginPath(); x.moveTo(sx,sy); x.lineTo(sx+(Math.random()*36-18),sy+(Math.random()*36-18)); x.stroke(); }
  const t=new THREE.CanvasTexture(c); t.magFilter=THREE.NearestFilter; t.minFilter=THREE.NearestFilter; t.wrapS=t.wrapT=THREE.RepeatWrapping; return t;
}
function genThatchTex(){
  const S=256, c=document.createElement('canvas'); c.width=c.height=S; const x=c.getContext('2d');
  const grd=x.createLinearGradient(0,0,0,S); grd.addColorStop(0,'#dcb043'); grd.addColorStop(0.5,'#c99a34'); grd.addColorStop(1,'#a97e2a'); x.fillStyle=grd; x.fillRect(0,0,S,S);
  const courses=5, ch=S/courses;                                               // combed horizontal tiers
  for(let i=0;i<courses;i++){ const y=i*ch;
    if(i%2===1){ x.fillStyle='rgba(120,88,34,0.16)'; x.fillRect(0,y,S,ch); }
    x.fillStyle='rgba(84,60,22,0.4)'; x.fillRect(0,y+ch-3,S,3);                 // comb shadow line
    x.fillStyle='rgba(250,224,150,0.3)'; x.fillRect(0,y+ch-6,S,2);              // highlight above the comb
  }
  for(let k=0;k<820;k++){ const sx=Math.random()*S, sy=Math.random()*S, len=12+Math.random()*26, lean=Math.random()*3-1.5, t=Math.random();  // dense straw down-slope
    x.strokeStyle = t<0.34?'rgba(120,86,30,0.34)':(t<0.67?'rgba(198,158,74,0.36)':'rgba(248,222,150,0.4)'); x.lineWidth=1;
    x.beginPath(); x.moveTo(sx,sy); x.lineTo(sx+lean,sy+len); x.stroke(); }
  x.fillStyle='rgba(84,60,22,0.45)'; x.fillRect(0,S-16,S,16);                   // darker trimmed eave
  const t=new THREE.CanvasTexture(c); t.magFilter=THREE.NearestFilter; t.minFilter=THREE.NearestFilter; t.wrapS=t.wrapT=THREE.RepeatWrapping; return t;
}
function makeThatchCottage(x,z,rot){
  const g=new THREE.Group(); g.position.set(x,0,z); if(rot)g.rotation.y=rot;
  const bw=5.6, bh=2.6, bd=4.6, peak=3.2, ov=0.95;                             // [dial] short walls, big deep thatch
  const plaster=genPlasterTex(); plaster.repeat.set(2,1);
  const body=new THREE.Mesh(new THREE.BoxGeometry(bw,bh,bd), new THREE.MeshStandardMaterial({map:plaster, roughness:0.98}));
  body.position.y=bh/2; body.castShadow=true; body.receiveShadow=true; g.add(body);
  const postMat=new THREE.MeshStandardMaterial({color:0x4a3418, roughness:0.9});   // rustic corner posts
  const post=(px,pz)=>{ const m=new THREE.Mesh(new THREE.BoxGeometry(0.22,bh,0.22),postMat); m.position.set(px,bh/2,pz); m.castShadow=true; g.add(m); };
  post(bw/2-0.11,bd/2-0.11); post(-(bw/2-0.11),bd/2-0.11); post(bw/2-0.11,-(bd/2-0.11)); post(-(bw/2-0.11),-(bd/2-0.11));
  const thatch=genThatchTex(); thatch.repeat.set(2,1.4);
  const roof=new THREE.Mesh(gableRoofGeo(bw+ov, bd+ov*2, peak), new THREE.MeshStandardMaterial({map:thatch, roughness:1.0}));
  roof.position.y=bh; roof.castShadow=true; g.add(roof);
  const rollMat=new THREE.MeshStandardMaterial({color:0xc79a38, roughness:1.0});   // chunky straw volume
  const span2=(bd+ov*2)/2, eaveGeo=new THREE.CylinderGeometry(0.34,0.34,bw+ov+0.15,8);
  [span2-0.1,-(span2-0.1)].forEach(zz=>{ const rl=new THREE.Mesh(eaveGeo, rollMat); rl.rotation.z=Math.PI/2; rl.position.set(0,bh+0.1,zz); rl.castShadow=true; g.add(rl); });
  const ridge=new THREE.Mesh(new THREE.CylinderGeometry(0.24,0.24,bw+ov+0.25,8), rollMat); ridge.rotation.z=Math.PI/2; ridge.position.set(0,bh+peak,0); ridge.castShadow=true; g.add(ridge);
  const chimney=new THREE.Mesh(new THREE.BoxGeometry(0.6,1.5,0.6), new THREE.MeshStandardMaterial({color:0x9a9088, roughness:0.95}));  // stone
  chimney.position.set(-bw*0.3, bh+peak, 0); chimney.castShadow=true; g.add(chimney);
  const frameMat=new THREE.MeshStandardMaterial({color:0x4a3418, roughness:0.9});
  const paneMat=new THREE.MeshStandardMaterial({color:0x2a3a48, roughness:0.3, metalness:0.1});
  const doorFrame=new THREE.Mesh(new THREE.BoxGeometry(1.05,1.85,0.1), frameMat); doorFrame.position.set(0,0.925,bd/2+0.03); doorFrame.castShadow=true; g.add(doorFrame);
  const door=new THREE.Mesh(new THREE.BoxGeometry(0.82,1.62,0.1), new THREE.MeshStandardMaterial({color:0x6e4a24, roughness:0.85})); door.position.set(0,0.86,bd/2+0.055); door.castShadow=true; g.add(door);
  const mkWin=(px,py,pz,ry)=>{ const wg=new THREE.Group(); wg.position.set(px,py,pz); if(ry)wg.rotation.y=ry;
    const frame=new THREE.Mesh(new THREE.BoxGeometry(0.85,0.95,0.1), frameMat); frame.position.z=0.03; frame.castShadow=true; wg.add(frame);
    const pane=new THREE.Mesh(new THREE.BoxGeometry(0.58,0.68,0.1), paneMat); pane.position.z=0.045; pane.castShadow=true; wg.add(pane);
    g.add(wg); };
  mkWin(-1.65,1.55,bd/2,0); mkWin(1.65,1.55,bd/2,0); mkWin(bw/2,1.5,0,Math.PI/2); mkWin(-bw/2,1.5,0,-Math.PI/2);
  g.updateMatrixWorld(true);
  const o={grp:g, body, box:new THREE.Box3().setFromObject(body), x, z, rot:rot||0, type:'thatchCottage'};
  g.userData.editSel={kind:'house',ref:o};
  if(typeof stylizeOne==='function') stylizeOne(g);
  return o;
}
function genBrickFlintTex(){
  const S=256, c=document.createElement('canvas'); c.width=c.height=S; const x=c.getContext('2d');
  const bands=7, bh=S/bands, bwid=S/7;
  for(let i=0;i<bands;i++){ const y=i*bh;
    if(i%2===0){                                                               // brick course band
      x.fillStyle='#a8442e'; x.fillRect(0,y,S,bh);
      const stag=((i/2)%2)*(bwid/2);
      for(let j=-1;j<=7;j++){ if(((i+j)%2)===0){ x.fillStyle='rgba(152,54,34,0.32)'; x.fillRect(j*bwid+stag+2,y,bwid-3,bh-2); } }  // per-brick tone
      x.fillStyle='#7c2e1d'; x.fillRect(0,y+bh-2,S,2);                          // horizontal mortar
      for(let j=-1;j<=7;j++){ x.fillRect(j*bwid+stag,y,2,bh); }                 // vertical mortar (staggered)
    } else {                                                                   // flint (knapped pale stone) band
      x.fillStyle='#cfc7b4'; x.fillRect(0,y,S,bh);
      for(let k=0;k<50;k++){ const fx=Math.random()*S, fy=y+Math.random()*bh, r=1.6+Math.random()*2.4, t=Math.random();
        x.fillStyle=t<0.4?'#9c9482':(t<0.75?'#b8b09c':'#e8e2d2'); x.beginPath(); x.arc(fx,fy,r,0,6.283); x.fill(); }
      x.fillStyle='#8a8270'; x.fillRect(0,y+bh-2,S,2);                          // seam
    }
  }
  const t=new THREE.CanvasTexture(c); t.magFilter=THREE.NearestFilter; t.minFilter=THREE.NearestFilter; t.wrapS=t.wrapT=THREE.RepeatWrapping; return t;
}
function makeBrickFlintFarmhouse(x,z,rot){
  const g=new THREE.Group(); g.position.set(x,0,z); if(rot)g.rotation.y=rot;
  const bw=7.5, bh=3.3, bd=5.4, peak=2.4, ov=0.55;                             // [dial] longer farmhouse
  const bf=genBrickFlintTex(); bf.repeat.set(2.4,1.25);
  const body=new THREE.Mesh(new THREE.BoxGeometry(bw,bh,bd), new THREE.MeshStandardMaterial({map:bf, roughness:0.95}));
  body.position.y=bh/2; body.castShadow=true; body.receiveShadow=true; g.add(body);
  const tile=genTileTex(); tile.repeat.set(7,4.4);
  const roof=new THREE.Mesh(gableRoofGeo(bw+ov, bd+ov*2, peak), new THREE.MeshStandardMaterial({map:tile, roughness:0.85}));
  roof.position.y=bh; roof.castShadow=true; g.add(roof);
  const chimney=new THREE.Mesh(new THREE.BoxGeometry(0.8,1.8,0.8), new THREE.MeshStandardMaterial({color:0x8a2e18, roughness:0.95}));
  chimney.position.set(bw*0.32, bh+peak+0.1, 0); chimney.castShadow=true; g.add(chimney);
const frameMat=new THREE.MeshStandardMaterial({color:0x6a5636, roughness:0.85});   // stone lintels/quoins
  const paneMat=new THREE.MeshStandardMaterial({color:0x2a3a48, roughness:0.3, metalness:0.1});
  const doorFrame=new THREE.Mesh(new THREE.BoxGeometry(1.2,2.1,0.1), frameMat); doorFrame.position.set(0,1.05,bd/2+0.03); doorFrame.castShadow=true; g.add(doorFrame);
  const door=new THREE.Mesh(new THREE.BoxGeometry(0.95,1.88,0.1), new THREE.MeshStandardMaterial({color:0x5c3d20, roughness:0.85})); door.position.set(0,0.94,bd/2+0.055); door.castShadow=true; g.add(door);
  const mkWin=(px,py,pz,ry)=>{ const wg=new THREE.Group(); wg.position.set(px,py,pz); if(ry)wg.rotation.y=ry;
    const frame=new THREE.Mesh(new THREE.BoxGeometry(1.0,1.2,0.1), frameMat); frame.position.z=0.03; frame.castShadow=true; wg.add(frame);
    const pane=new THREE.Mesh(new THREE.BoxGeometry(0.72,0.9,0.1), paneMat); pane.position.z=0.045; pane.castShadow=true; wg.add(pane);
    g.add(wg); };
  mkWin(-2.4,2.0,bd/2,0); mkWin(2.4,2.0,bd/2,0); mkWin(bw/2,1.95,0,Math.PI/2); mkWin(-bw/2,1.95,0,-Math.PI/2);
  g.updateMatrixWorld(true);
  const o={grp:g, body, box:new THREE.Box3().setFromObject(body), x, z, rot:rot||0, type:'brickFarm'};
  g.userData.editSel={kind:'house',ref:o};
  if(typeof stylizeOne==='function') stylizeOne(g);
  return o;
}
// ---- Norman manor: two-storey stone block + round corner tower with a conical witch-hat roof (landmark house type) ----
function makeManorHouse(x,z,rot){
  const g=new THREE.Group(); g.position.set(x,0,z); if(rot)g.rotation.y=rot;
  const wallMat=new THREE.MeshStandardMaterial({ map:BK.tex('plaster'), roughness:0.95 });
  const stoneMat=new THREE.MeshStandardMaterial({ map:BK.tex('drystone'), roughness:1.0 });
  const roofMat=new THREE.MeshStandardMaterial({ map:BK.tex('slate'), roughness:0.9 });
  const coneMat=new THREE.MeshStandardMaterial({ color:0x455161, roughness:0.85 });
  const trimMat=new THREE.MeshStandardMaterial({ color:0xd8ccae, roughness:1.0 });
  const doorMat=new THREE.MeshStandardMaterial({ color:0x5e3d1e, roughness:0.85 });
  const paneMat=new THREE.MeshStandardMaterial({ color:0x8fb3c7, roughness:0.3, metalness:0.1 });
  const bw=9.5, bh=5.6, bd=6.5, peak=3.2, ov=0.6;
  const body=new THREE.Mesh(new THREE.BoxGeometry(bw,bh,bd), wallMat); body.position.y=bh/2; body.castShadow=true; body.receiveShadow=true; g.add(body);
  const mr=_roofFor('hip', bw, bd, peak, ov); const roof=new THREE.Mesh(mr.geo, roofMat); roof.position.y=bh; roof.rotation.y=mr.rotY; roof.castShadow=true; g.add(roof);
  const qg=new THREE.BoxGeometry(0.5,bh,0.5);
  for(const qx of [-bw/2+0.25,bw/2-0.25]) for(const qz of [-bd/2+0.25,bd/2-0.25]){ const q=new THREE.Mesh(qg,trimMat); q.position.set(qx,bh/2,qz); g.add(q); }
  const band=new THREE.Mesh(new THREE.BoxGeometry(bw+0.12,0.24,bd+0.12), trimMat); band.position.y=bh*0.5; g.add(band);   // string course between floors
  const rT=1.95, hT=7.6, tcx=bw/2+rT-0.9, tcz=bd*0.1;
  const tower=new THREE.Mesh(new THREE.CylinderGeometry(rT,rT,hT,16), stoneMat); tower.position.set(tcx,hT/2,tcz); tower.castShadow=true; tower.receiveShadow=true; g.add(tower);
  const corbel=new THREE.Mesh(new THREE.CylinderGeometry(rT+0.3,rT+0.12,0.35,16), trimMat); corbel.position.set(tcx,hT-0.08,tcz); g.add(corbel);
  const cone=new THREE.Mesh(new THREE.ConeGeometry(rT+0.5,4.2,18), coneMat); cone.position.set(tcx,hT+2.1,tcz); cone.castShadow=true; g.add(cone);
  const finial=new THREE.Mesh(new THREE.CylinderGeometry(0.06,0.06,0.8,6), trimMat); finial.position.set(tcx,hT+4.4,tcz); g.add(finial);
  for(const ty of [2.5,4.7]){ const tw=new THREE.Mesh(new THREE.BoxGeometry(0.5,1.1,0.16), paneMat); tw.position.set(tcx+rT-0.06,ty,tcz); g.add(tw); }
  const doorH=2.7,doorW=1.6, dx=-bw*0.16, fz=bd/2;
  const dfr=new THREE.Mesh(new THREE.BoxGeometry(doorW+0.34,doorH+0.3,0.16), trimMat); dfr.position.set(dx,(doorH+0.3)/2,fz+0.02); g.add(dfr);
  const door=new THREE.Mesh(new THREE.BoxGeometry(doorW,doorH,0.12), doorMat); door.position.set(dx,doorH/2,fz+0.06); door.castShadow=true; g.add(door);
  const winAt=(px,py,pz,ry)=>{ const fr=new THREE.Mesh(new THREE.BoxGeometry(1.0,1.35,0.12), trimMat); fr.position.set(px,py,pz); fr.rotation.y=ry; g.add(fr); const pa=new THREE.Mesh(new THREE.BoxGeometry(0.66,1.0,0.12), paneMat); pa.position.set(px+Math.sin(ry)*0.03,py,pz+Math.cos(ry)*0.03); pa.rotation.y=ry; g.add(pa); };
  for(const wx of [-bw*0.32, bw*0.06, bw*0.30]) winAt(wx,4.1,fz+0.03,0);            // upper row
  for(const wx of [bw*0.10, bw*0.32]) winAt(wx,1.9,fz+0.03,0);                       // ground row
  for(const sz of [-bd*0.26, bd*0.26]) winAt(-bw/2-0.03,4.1,sz,-Math.PI/2);          // left side upper
  const chimGeo=new THREE.BoxGeometry(0.8,1.9,0.8);
  for(const cxx of [-bw*0.3, bw*0.26]){ const ch=new THREE.Mesh(chimGeo, stoneMat); ch.position.set(cxx,bh+peak-0.5,0); ch.castShadow=true; g.add(ch); const cap=new THREE.Mesh(new THREE.BoxGeometry(1.0,0.2,1.0), trimMat); cap.position.set(cxx,bh+peak+0.55,0); g.add(cap); }
  const o={ grp:g, body, localColliders:[
      { minx:-bw/2, miny:0, minz:-bd/2, maxx:bw/2, maxy:bh, maxz:bd/2 },
      { minx:tcx-rT, miny:0, minz:tcz-rT, maxx:tcx+rT, maxy:hT, maxz:tcz+rT }
    ], boxes:[], x, z, rot:rot||0, type:'manor' };
  g.updateMatrixWorld(true);
  _bkWorldBoxes(o); g.userData.editSel={kind:'house',ref:o};
  if(typeof stylizeOne==='function') stylizeOne(g);
  return o;
}

// ============================================================================
// BUILDING KIT . CORE   (self-contained; -> future shared-core)
//   Seeded PRNG + spec schema + parts registry + region presets + roll/build.
//   PURE w.r.t. game state: BK.build(spec) touches only THREE + kit assets and
//   returns {group, colliders(LOCAL AABBs), body}. The CALLER adds to scene,
//   registers colliders, and stylizes. Editor pickers are generated from
//   BK.PARTS/BK.DIMS (add a part here -> a control appears, no UI wiring).
//   Placed things persist the RESOLVED spec, so they never reshuffle if the kit
//   grows. KIT SEAM: gen{Timber,Tile,Thatch,Plaster}Tex are reused from the
//   legacy house block above; relocate them into this section at split time.
// ============================================================================
function _mulberry32(a){ return function(){ a|=0; a=(a+0x6D2B79F5)|0; let t=Math.imul(a^(a>>>15),1|a); t=(t+Math.imul(t^(t>>>7),61|t))^t; return ((t^(t>>>14))>>>0)/4294967296; }; }

// Hip roof: 4 slopes to a short ridge (ridge along X, insets by eave depth). Non-indexed -> flat cel normals.
function _hipRoofGeo(len, span, peak){
  const hw=len/2, hd=span/2, rl=Math.max(0.15, hw-hd);
  const P=[[-hw,0,-hd],[hw,0,-hd],[hw,0,hd],[-hw,0,hd],[-rl,peak,0],[rl,peak,0]];
  const tris=[[0,5,1],[0,4,5],[3,2,5],[3,5,4],[0,3,4],[1,5,2]];   // front, back (2 each) + two hip ends
  const pos=[], uvs=[];
  for(const t of tris){ for(const vi of t){ const p=P[vi]; pos.push(p[0],p[1],p[2]); uvs.push((p[0]+hw)/(2*hw),(p[2]+hd)/(2*hd)); } }
  const g=new THREE.BufferGeometry(); g.setAttribute('position',new THREE.Float32BufferAttribute(pos,3)); g.setAttribute('uv',new THREE.Float32BufferAttribute(uvs,2)); g.computeVertexNormals(); return g;
}
// Half-hip / jerkinhead: gable ends clipped by a small hip. Ridge along local X (so _roofFor can orient it). Non-indexed -> flat cel faces.
function _halfHipRoofGeo(len, span, peak){
  const hw=len/2, hd=span/2;
  const clip=Math.min(hd*0.9, hw*0.55), rl=Math.max(0.2, hw-clip);      // shortened ridge
  const sy=peak*0.55, zsh=hd*(1-sy/peak);                               // shoulder height + slope half-width there (Ep/Gp lie on the slope plane -> watertight)
  const A=[-hw,0,-hd], B=[-hw,0,hd], C=[hw,0,hd], D=[hw,0,-hd];
  const R0=[-rl,peak,0], R1=[rl,peak,0];
  const Gp=[-hw,sy,zsh], Gm=[-hw,sy,-zsh], Ep=[hw,sy,zsh], Em=[hw,sy,-zsh];
  const tris=[
    B,C,Ep,  B,Ep,R1,  B,R1,R0,  B,R0,Gp,        // +Z slope
    A,Gm,R0,  A,R0,R1,  A,R1,Em,  A,Em,D,          // -Z slope
    Ep,Em,R1,   Gm,Gp,R0,                          // +X hip, -X hip
    C,Em,Ep,  C,D,Em,                              // +X gablet (vertical)
    B,Gp,Gm,  B,Gm,A,                              // -X gablet (vertical)
    A,C,B,  A,D,C                                   // underside
  ];
  const pos=[], uvs=[];
  for(const p of tris){ pos.push(p[0],p[1],p[2]); uvs.push((p[0]+hw)/(2*hw),(p[2]+hd)/(2*hd)); }
  const g=new THREE.BufferGeometry(); g.setAttribute('position',new THREE.Float32BufferAttribute(pos,3)); g.setAttribute('uv',new THREE.Float32BufferAttribute(uvs,2)); g.computeVertexNormals(); return g;
}
// Gambrel / barn: each side has a steep lower slope + shallow upper slope meeting at a knuckle. Ridge along local X.
function _gambrelRoofGeo(len, span, peak){
  const hw=len/2, hd=span/2, zk=hd*0.55, yk=peak*0.6;                    // knuckle position -> lower steep, upper shallow
  const A=[-hw,0,-hd],B=[-hw,0,hd],C=[hw,0,hd],D=[hw,0,-hd];
  const Kp0=[-hw,yk,zk],Kp1=[hw,yk,zk],Km0=[-hw,yk,-zk],Km1=[hw,yk,-zk];
  const R0=[-hw,peak,0],R1=[hw,peak,0];
  const tris=[
    B,C,Kp1, B,Kp1,Kp0,            // +Z lower slope
    Kp0,Kp1,R1, Kp0,R1,R0,         // +Z upper slope
    A,Km0,Km1, A,Km1,D,            // -Z lower slope
    Km0,R0,R1, Km0,R1,Km1,         // -Z upper slope
    A,B,Kp0, A,Kp0,R0, A,R0,Km0,   // -X gable end (pentagon)
    D,Km1,R1, D,R1,Kp1, D,Kp1,C,   // +X gable end (pentagon)
    A,C,B, A,D,C                    // underside
  ];
  const pos=[], uvs=[];
  for(const p of tris){ pos.push(p[0],p[1],p[2]); uvs.push((p[0]+hw)/(2*hw),(p[2]+hd)/(2*hd)); }
  const g=new THREE.BufferGeometry(); g.setAttribute('position',new THREE.Float32BufferAttribute(pos,3)); g.setAttribute('uv',new THREE.Float32BufferAttribute(uvs,2)); g.computeVertexNormals(); return g;
}
// Orient a roof so its ridge runs along the LONGER footprint axis (fixes deep-building mis-orientation + hip pyramid-degeneration).
// gableRoofGeo/_hipRoofGeo build the ridge along their first arg (local X); we feed them long-axis first and rotate 90 deg when depth wins.
function _roofFor(roofType, w, d, peak, ov){
  const longX = (w >= d);
  const L = (longX ? w : d) + ov;        // ridge-direction extent (+ verge overhang)
  const S = (longX ? d : w) + ov*2;      // eaves-direction extent (+ eaves overhang each side)
  const geo = (roofType==='hip') ? _hipRoofGeo(L, S, peak)
            : (roofType==='halfhip') ? _halfHipRoofGeo(L, S, peak)
            : (roofType==='saltbox') ? gableRoofGeo(L, S, peak, S*0.22)
            : (roofType==='gambrel') ? _gambrelRoofGeo(L, S, peak)
            : gableRoofGeo(L, S, peak);
  return { geo, rotY: longX ? 0 : Math.PI/2 };
}
// New neutral skins the legacy block didn't have. Low/near-equal G&B on the brick -> warm terracotta, not salmon.
function _bkBrickTex(){ const S=256,c=document.createElement('canvas'); c.width=c.height=S; const x=c.getContext('2d');
  x.fillStyle='#6f3020'; x.fillRect(0,0,S,S);                                    // recessed mortar
  const rows=8, rh=S/rows, bw=S/4;
  for(let i=0;i<rows;i++){ const y=i*rh, off=(i%2)*(bw/2);
    for(let j=-1;j<=4;j++){ const v=(Math.random()*22)|0; x.fillStyle='rgb('+(150-v)+','+(66-(v>>1))+','+(48-(v>>1))+')'; x.fillRect(j*bw+off+2,y+2,bw-4,rh-4); } }  // deeper brick faces
  const t=new THREE.CanvasTexture(c); t.magFilter=THREE.NearestFilter; t.minFilter=THREE.NearestFilter; t.wrapS=t.wrapT=THREE.RepeatWrapping; return t; }
function _bkSlateTex(){ const S=256,c=document.createElement('canvas'); c.width=c.height=S; const x=c.getContext('2d');
  const rows=6, rh=S/rows, tw=S/5;
  for(let i=0;i<rows;i++){ const y=i*rh, off=(i%2)*(tw/2);
    x.fillStyle=(i%2===0)?'#586170':'#4e5765'; x.fillRect(0,y,S,rh);              // cool blue-grey course
    for(let j=-1;j<=5;j++){ const tx=j*tw+off, v=(Math.random()*16)|0; x.fillStyle='rgba('+(88-v)+','+(96-v)+','+(112-v)+',0.5)'; x.fillRect(tx+1,y,tw-2,rh-2); x.fillStyle='#33383f'; x.fillRect(tx,y,2,rh); }
    x.fillStyle='rgba(150,160,178,0.35)'; x.fillRect(0,y+3,S,2); x.fillStyle='#2b2f35'; x.fillRect(0,y+rh-5,S,5); }
  const t=new THREE.CanvasTexture(c); t.magFilter=THREE.NearestFilter; t.minFilter=THREE.NearestFilter; t.wrapS=t.wrapT=THREE.RepeatWrapping; return t; }

function _bkStoneTex(){ const S=256,c=document.createElement('canvas'); c.width=c.height=S; const x=c.getContext('2d');
  x.fillStyle='#8f897c'; x.fillRect(0,0,S,S);                                    // mortar
  const rows=6, rh=S/rows;
  for(let i=0;i<rows;i++){ const y=i*rh, n=4+((Math.random()*2)|0), bw=S/n, off=(i%2)*(bw/2);
    for(let j=-1;j<=n;j++){ const v=(Math.random()*26)|0; x.fillStyle='rgb('+(168-v)+','+(162-v)+','+(150-v)+')'; x.fillRect(j*bw+off+2,y+2,bw-4,rh-4); } }   // irregular pale stones
  x.strokeStyle='rgba(60,56,48,0.4)'; x.lineWidth=2; for(let i=1;i<rows;i++){ x.beginPath(); x.moveTo(0,i*rh); x.lineTo(S,i*rh); x.stroke(); }
  const t=new THREE.CanvasTexture(c); t.magFilter=THREE.NearestFilter; t.minFilter=THREE.NearestFilter; t.wrapS=t.wrapT=THREE.RepeatWrapping; return t; }
function _bkHedgeTex(){ const S=256,c=document.createElement('canvas'); c.width=c.height=S; const x=c.getContext('2d');
  x.fillStyle='#3f5a26'; x.fillRect(0,0,S,S);                                    // deep leaf base
  for(let i=0;i<560;i++){ const tt=Math.random(), r=2+Math.random()*4; x.fillStyle=tt<0.38?'#2f4a1e':(tt<0.72?'#4a6b2f':'#6f9440'); x.beginPath(); x.arc(Math.random()*S,Math.random()*S,r,0,6.283); x.fill(); }  // shadow / mid / highlight leaves
  const t=new THREE.CanvasTexture(c); t.magFilter=THREE.NearestFilter; t.minFilter=THREE.NearestFilter; t.wrapS=t.wrapT=THREE.RepeatWrapping; return t; }

function _bkDirtTex(){ const S=256,c=document.createElement('canvas'); c.width=c.height=S; const x=c.getContext('2d');
  x.fillStyle='#5a4632'; x.fillRect(0,0,S,S);                                    // packed soil base
  for(let i=0;i<620;i++){ const tt=Math.random(), r=1+Math.random()*3; x.fillStyle=tt<0.42?'#493626':(tt<0.76?'#6b5640':'#7d6a4e'); x.beginPath(); x.arc(Math.random()*S,Math.random()*S,r,0,6.283); x.fill(); }  // clods / pebbles / dry patches
  const t=new THREE.CanvasTexture(c); t.magFilter=THREE.NearestFilter; t.minFilter=THREE.NearestFilter; t.wrapS=t.wrapT=THREE.RepeatWrapping; return t; }

const BK={
  _tex:{},
  // ---- swappable modules; the editor builds one picker per key from opts ----
  PARTS:{
    footprint:{ label:'Footprint', opts:[['rect','Rectangle'],['L','L-shape'],['T','T-shape'],['U','U-shape'],['longbar','Long range']] },
    roof:{ label:'Roof', opts:[['gable','Gable'],['hip','Hip'],['halfhip','Half-hip'],['saltbox','Saltbox'],['gambrel','Gambrel']] },
    roofSkin:{ label:'Roof skin', opts:[['tile','Terracotta'],['slate','Slate'],['thatch','Thatch']] },
    wallSkin:{ label:'Wall skin', opts:[['timber','Half-timber'],['plaster','Plaster'],['brick','Brick']] },
    dormers:{ label:'Dormers', opts:[['0','None'],['1','One'],['2','Two']] }
  },
  // ---- one slider per entry: [key,label,min,max,step] ----
  DIMS:[ ['w','Width',5,14,0.1], ['d','Depth',5,11,0.1], ['wallH','Height',3.2,7,0.1], ['pitch','Roof pitch',1.2,4,0.1] ],
  // ---- a region = weighted dice tables + roll ranges over the kit ----
  PRESETS:{
    normandy:{ label:'Normandy',
      weights:{ footprint:{rect:40,L:20,T:14,U:10,longbar:16}, roof:{gable:34,hip:22,halfhip:20,saltbox:12,gambrel:12}, roofSkin:{tile:50,slate:30,thatch:20}, wallSkin:{timber:40,plaster:35,brick:25}, dormers:{'0':45,'1':35,'2':20} },
      dims:{ w:[7,12], d:[5.5,9], wallH:[3.4,5.4], pitch:[1.8,3.2] } }
  },
  // ---- named starting points (the old three houses, now just presets) ----
  NAMED:{
    timberHouse:{ footprint:'rect', w:6, d:5, wallH:3.4, pitch:2.6, roof:'gable', roofSkin:'tile', wallSkin:'timber' },
    thatchCottage:{ footprint:'rect', w:5.6, d:4.6, wallH:2.6, pitch:3.2, roof:'gable', roofSkin:'thatch', wallSkin:'plaster' },
    brickFarm:{ footprint:'rect', w:7.5, d:5.4, wallH:3.3, pitch:2.4, roof:'gable', roofSkin:'tile', wallSkin:'brick' }
  }
};
BK.tex=function(id){ if(this._tex[id])return this._tex[id];
  let t = id==='timber'?genTimberTex() : id==='plaster'?genPlasterTex() : id==='brick'?_bkBrickTex()
        : id==='tile'?genTileTex() : id==='thatch'?genThatchTex() : id==='slate'?_bkSlateTex()
        : id==='drystone'?_bkStoneTex() : id==='hedge'?_bkHedgeTex() : id==='dirt'?_bkDirtTex() : genPlasterTex();
  const rep={ timber:[2,1], plaster:[2,1], brick:[3,1.4], tile:[5.6,4.2], thatch:[2,1.4], slate:[6,4.6], drystone:[3,1], hedge:[4,1], dirt:[4,1] }[id]||[2,1];
  t.repeat.set(rep[0],rep[1]); this._tex[id]=t; return t;   // one shared, session-lifetime texture per skin (leak-free)
};
BK.roll=function(presetId, seed){
  const P=this.PRESETS[presetId]||this.PRESETS.normandy, rnd=_mulberry32((seed>>>0)||1);
  const pick=(tbl)=>{ let tot=0,k; for(k in tbl)tot+=tbl[k]; let r=rnd()*tot; for(k in tbl){ r-=tbl[k]; if(r<=0)return k; } for(k in tbl)return k; };
  const lp=(a,b)=>+(a+(b-a)*rnd()).toFixed(2), D=P.dims;
  return { preset:presetId, seed:(seed>>>0)||1,
    footprint:pick(P.weights.footprint), roof:pick(P.weights.roof), roofSkin:pick(P.weights.roofSkin), wallSkin:pick(P.weights.wallSkin), dormers:pick(P.weights.dormers||{'0':100}),
    w:lp(D.w[0],D.w[1]), d:lp(D.d[0],D.d[1]), wallH:lp(D.wallH[0],D.wallH[1]), pitch:lp(D.pitch[0],D.pitch[1]) };
};
// PURE builder: no scene/houses/SIM. Returns group + LOCAL-space collider AABBs + the main wall mesh.
BK.build=function(spec){
  const s=Object.assign({ footprint:'rect', w:8.5, d:6.5, wallH:3.8, pitch:2.4, roof:'gable', roofSkin:'tile', wallSkin:'timber', dormers:'0' }, spec||{});
  const self=this, g=new THREE.Group(), colliders=[]; const ov=0.5;
  const wallMat=()=>new THREE.MeshStandardMaterial({ map:self.tex(s.wallSkin), roughness:0.95, metalness:0.0 });
  const roofMat=()=>new THREE.MeshStandardMaterial({ map:self.tex(s.roofSkin), roughness:0.9, metalness:0.0 });
  const block=(cx,cz,w,d,h,peak,rtype)=>{ const wall=new THREE.Mesh(new THREE.BoxGeometry(w,h,d), wallMat()); wall.position.set(cx,h/2,cz); wall.castShadow=true; wall.receiveShadow=true; g.add(wall);
    const rf=_roofFor(rtype||s.roof, w, d, peak, ov); const r=new THREE.Mesh(rf.geo, roofMat()); r.position.set(cx,h,cz); if(rf.rotY)r.rotation.y=rf.rotY; r.castShadow=true; g.add(r);
    colliders.push({ minx:cx-w/2, miny:0, minz:cz-d/2, maxx:cx+w/2, maxy:h, maxz:cz+d/2 }); return wall; };
  let frontZ=s.d/2, frontWinX=s.w*0.3, chimX=s.w*0.28, chimZ=0, sideWinZ=0, body;                   // door/window/chimney anchors (footprint-aware)
  const wingP=+Math.min(s.pitch*0.5, s.wallH*0.28).toFixed(2), wingH=+(s.wallH-wingP-0.25).toFixed(2);   // subordinate wings: roof ridge below the main eave so they tuck under cleanly
  if(s.footprint==='U'){                                                            // Normandy clos-masure: a bigger farmhouse + two lower barns GAPPED around an OPEN yard (gap > 0.75 clears the roof overhangs -> no clip)
    const backD=+(s.d*0.48).toFixed(2), sideW=+(s.w*0.20).toFixed(2), gap=1.1;
    const sideStart=-s.d/2+backD+gap, sideD=+(s.d/2-sideStart).toFixed(2), sideZ=+((sideStart+s.d/2)/2).toFixed(2);
    body=block(0, -s.d/2+backD/2, s.w, backD, s.wallH, s.pitch);                   // back range = the (deeper) farmhouse, faces the courtyard
    block(-s.w/2+sideW/2, sideZ, sideW, sideD, wingH, wingP, 'gable');             // left barn: lower + gapped so its verge overhang never reaches the farmhouse eave
    block( s.w/2-sideW/2, sideZ, sideW, sideD, wingH, wingP, 'gable');             // right barn -> yard opens toward +Z
    frontZ=-s.d/2+backD; frontWinX=s.w*0.22; chimZ=-s.d/2+backD/2; sideWinZ=sideZ; // wider window spacing off the door; side windows on the barn outer walls
  } else {
    body=block(0,0,s.w,s.d,s.wallH,s.pitch);
    if(s.footprint==='L'){ const wingW=+(s.w*0.5).toFixed(2), wingD=+(s.d*0.55).toFixed(2);
      block(s.w/2-wingW/2, s.d/2+wingD/2, wingW, wingD, wingH, wingP, 'gable'); }   // lower wing extends +Z on the +X side, tucks under the main roof
    else if(s.footprint==='T'){ const wingW=+(s.w*0.34).toFixed(2), wingD=+(s.d*0.55).toFixed(2);
      block(0, -(s.d/2+wingD/2), wingW, wingD, wingH, wingP, 'gable'); }            // lower centered stem off the BACK (-Z), front stays clear
    else if(s.footprint==='longbar'){ const extW=+(s.w*0.7).toFixed(2), extD=+(s.d*0.82).toFixed(2);
      block(s.w/2+extW/2, 0, extW, extD, wingH, wingP, 'gable'); }                  // lower attached range extending +X -> stepped longere
  }
  if(s.footprint!=='U' && s.d>s.w){ chimX=0; chimZ=s.d*0.28; }                     // deep building: ridge runs along Z -> straddle it there
  const ridgeH=s.wallH+s.pitch, chH=1.6;                                            // chimney straddles the ridge (peak) so it embeds in the roof instead of floating on a slope
  const chim=new THREE.Mesh(new THREE.BoxGeometry(0.7,chH,0.7), new THREE.MeshStandardMaterial({ color:0x8a4a30, roughness:0.95 }));
  chim.position.set(chimX, ridgeH-0.35+chH/2, chimZ); chim.castShadow=true; chim.receiveShadow=true; g.add(chim);
  const chimCap=new THREE.Mesh(new THREE.BoxGeometry(0.88,0.18,0.88), new THREE.MeshStandardMaterial({ color:0x5f3423, roughness:0.95 }));
  chimCap.position.set(chimX, ridgeH-0.35+chH+0.05, chimZ); chimCap.castShadow=true; g.add(chimCap);
  const frameMat=new THREE.MeshStandardMaterial({ color:0x4a3418, roughness:0.9 });
  const paneMat=new THREE.MeshStandardMaterial({ color:0x2a3a48, roughness:0.3, metalness:0.1 });
  const doorH=Math.min(2.8, s.wallH-0.35), doorW=1.5;                                    // single door sized so the ~2.57m-tall / 1.4m-wide (r=0.7) character can actually pass through (clears head + shoulders)
  const df=new THREE.Mesh(new THREE.BoxGeometry(doorW+0.16, doorH+0.12, 0.12), frameMat); df.position.set(0,(doorH+0.12)/2,frontZ+0.03); df.castShadow=true; g.add(df);
  const dr=new THREE.Mesh(new THREE.BoxGeometry(doorW, doorH, 0.1), new THREE.MeshStandardMaterial({ color:0x6e4a24, roughness:0.85 })); dr.position.set(0,doorH/2,frontZ+0.055); dr.castShadow=true; g.add(dr);
  const win=(px,pz,ry)=>{ const wg=new THREE.Group(); wg.position.set(px, Math.min(s.wallH-0.7,1.9), pz); if(ry)wg.rotation.y=ry;
    const fr=new THREE.Mesh(new THREE.BoxGeometry(0.95,1.05,0.1), frameMat); fr.position.z=0.03; fr.castShadow=true; wg.add(fr);
    const pa=new THREE.Mesh(new THREE.BoxGeometry(0.66,0.76,0.1), paneMat); pa.position.z=0.045; wg.add(pa); g.add(wg); };
  win(-frontWinX,frontZ,0); win(frontWinX,frontZ,0); win(s.w/2,sideWinZ,Math.PI/2); win(-s.w/2,sideWinZ,-Math.PI/2);
  // ---- dormers: gabled roof-windows on the main block's front (+Z) slope; wide, single-slope roofs only for this first pass ----
  const nDorm=parseInt(s.dormers||'0',10)||0;
  if(nDorm>0 && s.w>=s.d && s.roof!=='gambrel' && s.roof!=='saltbox'){
    const halfD=s.d/2, dw=1.5, dh=1.4, dd=1.25;                                                       // bigger dormer body -> holds a full-size window
    const xs = nDorm===1 ? [0] : [-s.w*0.24, s.w*0.24];
    for(const xd of xs){
      const zf=halfD*0.52, cz=zf-dd/2+0.1, bodyFront=cz+dd/2;
      const ysl=s.wallH + s.pitch*(1 - bodyFront/halfD);                                              // roof surface height at the dormer front edge
      const bd=new THREE.Mesh(new THREE.BoxGeometry(dw,dh,dd), wallMat());
      bd.position.set(xd, ysl+dh/2-0.15, cz); bd.castShadow=true; bd.receiveShadow=true; g.add(bd);   // embed 0.15 so the base meets the slope with no gap
      const cap=new THREE.Mesh(gableRoofGeo(dd,dw,0.5), roofMat()); cap.rotation.y=Math.PI/2;
      cap.position.set(xd, ysl+dh-0.15, cz); cap.castShadow=true; g.add(cap);                          // ridge runs front-back -> gable faces the camera
      const dfr=new THREE.Mesh(new THREE.BoxGeometry(0.95,1.05,0.08), frameMat); dfr.position.set(xd, ysl+dh*0.5, bodyFront+0.04); dfr.castShadow=true; g.add(dfr);   // window matches the building's normal window
      const dpa=new THREE.Mesh(new THREE.BoxGeometry(0.66,0.76,0.08), paneMat); dpa.position.set(xd, ysl+dh*0.5, bodyFront+0.06); g.add(dpa);
    }
  }
  return { group:g, colliders, body };
};
// ---- Cover pieces (linear). label + collider semantics; los=false -> movement-only (see-through). ----
BK.COVER={
  hedge:{ label:'Bocage hedgerow', thickness:1.0, height:1.6, los:true },
  drystone:{ label:'Drystone wall', thickness:0.5, height:1.4, los:true },
  fence:{ label:'Post & rail fence', thickness:0.26, height:0.95, los:false }
};
// PURE: a cover run centered at origin, length along local +X. Returns {group, body}; caller places/rotates/collides.
BK.buildCover=function(type, len){
  const S=this.COVER[type]||this.COVER.hedge, g=new THREE.Group(), L=Math.max(0.5,len), t=S.thickness, h=S.height; let body=null;
  if(type==='fence'){
    const wood=()=>new THREE.MeshStandardMaterial({ color:0x6b4a2a, roughness:0.9 });
    const nP=Math.max(2, Math.min(40, Math.round(L/2)+1));
    for(let i=0;i<nP;i++){ const x=-L/2 + L*i/(nP-1); const p=new THREE.Mesh(new THREE.BoxGeometry(0.14,h,0.14), wood()); p.position.set(x,h/2,0); p.castShadow=true; g.add(p); if(!body)body=p; }
    [h*0.62,h*0.30].forEach(ry=>{ const r=new THREE.Mesh(new THREE.BoxGeometry(L,0.1,0.07), wood()); r.position.set(0,ry,0); r.castShadow=true; g.add(r); });
  } else if(type==='drystone'){
    const wall=new THREE.Mesh(new THREE.BoxGeometry(L,h,t), new THREE.MeshStandardMaterial({ map:this.tex('drystone'), roughness:1.0 })); wall.position.set(0,h/2,0); wall.castShadow=true; wall.receiveShadow=true; g.add(wall); body=wall;
    const cap=new THREE.Mesh(new THREE.BoxGeometry(L,0.14,t+0.12), new THREE.MeshStandardMaterial({ color:0xb8b2a4, roughness:1.0 })); cap.position.set(0,h+0.02,0); cap.castShadow=true; g.add(cap);
  } else {   // bocage hedgerow: raised earth bank + solid green wall + irregular overlapping bush crown
    const leaf=new THREE.MeshStandardMaterial({ map:this.tex('hedge'), roughness:1.0 }); leaf.userData.nature=true;
    const jit=(i,k)=>{ const v=Math.sin(i*127.1 + k*311.7 + L*0.37)*43758.5453; return v-Math.floor(v); };   // deterministic 0..1 -> identical shape on save/reload
    const bankH=0.35, baseH=0.5, crownY=bankH+baseH;
    const bankMat=new THREE.MeshStandardMaterial({ map:this.tex('dirt'), roughness:1.0 }); bankMat.userData.nature=true; const bank=new THREE.Mesh(new THREE.BoxGeometry(L,bankH,t+0.3), bankMat); bank.position.set(0,bankH/2,0); bank.castShadow=true; bank.receiveShadow=true; g.add(bank);   // the defining Normandy earth bank
    const base=new THREE.Mesh(new THREE.BoxGeometry(L,baseH,t*0.9), leaf); base.position.set(0,bankH+baseH/2,0); base.castShadow=true; base.receiveShadow=true; g.add(base); body=base;   // solid continuous foliage wall (no gaps)
    const nB=Math.max(3, Math.min(40, Math.round(L/0.95)));
    for(let i=0;i<nB;i++){ const x=-L/2 + (i+0.5)*L/nB, r=0.5 + jit(i,1)*0.3;                                // overlapping blobs -> continuous bushy crown
      const b=new THREE.Mesh(new THREE.IcosahedronGeometry(r,1), leaf);
      b.position.set(x, crownY + jit(i,2)*0.3, (jit(i,3)-0.5)*t*0.4);
      b.scale.set(1.0, 0.85+jit(i,4)*0.3, 1.05); b.rotation.set(jit(i,5)*0.5, jit(i,6)*Math.PI, jit(i,7)*0.5); b.castShadow=true; g.add(b);
    }
  }
  return { group:g, body };
};

// ===== BUILDING KIT . GAME GLUE  (-> future game): place a kit building into the world =====
function _bkWorldBoxes(o){ o.grp.updateMatrixWorld(true); const L=o.localColliders; o.boxes=o.boxes||[];   // recompute world compound AABBs from local + transform
  for(let i=0;i<L.length;i++){ const c=L[i]; let b=o.boxes[i]||(o.boxes[i]=new THREE.Box3());
    b.min.set(c.minx,c.miny,c.minz); b.max.set(c.maxx,c.maxy,c.maxz); b.applyMatrix4(o.grp.matrixWorld); }
  o.boxes.length=L.length; }
const MELEE_IDLE={x:-0.05,y:0.82};
function makeSword(){
  const g=new THREE.Group();
  const steel=new THREE.MeshStandardMaterial({color:0xc9d2d6,roughness:0.3,metalness:0.7,emissive:0x223036,emissiveIntensity:0.15});
  const iron=new THREE.MeshStandardMaterial({color:0x3a3f42,roughness:0.5,metalness:0.6});
  const wood=new THREE.MeshStandardMaterial({color:0x4a3522,roughness:0.85});
  const grip=new THREE.Mesh(new THREE.CylinderGeometry(0.05,0.05,0.34,8),wood); grip.rotation.x=Math.PI/2; grip.position.z=0.0;
  const pommel=new THREE.Mesh(new THREE.SphereGeometry(0.07,10,8),iron); pommel.position.z=-0.18;
  const guard=new THREE.Mesh(new THREE.BoxGeometry(0.42,0.08,0.09),iron); guard.position.z=0.18;
  const blade=new THREE.Mesh(new THREE.BoxGeometry(0.085,0.17,1.15),steel); blade.position.z=0.8;
  const tip=new THREE.Mesh(new THREE.ConeGeometry(0.09,0.28,4),steel); tip.rotation.x=Math.PI/2; tip.position.z=1.52;
  g.add(grip,pommel,guard,blade,tip); g.traverse(o=>noRay(o));
  g.position.set(-0.34,1.45,0.12); g.rotation.set(MELEE_IDLE.x,MELEE_IDLE.y,0); g.visible=false; return g;
}
function makeAxe(){
  const g=new THREE.Group();
  const iron=new THREE.MeshStandardMaterial({color:0x4b5155,roughness:0.45,metalness:0.65,emissive:0x202829,emissiveIntensity:0.12});
  const wood=new THREE.MeshStandardMaterial({color:0x5a3f28,roughness:0.85});
  const haft=new THREE.Mesh(new THREE.CylinderGeometry(0.055,0.06,1.25,8),wood); haft.rotation.x=Math.PI/2; haft.position.z=0.35;
  const bit=new THREE.Mesh(new THREE.BoxGeometry(0.12,0.62,0.5),iron); bit.position.set(0.14,0,0.88);
  const edge=new THREE.Mesh(new THREE.BoxGeometry(0.05,0.7,0.18),iron); edge.position.set(0.36,0,0.88);
  const spike=new THREE.Mesh(new THREE.ConeGeometry(0.1,0.26,4),iron); spike.rotation.z=Math.PI/2; spike.position.set(-0.16,0,0.88);
  const cap=new THREE.Mesh(new THREE.CylinderGeometry(0.07,0.07,0.1,8),iron); cap.rotation.x=Math.PI/2; cap.position.z=0.98;
  g.add(haft,bit,edge,spike,cap); g.traverse(o=>noRay(o));
  g.position.set(-0.34,1.45,0.12); g.rotation.set(MELEE_IDLE.x,MELEE_IDLE.y,0); g.visible=false; return g;
}
function makeShield(accent){
  const g=new THREE.Group();
  const wood=new THREE.MeshStandardMaterial({color:0x6b4f2e,roughness:0.9});
  const iron=new THREE.MeshStandardMaterial({color:0x383d40,roughness:0.5,metalness:0.6});
  const boss=new THREE.MeshStandardMaterial({color:0x4a5054,roughness:0.4,metalness:0.7,emissive:accent,emissiveIntensity:0.0});
  const acc=new THREE.MeshStandardMaterial({color:accent,roughness:0.55,emissive:accent,emissiveIntensity:0.25});
  const disc=new THREE.Mesh(new THREE.CylinderGeometry(0.82,0.82,0.11,26),wood); disc.rotation.x=Math.PI/2;
  const rim=new THREE.Mesh(new THREE.TorusGeometry(0.82,0.075,8,30),iron); rim.position.z=0.0;
  const bossM=new THREE.Mesh(new THREE.ConeGeometry(0.17,0.22,16),boss); bossM.rotation.x=Math.PI/2; bossM.position.z=0.12;
  for(let k=0;k<4;k++){ const s=new THREE.Mesh(new THREE.BoxGeometry(1.54,0.1,0.02),acc); s.rotation.z=k*Math.PI/4; s.position.z=0.057; g.add(s); }
  g.add(disc,rim,bossM); g.traverse(o=>noRay(o));
  g.position.set(0.52,1.3,0.3); g.visible=false; g.userData.boss=boss; return g;
}

/* ================= HEADLESS SIM MATH (pure: zero THREE/DOM) =================
 * Phase-6 lift-out target. Kept contiguous so a future headless-SIM extraction
 * is a clean block lift. resolveShot operates on the passed world; the live
 * scene is mirrored into SIM by the game-side bridge (shamble-game.js).
 * ========================================================================== */
const FIXED_DT=1/60;
const BTN={FIRE:1,ADS:2,RELOAD:4,MELEE:8,GRENADE:16,SHIELD:32,SPRINT:64};
function makeInputCommand(seq,tick){ return {seq:seq|0,tick:tick|0,dt:FIXED_DT,moveX:0,moveZ:0,aimX:0,aimZ:1,aimY:0,buttons:0}; }
const BODY={...CHAR_DEFAULT.hit};   // hit-shape derives from the character recipe (one source -> look + hitbox can never drift)
const PART={ head:2.6, body:1.0, legs:0.65 };
function raySphere(ox,oy,oz,dx,dy,dz,cx,cy,cz,r){ const mx=ox-cx,my=oy-cy,mz=oz-cz,b=mx*dx+my*dy+mz*dz,c=mx*mx+my*my+mz*mz-r*r; if(c>0&&b>0)return -1; const disc=b*b-c; if(disc<0)return -1; let t=-b-Math.sqrt(disc); return t<0?0:t; }
function rayCapsule(ox,oy,oz,dx,dy,dz,ax,ay,az,bx,by,bz,r){ const bax=bx-ax,bay=by-ay,baz=bz-az,oax=ox-ax,oay=oy-ay,oaz=oz-az; const baba=bax*bax+bay*bay+baz*baz,bard=bax*dx+bay*dy+baz*dz,baoa=bax*oax+bay*oay+baz*oaz,rdoa=dx*oax+dy*oay+dz*oaz,oaoa=oax*oax+oay*oay+oaz*oaz; const a=baba-bard*bard; if(Math.abs(a)<1e-7){ const ta=raySphere(ox,oy,oz,dx,dy,dz,ax,ay,az,r),tb=raySphere(ox,oy,oz,dx,dy,dz,bx,by,bz,r); if(ta<0)return tb; if(tb<0)return ta; return ta<tb?ta:tb; } let b=baba*rdoa-baoa*bard,c=baba*oaoa-baoa*baoa-r*r*baba,h=b*b-a*c; if(h<0)return -1; let t=(-b-Math.sqrt(h))/a; const y=baoa+t*bard; if(y>0&&y<baba)return t<0?-1:t; return (y<=0)?raySphere(ox,oy,oz,dx,dy,dz,ax,ay,az,r):raySphere(ox,oy,oz,dx,dy,dz,bx,by,bz,r); }
function rayAabb(ox,oy,oz,dx,dy,dz,minx,miny,minz,maxx,maxy,maxz,outN){ let tmin=-Infinity,tmax=Infinity,nx=0,ny=0,nz=0;
  if(Math.abs(dx)<1e-9){ if(ox<minx||ox>maxx)return -1; } else { const inv=1/dx; let t1=(minx-ox)*inv,t2=(maxx-ox)*inv,s=-1; if(t1>t2){const tt=t1;t1=t2;t2=tt;s=1;} if(t1>tmin){tmin=t1;nx=s;ny=0;nz=0;} if(t2<tmax)tmax=t2; if(tmin>tmax)return -1; }
  if(Math.abs(dy)<1e-9){ if(oy<miny||oy>maxy)return -1; } else { const inv=1/dy; let t1=(miny-oy)*inv,t2=(maxy-oy)*inv,s=-1; if(t1>t2){const tt=t1;t1=t2;t2=tt;s=1;} if(t1>tmin){tmin=t1;nx=0;ny=s;nz=0;} if(t2<tmax)tmax=t2; if(tmin>tmax)return -1; }
  if(Math.abs(dz)<1e-9){ if(oz<minz||oz>maxz)return -1; } else { const inv=1/dz; let t1=(minz-oz)*inv,t2=(maxz-oz)*inv,s=-1; if(t1>t2){const tt=t1;t1=t2;t2=tt;s=1;} if(t1>tmin){tmin=t1;nx=0;ny=0;nz=s;} if(t2<tmax)tmax=t2; if(tmin>tmax)return -1; }
  let t=tmin; if(t<0){ t=tmax; if(t<0)return -1; } if(outN){outN[0]=nx;outN[1]=ny;outN[2]=nz;} return t; }
// PURE hitscan query — reads world, never mutates it (so the server can run it
// against a rewound snapshot for lag compensation, then apply damage live).
function resolveShot(world,shooterId,ox,oy,oz,dx,dy,dz,range){
  const res=world._res||(world._res={}), nrm=world._nrm||(world._nrm=[0,0,0,0]);
  let bestT=range,kind='none',id=-1,part='',nx=-dx,ny=-dy,nz=-dz;
  const P=world.players;
  for(let i=0;i<P.length;i++){ const p=P[i]; if(!p.alive||p.id===shooterId)continue; const baseY=p.y||0, drop=p.stance?BODY.crouchDrop:0;
    let t=raySphere(ox,oy,oz,dx,dy,dz,p.x,baseY+BODY.headY-drop,p.z,BODY.headR);
    if(t>=0&&t<bestT){ bestT=t;kind='enemy';id=p.id;part='head'; nx=ox+dx*t-p.x;ny=oy+dy*t-(baseY+BODY.headY-drop);nz=oz+dz*t-p.z; }
    t=rayCapsule(ox,oy,oz,dx,dy,dz,p.x,baseY+BODY.torsoBotY-drop,p.z,p.x,baseY+BODY.torsoTopY-drop,p.z,BODY.torsoR);
    if(t>=0&&t<bestT){ bestT=t;kind='enemy';id=p.id;part='body'; nx=ox+dx*t-p.x;ny=0;nz=oz+dz*t-p.z; }
    t=rayCapsule(ox,oy,oz,dx,dy,dz,p.x,baseY+BODY.legBotY-drop,p.z,p.x,baseY+BODY.legTopY-drop,p.z,BODY.legR);
    if(t>=0&&t<bestT){ bestT=t;kind='enemy';id=p.id;part='legs'; nx=ox+dx*t-p.x;ny=0;nz=oz+dz*t-p.z; }
  }
  const Sx=world.statics, si=rayStatics(Sx,ox,oy,oz,dx,dy,dz,bestT,false,nrm);
  if(si>=0){ const s=Sx[si]; bestT=nrm[3]; kind=s.kind;id=s.id;part=''; nx=nrm[0];ny=nrm[1];nz=nrm[2]; }
  if(dy<0){ const tg=(world.groundY-oy)/dy; if(tg>=0&&tg<bestT){ bestT=tg;kind='block';id=-1;part='';nx=0;ny=1;nz=0; } }
  const nl=hyp3(nx,ny,nz)||1;
  res.t=bestT; res.px=ox+dx*bestT; res.py=oy+dy*bestT; res.pz=oz+dz*bestT; res.nx=nx/nl; res.ny=ny/nl; res.nz=nz/nl; res.kind=kind; res.id=id; res.part=part; res.mult=part==='head'?PART.head:part==='legs'?PART.legs:PART.body; return res;
}

/* ---- VIEW <-> SIM ADAPTER (the bridge; allowed to touch THREE) ----
 * Refreshes a persistent, pooled world snapshot from the live scene once per
 * shot, then castShot runs the pure resolveShot above. In Stage 2 this `SIM`
 * object is exactly what the server owns and the client predicts/reconciles. */
const SIM={ players:[], statics:[], groundY:0, _res:{}, _nrm:[0,0,0,0] };   // _nrm: hit normal xyz + t (rayStatics)
// Ray vs one SIM static: its world AABB, or — when s.rot — an oriented box rotated about Y (editor 45° pieces): the ray is
// taken into the box's frame (rotation keeps t), the hit normal rotated back to world. Same transform as resolveObstacles.
function rayStatic(ox,oy,oz,dx,dy,dz,s,out){
  if(s.rot!==true) return rayAabb(ox,oy,oz,dx,dy,dz,s.minx,s.miny,s.minz,s.maxx,s.maxy,s.maxz,out);
  const c=s.rc, sn=s.rs, px=ox-s.cx, pz=oz-s.cz;
  const t=rayAabb(c*px-sn*pz,oy,sn*px+c*pz, c*dx-sn*dz,dy,sn*dx+c*dz, -s.hw,s.miny,-s.hd, s.hw,s.maxy,s.hd, out);
  if(t>=0&&out){ const nx=out[0], nz=out[2]; out[0]=c*nx+sn*nz; out[2]=-sn*nx+c*nz; }
  return t; }
// All statics in one loop, the slab test inlined (B7): a call per static boxed its float arguments and result, about 80 B
// each, so hasLOS and resolveShot allocated 18-26 KB per call. Returns the index of the nearest static hit with t < maxT
// (the first one found when anyHit), or -1; writes the hit normal to out[0..2] and t to out[3]. Same results as rayStatic
// for finite inputs (a NaN ray counts as blocked).
function rayStatics(S,ox,oy,oz,dx,dy,dz,maxT,anyHit,out){
  let best=-1, bestT=maxT, bnx=0, bny=0, bnz=0;
  for(let i=0;i<S.length;i++){ const s=S[i]; if(s.alive===false)continue;
    let px=ox, pz=oz, rx=dx, rz=dz, minx=s.minx, minz=s.minz, maxx=s.maxx, maxz=s.maxz; const rot=s.rot===true;
    if(rot){ const c=s.rc, sn=s.rs, qx=ox-s.cx, qz=oz-s.cz; px=c*qx-sn*qz; pz=sn*qx+c*qz; rx=c*dx-sn*dz; rz=sn*dx+c*dz; minx=-s.hw; minz=-s.hd; maxx=s.hw; maxz=s.hd; }
    let tmin=-Infinity, tmax=Infinity, nx=0, ny=0, nz=0, t1=0, t2=0, tt=0, sg=0;
    if(Math.abs(rx)<1e-9){ if(px<minx||px>maxx)continue; } else { const inv=1/rx; t1=(minx-px)*inv; t2=(maxx-px)*inv; sg=-1; if(t1>t2){tt=t1;t1=t2;t2=tt;sg=1;} if(t1>tmin){tmin=t1;nx=sg;ny=0;nz=0;} if(t2<tmax)tmax=t2; if(tmin>tmax)continue; }
    if(Math.abs(dy)<1e-9){ if(oy<s.miny||oy>s.maxy)continue; } else { const inv=1/dy; t1=(s.miny-oy)*inv; t2=(s.maxy-oy)*inv; sg=-1; if(t1>t2){tt=t1;t1=t2;t2=tt;sg=1;} if(t1>tmin){tmin=t1;nx=0;ny=sg;nz=0;} if(t2<tmax)tmax=t2; if(tmin>tmax)continue; }
    if(Math.abs(rz)<1e-9){ if(pz<minz||pz>maxz)continue; } else { const inv=1/rz; t1=(minz-pz)*inv; t2=(maxz-pz)*inv; sg=-1; if(t1>t2){tt=t1;t1=t2;t2=tt;sg=1;} if(t1>tmin){tmin=t1;nx=0;ny=0;nz=sg;} if(t2<tmax)tmax=t2; if(tmin>tmax)continue; }
    let t=tmin; if(t<0){ t=tmax; if(t<0)continue; }
    if(t>=bestT)continue;
    if(rot){ const c=s.rc, sn=s.rs, wx=c*nx+sn*nz; nz=-sn*nx+c*nz; nx=wx; }
    best=i; bestT=t; bnx=nx; bny=ny; bnz=nz; if(anyHit)break; }
  if(best>=0&&out){ out[0]=bnx; out[1]=bny; out[2]=bnz; out[3]=bestT; }
  return best; }
function hasLOS(a,b){ let dx=b.x-a.x,dy=b.y-a.y,dz=b.z-a.z; const dist=hyp3(dx,dy,dz); if(dist<1e-4)return true; dx/=dist;dy/=dist;dz/=dist; return rayStatics(SIM.statics,a.x,a.y,a.z,dx,dy,dz,dist-0.5,true,null)<0; }