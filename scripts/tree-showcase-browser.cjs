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
    const page = await browser.newPage({ viewport: { width: 1440, height: 1100 } });
    page.on('pageerror', error => report.errors.push(error.message));
    await page.goto(`${base}/showcase.html?widget=tree&theme=blue&scheme=light`);
    await page.waitForFunction(() => window.ready === true
      && document.documentElement.getAttribute('data-sapid-widget') === 'tree');

    const section = id => page.locator(`#${id}`).locator('..');
    for (const id of [
      'tree-controlled', 'tree-expansion', 'tree-checked', 'tree-strict',
      'tree-lazy', 'tree-filter', 'tree-drag-drop', 'tree-render-label',
    ]) {
      await section(id).waitFor();
    }
    report.checks.push('Eight component-name Tree detail sections are mounted');

    const expansion = section('tree-expansion');
    assert.equal(await expansion.getByRole('treeitem').count(), 3);
    await expansion.getByRole('button', { name: 'Expand all', exact: true }).click();
    await expansion.getByRole('treeitem').nth(7).waitFor();
    assert.equal(await expansion.getByRole('treeitem').count(), 8);
    await expansion.getByRole('button', { name: 'Collapse all', exact: true }).click();
    await expansion.getByRole('treeitem').nth(3).waitFor({ state: 'detached' });
    assert.equal(await expansion.getByRole('treeitem').count(), 3);
    report.checks.push('Bulk expand and collapse use the current finite branch set');

    const cascading = section('tree-checked');
    const cascadeComponents = cascading.getByRole('treeitem', { name: 'components', exact: true });
    await cascadeComponents.click();
    await page.waitForFunction(() => document.querySelector('#tree-checked')
      ?.parentElement?.querySelector('[role="treeitem"][aria-label="components"]')
      ?.getAttribute('aria-checked') === 'true');
    assert.equal(await cascadeComponents.getAttribute('aria-checked'), 'true');
    assert.equal(await cascading.getByRole('treeitem', { name: 'Button.rs', exact: true }).getAttribute('aria-checked'), 'true');
    assert.equal(await cascading.getByRole('treeitem', { name: 'Input.rs', exact: true }).getAttribute('aria-checked'), 'true');
    assert.equal(await cascading.getByRole('treeitem', { name: 'src', exact: true }).getAttribute('aria-checked'), 'mixed');

    const strict = section('tree-strict');
    const strictComponents = strict.getByRole('treeitem', { name: 'components', exact: true });
    await strictComponents.click();
    await page.waitForFunction(() => document.querySelector('#tree-strict')
      ?.parentElement?.querySelector('[role="treeitem"][aria-label="components"]')
      ?.getAttribute('aria-checked') === 'true');
    assert.equal(await strictComponents.getAttribute('aria-checked'), 'true');
    assert.equal(await strict.getByRole('treeitem', { name: 'Button.rs', exact: true }).getAttribute('aria-checked'), 'false');
    report.checks.push('Cascading and independent checked policies remain distinct');

    const lazy = section('tree-lazy');
    await lazy.getByRole('button', { name: 'Toggle Remote package', exact: true }).click();
    await lazy.getByText('Loaded Remote package', { exact: true }).waitFor();
    await lazy.getByRole('treeitem', { name: 'index.tsx', exact: true }).waitFor();
    report.checks.push('Lazy expansion emits a load request and publishes the child declaration');

    const filter = section('tree-filter');
    const filterInput = filter.getByRole('textbox', { name: 'Filter tree nodes', exact: true });
    await filterInput.fill('Button');
    await page.waitForFunction(() => document.querySelector('#tree-filter')
      ?.parentElement?.querySelectorAll('[role="treeitem"]').length === 5);
    assert.deepEqual(await filter.getByRole('treeitem').allTextContents(), [
      'src', 'components', 'Button.rs', 'tests', 'button_test.rs',
    ]);
    await filterInput.fill('missing');
    await filter.getByText('Nothing found', { exact: true }).waitFor();
    await page.waitForFunction(() => document.querySelector('#tree-filter')
      ?.parentElement?.querySelectorAll('[role="treeitem"]').length === 0);
    assert.equal(await filter.getByRole('treeitem').count(), 0);
    report.checks.push('External filtering retains ancestors and exposes an empty result');

    const reorder = section('tree-drag-drop');
    const source = reorder.getByRole('treeitem', { name: 'main.rs', exact: true });
    const target = reorder.getByRole('treeitem', { name: 'components', exact: true });
    await source.scrollIntoViewIfNeeded();
    await target.scrollIntoViewIfNeeded();
    const sourceBox = await source.boundingBox();
    const targetBox = await target.boundingBox();
    assert(sourceBox && targetBox);
    await page.mouse.move(sourceBox.x + sourceBox.width * 0.7, sourceBox.y + sourceBox.height / 2);
    await page.mouse.down();
    await page.mouse.move(targetBox.x + targetBox.width * 0.7, targetBox.y + targetBox.height / 2, { steps: 8 });
    await page.mouse.up();
    await reorder.getByText('Moved main Inside components', { exact: true }).waitFor();
    assert.equal(await source.getAttribute('aria-level'), '3');
    report.checks.push('Real pointer drag proposes an inside drop and publishes the validated move');

    const labels = section('tree-render-label');
    assert.equal(await labels.locator('.sapid-badge').count(), await labels.getByRole('treeitem').count());
    assert.equal(await labels.locator('.sapid-tree-guide').count() > 0, true);
    report.checks.push('Typed custom labels, file icons and connecting lines render together');

    assert.deepEqual(report.errors, []);
  } finally {
    await browser.close();
    writeReport('tree-showcase-browser.json', report);
  }
  console.log(JSON.stringify(report, null, 2));
})().catch(error => {
  console.error(error);
  process.exitCode = 1;
});
