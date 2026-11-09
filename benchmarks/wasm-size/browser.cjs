'use strict';
const fs = require('node:fs');
const path = require('node:path');
const http = require('node:http');
const { createHash } = require('node:crypto');
const root = fs.realpathSync(path.resolve(process.argv[2]));
const hash = file => createHash('sha256').update(fs.readFileSync(file)).digest('hex');
const server = http.createServer((request, response) => {
  try {
    const file = fs.realpathSync(path.resolve(root, `.${decodeURIComponent(new URL(request.url, 'http://localhost').pathname)}`));
    const relative = path.relative(root, file);
    if (relative.startsWith('..') || path.isAbsolute(relative) || !fs.statSync(file).isFile()) throw Error('Not found');
    const mime = { '.html': 'text/html', '.js': 'application/javascript', '.wasm': 'application/wasm', '.css': 'text/css' };
    response.writeHead(200, { 'Content-Type': mime[path.extname(file)] || 'application/octet-stream' });
    fs.createReadStream(file).pipe(response);
  } catch { response.writeHead(404).end(); }
});
(async () => {
  let browser;
  const evidence = { complete: false, cases: [] };
  fs.writeFileSync(path.join(root, 'browser.json'), JSON.stringify(evidence, null, 2));
  try {
    const report = JSON.parse(fs.readFileSync(path.join(root, 'report.json')));
    if (!report.complete || !report.cases.length) throw Error('A complete size run is required');
    if (hash(path.join(root, 'sapid.css')) !== report.stylesheetSha256) throw Error('CSS changed after measurement');
    for (const item of report.cases) {
      if (hash(path.join(root, item.name, 'index.html')) !== item.htmlSha256) throw Error('HTML loader changed after measurement');
      if (hash(path.join(root, item.name, 'sapid_wasm_size_bg.wasm')) !== item.wasm.sha256) throw Error('WASM changed after measurement');
      for (const file of item.javascriptFiles) {
        if (hash(path.join(root, item.name, file.path)) !== file.sha256) throw Error('JavaScript changed after measurement');
      }
    }
    const { chromium } = require(process.env.SAPID_PLAYWRIGHT || 'playwright');
    await new Promise((resolve, reject) => { server.once('error', reject); server.listen(0, '127.0.0.1', resolve); });
    browser = await chromium.launch({ headless: true, ...(process.env.BROWSER_EXECUTABLE ? { executablePath: process.env.BROWSER_EXECUTABLE } : {}) });
    evidence.browser = browser.version();
    for (const item of report.cases) {
      const page = await browser.newPage();
      const errors = [];
      page.on('pageerror', error => errors.push(error.message));
      await page.goto(`http://127.0.0.1:${server.address().port}/${item.name}/index.html`);
      await page.waitForFunction(() => document.body.dataset.ready || document.body.dataset.error);
      const error = await page.locator('body').getAttribute('data-error');
      if (error) throw Error(`${item.name}: ${error}`);
      if (item.name === 'sapid-button') {
        await page.getByRole('button', { name: 'Count 0', exact: true }).click();
        await page.getByRole('button', { name: 'Count 1', exact: true }).waitFor();
      } else if (item.name === 'editor-used') {
        const editor = page.locator('[contenteditable="true"]');
        await editor.fill('Probe edit');
        await editor.getByText('Probe edit', { exact: true }).waitFor();
      } else if (item.name === 'json-used') {
        await page.getByRole('textbox', { name: 'Probe JSON', exact: true }).fill('{"ready":false}');
      } else if (item.name === 'files-used') {
        await page.getByRole('button', { name: 'Choose probe file', exact: true }).waitFor();
      } else if (item.name === 'dates-used') {
        const calendar = page.getByRole('grid', { name: 'Probe date', exact: true });
        await calendar.waitFor();
        await calendar.locator('button.sapid-calendar-day:enabled').first().click();
      } else if (item.name === 'charts-used') {
        if (!(await page.locator('#app svg').count())) throw Error('Chart geometry missing');
      } else if (item.name === 'code-used') {
        await page.getByText('fn main() { println!("hello"); }', { exact: true }).waitFor();
      } else if (item.name !== 'empty' && await page.locator('#app').textContent() !== 'hello') {
        throw Error('Text probe not mounted');
      }
      await page.evaluate(() => window.probe.unmount());
      if (await page.locator('#app').evaluate(node => node.childNodes.length) !== 0 || errors.length) throw Error(`${item.name}: cleanup/errors ${errors}`);
      evidence.cases.push({ name: item.name, sha256: item.wasm.sha256, passed: true });
      await page.close();
      console.log(`PASS ${item.name}`);
    }
    evidence.complete = true;
  } catch (error) { evidence.error = error.message; throw error; }
  finally {
    fs.writeFileSync(path.join(root, 'browser.json'), JSON.stringify(evidence, null, 2));
    if (browser) await browser.close();
    server.closeAllConnections();
    if (server.listening) await new Promise(resolve => server.close(resolve));
  }
})().catch(error => { console.error(error); process.exitCode = 1; });
