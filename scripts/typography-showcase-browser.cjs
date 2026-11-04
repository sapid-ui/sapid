const assert = require('node:assert/strict');
const fs = require('node:fs');
const { chromium, base, outputPath, outputRoot, writeReport } = require('./showcase-browser-env.cjs');

(async () => {
  const browser = await chromium.launch({ headless: true });
  const report = { checks: [], errors: [] };
  try {
    const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
    page.on('pageerror', error => report.errors.push(String(error)));
    await page.goto(`${base}/showcase.html?widget=text&theme=blue&scheme=light`);
    await page.waitForFunction(() => window.ready === true
      && document.documentElement.getAttribute('data-sapid-widget') === 'text');

    const clamp = page.locator('.showcase-text-clamp');
    const clampStyle = await clamp.evaluate(element => ({
      lines: getComputedStyle(element).getPropertyValue('-webkit-line-clamp'),
      orient: getComputedStyle(element).getPropertyValue('-webkit-box-orient'),
      overflow: getComputedStyle(element).overflow,
      text: element.textContent,
      height: element.getBoundingClientRect().height,
      lineHeight: parseFloat(getComputedStyle(element).lineHeight),
    }));
    assert.equal(clampStyle.lines, '2');
    assert.equal(clampStyle.orient, 'vertical');
    assert.equal(clampStyle.overflow, 'hidden');
    assert.match(clampStyle.text, /complete value remains/);
    assert.ok(clampStyle.height <= clampStyle.lineHeight * 2 + 1);
    report.checks.push('Text line clamp limits two visual lines while retaining complete text');

    const gradient = await page.locator('.showcase-text-gradient').evaluate(element => ({
      image: getComputedStyle(element).backgroundImage,
      clip: getComputedStyle(element).backgroundClip,
      color: getComputedStyle(element).color,
    }));
    assert.match(gradient.image, /linear-gradient/);
    assert.equal(gradient.clip, 'text');
    assert.match(gradient.color, /rgba\(0, 0, 0, 0\)|color\(srgb 0 0 0 \/ 0\)/);
    report.checks.push('Gradient text uses the typed multi-declaration host lowering');

    await page.goto(`${base}/showcase.html?widget=title&theme=blue&scheme=light`);
    await page.waitForFunction(() => window.ready === true
      && document.documentElement.getAttribute('data-sapid-widget') === 'title');
    const title = page.locator('h2.showcase-title-clamp');
    assert.equal(await title.count(), 1);
    assert.equal(await title.evaluate(element => getComputedStyle(element).getPropertyValue('-webkit-line-clamp')), '2');
    assert.match(await title.textContent(), /complete semantic name/);
    report.checks.push('Title keeps its real heading level with the same line-clamp contract');
    assert.deepEqual(report.errors, []);
  } finally {
    await browser.close();
    writeReport('typography-showcase-browser.json', report);
  }
  console.log(JSON.stringify(report, null, 2));
})().catch(error => { console.error(error); process.exitCode = 1; });
