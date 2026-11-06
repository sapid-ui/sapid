const assert=require('node:assert/strict'),fs=require('node:fs');
const {matrix,chromium,base,outputPath,writeReport}=require('./showcase-browser-env.cjs');
const normal=['schedule','day-view','week-view','month-view','year-view','mobile-month-view','agenda-view'];
const resources=['resources-day-view','resources-week-view','resources-month-view','resources-schedule'];
(async()=>{
 const browser=await chromium.launch(),page=await browser.newPage({hasTouch:true});page.setDefaultTimeout(9000);
 const report={browser:browser.version(),pages:[],errors:[]};page.on('pageerror',e=>report.errors.push(e.message));page.on('console',m=>{if(m.type()==='error')report.errors.push(m.text());});
 const reportName=process.env.SAPID_SCHEDULE_REPORT||'schedule-remediation.json';
 fs.mkdirSync(outputPath('schedule-remediation'),{recursive:true});
 const tick=()=>page.evaluate(()=>new Promise(resolve=>{let n=0;const f=()=>++n===5?resolve():requestAnimationFrame(f);requestAnimationFrame(f);}));
 const click=async l=>{await l.click();await tick();};
 const check=async(l,value=true)=>{await l.setChecked(value);await tick();};
 const rect=async l=>{const r=await l.boundingBox();assert(r);return r;};
 const section=(slug,part)=>page.locator('.site-component-detail').filter({has:page.locator('#schedule-'+slug+'-'+part)}).locator('.site-demo').first();
 const remember=async(l,key)=>l.evaluate((e,key)=>window[key]=e,key);
 const retained=async(l,key)=>assert(await l.evaluate((e,key)=>e===window[key],key));
 const waitText=async(l,value)=>l.getByText(value,{exact:true}).waitFor({state:'visible'});
 const date=(scope,day)=>scope.getByRole('button',{name:new RegExp('^'+(day===4?'Wednesday, November 4, 2026':'Thursday, November 5, 2026'))});
 const prepare=async(scope,slug)=>{
  if(slug==='year-view')await click(scope.locator('.sapid-schedule-more'));
  if(slug==='mobile-month-view'){const target=date(scope,4);await target.scrollIntoViewIfNeeded();const b=await rect(target);await page.touchscreen.tap(b.x+b.width/2,b.y+b.height/2);await tick();}
 };
 const visit=async url=>{for(let attempt=0;;attempt++){try{await page.goto(url);return;}catch(error){if(attempt>=2||!String(error).includes('ERR_NO_BUFFER_SPACE'))throw error;(report.navigationRetries??=[]).push({url,attempt,error:String(error)});await page.waitForTimeout(250*(attempt+1));}}};
 try{
 for(const theme of (process.env.SAPID_SCHEDULE_THEMES||matrix.themes.join(',')).split(','))for(const scheme of (process.env.SAPID_SCHEDULE_SCHEMES||matrix.schemes.join(',')).split(','))for(const width of (process.env.SAPID_SCHEDULE_WIDTHS||'1280,390').split(',').map(Number))for(const slug of (process.env.SAPID_SCHEDULE_PAGES||[...normal,...resources,'recurring-events','events-data'].join(',')).split(',')){
  const key=slug+'-'+theme+'-'+scheme+'-'+width;
  try{
   await page.setViewportSize({width,height:1100});await visit(base+'/showcase.html?widget='+slug+'&theme='+theme+'&scheme='+scheme);await page.waitForFunction(slug=>window.ready===true&&document.documentElement.getAttribute('data-sapid-widget')===slug,slug,{timeout:30000});await tick();
   const asset=await page.evaluate(()=>document.documentElement.innerHTML.match(/assets-[a-f0-9]{20}/)?.[0]);assert(asset);if(report.asset)assert.equal(asset,report.asset);report.asset=asset;
   assert(await page.locator('.site-title,.site-section-title').evaluateAll(es=>es.every(e=>getComputedStyle(e).fontWeight==='400')));
   assert.equal(await page.locator('.site-component-detail').count(),slug==='recurring-events'?4:slug==='events-data'?3:5);
   assert(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth+1));
   const primary=page.locator('.site-preview .site-demo').first();
   if(normal.includes(slug)){
    if(slug==='schedule'){
     const header=primary.locator('.sapid-schedule-view-controls');await remember(header,'scheduleHeader');
     for(const [label,style] of [['Day','day'],['Month','month'],['Year','year'],['Mobile month','mobile-month'],['Agenda','agenda'],['Week','week']]){
      const control=header.getByRole('button',{name:label,exact:true});await click(control);assert.equal(await control.getAttribute('aria-pressed'),'true');assert.equal(await primary.locator('.sapid-'+style+'-view').count(),1);await retained(header,'scheduleHeader');
     }
     for(const b of await header.getByRole('button').all()){const r=await rect(b),h=await rect(header);assert(r.x+r.width<=h.x+h.width+1&&r.height<=32);}
    }
    await prepare(primary,slug);if(['schedule','day-view','week-view'].includes(slug))assert.equal(await primary.locator('.sapid-schedule-slot').first().evaluate(e=>getComputedStyle(e).height),'44px');if(['month-view','mobile-month-view'].includes(slug))assert.equal(await date(primary,4).evaluate(e=>getComputedStyle(e).borderTopWidth),'2px');assert(await primary.locator('.sapid-agenda-heading').evaluateAll(es=>es.every(e=>getComputedStyle(e).fontWeight==='400')));await click(primary.getByRole('button',{name:/^Project planning/}).first());await primary.getByText(/^Selected: Project planning/).waitFor();
    if(slug==='mobile-month-view'){
     await date(primary,4).focus();await tick();const previous=await page.evaluateHandle(()=>document.activeElement);await date(primary,5).evaluate(e=>e.click());await tick();assert.equal(await primary.locator('.sapid-agenda-event').count(),0);assert(await page.evaluate(element=>document.activeElement===element,previous));await prepare(primary,slug);await previous.dispose();
    }
    const controlled=section(slug,'controlled');await prepare(controlled,slug);
    await click(controlled.getByRole('button',{name:/^Project planning/}).first());await waitText(controlled,'Choose an event or an available time.');
    await check(controlled.getByLabel('Accept schedule changes',{exact:true}));await click(controlled.getByRole('button',{name:/^Project planning/}).first());await controlled.getByText(/^Selected: Project planning/).waitFor();
    const saved=await controlled.locator('.sapid-text').allTextContents();await check(controlled.getByLabel('Readonly schedule',{exact:true}));await controlled.getByRole('button',{name:/^Release day/}).first().evaluate(e=>e.click());await tick();assert.deepEqual(await controlled.locator('.sapid-text').allTextContents(),saved);
    const window=section(slug,'window'),range=window.getByText(/^Showing /),before=await range.textContent();await click(window.getByRole('button',{name:'Next schedule window',exact:true}));assert.notEqual(await range.textContent(),before);
    const presentation=section(slug,'presentation');assert.equal(await presentation.locator('.sapid-schedule-header').count(),0);if(['schedule','day-view','week-view'].includes(slug)){assert.equal(await presentation.locator('.sapid-schedule-slot').first().evaluate(e=>getComputedStyle(e).height),'60px');assert.equal(await presentation.locator('.sapid-agenda-event').count(),0);}if(['month-view','year-view','mobile-month-view'].includes(slug))assert.equal(await presentation.locator('.sapid-schedule-weekday-row').count(),0);
    const locale=section(slug,'locale');assert(/年|月|日/.test(await locale.textContent()));if(slug!=='agenda-view')assert(await locale.locator('[data-gly-grid-navigation=rtl]').count()>0);
    if(['schedule','day-view','week-view'].includes(slug)){
     const scroll=section(slug,'scroll'),viewport=scroll.locator('.sapid-schedule-viewport');await click(scroll.getByRole('button',{name:'Show around 10:00',exact:true}));assert(await viewport.evaluate(e=>e.scrollTop>=175));
     const grid=scroll.getByRole('grid'),slot=grid.getByRole('button').first();await slot.focus();await tick();const old=await page.evaluateHandle(()=>document.activeElement);await page.keyboard.press(slug==='day-view'?'ArrowDown':'ArrowRight');await tick();assert(await page.evaluate(element=>document.activeElement!==element,old));await old.dispose();
    }else if(slug==='month-view'||slug==='year-view'){
     const disclosure=section(slug,'disclosure'),count=await disclosure.getByRole('button',{name:/^Project planning|^Daily standup|^Design review/}).count();await click(disclosure.locator('.sapid-schedule-more').first());assert(await disclosure.getByRole('button',{name:/^Project planning|^Daily standup|^Design review/}).count()>count);
    }else if(slug==='mobile-month-view'){
     const dayList=section(slug,'disclosure');await prepare(dayList,slug);assert.equal(await dayList.locator('.sapid-agenda-event').count(),4);
    }else{
     const empty=section(slug,'empty');await check(empty.getByLabel('Show an empty agenda',{exact:true}));assert.equal(await empty.locator('.sapid-agenda-event').count(),0);assert(/No events/.test(await empty.textContent()));await check(empty.getByLabel('Show an empty agenda',{exact:true}),false);assert(await empty.locator('.sapid-agenda-event').count()>0);
    }
   }else if(resources.includes(slug)){
    assert.equal(await primary.locator('.sapid-resources-edit-command').count(),0);
    if(slug==='resources-schedule'){
     const header=primary.locator('.sapid-resource-schedule-view-controls');await remember(header,'resourceHeader');
     for(const [label,view] of [['Week','week'],['Month','month'],['Day','day']]){const button=header.getByRole('button',{name:label,exact:true});await click(button);assert.equal(await button.getAttribute('aria-pressed'),'true');assert.equal(await primary.locator('.sapid-resources-'+view+'-view').count(),1);await retained(header,'resourceHeader');}
    }
    await click(primary.getByRole('button',{name:/^Project planning/}).first());await primary.getByText(/^Selected: Project planning/).waitFor();
    const windows=section(slug,'window'),label=windows.getByText(/^Rooms:/);assert((await label.textContent()).includes('Room A, Room B'));await click(windows.getByRole('button',{name:'Next resources',exact:true}));assert((await label.textContent()).includes('Room C, Room D'));const range=await label.textContent();await click(windows.getByRole('button',{name:'Next date window',exact:true}));assert.notEqual(await label.textContent(),range);
    const controlled=section(slug,'controlled');await click(controlled.getByRole('button',{name:/^Project planning/}).first());await waitText(controlled,'Choose an event or an available time.');await check(controlled.getByLabel('Accept booking changes',{exact:true}));await click(controlled.getByRole('button',{name:/^Project planning/}).first());await controlled.getByText(/^Selected: Project planning/).waitFor();
    const editing=section(slug,'editing');await click(editing.getByRole('button',{name:/^Edit Design review/}));await click(editing.getByRole('button',{name:'Move later',exact:true}));await editing.getByText(/^Saved Design review/).waitFor();await check(editing.getByLabel('Accept booking edits',{exact:true}),false);await click(editing.getByRole('button',{name:'Move later',exact:true}));await waitText(editing,'Changes were not saved.');
    await check(editing.getByLabel('Accept booking edits',{exact:true}));await click(editing.getByRole('button',{name:/^Edit Project planning/}));await click(editing.getByRole('button',{name:'Move later',exact:true}));await waitText(editing,'Booking not saved: That room is already occupied during the proposed time.');
    const axis=section(slug,'axis');assert.equal(await axis.getByRole('group',{name:'Column resource layout',exact:true}).count(),1);assert.equal(await axis.getByRole('group',{name:'Row resource layout',exact:true}).count(),1);
    const locale=section(slug,'locale');assert(/年|月|日/.test(await locale.textContent()));assert(await locale.locator('[data-gly-grid-navigation=rtl]').count()>0);
   }else if(slug==='recurring-events'){
    await waitText(primary,'6 current occurrences');assert.equal(await primary.getByRole('button',{name:/^Daily standup/}).count(),6);await check(primary.getByLabel('Skip Thursday standup',{exact:true}),false);await waitText(primary,'7 current occurrences');assert.equal(await primary.getByRole('button',{name:/^Daily standup/}).count(),7);await check(primary.getByLabel('Skip Thursday standup',{exact:true}));await waitText(primary,'6 current occurrences');
    const rules=section(slug,'rules');assert((await rules.textContent()).includes('Every 2 days through November 8: 4 · Weekly COUNT 6: 6'));
    const missing=section(slug,'missing');assert((await missing.textContent()).includes('Month-end Skip: 3 · Clamp: 6. Leap-year Skip: 0 · Clamp: 1.'));
    const replacement=section(slug,'replacement');assert.equal(await replacement.getByRole('button',{name:/^Rescheduled standup/}).count(),0);await check(replacement.getByLabel('Move Wednesday standup to afternoon',{exact:true}));assert.equal(await replacement.getByRole('button',{name:/^Rescheduled standup/}).count(),1);assert.equal(await replacement.getByRole('button',{name:/^Daily standup/}).count(),6);
   }else{
    assert((await primary.textContent()).includes('Civil: no implicit UTC offset'));assert(await primary.locator('.sapid-agenda-event').count()>0);
    assert((await section(slug,'intervals').textContent()).includes('Overlapping pairs: 0.'));
    const zones=await section(slug,'zones').textContent();for(const value of ['Civil has no instant: true','Explicit +05:30 resolves both endpoints: true','Supplied demo-zone resolver resolves both endpoints: true'])assert(zones.includes(value));
    const validation=section(slug,'validation'),event=validation.locator('.sapid-agenda-event').first();await remember(event,'validatedEvent');for(const [name,message]of [['Validate a key with spaces','Key rejected:'],['Validate a reversed interval','Interval rejected:'],['Validate an unknown room','Resource rejected:']]){await click(validation.getByRole('button',{name,exact:true}));assert((await validation.textContent()).includes(message));await retained(event,'validatedEvent');}
   }
   assert(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth+1));await primary.scrollIntoViewIfNeeded();await primary.screenshot({path:outputPath('schedule-remediation/'+key+'.png')});report.pages.push({key,passed:true});console.log('PASS '+key);
  }catch(error){report.pages.push({key,passed:false,failure:error.stack});console.error('FAIL '+key+': '+error.stack);await page.screenshot({path:outputPath('schedule-remediation/failure-'+key+'.png')});}
  writeReport(reportName,report);
 }
 assert.equal(report.pages.filter(p=>!p.passed).length,0);assert.deepEqual(report.errors,[]);
 }finally{writeReport(reportName,report);await browser.close();}
})().catch(error=>{console.error(error);process.exitCode=1;});

