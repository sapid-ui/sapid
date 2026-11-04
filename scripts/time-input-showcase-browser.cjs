const assert = require('node:assert/strict');
const fs = require('node:fs');
const { chromium, base, outputPath, outputRoot, writeReport } = require('./showcase-browser-env.cjs');

(async () => {
  const browser = await chromium.launch({ headless: true, ...(process.env.GLORY_CHROME ? { executablePath: process.env.GLORY_CHROME } : {}) });
  const report = { checks: [], errors: [] };
  try {
    const page = await browser.newPage();
    page.on('pageerror', error => report.errors.push(error.message));
    await page.goto(`${base}/showcase.html?widget=time-input&theme=blue&scheme=light`);
    await page.waitForFunction(() => window.ready && document.documentElement.getAttribute('data-sapid-widget') === 'time-input');

    assert.equal(await page.getByRole('textbox', { name: '24 hour minutes', exact: true }).inputValue(), '18:45');
    assert.equal(await page.getByRole('textbox', { name: '24 hour seconds', exact: true }).inputValue(), '18:45:30');
    assert.equal(await page.getByRole('textbox', { name: '12 hour seconds', exact: true }).inputValue(), '06:45:30 PM');
    assert.equal(await page.getByRole('textbox', { name: 'Chinese 12 hour seconds', exact: true }).inputValue(), '06:45:30 下午');
    report.checks.push('TimeInput showcase renders minute/second precision, 12/24-hour cycles and both finite locales');

    const empty = page.getByRole('textbox', { name: 'Empty time', exact: true });
    const prefilled = page.getByRole('textbox', { name: 'Prefilled time', exact: true });
    assert.equal(await empty.getAttribute('placeholder'), 'HH:MM');
    assert.equal(await prefilled.getAttribute('placeholder'), 'HH:MM');
    assert.equal(await prefilled.inputValue(), '09:15');
    await empty.fill('17:30');
    assert.equal(await empty.inputValue(), '17:30');
    report.checks.push('TimeInput showcase uses the real placeholder attribute and accepts a bounded valid draft');

    assert(await page.getByRole('textbox', { name: 'Readonly time', exact: true }).getAttribute('readonly') !== null);
    assert(await page.getByRole('textbox', { name: 'Disabled time', exact: true }).isDisabled());
    report.checks.push('Readonly, disabled and bounded state examples are mounted as real inputs');
    assert.deepEqual(report.errors, []);
  } finally {
    await browser.close();
    writeReport('time-input-showcase-browser.json', report);
  }
  console.log(JSON.stringify(report, null, 2));
})().catch(error => { console.error(error); process.exitCode = 1; });
