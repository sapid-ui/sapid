const assert=require('node:assert/strict'),fs=require('node:fs');
const {matrix,chromium,base,outputPath,writeReport}=require('./showcase-browser-env.cjs');
(async()=>{
 const browser=await chromium.launch(),page=await browser.newPage(),report={browser:browser.version(),pages:[],errors:[]};
 page.on('pageerror',e=>report.errors.push(e.message));page.on('console',m=>{if(m.type()==='error')report.errors.push(m.text());});
 const tick=()=>page.evaluate(()=>new Promise(resolve=>{let n=0;const f=()=>++n===6?resolve():requestAnimationFrame(f);requestAnimationFrame(f);}));
 const detail=title=>page.locator('.site-component-detail').filter({has:page.getByRole('heading',{name:title,exact:true})});
 fs.mkdirSync(outputPath('text-remediation'),{recursive:true});
 try{
 for(const theme of matrix.themes)for(const scheme of matrix.schemes)for(const width of [1280,390])for(const slug of ['input','text-input','textarea']){
  const key=`${slug}-${theme}-${scheme}-${width}`;
  try{
   await page.setViewportSize({width,height:1000});await page.goto(`${base}/showcase.html?widget=${slug}&theme=${theme}&scheme=${scheme}`);await page.waitForFunction(()=>window.ready===true,undefined,{timeout:20000});await tick();
   const asset=await page.evaluate(()=>document.documentElement.innerHTML.match(/assets-[a-f0-9]{20}/)?.[0]);assert(asset);if(report.asset)assert.equal(asset,report.asset);report.asset=asset;
   assert.equal(await page.locator('.site-component-detail').count(),slug==='textarea'?3:4);
   const primary=page.locator('.site-preview .site-demo').first(),editor=primary.getByRole('textbox',{name:slug==='input'?'Email address':slug==='text-input'?'Your name':'Biography',exact:true});
   await editor.evaluate(e=>window.primaryEditor=e);await editor.fill(slug==='textarea'?'First line\nSecond line':'Ada');await tick();assert.equal(await editor.inputValue(),slug==='textarea'?'First line\nSecond line':'Ada');assert(await editor.evaluate(e=>e===window.primaryEditor));
   if(slug==='input'){
    assert(await editor.evaluate(e=>e.required));await editor.fill('');assert(!(await editor.evaluate(e=>e.checkValidity())));
    const labels=detail('Labels, descriptions and validation'),invalid=labels.getByRole('textbox',{name:'Workspace name',exact:true});assert.equal(await invalid.getAttribute('aria-invalid'),'true');assert(await invalid.getAttribute('aria-describedby'));assert(await invalid.getAttribute('aria-errormessage'));await labels.getByText('Work email',{exact:true}).click();assert(await labels.getByRole('textbox',{name:'Work email',exact:true}).evaluate(e=>e===document.activeElement));
    const sizes=detail('Five sizes');const heights=await sizes.locator('input').evaluateAll(es=>es.map(e=>e.getBoundingClientRect().height));assert.equal(heights.length,5);assert(heights.every((v,i)=>i===0||v>heights[i-1]));
    const order=detail('Field ordering');assert(await order.evaluate(e=>{const d=e.querySelector('.sapid-field-description'),l=e.querySelector('.sapid-field-label'),c=e.querySelector('input'),err=e.querySelector('.sapid-field-error');return !!(d.compareDocumentPosition(l)&Node.DOCUMENT_POSITION_FOLLOWING)&&!!(l.compareDocumentPosition(c)&Node.DOCUMENT_POSITION_FOLLOWING)&&!!(c.compareDocumentPosition(err)&Node.DOCUMENT_POSITION_FOLLOWING);}));
   }else if(slug==='text-input'){
    const controlled=detail('Controlled editing, read-only and disabled'),input=controlled.getByRole('textbox',{name:'Controlled name',exact:true});await input.evaluate(e=>window.controlledText=e);await input.fill('Refused');await tick();assert.equal(await input.inputValue(),'Ada');await controlled.getByRole('checkbox',{name:'Accept text changes',exact:true}).check();await input.fill('Accepted');await tick();assert.equal(await input.inputValue(),'Accepted');
    await controlled.getByRole('checkbox',{name:'Make text read-only',exact:true}).check();await tick();assert(!(await input.isEditable()));await controlled.getByRole('button',{name:'Set name from application',exact:true}).click();await tick();assert.equal(await input.inputValue(),'Grace');await controlled.getByRole('checkbox',{name:'Disable text editor',exact:true}).check();await tick();assert(await input.isDisabled());assert(await input.evaluate(e=>e===window.controlledText));
    const variants=detail('Default, filled and borderless styles');assert.equal(await variants.getByRole('textbox',{name:'Borderless presentation',exact:true}).evaluate(e=>getComputedStyle(e).borderTopWidth),'0px');const colors=await variants.locator('input').evaluateAll(es=>es.map(e=>getComputedStyle(e).backgroundColor));assert.notEqual(colors[0],colors[1]);
    const states=detail('Long hints and unavailable states');assert(!(await states.getByRole('textbox',{name:'Read-only account',exact:true}).isEditable()));assert(await states.getByRole('textbox',{name:'Disabled account',exact:true}).isDisabled());
   }else{
    const fixed=detail('Fixed rows and ordered limits'),input=fixed.getByRole('textbox',{name:'Fixed three rows',exact:true});await input.evaluate(e=>window.fixedTextarea=e);const before=(await input.boundingBox()).height;await input.fill('A very long wrapped line '.repeat(80)+'\n'+('New line\n'.repeat(20)));await tick();assert.equal((await input.boundingBox()).height,before);assert(await input.evaluate(e=>e.scrollHeight>e.clientHeight));assert(await input.evaluate(e=>e===window.fixedTextarea));await input.press('End');await input.press('Enter');assert((await input.inputValue()).includes('\n'));
    const auto=detail('Autosize on HTML hosts').getByRole('textbox',{name:'Autosizing message',exact:true});assert(await auto.evaluate(()=>CSS.supports('field-sizing','content')));await auto.evaluate(e=>window.autoTextarea=e);await auto.fill('Short');await tick();const empty=(await auto.boundingBox()).height;await auto.fill('Line\n'.repeat(12));await tick();const grown=(await auto.boundingBox()).height;assert(grown>empty+20);assert(await auto.evaluate(e=>e.scrollHeight>e.clientHeight));await auto.fill('Short');await tick();assert.equal((await auto.boundingBox()).height,empty);assert(await auto.evaluate(e=>e===window.autoTextarea));
    const states=detail('Controlled draft, errors and unavailable states'),controlled=states.getByRole('textbox',{name:'Controlled notes',exact:true});assert.equal(await controlled.getAttribute('aria-invalid'),'true');await controlled.fill('Next step: write a test.\nThen review.');await tick();assert.equal(await controlled.inputValue(),'Next step: write a test.\nThen review.');assert(!(await states.getByRole('textbox',{name:'Read-only notes',exact:true}).isEditable()));assert(await states.getByRole('textbox',{name:'Disabled notes',exact:true}).isDisabled());
   }
   if(slug!=='textarea'){
    const section=detail(slug==='input'?'Reserved sections, clear action and loading':'Left decoration and inset clear action'),input=section.getByRole('textbox',{name:'Workspace URL',exact:true}),clear=section.getByRole('button',{name:'Clear workspace URL',exact:true});await input.evaluate(e=>window.sectionEditor=e);const b=await clear.boundingBox(),r=await input.boundingBox();assert(b.width<=25&&b.height<=25);assert(b.x>=r.x&&b.x+b.width<=r.x+r.width+1);await clear.click();await tick();assert.equal(await input.inputValue(),'');assert(await input.evaluate(e=>e===window.sectionEditor));await section.getByRole('checkbox',{name:'Show input loading indicator',exact:true}).check();await tick();assert(await section.locator('.sapid-input-loading.is-left').isVisible());
   }
   assert(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth+1));await primary.scrollIntoViewIfNeeded();await primary.screenshot({path:outputPath(`text-remediation/${key}.png`)});report.pages.push({key,passed:true});console.log(`PASS ${key}`);
  }catch(error){report.pages.push({key,passed:false,failure:error.stack});console.error(`FAIL ${key}: ${error.message}`);await page.screenshot({path:outputPath(`text-remediation/failure-${key}.png`)});}
 }
 assert.equal(report.pages.filter(p=>!p.passed).length,0);assert.deepEqual(report.errors,[]);
 }finally{writeReport('text-remediation.json',report);await browser.close();}
})().catch(e=>{console.error(e);process.exitCode=1;});
