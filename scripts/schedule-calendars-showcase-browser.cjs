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
    for (const [slug, rootClass] of [
      ['month-view', 'sapid-month-view'],
      ['year-view', 'sapid-year-view'],
      ['mobile-month-view', 'sapid-mobile-month-view'],
    ]) {
      const page = await browser.newPage({ viewport: { width: 1440, height: 1000 } });
      page.on('pageerror', error => report.errors.push(`${slug}: ${error.message}`));
      await page.goto(`${base}/showcase.html?widget=${slug}&theme=blue&scheme=light`);
      await page.waitForFunction(expected => window.ready === true
        && document.documentElement.getAttribute('data-sapid-widget') === expected, slug);
      for (const id of ['calendar-usage', 'calendar-presentation', 'calendar-locale', 'calendar-contract']) {
        await page.locator(`#${id}`).waitFor();
      }
      assert.equal(await page.locator(`.${rootClass}`).count(), 2);
      const compact = page.locator('.site-schedule-calendar-compact');
      assert.equal(await compact.locator('.sapid-schedule-header').count(), 0);
      assert.equal(await compact.locator('.sapid-schedule-weekday-row').count(), 0);
      if (slug === 'mobile-month-view') {
        await page.getByRole('button', { name: /Thursday, February 29, 2024, 5 events/ }).first().click();
        await page.waitForFunction(() => document.querySelectorAll('.sapid-agenda-event').length >= 5);
      } else {
        assert.equal(await page.locator('.sapid-schedule-more').count() >= 1, true);
      }
      if (slug !== 'year-view') {
        const today = page.getByRole('button', { name: /Thursday, February 29, 2024, 5 events/ }).first();
        assert.equal(await today.evaluate(node => getComputedStyle(node).borderTopWidth), '2px');
      }
      report.checks.push(`${slug}: sections, bounded disclosure, compact calendar and explicit today`);
      await page.close();
    }
    assert.deepEqual(report.errors, []);
  } finally {
    await browser.close();
    writeReport('schedule-calendars-showcase-browser.json', report);
  }
  console.log(JSON.stringify(report, null, 2));
})().catch(error => { console.error(error); process.exitCode = 1; });
