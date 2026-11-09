const { test } = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const { spawnSync } = require('node:child_process');
const { cases, select, metrics, comparable } = require('./run.cjs');
test('matrix includes real and unused cases for every optional family', () => {
  assert.equal(cases.length, 17);
  assert.equal(new Set(cases.map(value => value.name)).size, cases.length);
  for (const family of ['charts', 'code', 'files', 'editor', 'json', 'dates']) {
    assert.deepEqual(select(`${family}-used`)[0].features, [`use-${family}`]);
    assert.deepEqual(select(`${family}-unused`)[0].features, [family]);
  }
  assert.throws(() => select('not-a-case'), /Unknown case/);
});
test('compression and source hash cover the actual served bytes', () => {
  const directory = fs.mkdtempSync(path.join(os.tmpdir(), 'sapid-size-test-'));
  const file = path.join(directory, 'asset');
  try {
    fs.writeFileSync(file, 'abc'.repeat(1000));
    const first = metrics(file);
    assert.equal(first.raw, 3000);
    assert(first.brotli < first.raw && first.gzip < first.raw);
    fs.appendFileSync(file, '!');
    assert.notEqual(metrics(file).sha256, first.sha256);
  } finally { fs.rmSync(file); fs.rmdirSync(directory); }
});
test('comparison rejects partial, changed-toolchain, changed-lock and missing-case evidence', () => {
  const metric = { raw: 100, brotli: 50, gzip: 60 };
  const report = { schema: 1, complete: true, versions: { rustc: 'same' }, profile: {}, target: 'same', lockSha256: 'same',
    cases: [{ name: 'empty', features: [], wasm: metric, javascript: metric }] };
  comparable(report, report);
  for (const patch of [{ complete: false }, { versions: {} }, { lockSha256: 'changed' }, { cases: [] }]) {
    assert.throws(() => comparable({ ...report, ...patch }, report));
  }
});
test('browser preflight failures replace stale success evidence before loading Playwright', () => {
  const directory = fs.mkdtempSync(path.join(os.tmpdir(), 'sapid-size-preflight-'));
  try {
    fs.writeFileSync(path.join(directory, 'report.json'), JSON.stringify({ complete: false, cases: [] }));
    fs.writeFileSync(path.join(directory, 'browser.json'), JSON.stringify({ complete: true, cases: [{ passed: true }] }));
    const result = spawnSync(process.execPath, [path.join(__dirname, 'browser.cjs'), directory],
      { encoding: 'utf8', env: { ...process.env, SAPID_PLAYWRIGHT: path.join(directory, 'absent-module') } });
    assert.notEqual(result.status, 0);
    const evidence = JSON.parse(fs.readFileSync(path.join(directory, 'browser.json')));
    assert.equal(evidence.complete, false);
    assert.deepEqual(evidence.cases, []);
    assert.match(evidence.error, /complete size run/);
  } finally {
    for (const file of ['report.json', 'browser.json']) fs.rmSync(path.join(directory, file));
    fs.rmdirSync(directory);
  }
});
