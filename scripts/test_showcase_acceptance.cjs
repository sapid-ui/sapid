const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { outputPath } = require('./showcase-browser-env.cjs');
const { serve, runAcceptance, showcaseSuites } = require('./showcase-acceptance.cjs');
const temporary = outputPath('tmp');
fs.mkdirSync(temporary, { recursive: true });

test('the release gate includes every current documentation acceptance batch exactly once', () => {
  const root = path.resolve(__dirname, '..');
  const suites = showcaseSuites(root, temporary);
  assert.equal(new Set(suites.map(suite => suite.id)).size, suites.length);
  const expected = fs.readdirSync(__dirname).filter(name => /^showcase-remediation-.*\.cjs$/.test(name)
    && name !== 'showcase-remediation-p1.cjs'); // P1 is an aggregate of nine included batches.
  for (const file of expected) {
    const matches = suites.filter(suite => suite.source === path.join(__dirname, file));
    assert.equal(matches.length, 1, file);
    assert(matches[0].report, `${file} must preserve its structured report`);
  }
  for (const id of ['showcase', 'theme-studio', 'palette-order', 'theme-export', 'navigation', 'routing', 'cache']) {
    assert(suites.some(suite => suite.id === id), `missing site contract: ${id}`);
  }
});

test('quick coverage retains every behavior suite and skips only the redundant all-page sweep', () => {
  const root = path.resolve(__dirname, '..');
  const full = showcaseSuites(root, temporary, 'full'), quick = showcaseSuites(root, temporary, 'quick');
  assert.deepEqual(quick.map(suite => suite.id), full.filter(suite => suite.id !== 'all-pages-docs').map(suite => suite.id));
  assert.equal(quick.find(suite => suite.id === 'theme-export').after, 'theme-studio');
  for (const id of ['showcase', 'theme-studio', 'theme-export', 'buttons-docs', 'combobox-docs']) {
    assert.equal(quick.find(suite => suite.id === id).exclusive, true);
  }
  assert.throws(() => showcaseSuites(root, temporary, 'invalid'), /Unknown showcase mode/);
});

test('quick and full subprocesses select the intended palette axes without changing responsive coverage', () => {
  const { spawnSync } = require('node:child_process');
  for (const mode of ['quick', 'full']) {
    const run = spawnSync(process.execPath, ['-e', "console.log(JSON.stringify(require('./scripts/showcase-browser-env.cjs').matrix))"], {
      cwd: path.resolve(__dirname, '..'), encoding: 'utf8', windowsHide: true,
      env: { ...process.env, SAPID_BUILD_ROOT: temporary, SAPID_SHOWCASE_MODE: mode },
    });
    assert.equal(run.status, 0, run.stderr);
    assert.deepEqual(JSON.parse(run.stdout), mode === 'quick'
      ? { themes: ['blue'], schemes: ['light'] }
      : { themes: ['blue', 'green'], schemes: ['light', 'dark'] });
  }
});

function fixture(t) {
  const root = fs.mkdtempSync(path.join(temporary, 'showcase-runner-'));
  t.after(() => fs.rmSync(root, { recursive: true, force: true }));
  const directory = path.join(root, 'site'); fs.mkdirSync(directory);
  for (const name of ['index.html', 'showcase.html']) fs.writeFileSync(path.join(directory, name), '<!doctype html><title>Fixture</title>');
  fs.writeFileSync(path.join(directory, 'asset-manifest.json'), '{}');
  return { root, directory, output: path.join(root, 'results') };
}
function suite(root, name, source, report) {
  const file = path.join(root, `${name}.cjs`); fs.writeFileSync(file, source);
  return { id: name, source: file, program: process.execPath, args: [file], report };
}

test('two workers overlap independent suites, then honor exclusive clipboard ownership and export dependencies', async t => {
  const options = fixture(t), events = path.join(options.root, 'events.jsonl');
  const source = name => `
    const fs = require('node:fs');
    const record = kind => fs.appendFileSync(${JSON.stringify(events)}, JSON.stringify({name:${JSON.stringify(name)},kind})+'\\n');
    const wait = async predicate => { const end = Date.now()+5000; while(!predicate()) {
      if(Date.now()>end) throw Error('parallel handshake timed out'); await new Promise(r=>setTimeout(r,10));
    }};
    const events = () => fs.existsSync(${JSON.stringify(events)}) ? fs.readFileSync(${JSON.stringify(events)},'utf8') : '';
    (async()=>{record('start');
      ${['a', 'b'].includes(name) ? "await wait(()=>['a','b'].every(name=>events().includes(JSON.stringify({name,kind:'start'}))));" : ''}
      if(process.env.SAPID_SHOWCASE_MODE!=='quick') throw Error('mode not forwarded');
      record('end');
    })().catch(e=>{console.error(e);process.exitCode=1});
  `;
  options.jobs = 2; options.mode = 'quick';
  options.suites = [suite(options.root, 'a', source('a')), suite(options.root, 'b', source('b')),
    { ...suite(options.root, 'producer', source('producer')), exclusive: true },
    { ...suite(options.root, 'consumer', source('consumer')), after: 'producer' }];
  const result = await runAcceptance(options);
  assert.equal(result.complete, true, result.errors.join('\n'));
  const entries = fs.readFileSync(events, 'utf8').trim().split('\n').map(line => JSON.parse(line));
  const at = (name, kind) => entries.findIndex(event => event.name === name && event.kind === kind);
  assert(at('a', 'start') < at('b', 'end') && at('b', 'start') < at('a', 'end'));
  assert(at('producer', 'start') > at('a', 'end') && at('producer', 'start') > at('b', 'end'));
  assert(at('consumer', 'start') > at('producer', 'end'));
  assert(result.suites.every(entry => Number.isFinite(entry.durationMs)));
});

