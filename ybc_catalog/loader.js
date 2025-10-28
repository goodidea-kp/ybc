// Inline loader moved to an external module for clarity and to avoid inline parsing edge-cases.
// Attempts to load the wasm JS glue (Trunk-injected or wasm-pack pkg) and falls back to a simple DOM message.

async function tryLoadWasm(cand) {
  try {
    const res = await fetch(cand, { method: 'HEAD' });
    if (!res.ok) return false;
    const ct = res.headers.get('content-type') || '';
    if (!ct.includes('javascript') && !ct.includes('ecmascript') && !ct.includes('text/plain')) return false;

    const mod = await import(cand);
    if (mod && typeof mod.default === 'function') {
      try {
        await mod.default();
      } catch (e) {
        const wasmGuess = cand.replace(/\.js$/, '_bg.wasm');
        try { await mod.default(wasmGuess); } catch (_) {}
      }
    }
    if (mod && typeof mod.start === 'function') {
      try { mod.start(); } catch (_) {}
    }
    return true;
  } catch (_) {
    return false;
  }
}

async function init() {
  const root = document.getElementById('root');
  if (!root) return;

  // Prefer Trunk-injected modulepreload assets
  const modulePreloads = Array.from(document.querySelectorAll('link[rel="modulepreload"][href$=".js"]'))
    .map(l => {
      try { return new URL(l.getAttribute('href'), window.location.href).pathname; }
      catch (e) { return l.getAttribute('href'); }
    });

  // Meta hint for wasm-pack style builds
  const meta = document.querySelector('meta[name="wasm-pkg"]');
  const hinted = meta && meta.content ? meta.content.trim() : null;

  const candidates = (modulePreloads.length ? modulePreloads : (hinted ? [hinted] : []))
    .concat([
      'pkg/ybc_catalog.js',
      'pkg/ybc-catalog.js',
      'pkg/ybc_catalog_bg.js'
    ]);

  for (const c of candidates) {
    if (await tryLoadWasm(c)) {
      console.info('WASM loaded from', c);
      return;
    }
  }

  // Fallback DOM UI
  root.textContent = '';

  const section = document.createElement('section');
  section.className = 'section';

  const container = document.createElement('div');
  container.className = 'container';

  const h1 = document.createElement('h1');
  h1.className = 'title';
  h1.textContent = 'YBC Component Catalog (JS fallback)';

  const subtitle = document.createElement('p');
  subtitle.className = 'subtitle';
  subtitle.textContent = 'WASM failed to load — showing fallback UI and diagnostics below.';

  const catalog = document.createElement('div');
  catalog.id = 'catalog';

  const article = document.createElement('article');
  article.className = 'message is-danger';

  const messageHeader = document.createElement('div');
  messageHeader.className = 'message-header';
  const headerP = document.createElement('p');
  headerP.textContent = 'WASM load error';
  messageHeader.appendChild(headerP);

  const messageBody = document.createElement('div');
  messageBody.className = 'message-body';
  messageBody.innerHTML =
    'WASM could not be loaded. Check console for details.<br>' +
    'Build with: <code>wasm-pack build --target web</code> (or use <code>trunk</code>).<br>' +
    'Ensure <code>pkg/</code> files are served and that your server returns correct MIME types: ' +
    '<code>application/javascript</code> for .js and <code>application/wasm</code> for .wasm.';

  article.appendChild(messageHeader);
  article.appendChild(messageBody);
  catalog.appendChild(article);

  container.appendChild(h1);
  container.appendChild(subtitle);
  container.appendChild(catalog);
  section.appendChild(container);
  root.appendChild(section);

  try { if (window.bulmaCalendar && typeof window.bulmaCalendar.attach === 'function') window.bulmaCalendar.attach(); } catch (e) {}
  try { if (window.bulmaAccordion && typeof window.bulmaAccordion.attach === 'function') window.bulmaAccordion.attach(); } catch (e) {}
  try { if (window.bulmaTagsInput && typeof window.bulmaTagsInput.attach === 'function') window.bulmaTagsInput.attach(); } catch (e) {}
}

init();

