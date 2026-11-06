const assert = require('node:assert/strict');
const fs = require('node:fs');
const {matrix, chromium, base, outputPath, writeReport } = require('./showcase-browser-env.cjs');
(async () => {
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
  const report = { browser: browser.version(), pages: [], errors: [] };
  page.on('pageerror', e => report.errors.push(e.message));
  page.on('console', m => { if (m.type() === 'error') report.errors.push(m.text()); });
  const tick = () => page.evaluate(() => new Promise(resolve => { let n = 0; const frame = () => ++n === 5 ? resolve() : requestAnimationFrame(frame); requestAnimationFrame(frame); }));
  const detail = title => page.locator('.site-component-detail').filter({ has: page.getByRole('heading', { name: title, exact: true }) });
  const cell = (root, prefix) => root.getByRole('button', { name: new RegExp(`^${prefix.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')}`) });
  const focused = locator => locator.evaluate(e => e === document.activeElement);
  fs.mkdirSync(outputPath('heatmap-remediation'), { recursive: true });
  try {
    for (const theme of matrix.themes) for (const scheme of matrix.schemes) for (const width of [1280, 390]) {
      const key = `${theme}-${scheme}-${width}`;
      await page.setViewportSize({ width, height: 1000 });
      await page.goto(`${base}/showcase.html?widget=heatmap&theme=${theme}&scheme=${scheme}`);
      await page.waitForFunction(() => window.ready === true && document.documentElement.getAttribute('data-sapid-widget') === 'heatmap'); await tick();
      const primary = page.locator('.site-preview .site-demo').first();
      const grid = primary.getByRole('grid', { name: 'Calendar contributions', exact: true });
      assert.equal(await grid.getByRole('row').count(), 7); assert.equal(await grid.getByRole('gridcell').count(), 91);
      assert.equal(await page.locator('.site-component-detail').count(), 4);
      assert(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth + 1));
      assert(await primary.locator('.sapid-heatmap-scroll').evaluate(e => e.scrollWidth <= e.clientWidth + 1));
      assert(await primary.locator('.sapid-heatmap-cell').evaluateAll(cells => cells.every(cell => {
        const button = cell.querySelector('button');
        return !button || Math.abs(button.getBoundingClientRect().top - cell.getBoundingClientRect().top) < 1;
      })), 'date buttons align with their grid cells instead of inline text baselines');
      const leap = cell(grid, 'Thursday, February 29, 2024:');
      await leap.hover(); await tick(); assert((await page.getByRole('tooltip').innerText()).includes('4 contributions'));
      await leap.click(); await tick(); assert(await primary.getByText('2024-02-29: 4 contributions', { exact: true }).isVisible());
      await page.keyboard.press('ArrowRight'); await tick(); assert(await focused(cell(grid, 'Thursday, March 7, 2024:')));
      assert.equal(await page.getByRole('tooltip').count(), 1, 'pointer and keyboard share one current tooltip');
      await page.keyboard.press('ArrowDown'); await page.keyboard.press('Enter'); await tick();
      assert(await primary.getByText('2024-03-08: 1 contribution', { exact: true }).isVisible());
      assert.equal(await page.getByRole('tooltip').count(), 1);
      await primary.screenshot({ path: outputPath(`heatmap-remediation/${key}-tooltip.png`) });
      await page.mouse.move(width - 1, 1); await page.keyboard.press('Tab'); await tick();
      assert.equal(await page.getByRole('tooltip').count(), 0);
      await primary.screenshot({ path: outputPath(`heatmap-remediation/${key}.png`) });

      const sparse = detail('Dated data, missing records and zero');
      const sgrid = sparse.getByRole('grid'); const missing = cell(sgrid, 'Thursday, February 29, 2024:');
      await missing.evaluate(e => { window.heatmapLeap = e; });
      const emptyColor = await missing.evaluate(e => getComputedStyle(e).backgroundColor);
      assert((await missing.getAttribute('aria-label')).endsWith('No data'));
      await sparse.getByRole('button', { name: 'Record zero', exact: true }).click(); await tick();
      assert((await missing.getAttribute('aria-label')).endsWith('0 contributions'));
      assert.notEqual(await missing.evaluate(e => getComputedStyle(e).backgroundColor), emptyColor);
      await sparse.getByRole('button', { name: 'Record eight', exact: true }).click(); await tick();
      assert((await missing.getAttribute('aria-label')).endsWith('8 contributions'));
      assert(await missing.evaluate(e => e === window.heatmapLeap));
      await sparse.getByRole('button', { name: 'Remove the record', exact: true }).click(); await tick();
      assert((await missing.getAttribute('aria-label')).endsWith('No data')); assert(await missing.evaluate(e => e === window.heatmapLeap));

      const domain = detail('Automatic and controlled value domains');
      const four = domain.getByRole('button', { name: '2024-02-28 — 4 tasks', exact: true });
      const before = await four.evaluate(e => getComputedStyle(e).backgroundColor);
      await domain.getByRole('checkbox', { name: 'Use a fixed zero to ten domain', exact: true }).check(); await tick();
      assert.notEqual(await four.evaluate(e => getComputedStyle(e).backgroundColor), before);
      assert((await domain.locator('.sapid-heatmap-legend').innerText()).includes('10'));

      const localized = detail('Colors, geometry and localized labels');
      const localizedDay = cell(localized.getByRole('grid'), '2024年2月29日 星期四:');
      assert(await localizedDay.isVisible()); const box = await localizedDay.boundingBox(); assert(Math.abs(box.width - 20) < 1 && Math.abs(box.height - 20) < 1);
      const range = detail('Date ranges, keyboard and native boundaries'); const rgrid = range.getByRole('grid');
      const first = cell(rgrid, 'Monday, January 1, 2024:'); await first.evaluate(e => { window.heatmapFirst = e; });
      await range.getByRole('button', { name: 'Show full year', exact: true }).click(); await tick();
      assert.equal(await rgrid.getByRole('gridcell').count(), 366); assert(await first.evaluate(e => e === window.heatmapFirst));
      assert(await range.getByText('Showing 366 dates', { exact: true }).isVisible());
      assert(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth + 1));
      if (width === 390) assert(await range.locator('.sapid-heatmap-scroll').evaluate(e => e.scrollWidth > e.clientWidth));
      await first.focus(); await page.keyboard.press('Shift+PageDown'); await tick();
      const last = cell(rgrid, 'Tuesday, December 31, 2024:'); assert(await focused(last));
      const lastBox = await last.boundingBox(); const viewport = await range.locator('.sapid-heatmap-scroll').boundingBox();
      assert(lastBox.x >= viewport.x - 1 && lastBox.x + lastBox.width <= viewport.x + viewport.width + 1);
      await range.getByRole('button', { name: 'Show two weeks', exact: true }).click(); await tick();
      assert.equal(await rgrid.getByRole('gridcell').count(), 14); assert(await first.evaluate(e => e === window.heatmapFirst));
      await range.getByRole('checkbox', { name: 'Enable calendar interaction', exact: true }).uncheck(); await tick();
      assert(await rgrid.getByRole('button').evaluateAll(elements => elements.every(e => e.disabled)));
      report.pages.push({ key, passed: true }); console.log(`PASS ${key}`);
    }
    assert.deepEqual(report.errors, []);
  } catch (error) { report.failure = error.stack; await page.screenshot({ path: outputPath('heatmap-remediation/failure.png'), fullPage: true }); throw error; }
  finally { writeReport('heatmap-remediation.json', report); await browser.close(); }
})().catch(error => { console.error(error); process.exitCode = 1; });