test('parallel failure drains active work, leaves queued suites pending and cannot report success', async t => {
  const options = fixture(t), settled = path.join(options.root, 'settled');
  options.jobs = 2;
  options.suites = [suite(options.root, 'reject', 'throw Error("reject parallel run");'),
    suite(options.root, 'active', `setTimeout(()=>require('node:fs').writeFileSync(${JSON.stringify(settled)},'done'),300);`),
    suite(options.root, 'pending', 'throw Error("must stay pending");')];
  const result = await runAcceptance(options);
  assert.equal(result.complete, false);
  assert.deepEqual(result.suites.map(entry => entry.status), ['failed', 'passed', 'pending']);
  assert.equal(fs.readFileSync(settled, 'utf8'), 'done');
});

test('invalid concurrency and unresolved producer dependencies cannot become successful evidence', async t => {
  for (const jobs of [0, 5, 'invalid']) await assert.rejects(runAcceptance({ jobs }), /--jobs/);
  const options = fixture(t);
  options.suites = [{ ...suite(options.root, 'consumer', ''), after: 'missing-producer' }];
  const result = await runAcceptance(options);
  assert.equal(result.complete, false);
  assert.equal(result.suites[0].status, 'pending');
  assert.match(result.errors.join('\n'), /Unresolved showcase suite dependencies/);
});

test('changing the shared matrix helper during acceptance invalidates otherwise passing results', async t => {
  const options = fixture(t), helper = path.join(options.root, 'scripts/showcase-browser-env.cjs');
  fs.mkdirSync(path.dirname(helper)); fs.writeFileSync(helper, 'original matrix policy');
  options.suites = [suite(options.root, 'change-policy', `require('node:fs').writeFileSync(${JSON.stringify(helper)},'changed policy');`)];
  const result = await runAcceptance(options);
  assert.equal(result.complete, false);
  assert.match(result.errors.join('\n'), /Acceptance source changed: scripts[\\/]showcase-browser-env.cjs/);
});

test('a real failed HTTP assertion stops later suites and records the full pending manifest', async t => {
  const options = fixture(t), marker = path.join(options.root, 'unexpected');
  options.suites = [suite(options.root, 'missing-asset', `
    const assert = require('node:assert/strict');
    fetch(process.env.SAPID_SHOWCASE_BASE + '/missing.wasm').then(response => assert.equal(response.status, 200));
  `), suite(options.root, 'must-not-run', `require('node:fs').writeFileSync(${JSON.stringify(marker)}, 'ran');`)];
  const result = await runAcceptance(options);
  assert.equal(result.complete, false);
  assert.deepEqual(result.suites.map(entry => entry.status), ['failed', 'pending']);
  assert.deepEqual(result.suiteManifest, ['missing-asset', 'must-not-run']);
  assert.match(result.errors.join('\n'), /404/);
  assert.equal(fs.existsSync(marker), false);
});

test('JSON-reported browser errors fail even if a legacy driver exits zero; stale success is removed', async t => {
  const options = fixture(t), report = path.join(options.root, 'driver.json');
  fs.writeFileSync(report, JSON.stringify({ errors: [] }));
  options.suites = [suite(options.root, 'caught-assertion', `
    const fs = require('node:fs'), assert = require('node:assert/strict');
    const errors = []; try { assert.equal(1, 2, 'actual failed assertion'); } catch (error) { errors.push(error.message); }
    fs.writeFileSync(${JSON.stringify(report)}, JSON.stringify({ errors }));
  `, report)];
  const result = await runAcceptance(options);
  assert.equal(result.complete, false);
  assert.match(result.errors.join('\n'), /actual failed assertion/);
  options.suites = [suite(options.root, 'no-report', '', report)];
  const absent = await runAcceptance(options);
  assert.equal(absent.complete, false);
  assert.match(absent.errors.join('\n'), /ENOENT/);
});

test('a hung child is bounded and cannot produce passing evidence', async t => {
  const options = fixture(t);
  options.timeout = 200;
  options.suites = [suite(options.root, 'hung-driver', 'setInterval(() => {}, 1000);')];
  const result = await runAcceptance(options);
  assert.equal(result.complete, false);
  assert.equal(result.suites[0].status, 'failed');
});

test('a caught assertion in a documentation failure field cannot become passing evidence', async t => {
  const options = fixture(t), report = path.join(options.root, 'driver.json');
  options.suites = [suite(options.root, 'caught-failure', `
    require('node:fs').writeFileSync(${JSON.stringify(report)}, JSON.stringify({ errors: [], failure: 'assertion failed' }));
  `, report)];
  const result = await runAcceptance(options);
  assert.equal(result.complete, false);
  assert.match(result.errors.join('\n'), /assertion failed/);
});

test('the in-process server rejects traversal and follows no directory junction outside the site', async t => {
  const options = fixture(t), outside = path.join(options.root, 'private');
  fs.mkdirSync(outside); fs.writeFileSync(path.join(outside, 'private.txt'), 'private');
  fs.symlinkSync(outside, path.join(options.directory, 'linked'), process.platform === 'win32' ? 'junction' : 'dir');
  const server = await serve(options.directory);
  try {
    assert.equal((await fetch(`${server.base}/`)).status, 200);
    assert.equal((await fetch(`${server.base}/missing.js`)).status, 404);
    assert.equal((await fetch(`${server.base}/%2e%2e%2fprivate/private.txt`)).status, 403);
    assert.equal((await fetch(`${server.base}/linked/private.txt`)).status, 403);
  } finally { await server.close(); }
});
