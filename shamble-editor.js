/* ============================================================================
 * shamble-editor.js — Shamble Frontlines map editor + Character Lab.  See MODULE_MAP.md.
 * PURPOSE:   Authoring tools. Loaded ONLY by editor.html, NEVER by the player
 *            build (index.html). No index.html live path may depend on this file.
 * PROVIDES:  editor gizmo / tools / library / inspector / panels (edX, incl.
 *            enterEditor / updateEditor / editorPlay); Character Lab (clX);
 *            analysis overlays (ovX / ovHideAll); orchard scatter tools
 *            (_orchardPreview + orchard ghost); prefabs; undo/redo; World and
 *            Match-Rules panel builders (edBuildWorld / edBuildRules);
 *            edLeaveToMenu() (editor-state reset that backToMenu calls through a
 *            typeof guard); and bootEditor() (consolidates editor-boot side-
 *            effects incl. the menu Editor-button wiring; editor.html calls it
 *            after bootGame()).
 * REQUIRES:  shamble-game.js + shamble-core.js (may reference both freely).
 * LOAD ORDER: THREE -> core -> game -> shamble-editor.js.
 * SIDE-EFFECTS ON LOAD: none — declarations only; the page calls bootEditor().
 * ========================================================================== */
let edOrchardGhost=null,_orchDotGeo=null,_orchDotMat=null,_orchPostGeo=null,_orchPostMat=null;
function _orchardCells(x0,z0,x1,z1){ const W=x1-x0,D=z1-z0,out=[]; if(!(W>=2&&D>=2))return out;
  const sp=4.5,nx=Math.min(8,Math.max(1,Math.round(W/sp))),nz=Math.min(8,Math.max(1,Math.round(D/sp)));
  const base=((Math.round(x0)&2047)*31+(Math.round(z0)&2047))>>>0;
  for(let ix=0;ix<=nx;ix++)for(let iz=0;iz<=nz;iz++){ const sd=((ix*73856093)^(iz*19349663)^(base*40503))>>>0;
    const hx=Math.sin(sd*12.9898)*43758.5453,hz=Math.sin(sd*78.233)*43758.5453;
    out.push({x:x0+W*ix/nx+((hx-Math.floor(hx))-0.5)*sp*0.42, z:z0+D*iz/nz+((hz-Math.floor(hz))-0.5)*sp*0.42, sd}); }
  return out; }
function _orchardGhostObj(){ if(edOrchardGhost)return edOrchardGhost;
  _orchDotGeo=new THREE.CircleGeometry(1.5,14); _orchDotMat=new THREE.MeshBasicMaterial({ color:0x3f7f2a, transparent:true, opacity:0.5, depthWrite:false });
  _orchPostGeo=new THREE.CylinderGeometry(0.16,0.16,1.4,6); _orchPostMat=new THREE.MeshBasicMaterial({ color:0x214d16 });
  const grp=new THREE.Group(); grp.visible=false; grp.userData.dots=[]; grp.userData.posts=[];
  for(let i=0;i<4;i++){ const q=new THREE.Mesh(_orchPostGeo,_orchPostMat); q.visible=false; grp.add(q); grp.userData.posts.push(q); }
  scene.add(grp); edOrchardGhost=grp; return grp; }
function _vegShow(cells, markers){ const grp=_orchardGhostObj(); grp.visible=true;
  const dots=grp.userData.dots;
  while(dots.length<cells.length){ const d=new THREE.Mesh(_orchDotGeo,_orchDotMat); d.rotation.x=-Math.PI/2; grp.add(d); dots.push(d); }
  for(let i=0;i<dots.length;i++){ const on=i<cells.length; dots[i].visible=on; if(on)dots[i].position.set(cells[i].x,0.08,cells[i].z); }
  const posts=grp.userData.posts; for(let i=0;i<posts.length;i++){ const on=i<markers.length; posts[i].visible=on; if(on)posts[i].position.set(markers[i][0],0.7,markers[i][1]); }
  return cells.length; }
function _orchardPreview(x0,z0,x1,z1){ return _vegShow(_orchardCells(x0,z0,x1,z1), [[x0,z0],[x1,z0],[x1,z1],[x0,z1]]); }
function _rowCells(sx,sz,ex,ez){ const dx=ex-sx,dz=ez-sz,L=Math.hypot(dx,dz),out=[]; if(L<3)return out;
  const sp=3.0,n=Math.min(14,Math.max(1,Math.round(L/sp))),nx=-dz/L,nz=dx/L;                           // n+1 poplars along the line; nx/nz = perpendicular for slight organic offset
  const base=((Math.round(sx)&2047)*31+(Math.round(sz)&2047))>>>0;
  for(let i=0;i<=n;i++){ const t=i/n, sd=((i*73856093)^(base*40503))>>>0; const hx=Math.sin(sd*12.9898)*43758.5453; const off=((hx-Math.floor(hx))-0.5)*0.8;
    out.push({x:sx+dx*t+nx*off, z:sz+dz*t+nz*off, sd}); }
  return out; }

// ===== Enemy spawn markers (per faction) — placeable + serialized; drive enemy spawn positions & factions =====
// Editor-only visuals (MeshBasic -> skipped by stylizeScene; layer hidden outside the editor). Raycast-enabled so the
// editor can select them, but never in the gameplay aim/LOS/collision sets, so they can't interfere with play.
const edRoot=document.getElementById('editor'), edCursorEl=document.getElementById('edCursor');
const edPalette=document.getElementById('edPalette'), edMarqueeEl=document.getElementById('edMarquee'), edPvBanner=document.getElementById('edPvBanner');
let edTool='select', edDragging=false, edGhost=null;
let _edRecent=[];
let edMulti=[], edSel=null, edGrab=null; const edSelBoxes=[];           // selection can hold many; edSel = primary
const EDSNAPS=[0,0.5,1,2,5]; let edSnapIdx=0, edSnap=EDSNAPS[0];      // grid sizes in metres (0 = Off / continuous)
const EDROTS=[15,45,90]; let edRotIdx=1, edRotStep=EDROTS[1];        // rotation step (deg) for Shift+Wheel
let edMarquee=null;                                         // {x0,y0} screen px while box-selecting
let edPathStart=null;                                       // ground point while drawing a road
let edPreview=false;                                        // Tab playtest
const _pv1=new THREE.Vector3(), _pv2=new THREE.Vector3(), _pvAim=new THREE.Vector3();

function edFootprint(t){ switch(t){
  case 'crateS':return{w:2,h:1.5,d:2}; case 'crateM':return{w:3,h:2.5,d:3}; case 'crateL':return{w:4,h:3.5,d:4};
  case 'hqF': case 'hqH':return{w:8,h:9,d:8}; case 'refF': case 'refH':return{w:10,h:4,d:6}; case 'barF': case 'barH':return{w:7,h:5,d:7};
  case 'barracks':return{w:11,h:5,d:11}; case 'platform':return{w:10,h:2.5,d:10}; case 'ramp':return{w:12,h:2.5,d:8}; case 'spawn': case 'resupply':return{w:1.6,h:1.6,d:1.6}; case 'treeApple':return{w:2.0,h:4.6,d:2.0}; case 'treePoplar':return{w:1.6,h:9.7,d:1.6}; case 'treeOak':return{w:2.8,h:6.8,d:2.8}; case 'propHaystack':return{w:3.2,h:2.4,d:3.2}; case 'propHaybales':return{w:2.8,h:1.5,d:2.6}; case 'propCart':return{w:3.4,h:1.3,d:1.6}; case 'propWell':return{w:2.8,h:2.4,d:2.0}; case 'propLogpile':return{w:2.2,h:1.1,d:2.0}; case 'propBarrels':return{w:2.4,h:1.0,d:2.4}; case 'propTrough':return{w:2.6,h:0.7,d:1.0}; case 'propVegpatch':return{w:3.0,h:0.4,d:2.0}; case 'manor':return{w:13,h:9,d:7}; default:return null; } }
let edCamX=0, edCamZ=0, edCamH=90;
let edPanning=false, _edPanPX=0, _edPanPY=0;
const edTargets=()=>{ const a=obstacles.map(o=>o.mesh).concat(buildings.map(b=>b.mesh)); paths.forEach(o=>a.push(o.mesh)); platforms.forEach(o=>a.push(o.mesh)); ramps.forEach(o=>a.push(o.mesh)); houses.forEach(o=>a.push(o.grp)); covers.forEach(o=>a.push(o.grp)); trees.forEach(o=>a.push(o.grp)); props.forEach(o=>a.push(o.grp)); enemySpawns.forEach(o=>a.push(o.grp)); if(termMesh)a.push(termMesh); a.push(player); return a; };
function findEditSel(obj){ let o=obj; while(o){ if(o.userData&&o.userData.editSel) return o.userData.editSel; o=o.parent; } return null; }
function edSelTarget(sel){ if(!sel)return null; if(sel.kind==='crate'||sel.kind==='wall'||sel.kind==='building'||sel.kind==='path'||sel.kind==='platform'||sel.kind==='ramp')return sel.ref.mesh; if(sel.kind==='barracks')return sel.ref.roof; if(sel.kind==='house')return sel.ref.grp; if(sel.kind==='cover')return sel.ref.grp; if(sel.kind==='tree')return sel.ref.grp; if(sel.kind==='prop')return sel.ref.grp; if(sel.kind==='enemyspawn')return sel.ref.grp; if(sel.kind==='spawn')return player; if(sel.kind==='resupply')return termMesh; return null; }
function edAnchor(sel){ if(!sel)return null; if(sel.kind==='barracks')return{x:sel.ref.cx,z:sel.ref.cz}; if(sel.kind==='spawn')return{x:mapSpawn.x,z:mapSpawn.z}; if(sel.kind==='resupply')return{x:terminalPos.x,z:terminalPos.z}; const m=edSelTarget(sel); return m?{x:m.position.x,z:m.position.z}:null; }
function _selEq(a,b){ return a.ref?(a.ref===b.ref):(a.kind===b.kind); }
function edClearBoxes(){ for(const b of edSelBoxes){ scene.remove(b); if(b.geometry)b.geometry.dispose(); } edSelBoxes.length=0; }
function edSetSel(arr){ edClearBoxes(); edMulti=(arr||[]).filter(s=>s); edSel=edMulti[0]||null;
  const col=edMulti.length>1?0x6ad0ff:0xffd23f;
  for(const s of edMulti){ const m=edSelTarget(s); if(m){ const bx=new THREE.BoxHelper(m,col); noRay(bx); scene.add(bx); edSelBoxes.push(bx); } } edUpdateHandles(); edUpdateGizmo(); edUpdateRing(); edSyncInspector(); }
function edRefreshBoxes(){ for(let i=0;i<edSelBoxes.length;i++){ const m=edSelTarget(edMulti[i]); if(m)edSelBoxes[i].setFromObject(m); } edUpdateHandles(); edUpdateGizmo(); edUpdateRing(); }
let edGizmoMode='move', edRotDrag=false, _edRotMoved=false, _edRotPrev=null, _edRing=null;
let _edHandles=null; const _edHBox=new THREE.Box3();
function _edEnsureHandles(){ if(_edHandles)return _edHandles; _edHandles=new THREE.Group(); const mat=new THREE.MeshBasicMaterial({color:0xffe27a,depthTest:false,transparent:true,opacity:0.95}); for(let i=0;i<8;i++){ const c=new THREE.Mesh(new THREE.BoxGeometry(1,1,1),mat); c.renderOrder=998; c.userData.edHandle=true; _edHandles.add(c); } _edHandles.visible=false; scene.add(_edHandles); return _edHandles; }
function edUpdateHandles(){ const h=_edEnsureHandles(); const m=(edGizmoMode==='scale'&&edMulti.length===1&&edSel)?edSelTarget(edSel):null; if(!m){ h.visible=false; return; } _edHBox.setFromObject(m); if(_edHBox.isEmpty()){ h.visible=false; return; } h.visible=true; const mn=_edHBox.min, mx=_edHBox.max, sz=Math.max(0.45,Math.min(1.4,Math.min(mx.x-mn.x,mx.z-mn.z)*0.12)); let i=0; for(let xi=0;xi<2;xi++)for(let yi=0;yi<2;yi++)for(let zi=0;zi<2;zi++){ const c=h.children[i++]; c.position.set(xi?mx.x:mn.x, yi?mx.y:mn.y, zi?mx.z:mn.z); c.scale.setScalar(sz); } h.updateMatrixWorld(); }

/* ---- transform gizmo (UE-style 3-axis translate) + functional resize handles ---- */
let _edGizmo=null, edAxisDrag=null, edResizeDrag=false, _edGizmoMoved=false, _edResizeMoved=false, _edAxisPrev=0, _edRzS0=1, _edRzStartDist=1;
const _UP=new THREE.Vector3(0,1,0), _edGZC=new THREE.Vector3(), _edPlane=new THREE.Plane();
function _edEnsureGizmo(){ if(_edGizmo)return _edGizmo; _edGizmo=new THREE.Group(); const AX=[['x',0xff5a5a],['y',0x5ae06a],['z',0x5aa8ff]], L=6, R=0.3;
  AX.forEach(a=>{ const axis=a[0], col=a[1], g=new THREE.Group(); g.userData.gizmoAxis=axis;
    const mat=new THREE.MeshBasicMaterial({color:col,depthTest:false,transparent:true,opacity:0.96});
    const shaft=new THREE.Mesh(new THREE.CylinderGeometry(R*0.3,R*0.3,L,8),mat); shaft.position.y=L*0.5;
    const head=new THREE.Mesh(new THREE.ConeGeometry(R,R*2.4,12),mat); head.position.y=L+R*1.2;
    g.add(shaft); g.add(head); if(axis==='x')g.rotation.z=-Math.PI/2; else if(axis==='z')g.rotation.x=Math.PI/2;
    g.traverse(o=>{o.renderOrder=999;}); _edGizmo.add(g); });
  _edGizmo.visible=false; scene.add(_edGizmo); return _edGizmo;
}
function edUpdateGizmo(){ const gz=_edEnsureGizmo(); const m=(edGizmoMode==='move'&&edMulti.length===1&&edSel)?edSelTarget(edSel):null; if(!m){ gz.visible=false; return; } _edHBox.setFromObject(m); if(_edHBox.isEmpty()){ gz.visible=false; return; } gz.visible=true; _edHBox.getCenter(_edGZC); gz.position.copy(_edGZC); const span=Math.max(_edHBox.max.x-_edHBox.min.x,_edHBox.max.z-_edHBox.min.z); gz.scale.setScalar(Math.max(0.55,Math.min(2.2,span*0.14))); gz.updateMatrixWorld(); }
function _edAxisRef(axis){ const m=edSel?edSelTarget(edSel):null; if(!m)return null; _edHBox.setFromObject(m); _edHBox.getCenter(_edGZC); raycaster.setFromCamera(mouseNDC,edCam());
  if(axis==='y'){ _v.set(edCam().position.x-_edGZC.x,0,edCam().position.z-_edGZC.z); if(_v.lengthSq()<1e-6)_v.set(0,0,1); _v.normalize(); _edPlane.setFromNormalAndCoplanarPoint(_v,_edGZC); return raycaster.ray.intersectPlane(_edPlane,_p1)?_p1.y:null; }
  _edPlane.set(_UP,-_edGZC.y); if(!raycaster.ray.intersectPlane(_edPlane,_p1))return null; return axis==='x'?_p1.x:_p1.z;
}
function edMoveYBy(sel,dy){ if(!dy)return; const cl=v=>Math.max(0,Math.min(60,v));
  if(sel.kind==='crate'||sel.kind==='wall'||sel.kind==='path'||sel.kind==='platform'||sel.kind==='ramp'){ const o=sel.ref; o.mesh.position.y=cl(o.mesh.position.y+dy); if(o.box)o.box.setFromObject(o.mesh); }
  else if(sel.kind==='building'){ const b=sel.ref; const rdy=cl(b.mesh.position.y+dy)-b.mesh.position.y; [b.mesh,b.trim,b.label,b.bar&&b.bar.grp].forEach(o=>{if(o)o.position.y+=rdy;}); b.box.setFromObject(b.mesh); }
  else if(sel.kind==='barracks'){ const b=sel.ref, r0=b.roof||(b.walls[0]&&b.walls[0].mesh); if(!r0)return; const rdy=cl(r0.position.y+dy)-r0.position.y; b.walls.forEach(w=>{w.mesh.position.y+=rdy;w.box.setFromObject(w.mesh);}); [b.lintel,b.floor,b.roof,b.label,b.panel,b.plabel].forEach(o=>{if(o)o.position.y+=rdy;}); }
  else if(sel.kind==='spawn'){ player.position.y=cl(player.position.y+dy); }
  else if(sel.kind==='resupply'){ if(termMesh)termMesh.position.y=cl(termMesh.position.y+dy); if(resupplyLabel)resupplyLabel.position.y+=dy; }
}
function edResizeResupply(w,h,d){ if(!termMesh)return; w=Math.max(0.4,w); h=Math.max(0.4,h); d=Math.max(0.4,d); edPushUndo('insp'); termMesh.scale.set(w/1.4,h/1.6,d); termMesh.position.y=h/2; edRefreshBoxes(); }
function _edResizeFootprint(sel,halfW,halfD,dist){ const w=Math.max(0.4,halfW*2), d=Math.max(0.4,halfD*2);
  if(sel.kind==='ramp'){ edReshapeRamp(sel,w,d,sel.ref.h0,sel.ref.h1); }
  else if(sel.kind==='crate'||sel.kind==='wall'||sel.kind==='platform'){ edResize(sel,w,sel.ref.h,d); }
  else if(sel.kind==='path'){ edResize(sel,w,0,d); }
  else if(sel.kind==='resupply'){ edResizeResupply(w,(termMesh?termMesh.scale.y*1.6:1.6),d); }
  else if(sel.kind==='building'||sel.kind==='barracks'){ edScaleSel(sel, Math.max(0.2,_edRzS0*dist/_edRzStartDist)); }
}
function _edGizmoPick(){ const gz=_edEnsureGizmo(); if(!gz.visible)return null; raycaster.setFromCamera(mouseNDC,edCam()); const hits=raycaster.intersectObject(gz,true); if(!hits.length)return null; let o=hits[0].object; while(o){ if(o.userData&&o.userData.gizmoAxis)return o.userData.gizmoAxis; o=o.parent; } return null; }
function _edEnsureRing(){ if(_edRing)return _edRing; _edRing=new THREE.Group();
  const mat=new THREE.MeshBasicMaterial({color:0xffe27a,depthTest:false,transparent:true,opacity:0.85});
  const torus=new THREE.Mesh(new THREE.TorusGeometry(5,0.16,10,64),mat); torus.rotation.x=Math.PI/2; torus.renderOrder=999; torus.userData.edRing=true;
  _edRing.add(torus); _edRing.visible=false; scene.add(_edRing); return _edRing; }
function edUpdateRing(){ const rg=_edEnsureRing(); const m=(edGizmoMode==='rotate'&&edMulti.length===1&&edSel)?edSelTarget(edSel):null; if(!m){ rg.visible=false; return; } _edHBox.setFromObject(m); if(_edHBox.isEmpty()){ rg.visible=false; return; } rg.visible=true; _edHBox.getCenter(_edGZC); rg.position.copy(_edGZC); const span=Math.max(_edHBox.max.x-_edHBox.min.x,_edHBox.max.z-_edHBox.min.z); rg.scale.setScalar(Math.max(0.5,Math.min(2.6,span*0.15))); rg.updateMatrixWorld(); }
function _edRingPick(){ const rg=_edEnsureRing(); if(!rg.visible)return false; raycaster.setFromCamera(mouseNDC,edCam()); return raycaster.intersectObject(rg,true).length>0; }
function _edGroundAngle(){ raycaster.setFromCamera(mouseNDC,edCam()); _edPlane.set(_UP,-_edGZC.y); if(!raycaster.ray.intersectPlane(_edPlane,_p1))return null; return Math.atan2(_p1.z-_edGZC.z,_p1.x-_edGZC.x); }
function _edStartResize(){ edResizeDrag=true; _edResizeMoved=false; const m=edSel?edSelTarget(edSel):null; if(m){ _edHBox.setFromObject(m); _edHBox.getCenter(_edGZC); _edRzS0=(edSel&&edSel.ref&&edSel.ref.scl)||1; raycaster.setFromCamera(mouseNDC,edCam()); _edPlane.set(_UP,-_edGZC.y); if(raycaster.ray.intersectPlane(_edPlane,_p1))_edRzStartDist=Math.max(0.5,Math.hypot(_p1.x-_edGZC.x,_p1.z-_edGZC.z)); } }
function _edStartRotate(){ edRotDrag=true; _edRotMoved=false; const m=edSel?edSelTarget(edSel):null; if(m){ _edHBox.setFromObject(m); _edHBox.getCenter(_edGZC); } _edRotPrev=_edGroundAngle(); }
function edSetGizmoModeKey(m){ edGizmoMode=m; if(typeof setEdTool==='function')setEdTool('select'); const rl=document.getElementById('edRail'); if(rl)rl.querySelectorAll('.edRailBtn[data-group="tool"]').forEach(b=>b.classList.toggle('on',b.dataset.mode===m)); if(typeof edUpdateGizmo==='function'){edUpdateGizmo();edUpdateHandles();edUpdateRing();} }
function _edHandlePick(){ const hd=_edEnsureHandles(); if(!hd.visible)return false; raycaster.setFromCamera(mouseNDC,edCam()); return raycaster.intersectObject(hd,true).length>0; }


// ---- move whole selection (delta-based; grabbed object snaps to grid, the rest follow) ----
function edMoveBy(sel,dx,dz){
  if(sel.kind==='crate'||sel.kind==='wall'){ const o=sel.ref; o.mesh.position.x+=dx; o.mesh.position.z+=dz; o.box.setFromObject(o.mesh); }
  else if(sel.kind==='path'){ const m=sel.ref.mesh; m.position.x+=dx; m.position.z+=dz; }
  else if(sel.kind==='platform'){ const o=sel.ref; o.mesh.position.x+=dx; o.mesh.position.z+=dz; o.box.setFromObject(o.mesh); }
  else if(sel.kind==='ramp'){ const o=sel.ref; o.mesh.position.x+=dx; o.mesh.position.z+=dz; o.x+=dx; o.z+=dz; o.box.setFromObject(o.mesh); }
  else if(sel.kind==='house'){ const o=sel.ref; o.grp.position.x+=dx; o.grp.position.z+=dz; o.x=o.grp.position.x; o.z=o.grp.position.z; o.grp.updateMatrixWorld(true); if(o.localColliders)_bkWorldBoxes(o); else o.box.setFromObject(o.body); }
  else if(sel.kind==='cover'){ const o=sel.ref; o.grp.position.x+=dx; o.grp.position.z+=dz; o.cx+=dx; o.cz+=dz; o.grp.updateMatrixWorld(true); _coverWorldBoxes(o); }
  else if(sel.kind==='tree'){ const o=sel.ref; o.grp.position.x+=dx; o.grp.position.z+=dz; o.x+=dx; o.z+=dz; _treeWorldBox(o); }
  else if(sel.kind==='prop'){ const o=sel.ref; o.grp.position.x+=dx; o.grp.position.z+=dz; o.x+=dx; o.z+=dz; _propWorldBox(o); }
  else if(sel.kind==='enemyspawn'){ const o=sel.ref; o.grp.position.x+=dx; o.grp.position.z+=dz; o.x=o.grp.position.x; o.z=o.grp.position.z; }
  else if(sel.kind==='building'){ const b=sel.ref; [b.mesh,b.trim,b.label,b.bar.grp].forEach(o=>{o.position.x+=dx;o.position.z+=dz;}); b.box.setFromObject(b.mesh); }
  else if(sel.kind==='barracks'){ const b=sel.ref; b.walls.forEach(w=>{w.mesh.position.x+=dx;w.mesh.position.z+=dz;w.box.setFromObject(w.mesh);}); [b.lintel,b.floor,b.roof,b.label,b.panel,b.plabel].forEach(o=>{if(o){o.position.x+=dx;o.position.z+=dz;}}); b.cx+=dx; b.cz+=dz; if(b.panelPos){b.panelPos.x+=dx;b.panelPos.z+=dz;} }
  else if(sel.kind==='spawn'){ mapSpawn.x+=dx; mapSpawn.z+=dz; player.position.x+=dx; player.position.z+=dz; }
  else if(sel.kind==='resupply'){ terminalPos.x+=dx; terminalPos.z+=dz; if(termMesh){termMesh.position.x+=dx;termMesh.position.z+=dz;} if(resupplyLabel){resupplyLabel.position.x+=dx;resupplyLabel.position.z+=dz;} }
}
function edDragMove(){ const g=edGround(); if(!g||!edSel)return; const a=edAnchor(edGrab||edSel); if(!a)return; const dx=g.x-a.x, dz=g.z-a.z; if(!dx&&!dz)return; for(const s of edMulti)edMoveBy(s,dx,dz); edRefreshBoxes(); }

