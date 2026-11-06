const assert = require('node:assert/strict');
const fs = require('node:fs');
const {matrix,chromium, base, outputPath, writeReport} = require('./showcase-browser-env.cjs');
(async () => {
 const browser = await chromium.launch(), page = await browser.newPage();
 const report = {browser: browser.version(), pages: [], errors: []};
 page.on('pageerror', e => report.errors.push(e.message));
 page.on('console', m => { if (m.type() === 'error') report.errors.push(m.text()); });
 const tick = () => page.evaluate(() => new Promise(resolve => { let n=0; const f=()=>++n===6?resolve():requestAnimationFrame(f); requestAnimationFrame(f); }));
 const detail = title => page.locator('.site-component-detail').filter({has: page.getByRole('heading', {name:title,exact:true})});
 const clickCaption = async (area, name) => { const r=await area.locator('.sapid-switch-label').filter({hasText:name}).boundingBox(); assert(r); await page.mouse.click(r.x+r.width/2,r.y+r.height/2); await tick(); };
 fs.mkdirSync(outputPath('switch-remediation'), {recursive:true});
 try {
 for(const theme of matrix.themes) for(const scheme of matrix.schemes) for(const width of [1280,390]) {
  const key=`switch-${theme}-${scheme}-${width}`;
  try {
   await page.setViewportSize({width,height:1000});
   await page.goto(`${base}/showcase.html?widget=switch&theme=${theme}&scheme=${scheme}`);
   await page.waitForFunction(()=>window.ready===true,undefined,{timeout:20000});
   report.asset=await page.evaluate(()=>document.documentElement.innerHTML.match(/assets-[a-f0-9]{20}/)?.[0]); assert(report.asset);
   assert.equal(await page.locator('.site-component-detail').count(),4);
   const primary=page.locator('.site-preview .site-demo').first(), control=primary.getByRole('switch',{name:'Notifications',exact:true});
   assert(await control.isChecked()); await control.evaluate(e=>window.switchInput=e);
   const track=primary.locator('.sapid-switch-track'), thumb=primary.locator('.sapid-switch-thumb');
   const tr=await track.boundingBox(), on=await thumb.boundingBox(); assert.equal(tr.width,34); assert.equal(tr.height,20); assert.equal(on.width,16); assert(on.x>tr.x+10);
   await control.focus(); await control.press('Space'); await tick(); assert(!(await control.isChecked()));
   const off=await thumb.boundingBox(); assert.equal(off.x-tr.x,2); assert(await control.evaluate(e=>e===window.switchInput && document.activeElement===e));
   const ring=primary.locator('.sapid-switch-focus'); assert.equal(await ring.evaluate(e=>getComputedStyle(e).borderTopWidth),'2px');
   await clickCaption(primary,'Notifications'); assert(await control.isChecked());
   const sizes=detail('Sizes and track marks');
   const geometry=await sizes.locator('.sapid-switch-track').evaluateAll(es=>es.map(e=>{const r=e.getBoundingClientRect();return [r.width,r.height];}));
   assert.deepEqual(geometry,[[26,14],[30,16],[34,20],[42,24],[50,28]]); assert.equal(await sizes.locator('.sapid-switch-mark').count(),5);
   const labels=detail('Label position, description and error');
   for(const [name,before] of [['Before the track',true],['After the track',false]]){
    const root=labels.locator('.sapid-switch').filter({has:page.getByRole('switch',{name,exact:true})});
    const caption=await root.locator('.sapid-switch-label').boundingBox(), frame=await root.locator('.sapid-switch-track').boundingBox();
    assert(before?caption.x+caption.width<=frame.x+1:frame.x+frame.width<=caption.x+1);
    await root.scrollIntoViewIfNeeded(); await clickCaption(root,name); assert(await root.getByRole('switch').isChecked());
   }
   const invalid=labels.getByRole('switch',{name:'Account alerts',exact:true}); assert.equal(await invalid.getAttribute('aria-invalid'),'true'); assert(await invalid.getAttribute('aria-describedby')); assert(await invalid.getAttribute('aria-errormessage'));
   const states=detail('Controlled refusal, read-only and disabled'), controlled=states.getByRole('switch',{name:'Controlled notifications',exact:true});
   await controlled.click(); await tick(); assert(await controlled.isChecked());
   await states.getByRole('checkbox',{name:'Accept switch changes',exact:true}).check(); await controlled.uncheck(); await tick(); assert(!(await controlled.isChecked()));
   const readonly=states.getByRole('switch',{name:'Read-only notifications',exact:true}); await readonly.click(); await readonly.press('Space'); await tick(); assert(await readonly.isChecked());
   assert(await states.getByRole('switch',{name:'Disabled notifications',exact:true}).isDisabled());
   const group=detail('Independent values and disabled groups'), email=group.getByRole('switch',{name:'Email notifications',exact:true}), push=group.getByRole('switch',{name:'Push notifications',exact:true});
   await push.check(); assert(await email.isChecked()); assert(await push.isChecked()); await push.evaluate(e=>window.groupSwitch=e);
   await group.getByRole('checkbox',{name:'Disable notification channels',exact:true}).check(); await tick(); assert(await email.isDisabled()); assert(await push.isDisabled());
   await group.getByRole('checkbox',{name:'Disable notification channels',exact:true}).uncheck(); await tick(); assert(await push.isChecked()); assert(await push.evaluate(e=>e===window.groupSwitch));
   assert(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth+1));
   await primary.scrollIntoViewIfNeeded(); await primary.screenshot({path:outputPath(`switch-remediation/${key}.png`)});
   report.pages.push({key,passed:true}); console.log(`PASS ${key}`);
  } catch(error) { report.pages.push({key,passed:false,failure:error.stack}); console.error(`FAIL ${key}: ${error.message}`); await page.screenshot({path:outputPath(`switch-remediation/failure-${key}.png`)}); }
 }
 for(const theme of matrix.themes) for(const scheme of matrix.schemes) {
  const key=`switch-touch-${theme}-${scheme}`;
  const context=await browser.newContext({viewport:{width:390,height:850},isMobile:true,hasTouch:true});
  const mobile=await context.newPage(); mobile.on('pageerror',e=>report.errors.push(e.message));
  try {
   await mobile.goto(`${base}/showcase.html?widget=switch&theme=${theme}&scheme=${scheme}`);
   await mobile.waitForFunction(()=>window.ready===true,undefined,{timeout:20000});
   const primary=mobile.locator('.site-preview .site-demo').first(), input=primary.getByRole('switch',{name:'Notifications',exact:true});
   await primary.scrollIntoViewIfNeeded(); const r=await primary.locator('.sapid-switch-track').boundingBox(); assert(r);
   await mobile.touchscreen.tap(r.x+r.width-4,r.y+r.height/2); await mobile.waitForTimeout(120); assert(!(await input.isChecked()));
   const label=await primary.locator('.sapid-switch-label').boundingBox(); await mobile.touchscreen.tap(label.x+label.width/2,label.y+label.height/2); await mobile.waitForTimeout(120); assert(await input.isChecked());
   report.pages.push({key,passed:true}); console.log(`PASS ${key}`);
  } catch(error) { report.pages.push({key,passed:false,failure:error.stack}); console.error(`FAIL ${key}: ${error.message}`); }
  finally {await context.close();}
 }
 assert.equal(report.pages.filter(p=>!p.passed).length,0); assert.deepEqual(report.errors,[]);
 } finally { writeReport('switch-remediation.json',report); await browser.close(); }
})().catch(e=>{console.error(e);process.exitCode=1;});
