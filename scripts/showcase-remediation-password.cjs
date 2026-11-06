const assert = require('node:assert/strict');
const fs = require('node:fs');
const {matrix, chromium, base, outputPath, writeReport } = require('./showcase-browser-env.cjs');
(async () => {
  const browser = await chromium.launch(); const page = await browser.newPage();
  const report = { browser: browser.version(), pages: [], errors: [] };
  page.on('pageerror', e => report.errors.push(e.message)); page.on('console', m => { if (m.type() === 'error') report.errors.push(m.text()); });
  const tick = () => page.evaluate(() => new Promise(resolve => { let n = 0; const frame = () => ++n === 5 ? resolve() : requestAnimationFrame(frame); requestAnimationFrame(frame); }));
  const detail = title => page.locator('.site-component-detail').filter({ has: page.getByRole('heading', { name: title, exact: true }) });
  const field = (root, label) => root.locator('.sapid-field').filter({ has: page.getByLabel(label, { exact: true }) });
  fs.mkdirSync(outputPath('password-remediation'), { recursive: true });
  try {
    for (const theme of matrix.themes) for (const scheme of matrix.schemes) for (const width of [1280, 390]) {
      const key = `${theme}-${scheme}-${width}`; await page.setViewportSize({ width, height: 1000 });
      await page.goto(`${base}/showcase.html?widget=password-input&theme=${theme}&scheme=${scheme}`);
      await page.waitForFunction(() => window.ready === true && document.documentElement.getAttribute('data-sapid-widget') === 'password-input'); await tick();
      report.asset = await page.evaluate(() => document.documentElement.innerHTML.match(/assets-[a-f0-9]{20}/)?.[0]); assert(report.asset);
      assert.equal(await page.locator('.site-component-detail').count(), 5); assert(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth + 1));
      const primary = page.locator('.site-preview .site-demo').first(); const editor = primary.getByLabel('Password', { exact: true });
      let eye = primary.getByRole('button', { name: 'Show password', exact: true });
      assert.equal(await editor.getAttribute('type'), 'password'); assert.equal(await eye.innerText(), ''); assert.equal(await eye.locator('svg').count(), 1);
      const inputBox = await editor.boundingBox(), eyeBox = await eye.boundingBox();
      assert(eyeBox.width < inputBox.height && eyeBox.height < inputBox.height && eyeBox.x + eyeBox.width <= inputBox.x + inputBox.width + 1 && eyeBox.x > inputBox.x + inputBox.width - 50);
      assert.equal(await eye.evaluate(e => getComputedStyle(e).backgroundColor), 'rgba(0, 0, 0, 0)'); assert.equal(await eye.evaluate(e => getComputedStyle(e).borderTopWidth), '0px');
      await editor.fill('Demo 😀 selection'); await editor.evaluate(e => { window.passwordEditor = e; e.setSelectionRange(1, 4); });
      await eye.click(); await tick(); assert.equal(await editor.getAttribute('type'), 'text'); assert.equal(await editor.inputValue(), 'Demo 😀 selection');
      assert(await editor.evaluate(e => e === window.passwordEditor && e.selectionStart === 1 && e.selectionEnd === 4));
      await primary.getByRole('button', { name: 'Hide password', exact: true }).click(); await tick(); assert.equal(await editor.getAttribute('type'), 'password');
      await editor.fill(''); await primary.screenshot({ path: outputPath(`password-remediation/${key}.png`) });

      const controlled = detail('Controlled value and visibility'); const accepted = controlled.getByRole('checkbox', { name: 'Accept visibility proposals', exact: true });
      const ceditor = controlled.getByLabel('Controlled password', { exact: true }); await ceditor.fill('Controlled sample');
      await accepted.uncheck(); await controlled.getByRole('button', { name: 'Show password', exact: true }).click(); await tick();
      assert.equal(await ceditor.getAttribute('type'), 'password'); assert(await controlled.getByText('Requested reveal', { exact: true }).isVisible());
      await controlled.getByRole('checkbox', { name: 'Reveal from an external control', exact: true }).check(); await tick(); assert.equal(await ceditor.getAttribute('type'), 'text');
      assert.equal(await ceditor.inputValue(), 'Controlled sample'); await accepted.check(); await controlled.getByRole('button', { name: 'Hide password', exact: true }).click(); await tick(); assert.equal(await ceditor.getAttribute('type'), 'password');

      const sections = detail('Left and custom right sections'); assert.equal(await sections.locator('.sapid-password-toggle').count(), 1);
      const custom = sections.getByLabel('Password with a custom right section', { exact: true }); await sections.getByRole('button', { name: 'Show', exact: true }).click(); await tick(); assert.equal(await custom.getAttribute('type'), 'text');
      await sections.getByRole('button', { name: 'Hide', exact: true }).click(); await tick(); assert.equal(await custom.getAttribute('type'), 'password');

      const loading = detail('Loading indicators and visibility'); const rightField = field(loading, 'Loading on the right'), leftField = field(loading, 'Loading on the left');
      const right = loading.getByLabel('Loading on the right', { exact: true }); await right.evaluate(e => { window.loadingPassword = e; });
      const rightEye = rightField.locator('.sapid-password-toggle'); await rightEye.evaluate(e => { window.loadingEye = e; });
      await loading.getByRole('checkbox', { name: 'Show loading indicators', exact: true }).check(); await tick(); assert(!(await rightEye.isVisible())); assert(await leftField.getByRole('button', { name: 'Show password', exact: true }).isVisible());
      await leftField.getByRole('button', { name: 'Show password', exact: true }).click(); await tick(); assert.equal(await loading.getByLabel('Loading on the left', { exact: true }).getAttribute('type'), 'text');
      await loading.getByRole('checkbox', { name: 'Show loading indicators', exact: true }).uncheck(); await tick(); assert(await rightEye.isVisible()); assert(await rightEye.evaluate(e => e === window.loadingEye)); assert(await right.evaluate(e => e === window.loadingPassword));

      const states = detail('Field metadata and unavailable states'); const readonly = states.getByLabel('Read-only password', { exact: true });
      assert(!(await readonly.isEditable())); await field(states, 'Read-only password').getByRole('button', { name: 'Show password', exact: true }).click(); await tick(); assert.equal(await readonly.getAttribute('type'), 'text');
      assert(await states.getByLabel('Disabled password', { exact: true }).isDisabled()); assert(await field(states, 'Disabled password').getByRole('button', { name: 'Show password', exact: true }).isDisabled());
      assert(await states.getByLabel('Required password', { exact: true }).evaluate(e => e.required)); const invalid = states.getByLabel('Invalid password', { exact: true }); assert.equal(await invalid.getAttribute('aria-invalid'), 'true'); assert(await invalid.getAttribute('aria-errormessage'));

      const sizes = detail('Sizes and icon geometry'); for (const size of ['xs', 'sm', 'md', 'lg', 'xl']) {
        const row = field(sizes, `Password size ${size}`); const control = await sizes.getByLabel(`Password size ${size}`, { exact: true }).boundingBox(); const action = await row.getByRole('button', { name: 'Show password', exact: true }).boundingBox();
        assert(action.width > 0 && action.width < control.height && action.height < control.height && Math.abs(action.y + action.height / 2 - control.y - control.height / 2) < 1);
      }
      report.pages.push({ key, passed: true }); console.log(`PASS ${key}`);
    }
    assert.deepEqual(report.errors, []);
  } catch (error) { report.failure = error.stack; await page.screenshot({ path: outputPath('password-remediation/failure.png'), fullPage: true }); throw error; }
  finally { writeReport('password-remediation.json', report); await browser.close(); }
})().catch(error => { console.error(error); process.exitCode = 1; });
