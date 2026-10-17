// Headless 90 s match (globalThis.__simSecs overrides): node tools/preview.js . eval tools/sim.js [--map=tri] (Linux: prefix xvfb-run -a) -> bot stall %, stalls near
// village pieces, harvesters, and (three-teams builds) a kill matrix, deaths, deliveries, trucks and buildings lost, captures, era
// per team, engaged-bot stalls and the NAV starvation share. Guards keep it running on pre-three-teams builds (fields read null).
// Bot behaviour is random: compare 3+ runs per build, not single runs.
(function(){
  const has3=(typeof applyDamage==='function'&&typeof SOLDIERS!=='undefined'&&typeof TEAMS!=='undefined');
  const teamOfSlot=s=>(s&&s.team)||(s===ps?'player':'hostile');
  const teams=has3?TEAMS.map(t=>t.id):['player','hostile'];
  const zero=()=>{ const o={}; for(const t of teams) o[t]=0; return o; };
  // kill matrix (attacker team -> victim team) and friendly-fire attempts, from the damage layer (three-teams builds only)
  const km={}; for(const a of teams){ km[a]=zero(); } let ffAttempts=0, ffApplied=0;   // km, not kills: the game has a global kills
  let _ad=null;
  if(has3){ _ad=applyDamage; applyDamage=function(a,v,amount){ const A=SOLDIERS[a], V=SOLDIERS[v]; const r=_ad.apply(this,arguments);
    if(A&&V&&A!==V&&A.team===V.team){ ffAttempts++; if(r>0) ffApplied++; }
    if(r>0&&A&&V&&A!==V&&_dmgEv.killed&&_dmgEv.v===v&&km[A.team]) km[A.team][V.team]++;   // r>0: a guarded call leaves the previous event in _dmgEv
    return r; }; }
  deploy();
  // a wrapper snippet may set globalThis.__simSecs for a longer run
  const dt=1/60, secs=(typeof globalThis.__simSecs==='number'&&globalThis.__simSecs>0)?globalThis.__simSecs:90, trace=[]; let err=null, navStarved=0, steps=0;
  const soldiers=has3?SOLDIERS:[ps].concat(enemies);
  const deaths=zero(), wasAlive=soldiers.map(s=>s===ps?!ps.dead:s.alive);
  const trucks=(typeof harvesters!=='undefined')?harvesters:[], truckAlive=trucks.map(h=>h.alive), trucksLost={};
  const bAlive=buildings.map(b=>b.alive), bLost={};
  let tbOwner=(typeof tbox!=='undefined'&&tbox)?tbox.owner:null; const captures={tech:0,silo:0,byTeam:{}};
  const siloOwner=(typeof silos!=='undefined')?silos.map(s=>s.owner):[];
  const era=[];
  try{
    for(let f=0; f<secs*60; f++){
      if(typeof _frameId==='number') _frameId++;
      update(dt); steps++; if(typeof NAV!=='undefined'&&NAV.budget===0) navStarved++;
      for(let k=0;k<soldiers.length;k++){ const s=soldiers[k], al=s===ps?!ps.dead:(s.active!==false&&s.alive); if(wasAlive[k]&&!al){ const t=teamOfSlot(s); deaths[t]=(deaths[t]||0)+1; } wasAlive[k]=al; }
      for(let k=0;k<trucks.length;k++){ const h=trucks[k]; if(truckAlive[k]&&!h.alive) trucksLost[h.team]=(trucksLost[h.team]||0)+1; truckAlive[k]=h.alive; }
      for(let k=0;k<buildings.length;k++){ const b=buildings[k]; if(bAlive[k]&&!b.alive) bLost[b.team]=(bLost[b.team]||0)+1; bAlive[k]=b.alive; }
      if(typeof tbox!=='undefined'&&tbox&&tbox.owner!==tbOwner){ if(tbox.owner&&tbox.owner!=='neutral'){ captures.tech++; captures.byTeam[tbox.owner]=(captures.byTeam[tbox.owner]||0)+1; } tbOwner=tbox.owner; }
      for(let k=0;k<siloOwner.length;k++){ const o=silos[k].owner; if(o!==siloOwner[k]){ if(o&&o!=='neutral'){ captures.silo++; captures.byTeam[o]=(captures.byTeam[o]||0)+1; } siloOwner[k]=o; } }
      if(f%(15*60)===0&&typeof teamEco!=='undefined'){ const e={t:f/60}; for(const t of (has3?teams:Object.keys(teamEco))) if(teamEco[t]) e[t]=teamEco[t].era; era.push(e); }
      if(f%30===0) trace.push(enemies.map(e=>(e.active!==false&&e.alive)?[e.group.position.x,e.group.position.z,(e.targetId!=null&&e.targetId>=0)?1:0]:null));   // only active slots
    }
  }catch(e){ err=String(e&&e.stack||e).slice(0,600); }
  if(_ad) applyDamage=_ad;
  // stall = alive in two samples 3 s apart (6 samples) and moved < 0.3 m, measured per enemy-window; engaged = had a target at the end
  let windows=0, stalls=0, engW=0, engS=0; const stallPts=[];
  for(let i=6;i<trace.length;i++) for(let k=0;k<trace[i].length;k++){ const a=trace[i-6][k], b=trace[i][k]; if(a&&b){ windows++; const st=Math.hypot(a[0]-b[0],a[1]-b[1])<0.3; if(b[2]){ engW++; if(st)engS++; }
    if(st){ stalls++; if(!b[2]) stallPts.push([Math.round(b[0]),Math.round(b[1])]); } } }
  const vb=[]; const B3=new THREE.Box3();
  const push=g=>{ if(!g) return; g.updateMatrixWorld(true); B3.setFromObject(g); vb.push([B3.min.x,B3.min.z,B3.max.x,B3.max.z]); };
  (typeof houses!=='undefined'?houses:[]).forEach(h=>push(h.grp||h.group||h.mesh)); (typeof trees!=='undefined'?trees:[]).forEach(t=>push(t.group||t.mesh||t.grp));
  (typeof props!=='undefined'?props:[]).forEach(p=>push(p.group||p.mesh||p.grp)); (typeof covers!=='undefined'?covers:[]).forEach(c=>push(c.group||c.mesh||c.grp));
  const dBox=(p,b)=>Math.hypot(Math.max(b[0]-p[0],0,p[0]-b[2]),Math.max(b[1]-p[1],0,p[1]-b[3]));
  const nearVillage=stallPts.filter(p=>vb.some(b=>dBox(p,b)<3)).length;
  const hv=trucks.map(h=>({team:h.team, state:h.state, parked:!!h.parked, x:+h.group.position.x.toFixed(1), z:+h.group.position.z.toFixed(1), docks:h.docks||h.trips||0}));
  const deliveries={}; for(const h of trucks) if(!h.parked) deliveries[h.team]=(deliveries[h.team]||0)+(h.docks||0);
  return { map:(typeof curMap!=='undefined'?curMap.name:null), secs, mode, enemies:enemies.length, active:enemies.filter(e=>e.active!==false).length, samples:trace.length, windows,
    stallPct:+(100*stalls/Math.max(1,windows)).toFixed(1), engagedStallPct:engW?+(100*engS/engW).toFixed(1):null, engagedWindows:engW, freeStallPct:+(100*(stalls-engS)/Math.max(1,windows-engW)).toFixed(1),
    stallsNearVillagePiece:nearVillage, villagePieces:vb.length, stallSpots:[...new Set(stallPts.map(p=>p.join(',')))].slice(0,12),
    navStarvedPct:(typeof NAV!=='undefined')?+(100*navStarved/Math.max(1,steps)).toFixed(2):null, navDefinition:'share of update() steps that end with NAV.budget===0 (every route search of that step used)',
    kills:has3?km:null, ffAttempts:has3?ffAttempts:null, ffApplied:has3?ffApplied:null, deaths, deliveries, trucksLost, buildingsLost:bLost, captures, era,
    slotFrags:has3?SOLDIERS.map(s=>(s===ps||s.active)?(s.frags||0):null):null,   // K4: slot-order bias shows as low slots out-fragging high ones
    harvesters:hv, playerHP:(typeof ps!=='undefined'?Math.round(ps.health):null), err };
})()
