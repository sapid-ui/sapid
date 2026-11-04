// Capture live official previews for visual comparison; no upstream state is changed.
const fs = require('node:fs');
const { chromium, outputPath, writeReport } = require('./showcase-browser-env.cjs');
const entries = JSON.parse(fs.readFileSync(outputPath('pages.json'),'utf8'));

(async () => {
  const browser = await chromium.launch({headless:true});
  const context = await browser.newContext({viewport:{width:1280,height:1000}});
  const page = await context.newPage();
  const report = {chromium:browser.version(),pages:[],errors:[]};
  fs.mkdirSync(outputPath('upstream-previews'),{recursive:true});
  try {
    for (const [index,entry] of entries.entries()) {
      const result = {slug:entry.slug,url:entry.upstream_url};
      try {
        await page.goto(entry.upstream_url,{waitUntil:'domcontentloaded',timeout:30000});
        await page.locator('main h1').first().waitFor({timeout:10000});
        await page.evaluate(()=>new Promise(resolve=>requestAnimationFrame(()=>requestAnimationFrame(resolve))));
        result.title = await page.locator('main h1').first().innerText();
        const demos = page.locator('main [data-with-padding]');
        result.demoCount = await demos.count();
        result.docHeadingCount = await page.locator('main h2[class*="MdxTitle"]').count();
        if (result.demoCount) {
          const demo = demos.first();
          result.preview = await demo.evaluate(element=>({text:element.innerText.slice(0,1600),width:element.getBoundingClientRect().width,height:element.getBoundingClientRect().height,controls:[...element.querySelectorAll('button,input,select,textarea,[role=slider]')].map(control=>({tag:control.tagName,role:control.getAttribute('role'),label:control.getAttribute('aria-label'),text:control.textContent.slice(0,80)}))}));
          result.screenshot = `upstream-previews/${entry.slug}.png`;
          await demo.screenshot({path:outputPath(result.screenshot),animations:'disabled',timeout:15000});
        } else {
          result.standardPreviewNotLocated = true;
        }
      } catch(error) { result.error=error.message;report.errors.push({slug:entry.slug,message:error.message}); }
      report.pages.push(result);writeReport('upstream-previews.json',report);
      if ((index+1)%20===0 || result.error) console.log(JSON.stringify({index:index+1,slug:entry.slug,demos:result.demoCount,error:result.error}));
    }
  } finally {await browser.close();writeReport('upstream-previews.json',report);}
  console.log(JSON.stringify({pages:report.pages.length,errors:report.errors.length}));
})().catch(error=>{console.error(error);process.exitCode=1});
