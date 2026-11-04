const assert = require('node:assert/strict');
const fs = require('node:fs');
const { chromium, base, outputPath, outputRoot, writeReport } = require('./showcase-browser-env.cjs');

(async () => {
  const browser = await chromium.launch({ headless: true });
  const report = { checks: [], errors: [] };
  try {
    const page = await browser.newPage({ viewport: { width: 1280, height: 1200 } });
    page.on('pageerror', error => report.errors.push(String(error)));
    page.on('console', message => {
      if (message.type() === 'error') report.errors.push(message.text());
    });
    await page.goto(`${base}/showcase.html?widget=image&theme=blue&scheme=light`);
    await page.waitForFunction(() => window.ready === true
      && document.documentElement.getAttribute('data-sapid-widget') === 'image');

    const text = await page.locator('main').innerText();
    for (const heading of ['Usage', 'Image height', 'Image fit', 'Fallback image', 'Responsive source and loading']) {
      assert.match(text, new RegExp(heading));
    }
    assert.equal(await page.locator('.showcase-image-usage').count(), 1);
    assert.equal(await page.locator('.showcase-image-height').count(), 1);
    assert.equal(await page.locator('.showcase-image-fit').count(), 1);
    report.checks.push('Image page exposes the documented usage, height, fit and fallback examples');

    const height = await page.locator('.showcase-image-height').evaluate(element => element.getBoundingClientRect().height);
    const fit = await page.locator('.showcase-image-fit').evaluate(element => getComputedStyle(element).objectFit);
    assert.ok(Math.abs(height - 160) < 0.6, `fixed image height was ${height}`);
    assert.equal(fit, 'contain');

    const fallback = page.locator('.showcase-image-fallback');
    await fallback.waitFor({ state: 'visible' });
    await page.waitForFunction(() => document.querySelector('.showcase-image-fallback')?.getAttribute('src')?.includes('image-fallback.png'));
    assert.equal(await fallback.evaluate(element => element.complete && element.naturalWidth > 0), true);
    report.checks.push('Fallback remains one real versioned image request');

    const responsive = page.locator('.showcase-image-responsive');
    const attrs = await responsive.evaluate(element => ({
      tag: element.tagName,
      srcset: element.getAttribute('srcset'),
      sizes: element.getAttribute('sizes'),
      loading: element.getAttribute('loading'),
      currentSrc: element.currentSrc,
      complete: element.complete,
      width: element.getBoundingClientRect().width,
    }));
    assert.equal(attrs.tag, 'IMG');
    assert.match(attrs.srcset, /image-pattern\.png 320w, .*image-fallback\.png 960w/);
    assert.equal(attrs.sizes, '(max-width: 600px) 100vw, 480px');
    assert.equal(attrs.loading, 'lazy');
    assert.equal(attrs.complete, true);
    assert.match(attrs.currentSrc, /image-(pattern|fallback)\.png/);
    assert.ok(attrs.width <= 480.6);
    report.checks.push('Responsive candidates, sizes and lazy loading reach the real img host element');

    await page.goto(`${base}/showcase.html?widget=background-image&theme=blue&scheme=light`);
    await page.waitForFunction(() => window.ready === true
      && document.documentElement.getAttribute('data-sapid-widget') === 'background-image');
    const background = page.locator('.showcase-background-image');
    assert.equal(await background.count(), 1);
    assert.match(await page.locator('main').innerText(), /Usage/);
    assert.equal(await background.locator('img').getAttribute('aria-hidden'), 'true');
    assert.match(await background.innerText(), /Content over a background image/);
    report.checks.push('BackgroundImage keeps decorative media behind independent content');
    assert.deepEqual(report.errors, []);
  } finally {
    await browser.close();
    writeReport('image-showcase-browser.json', report);
  }
  console.log(JSON.stringify(report, null, 2));
})().catch(error => { console.error(error); process.exitCode = 1; });
