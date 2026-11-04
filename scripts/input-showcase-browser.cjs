const assert = require('node:assert/strict');
const fs = require('node:fs');
const { chromium, base, outputPath, outputRoot, writeReport } = require('./showcase-browser-env.cjs');

(async () => {
  const browser = await chromium.launch({ headless: true });
  const report = { checks: [], errors: [] };
  try {
    const page = await browser.newPage({ viewport: { width: 1280, height: 1000 } });
    page.on('pageerror', error => report.errors.push(String(error)));
    await page.goto(`${base}/showcase.html?widget=input&theme=blue&scheme=light`);
    await page.waitForFunction(() => window.ready === true
      && document.documentElement.getAttribute('data-sapid-widget') === 'input');

    const email = page.getByRole('textbox', { name: 'Email address', exact: true });
    const workspace = page.getByRole('textbox', { name: 'Workspace URL', exact: true });
    const success = page.getByRole('textbox', { name: 'Workspace name', exact: true });
    assert.equal(await email.inputValue(), 'alex@studio.design');
    assert.equal(await workspace.inputValue(), 'design');
    assert.equal(await success.inputValue(), 'north-star');
    report.checks.push('Input showcase keeps three genuine named editor identities');

    const workspaceField = workspace.locator('xpath=ancestor::*[contains(concat(" ", normalize-space(@class), " "), " sapid-field ")][1]');
    assert.equal(await workspaceField.locator('.sapid-input-left-section').getByText('@', { exact: true }).count(), 1);
    assert.equal(await workspace.evaluate(element => getComputedStyle(element).paddingRight), '44px');
    assert.equal(await workspaceField.locator('.sapid-input-loading:not([hidden])').count(), 1);
    await page.getByRole('button', { name: 'Stop loading', exact: true }).click();
    await page.getByRole('button', { name: 'Start loading', exact: true }).waitFor();
    assert.equal(await workspaceField.locator('.sapid-input-loading:not([hidden])').count(), 0);
    report.checks.push('Reactive loading, left section and finite right-section width affect the real input');

    const successField = success.locator('xpath=ancestor::*[contains(concat(" ", normalize-space(@class), " "), " sapid-field ")][1]');
    const order = await successField.locator(':scope > *').evaluateAll(elements => elements.map(element => element.className));
    assert.match(order[0], /sapid-field-description/);
    assert.match(order[1], /sapid-field-label/);
    const before = await success.evaluate(element => getComputedStyle(element).borderColor);
    await page.getByRole('button', { name: 'Show error', exact: true }).click();
    await page.getByText('That workspace name is already used', { exact: true }).waitFor();
    assert.equal(await success.getAttribute('aria-invalid'), 'true');
    const after = await success.evaluate(element => getComputedStyle(element).borderColor);
    assert.notEqual(after, before);
    report.checks.push('Wrapper order is physical and error styling overrides the success state');
    assert.deepEqual(report.errors, []);
  } finally {
    await browser.close();
    writeReport('input-showcase-browser.json', report);
  }
  console.log(JSON.stringify(report, null, 2));
})().catch(error => { console.error(error); process.exitCode = 1; });
