const assert = require('node:assert/strict');
const fs = require('node:fs');
const { chromium, base, outputPath, outputRoot, writeReport } = require('./showcase-browser-env.cjs');


(async () => {
  const report = { checks: [], errors: [] };
  let browser;
  try {
    browser = await chromium.launch({ headless: true });
    const page = await browser.newPage({ viewport: { width: 1280, height: 1000 } });
    page.on('pageerror', error => report.errors.push(String(error)));
    await page.goto(`${base}/showcase.html?widget=select&theme=blue&scheme=light`);
    await page.waitForFunction(() => window.ready === true
      && document.documentElement.getAttribute('data-sapid-widget') === 'select');

    const sectionIds = [
      'select-controlled', 'select-loading', 'select-content', 'select-geometry',
      'select-searchable', 'select-states',
    ];
    for (const id of sectionIds) assert.equal(await page.locator(`#${id}`).count(), 1, id);
    report.checks.push('Select documentation exposes every audited behavior section');

    const controlled = page.locator('#select-controlled').locator('..');
    assert.equal(await controlled.locator('.showcase-select-controlled-value').textContent(), 'Selected: rust');
    const controlledTrigger = controlled.getByRole('combobox', { name: 'Controlled selection', exact: true });
    await controlledTrigger.click();
    const controlledListbox = page.getByRole('listbox', { name: 'Controlled selection', exact: true });
    await controlledListbox.getByRole('option', { name: 'Go', exact: true }).click();
    await controlled.getByText('Selected: go', { exact: true }).waitFor();
    await controlled.getByRole('button', { name: 'Clear selection', exact: true }).click();
    await controlled.getByText('Selected: none', { exact: true }).waitFor();
    report.checks.push('Controlled value, clear and real option selection share one accepted path');

    const loading = page.locator('#select-loading').locator('..');
    const loadingTrigger = loading.getByRole('combobox', { name: 'Loading options', exact: true });
    assert.equal(await loadingTrigger.getAttribute('aria-busy'), 'true');
    assert.equal(await loading.locator('.sapid-select-loading').textContent(), '…');
    report.checks.push('Loading state exposes real busy metadata and bounded inline content');

    await page.getByRole('combobox', { name: 'Structured option content', exact: true }).click();
    const contentListbox = page.getByRole('listbox', { name: 'Structured option content', exact: true });
    const paris = contentListbox.getByRole('option', { name: 'Paris', exact: true });
    assert.equal(await paris.locator('.sapid-select-option-leading').textContent(), 'FR');
    assert.equal(await paris.locator('.sapid-select-option-description').textContent(), 'Capital of France');
    assert.equal(await paris.locator('.sapid-select-option-trailing').textContent(), 'UTC+1');
    assert.equal(await paris.locator('.sapid-select-option-check').textContent(), '✓');
    assert.equal(await paris.locator('.sapid-select-option-check').evaluate(element => element === element.parentElement.lastElementChild), true);
    report.checks.push('Grouped options render finite content and the selected check on the configured side');

    const geometry = page.locator('#select-geometry').locator('..');
    const geometryTrigger = geometry.getByRole('combobox', { name: 'Bounded dropdown geometry', exact: true });
    if (await geometryTrigger.getAttribute('aria-expanded') !== 'true') await geometryTrigger.click();
    const geometryListbox = page.getByRole('listbox', { name: 'Bounded dropdown geometry', exact: true });
    assert.equal(await geometryListbox.evaluate(element => getComputedStyle(element).maxHeight), '180px');
    assert.equal(await geometryListbox.getByRole('option').count(), 12);
    report.checks.push('Dropdown height is finite while all declared option identities remain available');

    const searchable = page.getByRole('combobox', { name: 'Searchable composition', exact: true });
    assert.equal(await searchable.inputValue(), 'a');
    assert.equal(await searchable.getAttribute('aria-busy'), 'true');
    await searchable.click();
    const searchableListbox = page.getByRole('listbox', { name: 'Searchable composition', exact: true });
    await searchableListbox.getByRole('option', { name: 'Angular', exact: true }).waitFor();
    assert.deepEqual(await searchableListbox.getByRole('option').allTextContents(), ['Angular', 'Astro']);
    await searchable.fill('zzz');
    await searchableListbox.getByText('Nothing found', { exact: true }).waitFor();
    report.checks.push('Explicit Combobox composition provides search, limits, loading and empty results');

    assert.deepEqual(report.errors, []);
  } catch (error) {
    report.errors.push(error.stack || String(error));
  } finally {
    if (browser) await browser.close();
    writeReport('select-showcase-browser.json', report);
  }
  console.log(JSON.stringify(report, null, 2));
  if (report.errors.length) process.exitCode = 1;
})();
