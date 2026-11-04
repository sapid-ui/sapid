const assert = require('node:assert/strict');
const fs = require('node:fs');
const { chromium, base, outputPath, outputRoot, writeReport } = require('./showcase-browser-env.cjs');

(async () => {
  const browser = await chromium.launch({ headless: true, ...(process.env.GLORY_CHROME ? { executablePath: process.env.GLORY_CHROME } : {}) });
  const report = { checks: [], errors: [] };
  try {
    const page = await browser.newPage();
    page.on('pageerror', error => report.errors.push(error.message));
    await page.goto(`${base}/showcase.html?widget=date-input&theme=blue&scheme=light`);
    await page.waitForFunction(() => window.ready && document.documentElement.getAttribute('data-sapid-widget') === 'date-input');

    const iso = page.getByRole('textbox', { name: 'Editable appointment', exact: true });
    const dmy = page.getByRole('textbox', { name: 'Day month year format', exact: true });
    const mdy = page.getByRole('textbox', { name: 'Month day year format', exact: true });
    assert.equal(await iso.getAttribute('placeholder'), 'YYYY-MM-DD');
    assert.equal(await dmy.inputValue(), '29/02/2024');
    assert.equal(await dmy.getAttribute('placeholder'), 'DD/MM/YYYY');
    assert.equal(await mdy.getAttribute('placeholder'), 'MM/DD/YYYY');
    report.checks.push('DateInput showcase exposes ISO, day-first and month-first finite formats with real placeholders');

    await mdy.fill('02/29/');
    assert.equal(await mdy.inputValue(), '02/29/');
    await mdy.fill('02/29/2024');
    assert.equal(await mdy.inputValue(), '02/29/2024');
    await mdy.press('Enter');
    assert.equal(await mdy.inputValue(), '02/29/2024');
    report.checks.push('Month-first incomplete drafts remain editable and valid commits retain the selected format');

    const custom = page.getByRole('textbox', { name: 'Custom date parser', exact: true });
    await custom.fill('29/02/2024');
    assert.equal(await custom.inputValue(), '29/02/2024');
    assert.notEqual(await custom.getAttribute('aria-invalid'), 'true');
    report.checks.push('Custom parser accepts its documented bounded spelling without marking the draft invalid');
    assert.deepEqual(report.errors, []);
  } finally {
    await browser.close();
    writeReport('date-input-showcase-browser.json', report);
  }
  console.log(JSON.stringify(report, null, 2));
})().catch(error => { console.error(error); process.exitCode = 1; });
