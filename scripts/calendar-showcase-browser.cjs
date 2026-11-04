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
    await page.goto(`${base}/showcase.html?widget=calendar&theme=blue&scheme=light`);
    await page.waitForFunction(() => window.ready === true
      && document.documentElement.getAttribute('data-sapid-widget') === 'calendar');

    for (const id of [
      'calendar-usage', 'calendar-custom-picker', 'calendar-full-width',
      'calendar-static', 'calendar-options',
    ]) await page.locator(`#${id}`).waitFor();
    report.checks.push('Five component-name Calendar documentation sections are mounted');

    const usage = page.getByRole('grid', { name: 'Calendar usage', exact: true });
    assert.equal(await usage.getByRole('button').count(), 42);
    assert.equal(await usage.locator('[role="gridcell"]').count(), 42);
    report.checks.push('Usage keeps the real 42-cell grid and roving day controls');

    const custom = page.getByRole('grid', { name: 'Pick up to three dates', exact: true });
    for (const day of [1, 2, 3, 4]) {
      await custom.getByRole('button', { name: new RegExp(`February ${day}, 2024`) }).click();
    }
    assert.equal(await custom.locator('[role="gridcell"][aria-selected="true"]').count(), 3);
    await page.getByText('Selected: 3 / 3', { exact: true }).waitFor();
    report.checks.push('Caller-owned DateSelection implements the bounded custom picker example');

    const full = page.getByRole('grid', { name: 'Full width calendar', exact: true });
    const fullOuter = full.locator('xpath=..');
    const [gridBox, outerBox] = await Promise.all([full.boundingBox(), fullOuter.boundingBox()]);
    assert(gridBox && outerBox);
    assert(Math.abs(gridBox.width - outerBox.width) <= 1, `${gridBox.width} !== ${outerBox.width}`);
    assert(gridBox.width > 600, `full width calendar is only ${gridBox.width}px`);
    report.checks.push('Full-width geometry reaches the actual calendar grid');

    const staticGrid = page.getByRole('grid', { name: 'Static event calendar', exact: true });
    assert.equal(await staticGrid.getByRole('button').count(), 0);
    assert.equal(await staticGrid.locator('.sapid-calendar-day-static').count(), 42);
    assert.equal(await staticGrid.getByText('16 •', { exact: true }).count(), 1);
    report.checks.push('Static mode renders passive custom day content without hidden buttons');

    const weeks = page.getByRole('grid', { name: 'Calendar with week numbers', exact: true });
    const noWeekdays = page.getByRole('grid', { name: 'Calendar without weekday header', exact: true });
    assert.equal(await weeks.locator('.sapid-calendar-week-number').count(), 6);
    assert.equal(await noWeekdays.locator('.sapid-calendar-weekdays').count(), 0);
    assert.deepEqual(report.errors, []);
    report.checks.push('Week numbers and weekday-header visibility remain real grid features');
  } finally {
    await browser.close();
    writeReport('calendar-showcase-browser.json', report);
  }
  console.log(JSON.stringify(report, null, 2));
})().catch(error => {
  console.error(error);
  process.exitCode = 1;
});
