const assert = require('node:assert/strict');
const fs = require('node:fs');
const { chromium, base, outputPath, writeReport } = require('./showcase-browser-env.cjs');

(async () => {
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
  const report = { browser: browser.version(), pages: [], errors: [] };
  page.on('pageerror', error => report.errors.push(error.message));
  page.on('console', message => { if (message.type() === 'error') report.errors.push(message.text()); });
  const tick = () => page.evaluate(() => new Promise(resolve => { let n = 0; const frame = () => ++n === 5 ? resolve() : requestAnimationFrame(frame); requestAnimationFrame(frame); }));
  const detail = title => page.locator('.site-component-detail').filter({ has: page.getByRole('heading', { name: title, exact: true }) });
  const focused = locator => locator.evaluate(element => document.activeElement === element);
  const sameEditor = locator => locator.evaluate(element => element === window.menuEditor);
  fs.mkdirSync(outputPath('menu-remediation'), { recursive: true });
  try {
    for (const theme of ['blue', 'green']) for (const scheme of ['light', 'dark']) for (const width of [1280, 390]) {
      for (const slug of ['menu', 'menubar']) {
        const key = `${slug}-${theme}-${scheme}-${width}`;
        await page.setViewportSize({ width, height: 1000 });
        await page.goto(`${base}/showcase.html?widget=${slug}&theme=${theme}&scheme=${scheme}`);
        await page.waitForFunction(slug => window.ready === true && document.documentElement.getAttribute('data-sapid-widget') === slug, slug);
        await tick();
        report.asset = await page.locator('link[rel=stylesheet]').evaluateAll(elements => elements.map(element => element.href.split('/').find(part => part.startsWith('assets-'))).find(Boolean));
        const primary = page.locator('.site-preview .site-demo').first();
        assert.equal(await page.locator('.site-component-detail').count(), slug === 'menu' ? 6 : 9);
        assert(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth + 1), `${key}: page overflow`);
        if (slug === 'menu') {
          const trigger = primary.getByRole('button', { name: 'Toggle menu', exact: true });
          await trigger.click(); await tick();
          const menu = page.getByRole('menu', { name: 'Workspace actions', exact: true });
          const box = await menu.boundingBox(); assert(box.x >= 0 && box.x + box.width <= width + 1);
          assert(await menu.evaluate(element => parseFloat(getComputedStyle(element).borderTopWidth) >= 1));
          assert(await menu.getByText('Application', { exact: true }).isVisible());
          const exportTrigger = menu.getByRole('menuitem', { name: 'Export notes', exact: true });
          assert.equal(await exportTrigger.locator('.sapid-menu-chevron').count(), 1);
          assert.equal(await exportTrigger.evaluate(element => getComputedStyle(element, '::after').content), 'none');
          const calendar = menu.getByRole('menuitem', { name: 'Calendar', exact: true });
          assert.equal(await calendar.locator('svg').evaluate(element => getComputedStyle(element).pointerEvents), 'none');
          const text = await calendar.locator('.sapid-menu-item-label').boundingBox();
          const shortcut = await calendar.getByText('Ctrl+K', { exact: true }).boundingBox();
          assert(text.x + text.width <= shortcut.x + 1, 'shortcut does not overlap label');
          await menu.screenshot({ path: outputPath(`menu-remediation/${key}-popup.png`) });
          await calendar.click(); await tick(); assert(await primary.getByText('Calendar selected', { exact: true }).isVisible());
          assert(await focused(trigger));

          const controls = detail('Controlled opening, refusal and selection policy');
          await controls.getByRole('checkbox', { name: 'Accept menu open and close requests', exact: true }).uncheck();
          await controls.getByRole('button', { name: 'Open controlled menu', exact: true }).click(); await tick();
          const refused = page.getByRole('menu', { name: 'Controlled actions', exact: true });
          await refused.getByRole('menuitem', { name: 'Preview without closing', exact: true }).click();
          await page.keyboard.press('Escape'); await tick(); assert(await refused.isVisible());
          await page.keyboard.press('Tab'); await tick();
          assert(await page.evaluate(() => !document.activeElement.closest('[role=menu]')), 'Tab leaves even a refused close');
          await controls.getByRole('button', { name: 'Close controlled menu', exact: true }).click(); await tick();

          const search = detail('Search, external filtering and empty results');
          const open = search.getByRole('button', { name: 'Find a destination', exact: true });
          await open.click(); await tick();
          const editor = page.getByRole('searchbox', { name: 'Search destinations', exact: true });
          assert(await focused(editor)); await editor.evaluate(element => { window.menuEditor = element; });
          await editor.fill('reports'); await page.keyboard.press('ArrowDown'); await tick();
          const reports = page.getByRole('menuitem', { name: 'Reports', exact: true });
          assert.equal(await editor.getAttribute('aria-activedescendant'), await reports.getAttribute('id'));
          assert(await focused(editor));
          await page.keyboard.press('Enter'); await tick();
          assert(await search.getByText('Reports selected', { exact: true }).isVisible());
          await open.click(); await tick(); assert.equal(await editor.inputValue(), ''); assert(await sameEditor(editor));
          await editor.fill('no matching destination'); await tick();
          assert(await page.getByRole('menu', { name: 'Find a destination', exact: true }).getByText('Nothing found', { exact: true }).isVisible());
          await page.keyboard.press('ArrowDown'); await page.keyboard.press('Enter'); await tick();
          assert.equal(await editor.getAttribute('aria-activedescendant'), null);
          await editor.fill('billing'); await page.keyboard.press('ArrowDown'); await page.keyboard.press('Enter'); await tick();
          const billing = page.getByRole('menu', { name: 'Settings', exact: true }).getByRole('menuitem', { name: 'Billing', exact: true });
          assert(await focused(billing)); await page.keyboard.press('Enter'); await tick();
          assert(await search.getByText('Billing selected', { exact: true }).isVisible());

          const persistentTrigger = search.getByRole('button', { name: 'Preserve a search query', exact: true });
          await persistentTrigger.click(); await tick();
          const persistent = page.getByRole('searchbox', { name: 'Persistent menu search', exact: true });
          await persistent.fill('team'); await page.keyboard.press('Escape'); await tick();
          await persistentTrigger.click(); await tick(); assert.equal(await persistent.inputValue(), 'team');
          await page.keyboard.press('Escape'); await tick();

          await search.getByRole('button', { name: 'Find a display option', exact: true }).click(); await tick();
          const options = page.getByRole('searchbox', { name: 'Search display options', exact: true });
          const ruler = page.getByRole('menuitemcheckbox', { name: 'Show ruler', exact: true });
          await ruler.locator('..').click(); await tick(); assert(await ruler.isChecked()); assert(await focused(options));
          await options.fill('compact'); await page.keyboard.press('ArrowDown'); await page.keyboard.press('Enter'); await tick();
          const compact = page.getByRole('menuitemradio', { name: 'Compact', exact: true });
          assert(await compact.isChecked()); assert(await focused(options));
          await compact.locator('..').click(); await tick(); assert(await compact.isChecked()); assert(await focused(options));
          await page.keyboard.press('Escape'); await tick();
          const keyboard = detail('Keyboard, RTL, disabled and unstyled states');
          assert(await keyboard.getByRole('button', { name: 'Disabled menu', exact: true }).isDisabled());
          const rtl = keyboard.getByRole('button', { name: 'RTL actions', exact: true });
          await rtl.focus(); await page.keyboard.press('ArrowUp'); await tick();
          const submenu = page.getByRole('menu', { name: 'RTL actions', exact: true }).getByRole('menuitem', { name: 'Export', exact: true });
          assert(await focused(submenu)); await page.keyboard.press('ArrowLeft'); await tick();
          assert(await page.getByRole('menuitem', { name: 'PDF', exact: true }).isVisible());
          await page.keyboard.press('Escape'); await tick(); await page.keyboard.press('Escape'); await tick();
        } else {
          const bar = primary.getByRole('menubar', { name: 'Document menu', exact: true });
          const file = bar.getByRole('menuitem', { name: 'File', exact: true });
          await file.click(); await tick();
          const menu = page.getByRole('menu', { name: 'File', exact: true });
          assert(await menu.getByText('Ctrl+S', { exact: true }).isVisible());
          await menu.screenshot({ path: outputPath(`menu-remediation/${key}-popup.png`) });
          await menu.getByRole('menuitem', { name: 'Save document', exact: true }).click(); await tick();
          assert(await primary.getByText('Document saved', { exact: true }).isVisible()); assert(await focused(file));
          await file.click(); await bar.getByRole('menuitem', { name: 'View', exact: true }).hover(); await tick();
          const ruler = page.getByRole('menuitemcheckbox', { name: 'Show ruler', exact: true });
          assert(await ruler.isChecked()); await ruler.click(); await tick(); assert(!await ruler.isChecked());
          await page.keyboard.press('Escape'); await tick();
          const search = detail('Search inside an application menu');
          const searchBar = search.getByRole('menubar', { name: 'Searchable application menu', exact: true });
          const files = searchBar.getByRole('menuitem', { name: 'Files', exact: true });
          await files.click(); await tick();
          const editor = page.getByRole('searchbox', { name: 'Search files', exact: true });
          assert(await focused(editor)); await editor.evaluate(element => { window.menuEditor = element; });
          await editor.fill('road'); await page.keyboard.press('ArrowDown'); await tick();
          assert(await focused(editor));
          await page.keyboard.press('Enter'); await tick();
          assert(await search.getByText('Roadmap opened', { exact: true }).isVisible());
          await files.click(); await tick(); assert.equal(await editor.inputValue(), ''); assert(await sameEditor(editor));
          await editor.fill('release'); await searchBar.getByRole('menuitem', { name: 'Help', exact: true }).hover(); await tick();
          assert(await page.getByRole('menuitem', { name: 'About the workspace', exact: true }).isVisible());
          await page.keyboard.press('Escape'); await tick(); await files.click(); await tick();
          assert.equal(await editor.inputValue(), ''); assert(await sameEditor(editor));
          const client = await page.context().newCDPSession(page); const ax = await client.send('Accessibility.getFullAXTree'); await client.detach();
          assert(ax.nodes.some(node => node.role?.value === 'searchbox' && node.name?.value === 'Search files'));
          await page.keyboard.press('Escape'); await tick();
        }
        await primary.screenshot({ path: outputPath(`menu-remediation/${key}.png`) });
        report.pages.push({ key, passed: true }); console.log(`PASS ${key}`);
      }
    }
    assert.deepEqual(report.errors, []);
  } catch (error) {
    report.failure = error.stack;
    await page.screenshot({ path: outputPath('menu-remediation/failure.png'), fullPage: true }); throw error;
  } finally { writeReport('menu-remediation.json', report); await browser.close(); }
})().catch(error => { console.error(error); process.exitCode = 1; });