// ---- rotate whole selection in strict 45° steps ----
function edRotateOne(sel,da){
  if(sel.kind==='crate'||sel.kind==='wall'){ const o=sel.ref; o.mesh.rotation.y+=da; o.box.setFromObject(o.mesh); }
  else if(sel.kind==='path'){ sel.ref.mesh.rotation.y+=da; }
  else if(sel.kind==='ramp'){ const o=sel.ref; o.mesh.rotation.y+=da; o.rot=(o.rot||0)+da; o.box.setFromObject(o.mesh); }
  else if(sel.kind==='house'){ const o=sel.ref; o.grp.rotation.y+=da; o.rot=(o.rot||0)+da; o.grp.updateMatrixWorld(true); if(o.localColliders)_bkWorldBoxes(o); else o.box.setFromObject(o.body); }
  else if(sel.kind==='cover'){ const o=sel.ref; o.grp.rotation.y+=da; o.rot=(o.rot||0)+da; o.grp.updateMatrixWorld(true); _coverWorldBoxes(o); }
  else if(sel.kind==='prop'){ const o=sel.ref; o.grp.rotation.y+=da; o.rot=(o.rot||0)+da; _propWorldBox(o); }
  else if(sel.kind==='building'){ const b=sel.ref, cx=b.mesh.position.x, cz=b.mesh.position.z; [b.mesh,b.trim].forEach(o=>{if(o)_rotAround(o,cx,cz,da);}); b.box.setFromObject(b.mesh); }
  else if(sel.kind==='barracks'){ _rotateBarracksBy(sel.ref,da); }
  else if(sel.kind==='resupply'){ if(termMesh)termMesh.rotation.y+=da; }
}
function edRotateSel(dir,fine){ if(!edMulti.length)return; edPushUndo('rot'); const da=dir*(fine?5:edRotStep)*Math.PI/180; for(const s of edMulti)edRotateOne(s,da); edRefreshBoxes(); edSyncInspector(); }
// --- composite-structure transforms (buildings + enterable barracks): uniform scale (layout-safe) + complete rotation ---
function edScaleSel(sel,target){ target=Math.max(0.3,Math.min(4,target)); edPushUndo('insp'); if(sel.kind==='building')_scaleBuilding(sel.ref,target); else if(sel.kind==='barracks')_scaleBarracks(sel.ref,target); edRefreshBoxes(); rebuildLOS(); }
// ---- Mirror (Tier 2): visible Off/X/Z toggle; live-mirrors placements + M mirrors the selection, swapping friend/hostile ----
let edMirror=null;
function edCycleMirror(){ edMirror=(edMirror===null)?'X':(edMirror==='X')?'Z':null; edUpdateMirrorBtn(); }
function edUpdateMirrorBtn(){ const b=document.getElementById('edMirrorBtn'); if(b){ {const l=b.querySelector('.edBtnLab'); if(l)l.textContent=edMirror?('Mirror '+edMirror):'Mirror Off';} b.classList.toggle('on',!!edMirror); } }
function _mirrorDesc(d){ const m=Object.assign({},d); if(edMirror==='X') m.ax=-d.ax; else m.az=-d.az; m.rot=-(d.rot||0); if(m.team==='friend')m.team='hostile'; else if(m.team==='hostile')m.team='friend'; return m; }
function edMirrorOne(sel){ if(!edMirror)return null; const d=edDescribe(sel); if(!d)return null; const m=_mirrorDesc(d); return edSpawnDesc(m, m.ax, m.az); }
function edMirrorSel(){ if(!edMirror||!edMulti.length)return; edPushUndo(); const clones=[]; for(const s of edMulti){ const c=edMirrorOne(s); if(c)clones.push(c); } rebuildLOS(); if(clones.length)edSetSel(clones); }
function edArraySel(n,gap,axis){ if(!edMulti.length||!isFinite(gap))return; n=Math.max(2,Math.min(50,Math.round(n)));
  const origs=edMulti.slice(); const descs=origs.map(edDescribe).filter(Boolean); if(!descs.length)return; edPushUndo();
  let dx=0,dz=0; if(axis==='+X')dx=gap; else if(axis==='-X')dx=-gap; else if(axis==='+Z')dz=gap; else dz=-gap;
  const clones=[]; for(let i=1;i<n;i++){ for(const d of descs){ const sel=edSpawnDesc(d, d.ax+dx*i, d.az+dz*i); if(sel)clones.push(sel); } }
  rebuildLOS(); if(clones.length)edSetSel(origs.concat(clones));
}



// ---- marquee box-pick (screen-space) ----
function edMarqueePick(x0,y0,x1,y1){ const ax=Math.min(x0,x1),bx=Math.max(x0,x1),ay=Math.min(y0,y1),by=Math.max(y0,y1); const out=[], seen=new Set();
  for(const m of edTargets()){ const sel=findEditSel(m); if(!sel||sel.kind==='spawn'||sel.kind==='resupply')continue; if(seen.has(sel.ref))continue;
    const a=edAnchor(sel); if(!a)continue; _pv1.set(a.x,1,a.z).project(edCam()); const sx=(_pv1.x*0.5+0.5)*innerWidth, sy=(-_pv1.y*0.5+0.5)*innerHeight;
    if(sx>=ax&&sx<=bx&&sy>=ay&&sy<=by){ seen.add(sel.ref); out.push(sel); } }
  return out; }

// ---- duplicate (Ctrl+D): clone the whole cluster, offset it, select the copies ----
function edDescribe(s){
  if(s.kind==='crate'||s.kind==='wall'){ const o=s.ref; return {t:'box', w:o.w,h:o.h,d:o.d, warm:o.warm, crate:o.crate, rot:o.mesh.rotation.y, ax:o.mesh.position.x, az:o.mesh.position.z}; }
  if(s.kind==='path'){ const o=s.ref; return {t:'path', w:o.w,d:o.d, rot:o.mesh.rotation.y, ax:o.mesh.position.x, az:o.mesh.position.z}; }
  if(s.kind==='cover'){ const o=s.ref; return {t:'cover', ctype:o.type, len:o.len, rot:o.grp.rotation.y, ax:o.grp.position.x, az:o.grp.position.z}; }
  if(s.kind==='tree'){ const o=s.ref; return {t:'tree', kind:o.kind, seed:o.seed, ax:o.grp.position.x, az:o.grp.position.z}; }
  if(s.kind==='prop'){ const o=s.ref; return {t:'prop', kind:o.kind, seed:o.seed, rot:o.grp.rotation.y, ax:o.grp.position.x, az:o.grp.position.z}; }
  if(s.kind==='building'){ const b=s.ref; return {t:'building', team:b.team, type:b.type, rot:b.mesh.rotation.y, scl:b.scl||1, ax:b.mesh.position.x, az:b.mesh.position.z, hp:b.maxHealth}; }
  if(s.kind==='barracks'){ const b=s.ref; return {t:'barracks', team:b.team, rot:b.rot||0, scl:b.scl||1, ax:b.cx, az:b.cz}; }
  if(s.kind==='platform'){ const o=s.ref; return {t:'platform', w:o.w,h:o.h,d:o.d, ax:o.mesh.position.x, az:o.mesh.position.z}; }
  if(s.kind==='ramp'){ const o=s.ref; return {t:'ramp', w:o.w,d:o.d,h0:o.h0,h1:o.h1,rot:o.rot||0, ax:o.x, az:o.z}; }
  if(s.kind==='house'){ const o=s.ref; return {t:'house', bt:o.type, rot:o.grp.rotation.y, ax:o.grp.position.x, az:o.grp.position.z}; }
  if(s.kind==='enemyspawn'){ const o=s.ref; return {t:'enemyspawn', faction:o.faction, ax:o.x, az:o.z}; }
  return null;
}
function edSpawnDesc(d,x,z){
  if(d.t==='box'){ const n=addBox(x,z,d.w,d.h,d.d, d.warm?crateMatWarm:crateMat, d.crate); n.mesh.rotation.y=d.rot; n.box.setFromObject(n.mesh); return n.mesh.userData.editSel; }
  if(d.t==='path'){ const n=addPath(x,z,d.w,d.d,d.rot); return n.mesh.userData.editSel; }
  if(d.t==='cover'){ const n=addCover(d.ctype,x,z,d.len,d.rot); return n.grp.userData.editSel; }
  if(d.t==='tree'){ const n=addTree(d.kind,x,z,d.seed); return n.grp.userData.editSel; }
  if(d.t==='prop'){ const n=addProp(d.kind,x,z,d.seed,d.rot); return n.grp.userData.editSel; }
  if(d.t==='building'){ const n=makeBuilding(d.team,d.type,x,z); if(d.hp){ n.maxHealth=d.hp; n.health=d.hp; } if(d.scl&&d.scl!==1)_scaleBuilding(n,d.scl); n.mesh.rotation.y=d.rot||0; if(n.trim)n.trim.rotation.y=d.rot||0; n.box.setFromObject(n.mesh); return n.mesh.userData.editSel; }
  if(d.t==='barracks'){ const n=makeBarracks(x,z,d.team); if(d.scl&&d.scl!==1)_scaleBarracks(n,d.scl); if(d.rot)_rotateBarracksBy(n,d.rot); return {kind:'barracks',ref:n}; }
  if(d.t==='platform'){ const n=makePlatform(x,z,d.w,d.h,d.d); return n.mesh.userData.editSel; }
  if(d.t==='ramp'){ const n=makeRamp(x,z,d.w,d.d,d.h0,d.h1,d.rot||0); return n.mesh.userData.editSel; }
  if(d.t==='house'){ const n=(d.spec?spawnKitBuilding(d.spec,x,z,d.rot||0):placeLegacyHouse((({thatchCottage:makeThatchCottage,brickFarm:makeBrickFlintFarmhouse,manor:makeManorHouse})[d.bt]||makeTimberHouse)(x,z,d.rot||0), d.bt==='manor')); return n.grp.userData.editSel; }
  if(d.t==='enemyspawn'){ const n=makeEnemySpawn(x,z,d.faction); return n.grp.userData.editSel; }
  return null;
}
function edDuplicate(){ if(!edMulti.length)return; edPushUndo(); const OFF=4, clones=[];
  for(const s of edMulti){ const d=edDescribe(s); if(!d)continue; const sel=edSpawnDesc(d, d.ax+OFF, d.az+OFF); if(sel)clones.push(sel); }
  rebuildLOS(); if(clones.length)edSetSel(clones);
}
// ---- copy / paste (Ctrl+C / Ctrl+V): clipboard keeps a centroid-relative layout; paste lands at the cursor ----
const edClipboard=[];
function edCopy(){ const items=edMulti.map(edDescribe).filter(Boolean); if(!items.length)return; let cx=0,cz=0; for(const d of items){ cx+=d.ax; cz+=d.az; } cx/=items.length; cz/=items.length; edClipboard.length=0; for(const d of items){ const c=Object.assign({},d); c.dx=d.ax-cx; c.dz=d.az-cz; edClipboard.push(c); } }
function edPaste(){ if(!edClipboard.length)return; edPushUndo(); const t=edGround()||{x:edCamX,z:edCamZ}; const clones=[]; for(const d of edClipboard){ const sel=edSpawnDesc(d, t.x+d.dx, t.z+d.dz); if(sel)clones.push(sel); } rebuildLOS(); if(clones.length)edSetSel(clones); }
// ---- arrow-key nudge of the selection (snap-aware; Shift = 0.25m fine) ----
function edNudge(code,fine){ if(!edMulti.length)return; const base=fine?0.25:(edSnap>0?edSnap:1); let dx=0,dz=0; if(code==='ArrowUp')dz=-base; else if(code==='ArrowDown')dz=base; else if(code==='ArrowLeft')dx=-base; else dx=base; edPushUndo('nudge'); for(const s of edMulti)edMoveBy(s,dx,dz); edRefreshBoxes(); edSyncInspector(); }

function enterEditor(){
  mode='editor'; started=false; paused=false;
  occRestoreAll();   // un-ghost anything the in-game cutaway left faded, so the editor shows solid geometry
  titleEl.style.display='none'; titleEl.style.opacity='0';
  pauseMenuEl.style.display='none'; endEl.style.display='none';
  hudEl.style.display='none'; reticleEl.style.display='none';
  edRoot.style.display='block';
  renderer.domElement.style.cursor='crosshair';
  enemies.forEach(en=>{en.group.visible=false;});
  player.visible=true; player.position.copy(mapSpawn); player.rotation.set(0,0,0); player.userData.editSel={kind:'spawn'};
  edCamX=0; edCamZ=0; edCamH=90; edPreview=false; edPvBanner.style.display='none'; edPalette.style.opacity='';
  if(!edGhost){ edGhost=new THREE.LineSegments(new THREE.EdgesGeometry(new THREE.BoxGeometry(1,1,1)),new THREE.LineBasicMaterial({color:0xffd23f,transparent:true,opacity:0.9})); noRay(edGhost); edGhost.visible=false; scene.add(edGhost); }
  document.getElementById('edSize').textContent=(T.arenaX*2)+' × '+(T.arenaZ*2);
  edSetSel([]); edGhost.visible=false;
  setEdTool('select'); edUpdateSnapBtn(); ovShow(); ovResetAll(); edUndoStack.length=0; edRedoStack.length=0; _edLastTag=null; edUpdateUndoBtns(); edSyncWorldPanel(); refreshPrefabList();
}
let charLabDef=null, charLabPreview=null, charLabSex='m', charLabFaction='Soviets';
let charLabSkin=0xd9a077, charLabHair=0x3a2a1c, charLabEyes=0x243038;
let charLabSlots={headgear:'none',torso:'none',legs:'none',feet:'none',webbing:'none',hair:'default',face:'none'};
// ---- Rifle-hold pose authoring tool (Character Lab) — author carry + ADS holds by eye; export feeds ANIM.aim ----
let poseLabActive=false, poseSlot='ads';
const poseLab={
  carry:{ gun:[0.13,-0.13,0.70], rot:[0.72,-0.44,0.58], gripL:[-0.20,-0.14,0.30], gripR:[0.17,-0.04,0.03], poleL:[-0.60,-0.85,-0.30], poleR:[0.60,-0.85,-0.30] },
  ads:{   gun:[0.33,0.16,0.70],  rot:[-0.04,0.00,0.10], gripL:[0.06,-0.20,0.44], gripR:[0.18,-0.14,-0.07], poleL:[-0.60,-0.85,-0.30], poleR:[0.60,-0.85,-0.30] }
};
const POSE_CTRL=[
  ['Gun X','gun',0,-0.5,0.5,0.01],['Gun Y','gun',1,-0.5,0.4,0.01],['Gun Z','gun',2,-0.2,1.2,0.01],
  ['Gun pitch','rot',0,-1.3,1.3,0.02],['Gun yaw','rot',1,-1.3,1.3,0.02],['Gun roll','rot',2,-1.3,1.3,0.02],
  ['L hand X','gripL',0,-0.4,0.4,0.01],['L hand Y','gripL',1,-0.4,0.4,0.01],['L hand Z','gripL',2,-0.3,0.6,0.01],
  ['R hand X','gripR',0,-0.4,0.4,0.01],['R hand Y','gripR',1,-0.4,0.4,0.01],['R hand Z','gripR',2,-0.4,0.4,0.01],
  ['L elbow X','poleL',0,-1.5,1.5,0.05],['L elbow Y','poleL',1,-1.5,1.5,0.05],['L elbow Z','poleL',2,-1.5,1.5,0.05],
  ['R elbow X','poleR',0,-1.5,1.5,0.05],['R elbow Y','poleR',1,-1.5,1.5,0.05],['R elbow Z','poleR',2,-1.5,1.5,0.05],
];
let charLabBase=0x2f6f63, charLabAccent=0x38e1b0;
let clSpin=true, clDrag=false, clAz=0, clEl=0.12, clDist=5.6; const _clLast={x:0,y:0}; let _clHidden=[];
const CL_LOOK=new THREE.Vector3(0,1.0,0);
function clClone(d){ return JSON.parse(JSON.stringify(d)); }
function clP(id){ return charLabDef.parts.find(p=>p.id===id); }
const CL_CTRL=[
  {k:'Head size',  min:0.26,max:0.50,step:0.005, get:()=>clP('head').r,     set:v=>{ clP('head').r=v; }},
  {k:'Head width', min:0.80,max:1.35,step:0.01,  get:()=>(clP('head').scale?clP('head').scale[0]:1), set:v=>{ clP('head').scale=[v,1,v]; }},
  {k:'Body girth', min:0.30,max:0.50,step:0.005, get:()=>clP('torso').rt,   set:v=>{ const t=clP('torso'),s=clP('shoulders'),d=v-t.rt; t.rt=v; t.rb=Math.max(0.12,t.rb+d); s.rt=Math.max(0.14,s.rt+d); s.rb=Math.max(0.14,s.rb+d); }},
  {k:'Shoulders',  min:0.40,max:0.80,step:0.005, get:()=>clP('shR').pos[0], set:v=>{ clP('shR').pos[0]=v; clP('shL').pos[0]=-v; }},
  {k:'Arm thick',  min:0.07,max:0.20,step:0.004, get:()=>clP('upR').rt,     set:v=>{ ['upR','upL'].forEach(id=>{const p=clP(id);p.rt=v;p.rb=Math.max(0.05,v-0.015);}); ['farmR','farmL'].forEach(id=>{const p=clP(id);p.rt=Math.max(0.05,v-0.015);p.rb=Math.max(0.04,v-0.03);}); }},
  {k:'Upper arm',  min:0.32,max:0.66,step:0.005, get:()=>clP('upR').h,      set:v=>{ ['upR','upL'].forEach(id=>{const p=clP(id);p.h=v;p.pos[1]=-v/2;}); ['elbowR','elbowL'].forEach(id=>{clP(id).pos[1]=-v;}); }},
  {k:'Forearm',    min:0.30,max:0.62,step:0.005, get:()=>clP('farmR').h,     set:v=>{ ['farmR','farmL'].forEach(id=>{const p=clP(id);p.h=v;p.pos[1]=-v/2;}); ['handR','handL'].forEach(id=>{clP(id).pos[1]=-v;}); }},
  {k:'Hand size',  min:0.07,max:0.18,step:0.004, get:()=>clP('handR').r,    set:v=>{ clP('handR').r=v; clP('handL').r=v; }},
  {k:'Hips',       min:0.26,max:0.48,step:0.005, get:()=>clP('hips').r,     set:v=>{ clP('hips').r=v; }},
  {k:'Leg thick',  min:0.10,max:0.26,step:0.004, get:()=>clP('thighR').rt,  set:v=>{ ['thighR','thighL'].forEach(id=>{const p=clP(id);p.rt=v;p.rb=Math.max(0.06,v-0.02);}); ['shinR','shinL'].forEach(id=>{const p=clP(id);p.rt=Math.max(0.06,v-0.03);p.rb=Math.max(0.05,v-0.06);}); }},
  {k:'Foot size',  min:0.70,max:1.60,step:0.02,  get:()=>(clP('footR').scale?clP('footR').scale[0]:1), set:v=>{ clP('footR').scale=[v,1,v]; clP('footL').scale=[v,1,v]; }},
  {k:'Helmet wide',min:0.36,max:0.62,step:0.005, get:()=>clP('brim').w,     set:v=>{ clP('brim').w=v; }},
  {k:'Helmet deep',min:0.20,max:0.50,step:0.005, get:()=>clP('brim').d,     set:v=>{ clP('brim').d=v; }},
];
function clDispose(g){ if(!g)return; scene.remove(g); g.traverse(o=>{ if(o.isMesh){ if(o.geometry&&o.geometry.dispose)o.geometry.dispose(); const m=o.material; if(m){ if(Array.isArray(m))m.forEach(x=>x&&x.dispose&&x.dispose()); else if(m.dispose)m.dispose(); } } }); }
function clAssembled(){ const def=clClone(charLabDef); const hide={}; if(charLabSlots.torso&&charLabSlots.torso!=='none')hide.band=1; if(charLabSlots.legs&&charLabSlots.legs!=='none')hide.shorts=1; if(charLabSlots.headgear&&charLabSlots.headgear!=='none'){hide.hair=1;hide.ponytail=1;} if(charLabSlots.hair&&charLabSlots.hair!=='default'){hide.hair=1;hide.ponytail=1;} if(Object.keys(hide).length)def.parts=def.parts.filter(p=>!hide[p.id]); for(const slot in CPARTS){ const id=charLabSlots[slot]; if(!id||id==='none')continue; if(slot==='hair'&&charLabSlots.headgear&&charLabSlots.headgear!=='none')continue; const fn=CPARTS[slot][id]; if(fn){ const ex=fn(charLabSex); if(ex&&ex.length)def.parts=def.parts.concat(ex); } } return def; }
function clCycleSlot(slot,ids){ const cur=charLabSlots[slot]||'none'; const i=ids.indexOf(cur); charLabSlots[slot]=ids[(i+1)%ids.length]; clBuildPanel(); clRebuild(); }
function clRebuild(){
  clDispose(charLabPreview);
  const _def=clAssembled(); if(poseLabActive||alOn){ _def.weapon=CHAR_DEFAULT.weapon; _def.armMode='rifle'; }
  const _fac=(charLabFaction==='EmpireElite')?'Empire':charLabFaction;
  charLabPreview=buildEntity(_def,{faction:_fac, skin:charLabSkin, hair:charLabHair, eyes:charLabEyes});
  charLabPreview.position.set(0,0,0); charLabPreview.rotation.y=0;
  stylizeOne(charLabPreview);
  applyFaction(charLabPreview, _fac);
  if(charLabFaction==='EmpireElite') applyVillain(charLabPreview);
  scene.add(charLabPreview);
  if(alOn) alPrepare(charLabPreview);
}
function clHideWorld(on){
  if(on){ _clHidden.length=0; scene.children.forEach(o=>{ if(o!==charLabPreview && !o.isLight && o.visible){ _clHidden.push(o); o.visible=false; } }); }
  else { _clHidden.forEach(o=>{ o.visible=true; }); _clHidden.length=0; }
}
function clCam(){ const ce=Math.cos(clEl); camera.position.set(CL_LOOK.x+clDist*Math.sin(clAz)*ce, CL_LOOK.y+clDist*Math.sin(clEl), CL_LOOK.z+clDist*Math.cos(clAz)*ce); camera.up.set(0,1,0); camera.lookAt(CL_LOOK); }
const clUndo=[];
function clUndoPush(fn){ clUndo.push(fn); if(clUndo.length>50)clUndo.shift(); }
function clUndoPop(){ if(clUndo.length){ (clUndo.pop())(); } }
function _clSyncPoseUI(){ const bt=document.getElementById('clPoseToggle'); if(bt){ bt.textContent='Pose Mode: '+(poseLabActive?'On':'Off'); bt.classList.toggle('on',poseLabActive); } ['clPoseSlots','clPoseExportRow'].forEach(id=>{ const el=document.getElementById(id); if(el)el.style.display=poseLabActive?'':'none'; }); const cb=document.getElementById('clPoseCarry'),ab=document.getElementById('clPoseAds'); if(cb)cb.classList.toggle('on',poseSlot==='carry'); if(ab)ab.classList.toggle('on',poseSlot==='ads'); }
function clBuildPosePanel(){ const host=document.getElementById('clPoseControls'); if(!host)return; host.innerHTML=''; if(!poseLabActive)return; const pose=poseLab[poseSlot];
  POSE_CTRL.forEach((c,i)=>{ const k=c[0],grp=c[1],idx=c[2],mn=c[3],mx=c[4],st=c[5]; const v=pose[grp][idx];
    const row=document.createElement('div'); row.className='dp-slider';
    row.innerHTML='<span class="dp-sl-l">'+k+'</span><input type="range" min="'+mn+'" max="'+mx+'" step="'+st+'"><span class="dp-sl-v" id="clPV'+i+'"></span>';
    host.appendChild(row);
    const inp=row.querySelector('input'), out=row.querySelector('.dp-sl-v'); inp.value=v; out.textContent=(+v).toFixed(2);
    const applyVal=(nv,hard)=>{ nv=Math.max(hard?-3.2:mn,Math.min(hard?3.2:mx,nv)); pose[grp][idx]=nv; inp.value=nv; out.textContent=(+nv).toFixed(2); if(charLabPreview)applyRifleHold(charLabPreview,pose); };
    const _snap=()=>{ const ov=pose[grp][idx],sl=poseSlot,g=grp,ix=idx; clUndoPush(()=>{ poseSlot=sl; poseLab[sl][g][ix]=ov; _clSyncPoseUI(); clBuildPosePanel(); if(charLabPreview)applyRifleHold(charLabPreview,poseLab[poseSlot]); }); };
    inp.addEventListener('pointerdown',_snap); out.addEventListener('focus',_snap);
    inp.oninput=()=>applyVal(parseFloat(inp.value));
    clEditable(out, nv=>{ if(nv==null){ out.textContent=(+pose[grp][idx]).toFixed(2); return; } applyVal(nv,true); });
  });
}
function clTogglePose(){ poseLabActive=!poseLabActive; _clSyncPoseUI(); clRebuild(); clBuildPosePanel(); }
function clPoseSlot(sl){ poseSlot=sl; _clSyncPoseUI(); clBuildPosePanel(); if(charLabPreview)applyRifleHold(charLabPreview,poseLab[poseSlot]); }
function clExportPose(){ const ta=document.getElementById('clJson'); if(!ta)return; const p=JSON.stringify(poseLab,null,1); ta.value=p; if(navigator.clipboard&&navigator.clipboard.writeText)navigator.clipboard.writeText(p).catch(()=>{}); }
function clTick(dt){ if(clSpin&&!clDrag)clAz+=dt*0.5; clCam(); if(alOn&&charLabPreview) alTick(dt); else if(poseLabActive&&charLabPreview)applyRifleHold(charLabPreview, poseLab[poseSlot]); }
// ============================ ANIM LAB (a Character Lab tab) ============================
// Loops any action on the lab soldier (a treadmill for movement: the ground slides under the planted feet) with live
// sliders over the ANIM knobs. The knobs are the game's own, so the editor's Tab playtest uses them too. "Copy values"
// exports the tuned knobs as JSON (to bake into ANIM); "Reset" restores the values the lab first opened with.
let alOn=false, alAction='run', alGun='amk', alStance='carry', alSpeed=1, alPaused=false, alT=0, alCycle=-1, _alSnap=null, _alUI=null;
const _alSt={ aimYaw:0, t:0, kick:0, dip:0, lean:0, grounded:true, ads:0, swap:0, rel:-1, throwC:-1 };
const AL_ACTIONS={ idle:{}, walk:{sp:2.5}, run:{sp:6.3}, sprint:{sp:9}, strafe:{sp:6.3,dir:Math.PI/2}, backpedal:{sp:6.3,dir:Math.PI},
  aim:{ads:1}, 'aim + strafe':{sp:3,ads:1,dir:Math.PI/2}, reload:{}, throw:{}, draw:{}, flinch:{}, 'death back':{}, 'death front':{},
  shield:{sp:2,mode:'shield'}, katana:{mode:'melee'}, bayonet:{sp:6.3,lean:1} };
