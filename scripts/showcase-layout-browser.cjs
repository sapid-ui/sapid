const fs = require('node:fs');
const assert = require('node:assert/strict');
const { chromium, base, outputPath, outputRoot, writeReport } = require('./showcase-browser-env.cjs');
const output = outputRoot;

(async () => {
  const report = { base, reports: [], errors: [] };
  const browser = await chromium.launch({ headless: true, ignoreDefaultArgs: ['--hide-scrollbars'] });
  report.chromium = browser.version();
  try {
    const page = await browser.newPage({ viewport: { width: 1440, height: 1100 } });
    page.on('pageerror', error => report.errors.push(error.message));
    page.on('console', event => { if (event.type() === 'error') report.errors.push(event.text()); });
    const tick = () => page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))));
    const button = name => page.getByRole('button', { name, exact: true });
    const select = name => page.getByRole('combobox', { name, exact: true });
    const toggle = name => page.getByRole('switch', { name, exact: true });
    const slider = name => page.getByRole('slider', { name, exact: true });
    const open = async (id, name, theme, scheme, hydrate) => {
      await page.goto(hydrate ? `${base}/ssr-${name}-${theme}-${scheme}.html` : `${base}/showcase.html?widget=${id}&theme=${theme}&scheme=${scheme}`);
      await page.waitForFunction(id => window.ready === true && document.documentElement.getAttribute('data-sapid-widget') === id, id);
      assert.equal(await page.locator('meta[name="sapid-showcase-render"]').getAttribute('content'), hydrate ? 'ssr' : 'csr');
      const preview = await page.locator('.site-layout-playground .site-playground-preview').boundingBox();
      const controls = await page.locator('.site-layout-playground .site-playground-controls').boundingBox();
      assert(controls.x >= preview.x + preview.width && Math.abs(controls.y - preview.y) < 2, 'desktop controls stay to the right');
      for (const range of await page.locator('.site-layout-playground .site-playground-controls .sapid-slider').all()) {
        const bounds = await range.boundingBox();
        assert(bounds.x + bounds.width <= controls.x + controls.width + 1, 'range control overflows the side panel');
        const thumb = await range.locator('.sapid-slider-thumb').boundingBox();
        assert(thumb.x + thumb.width <= controls.x + controls.width + 1, 'slider thumb overflows the side panel');
      }
    };
    const mobile = async name => {
      await page.setViewportSize({ width: 390, height: 844 }); await tick();
      assert(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth + 1), `${name}: mobile horizontal overflow`);
      await page.screenshot({ path: `${output}/layout-${name}-mobile.png`, fullPage: true });
      await page.setViewportSize({ width: 1440, height: 1100 }); await tick();
    };
    for (const theme of ['blue', 'green']) for (const scheme of ['light', 'dark']) for (const hydrate of [false, true]) {
      const mode = `${theme}-${scheme}-${hydrate ? 'hydrate' : 'csr'}`;
      await open('aspect-ratio', 'aspect-ratio', theme, scheme, hydrate);
      const frame = page.locator('.site-aspect-preview');
      await frame.locator('img').waitFor();
      await page.waitForFunction(() => document.querySelector('.site-aspect-preview img')?.naturalWidth > 0);
      await frame.locator('img').evaluate(node => { window.layoutImage = node; });
      for (const [value, ratio] of [['3-2', 1.5], ['1-1', 1], ['4-3', 4 / 3], ['16-9', 16 / 9], ['9-16', 9 / 16]]) {
        await select('Ratio').selectOption(value); await tick();
        const bounds = await frame.boundingBox();
        assert(Math.abs(bounds.width / bounds.height - ratio) < 0.01, `${mode}: actual ratio ${value}`);
        assert(await frame.locator('img').evaluate(node => node === window.layoutImage), 'ratio change replaced media');
        const image = await frame.locator('img').boundingBox();
        assert(Math.abs(image.width - bounds.width) < 1 && Math.abs(image.height - bounds.height) < 1, 'image fills ratio frame');
      }
      await select('Ratio').selectOption('3-2'); await slider('Width').focus(); await page.keyboard.press('End'); await tick();
      assert(await frame.evaluate(node => node.getBoundingClientRect().width <= node.parentElement.clientWidth + 1), 'ratio width exceeds preview');
      assert.equal(await frame.locator('img').evaluate(node => getComputedStyle(node).objectFit), 'cover');
      if (!hydrate) await page.locator('.site-layout-playground').screenshot({ path: `${output}/layout-aspect-${theme}-${scheme}.png` });
      if (theme === 'blue' && scheme === 'light' && !hydrate) await mobile('aspect');
      report.reports.push({ suite: `aspect-${mode}`, status: 'passed', checks: 'actual image geometry, responsive width, five ratios, retained image and desktop/mobile controls' });

      await open('splitter', 'splitter', theme, scheme, hydrate);
      assert.equal(await page.locator('.site-title').textContent(), 'Splitter');
      assert.equal(await page.locator('.site-sidebar').getByRole('link', { name: /Resizable/ }).count(), 0);
      const root = page.locator('.site-splitter-preview');
      const separator = root.getByRole('separator', { name: 'Resize panes', exact: true });
      const first = root.locator('.sapid-splitter-first');
      const second = root.locator('.sapid-splitter-second');
      await first.evaluate(node => { window.layoutFirst = node; });
      await second.evaluate(node => { window.layoutSecond = node; });
      await separator.evaluate(node => { window.layoutHandle = node; });
      const line = root.locator('.sapid-splitter-line');
      const grip = root.locator('.sapid-splitter-grip');
      assert.equal((await line.boundingBox()).width, 6);
      assert.equal(await separator.evaluate(node => getComputedStyle(node).cursor), 'col-resize');
      assert.deepEqual(await grip.evaluate(node => [node.offsetWidth, node.offsetHeight]), [2, 12]);
      assert.equal(await grip.evaluate(node => node.children.length), 3);
      await slider('Line thickness').focus(); await page.keyboard.press('End'); await tick();
      assert.equal((await line.boundingBox()).width, 6);
      assert.equal((await separator.boundingBox()).width, 6);
      await page.keyboard.press('Home'); await tick();
      assert.equal(await line.evaluate(node => node.offsetWidth), 0);
      assert(await grip.isVisible());
      await page.keyboard.press('ArrowRight');
      await toggle('Show grip').click(); await tick();
      assert(await grip.isHidden());
      await button('30 / 70').click(); await tick();
      assert.equal(await separator.getAttribute('aria-valuenow'), '30');
      const a = await first.boundingBox(), b = await second.boundingBox();
      assert(Math.abs(a.width / (a.width + b.width) - 0.3) < 0.01, 'controlled horizontal geometry');
      const firstFrame = await separator.evaluate(async node => {
        const [first, , second] = node.closest('.sapid-splitter-frame').children;
        const originalSetAttribute = Element.prototype.setAttribute;
        const originalSetProperty = CSSStyleDeclaration.prototype.setProperty;
        let fullStyles = 0, growWrites = 0;
        Element.prototype.setAttribute = function (name, value) {
          if (name === 'style' && (this === first || this === second)) fullStyles++;
          return originalSetAttribute.call(this, name, value);
        };
        CSSStyleDeclaration.prototype.setProperty = function (name, value, priority) {
          if (name === 'flex-grow' && (this === first.style || this === second.style)) growWrites++;
          return originalSetProperty.call(this, name, value, priority);
        };
        const { x, y, width, height } = node.getBoundingClientRect();
        const send = (type, dx) => node.dispatchEvent(new PointerEvent(type, {
          bubbles: true, cancelable: true, isPrimary: true, pointerId: 41, pointerType: 'mouse',
          button: 0, buttons: type === 'pointerup' ? 0 : 1,
          clientX: x + width / 2 + dx, clientY: y + height / 2,
        }));
        const originalOverflow = first.style.overflowX;
        try {
          send('pointerdown', 0);
          send('pointermove', 12);
          await new Promise(requestAnimationFrame);
          return {
            value: Number(node.getAttribute('aria-valuenow')), fullStyles, growWrites,
            preservedOverflow: first.style.overflowX === originalOverflow,
          };
        } finally {
          Element.prototype.setAttribute = originalSetAttribute;
          CSSStyleDeclaration.prototype.setProperty = originalSetProperty;
          send('pointerup', 12);
        }
      });
      assert(firstFrame.value > 30, 'a move queued with press missed the first frame');
      assert.equal(firstFrame.growWrites, 2, 'resizing should write one flex-grow property per pane');
      assert.equal(firstFrame.fullStyles, 0, 'resizing rewrote a complete pane style');
      assert(firstFrame.preservedOverflow, 'resizing changed unrelated inline declarations');
      await button('30 / 70').click(); await tick();
      await separator.focus(); await page.keyboard.press('ArrowRight'); await tick();
      assert.equal(await separator.getAttribute('aria-valuenow'), '31');
      await page.keyboard.press('Home'); await tick(); assert.equal(await separator.getAttribute('aria-valuenow'), '20');
      await page.keyboard.press('End'); await tick(); assert.equal(await separator.getAttribute('aria-valuenow'), '80');
      await button('50 / 50').click(); await tick();
      const handleBox = await separator.boundingBox();
      const startX = Math.round(handleBox.x + handleBox.width / 2);
      const monitorRaw = theme === 'blue' && scheme === 'light' && !hydrate && await page.evaluate(() => {
        if (!isSecureContext || !('onpointerrawupdate' in window)) return false;
        const add = document.addEventListener, remove = document.removeEventListener;
        const counts = { added: 0, removed: 0 };
        document.addEventListener = function (name, ...args) {
          if (name === 'pointerrawupdate') counts.added++;
          return add.call(this, name, ...args);
        };
        document.removeEventListener = function (name, ...args) {
          if (name === 'pointerrawupdate') counts.removed++;
          return remove.call(this, name, ...args);
        };
        window.layoutRawMotion = {
          counts,
          stop() {
            delete document.addEventListener;
            delete document.removeEventListener;
            delete window.layoutRawMotion;
            return counts;
          },
        };
        return true;
      });
      await page.mouse.move(startX, handleBox.y + handleBox.height / 2);
      if (monitorRaw) assert.equal(await page.evaluate(() => window.layoutRawMotion.counts.added), 0, 'raw listener installed before capture');
      await page.mouse.down(); await tick();
      await page.mouse.move(startX + 0.5, handleBox.y + handleBox.height / 2);
      await tick();
      assert(Number(await separator.getAttribute('aria-valuenow')) > 50, 'subpixel movement should resize the held separator');
      await page.mouse.move(handleBox.x + handleBox.width / 2 + 35, handleBox.y + handleBox.height / 2, { steps: 5 });
      await tick();
      assert(Number(await separator.getAttribute('aria-valuenow')) > 50, 'drag did not resize while the grip was hidden');
      await page.mouse.up(); await tick();
      if (monitorRaw) {
        assert.deepEqual(await page.evaluate(() => window.layoutRawMotion.stop()), { added: 1, removed: 1 },
          'raw motion listener lifetime did not match the captured drag');
      }
      await toggle('Show grip').click(); await tick();
      assert(await grip.isVisible());
      await select('Orientation').selectOption('vertical'); await tick();
      assert.equal((await line.boundingBox()).height, 1);
      assert.deepEqual(await grip.evaluate(node => [node.offsetWidth, node.offsetHeight]), [12, 2]);
      assert(await first.evaluate(node => node === window.layoutFirst));
      assert(await second.evaluate(node => node === window.layoutSecond));
      assert(await separator.evaluate(node => node === window.layoutHandle));
      assert.equal(await separator.getAttribute('aria-orientation'), 'horizontal');
      assert.equal(await root.locator('.sapid-splitter-frame').evaluate(node => getComputedStyle(node).flexDirection), 'column');
      await button('50 / 50').click(); await tick(); await separator.focus(); await page.keyboard.press('ArrowDown'); await tick();
      assert.equal(await separator.getAttribute('aria-valuenow'), '51');
      await toggle('Disabled').click(); await tick();
      assert.equal(await separator.getAttribute('aria-disabled'), 'true');
      assert.equal(await separator.getAttribute('tabindex'), '-1');
      await toggle('Disabled').click(); await toggle('Collapsible').click(); await button('Collapse first pane').click(); await tick();
      assert.equal(await separator.getAttribute('aria-valuenow'), '0'); assert(await first.isHidden());
      await button('Expand first pane').click(); await tick(); assert.equal(await separator.getAttribute('aria-valuenow'), '51');
      await button('Collapse first pane').click(); await toggle('Collapsible').click(); await tick();
      assert.equal(await separator.getAttribute('aria-valuenow'), '50');
      assert.equal(await page.getByRole('separator').count(), 9, 'primary, nested, fixed, multiple and redistributed handles');
      const nested = page.getByRole('separator', { name: 'Resize sidebar', exact: true });
      await nested.scrollIntoViewIfNeeded();
      const nestedBox = await nested.boundingBox();
      const grabOffset = nestedBox.width / 4;
      const x = nestedBox.x + grabOffset, y = nestedBox.y + nestedBox.height / 2;
      const nestedFrame = await nested.locator('xpath=../..').boundingBox();
      const axisExtent = nestedFrame.width - nestedBox.width;
      const minX = x - axisExtent * 0.15, maxX = x + axisExtent * 0.4;
      const assertAligned = async pointer => {
        const bounds = await nested.boundingBox();
        assert(Math.abs(bounds.x + grabOffset - pointer) < 0.1, 'divider lost its original pointer grab offset');
      };
      await page.mouse.move(x, y); await page.mouse.down(); await tick();
      assert.equal(await nested.evaluate(node => getComputedStyle(node).outlineStyle), 'none', 'mouse press showed a focus outline');
      for (const [boundary, beyond, value] of [[minX, -24, 15], [maxX, 24, 70]]) {
        await page.mouse.move(boundary + beyond, y); await tick();
        assert.equal(Number(await nested.getAttribute('aria-valuenow')), value);
        await page.mouse.move(boundary + beyond / 2, y); await tick();
        assert.equal(Number(await nested.getAttribute('aria-valuenow')), value, 'divider left its limit while the pointer was outside the range');
        const inside = boundary - beyond / 2;
        await page.mouse.move(inside, y); await tick();
        await assertAligned(inside);
      }
      await page.mouse.move(x, y); await tick();
      await assertAligned(x);
      await page.mouse.up(); await tick();
      await assertAligned(x);
      assert.equal(await nested.evaluate(node => getComputedStyle(node).outlineStyle), 'none', 'mouse release left a focus outline');
      await page.keyboard.press('ArrowLeft'); await tick();
      assert.equal(await nested.evaluate(node => getComputedStyle(node).outlineStyle), 'solid', 'keyboard focus lost its themed outline');
      await select('Orientation').selectOption('horizontal'); await tick();
      const fixedSidebar = page.locator('.site-splitter-fixed-sidebar');
      assert(Math.abs((await fixedSidebar.boundingBox()).width - 240) < 1, `${mode}: fixed 240px sidebar`);
      const flexible = page.getByRole('separator', { name: 'Resize flexible content', exact: true });
      await flexible.focus(); await page.keyboard.press('ArrowDown'); await tick();
      assert.equal(await flexible.getAttribute('aria-valuenow'), '61', `${mode}: flexible content remains resizable`);
      for (const label of ['Panel A', 'Panel B', 'Panel C', 'Panel D']) {
        assert(await page.locator('.site-splitter-multiple').getByText(label, { exact: true }).count() > 0, `${mode}: four workspaces include ${label}`);
      }
      const multiple = page.getByRole('separator', { name: 'Resize workspace groups', exact: true });
      await multiple.focus(); await page.keyboard.press('ArrowRight'); await tick();
      assert.equal(await multiple.getAttribute('aria-valuenow'), '46', `${mode}: multiple pane groups resize independently`);
      await button('Editor focus').click(); await tick();
      await page.locator('.site-splitter-redistribute-sizes').getByText('Current sizes: [30, 49, 21]', { exact: true }).waitFor();
      await button('Balanced').click(); await tick();
      await page.locator('.site-splitter-redistribute-sizes').getByText('Current sizes: [50, 25, 25]', { exact: true }).waitFor();
      await button('Terminal focus').click(); await tick();
      await page.locator('.site-splitter-redistribute-sizes').getByText('Current sizes: [65, 12, 23]', { exact: true }).waitFor();
      if (theme === 'blue' && scheme === 'light' && !hydrate) await page.screenshot({ path: `${output}/layout-splitter-examples.png`, fullPage: true });
      if (!hydrate) await page.locator('.site-layout-playground').screenshot({ path: `${output}/layout-splitter-${theme}-${scheme}.png` });
      if (theme === 'blue' && scheme === 'light' && !hydrate) await mobile('splitter');
      report.reports.push({ suite: `splitter-${mode}`, status: 'passed', checks: 'controlled sizes, subpixel drag, retained grab offset through both limits, keyboard-only focus ring, live line sizing, optional grip and stable orientation' });

      await open('scroll-area', 'scroll-area', theme, scheme, hydrate);
      const group = page.locator('.site-nav-group').filter({ has: page.getByText('Miscellaneous', { exact: true }) });
      assert.equal(await group.getByRole('link', { name: 'ScrollArea', exact: true }).count(), 1);
      assert.equal(await page.locator('.site-nav-group').filter({ has: page.getByText('Layout', { exact: true }) }).getByRole('link', { name: 'ScrollArea', exact: true }).count(), 0);
      const viewport = page.locator('.site-scroll-preview');
      const draft = page.getByRole('textbox', { name: 'Scroll area draft', exact: true });
      await viewport.evaluate(node => { window.layoutViewport = node; });
      await draft.fill('Retained scroll draft');
      await draft.evaluate(node => { window.layoutDraft = node; node.setSelectionRange(2, 9); });
      for (const type of ['auto', 'always', 'never', 'scroll', 'hover']) {
        await select('Type').selectOption(type); await tick();
        assert(await viewport.evaluate((node, type) => node.classList.contains(`sapid-scroll-${type}`), type));
        assert(await viewport.evaluate(node => node === window.layoutViewport));
      }
      await select('Type').selectOption('hover');
      let box = await viewport.boundingBox();
      await page.mouse.move(box.x + box.width / 2, box.y + 40); await tick();
      assert(await viewport.evaluate(node => node.classList.contains('is-scrolling')), 'hover shows scrollbar');
      await page.mouse.move(box.x + box.width + 100, box.y + 40); await tick();
      assert(await viewport.evaluate(node => node.classList.contains('is-scrolling')), 'hover delay was skipped');
      await page.waitForFunction(() => !document.querySelector('.site-scroll-preview').classList.contains('is-scrolling'));
      await select('Type').selectOption('scroll');
      box = await viewport.boundingBox(); await page.mouse.move(box.x + box.width / 2, box.y + 40); await page.mouse.wheel(0, 200); await tick();
      assert(await viewport.evaluate(node => node.scrollTop > 0 && node.classList.contains('is-scrolling')), 'wheel scroll shows scrollbar');
      await page.waitForFunction(() => !document.querySelector('.site-scroll-preview').classList.contains('is-scrolling'));
      await select('Type').selectOption('never'); await tick();
      assert.equal(await viewport.evaluate(node => getComputedStyle(node, '::-webkit-scrollbar').display), 'none');
      await button('Scroll to top').click(); await page.waitForFunction(() => document.querySelector('.site-scroll-preview').scrollTop === 0);
      await select('Type').selectOption('always'); await slider('Scrollbar size').focus(); await page.keyboard.press('End'); await tick();
      assert.equal(await viewport.evaluate(node => getComputedStyle(node, '::-webkit-scrollbar').width), '24px');
      await select('Offset scrollbars').selectOption('both'); await tick();
      assert.equal(await viewport.locator('.sapid-scroll-content').evaluate(node => getComputedStyle(node).paddingRight), '24px');
      assert.equal(await viewport.locator('.sapid-scroll-content').evaluate(node => getComputedStyle(node).paddingBottom), '24px');
      await select('Overscroll behavior').selectOption('contain'); await tick();
      assert.equal(await viewport.evaluate(node => getComputedStyle(node).overscrollBehavior), 'contain');
      await toggle('Wide content').click(); await tick();
      assert(await viewport.evaluate(node => node.scrollWidth >= 750), 'horizontal content extent');
      await button('Scroll to center').click(); await page.waitForFunction(() => { const node = document.querySelector('.site-scroll-preview'); return Math.abs(node.scrollTop - (node.scrollHeight - node.clientHeight) / 2) < 1; });
      await button('Scroll to bottom').click(); await page.waitForFunction(() => { const node = document.querySelector('.site-scroll-preview'); return Math.abs(node.scrollTop - (node.scrollHeight - node.clientHeight)) < 1; });
      await select('Overscroll behavior').selectOption('none'); await tick();
      const windowY = await page.evaluate(() => scrollY);
      box = await viewport.boundingBox(); await page.mouse.move(box.x + box.width / 2, box.y + 40); await page.mouse.wheel(0, 350); await page.waitForTimeout(100);
      assert.equal(await page.evaluate(() => scrollY), windowY, 'contained viewport leaked scroll to document');
      await select('Scrollbars').selectOption('y'); await tick();
      assert.equal(await viewport.evaluate(node => getComputedStyle(node).overflowX), 'hidden');
      await select('Scrollbars').selectOption('x'); await tick(); assert(await button('Scroll to bottom').isDisabled());
      await select('Scrollbars').selectOption('xy'); await tick();
      assert(await draft.evaluate(node => node === window.layoutDraft && node.value === 'Retained scroll draft' && node.selectionStart === 2 && node.selectionEnd === 9), 'scroll settings replaced or rewrote editor');
      if (!hydrate) await page.locator('.site-layout-playground').screenshot({ path: `${output}/layout-scroll-${theme}-${scheme}.png` });
      if (theme === 'blue' && scheme === 'light' && !hydrate) await mobile('scroll');
      report.reports.push({ suite: `scroll-${mode}`, status: 'passed', checks: 'Extensions navigation, five real scrollbar modes, timed hover/scroll, size/offset, both axes, overscroll, exact center/end and retained editor/selection' });
      console.log(`Layout ${mode}: passed`);
    }
  } catch (error) { report.errors.push(error.stack || String(error)); }
  finally {
    await browser.close();
    fs.writeFileSync(`${output}/showcase-layout-browser.json`, JSON.stringify(report, null, 2));
    console.log(JSON.stringify(report, null, 2));
    if (report.errors.length) process.exitCode = 1;
  }
})();
