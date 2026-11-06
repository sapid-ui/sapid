// Focused public-page acceptance for the first five remediation pages.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const {matrix, chromium, base, outputPath, writeReport } = require('./showcase-browser-env.cjs');

(async () => {
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
  const report = { pages: [], errors: [] };
  page.on('pageerror', error => report.errors.push(error.message));
  page.on('console', message => { if (message.type() === 'error') report.errors.push(message.text()); });
  const detail = title => page.locator('.site-component-detail').filter({ has: page.getByRole('heading', { name: title, exact: true }) });
  const tick = () => page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))));
  fs.mkdirSync(outputPath('navigation-remediation'), { recursive: true });
  try {
    for (const theme of matrix.themes) for (const scheme of matrix.schemes) {
      for (const width of [1280, 390]) for (const slug of ['pagination', 'breadcrumbs', 'nav-link', 'anchor', 'burger']) {
        const key = `${slug}-${theme}-${scheme}-${width}`;
        await page.setViewportSize({ width, height: 1000 });
        await page.goto(`${base}/showcase.html?widget=${slug}&theme=${theme}&scheme=${scheme}`);
        await page.waitForFunction(slug => window.ready === true && document.documentElement.getAttribute('data-sapid-widget') === slug, slug);
        await tick();
        const primary = page.locator('.site-preview .site-demo').first();
        const geometry = await primary.evaluate(element => ({
          previewOverflow: element.scrollWidth - element.clientWidth,
          documentOverflow: document.documentElement.scrollWidth - innerWidth,
        }));
        assert(geometry.previewOverflow <= 1 && geometry.documentOverflow <= 1, `${key}: overflow ${JSON.stringify(geometry)}`);
        assert(!/ordered=|requests=|Toggle.*owner|Before action/i.test(await primary.innerText()), `${key}: clean public example`);
        if (slug === 'pagination') {
          const nav = primary.getByRole('navigation', { name: 'Example pages', exact: true });
          const labels = await nav.getByRole('button').evaluateAll(elements => elements.filter(element => element.getBoundingClientRect().height).map(element => element.getAttribute('aria-label')));
          assert(labels.indexOf('Previous page') < labels.indexOf('Page 3') && labels.indexOf('Page 3') < labels.indexOf('Next page'), key);
          assert(await nav.getByRole('button').evaluateAll(elements => elements.every(element => element.getBoundingClientRect().height <= 52)), `${key}: compact controls`);
          await nav.getByRole('button', { name: 'Next page', exact: true }).click(); await tick();
          assert.equal(await nav.locator('[aria-current=page]').getAttribute('aria-label'), 'Page 4');
          assert.equal(await nav.locator('[aria-current=page]').innerText(), '4');
          const numbers = page.getByRole('navigation', { name: 'Numbers only', exact: true });
          assert.equal(await numbers.getByRole('button', { name: 'Next page', exact: true }).count(), 0);
          assert.equal(await numbers.getByRole('button', { name: 'First page', exact: true }).count(), 0);
          const unknown = detail('Unknown totals').locator('.sapid-pagination');
          for (let i = 0; i < 3; i++) { await unknown.getByRole('button', { name: 'Next page', exact: true }).click(); await tick(); }
          assert(await unknown.getByRole('button', { name: 'Next page', exact: true }).isDisabled());
          assert(await unknown.getByRole('button', { name: 'Last page', exact: true }).isDisabled());
        }
        if (slug === 'breadcrumbs') {
          assert.equal(await primary.locator('.sapid-breadcrumb-separator').count(), 3);
          assert.equal(await detail('Separator and spacing').locator('.sapid-breadcrumb-separator').first().innerText(), '→');
          const path = detail('Long paths and controlled expansion');
          await path.getByRole('button', { name: 'Expand breadcrumb path', exact: true }).click(); await tick();
          assert.equal(await path.getByRole('button', { name: 'Collapse breadcrumb path', exact: true }).getAttribute('aria-expanded'), 'true');
          assert(await path.getByRole('link', { name: 'Docs', exact: true }).isVisible());
        }
        if (slug === 'nav-link') {
          assert.equal(await primary.locator('.sapid-nav-link').count(), 3);
          assert.equal(await primary.getByRole('link', { name: 'Overview', exact: true }).getAttribute('aria-current'), 'page');
          assert(await primary.getByText('Recent activity across your workspace', { exact: true }).isVisible());
          const nested = detail('Nested links and controlled expansion');
          const input = nested.getByRole('textbox', { name: 'Account note', exact: true });
          await input.fill('Retained account note');
          await input.evaluate(element => { window.remediationNote = element; });
          await nested.getByRole('button', { name: 'Collapse Account', exact: true }).click(); await tick();
          assert.equal(await input.isVisible(), false);
          await nested.getByRole('button', { name: 'Expand Account', exact: true }).click(); await tick();
          assert.equal(await input.inputValue(), 'Retained account note');
          assert(await input.evaluate(element => element === window.remediationNote), `${key}: retained editor identity`);
        }
        if (slug === 'anchor') {
          assert.equal(await primary.getByRole('link').count(), 3);
          const before = page.url();
          await detail('Application-owned navigation requests').getByRole('link', { name: 'Open project settings', exact: true }).click(); await tick();
          assert.equal(page.url(), before);
          assert(await page.getByText('Requested /projects/settings', { exact: true }).isVisible());
          assert.equal(await detail('Passive content and accessible names').getByRole('link', { name: 'Download report', exact: true }).count(), 1);
        }
        if (slug === 'burger') {
          const button = primary.getByRole('button', { name: 'Open navigation', exact: true });
          await button.evaluate(element => { window.remediationBurger = element; });
          await button.click(); await tick();
          const opened = primary.getByRole('button', { name: 'Close navigation', exact: true });
          assert.equal(await opened.getAttribute('aria-expanded'), 'true');
          assert(await opened.evaluate(element => element === window.remediationBurger), `${key}: stable disclosure`);
          assert.equal(await opened.locator('svg').count(), 1);
          await opened.press('Space'); await tick();
          assert.equal(await button.getAttribute('aria-expanded'), 'false');
          assert(await detail('Sizes, local state and disabled controls').locator('button').last().isDisabled());
        }
        await primary.screenshot({ path: outputPath(`navigation-remediation/${key}.png`), animations: 'disabled' });
        report.pages.push({ slug, theme, scheme, width, status: 'passed', ...geometry });
        writeReport('navigation-remediation.json', report);
        console.log(key);
      }
    }
    assert.deepEqual(report.errors, []);
  } finally {
    await browser.close(); writeReport('navigation-remediation.json', report);
  }
})().catch(error => { console.error(error); process.exitCode = 1; });
