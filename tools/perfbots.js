// Bot-count scaling: the cost of one update() step with N soldiers, march and combat, plus one render pass's draw calls.
// node --expose-gc tools/preview.js . eval tools/perfbots.js [--map=tri] [--pre=<file.js>]   (Linux: prefix xvfb-run -a)
// --pre sets the soldier count before boot: three-teams builds TUNE.teams.size=4|8|16 (12, 24, 48 soldiers on three teams);
// older builds T.enemyCount=11|23|47 (and T.eliteCount). Each phase: deploy, 600 warm-up steps, 1200 measured steps at 1/60 s,
// repeated 3 times in this process; you stay alive (DEV.god).
//   march   from the spawn points (bots walk out; the first fights start near the middle)
//   combat  every active bot placed on a 26 m ring around the supply field, slot by slot, so all teams engage at once
// Reports mean / p50 / p95 / max ms per step, LOS checks per second from AI_STATE (three-teams builds), and heapUsed growth
// per second across the measured window (rough: a GC inside the window lowers it; for allocation rates use --heap-prof).
// Headless CPU timing is pessimistic and noisy: compare builds on the same machine, never against Brave numbers.
(function(){
  const now=()=>Number(process.hrtime.bigint())/1e6, gc=(typeof globalThis.gc==='function')?globalThis.gc:null;
  const soldiers=()=>(typeof SOLDIERS!=='undefined')?SOLDIERS.filter(s=>s===ps||s.active!==false):[ps].concat(enemies);
  const fx=(typeof HARV!=='undefined')?HARV.fieldX:0, fz=(typeof HARV!=='undefined')?HARV.fieldZ:0;
  const ring=()=>{ const L=enemies.filter(e=>e.active!==false); L.forEach((e,i)=>{ const a=i/L.length*6.2832; e.group.position.set(fx+26*Math.sin(a),0,fz+26*Math.cos(a)); }); player.position.set(fx,0,fz+40); };
  const q=(a,p)=>a[Math.min(a.length-1,Math.floor(p*a.length))];
  const run=(setup)=>{ deploy(); DEV.god=true; setup(); const W=600, M=1200, t=new Float64Array(M);
    for(let f=0;f<W;f++){ if(typeof _frameId==='number')_frameId++; update(1/60); }
    const c0=(typeof AI_STATE!=='undefined')?AI_STATE.checks:null; if(gc) gc(); const h0=process.memoryUsage().heapUsed;
    for(let f=0;f<M;f++){ if(typeof _frameId==='number')_frameId++; const a=now(); update(1/60); t[f]=now()-a; }
    const h1=process.memoryUsage().heapUsed, c1=(typeof AI_STATE!=='undefined')?AI_STATE.checks:null;
    const s=Array.from(t).sort((x,y)=>x-y), mean=s.reduce((x,y)=>x+y,0)/M;
    return {mean:+mean.toFixed(3), p50:+q(s,0.5).toFixed(3), p95:+q(s,0.95).toFixed(3), max:+s[M-1].toFixed(3), losPerSec:c0===null?null:+((c1-c0)/(M/60)).toFixed(0), heapMBs:+(((h1-h0)/1e6)/(M/60)).toFixed(3)}; };
  const out={map:(typeof curMap!=='undefined'?curMap.name:null), soldiers:soldiers().length, teamsSize:(typeof TUNE!=='undefined')?TUNE.teams.size:null, gc:!!gc, march:[], combat:[]};
  for(let r=0;r<3;r++){ out.march.push(run(()=>{})); out.combat.push(run(ring)); }
  // one render pass in combat: draw calls, triangles, shader programs
  deploy(); ring(); for(let f=0;f<120;f++){ if(typeof _frameId==='number')_frameId++; update(1/60); }
  renderer.info.autoReset=true; renderer.render(scene,camera);
  out.render={calls:renderer.info.render.calls, triangles:renderer.info.render.triangles, programs:(renderer.info.programs||[]).length, geometries:renderer.info.memory.geometries, textures:renderer.info.memory.textures};
  DEV.god=false; return out;
})()
