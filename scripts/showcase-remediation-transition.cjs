const assert = require('node:assert/strict');
const fs = require('node:fs');
const {matrix, chromium, base, outputPath, writeReport } = require('./showcase-browser-env.cjs');
(async () => {
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
  const report = { browser: browser.version(), pages: [], errors: [] };
  page.on('pageerror', error => report.errors.push(error.message));
  page.on('console', message => { if (message.type() === 'error') report.errors.push(message.text()); });
  const detail = title => page.locator('.site-component-detail').filter({ has: page.getByRole('heading', { name: title, exact: true }) });
  const until = async (locator, predicate, message) => {
    const deadline = Date.now() + 4000;
    do { if (await locator.evaluate(predicate)) return; await page.evaluate(() => new Promise(requestAnimationFrame)); } while (Date.now() < deadline);
    throw new Error(message);
  };
  const hidden = locator => until(locator, e => getComputedStyle(e).display === 'none', 'exit did not settle');
  const shown = locator => until(locator, e => getComputedStyle(e).display !== 'none' && Number(getComputedStyle(e).opacity) === 1 && !e.hasAttribute('data-gly-hidden') && e.getAnimations().length === 0, 'entry did not settle');
  fs.mkdirSync(outputPath('transition-remediation'), { recursive: true });
  try {
    for (const theme of matrix.themes) for (const scheme of matrix.schemes) for (const width of [1280, 390]) {
      const key = `${theme}-${scheme}-${width}`;
      await page.emulateMedia({ reducedMotion: 'no-preference' });
      await page.setViewportSize({ width, height: 1000 });
      await page.goto(`${base}/showcase.html?widget=transition&theme=${theme}&scheme=${scheme}`);
      await page.waitForFunction(() => window.ready === true && document.documentElement.getAttribute('data-sapid-widget') === 'transition');
      assert.equal(await page.locator('.site-component-detail').count(), 5);
      assert(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth + 1));
      const primary = page.locator('.site-preview .site-demo').first();
      const shell = primary.locator('.sapid-transition');
      const editor = primary.getByRole('textbox', { name: 'Transition draft', exact: true });
      await editor.fill('Retained actual browser draft');
      await editor.evaluate(e => { window.transitionEditor = e; });
      assert(await primary.getByText('Ready', { exact: true }).isVisible());
      await primary.getByRole('button', { name: 'Toggle transition', exact: true }).click();
      await until(shell, e => { const opacity = Number(getComputedStyle(e).opacity); return opacity > 0.05 && opacity < 0.95; }, 'no intermediate exit opacity');
      assert(await shell.evaluate(e => e.hasAttribute('inert') && e.getAttribute('aria-hidden') === 'true'));
      await primary.screenshot({ path: outputPath(`transition-remediation/${key}-exiting.png`) });
      await hidden(shell); await primary.getByText('Exited', { exact: true }).waitFor();
      await primary.getByRole('button', { name: 'Toggle transition', exact: true }).click();
      await shown(shell); await primary.getByText('Entered', { exact: true }).waitFor();
      assert(await editor.evaluate(e => e === window.transitionEditor)); assert.equal(await editor.inputValue(), 'Retained actual browser draft');
      await primary.screenshot({ path: outputPath(`transition-remediation/${key}.png`) });

      const effects = detail('Effects and timing'); const effectShell = effects.locator('.sapid-transition');
      const toggle = effects.getByRole('button', { name: 'Toggle effect example', exact: true });
      await effects.getByRole('button', { name: 'Scale', exact: true }).click();
      await toggle.click(); await hidden(effectShell); await toggle.click();
      await until(effectShell, e => { const matrix = new DOMMatrixReadOnly(getComputedStyle(e).transform); return matrix.a > .85 && matrix.a < .995; }, 'scale effect has no real transform');
      assert.equal(await effectShell.evaluate(e => getComputedStyle(e).transitionTimingFunction.split(',')[0].trim()), 'ease-out');
      await shown(effectShell);
      await effects.getByRole('button', { name: 'Slide left', exact: true }).click();
      await toggle.click(); await hidden(effectShell); await toggle.click();
      await until(effectShell, e => { const matrix = new DOMMatrixReadOnly(getComputedStyle(e).transform); return matrix.e > .1 && matrix.e < 24; }, 'slide effect has no real translation');
      await shown(effectShell);

      const custom = detail('Custom closed styles and transform origin'); const customShell = custom.locator('.sapid-transition');
      await custom.getByRole('button', { name: 'Toggle custom transition', exact: true }).click(); await hidden(customShell);
      await custom.getByRole('button', { name: 'Toggle custom transition', exact: true }).click();
      await until(customShell, e => { const m = new DOMMatrixReadOnly(getComputedStyle(e).transform); return m.b < -.005 && m.a > .8 && m.a < .995; }, 'custom scale and rotation have no intermediate transform');
      assert.equal(await customShell.evaluate(e => getComputedStyle(e).transformOrigin.split(' ')[1]), '0px');
      await shown(customShell);

      const lifecycle = detail('Delays, completion and interruption'); const delayed = lifecycle.locator('.sapid-transition');
      await lifecycle.getByRole('button', { name: 'Toggle delayed transition', exact: true }).click();
      await lifecycle.getByText('enter', { exact: true }).waitFor();
      assert(await delayed.evaluate(e => e.getAnimations().some(a => a.effect.getTiming().delay === 150)));
      await shown(delayed); await lifecycle.getByText('enter → entered', { exact: true }).waitFor();
      await lifecycle.getByRole('button', { name: 'Toggle delayed transition', exact: true }).click();
      await until(delayed, e => { const opacity = Number(getComputedStyle(e).opacity); return opacity > .1 && opacity < .9; }, 'no delayed exit frame');
      await lifecycle.getByRole('button', { name: 'Toggle delayed transition', exact: true }).click();
      await shown(delayed); await lifecycle.getByText('enter → entered → exit → enter → entered', { exact: true }).waitFor();

      const policies = detail('Keep mounted and unmount policies');
      const kept = policies.getByRole('textbox', { name: 'Kept editor', exact: true }); const unmounted = policies.getByRole('textbox', { name: 'Unmounted editor', exact: true });
      await kept.fill('Saved draft'); await unmounted.fill('Discarded draft');
      await kept.evaluate(e => { window.keptTransition = e; }); await unmounted.evaluate(e => { window.unmountedTransition = e; });
      await policies.getByRole('button', { name: 'Toggle both editors', exact: true }).click();
      await until(policies.locator('.sapid-transition').last(), e => e.children.length === 0, 'unmount did not release content after exit');
      assert(await page.evaluate(() => window.keptTransition.isConnected && !window.unmountedTransition.isConnected));
      await policies.getByRole('button', { name: 'Toggle both editors', exact: true }).click(); await shown(policies.locator('.sapid-transition').last());
      assert.equal(await kept.inputValue(), 'Saved draft'); assert(await kept.evaluate(e => e === window.keptTransition));
      assert.equal(await unmounted.inputValue(), 'Fresh initial value'); assert(await unmounted.evaluate(e => e !== window.unmountedTransition));

      const reduced = detail('Reduced motion, SSR and native support'); const reducedShell = reduced.locator('.sapid-transition');
      await reduced.getByRole('button', { name: 'Toggle reduced motion example', exact: true }).click(); await shown(reducedShell);
      assert.equal(await reducedShell.evaluate(e => e.getAnimations().length), 0);
      await reduced.getByRole('checkbox', { name: 'Enable requested animation', exact: true }).check();
      await page.emulateMedia({ reducedMotion: 'reduce' });
      await reduced.getByRole('button', { name: 'Toggle reduced motion example', exact: true }).click(); await hidden(reducedShell);
      await reduced.getByRole('button', { name: 'Toggle reduced motion example', exact: true }).click(); await shown(reducedShell);
      assert.equal(await reducedShell.evaluate(e => e.getAnimations().length), 0);
      report.pages.push({ key, passed: true }); console.log(`PASS ${key}`);
    }
    assert.deepEqual(report.errors, []);
  } catch (error) { report.failure = error.stack; await page.screenshot({ path: outputPath('transition-remediation/failure.png'), fullPage: true }); throw error; }
  finally { writeReport('transition-remediation.json', report); await browser.close(); }
})().catch(error => { console.error(error); process.exitCode = 1; });
