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
    await page.goto(`${base}/showcase.html?widget=schedule&theme=blue&scheme=light`);
    await page.waitForFunction(() => window.ready === true
      && document.documentElement.getAttribute('data-sapid-widget') === 'schedule');

    for (const id of [
      'schedule-usage', 'schedule-controlled-view', 'schedule-current-window', 'schedule-recurrence',
    ]) await page.locator(`#${id}`).waitFor();
    report.checks.push('Four Schedule documentation sections are mounted');

    const day = page.getByRole('button', { name: 'Day', exact: true });
    const week = page.getByRole('button', { name: 'Week', exact: true });
    const month = page.getByRole('button', { name: 'Month', exact: true });
    const year = page.getByRole('button', { name: 'Year', exact: true });
    for (const control of [day, week, month, year]) assert.equal(await control.count(), 1);
    await day.evaluate(element => { element.dataset.scheduleAcceptance = 'stable'; });

    for (const [control, name, className] of [
      [day, 'Day', 'sapid-day-view'],
      [month, 'Month', 'sapid-month-view'],
      [year, 'Year', 'sapid-year-view'],
      [week, 'Week', 'sapid-week-view'],
    ]) {
      await control.click();
      await page.waitForFunction(({ name, className }) =>
        document.querySelector('.site-schedule-current-view')?.textContent === `Current view: ${name}`
          && document.querySelectorAll(`.${className}`).length === 1,
      { name, className });
    }
    assert.equal(await day.getAttribute('data-schedule-acceptance'), 'stable');
    report.checks.push('Controlled Day, Week, Month and Year buttons switch real keyed views without remounting the header');

    await day.click();
    await page.waitForFunction(() => document.querySelector('.site-schedule-current-view')?.textContent === 'Current view: Day');
    const before = await page.locator('.site-schedule-current-window').textContent();
    await page.getByRole('button', { name: 'Next schedule window', exact: true }).click();
    await page.waitForFunction(previous =>
      document.querySelector('.site-schedule-current-window')?.textContent !== previous, before);
    report.checks.push('Window navigation proposes and renders application-accepted finite data');

    assert.equal(await page.getByText('Daily standup', { exact: true }).count() >= 1, true);
    assert.equal(await page.getByText('Current window only', { exact: true }).count(), 1);
    report.checks.push('The recurrence example documents bounded current-window expansion');
    assert.deepEqual(report.errors, []);
  } finally {
    await browser.close();
    writeReport('schedule-showcase-browser.json', report);
  }
  console.log(JSON.stringify(report, null, 2));
})().catch(error => { console.error(error); process.exitCode = 1; });
