const assert = require('node:assert/strict');
const fs = require('node:fs');
const { chromium, base, outputPath, writeReport } = require('./showcase-browser-env.cjs');

(async () => {
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
  const report = { browser: browser.version(), pages: [], errors: [] };
  page.on('pageerror', error => report.errors.push(error.message));
  page.on('console', message => { if (message.type() === 'error') report.errors.push(message.text()); });
  const tick = () => page.evaluate(() => new Promise(resolve => { let count = 0; const frame = () => ++count === 6 ? resolve() : requestAnimationFrame(frame); requestAnimationFrame(frame); }));
  const detail = title => page.locator('.site-component-detail').filter({ has: page.getByRole('heading', { name: title, exact: true }) });
  const select = async (editor, anchor, focus) => {
    await editor.focus();
    await editor.evaluate((root, { anchor, focus }) => {
      const point = offset => { const walker = document.createTreeWalker(root, NodeFilter.SHOW_TEXT); while (walker.nextNode()) { const node = walker.currentNode; if (offset <= node.length) return [node, offset]; offset -= node.length; } throw Error('selection outside document'); };
      const [a, ao] = point(anchor), [f, fo] = point(focus); document.getSelection().setBaseAndExtent(a, ao, f, fo);
    }, { anchor, focus });
    await tick();
  };
  fs.mkdirSync(outputPath('geometry-remediation'), { recursive: true });
  try {
    for (const theme of ['blue', 'green']) for (const scheme of ['light', 'dark']) {
      for (const width of [1280, 390]) for (const slug of ['pin-input', 'scroller', 'rich-text-editor', 'affix', 'background-image', 'time-grid']) {
        const key = `${slug}-${theme}-${scheme}-${width}`;
        await page.setViewportSize({ width, height: 1000 });
        await page.goto(`${base}/showcase.html?widget=${slug}&theme=${theme}&scheme=${scheme}`);
        await page.waitForFunction(slug => window.ready === true && document.documentElement.getAttribute('data-sapid-widget') === slug, slug);
        await tick();
        report.asset = await page.locator('link[rel=stylesheet]').evaluateAll(elements => elements.map(element => element.href.split('/').find(part => part.startsWith('assets-'))).find(Boolean));
        const primary = page.locator('.site-preview .site-demo').first();
        const geometry = await primary.evaluate(element => ({ previewOverflow: element.scrollWidth - element.clientWidth, documentOverflow: document.documentElement.scrollWidth - innerWidth }));
        assert(geometry.previewOverflow <= 1 && geometry.documentOverflow <= 1, `${key}: overflow ${JSON.stringify(geometry)}`);
        await primary.screenshot({ path: outputPath(`geometry-remediation/${key}.png`), animations: 'disabled' });
        if (slug === 'pin-input') {
          const editor = primary.getByRole('textbox', { name: 'Verification code', exact: true });
          await editor.evaluate(element => { window.pinIdentity = element; });
          await editor.fill('123456'); await tick();
          assert.equal(await editor.evaluate(element => getComputedStyle(element).outlineStyle), 'none');
          assert(await primary.getByText('Code ready to verify', { exact: true }).isVisible());
          const aligned = await editor.evaluate(element => {
            const cells = [...element.parentElement.querySelectorAll('.sapid-pin-cell')].map(cell => cell.getBoundingClientRect());
            const style = getComputedStyle(element), probe = document.createElement('span');
            probe.style.cssText = `position:fixed;display:block;width:max-content;font:${style.font};letter-spacing:0`;
            probe.textContent = '0'; document.body.append(probe);
            const glyph = probe.getBoundingClientRect().width; probe.remove();
            const pitch = glyph + parseFloat(style.letterSpacing);
            return cells.length === 6 && cells.every((cell, index) => index === 0 || Math.abs(cell.x - cells[0].x - pitch * index) < 0.2) && Math.abs(element.scrollLeft) < 1;
          });
          assert(aligned, `${key}: glyph/cell pitch and final native caret space`);
          await editor.fill('12x'); await tick(); assert.equal(await editor.inputValue(), '123456');
          assert(await editor.evaluate(element => element === window.pinIdentity));
          const masking = detail('Alphanumeric and reactive masking');
          const protectedEditor = masking.locator('.sapid-pin-editor').last();
          await protectedEditor.fill('987654'); await protectedEditor.evaluate(element => { window.protectedPin = element; element.setSelectionRange(1, 4); });
          await masking.getByRole('checkbox', { name: 'Hide code', exact: true }).uncheck(); await tick();
          assert.equal(await protectedEditor.getAttribute('type'), 'text'); assert.equal(await protectedEditor.inputValue(), '987654');
          assert(await protectedEditor.evaluate(element => element === window.protectedPin && element.selectionStart === 1 && element.selectionEnd === 4));
          const states = detail('Fields, errors and unavailable controls');
          assert(await states.getByRole('textbox', { name: 'Read-only code', exact: true }).getAttribute('readonly') !== null);
          assert(await states.getByRole('textbox', { name: 'Unavailable code', exact: true }).isDisabled());
        }
        if (slug === 'scroller') {
          const viewport = primary.locator('.sapid-scroll-area[aria-label="Browse destinations"]');
          const next = primary.getByRole('button', { name: 'Next scroll page', exact: true });
          const previous = primary.getByRole('button', { name: 'Previous scroll page', exact: true });
          assert.equal(await previous.evaluate(element => getComputedStyle(element).opacity), '0');
          const card = primary.getByText('Kyoto', { exact: true });
          const cardBox = await card.boundingBox(); assert(cardBox.width >= 100, `${key}: card content width`);
          await next.click(); await tick(); assert(await viewport.evaluate(element => element.scrollLeft > 100));
          assert.equal(await previous.evaluate(element => getComputedStyle(element).opacity), '1');
          const controls = detail('Scroll distance, control size and icons');
          const fixed = controls.locator('.sapid-scroll-area[aria-label="Fixed distance cards"]');
          await controls.getByRole('button', { name: 'Next scroll page', exact: true }).click(); await tick();
          assert.equal(await fixed.evaluate(element => element.scrollLeft), 180);
          const program = detail('Programmatic scrolling without controls');
          const draft = program.getByRole('textbox', { name: 'Retained draft', exact: true });
          await draft.fill('A retained draft'); await draft.evaluate(element => { window.scrollerDraft = element; });
          await program.getByRole('button', { name: 'End', exact: true }).click(); await tick();
          await program.getByRole('button', { name: 'Start', exact: true }).click(); await tick();
          assert.equal(await draft.inputValue(), 'A retained draft'); assert(await draft.evaluate(element => element === window.scrollerDraft));
          assert.equal(await program.getByRole('button', { name: 'Next scroll page', exact: true }).count(), 0);
        }
        if (slug === 'rich-text-editor') {
          const editor = primary.getByRole('textbox', { name: 'Article editor', exact: true });
          await editor.evaluate(element => { window.articleEditor = element; });
          await select(editor, 0, 5);
          const bold = primary.getByRole('button', { name: 'Bold', exact: true });
          await bold.click(); await tick(); assert.equal(await bold.getAttribute('aria-pressed'), 'true');
          assert.equal(await editor.locator('strong').first().textContent(), 'A pla');
          await primary.getByRole('button', { name: 'Undo', exact: true }).click(); await tick(); assert.equal(await editor.locator('strong').count(), 0);
          await primary.getByRole('button', { name: 'Redo', exact: true }).click(); await tick(); assert(await editor.locator('strong').count() > 0);
          assert(await editor.evaluate(element => element === window.articleEditor));
          const controlled = detail('Controlled documents and bounded history');
          const content = controlled.getByRole('textbox', { name: 'Controlled document', exact: true });
          await content.fill('Accepted note'); await tick();
          await controlled.getByRole('checkbox', { name: 'Accept editor changes', exact: true }).uncheck(); await tick();
          await content.fill('Refused note'); await tick(); assert.equal(await content.textContent(), 'Accepted note');
          const links = detail('Safe links and the editor controller');
          const linked = links.getByRole('textbox', { name: 'Link example', exact: true });
          await select(linked, 0, 6); await links.getByRole('button', { name: 'Link selection', exact: true }).click(); await tick();
          assert.equal(await linked.locator('a').getAttribute('href'), 'https://example.com/');
        }
        if (slug === 'affix') {
          const local = primary.getByRole('button', { name: 'Save draft', exact: true });
          const box = await local.boundingBox(), preview = await primary.boundingBox();
          assert(box.x >= preview.x && box.x + box.width <= preview.x + preview.width + 1);
          const viewport = detail('Viewport positioning and visibility');
          await viewport.getByRole('checkbox', { name: 'Show viewport action', exact: true }).check(); await tick();
          const floating = page.getByRole('button', { name: 'Dismiss floating action', exact: true });
          const before = await floating.boundingBox(); await page.evaluate(() => scrollBy(0, 150)); await tick();
          const after = await floating.boundingBox(); assert(Math.abs(before.y - after.y) < 1);
          await floating.click(); await tick(); assert(!await floating.isVisible());
          const offsets = detail('Reactive offsets within a container');
          const positioned = offsets.getByRole('button', { name: 'Positioned action', exact: true });
          const right = await positioned.boundingBox(); await offsets.getByRole('checkbox', { name: 'Use left corner', exact: true }).check(); await tick();
          assert((await positioned.boundingBox()).x < right.x);
        }
        if (slug === 'background-image') {
          const image = primary.locator('img'); await image.evaluate(element => element.decode());
          assert.equal(await image.evaluate(element => getComputedStyle(element).objectFit), 'cover');
          assert.equal(await primary.getByText('Make room for a new view', { exact: true }).evaluate(element => getComputedStyle(element).color), 'rgb(255, 255, 255)');
          const fallback = detail('Reactive sources and fallback'); const media = fallback.locator('img');
          await media.evaluate(element => { window.backgroundImage = element; });
          await fallback.getByRole('checkbox', { name: 'Use an invalid primary image', exact: true }).check(); await tick();
          await page.waitForFunction(() => { const element = window.backgroundImage; return element.complete && element.naturalWidth > 0 && element.currentSrc.endsWith('/mountain-evening.svg'); });
          assert(await media.evaluate(element => element === window.backgroundImage));
          assert(await fallback.getByRole('button', { name: 'Foreground action', exact: true }).isVisible());
        }
        if (slug === 'time-grid') {
          const grid = primary.getByRole('grid', { name: 'Appointment time', exact: true });
          await grid.getByRole('button', { name: '09:30', exact: true }).click(); await tick();
          assert.equal(await grid.locator('[role=gridcell][aria-selected=true]').innerText(), '09:30');
          await page.keyboard.press('ArrowRight'); await page.keyboard.press('Enter'); await tick();
          assert.equal(await grid.locator('[role=gridcell][aria-selected=true]').innerText(), '10:00');
          const dense = detail('Intervals, columns and time format').getByRole('grid', { name: 'Dense column window', exact: true });
          assert(await dense.evaluate(element => element.scrollWidth > element.clientWidth + 100));
          await dense.getByRole('button').first().focus(); await page.keyboard.press('End'); await tick();
          assert(await dense.evaluate(element => {
            const viewport = element.getBoundingClientRect(), target = document.activeElement.getBoundingClientRect();
            return element.contains(document.activeElement) && element.scrollLeft > 0 && target.left >= viewport.left - 1 && target.right <= viewport.right + 1;
          }), `${key}: keyboard navigation reveals the dense column`);
          const bounds = detail('Bounds, disabled slots and controlled selection');
          assert(await bounds.getByRole('button', { name: '10:30', exact: true }).isDisabled());
          await bounds.getByRole('button', { name: '10:00', exact: true }).click(); await tick();
          await bounds.getByRole('checkbox', { name: 'Accept time changes', exact: true }).uncheck(); await tick();
          await bounds.getByRole('button', { name: '11:00', exact: true }).click(); await tick();
          assert.equal(await bounds.locator('[role=gridcell][aria-selected=true]').innerText(), '10:00');
        }
        report.pages.push({ slug, theme, scheme, width, status: 'passed', ...geometry });
        writeReport('geometry-remediation.json', report); console.log(key);
      }
    }
    assert.deepEqual(report.errors, []);
  } finally { await browser.close(); writeReport('geometry-remediation.json', report); }
})().catch(error => { console.error(error); process.exitCode = 1; });
