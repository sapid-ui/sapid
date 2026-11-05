const assert = require('node:assert/strict');
const fs = require('node:fs');
const { chromium, base, outputPath, writeReport } = require('./showcase-browser-env.cjs');
(async () => {
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
  const report = { browser: browser.version(), pages: [], errors: [] };
  page.on('pageerror', e => report.errors.push(e.message));
  page.on('console', m => { if (m.type() === 'error') report.errors.push(m.text()); });
  const tick = () => page.evaluate(() => new Promise(resolve => { let count = 0; const frame = () => ++count === 5 ? resolve() : requestAnimationFrame(frame); requestAnimationFrame(frame); }));
  const day = (root, name) => root.getByRole('button', { name, exact: true });
  const focus = locator => locator.evaluate(e => e === document.activeElement);
  const detail = title => page.locator('.site-component-detail').filter({ has: page.getByRole('heading', { name: title, exact: true }) });
  fs.mkdirSync(outputPath('mini-calendar-remediation'), { recursive: true });
  try {
    for (const theme of ['blue', 'green']) for (const scheme of ['light', 'dark']) for (const width of [1280, 390]) {
      const key = `${theme}-${scheme}-${width}`;
      await page.setViewportSize({ width, height: 1000 });
      await page.goto(`${base}/showcase.html?widget=mini-calendar&theme=${theme}&scheme=${scheme}`);
      await page.waitForFunction(() => window.ready === true && document.documentElement.getAttribute('data-sapid-widget') === 'mini-calendar');
      await tick();
      const primary = page.locator('.site-preview .site-demo').first();
      const strip = primary.locator('.sapid-mini-calendar');
      const grid = strip.getByRole('grid', { name: 'Appointment dates', exact: true });
      assert.equal(await grid.getByRole('row').count(), 1);
      assert.equal(await grid.getByRole('gridcell').count(), 6);
      assert.equal(await grid.getByRole('button').count(), 6);
      assert.equal(await page.locator('.site-component-detail').count(), 4);
      assert(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth + 1));
      assert(await strip.evaluate(e => e.scrollWidth <= e.clientWidth + 1), 'six dates fit the preview');
      const dates = grid.getByRole('button');
      await dates.evaluateAll(elements => { window.miniCalendarDays = elements; });
      const boxes = await dates.evaluateAll(elements => elements.map(e => { const r = e.getBoundingClientRect(); return { x: r.x, y: r.y, w: r.width, h: r.height }; }));
      assert(boxes.every(r => r.w >= 28 && r.h >= 44 && Math.abs(r.y - boxes[0].y) < 1));
      await day(grid, 'Friday, March 1, 2024').click(); await tick();
      assert(await primary.getByText('Appointment: 2024-03-01', { exact: true }).isVisible());
      await strip.getByRole('button', { name: 'Next range', exact: true }).click(); await tick();
      assert(await day(grid, 'Monday, March 4, 2024').isVisible());
      assert(await dates.evaluateAll(elements => elements.every((e, i) => e === window.miniCalendarDays[i])));
      await strip.getByRole('button', { name: 'Previous range', exact: true }).click(); await tick();
      await day(grid, 'Wednesday, February 28, 2024').focus();
      await page.keyboard.press('ArrowRight'); await tick();
      const leap = day(grid, 'Thursday, February 29, 2024'); assert(await focus(leap));
      await page.keyboard.press('Enter'); await tick();
      assert(await primary.getByText('Appointment: 2024-02-29', { exact: true }).isVisible());
      await page.keyboard.press('End'); await tick(); assert(await focus(day(grid, 'Sunday, March 3, 2024')));
      await page.keyboard.press('ArrowRight'); await tick(); assert(await focus(day(grid, 'Monday, March 4, 2024')));

      const controlled = detail('Controlled dates, range and deselection');
      const cgrid = controlled.getByRole('grid', { name: 'Controlled date strip', exact: true });
      const cstrip = controlled.locator('.sapid-mini-calendar');
      await controlled.getByRole('checkbox', { name: 'Accept date and range requests', exact: true }).uncheck();
      await cstrip.getByRole('button', { name: 'Next range', exact: true }).click(); await tick();
      assert(await day(cgrid, 'Tuesday, February 27, 2024').isVisible());
      await day(cgrid, 'Friday, March 1, 2024').click(); await tick();
      assert.equal(await cgrid.locator('[aria-selected=true]').count(), 1);
      assert(await controlled.getByText('Range starts 2024-02-27; selected 2024-02-27', { exact: true }).isVisible());
      await controlled.getByRole('checkbox', { name: 'Accept date and range requests', exact: true }).check();
      await cstrip.getByRole('button', { name: 'Next range', exact: true }).click(); await tick();
      assert(await day(cgrid, 'Tuesday, March 5, 2024').isVisible());
      await controlled.getByRole('button', { name: 'Reset date range', exact: true }).click(); await tick();

      const localized = detail('Bounds, localization and day styles');
      const lgrid = localized.getByRole('grid', { name: 'Localized bounded dates', exact: true });
      assert(await day(lgrid, 'Saturday, April 13, 2024').isDisabled());
      assert(await day(lgrid, 'Tuesday, April 16, 2024').isDisabled());
      const sunday = day(lgrid, 'Sunday, April 14, 2024'); await sunday.evaluate(e => { window.miniLocalizedDay = e; });
      assert.equal(await day(lgrid, 'Monday, April 15, 2024').getAttribute('aria-current'), 'date');
      await localized.getByRole('checkbox', { name: 'Chinese date labels', exact: true }).check(); await tick();
      assert(await day(lgrid, '2024年4月14日 星期日').evaluate(e => e === window.miniLocalizedDay));
      const states = detail('Keyboard, RTL and unavailable states');
      const rtl = states.getByRole('grid', { name: 'RTL date strip', exact: true });
      assert.equal(await rtl.getAttribute('aria-readonly'), 'true');
      const [a, b] = await Promise.all([day(rtl, 'Tuesday, February 27, 2024').boundingBox(), day(rtl, 'Wednesday, February 28, 2024').boundingBox()]);
      assert(a.x > b.x);
      await states.getByRole('checkbox', { name: 'Enable date strip', exact: true }).uncheck(); await tick();
      assert(await rtl.getByRole('button').evaluateAll(elements => elements.every(e => e.disabled)));
      assert.equal(await states.getByRole('grid', { name: 'Final supported date strip', exact: true }).getByRole('gridcell').count(), 3);
      await primary.screenshot({ path: outputPath(`mini-calendar-remediation/${key}.png`) });
      report.pages.push({ key, passed: true }); console.log(`PASS ${key}`);
    }
    assert.deepEqual(report.errors, []);
  } catch (error) { report.failure = error.stack; await page.screenshot({ path: outputPath('mini-calendar-remediation/failure.png'), fullPage: true }); throw error; }
  finally { writeReport('mini-calendar-remediation.json', report); await browser.close(); }
})().catch(error => { console.error(error); process.exitCode = 1; });
