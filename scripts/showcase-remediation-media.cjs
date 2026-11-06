const assert = require('node:assert/strict');
const fs = require('node:fs');
const {matrix, chromium, base, outputPath, writeReport, tempDir } = require('./showcase-browser-env.cjs');

(async () => {
  const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext();
  const page = await context.newPage();
  const report = { browser: browser.version(), pages: [], errors: [] };
  page.on('pageerror', error => report.errors.push(error.message));
  page.on('console', message => { if (message.type() === 'error') report.errors.push(message.text()); });
  const tick = () => page.evaluate(() => new Promise(resolve => { let n = 0; const frame = () => ++n === 5 ? resolve() : requestAnimationFrame(frame); requestAnimationFrame(frame); }));
  const detail = title => page.locator('.site-component-detail').filter({ has: page.getByRole('heading', { name: title, exact: true }) });
  const choose = async (target, files) => {
    const pending = page.waitForEvent('filechooser');
    await target.click();
    const chooser = await pending;
    const paths = files.map(file => {
      const path = require('node:path').join(tempDir, file.name);
      fs.writeFileSync(path, file.buffer);
      return path;
    });
    await chooser.setFiles(paths);
    await tick();
  };
  const text = { name: 'review.txt', mimeType: 'text/plain', buffer: Buffer.from('A bounded review\n第二行') };
  fs.mkdirSync(outputPath('media-remediation'), { recursive: true });
  try {
    for (const theme of matrix.themes) for (const scheme of matrix.schemes) for (const width of [1280, 390]) {
      for (const slug of ['floating-window', 'carousel', 'dropzone']) {
        const key = `${slug}-${theme}-${scheme}-${width}`;
        await page.setViewportSize({ width, height: 1000 });
        await page.goto(`${base}/showcase.html?widget=${slug}&theme=${theme}&scheme=${scheme}`);
        await page.waitForFunction(slug => window.ready === true && document.documentElement.getAttribute('data-sapid-widget') === slug, slug);
        await tick();
        report.asset = await page.locator('link[rel=stylesheet]').evaluateAll(elements => elements.map(element => element.href.split('/').find(part => part.startsWith('assets-'))).find(Boolean));
        const primary = page.locator('.site-preview .site-demo').first();
        const overflow = await primary.evaluate(element => ({ local: element.scrollWidth - element.clientWidth, page: document.documentElement.scrollWidth - innerWidth }));
        assert(overflow.local <= 1 && overflow.page <= 1, `${key}: ${JSON.stringify(overflow)}`);
        assert.equal(await page.locator('.site-component-detail').count(), 4);
        if (slug === 'floating-window') {
          assert.equal(await page.getByRole('dialog').count(), 0, 'windows start closed');
          await primary.getByRole('button', { name: 'Open notes', exact: true }).click(); await tick();
          const panel = page.getByRole('dialog', { name: 'Release notes', exact: true });
          const box = await panel.boundingBox();
          assert(box.width <= 302 && box.x >= 0 && box.x + box.width <= width + 1);
          const close = panel.getByRole('button', { name: 'Close window', exact: true });
          assert(await close.evaluate(element => document.activeElement === element));
          for (const action of [close, panel.getByRole('button', { name: 'Window actions', exact: true })]) {
            const bounds = await action.boundingBox(); assert(bounds.width <= 34 && bounds.height <= 34);
            assert.equal(await action.locator('svg').evaluate(element => getComputedStyle(element).pointerEvents), 'none');
          }
          assert.equal(await panel.locator('.sapid-window-command:visible').count(), 0);
          const editor = panel.getByRole('textbox', { name: 'Notes', exact: true });
          await editor.fill('Retained draft'); await editor.evaluate(element => { window.mediaEditor = element; element.setSelectionRange(2, 6); });
          const caption = panel.locator('.sapid-window-caption'); const captionBox = await caption.boundingBox();
          await page.mouse.move(captionBox.x + 30, captionBox.y + 12); await page.mouse.down(); await page.mouse.move(captionBox.x + 50, captionBox.y + 32, { steps: 5 }); await page.mouse.up(); await tick();
          assert(await editor.evaluate(element => element === window.mediaEditor && element.selectionStart === 2 && element.selectionEnd === 6));
          const moved = await panel.boundingBox(); assert(moved.x > box.x + 10 && moved.y > box.y + 10);
          await panel.screenshot({ path: outputPath(`media-remediation/${key}-open.png`) });
          await panel.getByRole('button', { name: 'Window actions', exact: true }).click(); await tick();
          assert.equal(await panel.locator('.sapid-window-command:visible').count(), 8);
          await close.click(); await tick(); await primary.getByRole('button', { name: 'Open notes', exact: true }).click(); await tick();
          assert.equal(await editor.inputValue(), 'Retained draft'); assert(await editor.evaluate(element => element === window.mediaEditor));
          await panel.screenshot({ path: outputPath(`media-remediation/${key}-actions.png`) });
          await close.click(); await tick();
          const controlled = detail('Controlled bounds and refusal');
          await controlled.getByRole('checkbox', { name: 'Accept window geometry', exact: true }).uncheck();
          await controlled.getByRole('button', { name: 'Open controlled window', exact: true }).click(); await tick();
          const refused = page.getByRole('dialog', { name: 'Controlled geometry', exact: true });
          const before = await refused.boundingBox();
          await refused.getByRole('button', { name: 'Move window right', exact: true }).click(); await tick();
          assert.equal((await refused.boundingBox()).x, before.x);
          await refused.getByRole('button', { name: 'Close window', exact: true }).click(); await tick();
          const modalSection = detail('Nonmodal notes and modal confirmation');
          await modalSection.getByRole('button', { name: 'Open review notes', exact: true }).click(); await tick();
          const notes = page.getByRole('dialog', { name: 'Review notes', exact: true });
          await notes.getByRole('textbox', { name: 'Review draft', exact: true }).fill('Review retained');
          const trigger = notes.getByRole('button', { name: 'Request confirmation', exact: true });
          await trigger.click(); await tick(); await page.getByRole('dialog', { name: 'Confirm review', exact: true }).waitFor();
          await page.keyboard.press('Escape'); await tick();
          assert.equal(await page.getByRole('dialog', { name: 'Confirm review', exact: true }).count(), 0);
          assert(await trigger.evaluate(element => document.activeElement === element));
          assert.equal(await notes.getByRole('textbox', { name: 'Review draft', exact: true }).inputValue(), 'Review retained');
          await notes.getByRole('button', { name: 'Close window', exact: true }).click(); await tick();
          const lifetime = detail('Content lifetime');
          for (const [title, retained] of [['Retained notes', true], ['Fresh notes', false]]) {
            const open = lifetime.getByRole('button', { name: `Open ${title.toLowerCase()}`, exact: true });
            await open.click(); await tick();
            const window = page.getByRole('dialog', { name: title, exact: true });
            const draft = window.getByRole('textbox', { name: title, exact: true });
            await draft.fill('Lifecycle draft'); await draft.evaluate(element => { window.lifecycleEditor = element; });
            await window.getByRole('button', { name: 'Close window', exact: true }).click(); await tick();
            await open.click(); await tick();
            assert.equal(await draft.inputValue(), retained ? 'Lifecycle draft' : 'Edit this note, close and reopen.');
            assert.equal(await draft.evaluate(element => element === window.lifecycleEditor), retained);
            await window.getByRole('button', { name: 'Close window', exact: true }).click(); await tick();
          }
        } else if (slug === 'carousel') {
          const carousel = primary.getByRole('group', { name: 'Project highlights', exact: true });
          const next = carousel.getByRole('button', { name: 'Next slide', exact: true });
          const bounds = await next.boundingBox(); assert(bounds.width <= 34 && bounds.height <= 34);
          await next.evaluate(element => { element.click(); element.click(); }); await tick();
          assert.equal(await carousel.getByRole('button', { name: 'Go to slide 3', exact: true }).getAttribute('aria-pressed'), 'true', 'same-task commands read latest source');
          assert(await carousel.getByText('Share', { exact: true }).isVisible());
          await carousel.getByRole('button', { name: 'Go to slide 1', exact: true }).click(); await tick();
          const contrast = await carousel.locator('.sapid-carousel-indicator[aria-pressed="true"] svg').evaluate(element => {
            const luminance = color => {
              const channels = color.match(/[\d.]+/g).slice(0, 3).map(Number).map(value => value / 255).map(value => value <= 0.04045 ? value / 12.92 : ((value + 0.055) / 1.055) ** 2.4);
              return channels[0] * 0.2126 + channels[1] * 0.7152 + channels[2] * 0.0722;
            };
            let parent = element;
            while (parent && getComputedStyle(parent).backgroundColor === 'rgba(0, 0, 0, 0)') parent = parent.parentElement;
            const foreground = luminance(getComputedStyle(element.querySelector('path')).fill);
            const background = luminance(parent ? getComputedStyle(parent).backgroundColor : 'rgb(255,255,255)');
            return (Math.max(foreground, background) + 0.05) / (Math.min(foreground, background) + 0.05);
          });
          assert(contrast >= 3, `selected indicator contrast ${contrast}`);
          const geometry = detail('Control size, direction and visibility');
          const rtl = geometry.getByRole('group', { name: 'RTL cards', exact: true });
          assert((await rtl.getByRole('button', { name: 'Previous slide', exact: true }).boundingBox()).x > (await rtl.getByRole('button', { name: 'Next slide', exact: true }).boundingBox()).x);
          const hidden = geometry.getByRole('group', { name: 'Indicator navigation', exact: true });
          assert.equal(await hidden.locator('.sapid-carousel-next,.sapid-carousel-previous').count(), 0);
          await hidden.getByRole('button', { name: 'Go to slide 3', exact: true }).click(); await tick();
          assert(await hidden.getByText('Panel 3', { exact: true }).isVisible());
          const retained = detail('Windowed slides and editor lifetime').getByRole('group', { name: 'Windowed notes', exact: true });
          assert.equal(await retained.locator('.sapid-carousel-slide').count(), 3);
          assert.equal(await retained.getByRole('button', { name: /^Go to slide / }).count(), 9);
          const draft = retained.getByRole('textbox', { name: 'Draft for page 500001', exact: true });
          await draft.fill('Local slide draft'); await draft.evaluate(element => { window.mediaEditor = element; });
          await retained.getByRole('button', { name: 'Next slide', exact: true }).click(); await tick();
          await retained.getByRole('button', { name: 'Previous slide', exact: true }).click(); await tick();
          assert.equal(await draft.inputValue(), 'Local slide draft'); assert(await draft.evaluate(element => element === window.mediaEditor));
          const auto = detail('Explicit autoplay and pause policies');
          assert(await auto.getByRole('button', { name: 'Autoplay paused', exact: true }).isDisabled());
          if (width === 1280 && theme === 'blue' && scheme === 'light') {
            await auto.getByRole('checkbox', { name: 'Run autoplay', exact: true }).check();
            await page.mouse.move(0, 0); await primary.getByRole('group', { name: 'Project highlights', exact: true }).focus();
            await auto.getByRole('button', { name: 'Go to slide 2', exact: true }).waitFor();
            await page.waitForFunction(() => document.querySelector('[aria-label="Automatic cards"] .sapid-carousel-indicator[aria-pressed="true"]')?.getAttribute('aria-label') === 'Go to slide 2', undefined, { timeout: 6000 });
            await auto.getByRole('checkbox', { name: 'Run autoplay', exact: true }).uncheck(); await tick();
            assert(await auto.getByRole('button', { name: 'Autoplay paused', exact: true }).isDisabled());
          }
          await primary.screenshot({ path: outputPath(`media-remediation/${key}.png`) });
        } else {
          await choose(primary.getByRole('button', { name: 'Drop files here or choose files', exact: true }), [text]);
          assert.equal(await primary.getByRole('button', { name: 'Remove review.txt', exact: true }).count(), 1);
          assert(await primary.getByText('1 file selected', { exact: true }).isVisible());
          const limits = detail('File limits and rejection');
          const chooser = limits.getByRole('button', { name: 'Choose text or PNG files', exact: true });
          await choose(chooser, [text]);
          await choose(chooser, [{ name: 'bad.pdf', mimeType: 'application/pdf', buffer: Buffer.from('%PDF') }]);
          assert.equal(await limits.getByRole('button', { name: 'Remove review.txt', exact: true }).count(), 1);
          assert(await limits.getByText('Choose text or PNG files', { exact: true }).last().isVisible());
          const read = detail('Bounded text read and attachment content');
          await choose(read.getByRole('button', { name: 'Choose a small text file', exact: true }), [text]);
          await read.getByRole('button', { name: 'Read text', exact: true }).click();
          await read.getByText('A bounded review\n第二行', { exact: true }).waitFor();
          await read.getByRole('button', { name: 'Remove text file', exact: true }).click(); await tick();
          assert.equal(await read.getByRole('button', { name: 'Read text', exact: true }).count(), 0);
          const states = detail('Disabling and reactive configuration');
          const configured = states.getByRole('button', { name: 'Configured text files', exact: true });
          await choose(configured, [text]);
          await states.getByRole('checkbox', { name: 'Allow multiple text files', exact: true }).check(); await tick();
          assert.equal(await states.getByRole('button', { name: 'Remove review.txt', exact: true }).count(), 0);
          await states.getByRole('checkbox', { name: 'Enable file selection', exact: true }).uncheck(); await tick(); assert(await configured.isDisabled());
          // A genuine Chromium drag operation supplies the owned on-disk fixture.
          const path = `${tempDir}/media-drop.txt`; fs.writeFileSync(path, 'Actual Dropzone fixture');
          const target = primary.getByRole('button', { name: 'Drop files here or choose files', exact: true });
          await target.scrollIntoViewIfNeeded(); const box = await target.boundingBox();
          const cdp = await context.newCDPSession(page); const data = { items: [], files: [path], dragOperationsMask: 1 };
          for (const type of ['dragEnter', 'dragOver', 'drop']) await cdp.send('Input.dispatchDragEvent', { type, x: box.x + box.width / 2, y: box.y + box.height / 2, data });
          await tick(); await cdp.detach();
          await primary.getByRole('button', { name: 'Remove media-drop.txt', exact: true }).waitFor();
          const previews = detail('Explicit image previews');
          const image = fs.readFileSync(require('node:path').join(__dirname, '../examples/web/assets/image-pattern.png'));
          await choose(previews.getByRole('button', { name: 'Choose PNG or JPEG images', exact: true }), [{ name: 'pattern.png', mimeType: 'image/png', buffer: image }]);
          await previews.getByRole('button', { name: 'Preview pattern.png', exact: true }).click();
          await previews.locator('.sapid-file-preview').waitFor();
          await page.waitForFunction(() => [...document.querySelectorAll('.sapid-file-preview')].some(element => element.complete && element.naturalWidth > 0));
          await previews.getByRole('button', { name: 'Remove pattern.png', exact: true }).click(); await tick();
          assert.equal(await previews.locator('.sapid-file-preview').count(), 0);
          await primary.screenshot({ path: outputPath(`media-remediation/${key}.png`) });
        }
        report.pages.push({ key, passed: true }); console.log(`PASS ${key}`);
      }
    }
    assert.deepEqual(report.errors, []);
  } catch (error) {
    report.failure = error.stack;
    await page.screenshot({ path: outputPath('media-remediation/failure.png'), fullPage: true });
    throw error;
  } finally {
    writeReport('media-remediation.json', report);
    await browser.close();
  }
})().catch(error => { console.error(error); process.exitCode = 1; });
