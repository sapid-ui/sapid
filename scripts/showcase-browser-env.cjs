// Shared, environment-driven paths for browser acceptance scripts.
const fs = require('node:fs');
const path = require('node:path');

const buildRootValue = process.env.SAPID_BUILD_ROOT;
if (!buildRootValue) {
  throw new Error('SAPID_BUILD_ROOT is required');
}
const buildRoot = path.resolve(buildRootValue);
const outputRoot = path.resolve(process.env.SAPID_BROWSER_OUTPUT || buildRoot);
const tempDir = path.resolve(process.env.SAPID_BROWSER_TEMP || path.join(buildRoot, 'tmp'));
fs.mkdirSync(outputRoot, { recursive: true });
fs.mkdirSync(tempDir, { recursive: true });
process.env.TEMP = process.env.TMP = tempDir;

const playwrightModule = process.env.SAPID_PLAYWRIGHT || 'playwright';
const { chromium } = require(playwrightModule);
const base = process.env.SAPID_SHOWCASE_BASE || 'http://127.0.0.1:8795';

function outputPath(name) {
  const resolved = path.resolve(outputRoot, name);
  const relative = path.relative(outputRoot, resolved);
  if (relative.startsWith('..') || path.isAbsolute(relative)) {
    throw new Error(`Browser artifact must stay under ${outputRoot}: ${resolved}`);
  }
  return resolved;
}

function writeReport(name, report) {
  fs.writeFileSync(outputPath(name), JSON.stringify(report, null, 2));
}

module.exports = { base, buildRoot, chromium, outputPath, outputRoot, tempDir, writeReport };
