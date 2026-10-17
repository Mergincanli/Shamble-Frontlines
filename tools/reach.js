// Spawn-to-goal reachability on the NAV grid: node tools/preview.js . eval tools/reach.js [--map=tri] (Linux: prefix xvfb-run -a).
// Starts: every bot spawn point (teamSpawnPoint per active slot) plus your mapSpawn. Goals per team: other teams' structures
// (push), the tech points (objective), other teams' truck dock and lane points (hunt). Measures (each must be 100 % on DM_Tri):
//   route  the real navPath (8000-expansion cap) finds a route to every goal. A flood fill over the same grid is the ground
//          truth, so a miss reads "no route" (the flood can't reach it either) or "cap" (it can; the search gave up).
//   climb  sampled every 0.25 m, a route never stands on a surface higher than EJUMP_REACH: buildings are not NAV cells, so
//          a route through a live one would need a climb the bot can't make. Checked up to where the bot stops, as the bot
//          loop does (15 m from a structure's centre, 13 m from a truck, on the tech box), along:
//          - objectives: start -> each tech point and truck point, direct;
//          - siegeChain: what push bots do. From the start, the nearest live structure of each other team (XZ, as
//            nearestEnemyStructure); stop 15 m short; it falls (dead buildings are no surface); repeat until all are down.
//   direct start -> each structure, ignoring that nearer ones stand first: information only (bots never do this).
// Also: route length per team, structure gaps under 6 m, each truck's dock -> lane leg against every box. Three-teams builds only.
(function(){
  if(typeof teamSpawnPoint!=='function'||typeof TEAM_INDEX==='undefined') return {err:'needs a three-teams build (teamSpawnPoint, TEAM_INDEX)'};
  deploy(); if(!NAV.ok) navBuild();
  const nx=NAV.nx, nz=NAV.nz, blk=NAV.blk;
  const flood=(c0)=>{ const seen=new Uint8Array(nx*nz); if(c0<0) return seen; let s=c0; if(blk[s]){ s=_navNearestFree(s); if(s<0) return seen; }
    const q=new Int32Array(nx*nz); let h=0,t=0; q[t++]=s; seen[s]=1;
    while(h<t){ const c=q[h++], cx=c%nx, cz=(c/nx)|0;
      for(let k=0;k<8;k++){ const ox=_NDX[k], oz=_NDZ[k], ex=cx+ox, ez=cz+oz; if(ex<0||ez<0||ex>=nx||ez>=nz) continue; const e=ez*nx+ex;
        if(blk[e]||seen[e]) continue; if(ox!==0&&oz!==0&&(blk[cz*nx+ex]||blk[ez*nx+cx])) continue; seen[e]=1; q[t++]=e; } }
    return seen; };
  const goalCell=(x,z)=>{ let t=_navCell(x,z); if(t<0) return -1; if(blk[t]) t=_navNearestFree(t); return t; };
  const boxOf=b=>({minx:b.box.min.x,minz:b.box.min.z,maxx:b.box.max.x,maxz:b.box.max.z});
  const inBox=(x,z,bx,pad)=>bx&&x>bx.minx-pad&&x<bx.maxx+pad&&z>bx.minz-pad&&z<bx.maxz+pad;
  const pts=new Float32Array(256);
  // walk a navPath route from (sx,sz) toward goal g; stop within g.stopR; returns {ok, n, len, hi, at, sx, sz} (sx,sz = stop point)
  const walk=(sx,sz,g)=>{ const n=navPath(sx,sz,g.x,g.z,pts); if(!n) return {n:0};
    let ax=sx, az=sz, len=0, hi=0, at=null, stopped=(sx-g.x)*(sx-g.x)+(sz-g.z)*(sz-g.z)<=g.stopR*g.stopR, ex=sx, ez=sz;
    for(let i=0;i<n&&!stopped;i++){ const bx=pts[i*2], bz=pts[i*2+1], L=Math.sqrt((bx-ax)*(bx-ax)+(bz-az)*(bz-az)), k=Math.max(1,Math.ceil(L/0.25));
      for(let j=1;j<=k;j++){ const x=ax+(bx-ax)*j/k, z=az+(bz-az)*j/k; len+=L/k; ex=x; ez=z;
        if((x-g.x)*(x-g.x)+(z-g.z)*(z-g.z)<=g.stopR*g.stopR){ stopped=true; break; }
        if(inBox(x,z,g.box,1)) continue; const h=surfaceHeightAt(x,z); if(h>hi){ hi=h; at=[+x.toFixed(1),+z.toFixed(1)]; } }
      ax=bx; az=bz; }
    return {n, len, hi, at, sx:ex, sz:ez}; };
  const teams=TEAMS.map(t=>t.id).filter(id=>TEAM_INDEX[id].active);
  const res={}, fails=[], info=[]; const c={routes:0,routeOk:0,obj:0,objOk:0,legs:0,legsOk:0,direct:0,directOk:0};
  for(const id of teams){
    const starts=[]; const v=new THREE.Vector3();
    for(let s=1;s<SOLDIERS.length;s++){ const en=SOLDIERS[s]; if(en.active&&en.team===id){ teamSpawnPoint(id,en.id,v); starts.push({who:'slot'+en.id,x:v.x,z:v.z}); } }
    if(id===PLAYER_TEAM) starts.push({who:'you',x:mapSpawn.x,z:mapSpawn.z});
    const structs=buildings.filter(b=>{ const bt=teamOf(b.team); return bt&&bt!==id&&TEAM_INDEX[bt].active&&b.alive; }).map(b=>({what:teamOf(b.team)+':'+b.type,x:b.mesh.position.x,z:b.mesh.position.z,box:boxOf(b),stopR:15,b}));
    const objs=TBOX.points.map(p=>({what:'tech:'+p.name,x:p.x,z:p.z,box:null,stopR:0}));
    for(const o of teams) if(o!==id){ const e=TEAM_INDEX[o]; objs.push({what:o+':dock',x:e.dockX,z:e.dockZ,box:null,stopR:13}); objs.push({what:o+':lane',x:e.laneX,z:e.laneZ,box:null,stopR:13}); }
    const lens=[];
    for(const st of starts){ const seen=flood(_navCell(st.x,st.z));
      for(const g of structs.concat(objs)){ c.routes++; const r=walk(st.x,st.z,g); const t=goalCell(g.x,g.z);
        if(!r.n){ fails.push('route '+id+' '+st.who+' -> '+g.what+': '+((t>=0&&seen[t])?'cap':'no route')); continue; } c.routeOk++; lens.push(r.len);
        if(g.b){ c.direct++; if(r.hi<=EJUMP_REACH) c.directOk++; else info.push('direct '+id+' '+st.who+' -> '+g.what+': '+r.hi.toFixed(1)+' m at '+r.at.join(',')); }
        else { c.obj++; if(r.hi<=EJUMP_REACH) c.objOk++; else fails.push('climb '+id+' '+st.who+' -> '+g.what+': '+r.hi.toFixed(1)+' m at '+r.at.join(',')); } }
      // siege chain against each other team, nearest live structure first (what push bots do)
      for(const o of teams){ if(o===id) continue; const left=structs.filter(g=>g.what.startsWith(o+':')); let x=st.x, z=st.z;
        while(left.length){ let bi=0, bd=Infinity; for(let i=0;i<left.length;i++){ const d=(left[i].x-x)*(left[i].x-x)+(left[i].z-z)*(left[i].z-z); if(d<bd){ bd=d; bi=i; } }
          const g=left.splice(bi,1)[0]; c.legs++; const r=walk(x,z,g);
          if(!r.n) fails.push('chain '+id+' '+st.who+' -> '+g.what+': no route');
          else if(r.hi>EJUMP_REACH) fails.push('chain '+id+' '+st.who+' -> '+g.what+': climb '+r.hi.toFixed(1)+' m at '+r.at.join(','));
          else { c.legsOk++; x=r.sx; z=r.sz; }
          g.b.alive=false; }
        for(const g of structs) g.b.alive=true; } }
    lens.sort((a,b)=>a-b); res[id]={starts:starts.length, goals:structs.length+objs.length, routeLenToStop:{min:+(lens[0]||0).toFixed(0), p50:+(lens[lens.length>>1]||0).toFixed(0), max:+(lens[lens.length-1]||0).toFixed(0)}};
  }
  for(const b of buildings) b.alive=true;
  // structure gaps (buildings + enterable barracks walls), XZ distance between boxes
  const sb=buildings.map(b=>Object.assign({n:teamOf(b.team)+':'+b.type},boxOf(b)));
  for(const e of barracksList){ const bx={n:teamOf(e.team)+':ebarracks',minx:Infinity,minz:Infinity,maxx:-Infinity,maxz:-Infinity}; for(const w of e.walls){ bx.minx=Math.min(bx.minx,w.box.min.x); bx.minz=Math.min(bx.minz,w.box.min.z); bx.maxx=Math.max(bx.maxx,w.box.max.x); bx.maxz=Math.max(bx.maxz,w.box.max.z); } sb.push(bx); }
  const gaps=[]; let minGap=Infinity;
  for(let i=0;i<sb.length;i++) for(let j=i+1;j<sb.length;j++){ const a=sb[i], b=sb[j], dx=Math.max(0,Math.max(a.minx,b.minx)-Math.min(a.maxx,b.maxx)), dz=Math.max(0,Math.max(a.minz,b.minz)-Math.min(a.maxz,b.maxz)), g=Math.sqrt(dx*dx+dz*dz);
    if(g<minGap) minGap=g; if(g<6) gaps.push(a.n+' / '+b.n+' '+g.toFixed(1)+' m'); }
  // truck legs: the straight dock -> lane line (trucks don't path) against every structure and crate box, 2 m truck half-width
  const allBoxes=sb.concat(obstacles.filter(o=>o.box).map(o=>({n:'crate',minx:o.box.min.x,minz:o.box.min.z,maxx:o.box.max.x,maxz:o.box.max.z})));
  const legs={};
  for(const id of teams){ const e=TEAM_INDEX[id], L=Math.sqrt((e.laneX-e.dockX)*(e.laneX-e.dockX)+(e.laneZ-e.dockZ)*(e.laneZ-e.dockZ)), k=Math.ceil(L/0.25), hit=new Set();
    for(let j=0;j<=k;j++){ const x=e.dockX+(e.laneX-e.dockX)*j/k, z=e.dockZ+(e.laneZ-e.dockZ)*j/k; for(const b of allBoxes) if(b.n!==id+':refinery'&&inBox(x,z,b,2)) hit.add(b.n); }
    legs[id]={len:+L.toFixed(0), crosses:[...hit]}; }
  const pct=(a,b)=>b?+(100*a/b).toFixed(1):null;
  return { map:curMap.name, teams, ejumpReach:+EJUMP_REACH.toFixed(2),
    routePct:pct(c.routeOk,c.routes), routes:c.routes, climbObjectivesPct:pct(c.objOk,c.obj), objectiveRoutes:c.obj, siegeChainPct:pct(c.legsOk,c.legs), siegeLegs:c.legs,
    directToEachStructurePct:pct(c.directOk,c.direct), perTeam:res, minStructureGap:+minGap.toFixed(1), gapsUnder6:gaps, truckLegs:legs,
    fails:fails.slice(0,40), failCount:fails.length, directInfo:info.slice(0,20), directInfoCount:info.length };
})()
