// verifier scratch: parse plan.md section 6.2 mark column, expand ranges, check coverage
const fs=require('fs');
const lines=fs.readFileSync(process.argv[2],'utf8').split('\n').slice(281,321);
const seen={}; // key "0001-n" -> [rows]
for(const [i,l] of lines.entries()){
  if(!l.startsWith('| ')) continue;
  const cell=l.split('|')[1].trim();
  if(cell==='Mark'||cell.startsWith('---')) continue;
  let adr='0001';
  const m0=cell.match(/^PM-(\d{4})-/); if(m0) adr=m0[1];
  const parts=cell.replace(/^PM-\d{4}/,'').split(',').map(s=>s.trim());
  for(let p of parts){
    p=p.replace(/^PM-\d{4}/,'');
    let r=p.match(/^-(\d+) to -(\d+)$/);
    if(r){for(let n=+r[1];n<=+r[2];n++){(seen[adr+'-'+n]??=[]).push(282+i);} continue;}
    r=p.match(/^-(\d+)$/); if(r){(seen[adr+'-'+r[1]]??=[]).push(282+i); continue;}
    console.log('UNPARSED',p,'line',282+i);
  }
}
const want=[];
for(let n=1;n<=63;n++) want.push('0001-'+n);
want.push('0004-67'); for(let n=11;n<=27;n++) want.push('0006-'+n);
for(const k of want){ if(!seen[k]) console.log('MISSING',k); else if(seen[k].length>1) console.log('DUP',k,seen[k].join(',')); }
for(const k of Object.keys(seen)) if(!want.includes(k)) console.log('EXTRA',k,seen[k]);
console.log('rows parsed; keys',Object.keys(seen).length);