const AL_KNOBS=[ ['Run crouch',()=>ANIM.drop[1],v=>{ANIM.drop[1]=v;},0,0.2,0.005], ['Run bob',()=>ANIM.pelvisBob[1],v=>{ANIM.pelvisBob[1]=v;},0,0.08,0.002],
  ['Run lift',()=>ANIM.lift[1],v=>{ANIM.lift[1]=v;},0.05,0.4,0.005], ['Stance width',()=>ANIM.stanceW,v=>{ANIM.stanceW=v;},0.6,1.3,0.01],
  ['Twist max',()=>ANIM.twistMax,v=>{ANIM.twistMax=v;},0.3,1.4,0.01], ['Toe roll',()=>ANIM.toePitch,v=>{ANIM.toePitch=v;},0,3,0.05],
  ['Coat swing',()=>ANIM.coat.front,v=>{ANIM.coat.front=v; ANIM.coat.back=v*0.8;},0,2,0.05], ['Sprint lean',()=>ANIM.sprintLean,v=>{ANIM.sprintLean=v;},0,0.6,0.01] ];
const AL_HOLD=[ ['Stock x',h=>h.at[0],(h,v)=>{h.at[0]=v;},-0.2,0.8], ['Stock y',h=>h.at[1],(h,v)=>{h.at[1]=v;},-0.8,0.4], ['Stock z',h=>h.at[2],(h,v)=>{h.at[2]=v;},-0.4,0.6],
  ['Pitch',h=>(h.rot[0]==='aim'?0:h.rot[0]),(h,v)=>{ if(h.rot[0]!=='aim') h.rot[0]=v; },-1.5,1.5], ['Yaw',h=>h.rot[1],(h,v)=>{h.rot[1]=v;},-1.5,1.5],
  ['Roll',h=>h.rot[2],(h,v)=>{h.rot[2]=v;},-1,1], ['Chest blade',h=>h.blade,(h,v)=>{h.blade=v;},-0.3,1.0] ];
function _alSubset(){ return { drop:ANIM.drop, pelvisBob:ANIM.pelvisBob, lift:ANIM.lift, stanceW:ANIM.stanceW, twistMax:ANIM.twistMax, toePitch:ANIM.toePitch, coat:ANIM.coat, sprintLean:ANIM.sprintLean, hold:ANIM.hold }; }
function alCopy(){ const txt=JSON.stringify(_alSubset()), ask=()=>prompt('Copy the tuned ANIM values:',txt);
  try{ navigator.clipboard.writeText(txt).then(()=>showLevelToast('Anim Lab','Tuned values copied'),ask); }catch(e){ ask(); } }
function alReset(){ if(!_alSnap) return; const o=JSON.parse(_alSnap); for(const k in o) ANIM[k]=o[k]; alRefresh(); }
function alEnsureUI(){ if(_alUI) return; const root=document.getElementById('charLab'); if(!root) return;
  _alSnap=JSON.stringify(_alSubset());
  const mk=(tag,css,txt)=>{ const e=document.createElement(tag); if(css) e.style.cssText=css; if(txt!=null) e.textContent=txt; return e; };
  const P=mk('div','position:fixed;left:12px;top:72px;width:300px;max-height:calc(100% - 96px);overflow:auto;background:rgba(18,20,28,0.93);color:#e8e8ee;font:12px/1.35 sans-serif;padding:10px 12px;border-radius:8px;z-index:60;display:none;box-shadow:0 4px 18px rgba(0,0,0,0.4)');
  const sec=mk('div',null,'Animation'); sec.className='dp-sec'; const row=mk('div'); row.className='dp-row'; const tb=mk('button',null,'Anim Lab: off'); tb.className='dp-btn';
  tb.onclick=()=>{ alOn=!alOn; tb.textContent='Anim Lab: '+(alOn?'on':'off'); P.style.display=alOn?'block':'none'; alT=0; alCycle=-1; clRebuild(); };
  row.appendChild(tb); root.appendChild(sec); root.appendChild(row);
  P.appendChild(mk('div','font-weight:bold;font-size:13px;margin-bottom:6px','Anim Lab'));
  const sel=(label,opts,cur,fn)=>{ const d=mk('div','margin:4px 0'); d.appendChild(mk('span','display:inline-block;width:66px;opacity:.8',label));
    const e=mk('select','width:212px;background:#2a2d3a;color:#eee;border:1px solid #444;border-radius:4px;padding:2px');
    for(const o of opts){ const op=mk('option',null,o); op.value=o; if(o===cur) op.selected=true; e.appendChild(op); } e.onchange=()=>fn(e.value); d.appendChild(e); P.appendChild(d); };
  sel('Action',Object.keys(AL_ACTIONS),alAction,v=>{ alAction=v; alT=0; alCycle=-1; clRebuild(); });
  sel('Gun',Object.keys(GUN_KIT),alGun,v=>{ alGun=v; alPrepare(charLabPreview); alRefresh(); });
  sel('Tune hold',['carry','ads','sprint'],alStance,v=>{ alStance=v; alRefresh(); });
  const pr=mk('div','display:flex;align-items:center;gap:6px;margin:6px 0'); pr.appendChild(mk('span','width:66px;opacity:.8','Speed'));
  const spd=mk('input','flex:1'); spd.type='range'; spd.min=0.1; spd.max=1.5; spd.step=0.05; spd.value=alSpeed; const sv=mk('span','width:34px;text-align:right','1.00');
  spd.oninput=()=>{ alSpeed=+spd.value; sv.textContent=alSpeed.toFixed(2); }; const pb=mk('button','padding:2px 8px','Pause'); pb.className='dp-btn';
  pb.onclick=()=>{ alPaused=!alPaused; pb.textContent=alPaused?'Play':'Pause'; }; pr.append(spd,sv,pb); P.appendChild(pr);
  const box=mk('div'); P.appendChild(box);
  const br=mk('div','display:flex;gap:6px;margin-top:8px'); const cb=mk('button',null,'Copy values'); cb.className='dp-btn'; cb.onclick=alCopy;
  const rb=mk('button',null,'Reset'); rb.className='dp-btn'; rb.onclick=alReset; br.append(cb,rb); P.appendChild(br);
  P.appendChild(mk('div','margin-top:6px;opacity:.6;font-size:11px','Values drive the game too: Tab-playtest in the Map Editor to feel them.'));
  root.appendChild(P); _alUI={P,box}; alRefresh(); }
function alRefresh(){ if(!_alUI) return; const box=_alUI.box; box.innerHTML='';
  const hd=t=>{ const h=document.createElement('div'); h.style.cssText='margin:8px 0 2px;font-weight:bold;opacity:.9'; h.textContent=t; box.appendChild(h); };
  const add=(label,get,set,min,max,step)=>{ const d=document.createElement('div'); d.style.cssText='display:flex;align-items:center;gap:6px;margin:3px 0';
    const l=document.createElement('span'); l.style.cssText='width:86px;opacity:.85'; l.textContent=label;
    const r=document.createElement('input'); r.type='range'; r.min=min; r.max=max; r.step=step||0.01; r.value=get(); r.style.flex='1';
    const val=document.createElement('span'); val.style.cssText='width:40px;text-align:right'; val.textContent=(+get()).toFixed(2);
    r.oninput=()=>{ set(+r.value); val.textContent=(+r.value).toFixed(2); }; d.append(l,r,val); box.appendChild(d); };
  hd('Locomotion'); for(const k of AL_KNOBS) add(k[0],k[1],k[2],k[3],k[4],k[5]);
  const style=(GUN_KIT[alGun]||GUN_KIT.amk).hold, H=(ANIM.hold[style]||ANIM.hold.rifle)[alStance];
  hd('Hold: '+style+' / '+alStance); for(const k of AL_HOLD) add(k[0],()=>k[1](H),v=>k[2](H,v),k[3],k[4],0.01); }
function alPrepare(g){ if(!g||!g.userData.joints) return; const u=g.userData, j=u.joints;
  if(!u.sword){ u.sword=makeSword(); j.handR.add(u.sword); u.sword.position.set(0,-0.05,0.05); stylizeOne(u.sword); }
  if(!u.shield){ u.shield=makeShield(0x38e1b0); j.handL.add(u.shield); u.shield.position.set(0,-0.12,0.05); u.shield.scale.set(0.55,0.55,0.55); stylizeOne(u.shield); }
  const m=(AL_ACTIONS[alAction]||{}).mode||'rifle';
  if(u.wpn){ setHeldGun(g,alGun); u.wpn.visible=(m!=='melee'); } u.sword.visible=(m==='melee'); u.shield.visible=(m==='shield'); poseArms(g,m); }
function alTick(dt){ const g=charLabPreview; if(!g||!g.userData.joints||!g.userData.wpn) return; const d=alPaused?0:dt*alSpeed; if(d<=0) return;
  const a=AL_ACTIONS[alAction]||AL_ACTIONS.idle, st=_alSt, fx=Math.sin(g.rotation.y)*5, fz=Math.cos(g.rotation.y)*5; alT+=d;
  st.aimYaw=g.rotation.y; st.t=alT; st.kick=0; st.dip=0; st.lean=a.lean||0; st.grounded=true; st.ads=a.ads||0; st.swap=0; st.rel=-1; st.throwC=-1;
  if(alAction==='reload'){ const w=WEAPONS.find(x=>x.id===alGun)||WEAPONS[0], L=w.reload||2, c=alT%(L+0.6); st.rel=c<L?c/L:-1; st.dip=c<L?1:0; }
  else if(alAction==='throw'){ const c=alT%1.8, k=Math.floor(alT/1.8); if(c<0.6) st.throwC=c; else if(alCycle!==k){ alCycle=k; animThrow(g); } }
  else if(alAction==='draw'){ st.swap=Math.max(0,1-(alT%1.3)/0.26); }
  else if(alAction==='flinch'){ const k=Math.floor(alT/1.1); if(alCycle!==k){ alCycle=k; animHit(g,g.position.x+fx,g.position.z+fz,1); } }
  else if(alAction==='katana'&&g.userData.sword){ const pz=swingPose(Math.min(1,(alT%1.3)/0.55)); g.userData.sword.rotation.set(pz.x,pz.y,0); }
  if(alAction==='death back'||alAction==='death front'){                   // stand 0.4 s, fall, lie until the loop restarts
    const c=alT%3.2; if(c<0.4){ updateSoldierAnim(g,st,d); return; }
    const A=g.userData.anim, sg=(alAction==='death back')?1:-1; if(A&&!A.death.active) deathStart(g,g.position.x+fx*sg,g.position.z+fz*sg); animDeath(g,d); return; }
  const sp=a.sp||0, dir=a.dir||0; g.position.x+=Math.sin(g.rotation.y+dir)*sp*d; g.position.z+=Math.cos(g.rotation.y+dir)*sp*d;
  g.userData.wpn.rotation.x=0; updateSoldierAnim(g,st,d);
  const A=g.userData.anim, ox=g.position.x, oz=g.position.z; g.position.x=0; g.position.z=0;   // treadmill: soldier stays put,
  if(A){ A.prevX-=ox; A.prevZ-=oz; for(let i=0;i<2;i++){ const F=A.feet[i]; F.p.x-=ox; F.p.z-=oz; F.cur.x-=ox; F.cur.z-=oz; } } }   // planted feet slide back
function clHex(n){ return '#'+('000000'+((n>>>0)&0xffffff).toString(16)).slice(-6); }
function clEditable(span, apply){
  span.contentEditable='true'; span.spellcheck=false; span.classList.add('cl-edit');
  span.addEventListener('keydown', e=>{ if(e.key==='Enter'){ e.preventDefault(); span.blur(); } });
  span.addEventListener('blur', ()=>{ const n=parseFloat(span.textContent); apply(isNaN(n)?null:n); });
}
function clBuildPanel(){
  const host=document.getElementById('clControls'); if(host){ host.innerHTML='';
    CL_CTRL.forEach((c,i)=>{ let v; try{ v=c.get(); }catch(e){ return; } if(v===undefined||v===null)return;
      const row=document.createElement('div'); row.className='dp-slider';
      row.innerHTML='<span class="dp-sl-l">'+c.k+'</span><input type="range" min="'+c.min+'" max="'+c.max+'" step="'+c.step+'"><span class="dp-sl-v" id="clV'+i+'"></span>';
      host.appendChild(row);
      const inp=row.querySelector('input'), out=row.querySelector('.dp-sl-v'); inp.value=v; out.textContent=(+v).toFixed(2);
      const applyVal=(nv,hard)=>{ nv=Math.max(hard?c.min*0.5:c.min,Math.min(hard?c.max*2:c.max,nv)); c.set(nv); inp.value=nv; out.textContent=(+nv).toFixed(2); clRebuild(); };
      const _snap=()=>{ const ov=c.get(); clUndoPush(()=>{ c.set(ov); clBuildPanel(); clRebuild(); }); };
      inp.addEventListener('pointerdown',_snap); out.addEventListener('focus',_snap);
      inp.oninput=()=>applyVal(parseFloat(inp.value));
      clEditable(out, nv=>{ if(nv==null){ out.textContent=(+c.get()).toFixed(2); return; } applyVal(nv,true); });
    });
  }
  const cwrap=document.getElementById('clColors'); if(cwrap){ cwrap.innerHTML='';
    [['Skin','skin'],['Hair','hair'],['Eyes','eyes']].forEach(pair=>{ const lab=pair[0],key=pair[1];
      const cur=key==='skin'?charLabSkin:key==='hair'?charLabHair:charLabEyes;
      const row=document.createElement('div'); row.className='cl-color';
      row.innerHTML='<span>'+lab+'</span><input type="color" value="'+clHex(cur)+'">';
      cwrap.appendChild(row);
      row.querySelector('input').oninput=e=>{ const val=parseInt(e.target.value.slice(1),16); if(key==='skin')charLabSkin=val; else if(key==='hair')charLabHair=val; else charLabEyes=val; clRebuild(); };
    });
  }
  const _setOn=(id,on)=>{ const el=document.getElementById(id); if(el)el.classList.toggle('on',on); };
  _setOn('clSexM',charLabSex==='m'); _setOn('clSexF',charLabSex==='f');
  _setOn('clFacSov',charLabFaction==='Soviets'); _setOn('clFacAll',charLabFaction==='Allies'); _setOn('clFacEmp',charLabFaction==='Empire'); _setOn('clFacElite',charLabFaction==='EmpireElite');
  const _hg=document.getElementById('clHeadgear'); if(_hg){ const n=charLabSlots.headgear||'none'; _hg.textContent='Headgear: '+(n==='none'?'None':n==='brodie'?'Brodie':n==='m1'?'M1':n==='cap'?'Field Cap':n); }
  const _tn=document.getElementById('clTorso'); if(_tn){ const n=charLabSlots.torso||'none'; _tn.textContent='Torso: '+(n==='none'?'None':n==='tunic'?'Tunic':n==='greatcoat'?'Greatcoat':n); }
  const _lg=document.getElementById('clLegs'); if(_lg){ const n=charLabSlots.legs||'none'; _lg.textContent='Legs: '+(n==='none'?'None':n==='trousers'?'Trousers':n); }
  const _ft=document.getElementById('clFeet'); if(_ft){ const n=charLabSlots.feet||'none'; _ft.textContent='Feet: '+(n==='none'?'None':n==='boots'?'Boots':n); }
  const _wb=document.getElementById('clWebbing'); if(_wb){ const n=charLabSlots.webbing||'none'; _wb.textContent='Webbing: '+(n==='none'?'None':n==='webbing'?'Webbing':n); }
  const _hr=document.getElementById('clHair'); if(_hr){ const n=charLabSlots.hair||'default'; _hr.textContent='Hair: '+(n==='default'?'Default':n==='none'?'None':n==='crop'?'Buzz Crop':n==='swept'?'Swept Back':n==='sidepart'?'Side Part':n==='bun'?'Bun':n==='ponytail'?'Ponytail':n==='mohawk'?'Mohawk':n); }
  const _fc=document.getElementById('clFace'); if(_fc){ const n=charLabSlots.face||'none'; _fc.textContent='Facial: '+(n==='none'?'None':n==='stubble'?'Stubble':n==='mustache'?'Mustache':n==='goatee'?'Goatee':n==='beard'?'Full Beard':n); }
  const sb=document.getElementById('clSpinBtn'); if(sb){ sb.textContent='Spin: '+(clSpin?'On':'Off'); sb.classList.toggle('on',clSpin); }
}
function enterCharLab(){
  if(!DEV.enabled)return;
  if(!charLabDef){ charLabSex='m'; const b=CHAR_MALE; charLabDef=clClone(b); charLabSkin=b.colors.skin; charLabHair=b.colors.hair; charLabEyes=b.colors.eyes; }
  mode='charlab'; started=false; paused=true;
  if(devPanelEl)devPanelEl.style.display='none';
  hudEl.style.display='none'; titleEl.style.display='none'; if(endEl)endEl.style.display='none';
  const el=document.getElementById('charLab'); if(el)el.style.display='block'; alEnsureUI();
  poseLabActive=false; clUndo.length=0; clBuildPanel(); clRebuild(); clHideWorld(true); _clSyncPoseUI(); clBuildPosePanel();
  clAz=0; clEl=0.12; clDist=5.6; camera.fov=40; camera.updateProjectionMatrix(); clCam();
  renderer.domElement.style.cursor='grab';
}
function exitCharLab(){ clHideWorld(false); clDispose(charLabPreview); charLabPreview=null; const el=document.getElementById('charLab'); if(el)el.style.display='none'; renderer.domElement.style.cursor='default'; backToMenu(); }
function clExport(){ const ta=document.getElementById('clJson'); if(!ta)return; const p=JSON.stringify({def:charLabDef,base:charLabBase,accent:charLabAccent}); ta.value=p; if(navigator.clipboard&&navigator.clipboard.writeText)navigator.clipboard.writeText(p).catch(()=>{}); if(KVStore.persist){KVStore.set('shamble_charlab',p);} }
function clImport(){ const ta=document.getElementById('clJson'); if(!ta||!ta.value.trim())return; try{ const o=JSON.parse(ta.value); if(o&&o.def){ charLabDef=o.def; if(o.base!==undefined)charLabBase=o.base; if(o.accent!==undefined)charLabAccent=o.accent; clBuildPanel(); clRebuild(); } }catch(e){ ta.value='// parse error: '+e.message; } }
function clSetSex(sx){ charLabSex=sx; const b=(sx==='f')?CHAR_FEMALE:CHAR_MALE; charLabDef=clClone(b); charLabSkin=b.colors.skin; charLabHair=b.colors.hair; charLabEyes=b.colors.eyes; clBuildPanel(); clRebuild(); }
function clSetFaction(f){ charLabFaction=f; clRebuild(); clBuildPanel(); }
function clReset(){ const b=(charLabSex==='f')?CHAR_FEMALE:CHAR_MALE; charLabDef=clClone(b); charLabSkin=b.colors.skin; charLabHair=b.colors.hair; charLabEyes=b.colors.eyes; charLabFaction='Soviets'; charLabSlots={headgear:'none',torso:'none',legs:'none',feet:'none',webbing:'none',hair:'default',face:'none'}; clBuildPanel(); clRebuild(); }
function clResetProps(){ charLabDef.parts=clClone((charLabSex==='f')?CHAR_FEMALE:CHAR_MALE).parts; clBuildPanel(); clRebuild(); }
function clResetColors(){ const b=(charLabSex==='f')?CHAR_FEMALE:CHAR_MALE; charLabSkin=b.colors.skin; charLabHair=b.colors.hair; charLabEyes=b.colors.eyes; clBuildPanel(); clRebuild(); }
function editorPlay(){ if(typeof _bkCloseMaker==='function')_bkCloseMaker(); if(_edGizmo)_edGizmo.visible=false; if(_edHandles)_edHandles.visible=false; if(_edRing)_edRing.visible=false;
  ovHideAll(); edRoot.style.display='none'; edSetSel([]); if(edGhost)edGhost.visible=false; edPvBanner.style.display='none'; edPalette.style.opacity=''; edPreview=false;
  mouseDown=false; semiQueued=false; renderer.domElement.style.cursor='none';
  mode='game'; started=true; paused=false;
  resetRound(); hudEl.style.display='';
}
function setEdTool(t){ edTool=t; edPrefabActive=null; edSetSel([]); edPathStart=null; if(typeof measReset==='function')measReset(); if(edGhost){edGhost.visible=false; edGhost.rotation.y=0;} document.querySelectorAll('#edPalette .edTool').forEach(el=>el.classList.toggle('active',el.dataset.tool===t)); document.querySelectorAll('#edPalette .edPrefab.active').forEach(el=>el.classList.remove('active')); if(typeof _bkSyncPlaceBtn==='function')_bkSyncPlaceBtn(); if(typeof edStatusSync==='function')edStatusSync(); if(t&&t!=='select'&&t!=='measure'&&typeof _edRecent!=='undefined'){ _edRecent=_edRecent.filter(function(x){return x!==t;}); _edRecent.unshift(t); if(_edRecent.length>8)_edRecent.length=8; } }
function edToggleSnap(){ edSnapIdx=(edSnapIdx+1)%EDSNAPS.length; edSnap=EDSNAPS[edSnapIdx]; edUpdateSnapBtn(); }
// ============ EDITOR ANALYSIS OVERLAYS ============
const ovGroup=new THREE.Group(); ovGroup.visible=false; noRay(ovGroup); scene.add(ovGroup);
const ovMeasureG=new THREE.Group(), ovSpawnG=new THREE.Group(), ovSymG=new THREE.Group(), ovCoverG=new THREE.Group();
const ovLabelsEl=document.createElement('div'); ovLabelsEl.id='edOverlay'; edRoot.appendChild(ovLabelsEl);
let ovLabels=[]; const _ovProj=new THREE.Vector3(), OV_LINE=0x7af0d2;
function ovLabel(owner, html, x,y,z, cls){ const d=document.createElement('div'); d.className='ovLabel'+(cls?(' '+cls):''); d.innerHTML=html; ovLabelsEl.appendChild(d); const rec={el:d,x:x,y:y,z:z,owner:owner}; ovLabels.push(rec); return rec; }
function ovClearLabels(owner){ for(let i=ovLabels.length-1;i>=0;i--){ if(owner===undefined||ovLabels[i].owner===owner){ ovLabels[i].el.remove(); ovLabels.splice(i,1); } } }
function ovClearGroup(g){ for(let i=g.children.length-1;i>=0;i--){ const c=g.children[i]; g.remove(c); if(c.geometry)c.geometry.dispose(); if(c.material){ const mm=Array.isArray(c.material)?c.material:[c.material]; for(const m of mm){ if(m.map)m.map.dispose(); m.dispose(); } } } }
function ovUpdateLabels(){ if(!ovLabels.length)return; const cam=edCam(), w=innerWidth, h=innerHeight;
  for(let i=0;i<ovLabels.length;i++){ const l=ovLabels[i]; _ovProj.set(l.x,l.y,l.z).project(cam);
    if(_ovProj.z>1){ l.el.style.display='none'; continue; } l.el.style.display='block'; l.el.style.left=((_ovProj.x*0.5+0.5)*w).toFixed(0)+'px'; l.el.style.top=((-_ovProj.y*0.5+0.5)*h).toFixed(0)+'px'; } }
