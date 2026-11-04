const assert = require('node:assert/strict');
const fs = require('node:fs');
const { chromium, base, outputPath, outputRoot, writeReport } = require('./showcase-browser-env.cjs');
(async () => {
  const browser = await chromium.launch({ headless: true, ...(process.env.GLORY_CHROME ? { executablePath: process.env.GLORY_CHROME } : {}) });
  const report = { checks: [], errors: [] };
  try {
    const page = await browser.newPage(); page.on('pageerror', error => report.errors.push(error.message));
    await page.goto(`${base}/showcase.html?widget=time-picker&theme=blue&scheme=light`);
    await page.waitForFunction(() => window.ready && document.documentElement.getAttribute('data-sapid-widget') === 'time-picker');
    const sections = ['usage','controlled','format','duration','duration-digits','paste','dropdown','steps','grid','presets','grouped-presets','controlled-popup','labels-focus','popup-layout','states'];
    for (const id of sections) assert.equal(await page.locator(`#time-picker-${id}`).count(), 1, id);
    report.checks.push('All fifteen TimePicker behavior sections are mounted');

    const section = id => page.locator(`#${id}`).locator('..');
    const format = section('time-picker-format');
    assert.deepEqual(await format.getByRole('spinbutton').evaluateAll(es => es.map(e => e.value)), ['18','45','30','06','45','','','']);
    assert.deepEqual(await format.getByRole('spinbutton').evaluateAll(es => es.slice(-3).map(e => e.placeholder)), ['hh','mm','ss']);
    const duration = section('time-picker-duration');
    assert.deepEqual(await duration.getByRole('spinbutton').evaluateAll(es => es.map(e => e.value)), ['125','30','45']);
    const digits = section('time-picker-duration-digits');
    assert.deepEqual(await digits.getByRole('spinbutton', { name: 'Hours' }).evaluateAll(es => es.map(e => e.value)), ['07','0007']);
    report.checks.push('Second precision, 12-hour format, placeholders and typed durations render exact segment values');

    const presets = section('time-picker-presets');
    await presets.getByRole('button', { name: 'Choose time', exact: true }).click();
    await page.locator('.sapid-popover:visible .sapid-time-picker-preset').filter({ hasText: '12:00' }).click();
    await page.waitForTimeout(50);
    assert.deepEqual(await presets.getByRole('spinbutton').evaluateAll(es => es.map(e => e.value)), ['12','00']);
    report.checks.push('Preset activation uses the same accepted segmented value path');
    await page.keyboard.press('Escape');

    const dropdown = section('time-picker-dropdown');
    await dropdown.getByRole('button', { name: 'Choose time', exact: true }).first().click();
    await page.locator('.sapid-time-picker-controls').waitFor();
    assert.equal(await page.locator('.sapid-time-picker-controls').count(), 1);
    await page.keyboard.press('Escape');
    const controlled = section('time-picker-controlled-popup');
    await controlled.getByRole('button', { name: 'Choose time', exact: true }).click();
    await controlled.getByText('Preset popup: open', { exact: true }).waitFor();
    await page.keyboard.press('Escape');
    await controlled.getByText('Preset popup: closed', { exact: true }).waitFor();
    report.checks.push('Controls dropdown and controlled popup open and dismiss through real anchored layers');

    const states = section('time-picker-states');
    assert.equal(await states.locator('[role="spinbutton"]:visible').count(), 4);
    assert.equal(await states.getByRole('spinbutton').evaluateAll(es => es.filter(e => e.readOnly).length), 2);
    assert.equal(await states.getByRole('spinbutton').evaluateAll(es => es.filter(e => e.closest('fieldset[disabled]')).length), 2);
    report.checks.push('Readonly and disabled state reaches every visible segment');
    assert.deepEqual(report.errors, []);
  } finally { await browser.close(); writeReport('time-picker-showcase-browser.json', report); }
  console.log(JSON.stringify(report, null, 2));
})().catch(error => { console.error(error); process.exitCode = 1; });
