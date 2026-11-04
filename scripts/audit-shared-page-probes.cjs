// Focused reproduction of issues found in the complete screenshot pass.
const fs = require('node:fs');
const { chromium, base, outputPath, writeReport } = require('./showcase-browser-env.cjs');

(async () => {
  const browser = await chromium.launch({headless:true});
  const page = await browser.newPage({viewport:{width:1280,height:1000}});
  const report = {pages:[],errors:[]};
  page.on('pageerror',error=>report.errors.push(error.message));
  fs.mkdirSync(outputPath('probes'),{recursive:true});
  const tick=()=>page.evaluate(()=>new Promise(resolve=>requestAnimationFrame(()=>requestAnimationFrame(resolve))));
  try {
    for (const [slug,width] of [['pin-input',1280],['scroller',1280],['pagination',390],['affix',390],['background-image',390],['time-grid',390],['nav-link',1280],['transition',1280],['mini-calendar',1280],['heatmap',1280],['rich-text-editor',390],['accordion',1280],['tabs',1280],['select',1280],['menu',1280]]) {
      await page.setViewportSize({width,height:1000});
      await page.goto(`${base}/showcase.html?widget=${slug}`);
      await page.waitForFunction(slug=>window.ready===true&&document.documentElement.getAttribute('data-sapid-widget')===slug,slug);
      await tick();
      const result={slug,width,initial:await page.evaluate(()=>{
        const demo=document.querySelector('.site-demo');
        const box=element=>{const r=element.getBoundingClientRect(),s=getComputedStyle(element);return{text:element.textContent.slice(0,80),class:element.className,width:r.width,height:r.height,fontSize:s.fontSize,flexShrink:s.flexShrink,whiteSpace:s.whiteSpace}};
        return {text:demo.innerText.slice(0,1800),navLinkCount:demo.querySelectorAll('.sapid-nav-link').length,navigationMenuCount:demo.querySelectorAll('.sapid-navigation-menu').length,
          pinCells:[...demo.querySelectorAll('.sapid-pin-cell')].slice(0,6).map(box),
          scrollTiles:[...demo.querySelectorAll('.sapid-scroll-content *')].filter(element=>/^Item \d+$/.test(element.textContent)&&element.childElementCount<=1).slice(0,8).map(box),
          scrollable:[...demo.querySelectorAll('*')].filter(element=>element.scrollWidth>element.clientWidth+1).map(element=>({...box(element),scrollWidth:element.scrollWidth,clientWidth:element.clientWidth,scrollLeft:element.scrollLeft})).slice(0,8),
          buttons:[...demo.querySelectorAll('button')].filter(element=>{const r=element.getBoundingClientRect();return r.width>0&&r.height>0}).slice(0,16).map(box),
          calendarRows:demo.querySelectorAll('.sapid-calendar table tr').length,calendarDays:demo.querySelectorAll('.sapid-calendar-day').length,
          overflow:demo.scrollWidth-demo.clientWidth};
      })};
      if(slug==='scroller') {
        const next=page.getByRole('button',{name:'Next scroll page',exact:true}).first();
        result.nextEnabled=await next.isEnabled();
        if(result.nextEnabled){await next.click();await tick();}
        result.after=await page.locator('.site-demo').evaluate(element=>[...element.querySelectorAll('*')].filter(node=>node.scrollLeft>0).map(node=>({class:node.className,scrollLeft:node.scrollLeft})));
      }
      if(slug==='pagination') {
        await page.getByRole('button',{name:'Page 3',exact:true}).first().click();await tick();
        result.currentPage=await page.locator('.site-demo [aria-current=page]').first().innerText();
      }
      if(slug==='accordion') {
        const trigger=page.getByRole('button',{name:'How long does shipping take?',exact:true}).first();
        const before=await trigger.getAttribute('aria-expanded');await trigger.click();await tick();
        result.toggle={before,after:await trigger.getAttribute('aria-expanded')};
      }
      if(slug==='tabs') {
        const messages=page.getByRole('tab',{name:'Messages',exact:true}).first();
        await messages.click();await tick();result.messagesSelected=await messages.getAttribute('aria-selected');
      }
      if(slug==='select') {
        await page.locator('.site-demo').first().getByRole('combobox').first().click();await tick();
        result.openListboxes=await page.getByRole('listbox').count();
        result.optionCount=await page.getByRole('option').count();
      }
      if(slug==='menu') {
        await page.getByRole('button',{name:'Tools',exact:true}).first().click();await tick();
        result.openMenuNames=await page.getByRole('menu').evaluateAll(elements=>elements.map(element=>element.getAttribute('aria-label')));
        result.menuSearchInputs=await page.getByRole('menu').locator('input[type=text],input[type=search]').count();
      }
      const demo=page.locator('.site-demo').first();
      await demo.evaluate(element=>window.scrollTo(0,Math.max(0,scrollY+element.getBoundingClientRect().y-90)));
      await page.screenshot({path:outputPath(`probes/${slug}-${width}.png`)});
      report.pages.push(result);writeReport('probes.json',report);console.log(slug);
    }
  } finally {await browser.close();writeReport('probes.json',report);}
})().catch(error=>{console.error(error);process.exitCode=1});