function ovShow(){ ovGroup.visible=true; ovLabelsEl.style.display='block'; }
function ovHideAll(){ ovGroup.visible=false; ovLabelsEl.style.display='none'; }
function ovMakeSeg(g,color,w){ const m=new THREE.Mesh(new THREE.BoxGeometry(1,0.12,w||0.34), new THREE.MeshBasicMaterial({color:color,transparent:true,opacity:0.9})); noRay(m); g.add(m); return m; }
function ovPlaceSeg(m,ax,az,bx,bz,y){ const dx=bx-ax, dz=bz-az, L=Math.hypot(dx,dz); if(L<1e-3){ m.visible=false; return; } m.visible=true; m.scale.set(L,1,1); m.position.set((ax+bx)/2,y,(az+bz)/2); m.rotation.y=Math.atan2(-dz,dx); }

// ---- 4a: MEASURE TOOL (persistent reused meshes; zero per-frame allocation) ----
let edMeasA=null, edMeasB=null, _measSeg=null, _measDotA=null, _measDotB=null, _measLabel=null;
function _measEnsure(){ if(_measSeg)return;
  _measSeg=ovMakeSeg(ovMeasureG,OV_LINE,0.34);
  const dg=new THREE.SphereGeometry(0.55,12,10);
  _measDotA=new THREE.Mesh(dg,new THREE.MeshBasicMaterial({color:OV_LINE})); _measDotB=new THREE.Mesh(dg,new THREE.MeshBasicMaterial({color:OV_LINE})); noRay(_measDotA); noRay(_measDotB); ovMeasureG.add(_measDotA,_measDotB);
  _measLabel=ovLabel('measure','',0,0,0); }
function measHide(){ if(_measSeg){ _measSeg.visible=false; _measDotA.visible=false; _measDotB.visible=false; } if(_measLabel)_measLabel.el.style.display='none'; }
function measRender(bx,bz){ _measEnsure(); const ax=edMeasA.x, az=edMeasA.z, y=0.28, dx=bx-ax, dz=bz-az, L=Math.hypot(dx,dz);
  _measDotA.visible=true; _measDotA.position.set(ax,y,az);
  _measDotB.visible=true; _measDotB.position.set(bx,y,bz);
  ovPlaceSeg(_measSeg,ax,az,bx,bz,y);
  const man=Math.abs(dx)+Math.abs(dz);
  _measLabel.el.style.display='block';
  _measLabel.el.innerHTML='<b>'+L.toFixed(1)+' squares</b><span class="ovSub">Δ '+Math.abs(dx).toFixed(1)+' × '+Math.abs(dz).toFixed(1)+' · path '+man.toFixed(1)+'</span>';
  _measLabel.x=(ax+bx)/2; _measLabel.y=y+1.6; _measLabel.z=(az+bz)/2; }
function measReset(){ edMeasA=null; edMeasB=null; measHide(); }
function measClick(g){ if(edMeasB || !edMeasA){ edMeasA={x:g.x,z:g.z}; edMeasB=null; } else { edMeasB={x:g.x,z:g.z}; } }

// ---- 4b: SPAWN -> OBJECTIVE DISTANCES (fairness / rush-timing) ----
let ovSpawnOn=false, _ovSpawnSig='';
function ovObjectives(){ const list=[];
  (SILO&&SILO.points?SILO.points:[]).forEach((p,i)=>list.push({x:p.x,z:p.z,name:(SILO.points.length>1?('SILO '+(i+1)):'SILO'),kind:'silo'}));
  (TBOX&&TBOX.points?TBOX.points:[]).forEach(p=>list.push({x:p.x,z:p.z,name:(p.name||'TECH'),kind:'tech'}));
  if(HARV&&isFinite(HARV.fieldX))list.push({x:HARV.fieldX,z:HARV.fieldZ,name:'HARVESTER',kind:'harv'});
  return list; }
function ovBuildSpawn(){ ovClearGroup(ovSpawnG); ovClearLabels('spawn'); if(!ovSpawnOn)return;
  const F={x:mapSpawn.x,z:mapSpawn.z}, H={x:0,z:21-T.arenaZ}, objs=ovObjectives();
  const fm=new THREE.Mesh(new THREE.CylinderGeometry(2.4,2.4,0.3,22), new THREE.MeshBasicMaterial({color:0x38e1b0,transparent:true,opacity:0.5})); fm.position.set(F.x,0.2,F.z); noRay(fm); ovSpawnG.add(fm);
  const zx0=-(T.arenaX-8), zx1=(T.arenaX-8), zz0=-T.arenaZ+10, zz1=-T.arenaZ+32;
  const hz=new THREE.Mesh(new THREE.BoxGeometry(zx1-zx0,0.3,zz1-zz0), new THREE.MeshBasicMaterial({color:0xff5a5a,transparent:true,opacity:0.15})); hz.position.set((zx0+zx1)/2,0.18,(zz0+zz1)/2); noRay(hz); ovSpawnG.add(hz);
  ovLabel('spawn','<b>SPAWN</b>',F.x,3.2,F.z,'friend');
  ovLabel('spawn','<b>HOSTILE SPAWN</b><span class="ovSub">random across band</span>',H.x,3.2,H.z,'hostile');
  for(let i=0;i<objs.length;i++){ const o=objs[i], df=Math.hypot(o.x-F.x,o.z-F.z), dh=Math.hypot(o.x-H.x,o.z-H.z);
    const sf=ovMakeSeg(ovSpawnG,0x38e1b0,0.32); ovPlaceSeg(sf,F.x,F.z,o.x,o.z,0.22); sf.material.opacity=0.45;
    const sh=ovMakeSeg(ovSpawnG,0xff5a5a,0.32); ovPlaceSeg(sh,H.x,H.z,o.x,o.z,0.24); sh.material.opacity=0.45;
    const lo=Math.max(1,Math.min(df,dh)), warn=Math.abs(df-dh)/lo>0.15;
    let html='<b>'+o.name+'</b><span class="ovSub">F '+df.toFixed(0)+'m · H '+dh.toFixed(0)+'m';
    if(warn)html+=' · '+(df<dh?'F':'H')+' +'+Math.abs(df-dh).toFixed(0)+'m'; html+='</span>';
    ovLabel('spawn',html,o.x,3.4,o.z, warn?'warn':undefined); } }
function ovSpawnTick(){ if(!ovSpawnOn)return; const objs=ovObjectives(); let sig=mapSpawn.x.toFixed(1)+','+mapSpawn.z.toFixed(1)+'|'+T.arenaX.toFixed(1)+','+T.arenaZ.toFixed(1)+'|'; for(let i=0;i<objs.length;i++)sig+=objs[i].x.toFixed(1)+','+objs[i].z.toFixed(1)+';'; if(sig!==_ovSpawnSig){ _ovSpawnSig=sig; ovBuildSpawn(); } }
function toggleSpawnDist(){ ovSpawnOn=!ovSpawnOn; _ovSpawnSig=''; if(ovSpawnOn){ ovSpawnTick(); } else { ovClearGroup(ovSpawnG); ovClearLabels('spawn'); } const b=document.getElementById('ovSpawnBtn'); if(b){ b.classList.toggle('on',ovSpawnOn); const t=b.querySelector('.tg'); if(t)t.textContent=ovSpawnOn?'ON':'OFF'; } }
function ovResetAll(){ if(ovSpawnOn)toggleSpawnDist(); if(ovSymAxis)toggleSymmetry(); if(typeof toggleCover==='function'&&ovCoverOn)toggleCover(); }
function ovToggle(name){ if(name==='spawn')toggleSpawnDist(); else if(name==='sym'&&typeof toggleSymmetry==='function')toggleSymmetry(); else if(name==='cover'&&typeof toggleCover==='function')toggleCover(); }

// ---- 4c: SYMMETRY-DIFF (fairness auditor) ----
let ovSymAxis=null, _ovSymSig='';
const _ovTmpC=new THREE.Vector3(), _ovTmpS=new THREE.Vector3(), _ovTmpB=new THREE.Box3();
function ovMirror(x,z,axis){ return axis==='x'? {x:-x,z:z} : {x:x,z:-z}; }
function ovWireBox(g,cx,cy,cz,sx,sy,sz,color){ const bg=new THREE.BoxGeometry(Math.max(0.4,sx),Math.max(0.4,sy),Math.max(0.4,sz)); const eg=new THREE.EdgesGeometry(bg); bg.dispose(); const m=new THREE.LineSegments(eg,new THREE.LineBasicMaterial({color:color,transparent:true,opacity:0.92})); m.position.set(cx,cy,cz); noRay(m); g.add(m); return m; }
function ovStructures(){ const list=[], bkWalls=new Set();
  for(const bk of barracksList){ if(bk.walls)for(const w of bk.walls)bkWalls.add(w); }
  for(const o of obstacles){ if(bkWalls.has(o))continue; o.box.getCenter(_ovTmpC); o.box.getSize(_ovTmpS); list.push({cat:o.crate?'crate':'wall',type:null,team:null,x:_ovTmpC.x,z:_ovTmpC.z,cx:_ovTmpC.x,cy:_ovTmpC.y,cz:_ovTmpC.z,sx:_ovTmpS.x,sy:_ovTmpS.y,sz:_ovTmpS.z}); }
  for(const b of buildings){ b.box.getCenter(_ovTmpC); b.box.getSize(_ovTmpS); list.push({cat:'building',type:b.type,team:b.team,x:_ovTmpC.x,z:_ovTmpC.z,cx:_ovTmpC.x,cy:_ovTmpC.y,cz:_ovTmpC.z,sx:_ovTmpS.x,sy:_ovTmpS.y,sz:_ovTmpS.z}); }
  for(const p of platforms){ p.box.getCenter(_ovTmpC); p.box.getSize(_ovTmpS); list.push({cat:'platform',type:null,team:null,x:_ovTmpC.x,z:_ovTmpC.z,cx:_ovTmpC.x,cy:_ovTmpC.y,cz:_ovTmpC.z,sx:_ovTmpS.x,sy:_ovTmpS.y,sz:_ovTmpS.z}); }
  for(const rp of ramps){ rp.box.getCenter(_ovTmpC); rp.box.getSize(_ovTmpS); list.push({cat:'ramp',type:null,team:null,x:_ovTmpC.x,z:_ovTmpC.z,cx:_ovTmpC.x,cy:_ovTmpC.y,cz:_ovTmpC.z,sx:_ovTmpS.x,sy:_ovTmpS.y,sz:_ovTmpS.z}); }
  for(const bk of barracksList){ if(!bk.roof)continue; _ovTmpB.setFromObject(bk.roof); const hy=bk.roof.position.y; list.push({cat:'barracks',type:null,team:bk.team,x:bk.cx,z:bk.cz,cx:bk.cx,cy:hy/2,cz:bk.cz,sx:_ovTmpB.max.x-_ovTmpB.min.x,sy:hy,sz:_ovTmpB.max.z-_ovTmpB.min.z}); }
  for(const o of ovObjectives())list.push({cat:'obj',type:o.kind,team:null,x:o.x,z:o.z,cx:o.x,cy:2,cz:o.z,sx:5,sy:4,sz:5});
  return list; }
function ovBuildSym(){ ovClearGroup(ovSymG); ovClearLabels('sym'); if(ovSymAxis===null)return;
  const items=ovStructures(), axis=ovSymAxis, tol=3.0; let matched=0, unmatched=0;
  for(let i=0;i<items.length;i++){ const O=items[i], coord=(axis==='x'?O.x:O.z); let isM;
    if(Math.abs(coord)<tol && O.team==null){ isM=true; }
    else { const M=ovMirror(O.x,O.z,axis), expTeam=(axis==='z'&&O.team)?(O.team==='friend'?'hostile':'friend'):O.team; isM=false;
      for(let j=0;j<items.length;j++){ if(j===i)continue; const P=items[j]; if(P.cat===O.cat && (P.type||'')===(O.type||'') && Math.abs(P.x-M.x)<tol && Math.abs(P.z-M.z)<tol && (expTeam==null||P.team===expTeam)){ isM=true; break; } } }
    if(isM)matched++; else unmatched++;
    ovWireBox(ovSymG,O.cx,O.cy,O.cz,O.sx+0.4,O.sy+0.4,O.sz+0.4, isM?0x38e1b0:0xff5a5a); }
  ovLabel('sym','<b>SYMMETRY '+(axis==='z'?'N–S':'E–W')+'</b><span class="ovSub">'+matched+' mirrored · '+unmatched+' unmatched</span>', 0, 12, 0, unmatched?'warn':undefined); }
function _ovSymSigCalc(){ let acc=0; for(const o of obstacles)acc+=o.mesh.position.x*3+o.mesh.position.z*7; for(const b of buildings)acc+=b.mesh.position.x*3+b.mesh.position.z*7; for(const p of platforms)acc+=p.mesh.position.x*3+p.mesh.position.z*7; for(const r of ramps)acc+=r.mesh.position.x*3+r.mesh.position.z*7; for(const k of barracksList)acc+=k.cx*3+k.cz*7;
  return ovSymAxis+'|'+obstacles.length+','+buildings.length+','+platforms.length+','+ramps.length+','+barracksList.length+','+(SILO&&SILO.points?SILO.points.length:0)+','+(TBOX&&TBOX.points?TBOX.points.length:0)+'|'+acc.toFixed(2); }
function ovSymTick(){ if(ovSymAxis===null)return; const sig=_ovSymSigCalc(); if(sig!==_ovSymSig){ _ovSymSig=sig; ovBuildSym(); } }
function toggleSymmetry(){ ovSymAxis = ovSymAxis===null?'z' : (ovSymAxis==='z'?'x':null); _ovSymSig=''; if(ovSymAxis)ovSymTick(); else { ovClearGroup(ovSymG); ovClearLabels('sym'); } const b=document.getElementById('ovSymBtn'); if(b){ b.classList.toggle('on',ovSymAxis!==null); const t=b.querySelector('.tg'); if(t)t.textContent=(ovSymAxis===null?'OFF':(ovSymAxis==='z'?'N-S':'E-W')); } }

// ---- 4d: COVER / LOS HEATMAP (openness analysis; compute-on-toggle) ----
let ovCoverOn=false;
function heatColor(f){ f=Math.max(0,Math.min(1,f)); let r,g,b;
  if(f<0.5){ const t=f/0.5; r=Math.round(30+t*40); g=Math.round(120+t*135); b=Math.round(255-t*180); }
  else { const t=(f-0.5)/0.5; r=Math.round(70+t*185); g=Math.round(255-t*175); b=Math.round(75-t*40); }
  return 'rgb('+r+','+g+','+b+')'; }
function ovBuildCover(){ ovClearGroup(ovCoverG); ovClearLabels('cover'); if(!ovCoverOn)return;
  syncSimWorld(true); const S=SIM.statics, ax=T.arenaX, az=T.arenaZ, y=1.2;
  const span=Math.max(2*ax,2*az), step=Math.max(12,span/16);
  const cols=Math.max(2,Math.round(2*ax/step)), rows=Math.max(2,Math.round(2*az/step));
  const x0=-ax, z0=-az, cw=(2*ax)/cols, ch=(2*az)/rows, N=cols*rows;
  const px=new Float64Array(N), pz=new Float64Array(N), ok=new Uint8Array(N);
  for(let r=0;r<rows;r++)for(let c=0;c<cols;c++){ const i=r*cols+c, X=x0+(c+0.5)*cw, Z=z0+(r+0.5)*ch; px[i]=X; pz[i]=Z; let blk=false;
    for(let k=0;k<S.length;k++){ const s=S[k]; if(s.alive===false)continue; if(X>=s.minx&&X<=s.maxx&&Z>=s.minz&&Z<=s.maxz&&s.miny<=y&&s.maxy>=0.3){ blk=true; break; } } ok[i]=blk?0:1; }
  const expo=new Int32Array(N); let maxE=1, walk=0; for(let i=0;i<N;i++)if(ok[i])walk++;
  const A={x:0,y:y,z:0}, B={x:0,y:y,z:0};
  for(let i=0;i<N;i++){ if(!ok[i])continue; A.x=px[i]; A.z=pz[i]; let e=0;
    for(let j=0;j<N;j++){ if(j===i||!ok[j])continue; B.x=px[j]; B.z=pz[j]; if(hasLOS(A,B))e++; } expo[i]=e; if(e>maxE)maxE=e; }
  const cv=document.createElement('canvas'); cv.width=cols; cv.height=rows; const cx2=cv.getContext('2d'); cx2.clearRect(0,0,cols,rows);
  for(let r=0;r<rows;r++)for(let c=0;c<cols;c++){ const i=r*cols+c; if(!ok[i])continue; cx2.fillStyle=heatColor(expo[i]/maxE); cx2.fillRect(c,r,1,1); }
  const tex=new THREE.CanvasTexture(cv); tex.magFilter=THREE.NearestFilter; tex.minFilter=THREE.NearestFilter;
  const pl=new THREE.Mesh(new THREE.PlaneGeometry(2*ax,2*az), new THREE.MeshBasicMaterial({map:tex,transparent:true,opacity:0.62,depthWrite:false}));
  pl.rotation.x=-Math.PI/2; pl.position.set(0,0.15,0); noRay(pl); ovCoverG.add(pl);
  ovLabel('cover','<b>COVER MAP</b><span class="ovSub">blue hidden → red exposed · '+walk+' cells</span>', -ax+44, 5, az-16); }
