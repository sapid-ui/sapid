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
    await page.goto(`${base}/showcase.html?widget=resources-schedule&theme=blue&scheme=light`);
    await page.waitForFunction(() => window.ready === true
      && document.documentElement.getAttribute('data-sapid-widget') === 'resources-schedule');

    for (const id of [
      'resources-schedule-usage', 'resources-schedule-controlled',
      'resources-schedule-editing', 'resources-schedule-window',
    ]) await page.locator(`#${id}`).waitFor();
    report.checks.push('Four ResourcesSchedule documentation sections are mounted');

    const day = page.getByRole('button', { name: 'Day', exact: true });
    const week = page.getByRole('button', { name: 'Week', exact: true });
    const month = page.getByRole('button', { name: 'Month', exact: true });
    await day.evaluate(element => { element.dataset.resourcesAcceptance = 'stable'; });
    for (const [control, name, className] of [
      [week, 'Week', 'sapid-resources-week-view'],
      [month, 'Month', 'sapid-resources-month-view'],
      [day, 'Day', 'sapid-resources-day-view'],
    ]) {
      await control.click();
      await page.waitForFunction(({ name, className }) =>
        document.querySelector('.site-resources-current-view')?.textContent
          === `Current resource view: ${name}`
          && document.querySelectorAll(`.${className}`).length === 1,
      { name, className });
    }
    assert.equal(await day.getAttribute('data-resources-acceptance'), 'stable');
    report.checks.push('Controlled Day, Week and Month resource views replace keyed children under one stable header');

    await page.getByRole('button', { name: /^Edit Planning ·/ }).click();
    await page.getByRole('button', { name: 'Move later', exact: true }).click();
    await page.getByText('Proposed edit: Move', { exact: true }).waitFor();
    report.checks.push('Permission-checked editing publishes a real typed move proposal');

    const windowLabel = page.locator('.sapid-resources-window');
    const before = await windowLabel.textContent();
    await page.getByRole('button', { name: 'Next date window', exact: true }).click();
    await page.waitForFunction(previous =>
      document.querySelector('.sapid-resources-window')?.textContent !== previous, before);
    report.checks.push('Date navigation renders application-accepted finite resource data');
    assert.deepEqual(report.errors, []);
  } finally {
    await browser.close();
    writeReport('resources-schedule-showcase-browser.json', report);
  }
  console.log(JSON.stringify(report, null, 2));
})().catch(error => { console.error(error); process.exitCode = 1; });
