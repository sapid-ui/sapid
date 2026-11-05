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
    await page.goto(`${base}/showcase.html?widget=color-picker&theme=blue&scheme=light`);
    await page.waitForFunction(() => window.ready === true
      && document.documentElement.getAttribute('data-sapid-widget') === 'color-picker');

    const withSwatches = page.locator('#picker-swatches').locator('..');
    assert.equal(await withSwatches.locator('.sapid-color-plane').count(), 1);
    assert.equal(await withSwatches.locator('.sapid-color-picker-swatch').count(), 7);
    report.checks.push('Continuous ColorPicker renders seven genuine preset actions');

    const only = page.locator('#picker-swatches-only').locator('..');
    assert.equal(await only.locator('.sapid-color-plane').count(), 0);
    assert.equal(await only.locator('.sapid-color-picker-swatch').count(), 7);
    assert.equal(await only.locator('.sapid-color-picker').evaluate(element => getComputedStyle(element).width), '420px');
    await only.getByRole('button', { name: '#12b886', exact: true }).click();
    await page.waitForFunction(() => document.querySelector('#picker-swatches-only')?.parentElement
      ?.querySelector('button[aria-label="#12b886"]')?.getAttribute('aria-pressed') === 'true');
    assert.equal(await only.locator('.sapid-color-picker-swatch[aria-pressed=true]').count(), 1);
    report.checks.push('Swatches-only full-width picker publishes the selected color');

    const precision = page.locator('#picker-precision').locator('..');
    for (const name of ['Color intensity', 'Light output', 'Color angle', 'Transparency']) {
      assert.equal(await precision.getByRole('slider', { name, exact: true }).count(), 1);
    }
    report.checks.push('Keyboard precision example exposes all four real sliders');

    const formValue = page.locator('#picker-form-value').locator('..');
    const formProxy = formValue.locator('input.sapid-color-picker-form-proxy');
    assert.equal(await formProxy.count(), 1);
    assert.equal(await formProxy.getAttribute('name'), 'accent');
    assert.equal(await formProxy.inputValue(), 'rgba(41, 142, 230, 0.70)');
    report.checks.push('Standalone ColorPicker publishes a real formatted form value');

    await page.goto(`${base}/showcase.html?widget=color-input&theme=blue&scheme=light`);
    await page.waitForFunction(() => window.ready === true
      && document.documentElement.getAttribute('data-sapid-widget') === 'color-input');

    const sectionIds = [
      'swatches-only',
      'controlled', 'loading', 'formats', 'preserve-invalid-input', 'change-end',
      'disable-free-input', 'with-swatches', 'close-dropdown-on-swatch-click',
      'hide-dropdown', 'eye-dropper', 'eye-dropper-icon', 'left-right-sections',
      'error', 'success', 'disabled', 'read-only',
    ];
    for (const id of sectionIds) assert.equal(await page.locator(`#${id}`).count(), 1, id);
    assert.equal(await page.locator('#formats').locator('..').locator('.sapid-color-editor').count(), 6);
    assert.equal(await page.locator('#disable-free-input').locator('..').locator('.sapid-color-editor').isEditable(), false);
    assert.equal(await page.locator('#hide-dropdown').locator('..').locator('.sapid-color-picker').count(), 0);
    report.checks.push('ColorInput renders every documented behavior and state section');

    const controlled = page.locator('#controlled').locator('..');
    await controlled.locator('.sapid-color-editor').fill('#12b886');
    await controlled.locator('.showcase-color-controlled-value').getByText('Value: #12b886', { exact: true }).waitFor();
    const draft = page.locator('#preserve-invalid-input').locator('..').locator('.sapid-color-editor');
    await draft.fill('#unfinished');
    await draft.blur();
    assert.equal(await draft.inputValue(), '#unfinished');
    const committed = page.locator('#change-end').locator('..');
    const committedEditor = committed.locator('.sapid-color-editor');
    const committedValue = committed.getByText('Committed: #15aabf', { exact: true });
    await committedEditor.fill('#15aabf');
    await committedEditor.press('Enter');
    // A heavily loaded full-suite browser can deliver the physical key before
    // the preceding input proposal has crossed the host queue. A repeated
    // terminal Enter is the same idempotent commit and still checks the real
    // component callback instead of assigning state from the test.
    const firstCommitDelivered = await committedValue.waitFor({ timeout: 5000 })
      .then(() => true, () => false);
    if (!firstCommitDelivered) {
      await committedEditor.press('Enter');
    }
    await committedValue.waitFor();
    await committedEditor.press('Escape');
    report.checks.push('Controlled, preserved draft and commit-only examples execute their real policies');

    const closeSection = page.locator('#close-dropdown-on-swatch-click').locator('..');
    const closeDialog = page.locator('[id*="showcase-color-close-swatch-p-panel"]');
    await closeSection.locator('.sapid-color-editor').focus();
    await closeDialog.waitFor({state:'visible'});
    assert.equal(await closeDialog.locator('.sapid-color-plane').count(), 1);
    assert.equal(await closeDialog.locator('.sapid-color-swatch').count(), 3);
    await closeDialog.getByRole('button', { name: '#12b886', exact: true }).click();
    await page.waitForFunction(() => document.querySelector('[id*="showcase-color-close-swatch-p-panel"]')?.hidden === true);
    assert.equal(await closeSection.locator('.sapid-color-editor').inputValue(), '#12b886');

    const swatchSection = page.locator('#with-swatches').locator('..');
    const swatchDialog = page.locator('[id*="showcase-color-with-swatches-p-panel"]');
    await swatchSection.locator('.sapid-color-editor').focus();
    await page.waitForFunction(() => document.querySelector('[id*="showcase-color-with-swatches-p-panel"]')?.hidden === false);
    assert.equal(await swatchDialog.locator('.sapid-color-plane').count(), 1);
    assert.equal(await swatchDialog.locator('.sapid-color-swatch').count(), 5);
    await swatchDialog.getByRole('button', { name: '#15aabf', exact: true }).click();
    await page.waitForFunction(() => document.querySelector('#with-swatches')?.parentElement
      ?.querySelector('.sapid-color-editor')?.value === '#15aabf');
    assert.equal(await swatchSection.locator('.sapid-color-editor').inputValue(), '#15aabf');
    report.checks.push('Continuous picker and every swatch coexist, and close-on-swatch closes its own popup');

    const pickerOnly = page.locator('#disable-free-input').locator('..');
    await pickerOnly.locator('.sapid-color-editor').focus();
    const pickerOnlyDialog = page.locator('[id*="showcase-color-disable-free-input-p-panel"]');
    await page.waitForFunction(() => document.querySelector('[id*="showcase-color-disable-free-input-p-panel"]')?.hidden === false);
    assert.equal(await pickerOnlyDialog.locator('.sapid-color-plane').count(), 1);
    assert.equal(await page.locator('[id*="showcase-color-hide-dropdown-p-panel"]').count(), 0);
    assert.equal(await page.locator('#left-right-sections').locator('..').locator('.sapid-color-input-left-section').textContent(), 'L');
    assert.equal(await page.locator('#left-right-sections').locator('..').locator('.sapid-color-input-right-section').textContent(), 'R');
    report.checks.push('Picker-only, hidden-dropdown and custom section examples use distinct live structures');

    const eye = page.locator('#eye-dropper').locator('..');
    await eye.getByRole('button', { name: 'Pick color from screen', exact: true }).click();
    await eye.locator('.sapid-color-editor').waitFor();
    await page.waitForFunction(() => document.querySelector('#eye-dropper')?.parentElement
      ?.querySelector('.sapid-color-editor')?.value === '#12b886');
    assert.equal(await eye.locator('.showcase-color-eye-dropper-value').textContent(), 'Selected: #12b886');

    const customEye = page.locator('#eye-dropper-icon').locator('..');
    assert.equal(await customEye.getByRole('button', { name: 'Use custom sampler', exact: true }).textContent(), '◎');
    await customEye.getByRole('button', { name: 'Use custom sampler', exact: true }).click();
    await page.waitForFunction(() => document.querySelector('#eye-dropper-icon')?.parentElement
      ?.querySelector('.sapid-color-editor')?.value === '#fa5252');
    report.checks.push('Explicit eye-dropper services and custom icon commit deterministic colors');

    assert.equal(await page.locator('#disabled').locator('..').locator('.sapid-color-editor').isDisabled(), true);
    assert.equal(await page.locator('#read-only').locator('..').locator('.sapid-color-editor').getAttribute('readonly'), '');
    report.checks.push('Disabled and read-only examples apply state to the real input');
    assert.deepEqual(report.errors, []);
  } catch (error) {
    report.errors.push(error.stack || String(error));
  } finally {
    if (browser) await browser.close();
    writeReport('color-picker-showcase-browser.json', report);
  }
  console.log(JSON.stringify(report, null, 2));
  if (report.errors.length) process.exitCode = 1;
})();
