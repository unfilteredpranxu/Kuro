// Browser-only persistence and file adapters. All study algorithms live in Rust.
const DB = 'kuro-study-os';
function open() {
  return new Promise((resolve, reject) => {
    const req = indexedDB.open(DB, 1);
    req.onupgradeneeded = () => req.result.createObjectStore('state');
    req.onsuccess = () => resolve(req.result);
    req.onerror = () => reject(new Error('Cannot open study storage: ' + req.error?.name));
    req.onblocked = () => reject(new Error('Close other KURO tabs to finish the storage upgrade.'));
  });
}
export async function load() {
  const db = await open();
  try {
    return await new Promise((resolve, reject) => {
      const tx = db.transaction('state', 'readonly');
      const r = tx.objectStore('state').get('main');
      tx.oncomplete = () => resolve(r.result ?? '');
      tx.onabort = () => reject(new Error('Cannot read study data. Original data remains intact.'));
      tx.onerror = () => reject(tx.error);
    });
  } finally { db.close(); }
}
export async function save(json, expected) {
  const db = await open();
  try {
    await new Promise((resolve, reject) => {
      const tx = db.transaction('state', 'readwrite');
      const store = tx.objectStore('state');
      const get = store.get('main');
      let reason;
      get.onsuccess = () => {
        try {
          const current = get.result ? JSON.parse(get.result) : { revision: 0 };
          if (current.revision !== expected) {
            reason = 'Another KURO tab changed your data. Reload before editing; your change was not saved.';
            tx.abort(); return;
          }
          store.put(json, 'main');
        } catch (_) { reason = 'Stored data could not be validated. Nothing was overwritten.'; tx.abort(); }
      };
      tx.oncomplete = () => resolve();
      tx.onabort = () => reject(new Error(reason || 'Write failed (' + (tx.error?.name || 'storage unavailable') + '). Your previous data is intact.'));
      tx.onerror = () => {}; // onabort reports the transactional failure
    });
  } finally { db.close(); }
}
export function today() {
  const parts = new Intl.DateTimeFormat('en-CA', { timeZone: 'Asia/Kolkata', year: 'numeric', month: '2-digit', day: '2-digit' }).formatToParts(new Date());
  const get = type => parts.find(x => x.type === type).value;
  return `${get('year')}-${get('month')}-${get('day')}`;
}
export function uid() { return crypto.randomUUID(); }
export function download(text, name) {
  const url = URL.createObjectURL(new Blob([text], { type: 'application/json' }));
  const a = document.createElement('a'); a.href = url; a.download = name; a.click();
  setTimeout(() => URL.revokeObjectURL(url), 1000);
}
export function pick_backup() {
  return new Promise((resolve, reject) => {
    const input = document.createElement('input'); input.type = 'file'; input.accept = '.json,application/json';
    input.onchange = async () => {
      try {
        const file = input.files[0];
        if (!file) { resolve(''); return; }
        if (file.size > 20000000) { reject(new Error('Backup exceeds 20 MB limit')); return; }
        resolve(await file.text());
      } catch (_) { reject(new Error('Cannot read backup file')); }
    };
    input.oncancel = () => resolve(''); input.click();
  });
}
export function shortcuts() {
  const handler = e => {
    const dialog = document.querySelector('.palette');
    if (dialog && e.key === 'Tab') {
      const items = [...dialog.querySelectorAll('button, input, select, textarea, [tabindex="0"]')].filter(x => !x.disabled);
      const first = items[0], last = items[items.length - 1];
      if (e.shiftKey && document.activeElement === first) { e.preventDefault(); last?.focus(); }
      else if (!e.shiftKey && document.activeElement === last) { e.preventDefault(); first?.focus(); }
    }

    if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'k') {
      e.preventDefault(); document.getElementById('command-toggle')?.click();
    }
  };
  document.addEventListener('keydown', handler);
}
function draftKey() {
  let id = sessionStorage.getItem('kuro-editor-id');
  if (!id) { id = crypto.randomUUID(); sessionStorage.setItem('kuro-editor-id', id); }
  return 'kuro-note-draft-' + id;
}
export function save_draft(json) { localStorage.setItem(draftKey(), json); }
export function load_draft() { return localStorage.getItem(draftKey()) || ''; }
export function clear_draft() { localStorage.removeItem(draftKey()); }

let dialogReturnFocus;
export function dialog_focus(open) {
  if (open) {
    dialogReturnFocus = document.activeElement;
    requestAnimationFrame(() => document.querySelector('.palette input')?.focus());
  } else if (dialogReturnFocus) {
    dialogReturnFocus.focus(); dialogReturnFocus = undefined;
  }
}
