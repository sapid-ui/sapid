const assert = require('node:assert/strict');
const fs = require('node:fs');
const {matrix, chromium, base, outputPath, writeReport } = require('./showcase-browser-env.cjs');

(async () => {
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
  const report = { pages: [], errors: [] };
  page.on('pageerror', error => report.errors.push(error.message));
  page.on('console', message => { if (message.type() === 'error') report.errors.push(message.text()); });
  const tick = () => page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))));
  const detail = title => page.locator('.site-component-detail').filter({ has: page.getByRole('heading', { name: title, exact: true }) });
  fs.mkdirSync(outputPath('composite-remediation'), { recursive: true });
  try {
    for (const theme of matrix.themes) for (const scheme of matrix.schemes) {
      for (const width of [1280, 390]) for (const slug of ['accordion', 'tabs', 'timeline']) {
        const key = `${slug}-${theme}-${scheme}-${width}`;
        await page.setViewportSize({ width, height: 1000 });
        await page.goto(`${base}/showcase.html?widget=${slug}&theme=${theme}&scheme=${scheme}`);
        await page.waitForFunction(slug => window.ready === true && document.documentElement.getAttribute('data-sapid-widget') === slug, slug);
        await tick();
        const primary = page.locator('.site-preview .site-demo').first();
        const geometry = await primary.evaluate(element => ({ previewOverflow: element.scrollWidth - element.clientWidth, documentOverflow: document.documentElement.scrollWidth - innerWidth }));
        assert(geometry.previewOverflow <= 1 && geometry.documentOverflow <= 1, `${key}: overflow ${JSON.stringify(geometry)}`);
        await primary.screenshot({ path: outputPath(`composite-remediation/${key}.png`), animations: 'disabled' });
        if (slug === 'accordion') {
          const trigger = primary.getByRole('button', { name: 'How long does shipping take?', exact: true });
          const triggerBox = await trigger.boundingBox();
          const rootBox = await primary.locator('.sapid-accordion').boundingBox();
          assert(triggerBox.width >= rootBox.width - 2, `${key}: full-width heading`);
          assert.equal(await primary.locator('.sapid-disclosure-chevron').count(), 3);
          await trigger.click(); await tick(); assert.equal(await trigger.getAttribute('aria-expanded'), 'false');
          await trigger.click(); await tick(); assert.equal(await trigger.getAttribute('aria-expanded'), 'true');
          await trigger.focus(); await page.keyboard.press('End'); await tick();
          assert(await primary.getByRole('button', { name: 'How can I contact support?', exact: true }).evaluate(element => element === document.activeElement));
          const multiple = detail('Multiple sections and controlled state');
          await multiple.getByRole('button', { name: 'Payment methods', exact: true }).click(); await tick();
          for (const name of ['Delivery options', 'Payment methods']) assert.equal(await multiple.getByRole('button', { name, exact: true }).getAttribute('aria-expanded'), 'true');
          assert(await multiple.getByRole('button', { name: 'Archived information', exact: true }).isDisabled());
          const lifetime = detail('Retain or retire panel content');
          const retained = lifetime.getByRole('textbox', { name: 'Retained draft', exact: true });
          await retained.fill('Retained note'); await retained.evaluate(element => { window.retainedAccordion = element; });
          await lifetime.getByRole('button', { name: 'Retained draft', exact: true }).click(); await tick();
          assert.equal(await retained.isVisible(), false);
          await lifetime.getByRole('button', { name: 'Retained draft', exact: true }).click(); await tick();
          assert.equal(await retained.inputValue(), 'Retained note');
          assert(await retained.evaluate(element => element === window.retainedAccordion));
          const unmounted = lifetime.getByRole('textbox', { name: 'Unmounted draft', exact: true });
          await unmounted.fill('Discarded note');
          await lifetime.getByRole('button', { name: 'Unmounted draft', exact: true }).click(); await tick();
          assert.equal(await unmounted.count(), 0);
          await lifetime.getByRole('button', { name: 'Unmounted draft', exact: true }).click(); await tick();
          assert.equal(await unmounted.inputValue(), '');
          const dynamic = detail('Keyed sections');
          const shipping = dynamic.getByRole('button', { name: 'shipping', exact: true });
          await shipping.evaluate(element => { window.keyedAccordion = element; });
          await dynamic.getByRole('button', { name: 'Add account section', exact: true }).click(); await tick();
          assert(await dynamic.getByRole('button', { name: 'account', exact: true }).isVisible());
          assert(await shipping.evaluate(element => element === window.keyedAccordion));
        }
        if (slug === 'tabs') {
          const list = primary.getByRole('tablist', { name: 'Account settings', exact: true });
          const messages = list.getByRole('tab', { name: 'Messages', exact: true });
          await messages.click(); await tick(); assert.equal(await messages.getAttribute('aria-selected'), 'true');
          assert(await messages.evaluate(element => parseFloat(getComputedStyle(element).borderBottomWidth) >= 2));
          assert(await primary.getByRole('tabpanel').filter({ hasText: 'Messages tab content' }).isVisible());
          const vertical = detail('Vertical and RTL navigation');
          const vlist = vertical.getByRole('tablist', { name: 'Vertical settings', exact: true });
          const vroot = vertical.locator('.sapid-tabs').filter({ has: page.getByRole('tablist', { name: 'Vertical settings', exact: true }) });
          const vpanel = vroot.getByRole('tabpanel');
          const listBox = await vlist.boundingBox(), panelBox = await vpanel.boundingBox();
          assert(panelBox.x >= listBox.x + listBox.width - 1, `${key}: vertical panels beside list`);
          await vlist.getByRole('tab', { name: 'Profile', exact: true }).focus();
          await page.keyboard.press('ArrowDown'); await tick();
          assert.equal(await vlist.getByRole('tab', { name: 'Security', exact: true }).getAttribute('aria-selected'), 'true');
          const rtl = vertical.getByRole('tablist', { name: 'RTL settings', exact: true });
          await rtl.getByRole('tab', { name: 'Profile', exact: true }).focus(); await page.keyboard.press('ArrowLeft'); await tick();
          assert.equal(await rtl.getByRole('tab', { name: 'Security', exact: true }).getAttribute('aria-selected'), 'true');
          const manual = detail('Controlled value and manual activation').getByRole('tablist', { name: 'Manual settings', exact: true });
          await manual.getByRole('tab', { name: 'Profile', exact: true }).focus(); await page.keyboard.press('ArrowRight'); await tick();
          assert.equal(await manual.getByRole('tab', { name: 'Profile', exact: true }).getAttribute('aria-selected'), 'true');
          assert(await manual.getByRole('tab', { name: 'Security', exact: true }).evaluate(element => element === document.activeElement));
          await page.keyboard.press('Enter'); await tick();
          assert.equal(await manual.getByRole('tab', { name: 'Security', exact: true }).getAttribute('aria-selected'), 'true');
          assert(await manual.getByRole('tab', { name: 'Billing', exact: true }).isDisabled());
          for (const [name, retains] of [['Retained tab draft', true], ['Unmounted tab draft', false]]) {
            const root = detail('Keep mounted and unmount').locator('.sapid-tabs').filter({ has: page.getByRole('tablist', { name, exact: true }) });
            const input = root.getByRole('textbox', { name, exact: true });
            await input.fill('A draft'); await input.evaluate(element => { window.tabDraft = element; });
            await root.getByRole('tab', { name: 'Preview', exact: true }).click(); await tick();
            assert.equal(retains ? await input.isVisible() : await input.count() > 0, false);
            await root.getByRole('tab', { name: 'Draft', exact: true }).click(); await tick();
            assert.equal(await input.inputValue(), retains ? 'A draft' : '');
            assert.equal(await input.evaluate(element => element === window.tabDraft), retains);
          }
          const keyed = detail('Dynamic keyed tabs');
          const editor = keyed.getByRole('textbox', { name: 'overview draft', exact: true });
          await editor.fill('Kept during reordering'); await editor.evaluate(element => { window.keyedTabEditor = element; });
          await keyed.getByRole('button', { name: 'Reverse tab order', exact: true }).click(); await tick();
          assert.deepEqual(await keyed.getByRole('tab').allTextContents(), ['notes', 'activity', 'overview']);
          assert.equal(await editor.inputValue(), 'Kept during reordering');
          assert(await editor.evaluate(element => element === window.keyedTabEditor));
        }
        if (slug === 'timeline') {
          assert.equal(await primary.locator('.sapid-timeline-marker').count(), 3);
          assert.equal(await primary.locator('.sapid-timeline-line').count(), 2);
          assert.equal(await primary.locator('.sapid-timeline').evaluate(element => element.tagName), 'OL');
          assert(await primary.locator('.sapid-timeline-item').evaluateAll(elements => elements.every(element => getComputedStyle(element).display === 'flex')));
          assert(await primary.locator('.sapid-timeline-line').evaluateAll(elements => elements.every(element => element.getBoundingClientRect().height > 10)));
          const progress = detail('Reactive progress');
          await progress.locator('.sapid-timeline-item').first().evaluate(element => { window.timelineFirst = element; });
          await progress.getByRole('button', { name: 'Complete release', exact: true }).click(); await tick();
          assert.equal(await progress.locator('.sapid-timeline-completed').count(), 3);
          await progress.getByRole('button', { name: 'Restart review', exact: true }).click(); await tick();
          assert.equal(await progress.locator('.sapid-timeline-current').count(), 1);
          assert(await progress.locator('.sapid-timeline-item').first().evaluate(element => element === window.timelineFirst));
        }
        report.pages.push({ slug, theme, scheme, width, status: 'passed', ...geometry });
        writeReport('composite-remediation.json', report); console.log(key);
      }
    }
    assert.deepEqual(report.errors, []);
  } finally { await browser.close(); writeReport('composite-remediation.json', report); }
})().catch(error => { console.error(error); process.exitCode = 1; });
