// Read-only browser inventory and screenshot review of every shared guide.
const fs = require('node:fs');
const { chromium, base, outputPath, writeReport } = require('./showcase-browser-env.cjs');
const entries = JSON.parse(fs.readFileSync(outputPath('pages.json'), 'utf8'));
const headingsOnly = process.argv.includes('--headings-only');

(async () => {
  const browser = await chromium.launch({ headless: true });
  const report = { base, chromium: browser.version(), reviewDate: '2026-11-04', pages: [], errors: [] };
  const context = await browser.newContext({ viewport: { width: 1280, height: 1000 } });
  const page = await context.newPage();
  let active = null;
  page.on('pageerror', error => report.errors.push({ slug: active, kind: 'pageerror', message: error.message }));
  page.on('console', message => { if (message.type() === 'error') report.errors.push({ slug: active, kind: 'console', message: message.text() }); });
  fs.mkdirSync(outputPath('screenshots'), { recursive: true });
  try {
    for (const [index, entry] of entries.entries()) {
      active = entry.slug;
      const result = { slug: entry.slug, name: entry.name, group: entry.group, modes: {} };
      for (const [mode, width, height, scheme] of (headingsOnly ? [['desktop',1280,1000,'light']] : [
        ['desktop', 1280, 1000, 'light'], ['mobile', 390, 844, 'light'], ['dark', 1280, 1000, 'dark'],
      ])) {
        try {
          await page.setViewportSize({ width, height });
          await page.goto(`${base}/showcase.html?widget=${entry.slug}&theme=blue&scheme=${scheme}`, { waitUntil: 'domcontentloaded' });
          await page.waitForFunction(slug => window.ready === true && document.documentElement.getAttribute('data-sapid-widget') === slug, entry.slug, { timeout: 20000 });
          await page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))));
          const demo = page.locator('.site-preview .site-demo').first();
          await demo.waitFor({ state: 'visible', timeout: 10000 });
          result.modes[mode] = await page.evaluate(() => {
            const root = document.querySelector('.site-doc-main');
            const demo = root.querySelector('.site-preview .site-demo');
            const box = element => { const r = element.getBoundingClientRect(); return { x: r.x, y: r.y, width: r.width, height: r.height }; };
            const text = demo.innerText;
            const elements = [...demo.querySelectorAll('*')];
            const visible = element => { const r = element.getBoundingClientRect(); return r.width > 0 && r.height > 0 && getComputedStyle(element).visibility !== 'hidden'; };
            const controls = elements.filter(element => visible(element) && element.matches('button,input,select,textarea,[role=slider],[role=switch]'));
            return {
              title: root.querySelector('h1')?.textContent,
              headings: [...root.querySelectorAll('h2.site-section-title')].map(element => element.textContent),
              details: [...root.querySelectorAll('.site-component-detail')].map(element => ({ title: element.querySelector('h2')?.textContent, note: element.querySelector('.site-doc-note')?.textContent })),
              primaryText: text, primaryBox: box(demo),
              primaryHeadings: [...demo.querySelectorAll('h1,h2,h3,h4,h5,h6,[role=heading]')].map(element=>element.textContent),
              controls: controls.map(element => ({ tag: element.tagName, role: element.getAttribute('role'), type: element.getAttribute('type'), name: element.getAttribute('aria-label'), text: element.textContent.slice(0,100), disabled: element.disabled, box: box(element) })),
              graphicCount: demo.querySelectorAll('svg,img,canvas').length,
              elementCount: elements.length,
              documentOverflow: Math.max(0, document.documentElement.scrollWidth - innerWidth),
              previewOverflow: Math.max(0, demo.scrollWidth - demo.clientWidth),
              empty: !text.trim() && controls.length === 0 && demo.querySelectorAll('svg,img,canvas').length === 0,
              fixtureMarkers: [...new Set((text.match(/(?:requests=|creations=|ordered=|revision=|Before [^\n]{1,40}|After [^\n]{1,40}|Reject [^\n]{1,45}|protocol[^\n]{0,45}|fixture[^\n]{0,45})/gi) || []))],
              overflowingElements: elements.filter(element => visible(element) && element.getBoundingClientRect().right > demo.getBoundingClientRect().right + 2 && !element.closest('[popover],dialog,[hidden]')).slice(0,20).map(element => ({tag:element.tagName, classes:element.className?.baseVal ?? element.className, text:element.textContent.slice(0,80), box:box(element)})),
            };
          });
          if (!headingsOnly) await demo.screenshot({ path: outputPath(`screenshots/${entry.slug}-${mode}.png`), timeout: 15000, animations: 'disabled' });
          result.modes[mode].screenshot = `screenshots/${entry.slug}-${mode}.png`;
        } catch (error) {
          result.modes[mode] = { ...result.modes[mode], error: error.message };
          report.errors.push({ slug: entry.slug, mode, kind: 'audit', message: error.message });
        }
      }
      report.pages.push(result);
      writeReport(headingsOnly ? 'local-headings.json' : 'local-pages.json', report);
      console.log(JSON.stringify({index:index+1,slug:entry.slug,details:result.modes.desktop?.details?.length,
        mobileOverflow:result.modes.mobile?.documentOverflow,previewOverflow:result.modes.mobile?.previewOverflow,
        markers:result.modes.desktop?.fixtureMarkers,error:Object.values(result.modes).some(mode=>mode.error)}));
    }
  } finally { await browser.close(); writeReport(headingsOnly ? 'local-headings.json' : 'local-pages.json', report); }
  console.log(JSON.stringify({pages: report.pages.length, modes:report.pages.reduce((total,page)=>total+Object.keys(page.modes).length,0), errors: report.errors.length}));
})().catch(error => { console.error(error); process.exitCode = 1; });
