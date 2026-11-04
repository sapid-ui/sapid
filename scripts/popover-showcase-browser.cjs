const assert = require('node:assert/strict');
const fs = require('node:fs');
const { chromium, base, outputPath, outputRoot, writeReport } = require('./showcase-browser-env.cjs');

(async () => {
  const browser = await chromium.launch({ headless: true });
  const report = { checks: [], errors: [] };
  try {
    const page = await browser.newPage({ viewport: { width: 1280, height: 1000 } });
    page.on('pageerror', error => report.errors.push(String(error)));
    await page.goto(`${base}/showcase.html?widget=popover&theme=blue&scheme=light`);
    await page.waitForFunction(() => window.ready === true
      && document.documentElement.getAttribute('data-sapid-widget') === 'popover');

    const triggers = page.locator('button.sapid-popover-trigger');
    assert.equal(await triggers.count(), 4);
    for (const heading of ['Basic usage', 'Same width as target', 'Arrow, offset and placement', 'Controlled state']) {
      assert.equal(await page.getByText(heading, { exact: true }).count(), 1);
    }
    report.checks.push('Popover showcase exposes four distinct behavior examples');

    const target = page.getByRole('button', { name: 'Open matching panel', exact: true });
    const targetPanel = page.locator('.showcase-popover-target-width');
    await target.click();
    await page.waitForFunction(element => element.matches(':popover-open'), await targetPanel.elementHandle());
    const targetBox = await target.boundingBox();
    const panelBox = await targetPanel.boundingBox();
    assert.ok(targetBox && panelBox);
    assert.ok(Math.abs(targetBox.width - panelBox.width) < 0.6,
      `target ${targetBox.width} panel ${panelBox.width}`);
    assert.equal(await targetPanel.getAttribute('data-gly-anchor-match-width'), '');
    report.checks.push('Target width is measured by the host and applied to the live panel');

    await target.press('Escape');
    const arrowTrigger = page.getByRole('button', { name: 'Open positioned panel', exact: true });
    await arrowTrigger.scrollIntoViewIfNeeded();
    await arrowTrigger.click();
    const arrowPanel = page.locator('.showcase-popover-arrow');
    await page.waitForFunction(element => element.matches(':popover-open')
      && element.hasAttribute('data-gly-anchor-resolved'), await arrowPanel.elementHandle());
    const arrow = await arrowPanel.evaluate(element => ({
      requested: element.getAttribute('data-gly-anchor-side'),
      resolved: element.getAttribute('data-gly-anchor-resolved'),
      size: getComputedStyle(element, '::before').width,
      transform: getComputedStyle(element, '::before').transform,
      gap: element.getAttribute('data-gly-anchor-gap'),
    }));
    assert.ok(['top', 'bottom'].includes(arrow.resolved));
    assert.equal(arrow.requested, 'top');
    assert.equal(arrow.size, '10px');
    assert.notEqual(arrow.transform, 'none');
    assert.equal(arrow.gap, '12');
    report.checks.push('Arrow uses collision-resolved side with configured size and offset gap');

    await arrowTrigger.press('Escape');
    const controlledTrigger = page.getByRole('button', { name: 'Toggle controlled popover', exact: true });
    const controlledPanel = page.locator('.showcase-popover-controlled');
    await controlledTrigger.click();
    await page.locator('main').click({ position: { x: 8, y: 8 } });
    assert.equal(await controlledPanel.evaluate(element => element.matches(':popover-open')), true);
    await controlledTrigger.press('Escape');
    await page.waitForFunction(element => !element.matches(':popover-open'), await controlledPanel.elementHandle());
    report.checks.push('Controlled example preserves outside-dismiss policy and Escape proposal handling');
    assert.deepEqual(report.errors, []);
  } finally {
    await browser.close();
    writeReport('popover-showcase-browser.json', report);
  }
  console.log(JSON.stringify(report, null, 2));
})().catch(error => { console.error(error); process.exitCode = 1; });