function toggleCover(){ ovCoverOn=!ovCoverOn; if(ovCoverOn)ovBuildCover(); else { ovClearGroup(ovCoverG); ovClearLabels('cover'); } const b=document.getElementById('ovCoverBtn'); if(b){ b.classList.toggle('on',ovCoverOn); const t=b.querySelector('.tg'); if(t)t.textContent=ovCoverOn?'ON':'OFF'; } }
function edUpdateSnapBtn(){ const b=document.getElementById('edSnapBtn'); if(b){ {const l=b.querySelector('.edBtnLab'); if(l)l.textContent=edSnap?('Grid '+edSnap+' m'):'Grid Off';} b.classList.toggle('on',!!edSnap); } }
function edCycleRot(){ edRotIdx=(edRotIdx+1)%EDROTS.length; edRotStep=EDROTS[edRotIdx]; edUpdateRotBtn(); }
function edUpdateRotBtn(){ const b=document.getElementById('edRotBtn'); if(b){const l=b.querySelector('.edBtnLab'); if(l)l.textContent='Snap '+edRotStep+'°';} }
function edGround(){ raycaster.setFromCamera(mouseNDC,edCam()); if(!raycaster.ray.intersectPlane(groundPlane,_v))return null; const s=edSnap; if(!s) return {x:Math.round(_v.x*10)/10, z:Math.round(_v.z*10)/10}; return {x:Math.round(_v.x/s)*s, z:Math.round(_v.z/s)*s}; }
function edPlaceAt(p){ let sel=null;
  if(edTool==='crateS') sel=addBox(p.x,p.z,2,1.5,2,crateMat,true).mesh.userData.editSel;
  else if(edTool==='crateM') sel=addBox(p.x,p.z,3,2.5,3,crateMatWarm,true).mesh.userData.editSel;
  else if(edTool==='crateL') sel=addBox(p.x,p.z,4,3.5,4,crateMat,true).mesh.userData.editSel;
  else if(edTool==='platform') sel=makePlatform(p.x,p.z,10,10,2.5).mesh.userData.editSel;
  else if(edTool==='ramp') sel=makeRamp(p.x,p.z,12,8,0,2.5,0).mesh.userData.editSel;
  else if(edTool==='house') sel=placeLegacyHouse(makeTimberHouse(p.x,p.z)).grp.userData.editSel;
  else if(edTool==='cottage') sel=placeLegacyHouse(makeThatchCottage(p.x,p.z)).grp.userData.editSel;
  else if(edTool==='farmhouse') sel=placeLegacyHouse(makeBrickFlintFarmhouse(p.x,p.z)).grp.userData.editSel;
  else if(edTool==='manor') sel=placeLegacyHouse(makeManorHouse(p.x,p.z), true).grp.userData.editSel;
  else if(edTool==='bmaker'&&typeof _bkPlaceAt==='function') sel=_bkPlaceAt(p.x,p.z);
  else if(edTool==='hqF') sel=makeBuilding('friend','hq',p.x,p.z).mesh.userData.editSel;
  else if(edTool==='refF') sel=makeBuilding('friend','refinery',p.x,p.z).mesh.userData.editSel;
  else if(edTool==='barF') sel=makeBuilding('friend','barracks',p.x,p.z).mesh.userData.editSel;
  else if(edTool==='hqH') sel=makeBuilding('hostile','hq',p.x,p.z).mesh.userData.editSel;
  else if(edTool==='refH') sel=makeBuilding('hostile','refinery',p.x,p.z).mesh.userData.editSel;
  else if(edTool==='barH') sel=makeBuilding('hostile','barracks',p.x,p.z).mesh.userData.editSel;
  else if(edTool==='powF') sel=makeBuilding('friend','power',p.x,p.z).mesh.userData.editSel;
  else if(edTool==='powH') sel=makeBuilding('hostile','power',p.x,p.z).mesh.userData.editSel;
  else if(edTool==='espawnSov') sel=makeEnemySpawn(p.x,p.z,'Soviets').grp.userData.editSel;
  else if(edTool==='espawnAll') sel=makeEnemySpawn(p.x,p.z,'Allies').grp.userData.editSel;
  else if(edTool==='espawnEmp') sel=makeEnemySpawn(p.x,p.z,'Empire').grp.userData.editSel;
  else if(edTool==='treeApple') sel=addTree('apple',p.x,p.z).grp.userData.editSel;
  else if(edTool==='treePoplar') sel=addTree('poplar',p.x,p.z).grp.userData.editSel;
  else if(edTool==='treeOak') sel=addTree('oak',p.x,p.z).grp.userData.editSel;
  else if(edTool==='propHaystack') sel=addProp('haystack',p.x,p.z).grp.userData.editSel;
  else if(edTool==='propHaybales') sel=addProp('haybales',p.x,p.z).grp.userData.editSel;
  else if(edTool==='propCart') sel=addProp('cart',p.x,p.z).grp.userData.editSel;
  else if(edTool==='propWell') sel=addProp('well',p.x,p.z).grp.userData.editSel;
  else if(edTool==='propLogpile') sel=addProp('logpile',p.x,p.z).grp.userData.editSel;
  else if(edTool==='propBarrels') sel=addProp('barrels',p.x,p.z).grp.userData.editSel;
  else if(edTool==='propTrough') sel=addProp('trough',p.x,p.z).grp.userData.editSel;
  else if(edTool==='propVegpatch') sel=addProp('vegpatch',p.x,p.z).grp.userData.editSel;
  else if(edTool==='barracks') sel={kind:'barracks',ref:makeBarracks(p.x,p.z)};
  else if(edTool==='spawn'){ mapSpawn.set(p.x,0,p.z); player.position.set(p.x,0,p.z); }
  else if(edTool==='resupply'){ terminalPos.set(p.x,0,p.z); if(termMesh)termMesh.position.set(p.x,0.8,p.z); if(resupplyLabel)resupplyLabel.position.set(p.x,2.6,p.z); }
  rebuildLOS(); return sel;
}
function edRemoveOne(sel){
  if(sel.kind==='crate'||sel.kind==='wall'){ scene.remove(sel.ref.mesh); const i=obstacles.indexOf(sel.ref); if(i>=0)obstacles.splice(i,1); }
  else if(sel.kind==='path'){ scene.remove(sel.ref.mesh); const i=paths.indexOf(sel.ref); if(i>=0)paths.splice(i,1); }
  else if(sel.kind==='building'){ const b=sel.ref; [b.mesh,b.trim,b.label,b.bar.grp].forEach(o=>scene.remove(o)); const i=buildings.indexOf(b); if(i>=0)buildings.splice(i,1); }
  else if(sel.kind==='barracks'){ const b=sel.ref; b.walls.forEach(w=>{scene.remove(w.mesh); const i=obstacles.indexOf(w); if(i>=0)obstacles.splice(i,1);}); [b.lintel,b.floor,b.roof,b.label,b.panel,b.plabel].forEach(o=>{if(o)scene.remove(o);}); const i=barracksList.indexOf(b); if(i>=0)barracksList.splice(i,1); }
  else if(sel.kind==='platform'){ scene.remove(sel.ref.mesh); const i=platforms.indexOf(sel.ref); if(i>=0)platforms.splice(i,1); }
  else if(sel.kind==='ramp'){ scene.remove(sel.ref.mesh); const i=ramps.indexOf(sel.ref); if(i>=0)ramps.splice(i,1); }
  else if(sel.kind==='house'){ scene.remove(sel.ref.grp); const i=houses.indexOf(sel.ref); if(i>=0)houses.splice(i,1); }
  else if(sel.kind==='cover'){ scene.remove(sel.ref.grp); const i=covers.indexOf(sel.ref); if(i>=0)covers.splice(i,1); }
  else if(sel.kind==='tree'){ scene.remove(sel.ref.grp); const i=trees.indexOf(sel.ref); if(i>=0)trees.splice(i,1); }
  else if(sel.kind==='prop'){ scene.remove(sel.ref.grp); const i=props.indexOf(sel.ref); if(i>=0)props.splice(i,1); }
  else if(sel.kind==='enemyspawn'){ enemySpawnLayer.remove(sel.ref.grp); const i=enemySpawns.indexOf(sel.ref); if(i>=0)enemySpawns.splice(i,1); }
}
function edDeleteSel(){ if(!edMulti.length)return; edPushUndo(); for(const sel of edMulti)edRemoveOne(sel); edSetSel([]); rebuildLOS(); }
function edSetTeam(team){ const toSet=edMulti.filter(s=>{ const d=edDescribe(s); return d&&(d.t==='building'||d.t==='barracks')&&d.team!==team; }); if(!toSet.length)return;
  const descs=toSet.map(edDescribe); const keep=edMulti.filter(s=>toSet.indexOf(s)<0); edPushUndo('team');
  for(const s of toSet)edRemoveOne(s); const news=[]; for(const d of descs){ d.team=team; const sel=edSpawnDesc(d,d.ax,d.az); if(sel)news.push(sel); }
  rebuildLOS(); edSetSel(keep.concat(news)); }
function edZoom(dy){ edCamH=Math.max(28,Math.min(140,edCamH+(dy>0?6:-6))); }
// ---- Tier 4: orthographic top-down toggle (own camera; preview stays perspective) ----
let edOrtho=false, edOrthoCam=null, _edOrthoH=-1, _edOrthoA=-1;
function edEnsureOrtho(){ if(!edOrthoCam)edOrthoCam=new THREE.OrthographicCamera(-1,1,1,-1,0.1,600); return edOrthoCam; }
function edCam(){ return (edOrtho&&!edPreview&&edOrthoCam)?edOrthoCam:camera; }
function edSyncOrtho(){ if(!edOrtho)return; const oc=edEnsureOrtho(); const a=innerWidth/innerHeight, halfH=edCamH*0.49, halfW=halfH*a;
  if(edCamH!==_edOrthoH||a!==_edOrthoA){ oc.left=-halfW; oc.right=halfW; oc.top=halfH; oc.bottom=-halfH; oc.updateProjectionMatrix(); _edOrthoH=edCamH; _edOrthoA=a; }
  oc.up.set(0,0,-1); oc.position.set(edCamX,200,edCamZ); oc.lookAt(edCamX,0,edCamZ); }
function edToggleOrtho(){ edOrtho=!edOrtho; if(edOrtho){ edEnsureOrtho(); _edOrthoH=-1; edSyncOrtho(); } edUpdateOrthoBtn(); }
function edUpdateOrthoBtn(){ const b=document.getElementById('edOrthoBtn'); if(b){ {const l=b.querySelector('.edBtnLab'); if(l)l.textContent=edOrtho?'Ortho On':'Ortho Off';} b.classList.toggle('on',edOrtho); } }

// ---- Tab playtest: walk the geometry from a low over-the-shoulder view ----
function edTogglePreview(){ edPreview?edExitPreview():edEnterPreview(); }
function edEnterPreview(){ edPreview=true; edSetSel([]); if(edGhost)edGhost.visible=false; edMarquee=null; edMarqueeEl.style.display='none';
  player.position.set(edCamX,0,edCamZ); player.rotation.set(0,0,0); player.visible=true;
  renderer.domElement.style.cursor='none'; edPalette.style.opacity='0.18'; edPvBanner.style.display='block'; ovHideAll(); }
function edExitPreview(){ edPreview=false; renderer.domElement.style.cursor='crosshair'; edPalette.style.opacity=''; edPvBanner.style.display='none';
  player.position.copy(mapSpawn); player.rotation.set(0,0,0); ovShow(); }
function edPreviewUpdate(dt){
  raycaster.setFromCamera(mouseNDC,camera); if(raycaster.ray.intersectPlane(groundPlane,_v))_pvAim.copy(_v);
  let fx=0,fz=1; const fd=_pv1.set(_pvAim.x-player.position.x,0,_pvAim.z-player.position.z);
  if(fd.lengthSq()>0.05){ fd.normalize(); fx=fd.x; fz=fd.z; player.rotation.y=Math.atan2(fx,fz); }
  const rx=fz, rz=-fx; const wish=_pv2.set(0,0,0);
  if(keys['KeyW']){wish.x+=fx;wish.z+=fz;} if(keys['KeyS']){wish.x-=fx;wish.z-=fz;}
  if(keys['KeyD']){wish.x+=rx;wish.z+=rz;} if(keys['KeyA']){wish.x-=rx;wish.z-=rz;}
  const moving=wish.lengthSq()>0; if(moving){ wish.normalize(); const sp=((keys['ShiftLeft']||keys['ShiftRight'])?15:9.5); player.position.x+=wish.x*sp*dt; player.position.z+=wish.z*sp*dt; }
  const limX=T.arenaX-1, limZ=T.arenaZ-1; player.position.x=Math.max(-limX,Math.min(limX,player.position.x)); player.position.z=Math.max(-limZ,Math.min(limZ,player.position.z));
  resolveObstacles(player.position,0.7); tickVertical(player.position, ps, dt, keys['Space'] && !ps.dead, 0.7, true);
  _animSt.aimYaw=player.rotation.y;_animSt.t=performance.now()*0.001;_animSt.kick=0;_animSt.dip=0;_animSt.lean=0;_animSt.grounded=ps.grounded;_animSt.ads=0;_animSt.swap=0; updateSoldierAnim(player,_animSt,dt);
  _pv1.set(player.position.x-fx*7, player.position.y+4.4, player.position.z-fz*7); camera.up.set(0,1,0); camera.position.lerp(_pv1,1-Math.exp(-9*dt));
  camera.lookAt(player.position.x+fx*2.4, player.position.y+1.7, player.position.z+fz*2.4);
}
function updateEditor(dt){
  if(edPreview){ if(_edGizmo)_edGizmo.visible=false; if(_edHandles)_edHandles.visible=false; if(_edRing)_edRing.visible=false; edPreviewUpdate(dt); return; }
  const pan=edCamH*0.9*dt; let dx=0,dz=0;
  if(keys['KeyW'])dz-=1; if(keys['KeyS'])dz+=1; if(keys['KeyA'])dx-=1; if(keys['KeyD'])dx+=1;
  if(!edMulti.length){ if(keys['ArrowUp'])dz-=1; if(keys['ArrowDown'])dz+=1; if(keys['ArrowLeft'])dx-=1; if(keys['ArrowRight'])dx+=1; }
  if(dx||dz){ const l=Math.hypot(dx,dz); edCamX+=dx/l*pan; edCamZ+=dz/l*pan; }
  const _ecl=Math.min(230,Math.max(90,Math.max(T.arenaX,T.arenaZ)+30)); edCamX=Math.max(-_ecl,Math.min(_ecl,edCamX)); edCamZ=Math.max(-_ecl,Math.min(_ecl,edCamZ));
  if(camera.fov!==52){camera.fov=52;camera.updateProjectionMatrix();} camera.up.set(0,1,0); camera.position.set(edCamX,edCamH,edCamZ+edCamH*0.18); camera.lookAt(edCamX,0,edCamZ);
  edSyncOrtho();
  if(edSelBoxes.length)edRefreshBoxes();
  const g=edGround();
  if(g){ edCursorEl.textContent=g.x+', '+g.z; const fp=edFootprint(edTool); if(fp&&edGhost){ edGhost.visible=true; edGhost.scale.set(fp.w,fp.h,fp.d); edGhost.position.set(g.x,fp.h/2,g.z); } else if(edGhost&&edTool!=='path'&&!_bkIsCover(edTool)){ edGhost.visible=false; } if(edOrchardGhost&&!((edTool==='orchard'||edTool==='poprow')&&edPathStart))edOrchardGhost.visible=false; }
  else if(edGhost&&edTool!=='path'&&!_bkIsCover(edTool)){ edGhost.visible=false; }
  ovGroup.visible=true;
  if(edTool==='measure'){ if(edMeasA && (edMeasB||g)){ const b=edMeasB||g; measRender(b.x,b.z); } } else measHide();
  ovSpawnTick();
  ovSymTick();
  ovUpdateLabels();
}
const MapStore={ mem:{}, persist:KVStore.persist,
  async list(){ const s=new Set(Object.keys(this.mem)); if(this.persist){try{const r=await KVStore.list('vmap:');(r&&r.keys?r.keys:[]).forEach(k=>s.add(k.replace(/^vmap:/,'')));}catch(e){}} return [...s].sort(); },
  async save(n,def){ this.mem[n]=def; if(this.persist){try{await KVStore.set('vmap:'+n,JSON.stringify(def));}catch(e){}} },
  async load(n){ if(this.persist){try{const r=await KVStore.get('vmap:'+n); if(r&&r.value)return JSON.parse(r.value);}catch(e){}} return this.mem[n]||null; },
  async del(n){ delete this.mem[n]; if(this.persist){try{await KVStore.delete('vmap:'+n);}catch(e){}} }
};
const edMapsModal=document.getElementById('edMapsModal'), edMapList=document.getElementById('edMapList');
function openMaps(){ document.getElementById('edPersistNote').textContent = MapStore.persist?'(saved to browser storage)':'(this session only — use Export to keep permanently)'; document.getElementById('edSaveName').value=document.getElementById('edMapName').textContent||''; edMapsModal.style.display='flex'; refreshMapList(); }
let edPrefabActive=null;
const PrefabStore={ mem:{}, persist:KVStore.persist,
  async list(){ const s=new Set(Object.keys(this.mem)); if(this.persist){try{const r=await KVStore.list('vprefab:');(r&&r.keys?r.keys:[]).forEach(k=>s.add(k.replace(/^vprefab:/,'')));}catch(e){}} return [...s].sort(); },
  async save(n,data){ this.mem[n]=data; if(this.persist){try{await KVStore.set('vprefab:'+n,JSON.stringify(data));}catch(e){}} },
  async load(n){ if(this.persist){try{const r=await KVStore.get('vprefab:'+n); if(r&&r.value)return JSON.parse(r.value);}catch(e){}} return this.mem[n]||null; },
  async del(n){ delete this.mem[n]; if(this.persist){try{await KVStore.delete('vprefab:'+n);}catch(e){}} }
};
function edStampPrefab(items,cx,cz){ const placed=[]; for(const d of items){ const sel=edSpawnDesc(d, cx+(d.rx||0), cz+(d.rz||0)); if(sel)placed.push(sel); } return placed; }
function edSelectPrefab(name,items,rowEl){ edTool='prefab'; edPrefabActive=items; edSetSel([]); edPathStart=null; if(edGhost)edGhost.visible=false;
  document.querySelectorAll('#edPalette .edTool').forEach(el=>el.classList.remove('active'));
  document.querySelectorAll('#edPalette .edPrefab').forEach(el=>el.classList.toggle('active',el===rowEl)); }
const edWorldPanel=document.getElementById('edWorld'), edWorldBody=document.getElementById('edWorldBody');
const ENV_SCHEMA=[
  { group:'Ground', items:[
    { k:'ground.base',        label:'Base colour',       type:'color', tip:'Overall grass colour (the canvas fill)', regen:true },
    { k:'ground.detailDark',  label:'Detail \u00b7 dark',  type:'color', tip:'Darker tufts & blades', regen:true },
    { k:'ground.detailLight', label:'Detail \u00b7 light', type:'color', tip:'Lighter tufts & blades', regen:true },
    { k:'ground.density',     label:'Detail amount',     type:'range', min:0.4, max:2, step:0.05, tip:'How many tufts & blades are painted', regen:true },
    { k:'ground.contrast',    label:'Detail contrast',   type:'range', min:0.5, max:2, step:0.05, tip:'Strength of the tuft/blade detail', regen:true },
    { k:'ground.patch',       label:'Tonal patches',     type:'range', min:0, max:2, step:0.05, tip:'Large soft colour variation across the field', regen:true },
    { k:'ground.tiling',      label:'Tiling',            type:'range', min:6, max:40, step:1, tip:'How many times the texture repeats across the field', regen:false, int:true },
    { k:'ground.litCap',      label:'Brightness cap',    type:'range', min:170, max:255, step:1, tip:'How bright LIT grass gets \u2014 lower = richer, less blown-out (the shaded look)', regen:false, int:true },
    { k:'nature.gain',        label:'Nature brightness', type:'range', min:0.8, max:2.2, step:0.02, tip:'Brightens grass, trees and hedges only \u2014 buildings and characters keep the richer lighting', regen:false },
  ]},
  { group:'Lighting', items:[
    { k:'sun.color',   label:'Sun colour',    type:'color', tip:'Directional sunlight colour' },
    { k:'sun.int',     label:'Sun strength',  type:'range', min:0, max:2.5, step:0.02, tip:'Sunlight intensity' },
    { k:'sun.az',      label:'Sun direction', type:'range', min:-180, max:180, step:1, tip:'Sun compass angle (rotates the shadows)', int:true },
    { k:'sun.el',      label:'Sun height',    type:'range', min:10, max:89, step:1, tip:'Sun elevation (shadow length)', int:true },
    { k:'amb.color',   label:'Ambient tint',  type:'color', tip:'Scene-wide fill-light colour (tints shadows)' },
    { k:'amb.int',     label:'Ambient',       type:'range', min:0, max:1, step:0.01, tip:'Fill-light strength' },
    { k:'hemi.sky',    label:'Sky light',     type:'color', tip:'Hemisphere light coming from above' },
    { k:'hemi.ground', label:'Bounce light',  type:'color', tip:'Hemisphere light from the ground (colours shadows)' },
    { k:'hemi.int',    label:'Sky/bounce',    type:'range', min:0, max:1.5, step:0.02, tip:'Hemisphere light strength' },
  ]},
  { group:'Sky & Fog', items:[
    { k:'sky.bg',    label:'Sky colour', type:'color', tip:'Background sky colour' },
    { k:'fog.color', label:'Fog colour', type:'color', tip:'Distance-haze colour' },
    { k:'fog.near',  label:'Fog start',  type:'range', min:20, max:200, step:1, tip:'Distance where the haze begins', int:true },
    { k:'fog.far',   label:'Fog end',    type:'range', min:60, max:360, step:1, tip:'Distance where the haze becomes full', int:true },
  ]},
];
function _envGet(k){ const p=k.split('.'); return _env[p[0]][p[1]]; }
function _envSet(k,v){ const p=k.split('.'); _env[p[0]][p[1]]=v; }
const _edCtl={};
function edBuildWorld(){
  edWorldBody.innerHTML='';
  for(const grp of ENV_SCHEMA){
    const h=document.createElement('div'); h.className='edWGroup'; h.textContent=grp.group; edWorldBody.appendChild(h);
    for(const it of grp.items){
      const row=document.createElement('div'); row.className='edWRow'; row.setAttribute('data-tip',it.tip);
      const lab=document.createElement('span'); lab.className='edWLab'; lab.textContent=it.label; row.appendChild(lab);
      const inp=document.createElement('input'); inp.className='edWInp '+it.type;
      const val=document.createElement('span'); val.className='edWVal';
      if(it.type==='color'){ inp.type='color'; inp.value=_envGet(it.k); }
      else { inp.type='range'; inp.min=it.min; inp.max=it.max; inp.step=it.step; inp.value=_envGet(it.k); val.textContent=(it.int?Math.round(_envGet(it.k)):_envGet(it.k)); }
      const apply=(commit)=>{ const v=(it.type==='color')?inp.value:(it.int?Math.round(+inp.value):+inp.value); _envSet(it.k,v); if(it.type==='range')val.textContent=v; applyEnv(_env,{regen:!!(it.regen&&commit)}); };
      if(it.type==='color'){ inp.addEventListener('change',()=>{ edPushUndo('env'); apply(true); }); }
      else { inp.addEventListener('pointerdown',()=>edPushUndo('env')); inp.addEventListener('keydown',()=>edPushUndo('env')); inp.addEventListener('input',()=>apply(false)); inp.addEventListener('change',()=>apply(true)); }
      row.appendChild(inp); if(it.type==='range')row.appendChild(val);
      edWorldBody.appendChild(row); _edCtl[it.k]={inp:inp,val:val,it:it};
    }
  }
}
function edSyncWorldPanel(){ for(const k in _edCtl){ const c=_edCtl[k], v=_envGet(k); c.inp.value=v; if(c.it.type==='range'&&c.val)c.val.textContent=(c.it.int?Math.round(v):v); } edSyncRulesPanel(); }
function edToggleWorld(){ if(typeof edSetSel==='function')edSetSel([]); const el=document.getElementById('edWorld'); if(el){ if(el.scrollIntoView)el.scrollIntoView({block:'nearest'}); el.classList.remove('edFlash'); void el.offsetWidth; el.classList.add('edFlash'); } }
const edRulesPanel=document.getElementById('edRules'), edRulesBody=document.getElementById('edRulesBody'); const _edRulesCtl={};
const RULES_SCHEMA=[
  {group:'Objectives', items:[
    {k:'silo.capture', label:'Silo capture',     min:0.5, max:15,  step:0.5,        unit:'s', tip:'Seconds a team must hold a silo pad (uncontested) to claim it.'},
    {k:'tbox.active',  label:'Tech-box active',  min:20,  max:300, step:5,  int:1,  unit:'s', tip:'How long a Tech Box stays live before it expires.'},
    {k:'tbox.capture', label:'Tech-box capture', min:1,   max:30,  step:0.5,        unit:'s', tip:'Seconds on the ring to claim a live Tech Box.'},
    {k:'tbox.first',   label:'Tech-box first',   min:0,   max:60,  step:1,  int:1,  unit:'s', tip:'Delay before the FIRST Tech Box spawns in a match.'},
    {k:'tbox.gap',     label:'Tech-box gap',     min:0,   max:60,  step:1,  int:1,  unit:'s', tip:'Gap between Tech Boxes after one resolves.'},
  ]},
  {group:'Economy', items:[
    {k:'eco.resupply',  label:'Resupply cost',     min:0,   max:300, step:5,  int:1, unit:'cr', tip:'Credits to resupply (heal + refill ammo) at the terminal.'},
    {k:'harv.respawn',  label:'Harvester respawn', min:2,   max:60,  step:1,  int:1, unit:'s',  tip:'Seconds before a destroyed harvester respawns.'},
    {k:'harv.load',     label:'Harvester load',    min:0.5, max:15,  step:0.5,       unit:'s',  tip:'Seconds the harvester spends loading at the field.'},
    {k:'harv.dock',     label:'Harvester dock',    min:0.5, max:15,  step:0.5,       unit:'s',  tip:'Seconds the harvester spends unloading at the dock.'},
  ]},
];
function _rulesGet(k){ const p=k.split('.'); return _rules[p[0]][p[1]]; }
function _rulesSet(k,v){ const p=k.split('.'); _rules[p[0]][p[1]]=v; }
function _rulesFmt(it,v){ return (it.int?Math.round(v):v)+(it.unit?(' '+it.unit):''); }
function edBuildRules(){ edRulesBody.innerHTML='';
  for(const grp of RULES_SCHEMA){ const h=document.createElement('div'); h.className='edWGroup'; h.textContent=grp.group; edRulesBody.appendChild(h);
    for(const it of grp.items){ const row=document.createElement('div'); row.className='edRRow'; row.setAttribute('data-tip',it.tip);
      const head=document.createElement('div'); head.className='edRHead';
      const lab=document.createElement('span'); lab.className='edRLab'; lab.textContent=it.label; head.appendChild(lab);
      const val=document.createElement('span'); val.className='edRVal'; val.textContent=_rulesFmt(it,_rulesGet(it.k)); head.appendChild(val);
      row.appendChild(head);
      const inp=document.createElement('input'); inp.className='edWInp range'; inp.type='range'; inp.min=it.min; inp.max=it.max; inp.step=it.step; inp.value=_rulesGet(it.k);
      const apply=()=>{ const v=it.int?Math.round(+inp.value):+inp.value; _rulesSet(it.k,v); val.textContent=_rulesFmt(it,v); applyRules(_rules); };
      inp.addEventListener('pointerdown',()=>edPushUndo('rules')); inp.addEventListener('keydown',()=>edPushUndo('rules')); inp.addEventListener('input',apply); inp.addEventListener('change',apply);
      row.appendChild(inp); edRulesBody.appendChild(row); _edRulesCtl[it.k]={inp,val,it}; } } }
