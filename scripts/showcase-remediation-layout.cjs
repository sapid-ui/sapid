const assert=require('node:assert/strict');
const fs=require('node:fs');
const {matrix,chromium,base,outputPath,writeReport}=require('./showcase-browser-env.cjs');
(async()=>{
  const browser=await chromium.launch();const page=await browser.newPage();
  const report={browser:browser.version(),pages:[],errors:[]};
  page.on('pageerror',e=>report.errors.push(e.message));page.on('console',m=>{if(m.type()==='error')report.errors.push(m.text());});
  const tick=()=>page.evaluate(()=>new Promise(resolve=>{let n=0;const step=()=>++n===6?resolve():requestAnimationFrame(step);requestAnimationFrame(step);}));
  const detail=title=>page.locator('.site-component-detail').filter({has:page.getByRole('heading',{name:title,exact:true})});
  const box=l=>l.boundingBox();const near=(a,b)=>assert(Math.abs(a-b)<1,`${a} != ${b}`);
  const counts={'aspect-ratio':2,center:1,container:1,flex:2,grid:2,group:2,'simple-grid':2,space:1,stack:1,splitter:3};
  const capture=async(l,key)=>{await l.evaluate(e=>e.scrollIntoView({block:'center'}));await l.screenshot({path:outputPath(`layout-remediation/${key}.png`)});};
  const reflow=async(root,first,second,width,kind)=>{
    await first.fill('Keep this project draft');await first.evaluate(e=>{window.layoutEditor=e;});
    const check=async target=>{
      const a=await box(first),b=await box(second);
      if(target===390)assert(b.y>a.y+15,`${kind}: narrow row did not become a column`);
      else assert(b.x>a.x+15,`${kind}: wide items did not share a row`);
      assert(await first.evaluate(e=>e===window.layoutEditor));assert.equal(await first.inputValue(),'Keep this project draft');
    };
    await check(width);await page.setViewportSize({width:width===390?1280:390,height:1000});await tick();await check(width===390?1280:390);
    await page.setViewportSize({width,height:1000});await tick();await check(width);
  };
  fs.mkdirSync(outputPath('layout-remediation'),{recursive:true});
  try{
    for(const theme of matrix.themes)for(const scheme of matrix.schemes)for(const width of [1280,390])for(const slug of Object.keys(counts)){
      const key=`${slug}-${theme}-${scheme}-${width}`;await page.setViewportSize({width,height:1000});
      await page.goto(`${base}/showcase.html?widget=${slug}&theme=${theme}&scheme=${scheme}`);
      await page.waitForFunction(slug=>window.ready===true&&document.documentElement.getAttribute('data-sapid-widget')===slug,slug);await tick();
      report.asset=await page.evaluate(()=>document.documentElement.innerHTML.match(/assets-[a-f0-9]{20}/)?.[0]);assert(report.asset);
      assert.equal(await page.locator('.site-component-detail').count(),counts[slug]);
      assert(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth+1),`${key}: page overflow`);
      const primary=page.locator('.site-preview .site-demo').first();
      try {
      if(slug==='aspect-ratio'){
        const frame=primary.locator('.docs-aspect-frame'),image=frame.getByRole('img');let rect=await box(frame);assert(Math.abs(rect.width/rect.height-16/9)<.005);
        assert((await image.getAttribute('src')).includes('mountain-evening.svg'));assert.equal(await image.evaluate(e=>getComputedStyle(e).objectFit),'cover');await image.evaluate(e=>{window.ratioImage=e;});
        await primary.getByRole('checkbox',{name:'Portrait frame',exact:true}).check();await tick();rect=await box(frame);assert(Math.abs(rect.width/rect.height-.8)<.005);assert(await image.evaluate(e=>e===window.ratioImage));
        const fit=detail('Image fitting and cropping');assert.equal(await fit.getByRole('img',{name:'Complete mountains and cabin'}).evaluate(e=>getComputedStyle(e).objectFit),'contain');
        near((await box(detail('Flex constraints and frame dimensions').locator('.sapid-aspect-ratio'))).width,96);
      }else if(slug==='center'){
        const axes=detail('Axes, inline content and height');const both=axes.locator('.docs-center-both');const text=both.getByText('Horizontal and vertical',{exact:true});
        const outer=await box(both),inner=await box(text);near(inner.x+inner.width/2,outer.x+outer.width/2);near(inner.y+inner.height/2,outer.y+outer.height/2);
        const horizontal=axes.locator('.docs-center-horizontal');near((await box(horizontal.getByText('Horizontal only',{exact:true}))).y,(await box(horizontal)).y);
        assert.equal(await axes.locator('.sapid-center').last().evaluate(e=>getComputedStyle(e).display),'inline-flex');
      }else if(slug==='container'){
        const sizes=detail('Maximum widths, fluid layout and nesting');const small=sizes.locator('.docs-container-small > .sapid-container-content'),large=sizes.locator('.docs-container-large > .sapid-container-content'),fluid=sizes.locator('.docs-container-fluid');
        assert((await box(small)).width<=241);assert((await box(large)).width<=421);assert((await box(large)).width>(await box(small)).width+10);
        near((await box(fluid.locator('> .sapid-container-content'))).width,(await box(fluid)).width-32);
      }else if(slug==='flex'){
        const controls=detail('Direction, wrapping and spacing');const flex=controls.locator('.docs-flex-directions');await flex.evaluate(e=>{window.flexContainer=e;});
        await controls.getByRole('checkbox',{name:'Vertical direction',exact:true}).check();await controls.getByRole('checkbox',{name:'Larger gap',exact:true}).check();await tick();
        assert.equal(await flex.evaluate(e=>getComputedStyle(e).flexDirection),'column');assert.equal(await flex.evaluate(e=>getComputedStyle(e).gap),'24px');
        await controls.getByRole('checkbox',{name:'Wrap flex items',exact:true}).uncheck();await tick();assert.equal(await flex.evaluate(e=>getComputedStyle(e).flexWrap),'nowrap');assert(await flex.evaluate(e=>e===window.flexContainer));
        const responsive=detail('Measured responsive layout and alignment');await reflow(responsive,responsive.getByLabel('Responsive project name',{exact:true}),responsive.getByRole('button',{name:'Create project',exact:true}),width,'flex');
      }else if(slug==='group'){
        const growth=detail('Equal growth and long content');const actions=growth.locator('.docs-group-equal').getByRole('button');const a=await box(actions.nth(0)),b=await box(actions.nth(1)),c=await box(actions.nth(2));near(a.width,b.width);near(b.width,c.width);near(a.y,b.y);
        const long=growth.locator('.docs-group-long'),open=long.getByRole('button',{name:'Open',exact:true});assert((await box(open)).height<60);assert((await box(open)).x+(await box(open)).width<=(await box(long)).x+(await box(long)).width+1);
        const wrap=detail('Wrapping and local overflow');await wrap.getByRole('checkbox',{name:'Allow actions to wrap',exact:true}).uncheck();await tick();assert.equal(await wrap.locator('.docs-group-wrap').evaluate(e=>getComputedStyle(e).flexWrap),'nowrap');
        await wrap.getByRole('checkbox',{name:'Allow actions to wrap',exact:true}).check();await tick();assert.equal(await wrap.locator('.docs-group-wrap').evaluate(e=>getComputedStyle(e).flexWrap),'wrap');
      }else if(slug==='space'){
        const axes=detail('Horizontal/vertical spacing and parent gaps');const horizontal=axes.locator('.docs-space-horizontal');const before=await box(horizontal.getByText('Before',{exact:true})),after=await box(horizontal.getByText('After',{exact:true}));near(after.x-before.x-before.width,32);
        const vertical=axes.locator('.docs-space-vertical');const above=await box(vertical.getByText('Above',{exact:true})),below=await box(vertical.getByText('Below',{exact:true}));near(below.y-above.y-above.height,24);
        assert.equal(await axes.locator('.docs-space-gap').evaluate(e=>getComputedStyle(e).gap),'16px');
      }else if(slug==='stack'){
        const section=detail('Alignment, distribution and long content'),stack=section.locator('.docs-stack-distribution');await stack.evaluate(e=>{window.stackElement=e;});
        await section.getByRole('checkbox',{name:'Distribute through the available height',exact:true}).check();await tick();
        const rect=await box(stack),last=await box(stack.getByText('Deliver',{exact:true}));near(rect.height,240);near(last.y+last.height,rect.y+rect.height);
        assert.equal(await stack.evaluate(e=>getComputedStyle(e).alignItems),'flex-start');assert.equal(await stack.evaluate(e=>getComputedStyle(e).justifyContent),'space-between');assert(await stack.evaluate(e=>e===window.stackElement));
      }else if(slug==='grid'){
        const spans=detail('Spans, row spans and column offsets').locator('.docs-grid-spans'),cells=spans.locator('> .sapid-grid-item');const wide=await box(cells.nth(0)),side=await box(cells.nth(1)),offset=await box(cells.nth(3));
        assert(wide.width>side.width*1.8);assert(offset.x>wide.x+20);assert(offset.y>side.y);
        const responsive=detail('Responsive spans with stable content');await reflow(responsive,responsive.getByLabel('Grid project 1 note',{exact:true}),responsive.getByLabel('Grid project 2 note',{exact:true}),width,'grid');
      }else if(slug==='simple-grid'){
        const columns=detail('Column count, gaps and unequal content');for(const count of [2,3]){const cells=columns.locator(`.docs-simple-columns-${count} > .sapid-grid-item`);near((await box(cells.nth(0))).width,(await box(cells.nth(1))).width);}
        const responsive=detail('Measured columns without remounting');await reflow(responsive,responsive.getByLabel('Simple grid note 1',{exact:true}),responsive.getByLabel('Simple grid note 2',{exact:true}),width,'simple grid');
      }else if(slug==='splitter'){
        const editor=primary.getByLabel('Editor draft',{exact:true}),handle=primary.getByRole('separator',{name:'Resize editor and preview',exact:true});await editor.fill('Stable split draft');await editor.evaluate(e=>{window.splitEditor=e;});
        await handle.focus();await handle.press('Home');await tick();near(Number(await handle.getAttribute('aria-valuenow')),20);await handle.press('End');await tick();near(Number(await handle.getAttribute('aria-valuenow')),80);assert(await editor.evaluate(e=>e===window.splitEditor));assert.equal(await editor.inputValue(),'Stable split draft');
        const controlled=detail('Controlled sizes, limits and keyboard input'),h=controlled.getByRole('separator',{name:'Resize controlled panels',exact:true});
        await controlled.getByRole('checkbox',{name:'Accept resize proposals',exact:true}).uncheck();await h.focus();await h.press('ArrowRight');await tick();near(Number(await h.getAttribute('aria-valuenow')),45);
        await controlled.getByRole('checkbox',{name:'Accept resize proposals',exact:true}).check();await h.focus();await h.press('ArrowRight');await tick();near(Number(await h.getAttribute('aria-valuenow')),50);
        await h.evaluate(e=>e.scrollIntoView({block:'center'}));let rect=await box(h);const frame=await box(controlled.locator('.sapid-splitter'));
        await page.mouse.move(rect.x+rect.width/2,rect.y+rect.height/2);await page.mouse.down();await page.mouse.move(frame.x-100,rect.y+rect.height/2,{steps:4});await page.mouse.up();await tick();near(Number(await h.getAttribute('aria-valuenow')),25);
        rect=await box(h);await page.mouse.move(rect.x+rect.width/2,rect.y+rect.height/2);await page.mouse.down();await page.mouse.move(frame.x+frame.width+100,rect.y+rect.height/2,{steps:4});await page.mouse.up();await tick();near(Number(await h.getAttribute('aria-valuenow')),75);
        const nested=detail('Nested splitters and multiple panels').getByRole('separator',{name:'Resize editor and terminal',exact:true});await nested.focus();await nested.press('ArrowDown');await tick();near(Number(await nested.getAttribute('aria-valuenow')),66);
      }
      assert(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth+1),`${key}: interaction overflow`);
      await capture(primary,key);report.pages.push({key,passed:true});console.log(`PASS ${key}`);
      } catch(error) {
        report.pages.push({key,passed:false,failure:error.stack});
        console.error(`FAIL ${key}: ${error.message}`);
        await capture(primary,`failure-${key}`);
      }
    }
    assert.equal(report.pages.filter(p=>!p.passed).length,0,'layout cases failed; inspect individual report entries');
    assert.deepEqual(report.errors,[]);
  }catch(error){report.failure=error.stack;await page.screenshot({path:outputPath('layout-remediation/failure.png'),fullPage:true});throw error;}
  finally{writeReport('layout-remediation.json',report);await browser.close();}
})().catch(error=>{console.error(error);process.exitCode=1;});
