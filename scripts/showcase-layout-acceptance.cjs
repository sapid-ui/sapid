// Run after check-showcase.ps1 -Serve. Artifacts and browser caches stay on D:.
const assert = require('node:assert/strict');
const { chromium, base, outputPath, writeReport } = require('./showcase-browser-env.cjs');

(async () => {
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
  const errors = [], reports = [];
  page.on('pageerror', error => errors.push(error.message));
  page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
  const tick = () => page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))));
  const goto = async (slug, theme, scheme) => {
    await page.setViewportSize({ width: 1280, height: 900 });
    await page.goto(`${base}/showcase.html?widget=${slug}&theme=${theme}&scheme=${scheme}`);
    await page.waitForFunction(() => window.ready === true);
    await tick();
  };
  try {
    for (const theme of ['blue', 'green']) for (const scheme of ['light', 'dark']) {
      const mode = `${theme}-${scheme}`;
      await goto('password-input', theme, scheme);
      const password = page.locator('.sapid-password-input').first();
      const eye = page.getByRole('button', { name: 'Show password', exact: true }).first();
      await password.fill('secret🦀value');
      await password.evaluate(element => { window.savedPassword = element; element.setSelectionRange(2, 8, 'backward'); });
      const inputBox = await password.boundingBox(), buttonBox = await eye.boundingBox();
      const iconBox = await eye.locator('svg').boundingBox();
      assert(buttonBox.x >= inputBox.x && buttonBox.x + buttonBox.width <= inputBox.x + inputBox.width + 1, `${mode}: eye inside input`);
      assert(buttonBox.y >= inputBox.y && buttonBox.y + buttonBox.height <= inputBox.y + inputBox.height + 1, `${mode}: eye vertically aligned`);
      assert(iconBox.width <= 20 && buttonBox.width <= 36 && await eye.innerText() === '', `${mode}: compact icon without button text`);
      await eye.click(); await tick();
      assert(await password.evaluate(element => element === window.savedPassword && element.type === 'text' && element.value === 'secret🦀value' && element.selectionStart === 2 && element.selectionEnd === 8), `${mode}: visibility preserves editor/value/selection`);
      await page.getByRole('button', { name: 'Hide password', exact: true }).first().press('Space'); await tick();
      assert.equal(await password.getAttribute('type'), 'password');
      await page.locator('.site-preview').first().screenshot({ path: outputPath(`password-eye-${mode}.png`) });
      reports.push({ suite: `password-eye-${mode}`, status: 'passed' });

      await goto('app-shell', theme, scheme);
      const shell = page.locator('.site-shell-example'), header = shell.locator('.sapid-shell-header');
      const sidebar = shell.locator('.sapid-sidebar'), main = shell.locator('.sapid-shell-main');
      const footer = shell.locator('.sapid-shell-footer'), trigger = shell.locator('.sapid-sidebar-trigger');
      const shellBox = await shell.boundingBox(), headerBox = await header.boundingBox();
      const sidebarBox = await sidebar.boundingBox(), mainBox = await main.boundingBox(), footerBox = await footer.boundingBox();
      assert(Math.abs(headerBox.width - shellBox.width) < 2 && Math.abs(footerBox.width - shellBox.width) < 2, `${mode}: header/footer span shell`);
      assert(Math.abs(mainBox.x - sidebarBox.x - sidebarBox.width) < 2 && mainBox.y >= headerBox.y + headerBox.height - 1, `${mode}: body regions do not overlap`);
      assert(await shell.locator('.shell-status,input:not(.sapid-text-field)').count() === 0, `${mode}: no test counters or raw fixture controls`);
      const note = shell.getByRole('textbox', { name: 'Workspace note' });
      await note.fill('Keep this draft'); await note.evaluate(element => { window.workspaceNote = element; });
      await trigger.click(); await tick();
      assert(Math.abs((await sidebar.boundingBox()).width - 72) < 1, `${mode}: compact rail`);
      await trigger.click(); await tick();
      await shell.getByRole('link', { name: 'Projects', exact: true }).click(); await tick();
      assert.equal(await main.getByRole('heading').innerText(), 'Projects');
      await page.locator('.site-preview').first().screenshot({ path: outputPath(`app-shell-${mode}.png`) });
      await page.setViewportSize({ width: 390, height: 844 }); await tick();
      await page.waitForFunction(() => document.querySelector('.site-shell-example .sapid-sidebar-trigger').getAttribute('aria-label') === 'Open navigation');
      await trigger.click(); await tick();
      const drawer = page.getByRole('dialog', { name: 'Workspace navigation', exact: true });
      assert(await drawer.isVisible(), `${mode}: mobile drawer opens`);
      await page.keyboard.press('Escape'); await tick();
      assert(await trigger.evaluate(element => element === document.activeElement), `${mode}: drawer restores focus`);
      assert(await note.evaluate(element => element === window.workspaceNote && element.value === 'Keep this draft'), `${mode}: draft survives layout change`);
      assert(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth + 1), `${mode}: mobile page has no horizontal overflow`);
      reports.push({ suite: `app-shell-${mode}`, status: 'passed' });

      await goto('menubar', theme, scheme);
      assert.equal(await page.locator('.site-component-detail').count(), 7, `${mode}: separate Menubar documentation sections`);
      const bar = page.getByRole('menubar', { name: 'Editor menu' });
      const file = bar.getByRole('menuitem', { name: 'File', exact: true });
      const edit = bar.getByRole('menuitem', { name: 'Edit', exact: true });
      await file.click(); await tick(); await edit.hover(); await tick();
      assert.equal(await file.getAttribute('aria-expanded'), 'false');
      assert.equal(await edit.getAttribute('aria-expanded'), 'true');
      await page.keyboard.press('Escape'); await tick();
      await file.focus(); await file.press('ArrowDown'); await tick();
      assert.equal(await page.evaluate(() => document.activeElement.textContent), 'New file');
      await page.keyboard.press('Escape'); await tick();
      await page.locator('.site-preview').first().screenshot({ path: outputPath(`menubar-${mode}.png`) });

      const hover = page.getByRole('menubar', { name: 'Hover menu' });
      await hover.getByRole('menuitem', { name: 'File', exact: true }).hover(); await tick();
      assert.equal(await hover.getByRole('menuitem', { name: 'File', exact: true }).getAttribute('aria-expanded'), 'true');
      await page.getByRole('menuitem', { name: 'New file', exact: true }).hover(); await tick();
      assert(await page.getByRole('menu', { name: 'File', exact: true }).isVisible(), `${mode}: pointer reaches hover dropdown`);
      await page.mouse.move(5, 5); await tick();
      await hover.getByRole('menuitem', { name: 'File', exact: true }).focus();
      await page.keyboard.press('End'); await tick(); await page.keyboard.press('ArrowRight'); await tick();
      assert(await hover.getByRole('menuitem', { name: 'View', exact: true }).evaluate(element => element === document.activeElement), `${mode}: loop disabled`);

      const choices = page.getByRole('menubar', { name: 'View preferences' });
      await choices.getByRole('menuitem', { name: 'View', exact: true }).click(); await tick();
      await page.getByRole('menuitemcheckbox', { name: 'Show sidebar', exact: true }).click(); await tick();
      assert.equal(await page.getByRole('menuitemcheckbox', { name: 'Show sidebar', exact: true }).getAttribute('aria-checked'), 'false');
      await page.getByRole('menuitemradio', { name: 'Compact', exact: true }).click(); await tick();
      assert.equal(await page.getByRole('menuitemradio', { name: 'Compact', exact: true }).getAttribute('aria-checked'), 'true');
      await page.keyboard.press('Escape'); await tick();
      await page.getByRole('button', { name: 'Open Edit', exact: true }).click(); await tick();
      const controlled = page.getByRole('menubar', { name: 'Controlled menu' });
      assert.equal(await controlled.getByRole('menuitem', { name: 'Edit', exact: true }).getAttribute('aria-expanded'), 'true');
      await page.getByRole('button', { name: 'Close all', exact: true }).click(); await tick();
      assert.equal(await controlled.locator('[aria-expanded=true]').count(), 0);
      const position = page.getByRole('menubar', { name: 'Position example' });
      await position.getByRole('menuitem', { name: 'Top start', exact: true }).click(); await tick();
      assert(Math.abs((await page.getByRole('menu', { name: 'Top start', exact: true }).boundingBox()).width - 180) < 2, `${mode}: per-menu width`);
      await page.keyboard.press('Escape'); await tick();
      reports.push({ suite: `menubar-${mode}`, status: 'passed' });
    }
    assert.deepEqual(errors, [], 'browser errors');
    writeReport('showcase-layout-acceptance.json', { reports, errors });
    console.log(JSON.stringify({ passed: reports.length, errors }));
  } catch (error) {
    await page.screenshot({ path: outputPath('showcase-layout-failure.png'), fullPage: true });
    writeReport('showcase-layout-acceptance.json', { reports, errors, failure: error.stack });
    throw error;
  } finally { await browser.close(); }
})().catch(error => { console.error(error); process.exitCode = 1; });
