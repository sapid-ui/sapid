const fs = require('node:fs');
const assert = require('node:assert/strict');
const { chromium } = require(process.env.SAPID_PLAYWRIGHT || 'D:/Works/glory-rs/glory/tests/playwright/node_modules/playwright');
process.env.TEMP = process.env.TMP = 'D:/BuildCache/sapid/tmp';
const base = process.env.SAPID_SHOWCASE_BASE || 'http://127.0.0.1:8795';
const output = 'D:/BuildCache/sapid';

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
      await open('P3-04', 'aspect-ratio', theme, scheme, hydrate);
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

      await open('P6-08', 'splitter', theme, scheme, hydrate);
      assert.equal(await page.locator('.site-title').textContent(), 'Splitter');
      assert.equal(await page.locator('.site-sidebar').getByRole('link', { name: /Resizable/ }).count(), 0);
      const root = page.locator('.site-splitter-preview');
      const separator = root.getByRole('separator', { name: 'Resize panes', exact: true });
      const first = root.locator('.sapid-splitter-first');
      const second = root.locator('.sapid-splitter-second');
      await first.evaluate(node => { window.layoutFirst = node; });
      await second.evaluate(node => { window.layoutSecond = node; });
      await separator.evaluate(node => { window.layoutHandle = node; });
      await button('30 / 70').click(); await tick();
      assert.equal(await separator.getAttribute('aria-valuenow'), '30');
      const a = await first.boundingBox(), b = await second.boundingBox();
      assert(Math.abs(a.width / (a.width + b.width) - 0.3) < 0.01, 'controlled horizontal geometry');
      await separator.focus(); await page.keyboard.press('ArrowRight'); await tick();
      assert.equal(await separator.getAttribute('aria-valuenow'), '31');
      await page.keyboard.press('Home'); await tick(); assert.equal(await separator.getAttribute('aria-valuenow'), '20');
      await page.keyboard.press('End'); await tick(); assert.equal(await separator.getAttribute('aria-valuenow'), '80');
      await button('50 / 50').click(); await tick();
      const handleBox = await separator.boundingBox();
      await page.mouse.move(handleBox.x + handleBox.width / 2, handleBox.y + handleBox.height / 2);
      await page.mouse.down(); await tick();
      await page.mouse.move(handleBox.x + handleBox.width / 2 + 35, handleBox.y + handleBox.height / 2, { steps: 5 });
      await page.mouse.up(); await tick();
      assert(Number(await separator.getAttribute('aria-valuenow')) > 50, 'real pointer drag did not resize panes');
      await select('Orientation').selectOption('vertical'); await tick();
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
      assert.equal(await page.getByRole('separator').count(), 3, 'nested splitter handles');
      await select('Orientation').selectOption('horizontal'); await tick();
      if (!hydrate) await page.locator('.site-layout-playground').screenshot({ path: `${output}/layout-splitter-${theme}-${scheme}.png` });
      if (theme === 'blue' && scheme === 'light' && !hydrate) await mobile('splitter');
      report.reports.push({ suite: `splitter-${mode}`, status: 'passed', checks: 'rename, controlled sizes, actual drag, arrows/bounds, live orientation identity, disable, collapse/restore and nested panes' });

      await open('P6-07', 'scroll-area', theme, scheme, hydrate);
      const group = page.locator('.site-nav-group').filter({ has: page.getByText('Extensions', { exact: true }) });
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
