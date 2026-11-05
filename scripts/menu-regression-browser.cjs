// Run the existing CSR/hydration/stream menu suites against one rebuilt bundle.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { chromium, writeReport, outputRoot } = require('./showcase-browser-env.cjs');
(async () => {
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
  const report = { browser: browser.version(), suites: [] };
  try {
    for (const name of ['menu', 'menubar', 'context-menu']) {
      const source = fs.readFileSync(path.join(__dirname, '..', 'examples', 'web', `${name}-acceptance.js`), 'utf8');
      const acceptance = new Function(`return (${source})`)();
      const result = await acceptance(page, process.env.SAPID_MENU_REGRESSION_BASE || 'http://127.0.0.1:58962', outputRoot);
      assert.deepEqual(result.errors, []);
      report.suites.push({ name, ...result });
      console.log(`PASS ${name}: ${result.reports.length} checks`);
    }
  } catch (error) { report.failure = error.stack; throw error; }
  finally { writeReport('menu-regression.json', report); await browser.close(); }
})().catch(error => { console.error(error); process.exitCode = 1; });
