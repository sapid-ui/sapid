const assert = require('node:assert/strict');
const fs = require('node:fs');
const {chromium} = require(process.env.SAPID_PLAYWRIGHT || 'D:/Works/glory-rs/glory/tests/playwright/node_modules/playwright');
process.env.TEMP = process.env.TMP = 'D:/BuildCache/sapid/tmp';
(async () => {
  const report = {checks: [], errors: []};
  const browser = await chromium.launch({headless: true});
  report.chromium = browser.version();
  try {
    const page = await browser.newPage({viewport: {width: 1440, height: 1100}});
    page.on('pageerror', e => report.errors.push(String(e)));
    page.on('console', e => {if(e.type() === 'error') report.errors.push(e.text())});
    let documents = 0;
    page.on('request', r => {if(r.isNavigationRequest() && r.frame() === page.mainFrame()) documents++});
    const base = process.env.SAPID_SHOWCASE_BASE || 'http://127.0.0.1:8795';
    const ready = id => page.waitForFunction(id => window.ready === true && document.documentElement.getAttribute('data-sapid-widget') === id, id);
    const tick = () => page.evaluate(() => new Promise(r => requestAnimationFrame(() => requestAnimationFrame(r))));
    const button = name => page.getByRole('button', {name, exact: true});
    const nav = name => page.locator('.site-sidebar').getByRole('link', {name, exact: true});
    const retainSidebar = async () => {
      const scroll = await page.locator('.site-sidebar').evaluate(e => {window.feedbackSidebar = e; window.feedbackScroll = e.scrollTop; return e.scrollTop;});
      assert(scroll > 100, 'exercise an actually scrolled sidebar');
    };
    const sameSidebar = async () => assert(await page.locator('.site-sidebar').evaluate(e => e === window.feedbackSidebar && Math.abs(e.scrollTop - window.feedbackScroll) < 1), 'sidebar node or scroll position changed');
    for(const hue of ['blue', 'green']) for(const scheme of ['light', 'dark']) {
      await page.goto(`${base}/showcase.html?widget=P5-05&theme=${hue}&scheme=${scheme}`); await ready('P5-05');
      const alert = page.locator('.site-demo .sapid-alert');
      assert.equal(await alert.locator('.sapid-alert-title').textContent(), 'Alert title');
      assert.equal(await alert.locator('.sapid-alert-icon').count(), 1);
      assert.equal(await alert.evaluate(e => getComputedStyle(e).textAlign), 'start');
      assert.equal(await alert.evaluate(e => getComputedStyle(e).borderTopWidth), '0px');
      const previewBox = await page.locator('.site-playground-preview').boundingBox();
      const controlsBox = await page.locator('.site-playground-controls').boundingBox();
      assert(controlsBox.x >= previewBox.x + previewBox.width && Math.abs(controlsBox.y - previewBox.y) < 10, 'desktop controls must stay to the right of the preview');
      for(const variant of ['filled', 'outline', 'transparent', 'light']) {
        await page.getByRole('combobox', {name: 'Variant', exact: true}).selectOption(variant); await tick();
        assert.equal(await alert.evaluate(e => getComputedStyle(e).borderTopWidth), variant === 'outline' ? '1px' : '0px');
      }
      await button('Use Red').click(); await tick();
      const red = await alert.evaluate(e => getComputedStyle(e).backgroundColor);
      assert.match(red, /^rgba\(250, 82, 82, /);
      assert(Math.abs(Number.parseFloat(red.split(',')[3]) - 0.1) < 0.01);
      await page.getByRole('slider', {name: 'Radius', exact: true}).focus(); await page.keyboard.press('End'); await tick();
      assert.equal(await alert.evaluate(e => getComputedStyle(e).borderRadius), '12px');
      await page.getByRole('textbox', {name:'Title',exact:true}).fill('Updated alert');
      await page.getByRole('textbox', {name:'Children',exact:true}).fill('Updated message'); await tick();
      assert.equal(await alert.locator('.sapid-alert-title').textContent(), 'Updated alert');
      assert.equal(await alert.locator('.sapid-alert-content').textContent(), 'Updated message');
      await page.getByRole('switch', {name:'With close button',exact:true}).click(); await tick();
      const close = button('Dismiss alert');
      const rect = await close.boundingBox(), root = await alert.boundingBox();
      assert(rect.x > root.x + root.width - 50 && rect.y < root.y + 25, 'close action must be at top right');
      await close.click(); await alert.waitFor({state:'hidden'});
      await button('Show alert').click(); await alert.waitFor();
      await page.getByRole('textbox',{name:'Title',exact:true}).fill(''); await tick();
      assert.equal(await alert.locator('.sapid-alert-title').count(),0);
      await page.getByRole('textbox',{name:'Children',exact:true}).fill(''); await tick();
      report.checks.push(`${hue}-${scheme}: Alert variants, colors, radius, text, empty title/message and dismiss/restore`);
      await nav('Skeleton').evaluate(e => e.scrollIntoView({block:'center'})); await retainSidebar();
      const before = documents; await nav('Skeleton').click(); await ready('P5-10'); await sameSidebar();
      await page.goBack(); await ready('P5-05'); await sameSidebar();
      await page.goForward(); await ready('P5-10'); await sameSidebar();
      assert.equal(documents, before);
      await nav('Overlay').click(); await ready('P5-11'); await sameSidebar();
      assert.equal(await nav('Overlay').getAttribute('aria-current'), 'page');
      await page.getByRole('textbox',{name:'Region editor',exact:true}).fill('Retained after load');
      await button('Open region dialog').click(); let dialog=page.getByRole('dialog',{name:'Region dialog',exact:true}); await dialog.waitFor();
      await dialog.getByRole('button',{name:'Close',exact:true}).click(); await dialog.waitFor({state:'hidden'});
      await button('Open region dialog').click(); await dialog.waitFor(); await page.keyboard.press('Escape'); await dialog.waitFor({state:'hidden'});
      await button('Open region dialog').click(); await dialog.waitFor();
      await dialog.getByRole('button',{name:'Load from region dialog',exact:true}).click(); await dialog.waitFor({state:'hidden'}); await tick();
      assert.equal(await page.locator('.loading-region').getAttribute('aria-busy'),'true');
      await button('Finish region loading').click(); await tick();
      assert.equal(await page.getByRole('textbox',{name:'Region editor',exact:true}).inputValue(),'Retained after load');
      await button('Open region dialog').click(); await dialog.waitFor(); await dialog.getByRole('button',{name:'Close',exact:true}).click(); await dialog.waitFor({state:'hidden'});
      report.checks.push(`${hue}-${scheme}: retained sidebar scroll/document/back/forward; dialog Close, Escape, load, recovery`);
      await nav('Alert').click(); await ready('P5-05');
      await page.screenshot({path:`D:/BuildCache/sapid/feedback-${hue}-${scheme}.png`,fullPage:true});
      await page.setViewportSize({width:390,height:844});await tick();
      assert(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth + 1),'mobile horizontal overflow');
      await page.setViewportSize({width:1440,height:1100});
    }
    await page.goto(`${base}/ssr-button-blue-dark.html`);await ready('P0-01');
    await nav('Alert').evaluate(e=>e.scrollIntoView({block:'center'}));await retainSidebar();const before=documents;
    await nav('Alert').click();await ready('P5-05');await sameSidebar();await page.goBack();await ready('P0-01');await sameSidebar();assert.equal(documents,before);
    report.checks.push('SSR hydration retains the sidebar across client navigation and history');
  } catch(e) {report.errors.push(e.stack || String(e))}
  finally {await browser.close();fs.writeFileSync('D:/BuildCache/sapid/showcase-feedback-browser.json',JSON.stringify(report,null,2));console.log(JSON.stringify(report,null,2));if(report.errors.length)process.exitCode=1}
})();
