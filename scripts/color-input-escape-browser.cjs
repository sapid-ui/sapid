const assert = require('node:assert/strict');
const { chromium, base, writeReport } = require('./showcase-browser-env.cjs');
(async () => {
  const browser = await chromium.launch(); const page = await browser.newPage();
  const report = { browser: browser.version(), pages: [], errors: [] };
  page.on('pageerror', e => report.errors.push(e.message)); page.on('console', m => { if (m.type() === 'error') report.errors.push(m.text()); });
  const tick = () => page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))));
  try {
    for (const theme of ['blue', 'green']) for (const scheme of ['light', 'dark']) for (const width of [1280, 390]) {
      const key = `${theme}-${scheme}-${width}`; await page.setViewportSize({ width, height: 1000 });
      await page.goto(`${base}/showcase.html?widget=color-input&theme=${theme}&scheme=${scheme}`); await page.waitForFunction(() => window.ready === true && document.documentElement.getAttribute('data-sapid-widget') === 'color-input');
      const primary = page.locator('.site-preview .site-demo').first();
      const editor = primary.locator('.sapid-color-editor');
      const popup = primary.getByRole('dialog', { name: 'Choose color', exact: true });
      await editor.evaluate(e => { window.colorInputBefore = e; });
      for (let repeat = 0; repeat < 2; repeat++) {
        await editor.focus(); await popup.waitFor({ state: 'visible' }); await tick(); assert(await editor.evaluate(e => e === document.activeElement), 'opening preserves text editing focus');
        await popup.getByRole('slider', { name: 'Hue', exact: true }).focus(); await page.keyboard.press('ArrowRight');
        await page.keyboard.press('Escape'); await popup.waitFor({ state: 'hidden' }); await tick();
        assert(await editor.evaluate(e => e === window.colorInputBefore && e === document.activeElement), 'Escape closes and returns to the same editor without reopening');
        await page.keyboard.press('Tab'); await tick();
      }
      await editor.focus(); await popup.waitFor({ state: 'visible' });
      await page.getByRole('heading', { level: 1 }).click(); await popup.waitFor({ state: 'hidden' }); await tick();
      await editor.focus(); await popup.waitFor({ state: 'visible' }); assert(await editor.evaluate(e => e === window.colorInputBefore));
      report.pages.push({ key, passed: true }); console.log(`PASS ${key}`);
    }
    assert.deepEqual(report.errors, []);
  } finally { writeReport('color-input-escape.json', report); await browser.close(); }
})().catch(error => { console.error(error); process.exitCode = 1; });
