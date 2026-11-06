const assert = require('node:assert/strict');
const fs = require('node:fs');
const {matrix, chromium, base, outputPath, writeReport } = require('./showcase-browser-env.cjs');
(async () => {
  const browser = await chromium.launch(); const page = await browser.newPage();
  const report = { browser: browser.version(), pages: [], errors: [] };
  page.on('pageerror', e => report.errors.push(e.message));
  page.on('console', m => { if (m.type() === 'error') report.errors.push(m.text()); });
  const tick = () => page.evaluate(() => new Promise(resolve => { let n=0; const frame=()=>++n===6?resolve():requestAnimationFrame(frame); requestAnimationFrame(frame); }));
  const detail = title => page.locator('.site-component-detail').filter({ has: page.getByRole('heading', { name:title, exact:true }) });
  const box = locator => locator.boundingBox();
  const local = locator => locator.evaluate(e => {const a=e.getBoundingClientRect(),b=e.closest('.sapid-app-shell').getBoundingClientRect();return {x:a.x-b.x,y:a.y-b.y,width:a.width,height:a.height};});
  const near = (a,b) => assert(Math.abs(a-b)<1, `${a} != ${b}`);
  const scroll = locator => locator.evaluate(e => ({ y:e.scrollTop, h:e.clientHeight, total:e.scrollHeight }));
  fs.mkdirSync(outputPath('shell-remediation'), { recursive:true });
  try {
    for (const theme of matrix.themes) for (const scheme of matrix.schemes) for (const width of [1280,390]) {
      const key = `${theme}-${scheme}-${width}`; await page.setViewportSize({width,height:1000});
      await page.goto(`${base}/showcase.html?widget=app-shell&theme=${theme}&scheme=${scheme}`);
      await page.waitForFunction(() => window.ready===true && document.documentElement.getAttribute('data-sapid-widget')==='app-shell'); await tick();
      report.asset = await page.evaluate(() => document.documentElement.innerHTML.match(/assets-[a-f0-9]{20}/)?.[0]); assert(report.asset);
      assert.equal(await page.locator('.site-component-detail').count(),6);
      assert(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth+1));
      const primary = page.locator('.site-preview .site-demo').first();
      const note = primary.getByLabel('Workspace note',{exact:true}); await note.fill('Persistent project draft');
      await note.evaluate(e=>{window.shellNote=e;});
      const trigger = primary.locator('.sapid-sidebar-trigger');
      await trigger.click(); await tick();
      if (width===390) { assert(await primary.getByRole('dialog').isVisible()); await page.keyboard.press('Escape'); await tick(); assert(await trigger.evaluate(e=>e===document.activeElement)); }
      else { near((await box(primary.locator('.sapid-sidebar'))).width,72); await trigger.click(); await tick(); near((await box(primary.locator('.sapid-sidebar'))).width,200); }
      assert.equal(await note.inputValue(),'Persistent project draft'); assert(await note.evaluate(e=>e===window.shellNote));

      const responsive=detail('Responsive navigation'); await responsive.scrollIntoViewIfNeeded();
      const rtoggle=responsive.locator('.sapid-sidebar-trigger');
      if(width===390){await rtoggle.click();await tick();}
      const draft=responsive.getByLabel('Navigation draft',{exact:true}); await draft.fill('Navigation draft survives resize'); await draft.evaluate(e=>{window.responsiveDraft=e;});
      await page.setViewportSize({width:width===390?1280:390,height:1000});await tick();
      if(width===1280){assert(await rtoggle.evaluate(e=>e===document.activeElement));await rtoggle.click();await tick();}
      assert.equal(await draft.inputValue(),'Navigation draft survives resize');assert(await draft.evaluate(e=>e===window.responsiveDraft));
      if(width===1280){await page.keyboard.press('Escape');await tick();}
      await page.setViewportSize({width,height:1000});await tick();
      if(width===390 && await responsive.getByRole('dialog').isVisible()){await page.keyboard.press('Escape');await tick();}

      const scrolling = detail('Independent scrolling and sections'); await scrolling.scrollIntoViewIfNeeded();
      const frame = scrolling.locator('.sapid-app-shell'), main = frame.locator('.sapid-shell-main');
      const nav = scrolling.getByRole('group',{name:'Project collections',exact:true});
      const header = frame.locator('.sapid-shell-header'), footer = frame.locator('.sapid-shell-footer'), bottom = frame.locator('.docs-nav-footer');
      const h0=await local(header),f0=await local(footer),b0=await local(bottom);
      near((await box(frame)).height,320); assert((await scroll(main)).total > (await scroll(main)).h+400);
      assert((await scroll(nav)).total > (await scroll(nav)).h+200);
      await main.focus(); await page.keyboard.press('PageDown'); await tick(); assert((await scroll(main)).y>0);
      await nav.focus(); await page.keyboard.press('End'); await tick(); assert((await scroll(nav)).y>200);
      near((await local(header)).y,h0.y); near((await local(footer)).y,f0.y); near((await local(bottom)).y,b0.y);
      await nav.press('Home'); await tick(); near((await scroll(nav)).y,0);
      await frame.evaluate(e=>e.scrollIntoView({block:'center'})); await frame.screenshot({path:outputPath(`shell-remediation/sections-${key}.png`)});

      const config = detail('Region dimensions, borders and visibility'); await config.scrollIntoViewIfNeeded();
      const cmain=config.locator('.sapid-shell-main'),cheader=config.locator('.sapid-shell-header'),cfooter=config.locator('.sapid-shell-footer'),aside=config.locator('.sapid-shell-aside');
      const editor=config.getByLabel('Task note',{exact:true}); await editor.fill('Task stays mounted'); await editor.evaluate(e=>{window.taskNote=e;});
      near((await box(cheader)).height,68); const before=await box(cmain);
      await config.getByRole('checkbox',{name:'Compact header',exact:true}).check(); await tick(); near((await box(cheader)).height,44);
      await config.getByRole('checkbox',{name:'Show details panel',exact:true}).check(); await tick(); near((await box(aside)).width,112); assert((await box(cmain)).width < before.width-100);
      await config.getByRole('checkbox',{name:'Region borders',exact:true}).uncheck(); await tick();
      for (const region of [cheader,cfooter,aside]) assert.equal(await region.evaluate(e=>[getComputedStyle(e).borderTopWidth,getComputedStyle(e).borderLeftWidth,getComputedStyle(e).borderBottomWidth].every(v=>v==='0px')),true);
      await config.getByRole('checkbox',{name:'Show footer',exact:true}).uncheck(); await tick(); assert(!(await cfooter.isVisible()));
      await config.getByRole('checkbox',{name:'Position header above body',exact:true}).check(); await tick();
      assert.equal(await cheader.evaluate(e=>getComputedStyle(e).position),'absolute'); near((await box(cmain)).y,(await box(cheader)).y+(await box(cheader)).height);
      await config.getByRole('checkbox',{name:'Compact header',exact:true}).uncheck(); await tick(); near((await box(cheader)).height,68); near((await box(cmain)).y,(await box(cheader)).y+68);
      assert.equal(await editor.inputValue(),'Task stays mounted'); assert(await editor.evaluate(e=>e===window.taskNote));
      await config.locator('.sapid-app-shell').evaluate(e=>e.scrollIntoView({block:'center'})); await config.locator('.sapid-app-shell').screenshot({path:outputPath(`shell-remediation/configuration-${key}.png`)});
      if (width===1280) {
        const controlled=detail('Collapsed desktop navigation'); await controlled.scrollIntoViewIfNeeded();
        const expand=controlled.getByRole('button',{name:'Expand navigation',exact:true});
        await controlled.getByRole('checkbox',{name:'Accept navigation changes',exact:true}).uncheck(); await expand.click(); await tick(); near((await box(controlled.locator('.sapid-sidebar'))).width,64);
        await controlled.getByRole('checkbox',{name:'Accept navigation changes',exact:true}).check(); await expand.click(); await tick(); near((await box(controlled.locator('.sapid-sidebar'))).width,180);
      }
      assert.equal(await detail('Full-window application frame').getByRole('link',{name:'Open full-window workspace',exact:true}).getAttribute('href'),'shell-window.html');
      report.pages.push({key:`documentation-${key}`,passed:true}); console.log(`PASS documentation-${key}`);

      await page.goto(`${base}/shell-window.html?theme=${theme}&scheme=${scheme}`); await page.waitForFunction(()=>window.ready===true); await tick();
      const shell=page.locator('.sapid-app-shell'); const smain=shell.getByRole('main');
      const sheader=shell.locator('.sapid-shell-header'),sfooter=shell.locator('.sapid-shell-footer');
      const rect=await box(shell); near(rect.x,0); near(rect.y,0); near(rect.width,width); near(rect.height,1000);
      assert.equal(await shell.evaluate(e=>getComputedStyle(e).position),'fixed');
      const sh0=await box(sheader),sf0=await box(sfooter); const project=page.getByLabel('Project note',{exact:true});
      await project.fill('Full-window draft'); await project.evaluate(e=>{window.fullNote=e;});
      await smain.focus(); await smain.press('End'); await tick(); assert((await scroll(smain)).y>300);
      near((await box(sheader)).y,sh0.y); near((await box(sfooter)).y,sf0.y);
      const toggle=shell.locator('.sapid-sidebar-trigger'); await toggle.click(); await tick();
      if (width===390) {const dialog=shell.getByRole('dialog'); assert(await dialog.isVisible()); await page.keyboard.press('Escape'); await tick(); assert(!(await dialog.isVisible())); assert(await toggle.evaluate(e=>e===document.activeElement));}
      else {near((await box(shell.locator('.sapid-sidebar'))).width,72);await toggle.click();await tick(); const snav=shell.getByRole('navigation'); await snav.focus();await snav.press('End');await tick();assert((await scroll(snav)).y>100);}
      assert(await project.evaluate(e=>e===window.fullNote)); assert.equal(await project.inputValue(),'Full-window draft');
      assert(await page.evaluate(()=>document.documentElement.scrollWidth <= innerWidth+1));
      await page.screenshot({path:outputPath(`shell-remediation/viewport-${key}.png`)});
      report.pages.push({key:`viewport-${key}`,passed:true}); console.log(`PASS viewport-${key}`);
    }
    assert.deepEqual(report.errors,[]);
  } catch(error) {report.failure=error.stack;await page.screenshot({path:outputPath('shell-remediation/failure.png'),fullPage:true});throw error;}
  finally {writeReport('shell-remediation.json',report);await browser.close();}
})().catch(error=>{console.error(error);process.exitCode=1;});
