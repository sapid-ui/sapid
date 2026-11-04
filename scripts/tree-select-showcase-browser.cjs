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
    await page.goto(`${base}/showcase.html?widget=tree-select&theme=blue&scheme=light`);
    await page.waitForFunction(() => window.ready === true
      && document.documentElement.getAttribute('data-sapid-widget') === 'tree-select');

    for (const id of [
      'tree-select-usage', 'tree-select-modes', 'tree-select-searchable',
      'tree-select-expanded', 'tree-select-limits', 'tree-select-rendering',
      'tree-select-dropdown', 'tree-select-states',
    ]) await page.locator(`#${id}`).waitFor();
    report.checks.push('Eight component-name TreeSelect documentation sections are mounted');
    const controlledTree = async trigger => {
      const id = await trigger.getAttribute('aria-controls');
      assert(id);
      const tree = page.locator(`[id="${id}"]`);
      await tree.waitFor({ state: 'visible' });
      return tree;
    };
    const waitAttribute = async (locator, name, value) => {
      for (let attempt = 0; attempt < 100; attempt += 1) {
        if (await locator.getAttribute(name) === value) return;
        await page.waitForTimeout(10);
      }
      assert.equal(await locator.getAttribute(name), value);
    };

    const cascade = page.getByRole('combobox', { name: 'Include categories', exact: true });
    await cascade.scrollIntoViewIfNeeded();
    await cascade.click();
    let tree = await controlledTree(cascade);
    await tree.getByRole('treeitem', { name: 'Electronics', exact: true }).click();
    await waitAttribute(tree.getByRole('treeitem', { name: 'iPhone', exact: true }), 'aria-checked', 'true');
    assert.equal(await tree.getByRole('treeitem', { name: 'iPhone', exact: true }).getAttribute('aria-checked'), 'true');
    assert.equal(await tree.getByRole('treeitem', { name: 'Headphones', exact: true }).getAttribute('aria-checked'), 'true');
    await page.keyboard.press('Escape');
    await page.waitForFunction(id => document.getElementById(id)?.getAttribute('data-gly-hidden') === '', await cascade.getAttribute('aria-controls'));
    report.checks.push('Checkbox mode cascades a branch proposal to current enabled leaves');

    const search = page.getByRole('combobox', { name: 'Filtered items', exact: true });
    await search.scrollIntoViewIfNeeded();
    await search.focus();
    await search.fill('Pixel');
    tree = await controlledTree(search);
    await tree.getByRole('treeitem', { name: 'Google Pixel', exact: true }).waitFor();
    assert.deepEqual(await tree.getByRole('treeitem').allTextContents(), [
      'Electronics', 'Phones', 'Google Pixel',
    ]);
    await search.fill('missing');
    await tree.getByText('Nothing found...', { exact: true }).waitFor();
    assert.equal(await tree.getByRole('treeitem').count(), 0);
    await page.keyboard.press('Escape');
    report.checks.push('Built-in editable search retains ancestors and renders the empty message');

    const limited = page.getByRole('combobox', { name: 'Pick up to 2 items', exact: true });
    await limited.scrollIntoViewIfNeeded();
    await limited.focus();
    tree = await controlledTree(limited);
    await tree.getByRole('treeitem', { name: 'iPhone', exact: true }).click();
    await tree.getByRole('treeitem', { name: 'iPhone', exact: true }).waitFor({ state: 'detached' });
    await tree.getByRole('treeitem', { name: 'Samsung Galaxy', exact: true }).click();
    await tree.getByRole('treeitem', { name: 'Samsung Galaxy', exact: true }).waitFor({ state: 'detached' });
    const third = tree.getByRole('treeitem', { name: 'Google Pixel', exact: true });
    await third.click();
    assert.equal(await third.getAttribute('aria-checked'), 'false');
    report.checks.push('hide_picked removes accepted values and max_values rejects a third pick');
    await page.keyboard.press('Escape');

    const rendered = page.getByRole('combobox', { name: 'Rendered categories', exact: true });
    await rendered.scrollIntoViewIfNeeded();
    assert.equal(await rendered.getByText('Headphones', { exact: true }).count(), 1);
    await rendered.click();
    tree = await controlledTree(rendered);
    await tree.getByRole('treeitem', { name: 'Headphones', exact: true }).waitFor();
    assert.equal(await tree.locator('.sapid-theme-icon').count() > 0, true);
    assert.equal(await tree.getByRole('treeitem', { name: 'Headphones', exact: true }).count(), 1);
    await page.keyboard.press('Escape');
    report.checks.push('Typed node and selected-value renderers preserve plain-text accessible names');

    const compact = page.getByRole('combobox', { name: 'Compact dropdown without lines', exact: true });
    await compact.scrollIntoViewIfNeeded();
    await compact.click();
    tree = await controlledTree(compact);
    const [triggerBox, treeBox] = await Promise.all([compact.boundingBox(), tree.boundingBox()]);
    assert(triggerBox && treeBox);
    assert(Math.abs(treeBox.width - 220) <= 1, `dropdown width ${treeBox.width}`);
    assert(Math.abs(treeBox.height - 180) <= 1, `dropdown height ${treeBox.height}`);
    assert(treeBox.y + treeBox.height <= triggerBox.y + 1, 'dropdown is placed above the trigger');
    assert.equal(await tree.locator('.sapid-tree-guide').count(), 0);
    report.checks.push('Finite dropdown width, height, top placement and line toggle reach the host');

    assert(await page.getByRole('combobox', { name: 'Disabled tree select', exact: true }).isDisabled());
    assert.equal(
      await page.getByRole('combobox', { name: 'Read only tree select', exact: true }).getAttribute('aria-readonly'),
      'true',
    );
    assert.deepEqual(report.errors, []);
    report.checks.push('Disabled and read-only states retain intrinsic combobox metadata');
  } finally {
    await browser.close();
    writeReport('tree-select-showcase-browser.json', report);
  }
  console.log(JSON.stringify(report, null, 2));
})().catch(error => {
  console.error(error);
  process.exitCode = 1;
});
