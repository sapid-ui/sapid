const assert = require('node:assert/strict');
const fs = require('node:fs');
const { chromium, base, outputPath, outputRoot, writeReport } = require('./showcase-browser-env.cjs');
(async () => {
  const browser = await chromium.launch({ headless: true, ...(process.env.GLORY_CHROME ? { executablePath: process.env.GLORY_CHROME } : {}) });
  const report = { checks: [], errors: [] };
  const ready = (page, widget, palette, scheme) => page.waitForFunction(({ widget, palette, scheme }) => window.ready &&
    document.documentElement.getAttribute('data-sapid-widget') === widget &&
    document.documentElement.getAttribute('data-sapid-palette') === palette &&
    document.documentElement.getAttribute('data-sapid-scheme') === scheme, { widget, palette, scheme });
  try {
    const page = await browser.newPage();
    page.on('pageerror', error => report.errors.push(error.message));
    await page.route('**/sapid_web_example.js', async route => {
      await page.evaluate(() => history.replaceState({ otherApplication: 'retained' }, '', location.href));
      await route.continue();
    });
    await page.goto(base + '/showcase.html?widget=button&theme=blue&scheme=dark');
    await ready(page, 'button', 'blue', 'dark');
    const firstKey = await page.evaluate(() => history.state.__glory_routing_v1);
    assert.equal(typeof firstKey, 'string');
    await page.locator('.site-demo').getByRole('button', { name: 'Filled button', exact: true }).click();
    await page.getByText('Clicked 1 times', { exact: true }).waitFor();
    await page.evaluate(() => { window.retainedDemo = document.querySelector('.site-demo'); });
    await page.getByRole('navigation', { name: 'On this page' }).getByRole('link', { name: 'Composition', exact: true }).click();
    await page.waitForURL(/#composition$/);
    const secondKey = await page.evaluate(() => history.state.__glory_routing_v1);
    assert.notEqual(secondKey, firstKey);
    assert.equal(await page.evaluate(() => history.state.otherApplication), 'retained');
    await page.goBack(); await ready(page, 'button', 'blue', 'dark');
    assert.equal(await page.evaluate(() => history.state.__glory_routing_v1), firstKey);
    assert(await page.evaluate(() => window.retainedDemo === document.querySelector('.site-demo')));
    await page.getByText('Clicked 1 times', { exact: true }).waitFor();
    report.checks.push('Glory history keys retain foreign state and fragment traversal keeps the live demo');

    const namedRoute = await browser.newPage();
    namedRoute.on('pageerror', error => report.errors.push(error.message));
    await namedRoute.goto(base + '/showcase.html?widget=range-slider&theme=blue&scheme=light#range-domain');
    await ready(namedRoute, 'range-slider', 'blue', 'light');
    assert.match(namedRoute.url(), /widget=range-slider&theme=blue&scheme=light#range-domain$/);
    assert.equal(await namedRoute.locator('a[href*="widget=range-slider"]').count() > 0, true);
    const publicTargets = await namedRoute.locator('a[href*="widget="]').evaluateAll(links => links.map(link => new URL(link.href).searchParams.get('widget')));
    assert.equal(publicTargets.some(widget => /^P\d+-\d+/.test(widget || '')), false);
    report.checks.push('Public widget routes and navigation links use component-name slugs only');
    await namedRoute.close();

    const rejectedInternal = await browser.newPage();
    rejectedInternal.on('pageerror', error => report.errors.push(error.message));
    const internalTask = ['P7', '03'].join('-');
    await rejectedInternal.goto(`${base}/showcase.html?widget=${internalTask}&theme=blue&scheme=light`);
    await ready(rejectedInternal, internalTask, 'blue', 'light');
    await rejectedInternal.getByRole('heading', { name: 'Widget not found', exact: true }).waitFor();
    assert.equal(new URL(rejectedInternal.url()).searchParams.get('widget'), internalTask);
    assert.equal(await rejectedInternal.getByRole('heading', { name: 'RangeSlider', exact: true }).count(), 0);
    report.checks.push('Internal audit task keys are rejected instead of being treated as routing aliases');
    await rejectedInternal.close();

    const late = await browser.newPage();
    late.on('pageerror', error => report.errors.push(error.message));
    await late.route('**/sapid_web_example.js', async route => {
      assert.equal(await late.locator('.site-demo').count(), 1);
      await late.evaluate(() => history.replaceState({ foreign: 'late' }, '', '/showcase.html?widget=text-input&theme=green&scheme=light'));
      await route.continue();
    });
    await late.goto(base + '/ssr-button-blue-dark.html');
    await ready(late, 'text-input', 'green', 'light');
    assert.equal(await late.evaluate(() => history.state.foreign), 'late');
    report.checks.push('SSR alias adopts its bootstrap before a changed pre-WASM URL selects a different widget/theme');

    assert.deepEqual(report.errors, []);
  } finally {
    await browser.close();
    writeReport('showcase-routing-browser.json', report);
  }
  console.log(JSON.stringify(report, null, 2));
})().catch(error => { console.error(error); process.exitCode = 1; });
