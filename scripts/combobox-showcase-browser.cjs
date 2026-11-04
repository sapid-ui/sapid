const assert = require('node:assert/strict');
const fs = require('node:fs');
const { chromium, base, outputPath, outputRoot, writeReport } = require('./showcase-browser-env.cjs');
(async () => {
  const browser = await chromium.launch({ headless: true, ...(process.env.GLORY_CHROME ? { executablePath: process.env.GLORY_CHROME } : {}) });
  const report = { checks: [], errors: [] };
  try {
    const page = await browser.newPage(); page.on('pageerror', error => report.errors.push(error.message));
    await page.goto(`${base}/showcase.html?widget=combobox&theme=blue&scheme=light`);
    await page.waitForFunction(() => window.ready && document.documentElement.getAttribute('data-sapid-widget') === 'combobox');
    const limited = page.getByRole('combobox', { name: 'Prefix filter with visible limit', exact: true });
    assert.equal(await limited.inputValue(), 'a');
    assert.equal(await page.locator('.sapid-combobox-loading.is-left').count(), 1);
    const visible = page.getByRole('option').filter({ visible: true });
    assert.equal(await visible.count(), 2);
    assert.deepEqual(await visible.allTextContents(), ['Angular', 'Astro']);
    report.checks.push('Finite prefix filter, visible limit and left loading state are active');
    await limited.press('Escape');
    const grouped = page.getByRole('combobox', { name: 'Grouped options', exact: true });
    await grouped.click();
    assert(await page.getByText('Frontend', { exact: true }).count());
    assert(await page.getByText('Backend', { exact: true }).count());
    assert(await page.getByRole('option', { name: 'Django', exact: true }).isDisabled());
    report.checks.push('Grouped declarations and disabled option semantics use real listbox controls');
    await grouped.press('Escape');
    const controlled = page.getByRole('combobox', { name: 'Your favorite framework', exact: true });
    await controlled.click();
    await controlled.fill('vue');
    await page.getByRole('option', { name: 'Vue', exact: true }).click();
    await page.waitForTimeout(50);
    assert.equal(await controlled.inputValue(), 'Vue');
    assert.equal(await controlled.getAttribute('aria-expanded'), 'false');
    report.checks.push('Controlled search and option selection reconcile to the accepted label');
    assert.deepEqual(report.errors, []);
  } finally { await browser.close(); writeReport('combobox-showcase-browser.json', report); }
  console.log(JSON.stringify(report, null, 2));
})().catch(error => { console.error(error); process.exitCode = 1; });
