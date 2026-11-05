const assert=require('node:assert/strict'),fs=require('node:fs');
const {chromium,base,outputPath,writeReport}=require('./showcase-browser-env.cjs');
const chartPages=['area-chart','bar-chart','line-chart','composite-chart'];
const plots=['candlestick-chart','donut-chart','pie-chart','funnel-chart','radar-chart','scatter-chart','bubble-chart','radial-bar-chart','sparkline','bars-list','bullet-chart','gauge-chart','waffle-chart','treemap','sunburst-chart','sankey-chart','matrix-chart'];
(async()=>{
 const browser=await chromium.launch(),page=await browser.newPage();page.setDefaultTimeout(7000);
 const report={browser:browser.version(),pages:[],errors:[]};page.on('pageerror',e=>report.errors.push(e.message));page.on('console',m=>{if(m.type()==='error')report.errors.push(m.text());});
 fs.mkdirSync(outputPath('charts-remediation'),{recursive:true});
 const tick=()=>page.evaluate(()=>new Promise(resolve=>{let n=0;const f=()=>++n===5?resolve():requestAnimationFrame(f);requestAnimationFrame(f);}));
 const click=async l=>{await l.click();await tick();};
 const rect=async l=>{const r=await l.boundingBox();assert(r);return r;};
 const section=(slug,part)=>page.locator('.site-component-detail').filter({has:page.locator(`#chart-${slug}-${part}`)}).locator('.site-demo').first();
 const remember=async(l,k)=>l.evaluate((e,k)=>window[k]=e,k),retained=async(l,k)=>assert(await l.evaluate((e,k)=>e===window[k],k));
 try{
 for(const theme of (process.env.SAPID_CHART_THEMES||'blue,green').split(','))for(const scheme of (process.env.SAPID_CHART_SCHEMES||'light,dark').split(','))for(const width of (process.env.SAPID_CHART_WIDTHS||'1280,390').split(',').map(Number))for(const slug of (process.env.SAPID_CHART_PAGES||[...chartPages,...plots].join(',')).split(',')){
 const key=`${slug}-${theme}-${scheme}-${width}`;
 try{
 await page.setViewportSize({width,height:1100});await page.goto(`${base}/showcase.html?widget=${slug}&theme=${theme}&scheme=${scheme}`);await page.waitForFunction(()=>window.ready===true,undefined,{timeout:30000});await tick();
 const asset=await page.evaluate(()=>document.documentElement.innerHTML.match(/assets-[a-f0-9]{20}/)?.[0]);assert(asset);if(report.asset)assert.equal(asset,report.asset);report.asset=asset;
 assert(await page.locator('.site-title,.site-section-title').evaluateAll(es=>es.every(e=>getComputedStyle(e).fontWeight==='400')));assert(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth+1));
 const cartesian=chartPages.includes(slug),primary=page.locator('.site-preview .site-demo').first(),frameSelector=cartesian?'.sapid-chart-plot':'.sapid-plot-drawing',pointSelector=cartesian?'.sapid-chart-point':'.sapid-plot-point',legendSelector=cartesian?'.sapid-chart-legend-item':'.sapid-plot-legend-item',detailSelector=cartesian?'.sapid-chart-point-detail':'.sapid-plot-detail';
 const expected=cartesian?(slug==='area-chart'||slug==='bar-chart'?6:5):4;assert.equal(await page.locator('.site-component-detail').count(),expected);assert.equal(await primary.locator('.site-chart-card').count(),0);
 const frame=primary.locator(frameSelector),b=await rect(frame),container=await rect(primary);assert(b.width<=container.width+1&&b.width>=250);assert(b.height>=65);
 await remember(frame,'primaryChartFrame');
 if(slug==='sparkline'){
  assert(b.width<=321&&b.height<=81);assert.equal(await primary.locator(legendSelector).count(),0);assert.equal(await primary.getByRole('button',{name:'Next data point',exact:true}).count(),0);assert((await primary.textContent()).includes('Weekly values: 4, 7, 5, 9, 8, 12, 15'));
 }else{
  assert(b.width>=container.width-80);assert(await primary.getByRole('table').count()>0);
  const next=primary.getByRole('button',{name:'Next data point',exact:true});assert((await rect(next)).height<=31);await click(next);assert.notEqual((await primary.locator(detailSelector).textContent()).trim(),'No data point selected');
  await click(primary.getByRole('button',{name:'Clear data point',exact:true}));assert.equal((await primary.locator(detailSelector).textContent()).trim(),'No data point selected');
  const legend=primary.locator(legendSelector);if(await legend.count()){
   const item=legend.first();await remember(item,'primaryLegend');assert.equal(await item.getAttribute('aria-pressed'),'true');assert((await rect(item)).height<=31);const name=await item.getAttribute('aria-label')||await item.textContent();assert(name.includes('Hide'));await click(item);assert.equal(await item.getAttribute('aria-pressed'),'false');assert((await item.textContent()).includes('Show'));await retained(item,'primaryLegend');await click(item);assert.equal(await item.getAttribute('aria-pressed'),'true');
  }
 }
 const controlled=section(slug,'controlled'),readout=controlled.locator(detailSelector);await click(controlled.getByRole('button',{name:'Next data point',exact:true}));assert.equal((await readout.textContent()).trim(),'No data point selected');await controlled.getByLabel('Accept chart changes',{exact:true}).check();await tick();await click(controlled.getByRole('button',{name:'Next data point',exact:true}));assert.notEqual((await readout.textContent()).trim(),'No data point selected');
 const recovery=section(slug,'recovery'),recoveryFrame=recovery.locator(frameSelector);await remember(recoveryFrame,'recoveryFrame');const pointCount=await recovery.locator(pointSelector).count(),tables=await recovery.getByRole('table').allTextContents();await click(recovery.getByRole('button',{name:'Reject invalid update',exact:true}));assert.equal(await recovery.locator(pointSelector).count(),pointCount);assert.deepEqual(await recovery.getByRole('table').allTextContents(),tables);assert((await recovery.textContent()).includes('Rejected update:'));await click(recovery.getByRole('button',{name:'Show empty data',exact:true}));assert.equal(await recovery.locator(pointSelector).count(),0);assert((await recovery.textContent()).includes(cartesian?'No visible chart data':'No visible plot data'));await click(recovery.getByRole('button',{name:'Restore chart data',exact:true}));assert.equal(await recovery.locator(pointSelector).count(),pointCount);await retained(recoveryFrame,'recoveryFrame');
 if(cartesian){
  const window=section(slug,'window'),before=await window.locator(pointSelector).count(),rows=await window.getByRole('row').count();await window.getByLabel('Show March through May',{exact:true}).check();await tick();assert(await window.locator(pointSelector).count()<before);assert.equal(await window.getByRole('row').count(),rows);assert((await window.locator('.sapid-chart-clipping').textContent()).includes('clipped'));
  const missing=section(slug,'series');assert((await missing.getByRole('table').textContent()).includes('No value'));assert((await missing.getByRole('table').textContent()).includes('0'));
 }else{
  const data=section(slug,'data');await click(data.getByRole('button',{name:'Next data point',exact:true}));const reading=await data.locator(detailSelector).textContent();
  if(slug==='candlestick-chart')for(const field of ['open','high','low','close'])assert(reading.includes(field));
  if(slug==='scatter-chart'||slug==='bubble-chart'){for(const field of ['x=','y=','size='])assert(reading.includes(field));assert.equal(await primary.locator('.sapid-plot-label').count(),12);}
  if(slug==='treemap'||slug==='sunburst-chart'){await click(data.getByRole('button',{name:'First child',exact:true}));assert.notEqual(await data.locator(detailSelector).textContent(),reading);await click(data.getByRole('button',{name:'Parent node',exact:true}));assert.equal(await data.locator(detailSelector).textContent(),reading);}
  const presentation=section(slug,'presentation');
  if(slug==='matrix-chart'){const labels=await primary.locator('.sapid-plot-label').allTextContents();for(const label of ['Product A','Product B','Product C','Q1','Q2','Q3'])assert(labels.includes(label));assert.equal(labels.length,15);const row=primary.locator('.sapid-plot-label').filter({hasText:/^Product A$/}),column=primary.locator('.sapid-plot-label').filter({hasText:/^Q1$/});assert((await rect(row)).x<(await rect(column)).x);}
  if(slug==='donut-chart'){assert.equal(await primary.locator('.sapid-plot-label').textContent(),'72%');const center=presentation.getByText('Completion',{exact:true}),c=await rect(center),f=await rect(presentation.locator(frameSelector));assert(Math.abs(c.x+c.width/2-f.x-f.width/2)<2);assert(c.width<f.width/2);}
  if(slug==='waffle-chart'){const cells=async scope=>scope.locator('.sapid-plot-layer path').evaluateAll(es=>es.reduce((sum,e)=>sum+(e.getAttribute('d').match(/M/g)||[]).length,0));assert.equal(await cells(primary),100);assert.equal(await cells(presentation),20);await click(presentation.getByRole('button',{name:'Next data point',exact:true}));assert((await presentation.locator(detailSelector).textContent()).includes('of 20 cells'));}
  if(slug==='bubble-chart'){const maximum=await presentation.locator('.sapid-plot-layer path').evaluateAll(es=>Math.max(...es.map(e=>e.getBBox()).filter(b=>Math.abs(b.width-b.height)<0.01).map(b=>b.width)));assert(Math.abs(maximum-64)<0.1);}
 }
 await page.setViewportSize({width:width===390?1280:390,height:1100});await tick();await retained(frame,'primaryChartFrame');await page.setViewportSize({width,height:1100});await tick();await primary.scrollIntoViewIfNeeded();await primary.screenshot({path:outputPath(`charts-remediation/${key}.png`)});assert(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth+1));
 report.pages.push({key,passed:true});console.log(`PASS ${key}`);
 }catch(error){report.pages.push({key,passed:false,failure:error.stack});console.error(`FAIL ${key}: ${error.stack}`);await page.screenshot({path:outputPath(`charts-remediation/failure-${key}.png`)});}
 writeReport('charts-remediation.json',report);
 }
 assert.equal(report.pages.filter(p=>!p.passed).length,0);assert.deepEqual(report.errors,[]);
 }finally{writeReport('charts-remediation.json',report);await browser.close();}
})().catch(e=>{console.error(e);process.exitCode=1;});
