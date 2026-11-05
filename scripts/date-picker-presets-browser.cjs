const assert = require('node:assert/strict');
const fs = require('node:fs');
const { chromium, base, outputPath, outputRoot, writeReport } = require('./showcase-browser-env.cjs');


(async () => {
  const report = { checks: [], errors: [] };
  let browser;
  try {
    browser = await chromium.launch({ headless: true });
    const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
    page.on('pageerror', error => report.errors.push(String(error)));
    const open = async slug => {
      await page.goto(`${base}/showcase.html?widget=${slug}&theme=blue&scheme=light`);
      await page.waitForFunction(slug => window.ready === true
        && document.documentElement.getAttribute('data-sapid-widget') === slug, slug);
    };

    await open('date-picker');
    await page.getByRole('button', { name: 'Yesterday', exact: true }).click();
    await page.waitForFunction(() => [...document.querySelectorAll('[role="gridcell"]')]
      .some(cell => cell.getAttribute('aria-selected') === 'true' && cell.textContent.trim() === '28'));
    assert.equal(
      await page.getByRole('grid', { name: 'Appointment date', exact: true })
        .getByRole('gridcell', { name: 'Wednesday, February 28, 2024', exact: true })
        .getAttribute('aria-selected'),
      'true',
    );
    assert.equal(await page.locator('.sapid-date-picker-preset').count(), 2);
    report.checks.push('Inline DatePicker preset selects the real calendar cell');

    const columns = page.locator('#date-picker-columns').locator('..');
    assert.equal(await columns.getByRole('grid').count(), 2);
    assert.equal(await columns.getByRole('button', { name: 'Previous month', exact: true }).count(), 1);
    assert.equal(await columns.getByRole('button', { name: 'Next month', exact: true }).count(), 1);
    await columns.getByRole('button', { name: 'Wednesday, February 28, 2024', exact: true }).click();
    await columns.getByRole('button', { name: 'Saturday, March 2, 2024', exact: true }).click();
    await page.waitForFunction(() => document.querySelector('#date-picker-columns')?.parentElement
      ?.querySelectorAll('.sapid-calendar-cell[aria-selected="true"]').length === 4);
    assert.equal(await columns.locator('.sapid-calendar-cell[aria-selected="true"]').count(), 4);
    await columns.getByRole('button', { name: 'Next month', exact: true }).click();
    await page.waitForFunction(() => {
      const section = document.querySelector('#date-picker-columns')?.parentElement;
      const names = [...(section?.querySelectorAll('[role="grid"]') || [])].map(grid => grid.getAttribute('aria-label'));
      return names.some(name => name?.includes('April 2024')) && names.some(name => name?.includes('May 2024'));
    });
    report.checks.push('Two consecutive grids share one cross-month range and scroll by two columns');

    const levels = page.locator('#date-picker-levels').locator('..');
    assert.equal(await levels.locator('.sapid-date-picker-decade-level:visible').count(), 1);
    await levels.getByRole('button', { name: '2025', exact: true }).click();
    await levels.getByRole('button', { name: 'March 2025', exact: true }).click();
    await levels.getByRole('button', { name: 'Saturday, March 1, 2025', exact: true }).waitFor();
    assert.equal(await levels.getByRole('button', { name: 'Choose month or year', exact: true }).count(), 1);
    assert.equal(await levels.getByRole('grid', { name: 'Month-only date picker', exact: true }).count(), 1);
    assert.equal(await levels.getByRole('button', { name: 'Choose month or year', exact: true }).count(), 1);
    report.checks.push('DatePicker traverses real decade, year and month grids while maxLevel keeps the month-only example fixed');

    const calendarRows = page.locator('#date-picker-calendar-rows').locator('..');
    assert.equal(await calendarRows.locator('.sapid-calendar-week-number').count(), 6);
    assert.equal(await calendarRows.locator('.sapid-calendar-weekdays').count(), 1);
    assert.deepEqual(
      (await calendarRows.locator('.sapid-calendar-week-number').allTextContents()).map(value => value.trim()),
      ['5', '6', '7', '8', '9', '10'],
    );
    assert.equal(await calendarRows.locator('.sapid-calendar-day').count(), 84);
    report.checks.push('ISO week numbers and hidden weekday labels preserve all real day controls');

    await open('date-picker-input');
    await page.getByRole('button', { name: 'Choose appointment date', exact: true }).click();
    const dialog = page.getByRole('dialog', { name: 'Choose dates', exact: true });
    await dialog.waitFor();
    await dialog.getByRole('button', { name: 'Wednesday, November 4, 2026', exact: true }).click();
    await dialog.waitFor({ state: 'hidden' });
    assert.equal(
      await page.getByRole('textbox', { name: 'Appointment date', exact: true }).getAttribute('value'),
      '2026-11-04',
    );
    report.checks.push('DatePickerInput primary selects a real calendar day and closes the dialog');

    const inputColumns = page.locator('#date-picker-input-columns').locator('..');
    await inputColumns.getByRole('button', { name: 'Choose travel dates', exact: true }).click();
    const rangeDialog = page.getByRole('dialog', { name: 'Choose dates', exact: true });
    await rangeDialog.waitFor();
    assert.equal(await rangeDialog.getByRole('grid').count(), 2);
    report.checks.push('DatePickerInput forwards the two-column popup calendar');

    const formats = page.locator('#date-picker-value-format').locator('..');
    assert.equal(
      await formats.getByRole('textbox', { name: 'Day first appointment', exact: true }).inputValue(),
      '04/11/2026',
    );
    assert.equal(
      await formats.getByRole('textbox', { name: 'Month first dates', exact: true }).inputValue(),
      '11/04/2026, 11/06/2026',
    );
    assert.equal(
      await formats.getByRole('textbox', { name: 'Day first range', exact: true }).inputValue(),
      '04/11/2026 – 06/11/2026',
    );
    report.checks.push('DatePickerInput renders finite single, multiple and range value formats without stringifying its model');

    await open('date-time-picker');
    const dateTimeControls = page.locator('#date-time-picker-calendar-controls').locator('..');
    assert.equal(await dateTimeControls.locator('.sapid-date-picker-decade-level:visible').count(), 1);
    assert.equal(await dateTimeControls.locator('.sapid-date-picker-year-level:visible').count(), 1);
    await dateTimeControls.getByRole('button', { name: 'Choose date and time', exact: true }).click();
    const dateTimeDialog = page.getByRole('dialog', { name: 'Date and time', exact: true });
    await dateTimeDialog.waitFor();
    assert.equal(await dateTimeDialog.locator('.sapid-calendar-grid:visible').count(), 2);
    assert.equal(await dateTimeDialog.locator('.sapid-calendar-week-number:visible').count(), 12);
    await dateTimeDialog.getByRole('button', { name: 'Close date and time', exact: true }).click();
    await dateTimeControls.getByRole('button', { name: '2027', exact: true }).click();
    await dateTimeControls.getByRole('button', { name: 'November 2027', exact: true }).click();
    await dateTimeControls.getByRole('button', { name: 'Monday, November 1, 2027', exact: true }).waitFor();
    assert.equal(await dateTimeControls.locator('.sapid-calendar-grid:visible').count(), 1);
    report.checks.push('DateTimePicker and InlineDateTimePicker forward real levels, two columns and week rows');
    assert.deepEqual(report.errors, []);
  } catch (error) {
    report.errors.push(error.stack || String(error));
  } finally {
    if (browser) await browser.close();
    writeReport('date-picker-presets-browser.json', report);
  }
  console.log(JSON.stringify(report, null, 2));
  if (report.errors.length) process.exitCode = 1;
})();
