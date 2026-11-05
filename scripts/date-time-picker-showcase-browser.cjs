const assert = require('node:assert/strict');
const fs = require('node:fs');
const { chromium, base, outputPath, outputRoot, writeReport } = require('./showcase-browser-env.cjs');
(async () => {
  const browser = await chromium.launch({ headless: true, ...(process.env.GLORY_CHROME ? { executablePath: process.env.GLORY_CHROME } : {}) });
  const report = { checks: [], errors: [] };
  try {
    const page = await browser.newPage(); page.on('pageerror', error => report.errors.push(error.message));
    await page.goto(`${base}/showcase.html?widget=date-time-picker&theme=blue&scheme=light`);
    await page.waitForFunction(() => window.ready && document.documentElement.getAttribute('data-sapid-widget') === 'date-time-picker');
    const ids = ['actions','usage','seconds','presets','time-options','controlled','inline','calendar-controls','range','inline-range','states'];
    for (const id of ids) assert.equal(await page.locator(`#date-time-picker-${id}`).count(), 1, id);
    const section = id => page.locator(`#${id}`).locator('..');
    report.checks.push('All eleven DateTimePicker form and behavior sections are mounted');

    const seconds = section('date-time-picker-seconds');
    const secondsInput = seconds.getByRole('textbox', { name: 'Date and time with seconds', exact: true });
    assert.equal(await secondsInput.inputValue(), '04/11/2026 06:45:30 PM');
    assert.equal(await secondsInput.getAttribute('placeholder'), 'Choose date and time');
    await seconds.getByRole('button', { name: 'Choose date and time', exact: true }).click();
    assert.deepEqual(await seconds.locator('[role="spinbutton"]').evaluateAll(es => es.map(e => e.value)), ['06','45','30']);
    await seconds.getByRole('button', { name: 'Done', exact: true }).click();
    report.checks.push('Finite date display format, 12-hour second precision and placeholder agree with segmented controls');

    const controlled = section('date-time-picker-controlled');
    await controlled.getByRole('button', { name: 'Clear date and time', exact: true }).click();
    await controlled.getByText('No date and time selected', { exact: true }).waitFor();
    report.checks.push('Controlled clear proposes and reconciles one typed optional value');

    const inline = section('date-time-picker-inline');
    await inline.getByRole('button', { name: 'Evening', exact: true }).click();
    await page.waitForTimeout(50);
    assert.deepEqual(await inline.locator('[role="spinbutton"]').evaluateAll(es => es.map(e => e.value)), ['18','00']);
    report.checks.push('Inline preset selection updates the shared segmented time value');

    const calendar = section('date-time-picker-calendar-controls');
    assert(await calendar.getByRole('button', { name: '2026', exact: true }).count());
    assert.equal(await calendar.getByRole('textbox', { name: 'Date and time with two months', exact: true }).count(), 1);
    assert(await calendar.getByText('November 2026', { exact: true }).count());
    assert(await calendar.getByText('December 2026', { exact: true }).count());
    report.checks.push('Inline levels, popup two-column month configuration and range level controls are all real calendars');

    const range = section('date-time-picker-range');
    await range.getByRole('button', { name: 'Choose date and time range', exact: true }).click();
    await range.getByRole('button', { name: 'Conference', exact: true }).click();
    await range.getByRole('button', { name: 'Done', exact: true }).click();
    await range.getByText('2026-11-04T09:00:00 – 2026-11-06T17:30:00', { exact: true }).waitFor();
    report.checks.push('Range preset and Done commit one complete ordered pair atomically');

    const states = section('date-time-picker-states');
    assert(await states.getByRole('textbox', { name: 'Readonly appointment', exact: true }).getAttribute('readonly') !== null);
    assert(await states.getByRole('textbox', { name: 'Disabled appointment', exact: true }).isDisabled());
    report.checks.push('Readonly and disabled state reaches the outer editor and retained popup contract');
    assert.deepEqual(report.errors, []);
  } finally { await browser.close(); writeReport('date-time-picker-showcase-browser.json', report); }
  console.log(JSON.stringify(report, null, 2));
})().catch(error => { console.error(error); process.exitCode = 1; });