function edSyncRulesPanel(){ for(const k in _edRulesCtl){ const c=_edRulesCtl[k], v=_rulesGet(k); c.inp.value=v; c.val.textContent=_rulesFmt(c.it,v); } }
function edToggleRules(){ if(typeof edSetSel==='function')edSetSel([]); const el=document.getElementById('edRules'); if(el){ if(el.scrollIntoView)el.scrollIntoView({block:'nearest'}); el.classList.remove('edFlash'); void el.offsetWidth; el.classList.add('edFlash'); } }
let _bkSpec=Object.assign({preset:'normandy',seed:1}, BK.NAMED.timberHouse);
let _bkPreview=null, _bkOpen=false, _bkDirtyRAF=0, _bkSeedInp=null;
const _bkTmpV=new THREE.Vector3();
const _edMaker=document.getElementById('edMaker'), _edMakerBody=document.getElementById('edMakerBody');
const _bkPickers={}, _bkSliders={}, _bkValSpans={};
function _bkPlaceAt(x,z){ return spawnKitBuilding(_bkSpec,x,z,0).grp.userData.editSel; }
function _bkSyncPlaceBtn(){ const b=document.getElementById('edMkPlaceBtn'); if(b)b.classList.toggle('on',edTool==='bmaker'); }
function _bkWorkbenchPt(){ const cam=(typeof edCam==='function')?edCam():camera; cam.getWorldDirection(_bkTmpV);
  if(Math.abs(_bkTmpV.y)>1e-3){ const t=-cam.position.y/_bkTmpV.y; if(t>0&&t<400) return {x:cam.position.x+_bkTmpV.x*t, z:cam.position.z+_bkTmpV.z*t}; }
  return {x:(typeof mapSpawn!=='undefined'&&mapSpawn?mapSpawn.x:0)+6, z:(typeof mapSpawn!=='undefined'&&mapSpawn?mapSpawn.z:0)}; }
function _bkClearPreview(){ if(_bkPreview){ if(typeof clDispose==='function')clDispose(_bkPreview); else scene.remove(_bkPreview); _bkPreview=null; } }
function _bkBuildPreview(){ _bkClearPreview(); const built=BK.build(_bkSpec); _bkPreview=built.group; const p=_bkWorkbenchPt();
  _bkPreview.position.set(p.x,0,p.z); scene.add(_bkPreview); _bkPreview.updateMatrixWorld(true); if(typeof stylizeOne==='function')stylizeOne(_bkPreview);
  _bkPreview.traverse(o=>{ if(o.userData&&o.userData.isOutline&&o.material&&o.material.uniforms&&o.material.uniforms.ink)o.material.uniforms.ink.value.set(0x4c8dff); }); }
function _bkQueuePreview(){ if(_bkDirtyRAF)return; _bkDirtyRAF=requestAnimationFrame(()=>{ _bkDirtyRAF=0; if(_bkOpen)_bkBuildPreview(); }); }
function _bkSyncControls(){ for(const k in _bkPickers)_bkPickers[k].value=_bkSpec[k];
  for(const k in _bkSliders){ _bkSliders[k].value=_bkSpec[k]; _bkValSpans[k].textContent=(+_bkSpec[k]).toFixed(1); }
  if(_bkSeedInp)_bkSeedInp.value=String(_bkSpec.seed||1); }
function _bkRoll(seed){ const rolled=BK.roll(_bkSpec.preset||'normandy', seed>>>0); _bkSpec=Object.assign(_bkSpec, rolled); _bkSyncControls(); _bkQueuePreview(); }
function _bkBuildPanel(){ _edMakerBody.innerHTML=''; for(const k in _bkPickers)delete _bkPickers[k]; for(const k in _bkSliders)delete _bkSliders[k]; for(const k in _bkValSpans)delete _bkValSpans[k];
  const grp=(t)=>{ const d=document.createElement('div'); d.className='edMkGroup'; d.textContent=t; _edMakerBody.appendChild(d); };
  const row=()=>{ const r=document.createElement('div'); r.className='edMkRow'; _edMakerBody.appendChild(r); return r; };
  const lab=(t)=>{ const l=document.createElement('span'); l.className='edMkLab'; l.textContent=t; return l; };
  grp('Region'); { const r=row(); const sel=document.createElement('select'); sel.className='edMkSel';
    for(const id in BK.PRESETS){ const o=document.createElement('option'); o.value=id; o.textContent=BK.PRESETS[id].label; sel.appendChild(o); }
    sel.value=_bkSpec.preset||'normandy'; sel.addEventListener('change',()=>{ _bkSpec.preset=sel.value; }); r.appendChild(lab('Preset')); r.appendChild(sel); }
  grp('Parts'); for(const key in BK.PARTS){ const P=BK.PARTS[key], r=row(); const sel=document.createElement('select'); sel.className='edMkSel';
    P.opts.forEach(op=>{ const o=document.createElement('option'); o.value=op[0]; o.textContent=op[1]; sel.appendChild(o); });
    sel.value=_bkSpec[key]; sel.addEventListener('change',()=>{ _bkSpec[key]=sel.value; _bkQueuePreview(); }); _bkPickers[key]=sel; r.appendChild(lab(P.label)); r.appendChild(sel); }
  grp('Dimensions'); BK.DIMS.forEach(dm=>{ const key=dm[0], r=row(); const rng=document.createElement('input'); rng.type='range'; rng.className='edMkRange'; rng.min=dm[2]; rng.max=dm[3]; rng.step=dm[4]; rng.value=_bkSpec[key];
    const val=document.createElement('span'); val.className='edMkVal'; val.textContent=(+_bkSpec[key]).toFixed(1);
    rng.addEventListener('input',()=>{ _bkSpec[key]=+rng.value; val.textContent=(+rng.value).toFixed(1); _bkQueuePreview(); }); _bkSliders[key]=rng; _bkValSpans[key]=val; r.appendChild(lab(dm[1])); r.appendChild(rng); r.appendChild(val); });
  grp('Variation'); { const r=row(); _bkSeedInp=document.createElement('input'); _bkSeedInp.className='edMkSeed'; _bkSeedInp.type='text'; _bkSeedInp.value=String(_bkSpec.seed||1);
    _bkSeedInp.addEventListener('change',()=>{ const v=parseInt(_bkSeedInp.value,10); if(isFinite(v))_bkRoll(v>>>0); }); r.appendChild(lab('Seed')); r.appendChild(_bkSeedInp); }
  { const br=document.createElement('div'); br.className='edMkBtnRow'; const dice=document.createElement('div'); dice.className='edMkBtn primary'; dice.textContent='⚅ Dice'; dice.title='Roll a new in-preset variation from a random seed';
    dice.addEventListener('click',()=>_bkRoll((Math.random()*4294967295)>>>0)); br.appendChild(dice); _edMakerBody.appendChild(br); }
  { const br=document.createElement('div'); br.className='edMkBtnRow'; const pb=document.createElement('div'); pb.className='edMkBtn'; pb.id='edMkPlaceBtn'; pb.textContent='+ Place mode'; pb.title='Toggle, then click the map to stamp this building at the cursor';
    pb.addEventListener('click',()=>{ if(typeof setEdTool==='function')setEdTool(edTool==='bmaker'?'select':'bmaker'); _bkSyncPlaceBtn(); }); br.appendChild(pb); _edMakerBody.appendChild(br); }
  _bkSyncPlaceBtn();
}
function _bkCloseMaker(){ const note=_edMaker&&_edMaker.querySelector('.edWNote'), body=document.getElementById('edMakerBody'); if(note)note.style.display='none'; if(body)body.style.display='none'; if(_edMaker)_edMaker.classList.add('collapsed'); const b=document.getElementById('edMakerBtn'); if(b)b.classList.remove('on'); _bkOpen=false; _bkClearPreview(); if(edTool==='bmaker'&&typeof setEdTool==='function')setEdTool('select'); }
function _bkToggle(){ if(_bkOpen){ _bkCloseMaker(); return; }
  const note=_edMaker&&_edMaker.querySelector('.edWNote'), body=document.getElementById('edMakerBody'); if(note)note.style.display=''; if(body)body.style.display=''; if(_edMaker)_edMaker.classList.remove('collapsed'); const b=document.getElementById('edMakerBtn'); if(b)b.classList.add('on'); _bkOpen=true; _bkBuildPanel(); _bkBuildPreview(); }
const edUndoStack=[], edRedoStack=[]; let _edDragSnapped=false, _edLastTag=null, _edLastPushT=0; const _EDUNDO_MAX=60, _EDCO_WIN=1200;
function edUpdateUndoBtns(){ const u=document.getElementById('edUndo'), r=document.getElementById('edRedo'); if(u)u.classList.toggle('off',!edUndoStack.length); if(r)r.classList.toggle('off',!edRedoStack.length); }
function edPushUndo(tag){ const now=performance.now(); if(tag&&tag===_edLastTag&&(now-_edLastPushT)<_EDCO_WIN){ _edLastPushT=now; return; } edRedoStack.length=0; edUndoStack.push(JSON.stringify(serializeMap())); if(edUndoStack.length>_EDUNDO_MAX)edUndoStack.shift(); _edLastTag=tag||null; _edLastPushT=now; edUpdateUndoBtns(); }
function _edApplySnap(str){ const def=JSON.parse(str); buildMap(def); player.position.copy(mapSpawn); player.userData.editSel={kind:'spawn'}; edSetSel([]); edSyncWorldPanel(); }
function edUndoDo(){ if(!edUndoStack.length)return; edRedoStack.push(JSON.stringify(serializeMap())); _edApplySnap(edUndoStack.pop()); _edLastTag=null; edUpdateUndoBtns(); }
function edRedoDo(){ if(!edRedoStack.length)return; edUndoStack.push(JSON.stringify(serializeMap())); _edApplySnap(edRedoStack.pop()); _edLastTag=null; edUpdateUndoBtns(); }
function edStatusSync(){ const se=document.getElementById('edStatSel'); if(se)se.textContent=String((typeof edMulti!=='undefined'&&edMulti)?edMulti.length:0);
  const te=document.getElementById('edStatTool'); if(te){ const nm={select:'Select',bmaker:'Building Maker',measure:'Measure'}; te.textContent=nm[edTool]||(edTool?('Place \u00b7 '+edTool):'Select'); } }
function edSetLibMode(m){ const gen=(m==='gen'); _edLibMode=m;
  ['edLibTabs','edLibSearch','edPalette'].forEach(id=>{ const el=document.getElementById(id); if(el)el.style.display=gen?'none':''; });
  const g=document.getElementById('edGenerators'); if(g)g.style.display=gen?'flex':'none';
  const rl=document.getElementById('edRail'); if(rl)rl.querySelectorAll('.edRailBtn[data-group="lib"]').forEach(b=>b.classList.toggle('on',b.dataset.mode===(gen?'gen':'place')));
  if(gen){ if(typeof _bkToggle==='function'&&!_bkOpen)_bkToggle(); }
  else if(typeof _bkCloseMaker==='function'&&_bkOpen)_bkCloseMaker(); }
function edBuildRecent(){ const wrap=document.getElementById('edRecentCards'), empty=document.getElementById('edRecentEmpty'); if(!wrap)return; wrap.innerHTML=''; if(!_edRecent.length){ if(empty)empty.style.display='block'; return; } if(empty)empty.style.display='none'; _edRecent.forEach(function(id){ const orig=document.querySelector('#edPalette .edTabPanel:not([data-cat="recent"]) .edTool[data-tool="'+id+'"]'); if(!orig)return; const c=orig.cloneNode(true); c.classList.remove('active'); c.addEventListener('click',function(){ setEdTool(c.dataset.tool); }); wrap.appendChild(c); }); }
const edInsp=document.getElementById('edInsp'), edInspBody=edInsp.querySelector('.edIBody'), edInspKind=document.getElementById('edIKind');
let _edInspRef=undefined, _edInspHidden=false, _edLibCollapsed=false, _edLibMode='place'; const _SIZEABLE={crate:1,wall:1,path:1,platform:1}, _SCALEABLE={building:1,barracks:1};
function edKindLabel(s){ switch(s.kind){ case 'crate':return 'Crate'; case 'wall':return 'Wall'; case 'path':return 'Path / Road'; case 'building':return ((s.ref&&s.ref.team==='friend')?'Friendly ':'Hostile ')+((s.ref&&s.ref.type)||'building'); case 'barracks':return 'Barracks'; case 'platform':return 'Platform'; case 'ramp':return 'Ramp'; case 'cover':return (s.ref&&BK.COVER[s.ref.type]?BK.COVER[s.ref.type].label:'Cover'); case 'tree':return (s.ref&&BK.TREES[s.ref.kind]?BK.TREES[s.ref.kind].label:'Tree'); case 'prop':return (s.ref&&BK.PROPS[s.ref.kind]?BK.PROPS[s.ref.kind].label:'Prop'); case 'spawn':return 'Player Spawn'; case 'resupply':return 'Resupply'; } return s.kind; }
function _edRotCur(s){ if(!s||s.kind==='spawn'||s.kind==='platform'||s.kind==='enemyspawn')return null; if(s.kind==='barracks')return s.ref.rot||0; const m=edSelTarget(s); return m?m.rotation.y:null; }
function _edRow(label){ const r=document.createElement('div'); r.className='edIRow'; const l=document.createElement('span'); l.className='edILab'; l.textContent=label; r.appendChild(l); return r; }
function _edNum(val,step){ const i=document.createElement('input'); i.type='number'; i.className='edIInp'; i.step=step||0.5; i.value=String(Math.round(val*100)/100); return i; }
function edMoveSelBy(dx,dz){ if(!dx&&!dz)return; edPushUndo('insp'); for(const s of edMulti)edMoveBy(s,dx,dz); edRefreshBoxes(); }
function edRotateSelTo(deg){ const cur=_edRotCur(edSel); if(cur===null)return; const d=deg-cur*180/Math.PI; if(Math.abs(d)<1e-4)return; const da=d*Math.PI/180; edPushUndo('insp'); for(const s of edMulti)edRotateOne(s,da); edRefreshBoxes(); }
function edRotateSnap(){ const cur=_edRotCur(edSel); if(cur===null)return; const snapped=Math.round((cur*180/Math.PI)/edRotStep)*edRotStep; edRotateSelTo(snapped); edSyncInspector(); }
function edSetHP(sel,hp){ if(!sel||sel.kind!=='building'||!sel.ref)return; hp=Math.max(1,Math.round(hp)); const b=sel.ref; if(b.maxHealth===hp&&b.health===hp)return; edPushUndo('insp'); b.maxHealth=hp; b.health=hp; if(b.bar&&b.bar.fg){ b.bar.fg.scale.x=1; b.bar.fg.position.x=0; } }
function edResize(sel,w,h,d){ const o=sel.ref, isPath=(sel.kind==='path'); w=Math.max(0.2,w); d=Math.max(0.2,d); if(!isPath)h=Math.max(0.2,h); edPushUndo('insp');
  const ng=isPath?new THREE.BoxGeometry(w,0.06,d):new THREE.BoxGeometry(w,h,d), old=o.mesh.geometry;
  o.mesh.geometry=ng; ensureOutlineNormals(ng); o.mesh.children.forEach(ch=>{ if(ch.userData&&ch.userData.isOutline)ch.geometry=ng; }); if(old&&old!==ng)old.dispose();
  if(isPath){ o.w=w; o.d=d; o.mesh.position.y=0.03; } else { o.w=w; o.h=h; o.d=d; o.mesh.position.y=h/2; if(o.box)o.box.setFromObject(o.mesh); }
  edRefreshBoxes(); rebuildLOS();
}
function edReshapeRamp(sel,w,d,h0,h1){ const o=sel.ref; w=Math.max(0.5,w); d=Math.max(0.5,d); h0=Math.max(0,h0); h1=Math.max(0.1,h1); edPushUndo('insp');
  const ng=rampGeometry(w,d,h0,h1), old=o.mesh.geometry; o.mesh.geometry=ng; ensureOutlineNormals(ng); o.mesh.children.forEach(ch=>{ if(ch.userData&&ch.userData.isOutline)ch.geometry=ng; }); if(old&&old!==ng)old.dispose();
  o.w=w; o.d=d; o.h0=h0; o.h1=h1; if(o.box)o.box.setFromObject(o.mesh); edRefreshBoxes(); rebuildLOS();
}
function edBuildInspector(s){
  edInspBody.innerHTML=''; edInspKind.textContent=(edMulti.length>1?(edMulti.length+' selected'):edKindLabel(s));
  const a=edAnchor(s)||{x:0,z:0};
  const rx=_edRow('X'), ix=_edNum(a.x,0.5); rx.appendChild(ix); edInspBody.appendChild(rx);
  const rz=_edRow('Z'), iz=_edNum(a.z,0.5); rz.appendChild(iz); edInspBody.appendChild(rz);
  ix.addEventListener('change',()=>{ const v=ix.valueAsNumber; if(!isFinite(v))return; const a2=edAnchor(edSel); if(a2)edMoveSelBy(v-a2.x,0); });
  iz.addEventListener('change',()=>{ const v=iz.valueAsNumber; if(!isFinite(v))return; const a2=edAnchor(edSel); if(a2)edMoveSelBy(0,v-a2.z); });
  const rc=_edRotCur(s);
  if(rc!==null){ const rr=_edRow('Rotation°'), ir=_edNum(rc*180/Math.PI,1); rr.appendChild(ir); edInspBody.appendChild(rr);
    ir.addEventListener('change',()=>{ const v=ir.valueAsNumber; if(!isFinite(v))return; edRotateSelTo(v); });
    const rsnap=_edRow(''); rsnap.classList.add('snaprow'); const grp=document.createElement('div'); grp.className='edIBtnRow';
    const mkR=(txt,tip,fn)=>{ const b=document.createElement('div'); b.className='edBtn edBtnSm edIStep'; b.textContent=txt; b.title=tip; b.addEventListener('click',fn); grp.appendChild(b); };
    mkR('−'+edRotStep+'°','Rotate −'+edRotStep+'° (snap step)',()=>{ edRotateSel(-1); });
    mkR('⊕ snap','Snap rotation to nearest '+edRotStep+'°',()=>{ edRotateSnap(); });
    mkR('+'+edRotStep+'°','Rotate +'+edRotStep+'° (snap step)',()=>{ edRotateSel(1); });
    rsnap.appendChild(grp); edInspBody.appendChild(rsnap); }
  if(edMulti.length===1 && _SIZEABLE[s.kind]){ const o=s.ref, isPath=(s.kind==='path');
    const rs=document.createElement('div'); rs.className='edIRow triple';
    const addAx=(letter,val,title)=>{ const ax=document.createElement('span'); ax.className='edIAx'; ax.textContent=letter; const inp=_edNum(val,0.5); inp.title=title; rs.appendChild(ax); rs.appendChild(inp); return inp; };
    const iw=addAx('W',o.w,'Width'); const ih=isPath?null:addAx('H',o.h,'Height'); const id=addAx('D',o.d,'Depth');
    edInspBody.appendChild(rs);
    const doResize=()=>{ const w=iw.valueAsNumber, d=id.valueAsNumber, hh=ih?ih.valueAsNumber:o.h; if(!isFinite(w)||!isFinite(d)||(ih&&!isFinite(hh)))return; edResize(s,w,hh,d); };
    iw.addEventListener('change',doResize); if(ih)ih.addEventListener('change',doResize); id.addEventListener('change',doResize);
  }
  if(edMulti.length===1 && s.kind==='resupply' && termMesh){
    const rsr=document.createElement('div'); rsr.className='edIRow triple';
    const addAx=(letter,val,title)=>{ const ax=document.createElement('span'); ax.className='edIAx'; ax.textContent=letter; const inp=_edNum(val,0.5); inp.title=title; rsr.appendChild(ax); rsr.appendChild(inp); return inp; };
    const iw=addAx('W',termMesh.scale.x*1.4,'Width'), ih=addAx('H',termMesh.scale.y*1.6,'Height'), id=addAx('D',termMesh.scale.z*1,'Depth');
    edInspBody.appendChild(rsr);
    const doR=()=>{ const w=iw.valueAsNumber,h=ih.valueAsNumber,d=id.valueAsNumber; if(!isFinite(w)||!isFinite(h)||!isFinite(d))return; edResizeResupply(w,h,d); };
    iw.addEventListener('change',doR); ih.addEventListener('change',doR); id.addEventListener('change',doR);
  }
  if(edMulti.length===1 && s.kind==='platform'){ const o=s.ref;
    const rp=_edRow('Preset h'); const grp=document.createElement('div'); grp.className='edIBtnRow';
    [['Step',1.2],['Low',2.5],['Mid',4],['High',6]].forEach(pr=>{ const b=document.createElement('div'); b.className='edBtn edBtnSm edIStep'; b.textContent=pr[0]; b.title='Set height to '+pr[1]+'m'; b.addEventListener('click',()=>{ edResize(s,o.w,pr[1],o.d); edSyncInspector(); }); grp.appendChild(b); });
    rp.appendChild(grp); edInspBody.appendChild(rp);
  }
  if(edMulti.length===1 && s.kind==='ramp'){ const o=s.ref;
    const mkRow=(label,val,title,setter)=>{ const r=_edRow(label); const inp=_edNum(val,0.5); inp.title=title; r.appendChild(inp); edInspBody.appendChild(r); inp.addEventListener('change',()=>{ const v=inp.valueAsNumber; if(isFinite(v))setter(v); }); };
    mkRow('Run (len)',o.w,'Length; the slope rises along this axis', v=>edReshapeRamp(s,v,o.d,o.h0,o.h1));
    mkRow('Width',o.d,'Ramp width', v=>edReshapeRamp(s,o.w,v,o.h0,o.h1));
    mkRow('Low h',o.h0,'Height at the low end', v=>edReshapeRamp(s,o.w,o.d,v,o.h1));
    mkRow('High h',o.h1,'Height at the high end', v=>edReshapeRamp(s,o.w,o.d,o.h0,v));
    const rp=_edRow('High →'); const grp=document.createElement('div'); grp.className='edIBtnRow';
    [['2.5',2.5],['4',4],['6',6]].forEach(pr=>{ const b=document.createElement('div'); b.className='edBtn edBtnSm edIStep'; b.textContent=pr[0]; b.title='Set high end to '+pr[1]+'m'; b.addEventListener('click',()=>{ edReshapeRamp(s,o.w,o.d,o.h0,pr[1]); edSyncInspector(); }); grp.appendChild(b); });
    rp.appendChild(grp); edInspBody.appendChild(rp);
  }
  if(edMulti.length===1 && (s.kind==='building'||s.kind==='barracks')){ const rt=_edRow('Team'); const seg=document.createElement('div'); seg.className='edIBtnRow';
    const mkT=(label,team)=>{ const b=document.createElement('div'); b.className='edBtn edBtnSm edIStep edITeam team_'+team+(s.ref.team===team?' on':''); b.textContent=label; b.title='Set faction to '+label; b.addEventListener('click',()=>{ if(s.ref.team!==team)edSetTeam(team); }); seg.appendChild(b); };
    mkT('Friendly','friend'); mkT('Hostile','hostile'); rt.appendChild(seg); edInspBody.appendChild(rt); }
  if(edMulti.length===1 && _SCALEABLE[s.kind]){ const cur=(s.ref.scl||1);
    const rsc=_edRow('Scale ×'), isc=_edNum(cur,0.05); isc.title='Uniform scale'; rsc.appendChild(isc);
    const mkS=(txt,delta)=>{ const b=document.createElement('div'); b.className='edBtn edBtnSm edIStep edIStepSq'; b.textContent=txt; b.title=(delta>0?'+':'')+delta+' scale'; b.addEventListener('click',()=>{ edScaleSel(s,(s.ref.scl||1)+delta); edSyncInspector(); }); return b; };
    rsc.appendChild(mkS('−',-0.1)); rsc.appendChild(mkS('+',0.1)); edInspBody.appendChild(rsc);
    isc.addEventListener('change',()=>{ const v=isc.valueAsNumber; if(!isFinite(v))return; edScaleSel(s,v); }); }
  if(edMulti.length===1 && s.kind==='building'){ const b=s.ref;
    const rhp=_edRow('HP'), ihp=_edNum(b.maxHealth,10); ihp.title='Building max health (sets current to full)'; rhp.appendChild(ihp);
    const mkH=(txt,delta)=>{ const bt=document.createElement('div'); bt.className='edBtn edBtnSm edIStep edIStepSq'; bt.textContent=txt; bt.title=(delta>0?'+':'')+delta+' HP'; bt.addEventListener('click',()=>{ edSetHP(s,(s.ref.maxHealth||0)+delta); edSyncInspector(); }); return bt; };
    rhp.appendChild(mkH('\u2212',-25)); rhp.appendChild(mkH('+',25)); edInspBody.appendChild(rhp);
    ihp.addEventListener('change',()=>{ const v=ihp.valueAsNumber; if(!isFinite(v))return; edSetHP(s,v); }); }
  if(edMulti.some(x=>!!edDescribe(x))){ // Array / line duplicate
    const ra=_edRow('Array ×'); ra.classList.add('arr');
    const inN=document.createElement('input'); inN.type='number'; inN.className='edIInp edINarrow'; inN.step=1; inN.min=2; inN.value='3'; inN.title='Total count (incl. original)';
    const axSel=document.createElement('select'); axSel.className='edISel'; axSel.title='Direction'; ['+X','-X','+Z','-Z'].forEach(v=>{ const op=document.createElement('option'); op.value=v; op.textContent=v; axSel.appendChild(op); });
    const mk=document.createElement('div'); mk.className='edBtn edBtnSm'; mk.textContent='Make';
    ra.appendChild(inN); ra.appendChild(axSel); ra.appendChild(mk); edInspBody.appendChild(ra);
    const rg=_edRow('Gap m'); const inG=_edNum(6,0.5); inG.title='Spacing between copies (m)'; rg.appendChild(inG); edInspBody.appendChild(rg);
    mk.addEventListener('click',()=>{ const n=Math.round(inN.valueAsNumber), g=inG.valueAsNumber; if(!isFinite(n)||!isFinite(g))return; edArraySel(n,g,axSel.value); });
  }
  if(edMulti.some(x=>!!edDescribe(x))){ const rsv=document.createElement('div'); rsv.className='edIRow save'; const btn=document.createElement('div'); btn.className='edBtn edBtnSm edISave'; btn.textContent='⊕ Save as prefab'; rsv.appendChild(btn); edInspBody.appendChild(rsv); btn.addEventListener('click',async()=>{ const nm=await edSavePrefab(); if(nm){ btn.textContent='Saved · '+nm; setTimeout(()=>{ btn.textContent='⊕ Save as prefab'; },1400); } }); }
}
function edSyncInspector(){ if(typeof edStatusSync==='function')edStatusSync();
  if(_edInspHidden){ edInsp.style.display='none'; return; }
  const _empty=document.getElementById('edIEmpty');
  if(!edMulti.length){ edInsp.style.display='flex'; edInspBody.style.display='none'; if(_empty)_empty.style.display='flex'; edInspKind.textContent='World & Match Rules'; _edInspRef=undefined; if(typeof edSyncWorldPanel==='function')edSyncWorldPanel(); return; }
  const s=edSel, ref=s.ref||s.kind;
  if(document.activeElement&&edInsp.contains(document.activeElement)&&ref===_edInspRef) return;
  edInsp.style.display='flex'; edInspBody.style.display=''; if(_empty)_empty.style.display='none'; _edInspRef=ref; edBuildInspector(s);
}
function edApplyGridCols(){ const g=document.getElementById('edGrid'); if(!g)return; g.style.gridTemplateColumns='48px '+(_edLibCollapsed?'0':'264px')+' 1fr '+(_edInspHidden?'0':'300px'); }
function edToggleInsp(){ _edInspHidden=!_edInspHidden; edApplyGridCols(); if(_edInspHidden){ edInsp.style.display='none'; } else if(typeof edSyncInspector==='function'){ edSyncInspector(); } const b=document.getElementById('edWRBtn'); if(b)b.classList.toggle('on',!_edInspHidden); }

