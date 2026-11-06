const test = require('node:test');
const assert = require('node:assert/strict');
const crypto = require('node:crypto');
const http = require('node:http');
const { normalizeBase, sourceAtBase, verifyServedArtifacts } = require('./browser.cjs');

test('an alternate owned fixture port changes only the origin, preserving paths and assertions', () => {
  const source = "await page.goto('http://127.0.0.1:8794/example.html?hydrate'); assert.equal(value, 4);";
  assert.equal(sourceAtBase(source, 'http://127.0.0.1:8914/'),
    "await page.goto('http://127.0.0.1:8914/example.html?hydrate'); assert.equal(value, 4);");
  assert.equal(normalizeBase(), 'http://127.0.0.1:8794');
  for (const value of ['https://example.com', 'http://127.0.0.1:8794/path',
    'http://user:secret@127.0.0.1', 'http://127.0.0.1/?other']) assert.throws(() => normalizeBase(value));
});

test('a stale service cannot produce acceptance for different files on disk', async () => {
  let current = Buffer.from('current fixture');
  const server = http.createServer((req, res) => req.url === '/asset.wasm'
    ? res.end(current) : res.writeHead(404).end());
  await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
  try {
    const base = `http://127.0.0.1:${server.address().port}`;
    const hashes = { 'asset.wasm': crypto.createHash('sha256').update(current).digest('hex') };
    await verifyServedArtifacts(base, hashes);
    current = Buffer.from('old fixture');
    await assert.rejects(verifyServedArtifacts(base, hashes), /differs from built artifact/);
    await assert.rejects(verifyServedArtifacts(base, { 'missing.wasm': hashes['asset.wasm'] }), /HTTP 404/);
  } finally { server.closeAllConnections(); await new Promise(resolve => server.close(resolve)); }
});

const { runSuiteWithInfrastructureRetry } = require('./browser.cjs');
test('socket exhaustion reruns every assertion in a fresh browser and retains failed evidence', async () => {
  const first = { file: 'fixture.js', reports: [], errors: ['page.waitForFunction: Timeout 30000ms exceeded'],
    diagnostics: [{ type: 'requestfailed', message: 'net::ERR_NO_BUFFER_SPACE' }] };
  const passed = { file: 'fixture.js', reports: [{ mode: 'all assertions passed' }], errors: [], diagnostics: [] };
  let runs = 0, restarts = 0;
  const result = await runSuiteWithInfrastructureRetry(async () => ++runs === 1 ? first : passed, async () => { restarts++; });
  assert.equal(runs, 2); assert.equal(restarts, 1);
  assert.deepEqual(result.reports, passed.reports); assert.deepEqual(result.errors, []);
  assert.deepEqual(result.infrastructureAttempts, [first]);
});
test('assertion failures and generic timeouts never get infrastructure retries', async () => {
  for (const diagnostics of [[], [{ type: 'requestfailed', message: 'net::ERR_CONNECTION_REFUSED' }], [{ type: 'requestfailed', message: 'net::ERR_NO_BUFFER_SPACE' }]]) {
    let runs = 0;
    const failed = { errors: ['assertion failed'], reports: [], diagnostics };
    assert.deepEqual(await runSuiteWithInfrastructureRetry(async () => { runs++; return failed; },
      async () => assert.fail('must not restart')), failed);
    assert.equal(runs, 1);
  }
});
test('repeated socket exhaustion stays a failure after one bounded retry', async () => {
  const failed = { file: 'fixture.js', errors: ['page.waitForFunction: Timeout 30000ms exceeded'], reports: [],
    diagnostics: [{ type: 'requestfailed', message: 'net::ERR_NO_BUFFER_SPACE' }] };
  let runs = 0;
  const result = await runSuiteWithInfrastructureRetry(async () => { runs++; return failed; }, async () => {});
  assert.equal(runs, 2); assert.deepEqual(result.errors, failed.errors);
  assert.deepEqual(result.infrastructureAttempts, [failed]);
});
