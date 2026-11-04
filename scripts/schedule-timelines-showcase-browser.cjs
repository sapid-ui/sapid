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
    for (const [slug, rootClass] of [['day-view', 'sapid-day-view'], ['week-view', 'sapid-week-view']]) {
      const page = await browser.newPage({ viewport: { width: 1440, height: 1000 } });
      page.on('pageerror', error => report.errors.push(`${slug}: ${error.message}`));
      await page.goto(`${base}/showcase.html?widget=${slug}&theme=blue&scheme=light`);
      await page.waitForFunction(expected => window.ready === true
        && document.documentElement.getAttribute('data-sapid-widget') === expected, slug);
      for (const id of ['timeline-usage', 'timeline-overlap', 'timeline-presentation', 'timeline-contract']) {
        await page.locator(`#${id}`).waitFor();
      }
      assert.equal(await page.locator(`.${rootClass}`).count(), 2);
      assert.equal(await page.locator(`.${rootClass}`).first().locator('.sapid-agenda-event').count() >= 1, true);
      const compact = page.locator('.site-schedule-timeline-compact');
      assert.equal(await compact.locator('.sapid-schedule-header').count(), 0);
      assert.equal(await compact.locator('.sapid-agenda-event').count(), 0);
      assert.equal(await compact.locator('.sapid-schedule-slot').first().evaluate(node => getComputedStyle(node).height), '60px');
      assert.equal(await page.locator(`.${rootClass}`).first().locator('.sapid-schedule-slot').first()
        .evaluate(node => getComputedStyle(node).height), '44px');
      report.checks.push(`${slug}: sections, exact timeline geometry, overlap/all-day and compact options`);
      await page.close();
    }
    assert.deepEqual(report.errors, []);
  } finally {
    await browser.close();
    writeReport('schedule-timelines-showcase-browser.json', report);
  }
  console.log(JSON.stringify(report, null, 2));
})().catch(error => { console.error(error); process.exitCode = 1; });
