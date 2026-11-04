const assert = require('node:assert/strict');
const fs = require('node:fs');
const { chromium, base, outputPath, outputRoot, writeReport } = require('./showcase-browser-env.cjs');

(async () => {
  const report = { checks: [], errors: [] };
  let browser;
  try {
    browser = await chromium.launch({ headless: true });
    const page = await browser.newPage({ viewport: { width: 1100, height: 760 } });
    page.on('pageerror', error => report.errors.push(String(error)));
    await page.goto(`${base}/showcase.html?widget=modal&theme=blue&scheme=light`);
    await page.waitForFunction(() => window.ready === true
      && document.documentElement.getAttribute('data-sapid-widget') === 'modal');

    await page.getByRole('button', { name: 'Open centered modal', exact: true }).click();
    const centered = page.getByRole('dialog', { name: 'Centered modal', exact: true });
    await centered.waitFor();
    const centeredBox = await centered.boundingBox();
    assert(Math.abs(centeredBox.x + centeredBox.width / 2 - 550) < 2);
    assert(Math.abs(centeredBox.y + centeredBox.height / 2 - 380) < 2);
    await centered.getByRole('button', { name: 'Close', exact: true }).click();
    report.checks.push('Ordinary modal uses the shared centered layer and closes normally');

    const trigger = page.getByRole('button', { name: 'Open full-screen modal', exact: true });
    await trigger.click();
    const full = page.getByRole('dialog', { name: 'Full-screen workspace', exact: true });
    await full.waitFor();
    const box = await full.boundingBox();
    assert.deepEqual(
      [Math.round(box.x), Math.round(box.y), Math.round(box.width), Math.round(box.height)],
      [0, 0, 1100, 760],
    );
    assert.equal(await full.evaluate(element => getComputedStyle(element).borderRadius), '0px');
    await page.keyboard.press('Escape');
    await full.waitFor({ state: 'hidden' });
    assert.equal(await trigger.evaluate(element => element === document.activeElement), true);
    report.checks.push('Full-screen modal fills the viewport and preserves Escape return focus');
    assert.deepEqual(report.errors, []);
  } catch (error) {
    report.errors.push(error.stack || String(error));
  } finally {
    if (browser) await browser.close();
    writeReport('modal-showcase-browser.json', report);
  }
  console.log(JSON.stringify(report, null, 2));
  if (report.errors.length) process.exitCode = 1;
})();
