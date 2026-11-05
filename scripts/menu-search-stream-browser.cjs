const assert = require('node:assert/strict');
const { chromium, writeReport } = require('./showcase-browser-env.cjs');
(async () => {
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
  const report = { browser: browser.version(), errors: [] };
  page.on('pageerror', error => report.errors.push(error.message));
  page.on('console', message => { if (message.type() === 'error') report.errors.push(message.text()); });
  await page.addInitScript(() => {
    window.streamEditorReplaced = false;
    new MutationObserver(() => {
      const editor = document.querySelector('#menu-search-stream-root #search-editor');
      if (editor) {
        if (!window.firstStreamEditor) {
          window.firstStreamEditor = editor;
          window.firstStreamActive = editor.getAttribute('aria-activedescendant');
        } else if (window.firstStreamEditor !== editor) window.streamEditorReplaced = true;
      }
    }).observe(document, { childList: true, subtree: true, attributes: true });
  });
  try {
    await page.goto(process.env.SAPID_MENU_SEARCH_STREAM || 'http://127.0.0.1:8796/menu-search-stream.html');
    await page.waitForFunction(() => document.querySelector('#menu-search-stream-root')?.dataset.glyStreamState === 'ready');
    const editor = page.getByRole('searchbox', { name: 'Find stream action', exact: true });
    const item = page.getByRole('menuitem', { name: 'Save stream document', exact: true });
    assert.equal(await editor.getAttribute('data-gly-menu-search'), 'search-menu');
    assert.equal(await editor.getAttribute('aria-controls'), 'search-menu');
    assert.equal(await editor.getAttribute('aria-activedescendant'), await item.getAttribute('id'));
    assert(await editor.evaluate(element => element === window.firstStreamEditor && !window.streamEditorReplaced));
    assert.equal(await page.evaluate(() => window.firstStreamActive), null);
    assert(await page.evaluate(() => GloryStream.active() === 0 && JSON.parse(document.querySelector('#menu-search-stream-data').textContent).records.some(record => record.name === 'menu-search')));
    assert.deepEqual(report.errors, []);
    report.passed = true;
    report.checks = ['real Rust Shell/Patch/End', 'retained editor', 'active relation set before WASM', 'resource handoff'];
  } catch (error) { report.failure = error.stack; throw error; }
  finally { writeReport('menu-search-stream.json', report); await browser.close(); }
})().catch(error => { console.error(error); process.exitCode = 1; });
