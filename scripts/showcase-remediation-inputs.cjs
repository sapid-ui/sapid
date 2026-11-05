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
  const identity = locator => locator.evaluate(element => { window.inputIdentity = element; });
  const same = locator => locator.evaluate(element => element === window.inputIdentity);
  const successful = (section, name) => section.evaluate((element, name) => {
    const form = element.matches('form') ? element : element.querySelector('form');
    if (form) return new FormData(form).getAll(name);
    return [...element.querySelectorAll('[name]')].filter(control => control.name === name).map(control => control.value);
  }, name);
  const inset = async (section, editor) => {
    const rect = await editor.boundingBox();
    assert(rect && rect.width >= 100);
    const actions = section.locator('.sapid-select-clear,.sapid-date-picker-clear,.sapid-date-time-clear,.sapid-period-picker-clear,.sapid-dialog-trigger');
    for (const action of await actions.all()) {
      if (!await action.isVisible()) continue;
      const box = await action.boundingBox();
      assert(box.width <= 48 && box.height <= 56, 'compact action bounds');
      assert(box.x >= rect.x - 1 && box.x + box.width <= rect.x + rect.width + 1, 'action inside full input frame');
      assert(box.y >= rect.y - 1 && box.y + box.height <= rect.y + rect.height + 1, 'action inside input height');
      assert((await action.getAttribute('aria-label'))?.length > 3, 'full accessible action name');
      const glyph = action.locator('svg');
      if (await glyph.count()) assert.equal(await glyph.first().evaluate(element => getComputedStyle(element).pointerEvents), 'none');
    }
  };
  fs.mkdirSync(outputPath('input-remediation'), { recursive: true });
  try {
    for (const theme of ['blue', 'green']) for (const scheme of ['light', 'dark']) for (const width of [1280, 390]) {
      for (const slug of ['select', 'date-picker-input', 'date-time-picker', 'month-picker-input', 'year-picker-input']) {
        const key = `${slug}-${theme}-${scheme}-${width}`;
        await page.setViewportSize({ width, height: 1000 });
        await page.goto(`${base}/showcase.html?widget=${slug}&theme=${theme}&scheme=${scheme}`);
        await page.waitForFunction(slug => window.ready === true && document.documentElement.getAttribute('data-sapid-widget') === slug, slug);
        await tick();
        report.asset = await page.locator('link[rel=stylesheet]').evaluateAll(elements => elements.map(element => element.href.split('/').find(part => part.startsWith('assets-'))).find(Boolean));
        const primary = page.locator('.site-preview .site-demo').first();
        const overflow = await primary.evaluate(element => ({ local: element.scrollWidth - element.clientWidth, page: document.documentElement.scrollWidth - innerWidth }));
        assert(overflow.local <= 1 && overflow.page <= 1, `${key}: ${JSON.stringify(overflow)}`);
        if (slug === 'select') {
          const editor = primary.getByRole('combobox', { name: 'Your favorite framework', exact: true });
          await identity(editor);
          await inset(primary, editor);
          const textBounds = await editor.locator('.sapid-select-value').boundingBox();
          const fieldBounds = await editor.boundingBox();
          assert(textBounds.x - fieldBounds.x <= 16, 'selected text starts at input padding');
          assert.notEqual(await editor.locator('.sapid-select-value').evaluate(element => getComputedStyle(element).color), await editor.evaluate(element => getComputedStyle(element).color), 'placeholder is visually distinct');
          await editor.click();
          await page.getByRole('listbox', { name: 'Your favorite framework', exact: true }).getByRole('option', { name: 'Vue', exact: true }).click();
          await tick();
          assert.equal(await editor.locator('.sapid-select-value').textContent(), 'Vue');
          assert.equal(await editor.locator('.sapid-select-value').evaluate(element => getComputedStyle(element).color), await editor.evaluate(element => getComputedStyle(element).color), 'accepted value uses input foreground');
          await inset(primary, editor);
          await primary.getByRole('button', { name: 'Clear selection', exact: true }).click(); await tick();
          assert(await editor.evaluate(element => document.activeElement === element)); assert(await same(editor));
          assert.equal(await editor.locator('.sapid-select-value').textContent(), 'Choose a framework');
          const states = detail('Field states and controlled refusal');
          const accepted = states.getByRole('combobox', { name: 'Accepted language', exact: true });
          await states.getByRole('checkbox', { name: 'Accept select changes', exact: true }).uncheck();
          await states.getByRole('button', { name: 'Clear selection', exact: true }).click(); await tick();
          assert.equal(await accepted.locator('.sapid-select-value').textContent(), 'Rust');
          assert(await states.getByRole('combobox', { name: 'Disabled selection', exact: true }).isDisabled());
          assert.equal(await states.getByRole('combobox', { name: 'Required selection', exact: true }).getAttribute('aria-invalid'), 'true');
          const long = detail('Dropdown height and placement').locator('.sapid-select-value');
          assert.equal(await long.evaluate(element => getComputedStyle(element).textOverflow), 'ellipsis');
          if (width === 390) assert(await long.evaluate(element => element.scrollWidth > element.clientWidth));
        } else {
          const names = { 'date-picker-input': ['Appointment date','Choose appointment date','Clear dates'], 'date-time-picker': ['Appointment','Choose date and time','Clear date and time'], 'month-picker-input': ['Billing month','Choose months','Clear months'], 'year-picker-input': ['Graduation year','Choose years','Clear years'] };
          const [name, triggerName, clearName] = names[slug];
          const editor = primary.getByRole('textbox', { name, exact: true });
          await identity(editor); await inset(primary, editor);
          const trigger = primary.getByRole('button', { name: triggerName, exact: true });
          await trigger.click();
          const dialog = page.getByRole('dialog'); await dialog.waitFor();
          if (slug.startsWith('month')) await dialog.getByRole('button', { name: 'November 2026', exact: true }).click();
          else if (slug.startsWith('year')) await dialog.getByRole('button', { name: '2026', exact: true }).click();
          else {
            await dialog.getByRole('button', { name: 'Wednesday, November 4, 2026', exact: true }).click();
            if (slug === 'date-time-picker') await dialog.getByRole('button', { name: 'Done', exact: true }).click();
          }
          await dialog.waitFor({ state: 'hidden' }); await tick();
          assert((await editor.inputValue()).length > 0); assert(await same(editor)); await inset(primary, editor);
          await primary.screenshot({ path: outputPath(`input-remediation/${key}-selected.png`) });
          await primary.getByRole('button', { name: clearName, exact: true }).click(); await tick();
          assert.equal(await editor.inputValue(), ''); assert(await same(editor));
          assert(await editor.evaluate(element => document.activeElement === element));
          await trigger.click(); await dialog.waitFor(); await page.keyboard.press('Escape'); await dialog.waitFor({ state: 'hidden' });
          assert(await trigger.evaluate(element => document.activeElement === element));

          if (slug === 'date-picker-input' || slug === 'date-time-picker') {
            const time = slug === 'date-time-picker';
            const section = detail(time ? 'Inset actions and ISO form submission' : 'Clear, controlled refusal and forms');
            const field = section.getByRole('textbox', { name: time ? 'Formatted appointment time' : 'Formatted appointment date', exact: true });
            const formName = time ? 'appointment_time' : 'appointment_date';
            const expected = time ? '2026-11-04T10:30:00' : '2026-11-04';
            assert.equal(await field.inputValue(), time ? '04/11/2026 10:30' : '04/11/2026');
            assert.deepEqual(await successful(section, formName), [expected]);
            await identity(field);
            const acceptance = section.getByRole('checkbox', { name: time ? 'Accept date time changes' : 'Accept date changes', exact: true });
            const clear = section.getByRole('button', { name: clearName, exact: true });
            await acceptance.uncheck(); await clear.click(); await tick();
            assert.deepEqual(await successful(section, formName), [expected]);
            await acceptance.check();
            const readonly = section.getByRole('checkbox', { name: time ? 'Read only date time' : 'Read only date', exact: true });
            await readonly.check(); await tick(); assert(await clear.isDisabled()); await readonly.uncheck(); await tick();
            await clear.click(); await tick();
            assert.deepEqual(await successful(section, formName), ['']); assert(await same(field));
            assert(await field.evaluate(element => document.activeElement === element));
            await section.getByRole('button', { name: time ? 'Reset date time' : 'Reset date', exact: true }).click(); await tick();
            assert.deepEqual(await successful(section, formName), [expected]);
            await section.getByRole('button', { name: time ? 'Submit date time' : 'Submit date', exact: true }).click(); await tick();
            assert(await section.getByText(`${time ? 'Submitted date time' : 'Submitted date'}: ${expected}`, { exact: true }).isVisible());
          } else {
            const kind = slug.startsWith('month') ? 'month' : 'year';
            const format = detail('Display format and locale');
            const localized = format.getByRole('textbox', { name: `Localized ${kind}`, exact: true });
            const initial = kind === 'month' ? '2026-11' : '2026';
            await identity(localized);
            assert.deepEqual(await successful(format, `localized_${kind}`), [initial]);
            await format.getByRole('button', { name: `Toggle ${kind} locale`, exact: true }).click(); await tick();
            assert.equal(await localized.inputValue(), kind === 'month' ? '2026年11月' : '2026年');
            assert.deepEqual(await successful(format, `localized_${kind}`), [initial]); assert(await same(localized));
            const controlled = detail('Controlled and clearable');
            await controlled.getByRole('checkbox', { name: `Accept ${kind} changes`, exact: true }).uncheck();
            await controlled.getByRole('button', { name: clearName, exact: true }).click(); await tick();
            assert.equal(await controlled.getByRole('textbox', { name: `Controlled ${kind}`, exact: true }).inputValue(), initial);
            await controlled.getByRole('checkbox', { name: `Accept ${kind} changes`, exact: true }).check();
            await controlled.getByRole('button', { name: clearName, exact: true }).click(); await tick();
            assert.equal(await controlled.getByRole('textbox', { name: `Controlled ${kind}`, exact: true }).inputValue(), '');
            const states = detail('Bounds, exclusions and field states');
            assert(await states.getByRole('textbox', { name: `Disabled ${kind}`, exact: true }).isDisabled());
            assert.equal(await states.getByRole('textbox', { name: `Required ${kind}`, exact: true }).getAttribute('aria-invalid'), 'true');
            const mode = detail('Multiple values and ranges');
            assert.deepEqual(await successful(mode, `${kind}_range`), [kind === 'month' ? '2026-11/2026-12' : '2026/2027']);
          }
        }
        await primary.screenshot({ path: outputPath(`input-remediation/${key}.png`) });
        report.pages.push({ key, status: 'pass' });
      }
    }
    assert.deepEqual(report.errors, []);
  } catch (error) { report.errors.push(error.stack || String(error)); }
  finally { await browser.close(); writeReport('input-remediation.json', report); }
  console.log(JSON.stringify(report, null, 2));
  if (report.errors.length) process.exitCode = 1;
})();
