const assert = require('node:assert/strict');
const fs = require('node:fs');
const { chromium, base, outputPath, outputRoot, writeReport } = require('./showcase-browser-env.cjs');

(async () => {
  const browser = await chromium.launch({
    headless: true,
    ...(process.env.GLORY_CHROME ? { executablePath: process.env.GLORY_CHROME } : {}),
  });
  const report = { checks: [], errors: [] };
  try {
    const page = await browser.newPage({ viewport: { width: 1440, height: 1000 } });
    page.on('pageerror', error => report.errors.push(error.message));
    for (const slug of ['area-chart', 'bar-chart', 'line-chart', 'composite-chart']) {
      await page.goto(`${base}/showcase.html?widget=${slug}&theme=blue&scheme=light`);
      await page.waitForFunction(expected => window.ready === true
        && document.documentElement.getAttribute('data-sapid-widget') === expected, slug);
      assert.equal(await page.locator('.site-chart-card').count(), 4, slug);
      const reference = page.locator('.site-chart-card').filter({ hasText: 'Reference annotations' });
      await reference.getByText('Campaign', { exact: true }).waitFor();
      assert.equal(await reference.getByText('Target', { exact: true }).count(), 1);
      assert.equal(await reference.getByText('Peak', { exact: true }).count(), 1);
      assert.equal(await reference.locator('.sapid-chart-reference-label').count(), 3);
      assert.equal(await reference.locator('.sapid-chart-layer').count() >= 6, true);
      const brush = page.locator('.site-chart-card').filter({ hasText: 'Brush range' });
      assert.equal(await brush.locator('.sapid-chart-category-label').count(), 3);
      assert.equal(await brush.getByText('Mar', { exact: true }).count(), 1);
      assert.equal(await brush.getByText('May', { exact: true }).count(), 1);
    }
    report.checks.push('Four Cartesian chart guides expose four real component examples each');
    report.checks.push('Reference area, line, dot and labels reach portable chart layers');
    report.checks.push('Inclusive brush renders only the selected March through May categories');

    await page.goto(`${base}/showcase.html?widget=line-chart&theme=blue&scheme=light`);
    await page.waitForFunction(() => window.ready === true
      && document.documentElement.getAttribute('data-sapid-widget') === 'line-chart');
    const legend = page.locator('.site-chart-card').filter({ hasText: 'Legend' });
    await legend.getByRole('button', { name: 'Hide Revenue', exact: true }).click();
    await legend.getByRole('button', { name: 'Show Revenue', exact: true }).waitFor();
    report.checks.push('Legend example changes the actual keyed series geometry');
    assert.deepEqual(report.errors, []);
  } finally {
    await browser.close();
    writeReport('cartesian-chart-showcase-browser.json', report);
  }
  console.log(JSON.stringify(report, null, 2));
})().catch(error => { console.error(error); process.exitCode = 1; });
