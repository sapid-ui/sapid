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
    await page.goto(`${base}/showcase.html?widget=rich-text-editor&theme=blue&scheme=light`);
    await page.waitForFunction(() => window.ready === true
      && document.documentElement.getAttribute('data-sapid-widget') === 'rich-text-editor');

    for (const id of [
      'rich-editor-usage', 'rich-editor-subtle', 'rich-editor-controlled',
      'rich-editor-placeholder', 'rich-editor-content', 'rich-editor-states',
    ]) await page.locator(`#${id}`).waitFor();
    report.checks.push('Six component-name RichTextEditor documentation sections are mounted');

    const usage = page.getByRole('textbox', { name: 'Article editor', exact: true });
    const usageShell = usage.locator('xpath=../..');
    const toolbar = usageShell.getByRole('toolbar', { name: 'Rich text formatting', exact: true });
    await toolbar.waitFor();
    for (const name of ['Bold', 'Italic', 'Code', 'Undo', 'Redo', 'Paragraph', 'Heading', 'Quote']) {
      assert.equal(await toolbar.getByRole('button', { name, exact: true }).count(), 1);
    }
    report.checks.push('Usage exposes the finite real formatting and history controls');

    const subtle = page.getByRole('textbox', { name: 'Subtle editor', exact: true });
    assert((await subtle.getAttribute('class')).includes('sapid-rich-text-editor-subtle'));
    assert.equal(await subtle.locator('xpath=../..').locator('.sapid-button-subtle').count(), 8);
    report.checks.push('Subtle variant reaches both editor and toolbar controls');

    const controlled = page.getByRole('textbox', { name: 'Controlled editor', exact: true });
    await controlled.fill('Application accepted value');
    await page.waitForFunction(() => document.querySelector('.site-rich-editor-value')?.textContent
      === 'Application accepted value');
    assert.equal(await page.locator('.site-rich-editor-value').textContent(), 'Application accepted value');
    report.checks.push('Controlled document accepts actual contenteditable proposals');

    const placeholderEditor = page.getByRole('textbox', { name: 'Editor with placeholder', exact: true });
    await page.getByText('Start writing your note...', { exact: true }).waitFor();
    await placeholderEditor.fill('A note');
    await page.getByText('Start writing your note...', { exact: true }).waitFor({ state: 'detached' });
    report.checks.push('Bounded placeholder is passive and retires after editing');

    const noToolbar = page.getByRole('textbox', { name: 'Editor without toolbar', exact: true });
    assert.equal(await noToolbar.locator('xpath=../..').getByRole('toolbar').count(), 0);
    const readonly = page.getByRole('textbox', { name: 'Read only rich document', exact: true });
    const disabled = page.getByRole('textbox', { name: 'Disabled rich document', exact: true });
    assert.equal(await readonly.getAttribute('aria-readonly'), 'true');
    assert.equal(await disabled.getAttribute('contenteditable'), 'false');
    assert.deepEqual(report.errors, []);
    report.checks.push('Toolbar-free, read-only and disabled states preserve real editor metadata');
  } finally {
    await browser.close();
    writeReport('rich-editor-showcase-browser.json', report);
  }
  console.log(JSON.stringify(report, null, 2));
})().catch(error => {
  console.error(error);
  process.exitCode = 1;
});
