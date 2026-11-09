'use strict';
const fs = require('node:fs');
const path = require('node:path');
const cp = require('node:child_process');
const crypto = require('node:crypto');
const zlib = require('node:zlib');
const fixture = __dirname;
const repository = path.resolve(fixture, '../..');
const families = ['charts', 'code', 'files', 'editor', 'json', 'dates'];
const cases = [
  { name: 'empty', features: [] },
  { name: 'glory-text', features: ['base'] },
  { name: 'sapid-unused', features: ['sapid-base'] },
  { name: 'sapid-button', features: ['button'] },
  ...families.flatMap(name => [
    { name: `${name}-unused`, features: [name] },
    { name: `${name}-used`, features: [`use-${name}`] },
  ]),
  { name: 'all-unused', features: ['all-unused'] },
];
const hash = bytes => crypto.createHash('sha256').update(bytes).digest('hex');
function execute(exe, args, cwd = fixture) {
  const result = cp.spawnSync(exe, args, { cwd, encoding: 'utf8', windowsHide: true, maxBuffer: 32 * 1024 * 1024 });
  if (result.error || result.status !== 0) throw Error(`${exe} ${args.join(' ')}\n${result.error || result.stderr || result.stdout}`);
  return result.stdout.trim();
}
function metrics(file) {
  const bytes = fs.readFileSync(file);
  return { sha256: hash(bytes), raw: bytes.length,
    brotli: zlib.brotliCompressSync(bytes, { params: { [zlib.constants.BROTLI_PARAM_QUALITY]: 11 } }).length,
    gzip: zlib.gzipSync(bytes, { level: 9 }).length };
}
function select(names = '') {
  const requested = names.split(',').filter(Boolean);
  if (requested.some(name => !cases.some(item => item.name === name))) throw Error('Unknown case');
  return requested.length ? cases.filter(item => requested.includes(item.name)) : cases;
}
function sources() {
  return Object.fromEntries(['sapid', 'glory', 'apace', 'takia', 'wingy'].map(name => {
    const root = path.resolve(repository, '..', name);
    const files = execute('git', ['ls-files', '--cached', '--others', '--exclude-standard', '-z'], root)
      .split('\0').filter(Boolean).sort();
    const entries = files.filter(file => fs.existsSync(path.join(root, file)))
      .map(file => [file, hash(fs.readFileSync(path.join(root, file)))]);
    return [name, { commit: execute('git', ['rev-parse', 'HEAD'], root),
      status: execute('git', ['status', '--short'], root), digest: hash(JSON.stringify(entries)) }];
  }));
}
function comparable(previous, current) {
  for (const key of ['schema', 'versions', 'profile', 'target', 'lockSha256']) {
    if (JSON.stringify(previous[key]) !== JSON.stringify(current[key])) throw Error(`Incompatible baseline: ${key}`);
  }
  if (!previous.complete) throw Error('Incomplete baseline');
  for (const item of current.cases) {
    const before = previous.cases.find(value => value.name === item.name);
    if (!before || JSON.stringify(before.features) !== JSON.stringify(item.features)) throw Error(`Missing or different baseline case: ${item.name}`);
    for (const kind of ['wasm', 'javascript']) {
      for (const metric of ['raw', 'brotli', 'gzip']) {
        if (!Number.isFinite(before[kind]?.[metric]) || before[kind][metric] < 0) throw Error('Invalid baseline metrics');
      }
    }
  }
}
async function main() {
  const args = process.argv.slice(2);
  function arg(name, fallback = '') {
    const index = args.indexOf(name);
    if (index < 0) return fallback;
    if (!args[index + 1] || args[index + 1].startsWith('--')) throw Error(`Missing ${name} value`);
    return args[index + 1];
  }
  const selected = select(arg('--only'));
  const label = arg('--label', 'baseline');
  if (!/^[a-z0-9-]+$/.test(label)) throw Error('Invalid label');
  const metadata = JSON.parse(execute('cargo', ['metadata', '--no-deps', '--format-version', '1']));
  const target = path.resolve(metadata.target_directory);
  const normalized = target.replaceAll('\\', '/').toLowerCase();
  if (normalized === 'd:/buildcache' || normalized.startsWith('d:/buildcache/') || normalized.includes('/.codex/build-targets')) {
    throw Error('Cargo metadata resolved a retired build location');
  }
  const out = path.join(target, 'verification/sapid-wasm-size/runs', label);
  fs.mkdirSync(path.dirname(out), { recursive: true });
  fs.mkdirSync(out); // Existing evidence must never be overwritten.
  if (!fs.existsSync(path.join(fixture, 'Cargo.lock'))) execute('cargo', ['generate-lockfile']);
  const bindgen = arg('--bindgen', process.env.WASM_BINDGEN || 'wasm-bindgen');
  const versions = { rustc: execute('rustc', ['-Vv']), cargo: execute('cargo', ['-V']),
    bindgen: execute(bindgen, ['--version']), node: process.version };
  if (versions.bindgen !== 'wasm-bindgen 0.2.130') throw Error('Requires wasm-bindgen 0.2.130');
  const report = { schema: 1, complete: false, versions, target, sources: sources(),
    profile: { release: true, optLevel: 'z', lto: true, codegenUnits: 1,
      optimizer: 'rust-release-and-bindgen', compression: 'brotli-q11/gzip-9',
      environment: Object.fromEntries(Object.entries(process.env).filter(([key]) =>
        /^(RUSTFLAGS|CARGO_ENCODED_RUSTFLAGS|RUSTC_WRAPPER|RUSTC_WORKSPACE_WRAPPER|CARGO_PROFILE_RELEASE_.*|CARGO_TARGET_.*_RUSTFLAGS)$/.test(key))) },
    lockSha256: hash(fs.readFileSync(path.join(fixture, 'Cargo.lock'))), cases: [] };
  fs.copyFileSync(path.join(fixture, 'Cargo.lock'), path.join(out, 'Cargo.lock'));
  fs.copyFileSync(path.join(repository, 'styles/sapid.css'), path.join(out, 'sapid.css'));
  report.stylesheetSha256 = hash(fs.readFileSync(path.join(out, 'sapid.css')));
  const resultFile = path.join(out, 'report.json');
  try {
    for (const item of selected) {
      console.log(`Building ${item.name}`);
      execute('cargo', ['build', '--locked', '--release', '--target', 'wasm32-unknown-unknown', '--lib',
        '--no-default-features', ...(item.features.length ? ['--features', item.features.join(',')] : [])]);
      const directory = path.join(out, item.name);
      fs.mkdirSync(directory);
      execute(bindgen, [path.join(target, 'wasm32-unknown-unknown/release/sapid_wasm_size.wasm'),
        '--target', 'web', '--out-dir', directory, '--remove-name-section']);
      const entry = path.join(directory, 'sapid_wasm_size.js');
      // Include all imported snippets, compressed as independently served resources.
      function resources(root) {
        return fs.readdirSync(root, { withFileTypes: true }).flatMap(value => {
          const file = path.join(root, value.name);
          return value.isDirectory() ? resources(file) : file.endsWith('.js') ? [file] : [];
        });
      }
      const javascriptFiles = resources(directory).sort().map(file => ({ path: path.relative(directory, file), ...metrics(file) }));
      const javascript = Object.fromEntries(['raw', 'brotli', 'gzip'].map(key =>
        [key, javascriptFiles.reduce((sum, file) => sum + file[key], 0)]));
      javascript.sha256 = hash(JSON.stringify(javascriptFiles));
      if (!fs.existsSync(entry)) throw Error('Missing bindgen entry');
      fs.writeFileSync(path.join(directory, 'index.html'), `<!doctype html><meta charset="utf-8"><title>${item.name}</title><link rel="stylesheet" href="../sapid.css"><div id="app"></div><script type="module">import init, {mount, unmount} from './sapid_wasm_size.js'; try { await init(); mount(); window.probe={unmount}; document.body.dataset.ready='true'; } catch(e) {document.body.dataset.error=String(e);}</script>`);
      report.cases.push({ ...item, htmlSha256: hash(fs.readFileSync(path.join(directory, 'index.html'))),
        wasm: metrics(path.join(directory, 'sapid_wasm_size_bg.wasm')), javascript, javascriptFiles });
      fs.writeFileSync(resultFile, JSON.stringify(report, null, 2));
    }
    if (hash(JSON.stringify(sources())) !== hash(JSON.stringify(report.sources))) throw Error('Sources changed during measurement');
    if (hash(fs.readFileSync(path.join(fixture, 'Cargo.lock'))) !== report.lockSha256) throw Error('Lock changed during measurement');
    report.complete = true;
    if (arg('--compare')) {
      const previous = JSON.parse(fs.readFileSync(path.resolve(arg('--compare'))));
      comparable(previous, report);
      const percent = Number(arg('--max-growth', '5'));
      if (!Number.isFinite(percent) || percent < 0) throw Error('Invalid growth budget');
      report.regressions = report.cases.flatMap(item => {
        const before = previous.cases.find(value => value.name === item.name);
        return ['wasm', 'javascript'].flatMap(kind => ['raw', 'brotli', 'gzip']
          .filter(metric => item[kind][metric] > before[kind][metric] * (1 + percent / 100))
          .map(metric => `${item.name}.${kind}.${metric}`));
      });
      if (report.regressions.length) throw Error(`Size regressions: ${report.regressions.join(', ')}`);
    }
  } catch (error) {
    report.complete = false;
    report.error = error.message;
    throw error;
  } finally { fs.writeFileSync(resultFile, JSON.stringify(report, null, 2)); }
  console.log(resultFile);
}
module.exports = { cases, select, metrics, comparable };
if (require.main === module) main().catch(error => { console.error(error); process.exitCode = 1; });
