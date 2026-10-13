// Headless 90 s match: node tools/preview.js . eval tools/sim.js (Linux: prefix xvfb-run -a) -> bot stall %, stalls near village pieces, harvesters.
// Bot behaviour is random: compare 3+ runs per build, not single runs.
(function(){
  deploy();
  const dt=1/60, secs=90, trace=[]; let err=null;
  try{
    for(let f=0; f<secs*60; f++){
      update(dt);
      if(f%30===0) trace.push(enemies.map(e=>e.alive?[e.group.position.x,e.group.position.z]:null));
    }
  }catch(e){ err=String(e&&e.stack||e).slice(0,600); }
  // stall = alive in two samples 3 s apart (6 samples) and moved < 0.3 m, measured per enemy-window
  let windows=0, stalls=0; const stallPts=[];
  for(let i=6;i<trace.length;i++) for(let k=0;k<trace[i].length;k++){ const a=trace[i-6][k], b=trace[i][k]; if(a&&b){ windows++; if(Math.hypot(a[0]-b[0],a[1]-b[1])<0.3){ stalls++; stallPts.push([Math.round(b[0]),Math.round(b[1])]); } } }
  const vb=[]; const B3=new THREE.Box3();
  const push=g=>{ if(!g) return; g.updateMatrixWorld(true); B3.setFromObject(g); vb.push([B3.min.x,B3.min.z,B3.max.x,B3.max.z]); };
  (typeof houses!=='undefined'?houses:[]).forEach(h=>push(h.grp||h.group||h.mesh)); (typeof trees!=='undefined'?trees:[]).forEach(t=>push(t.group||t.mesh||t.grp));
  (typeof props!=='undefined'?props:[]).forEach(p=>push(p.group||p.mesh||p.grp)); (typeof covers!=='undefined'?covers:[]).forEach(c=>push(c.group||c.mesh||c.grp));
  const dBox=(p,b)=>Math.hypot(Math.max(b[0]-p[0],0,p[0]-b[2]),Math.max(b[1]-p[1],0,p[1]-b[3]));
  const nearVillage=stallPts.filter(p=>vb.some(b=>dBox(p,b)<3)).length;
  const hv=(typeof harvesters!=='undefined')?harvesters.map(h=>({team:h.team, state:h.state, x:+h.group.position.x.toFixed(1), z:+h.group.position.z.toFixed(1), docks:h.docks||h.trips||0})):[];
  return { mode, enemies:enemies.length, samples:trace.length, windows, stallPct:+(100*stalls/Math.max(1,windows)).toFixed(1), stallsNearVillagePiece:nearVillage, villagePieces:vb.length, stallSpots:[...new Set(stallPts.map(p=>p.join(',')))].slice(0,12), harvesters:hv, playerHP:(typeof ps!=='undefined'?Math.round(ps.health):null), err };
})()
