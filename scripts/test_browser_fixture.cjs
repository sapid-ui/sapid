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