function edLeaveToMenu(){
  if(_edGizmo)_edGizmo.visible=false; if(_edHandles)_edHandles.visible=false; if(_edRing)_edRing.visible=false;
  ovHideAll();
  edRoot.style.display='none';
  edSetSel([]); if(edGhost)edGhost.visible=false; edPreview=false; edPvBanner.style.display='none'; edPalette.style.opacity='';
}

/* Called from top-level code (enterEditor, openMaps, the inspector's Save-as-prefab button), so they must live at the
   top level: declared inside bootEditor() they were invisible there and every call threw a ReferenceError. */
/* Status-bar note (Maps > Load reports the duplicates dedupeMapDef dropped). The hint slot is static markup, so its text is
   read once. Every call first cancels a pending restore, so a clean Load clears a stale note at once; a note lasts 8 s. */
function edFlashHint(msg){ const h=document.getElementById('edStatHint'); if(!h)return; if(h._orig==null)h._orig=h.textContent;
  clearTimeout(h._t); h._t=0; h.textContent=msg||h._orig; if(msg)h._t=setTimeout(()=>{ h.textContent=h._orig; h._t=0; },8000); }
async function refreshMapList(){ const names=await MapStore.list(); edMapList.innerHTML='';
  if(!names.length){ const e=document.createElement('div'); e.className='edEmpty'; e.textContent='No saved maps yet.'; edMapList.appendChild(e); return; }
  names.forEach(name=>{ const row=document.createElement('div'); row.className='edMapRow';
    const nm=document.createElement('span'); nm.className='edMapName'; nm.textContent=name;
    const load=document.createElement('div'); load.className='edBtn edBtnSm'; load.textContent='Load';
    const del=document.createElement('div'); del.className='edBtn edBtnSm edBtnDel'; del.textContent='Delete';
    load.onclick=async()=>{ const saved=await MapStore.load(name); if(!saved)return;
      const dd=dedupeMapDef(saved), parts=[];
      if(dd.droppedTrees)parts.push(dd.droppedTrees+' duplicate tree'+(dd.droppedTrees===1?'':'s'));
      if(dd.droppedProps)parts.push(dd.droppedProps+' duplicate prop'+(dd.droppedProps===1?'':'s'));
      buildMap(dd.def); player.position.copy(mapSpawn); player.userData.editSel={kind:'spawn'}; edSetSel([]); edUndoStack.length=0; edRedoStack.length=0; _edLastTag=null; edUpdateUndoBtns(); edSyncWorldPanel(); document.getElementById('edMapName').textContent=name; edMapsModal.style.display='none';
      edFlashHint(parts.length?'Removed '+parts.join(' and ')+' (exact '+(dd.droppedTrees+dd.droppedProps===1?'copy':'copies')+'). Save to keep this.':''); };
    let armed=false, tmr=null;
    del.onclick=async()=>{ if(!armed){ armed=true; del.textContent='Confirm?'; del.classList.add('arm'); clearTimeout(tmr); tmr=setTimeout(()=>{armed=false;del.textContent='Delete';del.classList.remove('arm');},2200); return; } await MapStore.del(name); refreshMapList(); };
    row.append(nm,load,del); edMapList.appendChild(row);
  });
}
async function edSavePrefab(){ const items=edMulti.map(edDescribe).filter(Boolean); if(!items.length)return null;
  let sx=0,sz=0,n=0; for(const s of edMulti){ const a=edAnchor(s); if(a){ sx+=a.x; sz+=a.z; n++; } } if(!n)return null; const cx=sx/n, cz=sz/n;
  items.forEach(d=>{ d.rx=(d.ax||0)-cx; d.rz=(d.az||0)-cz; });
  const names=await PrefabStore.list(); let i=names.length+1, name='Prefab '+i; while(names.indexOf(name)>=0){ i++; name='Prefab '+i; }
  await PrefabStore.save(name,{items}); await refreshPrefabList(); return name;
}
async function refreshPrefabList(){ const list=document.getElementById('edPrefabList'); if(!list)return; const names=await PrefabStore.list(); list.innerHTML='';
  if(!names.length){ const e=document.createElement('div'); e.className='edPrefabEmpty'; e.textContent='No prefabs saved yet.'; list.appendChild(e); return; }
  names.forEach(name=>{ const row=document.createElement('div'); row.className='edPrefab'; row.dataset.prefab=name;
    const nm=document.createElement('span'); nm.className='edPrefabNm'; nm.textContent=name;
    const del=document.createElement('span'); del.className='edPrefabX'; del.textContent='✕'; del.title='Delete prefab';
    row.append(nm,del); list.appendChild(row);
    row.addEventListener('click',async(ev)=>{ if(ev.target===del)return; const data=await PrefabStore.load(name); if(data&&data.items&&data.items.length)edSelectPrefab(name,data.items,row); });
    let armed=false,tmr=null;
    del.addEventListener('click',async(ev)=>{ ev.stopPropagation(); if(!armed){ armed=true; del.textContent='✓?'; clearTimeout(tmr); tmr=setTimeout(()=>{ armed=false; del.textContent='✕'; },2000); return; } await PrefabStore.del(name); if(edTool==='prefab')setEdTool('select'); refreshPrefabList(); });
  });
}

