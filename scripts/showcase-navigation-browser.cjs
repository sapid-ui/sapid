const fs=require('node:fs');
const assert=require('node:assert/strict');
const {chromium,base,outputPath,writeReport}=require('./showcase-browser-env.cjs');
(async()=>{let browser;const report={checks:[],errors:[]};const ok=s=>{report.checks.push(s);console.log(s)};try{
 browser=await chromium.launch({headless:true});const context=await browser.newContext({viewport:{width:1720,height:1200}});const page=await context.newPage();page.on('pageerror',e=>report.errors.push(String(e)));
 let documents=0;page.on('request',r=>{if(r.isNavigationRequest()&&r.frame()===page.mainFrame())documents++});
 const ready=async(widget,palette,scheme)=>page.waitForFunction(({widget,palette,scheme})=>{const h=document.documentElement;return window.ready===true&&h.getAttribute('data-sapid-widget')===widget&&(!palette||h.getAttribute('data-sapid-palette')===palette)&&(!scheme||h.getAttribute('data-sapid-scheme')===scheme)},{widget,palette,scheme});
 const input=name=>page.getByRole('textbox',{name,exact:true});
 const button=name=>page.getByRole('button',{name,exact:true});
 const until=async(fn,message)=>{for(let n=0;n<160;n++){if(await fn())return;await page.waitForTimeout(25)}throw Error(message)};
 const checkDocEdges=async mode=>{
  const viewport=page.viewportSize();
  for(const width of [320,390,450,451,600,800,801,1000,1001,1100,1200,1201,1440,1720,2200]){
   await page.setViewportSize({...viewport,width});
   const geometry=await page.evaluate(()=>{
    const box=selector=>document.querySelector(selector).getBoundingClientRect();
    const sidebar=box('.site-sidebar'),main=box('.site-doc-main'),toc=box('.site-toc');
    return {
     headerLeft:box('.site-brand-wrap').left,headerRight:box('.site-header-tools').right,
     docLeft:sidebar.width?sidebar.left:main.left,docRight:toc.width?toc.right:main.right,
     clientWidth:document.documentElement.clientWidth,scrollWidth:document.documentElement.scrollWidth,
    };
   });
   const label=`${mode} at ${width}px: ${JSON.stringify(geometry)}`;
   assert(Math.abs(geometry.docLeft-geometry.headerLeft)<1,label);
   assert(Math.abs(geometry.docRight-geometry.headerRight)<1,label);
   assert(geometry.scrollWidth<=geometry.clientWidth+1,label);
  }
  await page.setViewportSize(viewport);
  ok(`${mode} Showcase aligns the header and documentation edges from 320 to 2200px`);
 };
 const review=await context.newPage();review.on('pageerror',e=>report.errors.push(String(e)));
 const reviewReady=widget=>review.waitForFunction(widget=>window.ready===true&&document.documentElement.getAttribute('data-sapid-widget')===widget,widget);
 const focused=selector=>review.waitForFunction(selector=>document.querySelector(selector)===document.activeElement,selector);
 const verifyToc=async()=>{
  await review.locator('.site-toc').waitFor({state:'visible'});
  await review.waitForFunction(()=>document.querySelectorAll('.site-toc-link').length>0);
  const count=await review.locator('.site-toc-link').count();assert(count>0,'Documentation must expose a nonempty table of contents');
  const missing=await review.locator('.site-toc-link').evaluateAll(links=>links.map(link=>link.getAttribute('href')).filter(href=>!href?.startsWith('#')||!document.getElementById(href.slice(1))));
  assert.deepEqual(missing,[],'Every current table-of-contents destination must exist');
 };
 await review.goto(base+'/showcase.html?widget=button&theme=blue&scheme=dark');await reviewReady('button');
 await review.keyboard.press('Tab');const skip=review.getByRole('link',{name:'Skip to main content',exact:true});await skip.waitFor({state:'visible'});await focused('.site-skip-link');
 const skipBox=await skip.boundingBox();assert(skipBox.y>=0&&skipBox.y<review.viewportSize().height);
 await review.keyboard.press('Enter');await focused('.site-doc-main');
 await review.keyboard.press('Tab');await review.waitForFunction(()=>document.querySelector('.site-doc-main')?.contains(document.activeElement));
 const docViews=review.getByRole('group',{name:'Documentation views',exact:true});
 await docViews.getByRole('button',{name:'Properties',exact:true}).click();await review.waitForFunction(()=>{const group=document.querySelector('.site-doc-tabs');return [...group.querySelectorAll('button')].some(button=>button.textContent==='Properties'&&button.getAttribute('aria-pressed')==='true')&&[...group.querySelectorAll('button')].some(button=>button.textContent==='Documentation'&&button.getAttribute('aria-pressed')==='false')});
 await review.getByRole('heading',{name:'Button properties',exact:true}).waitFor();
 await docViews.getByRole('button',{name:'Documentation',exact:true}).click();await review.waitForFunction(()=>[...document.querySelectorAll('.site-doc-tabs button')].some(button=>button.textContent==='Documentation'&&button.getAttribute('aria-pressed')==='true'));await verifyToc();
 for(const [name,slug] of [['TextInput','text-input'],['Slider','slider']]){await review.locator('.site-sidebar .site-nav-link').filter({hasText:new RegExp(`^${name}$`)}).click();await reviewReady(slug);await verifyToc();}
 await review.goBack();await reviewReady('text-input');await verifyToc();await review.goBack();await reviewReady('button');await verifyToc();
 await review.goForward();await reviewReady('text-input');await verifyToc();await review.goForward();await reviewReady('slider');await verifyToc();
 ok('First Tab skips the catalogue into main; view buttons expose pressed state; TOC follows document navigation and history');
 const search=review.getByRole('textbox',{name:'Search widgets',exact:true}),results=review.getByRole('dialog',{name:'Search results',exact:true});
 const verifySearchGeometry=async()=>{
  await review.waitForFunction(()=>{
   const element=document.querySelector('.site-search-results'),target=document.getElementById('site-search-anchor');if(!element||!target)return false;
   const panel=element.getBoundingClientRect(),anchor=target.getBoundingClientRect();
   return getComputedStyle(element).position==='fixed'&&panel.width>0&&panel.height>0&&panel.left>=0&&panel.right<=innerWidth+1&&panel.top>=0&&panel.bottom<=innerHeight+1&&Math.abs(panel.top-anchor.bottom)<=20&&panel.left<anchor.right&&panel.right>anchor.left;
  });
  const geometry=await results.evaluate(element=>{
   const panel=element.getBoundingClientRect(),anchor=document.getElementById('site-search-anchor').getBoundingClientRect();
   return {position:getComputedStyle(element).position,left:panel.left,right:panel.right,top:panel.top,bottom:panel.bottom,anchorLeft:anchor.left,anchorRight:anchor.right,anchorBottom:anchor.bottom,width:innerWidth,height:innerHeight};
  });
  assert.equal(geometry.position,'fixed',JSON.stringify(geometry));
  assert(geometry.left>=0&&geometry.right<=geometry.width+1&&geometry.top>=0&&geometry.bottom<=geometry.height+1,JSON.stringify(geometry));
  assert(Math.abs(geometry.top-geometry.anchorBottom)<=20&&geometry.left<geometry.anchorRight&&geometry.right>geometry.anchorLeft,JSON.stringify(geometry));
 };
 await search.fill('Calendar');await results.waitFor();await verifySearchGeometry();await review.keyboard.press('Escape');await results.waitFor({state:'hidden'});assert.equal(await search.inputValue(),'Calendar');
 await review.getByRole('heading',{name:'Slider',exact:true}).click();await search.focus();await results.waitFor();
 await review.getByRole('heading',{name:'Slider',exact:true}).click();await results.waitFor({state:'hidden'});assert.equal(await search.inputValue(),'Calendar');
 await search.focus();await results.waitFor();await review.keyboard.press('Tab');await review.waitForFunction(()=>document.querySelector('.site-search-results')?.contains(document.activeElement));
 const resultCount=await results.locator('.site-search-result').count();for(let index=0;index<resultCount;index++)await review.keyboard.press('Tab');await results.waitFor({state:'hidden'});assert.equal(await search.inputValue(),'Calendar');
 ok('Search Escape, outside click and Tab departure close the shared layer without clearing query; refocus reopens it');
 await review.setViewportSize({width:390,height:844});await search.focus();await results.waitFor();await verifySearchGeometry();await review.keyboard.press('Escape');await results.waitFor({state:'hidden'});await review.getByRole('button',{name:'Open navigation',exact:true}).click();
 await review.getByRole('navigation',{name:'Widget navigation',exact:true}).getByRole('link',{name:'Theme studio',exact:true}).click();await reviewReady('theme-studio');
 ok('A narrow document drawer exposes Theme Studio directly');
 await review.setViewportSize({width:1720,height:1200});await review.goto(base+'/ssr-button-blue-dark.html');await reviewReady('button');await verifyToc();
 await review.keyboard.press('Tab');await focused('.site-skip-link');await review.keyboard.press('Enter');await focused('.site-doc-main');
 await review.close();ok('Hydrated documents preserve the same skip-to-main keyboard path');
 await page.goto(base+'/showcase.html?widget=theme-studio&theme=blue&scheme=dark');await ready('theme-studio','blue','dark');
 await page.evaluate(()=>{window.navigationIdentity={token:'same-document'};});const initialDocuments=documents;
 await input('Theme name').fill('My retained workspace');await input('Base canvas').fill('#1');await until(()=>button('Save & preview').isDisabled(),'invalid draft must disable save');
 await button('Edit Dark colors').click();await input('Dark canvas').waitFor();await input('Preview name').fill('Kept preview');
 await page.getByRole('link',{name:'Use green palette',exact:true}).click();await ready('theme-studio','green','dark');
 assert.equal(await input('Theme name').inputValue(),'My retained workspace');assert.equal(await input('Preview name').inputValue(),'Kept preview');assert(await input('Dark canvas').count());
 await page.getByRole('link',{name:'Switch to light theme',exact:true}).click();await ready('theme-studio','green','light');
 await button('Edit Base colors').click();await input('Base canvas').waitFor();assert.equal(await input('Base canvas').inputValue(),'#1');await input('Base canvas').fill('#faf7ffff');
 ok('Palette and scheme navigation preserve the unsaved draft, layer and preview values');
  await page.getByRole('navigation',{name:'Primary navigation'}).getByRole('link',{name:'Widgets',exact:true}).click();await ready('button','green','light');
 await checkDocEdges('CSR');
  await page.locator('.site-nav-link').filter({hasText:/^TextInput$/}).click();await ready('text-input','green','light');
 await input('Search widgets').fill('Calendar');const result=page.locator('.site-search-result').first();await result.waitFor();const target=new URL(await result.getAttribute('href'),base).searchParams.get('widget');await result.click();await ready(target,'green','light');
  await page.goBack();await ready('text-input','green','light');await page.goBack();await ready('button','green','light');await page.goBack();await ready('theme-studio','green','light');
 assert.equal(await input('Theme name').inputValue(),'My retained workspace');assert.equal(await input('Base canvas').inputValue(),'#faf7ffff');assert.equal(await input('Preview name').inputValue(),'Kept preview');
  await page.goForward();await ready('button','green','light');await page.goBack();await ready('theme-studio','green','light');
 assert.equal(documents,initialDocuments);assert.equal(await page.evaluate(()=>window.navigationIdentity?.token),'same-document');
 ok('Header, sidebar, search and browser back/forward use one document without WASM reload');
 const popupEvent=context.waitForEvent('page');await page.getByRole('navigation',{name:'Primary navigation'}).getByRole('link',{name:'Widgets',exact:true}).click({modifiers:['Control']});const popup=await popupEvent;await popup.waitForURL(/widget=button/);assert(popup.url().includes('widget=button'));await popup.close();assert.equal(documents,initialDocuments);ok('Control-click retains native new-tab behavior');
  await page.getByRole('navigation',{name:'Primary navigation'}).getByRole('link',{name:'Widgets',exact:true}).click();await ready('button','green','light');
 const filled=page.locator('.site-demo').getByRole('button',{name:'Filled button',exact:true});await filled.click();await page.getByText('Button presses: 1',{exact:true}).waitFor();const identity=await filled.getAttribute('data-gly-node');
  await page.getByRole('navigation',{name:'On this page'}).getByRole('link',{name:'Composition',exact:true}).click();await page.waitForURL(/#composition$/);await page.goBack();await ready('button','green','light');
 assert.equal(await filled.getAttribute('data-gly-node'),identity);assert(await page.getByText('Button presses: 1',{exact:true}).count());ok('Fragment history retains live widget identity and state');
  await page.goto(base+'/ssr-button-blue-dark.html');await ready('button','blue','dark');const afterSSR=documents;await page.evaluate(()=>window.navigationIdentity={token:'ssr-adopted'});
 await checkDocEdges('Hydrated');
  await page.locator('.site-nav-link').filter({hasText:/^TextInput$/}).click();await ready('text-input','blue','dark');await page.goBack();await ready('button','blue','dark');assert.equal(documents,afterSSR);assert.equal(await page.evaluate(()=>window.navigationIdentity?.token),'ssr-adopted');ok('Hydrated SSR pages join client routing and restore their original history entry');
 await page.getByRole('navigation',{name:'Primary navigation'}).getByRole('link',{name:'Theme studio',exact:true}).click();await ready('theme-studio','blue','dark');
 for(const width of [320,390,1720,2200]){await page.setViewportSize({width,height:1100});await page.waitForTimeout(50);const box=await page.locator('.studio-main').boundingBox();const geometry=await page.evaluate(()=>({width:document.documentElement.clientWidth,scroll:document.documentElement.scrollWidth,height:document.querySelector('.sapid-site').getBoundingClientRect().height}));assert(Math.abs(box.width-geometry.width)<=1,JSON.stringify({width,box,geometry}));assert(geometry.scroll<=width+1);assert(geometry.height>=1100-1);}
 await page.screenshot({path:outputPath('theme-studio-fluid-wide.png')});ok('Studio fills the viewport at 320–2200px and follows browser resizing');
 const migrationContext=await browser.newContext();const migration=await migrationContext.newPage();await migration.goto(base+'/showcase.html?widget=theme-studio');await migration.waitForFunction(()=>window.ready===true);await migration.getByRole('textbox',{name:'Theme name',exact:true}).fill('Legacy preview');await migration.getByRole('button',{name:'Save & preview',exact:true}).click();await migration.waitForURL(/theme=custom-1/);await migration.waitForFunction(()=>document.documentElement.getAttribute('data-sapid-palette')==='custom-1');
 const legacy=await migration.evaluate(()=>{let d=JSON.parse(localStorage.getItem('sapid.theme-studio.v1'));d.version=1;delete d.themes[0].base.colors.backdrop;delete d.themes[0].dark.colors.backdrop;d.themes[0].base.colors.canvas=[1,2,3,4];let s=JSON.stringify(d);localStorage.setItem('sapid.theme-studio.v1',s);return s});
 await migration.reload();await migration.waitForFunction(()=>window.ready===true);const upgraded=await migration.evaluate(()=>({data:JSON.parse(localStorage.getItem('sapid.theme-studio.v1')),backup:localStorage.getItem('sapid.theme-studio.v1.backup'),notice:document.querySelector('.site-host-notice').textContent}));assert.equal(upgraded.data.version,2);assert.equal(upgraded.backup,legacy);assert.deepEqual(upgraded.data.themes[0].base.colors.canvas,[1,2,3,4]);assert.equal(upgraded.notice,'');
 await migration.getByRole('link',{name:'Edit theme',exact:true}).click();await migration.getByRole('textbox',{name:'Theme name',exact:true}).waitFor();await migration.getByRole('textbox',{name:'Theme name',exact:true}).fill('Legacy updated');await migration.getByRole('button',{name:'Save & preview',exact:true}).click();await migration.waitForURL(/widget=button/);await migration.waitForFunction(()=>document.documentElement.getAttribute('data-sapid-widget')==='button');assert.equal(await migration.evaluate(()=>JSON.parse(localStorage.getItem('sapid.theme-studio.v1')).themes[0].title),'Legacy updated');
 await migration.getByRole('navigation',{name:'Primary navigation'}).getByRole('link',{name:'Theme studio',exact:true}).click();await migration.getByRole('textbox',{name:'Theme name',exact:true}).waitFor();
 await migration.getByRole('textbox',{name:'Theme name',exact:true}).fill('Updated twice');await migration.getByRole('button',{name:'Save & preview',exact:true}).click();await migration.waitForURL(/widget=button/);await migration.waitForFunction(()=>document.documentElement.getAttribute('data-sapid-widget')==='button');
 const saved=await migration.evaluate(()=>JSON.parse(localStorage.getItem('sapid.theme-studio.v1')));assert.equal(saved.themes.length,1);assert.equal(saved.themes[0].title,'Updated twice');

 await migration.getByRole('navigation',{name:'Primary navigation'}).getByRole('link',{name:'Theme studio',exact:true}).click();await migration.getByRole('textbox',{name:'Theme name',exact:true}).waitFor();
 await migration.getByRole('link',{name:'Use blue palette',exact:true}).click();await migration.waitForFunction(()=>document.documentElement.getAttribute('data-sapid-palette')==='blue');
 await migration.getByRole('button',{name:'Remove saved preview',exact:true}).click();await migration.getByRole('button',{name:'Remove saved preview',exact:true}).waitFor({state:'hidden'});
 assert.equal(await migration.getByRole('link',{name:'Use Updated twice palette',exact:true}).count(),0);
 assert.equal(await migration.evaluate(()=>JSON.parse(localStorage.getItem('sapid.theme-studio.v1')).themes.length),0);
 await migrationContext.close();ok('Old saved palettes migrate with an exact backup, repeated saves update, and removal works without changing route');
 const deletionContext=await browser.newContext();const deletion=await deletionContext.newPage();
 let deletionDocuments=0;deletion.on('request',r=>{if(r.isNavigationRequest()&&r.frame()===deletion.mainFrame())deletionDocuments++});
 deletion.on('pageerror',e=>report.errors.push(String(e)));
 const deletionReady=(widget,palette)=>deletion.waitForFunction(({widget,palette})=>window.ready===true&&document.documentElement.getAttribute('data-sapid-widget')===widget&&document.documentElement.getAttribute('data-sapid-palette')===palette,{widget,palette});
 const deletionSave=async name=>{await deletion.getByRole('textbox',{name:'Theme name',exact:true}).fill(name);await deletion.getByRole('button',{name:'Save & preview',exact:true}).click()};
 await deletion.goto(base+'/showcase.html?widget=theme-studio&scheme=dark');await deletionReady('theme-studio','blue');
  await deletionSave('Keep this theme');await deletionReady('button','custom-1');
 // A fresh page starts a separate workspace, so the second save creates another theme.
 await deletion.goto(base+'/showcase.html?widget=theme-studio&scheme=dark');await deletionReady('theme-studio','blue');
  await deletionSave('Delete this theme');await deletionReady('button','custom-2');const beforeDelete=deletionDocuments;
  await deletion.getByRole('button',{name:'Delete theme',exact:true}).click();await deletionReady('button','blue');
 assert.equal(new URL(deletion.url()).searchParams.get('scheme'),'dark');assert.equal(deletionDocuments,beforeDelete);
 assert.equal(await deletion.locator('.site-saved-actions').count(),0);
 assert.equal(await deletion.getByRole('link',{name:'Use Delete this theme palette',exact:true}).count(),0);
 assert.equal(await deletion.getByRole('link',{name:'Use Keep this theme palette',exact:true}).count(),1);
 assert.deepEqual(await deletion.evaluate(()=>JSON.parse(localStorage.getItem('sapid.theme-studio.v1')).themes.map(t=>t.id)),['custom-1']);
 await deletion.getByRole('navigation',{name:'Primary navigation'}).getByRole('link',{name:'Theme studio',exact:true}).click();await deletionReady('theme-studio','blue');
 assert.equal(await deletion.getByRole('textbox',{name:'Theme name',exact:true}).inputValue(),'Delete this theme');
 assert.equal(await deletion.getByRole('button',{name:'Remove saved preview',exact:true}).count(),0);
 await deletion.getByRole('link',{name:'Use Keep this theme palette',exact:true}).click();await deletionReady('theme-studio','custom-1');
 await deletion.getByRole('button',{name:'Delete theme',exact:true}).click();await deletionReady('theme-studio','blue');
 assert.equal(deletionDocuments,beforeDelete);assert.equal(await deletion.getByRole('textbox',{name:'Theme name',exact:true}).inputValue(),'Delete this theme');
 assert.equal(await deletion.evaluate(()=>JSON.parse(localStorage.getItem('sapid.theme-studio.v1')).themes.length),0);
 await deletion.reload();await deletionReady('theme-studio','blue');assert.equal(await deletion.locator('.site-saved-actions').count(),0);
 await deletionContext.close();ok('Header deletes only the current custom theme, preserves page, scheme and draft, and persists without a document reload');
 }catch(e){report.errors.push(e.stack||String(e))}finally{if(browser)await browser.close();writeReport('showcase-navigation-browser.json',report);console.log(JSON.stringify(report,null,2));if(report.errors.length)process.exitCode=1}})();
