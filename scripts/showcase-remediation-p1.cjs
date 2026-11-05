// Re-run the 27 priority pages against one immutable showcase release.
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),{spawnSync}=require('node:child_process');
const {outputPath,base}=require('./showcase-browser-env.cjs');
const batches=[['navigation','navigation',40],['composites','composite',24],['geometry','geometry',48],['inputs','input',40],['media','media',24],['menus','menu',16],['mini-calendar','mini-calendar',8],['heatmap','heatmap',8],['transition','transition',8]];
(async()=>{
 const html=await (await fetch(base+'/showcase.html')).text(),asset=html.match(/assets-[a-f0-9]{20}/)?.[0];assert(asset);
 const directory=outputPath('final-p1');fs.mkdirSync(directory,{recursive:true});const report={asset,pages:27,cases:0,errors:[],batches:[]};
 for(const [script,prefix,count]of batches){const run=spawnSync(process.execPath,[path.join(__dirname,'showcase-remediation-'+script+'.cjs')],{env:{...process.env,SAPID_BROWSER_OUTPUT:directory},encoding:'utf8',maxBuffer:4*1024*1024});fs.writeFileSync(path.join(directory,prefix+'.log'),(run.stdout||'')+(run.stderr||''));assert.equal(run.status,0,prefix+' failed; inspect '+path.join(directory,prefix+'.log'));const result=JSON.parse(fs.readFileSync(path.join(directory,prefix+'-remediation.json'),'utf8'));assert.deepEqual(result.errors,[]);assert.equal(result.pages.length,count);assert(result.pages.every(p=>p.passed!==false));if(result.asset)assert.equal(result.asset,asset);report.browser=result.browser;report.cases+=count;report.batches.push({batch:prefix,cases:count,report:path.join(directory,prefix+'-remediation.json')});console.log('PASS '+prefix+' '+count);fs.writeFileSync(path.join(directory,'p1-final-current.json'),JSON.stringify(report,null,2));}
 assert.equal(report.cases,216);const latest=await(await fetch(base+'/showcase.html')).text();assert(latest.includes(asset));
})().catch(error=>{console.error(error);process.exitCode=1;});
