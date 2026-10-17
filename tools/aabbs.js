// Collision boxes + objective zones from the live SIM: node tools/preview.js . eval tools/aabbs.js > .gate/aabbs.json (Linux: prefix xvfb-run -a; input for village.py).
(function(){
  if(typeof syncSimWorld==='function') syncSimWorld();
  const st=(SIM&&SIM.statics)||[]; const s0=st[0]||{};
  const keys=Object.keys(s0);
  const box=o=>[o.kind, +o.minx.toFixed(1), +o.minz.toFixed(1), +o.maxx.toFixed(1), +o.maxz.toFixed(1), +o.maxy.toFixed(1)];
  const zones={ silos:((typeof SILO!=='undefined'&&SILO.points)?SILO.points:(typeof silos!=='undefined'?silos:[])).map(s=>({x:s.x??s.pos?.x??s.group?.position.x, z:s.z??s.pos?.z??s.group?.position.z, r:s.r||s.radius||T.siloRadius||null})),
    tech:(typeof TBOX!=='undefined'&&TBOX.points?TBOX.points:(DM_Cabana.techPoints||[])).map(t=>({x:t.x,z:t.z})),   // the live map's points (DM_Cabana's on older builds)
    field:(typeof HARV!=='undefined'&&Number.isFinite(HARV.fieldX))?{x:HARV.fieldX,z:HARV.fieldZ}:DM_Cabana.harvField,
    spawn:(typeof mapSpawn!=='undefined'&&mapSpawn)?{x:mapSpawn.x,z:mapSpawn.z}:DM_Cabana.spawn, resupply:(typeof terminalPos!=='undefined'&&terminalPos)?{x:terminalPos.x,z:terminalPos.z}:DM_Cabana.resupply,
    siloPoints:(typeof SILO!=='undefined'&&SILO.points)?SILO.points.map(p=>({x:p.x,z:p.z})):null, map:(typeof curMap!=='undefined')?curMap.name:null,
    enemySpawns:(typeof enemySpawns!=='undefined'?enemySpawns:[]).map(e=>{const p=e.position||e.pos||(e.group&&e.group.position)||e; return {x:+(+p.x).toFixed(1), z:+(+p.z).toFixed(1)};}),
    radii:{ silo:T.siloRadius, tech:T.techRadius||T.techBoxRadius, field:T.harvRadius||T.fieldRadius } };
  return { statics:st.length, keys, boxes:st.map(box).filter(Boolean), zones };
})()