function bootEditor(){
document.getElementById('mEditor').addEventListener('click',enterEditor);
addEventListener('keydown',e=>{ if(mode==='charlab' && e.altKey && (e.key==='z'||e.key==='Z')){ e.preventDefault(); clUndoPop(); } });
(function(){ const cv=renderer.domElement;
  cv.addEventListener('pointerdown', e=>{ if(mode!=='charlab')return; clDrag=true; _clLast.x=e.clientX; _clLast.y=e.clientY; cv.style.cursor='grabbing'; });
  cv.addEventListener('pointermove', e=>{ if(mode!=='charlab'||!clDrag)return; const dx=e.clientX-_clLast.x, dy=e.clientY-_clLast.y; _clLast.x=e.clientX; _clLast.y=e.clientY; clAz-=dx*0.008; clEl=Math.max(-1.25,Math.min(1.35,clEl+dy*0.008)); });
  window.addEventListener('pointerup', ()=>{ if(clDrag){ clDrag=false; if(mode==='charlab')cv.style.cursor='grab'; } });
  const _b=(id,fn)=>{ const el=document.getElementById(id); if(el)el.onclick=fn; };
  _b('dvCharLab',enterCharLab); _b('clBack',exitCharLab); _b('clBack2',exitCharLab); _b('clExport',clExport); _b('clImport',clImport); _b('clReset',clReset);
  _b('clSpinBtn',()=>{ clSpin=!clSpin; const sb=document.getElementById('clSpinBtn'); if(sb){ sb.textContent='Spin: '+(clSpin?'On':'Off'); sb.classList.toggle('on',clSpin);} });
  _b('clResetView',()=>{ clAz=0; clEl=0.12; clDist=5.6; clCam(); });
  _b('clResetProps',clResetProps); _b('clResetColors',clResetColors);
  _b('clSexM',()=>clSetSex('m')); _b('clSexF',()=>clSetSex('f'));
  _b('clFacSov',()=>clSetFaction('Soviets')); _b('clFacAll',()=>clSetFaction('Allies')); _b('clFacEmp',()=>clSetFaction('Empire')); _b('clFacElite',()=>clSetFaction('EmpireElite'));
  _b('clHeadgear',()=>clCycleSlot('headgear',['none','brodie','m1','cap']));
  _b('clTorso',()=>clCycleSlot('torso',['none','tunic','greatcoat']));
  _b('clLegs',()=>clCycleSlot('legs',['none','trousers']));
  _b('clFeet',()=>clCycleSlot('feet',['none','boots']));
  _b('clWebbing',()=>clCycleSlot('webbing',['none','webbing']));
  _b('clHair',()=>clCycleSlot('hair',['default','none','crop','swept','sidepart','bun','ponytail','mohawk']));
  _b('clFace',()=>clCycleSlot('face',['none','stubble','mustache','goatee','beard']));
  _b('clPoseToggle',clTogglePose); _b('clPoseCarry',()=>clPoseSlot('carry')); _b('clPoseAds',()=>clPoseSlot('ads')); _b('clPoseExport',clExportPose);
  [['dvBoom','dvBoomV'],['dvHeight','dvHeightV'],['dvPitch','dvPitchV'],['dvFov','dvFovV'],['dvMap','dvMapV']].forEach(pair=>{
    const s=document.getElementById(pair[0]), v=document.getElementById(pair[1]); if(!s||!v)return;
    clEditable(v, nv=>{ if(nv==null){ s.dispatchEvent(new Event('input',{bubbles:true})); return; } nv=Math.max(parseFloat(s.min),Math.min(parseFloat(s.max),nv)); s.value=nv; s.dispatchEvent(new Event('input',{bubbles:true})); });
  });
})();
ovGroup.add(ovMeasureG, ovSpawnG, ovSymG, ovCoverG);
renderer.domElement.addEventListener('mousedown',e=>{ if(mode!=='editor'||edPreview||e.button!==1)return; e.preventDefault(); edPanning=true; _edPanPX=e.clientX; _edPanPY=e.clientY; });
addEventListener('mousemove',e=>{ if(!edPanning)return; const k=edCamH*0.98/Math.max(1,innerHeight); const dx=e.clientX-_edPanPX, dy=e.clientY-_edPanPY; _edPanPX=e.clientX; _edPanPY=e.clientY; edCamX-=dx*k; edCamZ-=dy*k; const _ecl=Math.min(230,Math.max(90,Math.max(T.arenaX,T.arenaZ)+30)); edCamX=Math.max(-_ecl,Math.min(_ecl,edCamX)); edCamZ=Math.max(-_ecl,Math.min(_ecl,edCamZ)); });
addEventListener('mouseup',e=>{ if(e.button===1)edPanning=false; });
renderer.domElement.addEventListener('mousedown',e=>{
  if(mode!=='editor'||edPreview||e.button!==0)return;
  if(edTool==='select'){
    if(edGizmoMode==='move'){ const _ax=_edGizmoPick(); if(_ax){ edAxisDrag=_ax; _edAxisPrev=_edAxisRef(_ax); _edGizmoMoved=false; return; } }
    else if(edGizmoMode==='scale'){ if(_edHandlePick()){ _edStartResize(); return; } }
    else if(edGizmoMode==='rotate'){ if(_edRingPick()){ _edStartRotate(); return; } }
  }
  const g=edGround(); if(!g)return;
  if(edTool==='select'){
    raycaster.setFromCamera(mouseNDC,edCam()); const hits=raycaster.intersectObjects(edTargets(),true);
    let sel=null; for(const h of hits){ const s=findEditSel(h.object); if(s){sel=s;break;} }
    if(sel){ if(!edMulti.some(m=>_selEq(m,sel))) edSetSel([sel]); if(edGizmoMode==='rotate'){ _edStartRotate(); } else if(edGizmoMode==='scale'){ _edStartResize(); } else { edGrab=sel; edDragging=true; } }
    else { edMarquee={x0:mx,y0:my}; edMarqueeEl.style.display='block'; edMarqueeEl.style.left=mx+'px'; edMarqueeEl.style.top=my+'px'; edMarqueeEl.style.width='0px'; edMarqueeEl.style.height='0px'; }
  } else if(edTool==='path'||_bkIsCover(edTool)||edTool==='orchard'||edTool==='poprow'){ edPathStart=g; }
  else if(edTool==='measure'){ measClick(g); }
  else if(edTool==='prefab'&&edPrefabActive){ edPushUndo(); const placed=edStampPrefab(edPrefabActive,g.x,g.z); if(edMirror){ for(const s of placed) edMirrorOne(s); } rebuildLOS(); if(placed.length)edSetSel(placed); }
  else { edPushUndo(); const sel=edPlaceAt(g); if(edMirror&&sel){ edMirrorOne(sel); rebuildLOS(); } }
});
renderer.domElement.addEventListener('mousemove',()=>{
  if(mode!=='editor'||edPreview)return;
  if(edAxisDrag){ const now=_edAxisRef(edAxisDrag); if(now!=null){ if(!_edGizmoMoved){edPushUndo('gizmo');_edGizmoMoved=true;} const d=now-_edAxisPrev; if(Math.abs(d)>1e-5){ if(edAxisDrag==='x')for(const s of edMulti)edMoveBy(s,d,0); else if(edAxisDrag==='z')for(const s of edMulti)edMoveBy(s,0,d); else for(const s of edMulti)edMoveYBy(s,d); _edAxisPrev=now; edRefreshBoxes(); } } return; }
  if(edResizeDrag){ const m=edSel?edSelTarget(edSel):null; if(m){ _edHBox.setFromObject(m); _edHBox.getCenter(_edGZC); raycaster.setFromCamera(mouseNDC,edCam()); _edPlane.set(_UP,-_edGZC.y); if(raycaster.ray.intersectPlane(_edPlane,_p1)){ if(!_edResizeMoved){edPushUndo('insp');_edResizeMoved=true;} const hw=Math.abs(_p1.x-_edGZC.x), hd=Math.abs(_p1.z-_edGZC.z), dist=Math.max(0.5,Math.hypot(hw,hd)); _edResizeFootprint(edSel,hw,hd,dist); } } return; }
  if(edRotDrag){ const a=_edGroundAngle(); if(a!=null&&_edRotPrev!=null){ let da=a-_edRotPrev; while(da>Math.PI)da-=2*Math.PI; while(da<-Math.PI)da+=2*Math.PI; if(Math.abs(da)>1e-5){ if(!_edRotMoved){edPushUndo('rot');_edRotMoved=true;} for(const s of edMulti)edRotateOne(s,-da); _edRotPrev=a; edRefreshBoxes(); } } return; }
  if(edDragging&&edSel){ if(!_edDragSnapped){edPushUndo();_edDragSnapped=true;} edDragMove(); }
  else if(edMarquee){ const x=Math.min(edMarquee.x0,mx),y=Math.min(edMarquee.y0,my),w=Math.abs(mx-edMarquee.x0),hh=Math.abs(my-edMarquee.y0); edMarqueeEl.style.left=x+'px'; edMarqueeEl.style.top=y+'px'; edMarqueeEl.style.width=w+'px'; edMarqueeEl.style.height=hh+'px'; }
  else if(edTool==='path'&&edPathStart){ const g=edGround(); if(g&&edGhost){ const w=Math.max(1,Math.abs(g.x-edPathStart.x)), d=Math.max(1,Math.abs(g.z-edPathStart.z)); edGhost.visible=true; edGhost.scale.set(w,0.3,d); edGhost.position.set((g.x+edPathStart.x)/2,0.15,(g.z+edPathStart.z)/2); } }
  else if(_bkIsCover(edTool)&&edPathStart){ const g=edGround(); if(g&&edGhost){ const S=BK.COVER[edTool]||BK.COVER.hedge; const dx=g.x-edPathStart.x, dz=g.z-edPathStart.z, len=Math.max(0.5,Math.hypot(dx,dz)); edGhost.visible=true; edGhost.rotation.y=Math.atan2(-dz,dx); edGhost.scale.set(len,S.height,S.thickness); edGhost.position.set((g.x+edPathStart.x)/2,S.height/2,(g.z+edPathStart.z)/2); } }
  else if(edTool==='orchard'&&edPathStart){ const g=edGround(); if(g){ const x0=Math.min(edPathStart.x,g.x),x1=Math.max(edPathStart.x,g.x),z0=Math.min(edPathStart.z,g.z),z1=Math.max(edPathStart.z,g.z); const nT=_orchardPreview(x0,z0,x1,z1); if(edGhost)edGhost.visible=false; if(edCursorEl)edCursorEl.textContent=nT+' apple tree'+(nT===1?'':'s'); } }
  else if(edTool==='poprow'&&edPathStart){ const g=edGround(); if(g){ const nT=_vegShow(_rowCells(edPathStart.x,edPathStart.z,g.x,g.z), [[edPathStart.x,edPathStart.z],[g.x,g.z]]); if(edGhost)edGhost.visible=false; if(edCursorEl)edCursorEl.textContent=nT+' poplar'+(nT===1?'':'s'); } }
});
addEventListener('mouseup',e=>{
  if(mode!=='editor'||edPreview)return;
  edAxisDrag=null; edResizeDrag=false; _edGizmoMoved=false; _edResizeMoved=false; edRotDrag=false; _edRotMoved=false;
  edDragging=false; edGrab=null; _edDragSnapped=false; if(!(e.target&&e.target.closest&&e.target.closest('#edInsp'))) edSyncInspector();
  if(edMarquee){ const moved=Math.abs(mx-edMarquee.x0)>4||Math.abs(my-edMarquee.y0)>4; if(moved)edSetSel(edMarqueePick(edMarquee.x0,edMarquee.y0,mx,my)); else edSetSel([]); edMarquee=null; edMarqueeEl.style.display='none'; }
  if(edTool==='path'&&edPathStart){ const g=edGround(); if(g){ let w=Math.abs(g.x-edPathStart.x), d=Math.abs(g.z-edPathStart.z), cx=(g.x+edPathStart.x)/2, cz=(g.z+edPathStart.z)/2; if(w<1&&d<1){ w=6;d=6;cx=edPathStart.x;cz=edPathStart.z; } edPushUndo(); addPath(cx,cz,Math.max(1,w),Math.max(1,d),0); } edPathStart=null; if(edGhost)edGhost.visible=false; }
  else if(_bkIsCover(edTool)&&edPathStart){ const g=edGround(); if(g){ const dx=g.x-edPathStart.x, dz=g.z-edPathStart.z; let len=Math.hypot(dx,dz), rot=Math.atan2(-dz,dx), cx=(g.x+edPathStart.x)/2, cz=(g.z+edPathStart.z)/2; if(len<1){ len=4; rot=0; cx=edPathStart.x; cz=edPathStart.z; } edPushUndo(); addCover(edTool,cx,cz,len,rot); } edPathStart=null; if(edGhost){edGhost.visible=false; edGhost.rotation.y=0;} }
  else if(edTool==='orchard'&&edPathStart){ const g=edGround(); if(g){ const x0=Math.min(edPathStart.x,g.x),x1=Math.max(edPathStart.x,g.x),z0=Math.min(edPathStart.z,g.z),z1=Math.max(edPathStart.z,g.z); const cells=_orchardCells(x0,z0,x1,z1); if(cells.length){ edPushUndo(); for(const c of cells)addTree('apple',c.x,c.z,c.sd); rebuildLOS(); } } edPathStart=null; if(edOrchardGhost)edOrchardGhost.visible=false; if(edGhost){edGhost.visible=false; edGhost.rotation.y=0;} }
  else if(edTool==='poprow'&&edPathStart){ const g=edGround(); if(g){ const cells=_rowCells(edPathStart.x,edPathStart.z,g.x,g.z); if(cells.length){ edPushUndo(); for(const c of cells)addTree('poplar',c.x,c.z,c.sd); rebuildLOS(); } } edPathStart=null; if(edOrchardGhost)edOrchardGhost.visible=false; if(edGhost){edGhost.visible=false; edGhost.rotation.y=0;} }
});
document.getElementById('edSnapBtn').addEventListener('click',edToggleSnap);
document.getElementById('edRotBtn').addEventListener('click',edCycleRot); edUpdateRotBtn();
document.getElementById('edMirrorBtn').addEventListener('click',edCycleMirror); edUpdateMirrorBtn();
document.getElementById('edOrthoBtn').addEventListener('click',edToggleOrtho); edUpdateOrthoBtn();
document.getElementById('edPlay').addEventListener('click',editorPlay);
document.getElementById('edExport').addEventListener('click',()=>{ document.getElementById('edJson').value=JSON.stringify(serializeMap(),null,2); document.getElementById('edExportModal').style.display='flex'; });
document.getElementById('edReset').addEventListener('click',()=>{ edPushUndo(); buildMap(DM_Cabana); edSetSel([]); player.position.copy(mapSpawn); player.userData.editSel={kind:'spawn'}; edSyncWorldPanel(); });
document.getElementById('edMenu').addEventListener('click',backToMenu);
document.getElementById('edCloseModal').addEventListener('click',()=>document.getElementById('edExportModal').style.display='none');
document.getElementById('edCopy').addEventListener('click',()=>{ const ta=document.getElementById('edJson'); ta.select(); try{document.execCommand('copy');}catch(e){} try{ if(navigator.clipboard)navigator.clipboard.writeText(ta.value); }catch(e){} const b=document.getElementById('edCopy'); b.textContent='Copied!'; setTimeout(()=>b.textContent='Copy',1200); });
document.querySelectorAll('#edPalette .edTool').forEach(el=>el.addEventListener('click',()=>setEdTool(el.dataset.tool==='measure'&&edTool==='measure'?'select':el.dataset.tool)));
document.querySelectorAll('#edPalette .edToggle').forEach(el=>el.addEventListener('click',()=>ovToggle(el.dataset.ov)));

/* ---- map save/load (persistent storage if available, else in-session) ---- */
document.getElementById('edMaps').addEventListener('click',openMaps);
document.getElementById('edMapsClose').addEventListener('click',()=>edMapsModal.style.display='none');
document.getElementById('edSaveBtn').addEventListener('click',async()=>{ const n=document.getElementById('edSaveName').value.trim(); if(!n)return; await MapStore.save(n,serializeMap()); document.getElementById('edMapName').textContent=n; const b=document.getElementById('edSaveBtn'); b.textContent='Saved!'; setTimeout(()=>b.textContent='Save current',1200); refreshMapList(); });

/* ============ PREFAB STAMPS (editor) ============ */
/* ============ WORLD / ENVIRONMENT PANEL + UNDO/REDO (editor) ============ */
edBuildWorld();
document.getElementById('edWorldBtn').addEventListener('click',edToggleWorld);
document.getElementById('edWorldClose').addEventListener('click',()=>{ edWorldPanel.style.display='none'; const b=document.getElementById('edWorldBtn'); if(b)b.classList.remove('on'); });
document.getElementById('edWorldReset').addEventListener('click',()=>{ edPushUndo(); _env=envMerge(null); applyEnv(_env,{regen:true}); edSyncWorldPanel(); });
/* ============ MATCH RULES PANEL (per-map objective + economy timings) ============ */
edBuildRules();
document.getElementById('edRulesBtn').addEventListener('click',edToggleRules);
document.getElementById('edRulesClose').addEventListener('click',()=>{ edRulesPanel.style.display='none'; const b=document.getElementById('edRulesBtn'); if(b)b.classList.remove('on'); });

// ============================================================================
// BUILDING KIT . EDITOR UI   (self-contained; -> future editor)
//   Dockable panel (mirrors World/Rules). Pickers + sliders are GENERATED from
//   BK.PARTS / BK.DIMS (add a part to the registry -> a control appears here).
//   Workbench preview rebuilds on any change (leak-free via clDispose); Place-mode
//   stamps the current RESOLVED spec at the grid cursor via edPlaceAt('bmaker').
// ============================================================================
document.getElementById('edMakerBtn').addEventListener('click',function(){ if(typeof edSetLibMode==='function')edSetLibMode('gen'); if(!_bkOpen)_bkToggle(); const el=document.getElementById('edMaker'); if(el&&el.scrollIntoView)el.scrollIntoView({block:'nearest'}); });
document.getElementById('edMakerClose').addEventListener('click',_bkCloseMaker);
(function(){ const bp=document.getElementById('edGenerators'), mk=document.getElementById('edMaker'); if(bp&&mk){ mk.classList.add('edMakerDock','collapsed'); bp.appendChild(mk); const hd=mk.querySelector('.edWHead'); if(hd)hd.addEventListener('click',function(e){ if(e.target.closest&&e.target.closest('#edMakerClose'))return; _bkToggle(); }); const note=mk.querySelector('.edWNote'), body=document.getElementById('edMakerBody'); if(note)note.style.display='none'; if(body)body.style.display='none'; } })();
document.getElementById('edRulesReset').addEventListener('click',()=>{ edPushUndo('rules'); _rules=rulesMerge(null); applyRules(_rules); edSyncRulesPanel(); });

/* ---- Undo / Redo: snapshot the whole map (incl. environment) via serialize/buildMap ---- */
document.getElementById('edUndo').addEventListener('click',edUndoDo);
document.getElementById('edRedo').addEventListener('click',edRedoDo);
edUpdateUndoBtns();
/* ============ MAP-EDITOR CHROME (overhaul): icon injection, tool rail, library search, status bar ============ */
(function(){ const IC={
  folder:'<path d="M3 6h5l1.5 2H17v8H3z"/>',
  export:'<path d="M4 12v4h12v-4"/><path d="M10 12V4M7 7l3-3 3 3"/>',
  reset:'<path d="M15.5 7A6 6 0 1 0 17 11"/><path d="M16.5 4v4h-4"/>',
  undo:'<path d="M8 5 4 9l4 4"/><path d="M4 9h8a4.5 4.5 0 0 1 0 9h-3"/>',
  redo:'<path d="M12 5l4 4-4 4"/><path d="M16 9H8a4.5 4.5 0 0 0 0 9h3"/>',
  grid:'<rect x="4" y="4" width="12" height="12" rx="1"/><path d="M8 4v12M12 4v12M4 8h12M4 12h12"/>',
  angle:'<path d="M4 15h12"/><path d="M4 15L14 5"/><path d="M4 15a8 8 0 0 1 4.5-6"/>',
  mirror:'<path d="M10 3v14" stroke-dasharray="1.5 2"/><path d="M7.5 7 4.5 10l3 3z"/><path d="M12.5 7l3 3-3 3z"/>',
  ortho:'<path d="M10 3l6 3.5v7L10 17l-6-3.5v-7z"/><path d="M10 3v14M4 6.5l6 3.5 6-3.5"/>',
  world:'<circle cx="10" cy="10" r="7"/><path d="M3 10h14"/><path d="M10 3c3 3 3 11 0 14M10 3c-3 3-3 11 0 14"/>',
  rules:'<path d="M4 6h12M4 10h12M4 14h12"/><circle cx="8" cy="6" r="1.7" fill="currentColor" stroke="none"/><circle cx="13" cy="10" r="1.7" fill="currentColor" stroke="none"/><circle cx="7" cy="14" r="1.7" fill="currentColor" stroke="none"/>',
  maker:'<path d="M4 8l6-4 6 4-6 4z"/><path d="M4 8v4l6 4 6-4V8"/>',
  menu:'<path d="M4 6h12M4 10h12M4 14h12"/>',
  panel:'<rect x="3" y="4" width="14" height="12" rx="1.5"/><path d="M12.5 4v12"/>',
  play:'<path d="M6.5 4.2 15 10l-8.5 5.8z"/>',
  select:'<path d="M5 3l3.4 13 2.1-5.2 5.5-1.6z"/>',
  move:'<path d="M10 3v14M3 10h14"/><path d="M10 3 8 5.3M10 3l2 2.3M10 17l-2-2.3M10 17l2-2.3M3 10l2.3-2M3 10l2.3 2M17 10l-2.3-2M17 10l-2.3 2"/>',
  rotate:'<path d="M15.8 7.2A6 6 0 1 0 17 11"/><path d="M16.8 4v3.6h-3.6"/>',
  scale:'<path d="M4 8V4h4M16 12v4h-4"/><path d="M4 4l4.5 4.5M16 16l-4.5-4.5"/>',
  place:'<rect x="4" y="4" width="12" height="12" rx="1.5"/><path d="M10 7v6M7 10h6"/>',
  scatter:'<circle cx="6" cy="7" r="1.4"/><circle cx="13" cy="5.5" r="1.4"/><circle cx="9.5" cy="12" r="1.4"/><circle cx="15" cy="13" r="1.4"/><circle cx="5" cy="14" r="1.4"/>',
  path:'<circle cx="5" cy="15" r="1.7"/><circle cx="15" cy="5" r="1.7"/><path d="M6.2 13.8C9 11 9 8.5 13.8 6.2" stroke-dasharray="1.6 2"/>',
  measure:'<rect x="2.5" y="7.5" width="15" height="5" rx="1" transform="rotate(-24 10 10)"/><path d="M7.4 6.7l1 1.6M10 5.4l.8 1.4M12.6 4.1l1 1.6" transform="rotate(-24 10 10)"/>',
  search:'<circle cx="9" cy="9" r="5.5"/><path d="M13.5 13.5 17 17"/>',
  crate:'<rect x="4" y="4" width="12" height="12" rx="1"/><path d="M4 4l12 12M16 4L4 16"/>',
  path:'<path d="M6 3 4 17"/><path d="M14 3l2 14"/><path d="M10 4v3M10 9v3M10 14v2"/>',
  platform:'<path d="M3 8l7-3 7 3-7 3z"/><path d="M3 8v3l7 3 7-3V8"/>',
  ramp:'<path d="M3 16h13L3 8z"/>',
  treeApple:'<circle cx="10" cy="8" r="5"/><path d="M10 13v4"/>',
  treePoplar:'<path d="M10 3c-3 3-3 9 0 11 3-2 3-8 0-11z"/><path d="M10 14v3"/>',
  treeOak:'<circle cx="7.5" cy="8" r="3.2"/><circle cx="12.5" cy="8" r="3.2"/><circle cx="10" cy="10.5" r="3.2"/><path d="M10 13v4"/>',
  orchard:'<circle cx="6.5" cy="6.5" r="2"/><circle cx="13.5" cy="6.5" r="2"/><circle cx="6.5" cy="13" r="2"/><circle cx="13.5" cy="13" r="2"/>',
  poprow:'<path d="M5 5c-1.5 1.5-1.5 4.6 0 6.2 1.5-1.6 1.5-4.7 0-6.2z"/><path d="M10 5c-1.5 1.5-1.5 4.6 0 6.2 1.5-1.6 1.5-4.7 0-6.2z"/><path d="M15 5c-1.5 1.5-1.5 4.6 0 6.2 1.5-1.6 1.5-4.7 0-6.2z"/><path d="M3 15h14"/>',
  haystack:'<path d="M4 15c1-6 11-6 12 0z"/><path d="M4 15h12"/>',
  haybales:'<rect x="4" y="10" width="5" height="5" rx="1"/><rect x="10" y="10" width="5" height="5" rx="1"/><rect x="7" y="4.5" width="5" height="5" rx="1"/>',
  cart:'<circle cx="7" cy="14" r="2.2"/><circle cx="14" cy="14" r="2.2"/><path d="M4 8h11l-1.3 4H6.5z"/>',
  well:'<path d="M4 7l6-3 6 3"/><rect x="6" y="11" width="8" height="5"/><path d="M7 7v4M13 7v4"/>',
  logpile:'<circle cx="7" cy="12.5" r="2"/><circle cx="11" cy="12.5" r="2"/><circle cx="9" cy="9" r="2"/>',
  barrels:'<path d="M6 5h8l-1 11H7z"/><path d="M5.5 9h9M5.5 12.5h9"/>',
  trough:'<path d="M4 8h12v4a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2z"/>',
  vegpatch:'<rect x="4" y="6" width="12" height="9" rx="1"/><path d="M7 8.5v4M10 8.5v4M13 8.5v4"/>',
  spawn:'<circle cx="10" cy="10" r="6"/><circle cx="10" cy="10" r="2.4"/>',
  resupply:'<circle cx="10" cy="10" r="6"/><path d="M10 7v6M7 10h6"/>',
  flag:'<path d="M6 3v14"/><path d="M6 4h8l-2 3 2 3H6"/>',
  house:'<path d="M4 9l6-5 6 5"/><path d="M5.5 8.5V16h9V8.5"/><rect x="8.5" y="11.5" width="3" height="4.5"/>',
  cottage:'<path d="M4 9c2.4-4.2 9.6-4.2 12 0"/><path d="M5.5 8.5V16h9V8.5"/><rect x="8.5" y="11.8" width="3" height="4.2"/>',
  farmhouse:'<path d="M3 9l7-4 7 4"/><path d="M4.5 8.5V16h11V8.5"/><rect x="8.6" y="11" width="2.8" height="5"/><path d="M13 16v-5h1.5v5"/>',
  manor:'<path d="M3 16V9l4-2.2L11 9v7"/><path d="M12 16V7.5"/><path d="M10.6 7.5L12.2 4l1.6 3.5z"/><rect x="10.8" y="9" width="2.8" height="7"/>',
  hq:'<rect x="5" y="7" width="10" height="9"/><path d="M8 7V4h5v2.2"/><path d="M7.5 16v-3h2v3"/>',
  refinery:'<rect x="4" y="8" width="12" height="8" rx="1"/><path d="M7 8V5.5a1.5 1.5 0 0 1 3 0V8"/><path d="M12 8V6h3v2"/>',
  barracks:'<path d="M4 16l6-9 6 9"/><path d="M8 16v-4h4v4"/>',
  power:'<path d="M11 3l-5 8h3.5l-.5 6 5-8H10.5z"/>',
  hedge:'<path d="M3 14c0-3 2.6-3.6 3.6-1.6 1-2.4 5-2.4 5.4 0 2-1.4 4.4-.2 4.4 2.2V15H3z"/>',
  wall:'<rect x="3" y="8" width="14" height="8"/><path d="M3 12h14M7 8v4M12 12v4M10 8v4"/>',
  fence:'<path d="M5.5 6v10M11 6v10M4 9.5h12.5M4 13h12.5"/>',
  star:'<path d="M10 2.5l2.05 4.55 4.95.5-3.7 3.35 1.05 4.85L10 13.6l-4.4 2.55 1.05-4.85-3.7-3.35 4.95-.5z"/>' 
};
  document.querySelectorAll('#editor [data-ic]').forEach(e=>{ const path=IC[e.dataset.ic]; if(!path)return; const cls=e.classList.contains('edField')?'ed-ico sm':(e.classList.contains('edTool')?'ed-ico lg':'ed-ico'); e.insertAdjacentHTML('afterbegin','<svg class="'+cls+'" viewBox="0 0 20 20">'+path+'</svg>'); });
})();
(function(){ let sc=null;
  document.addEventListener('pointerdown',function(e){ if(typeof mode!=='undefined'&&mode!=='editor')return; const inp=e.target&&e.target.closest&&e.target.closest('.edIInp'); if(!inp||!inp.closest('#edInsp'))return; sc={inp:inp,x:e.clientX,v:parseFloat(inp.value)||0,step:parseFloat(inp.step)||1,moved:false}; });
  document.addEventListener('pointermove',function(e){ if(!sc)return; const dx=e.clientX-sc.x; if(!sc.moved&&Math.abs(dx)<3)return; if(!sc.moved){ sc.moved=true; try{sc.inp.blur();}catch(_){} document.body.style.cursor='ew-resize'; } let v=sc.v+(dx/7)*sc.step; if(sc.inp.min!==''&&sc.inp.min!=null)v=Math.max(parseFloat(sc.inp.min),v); if(sc.inp.max!==''&&sc.inp.max!=null)v=Math.min(parseFloat(sc.inp.max),v); v=Math.round(v*100)/100; sc.inp.value=String(v); sc.inp.dispatchEvent(new Event('input',{bubbles:true})); e.preventDefault(); });
  document.addEventListener('pointerup',function(){ if(!sc)return; if(sc.moved){ sc.inp.dispatchEvent(new Event('change',{bubbles:true})); document.body.style.cursor=''; const _i=sc.inp; setTimeout(function(){ try{_i.blur();}catch(_){} },0); } sc=null; },true);
})();
(function(){ const rail=document.getElementById('edRail');
  if(rail){ rail.querySelectorAll('.edRailBtn').forEach(btn=>{ btn.addEventListener('click',()=>{ const m=btn.dataset.mode, grp=btn.dataset.group;
    if(grp==='lib'){ const lib=document.getElementById('edLib');
      if(!_edLibCollapsed && _edLibMode===m){ _edLibCollapsed=true; if(lib)lib.style.display='none'; if(typeof edApplyGridCols==='function')edApplyGridCols(); rail.querySelectorAll('.edRailBtn[data-group="lib"]').forEach(b=>b.classList.remove('on')); }
      else { _edLibCollapsed=false; if(lib)lib.style.display=''; if(typeof edApplyGridCols==='function')edApplyGridCols(); if(typeof edSetLibMode==='function')edSetLibMode(m); }
      return; }
    rail.querySelectorAll('.edRailBtn[data-group="tool"]').forEach(b=>b.classList.remove('on')); btn.classList.add('on');
    if(m==='rotate'||m==='scale'||m==='move')edGizmoMode=m; else if(m==='select')edGizmoMode='move';
    if(typeof setEdTool==='function')setEdTool(m==='measure'?'measure':'select');
    if(typeof edUpdateGizmo==='function'){edUpdateGizmo();edUpdateHandles();edUpdateRing();} }); }); }
  const f=document.getElementById('edLibFilter');
  function edLibSearch(q){ q=(q||'').trim().toLowerCase();
    document.querySelectorAll('#edPalette .edTabPanel').forEach(pn=>{ let panelAny=false, cur=null; const groups=[];
      Array.prototype.forEach.call(pn.children,ch=>{ if(ch.classList.contains('edGroup')){ cur={head:ch,items:[],cards:[]}; groups.push(cur); } else if(cur){ if(ch.classList.contains('edCards')){ cur.cards.push(ch); Array.prototype.push.apply(cur.items,ch.querySelectorAll('.edTool')); } else { cur.items.push(ch); } } });
      groups.forEach(g=>{ const gName=(g.head.textContent||'').toLowerCase(), gMatch=!!q&&gName.indexOf(q)>=0; let any=false;
        g.items.forEach(it=>{ const nm=(it.textContent||'').toLowerCase(), show=(!q)||gMatch||nm.indexOf(q)>=0; it.style.display=show?'':'none'; if(show)any=true; });
        g.cards.forEach(cs=>{ const csAny=[].some.call(cs.querySelectorAll('.edTool'),t=>t.style.display!=='none'); cs.style.display=csAny?'':'none'; });
        g.head.style.display=(any||!q)?'':'none'; if(any)panelAny=true; });
      if(q){ pn.style.display=panelAny?'flex':'none'; } else { pn.style.display=pn.classList.contains('active')?'flex':'none'; pn.querySelectorAll('.edCards,.edGroup,.edTool,.edToggle').forEach(x=>{x.style.display='';}); } }); }
  if(f)f.addEventListener('input',()=>edLibSearch(f.value));
  const tabs=document.getElementById('edLibTabs');
  if(tabs){ tabs.querySelectorAll('.edTab').forEach(tab=>{ tab.addEventListener('click',()=>{ tabs.querySelectorAll('.edTab').forEach(t=>t.classList.remove('on')); tab.classList.add('on'); const cat=tab.dataset.cat;
    if(cat==='recent'&&typeof edBuildRecent==='function')edBuildRecent(); document.querySelectorAll('#edPalette .edTabPanel').forEach(pn=>{ const on=pn.dataset.cat===cat; pn.classList.toggle('active',on); pn.style.display=on?'flex':'none'; pn.querySelectorAll('.edCards,.edGroup,.edTool,.edToggle').forEach(x=>{x.style.display='';}); if(on){ const _cds=pn.querySelectorAll('.edTool'); _cds.forEach(c=>c.classList.remove('edCardIn')); void pn.offsetWidth; _cds.forEach((c,i)=>{ c.style.animationDelay=(i*22)+'ms'; c.classList.add('edCardIn'); }); } }); if(f)f.value=''; }); }); }
  edStatusSync();
})();
/* ---- editor tooltips: fill label-less card tips + one floating tooltip for all [data-tip] ---- */
(function(){ const CT={hqF:'Headquarters — the base you must destroy to win.',hqH:'Enemy headquarters — destroy it to win the round.',refF:'Refinery — turns harvested ore into credits.',refH:'Enemy refinery — turns ore into credits.',barF:'Barracks — troop & armory building.',barH:'Enemy barracks — troop & armory building.',powF:'Power plant — powers the base.',powH:'Enemy power plant — powers the base.',crateS:'Small crate — low cover.',crateM:'Medium crate — waist-high cover.',crateL:'Large crate — full standing cover.',spawn:'Player spawn point.',resupply:'Resupply terminal — heal & refill ammo.',espawnSov:'Soviet enemy spawn point.',espawnAll:'Allies enemy spawn point.',espawnEmp:'Empire elite enemy spawn point.'};
  document.querySelectorAll('#edPalette .edTool').forEach(el=>{ const t=el.dataset.tool; if(t&&CT[t]&&!el.getAttribute('data-tip'))el.setAttribute('data-tip',CT[t]); });
  const root=document.getElementById('editor'); if(!root)return;
  const tip=document.createElement('div'); tip.id='edTipEl'; root.appendChild(tip);
  let cur=null, showT=0;
  function hide(){ clearTimeout(showT); cur=null; tip.style.display='none'; }
  function place(el){ tip.textContent=el.getAttribute('data-tip'); tip.style.display='block'; const r=el.getBoundingClientRect(), tr=tip.getBoundingClientRect(); let x,y;
    if(el.closest('#edRail')){ x=r.right+8; y=r.top+r.height/2-tr.height/2; } else { x=r.left+r.width/2-tr.width/2; y=r.bottom+7; }
    x=Math.max(6,Math.min(x,innerWidth-tr.width-6)); y=Math.max(6,Math.min(y,innerHeight-tr.height-6)); tip.style.left=x+'px'; tip.style.top=y+'px'; }
  document.addEventListener('mouseover',e=>{ if(typeof mode!=='undefined'&&mode!=='editor'){ if(cur)hide(); return; } const el=e.target.closest?e.target.closest('#editor [data-tip]'):null; if(el===cur)return; cur=el; clearTimeout(showT); if(el){ showT=setTimeout(()=>{ if(cur===el)place(el); },350); } else tip.style.display='none'; });
  document.addEventListener('mousedown',hide); document.addEventListener('wheel',hide,{passive:true});
})();

/* ============ SELECTION INSPECTOR (numeric transform) ============ */
(function(){ const empty=document.getElementById('edIEmpty'); if(!empty)return; const w=document.getElementById('edWorld'), r=document.getElementById('edRules'); if(w){ w.classList.add('edInlineSec'); empty.appendChild(w); } if(r){ r.classList.add('edInlineSec'); empty.appendChild(r); } if(typeof edSyncInspector==='function')edSyncInspector(); })();
(function(){ const b=document.getElementById('edWRBtn'); if(b)b.addEventListener('click',edToggleInsp); })();

// Position every cursor-following element ONCE per frame, in lockstep with rendering.
// Reticle moves on the compositor via translate3d (no layout); the transient overlays keep
// their CSS centring via left/top. Dirty-checked so a stationary cursor costs nothing.
}
