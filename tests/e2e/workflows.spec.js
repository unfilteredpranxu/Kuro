import { test, expect } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';
const nav = (page, name) => page.getByRole('navigation').getByRole('button', { name: new RegExp(`^\\d+ ${name}`) }).click();
async function open(page) {
  await page.goto('/');
  await expect(page.getByRole('heading', { name: 'Make room for understanding.' })).toBeVisible();
  await expect(page.getByText('OPENING DEVICE STORAGE…')).toHaveCount(0);
}
async function addTask(page, title='Vectors — first principles') {
  await nav(page, 'Tasks');
  await page.getByLabel('Task title', { exact: true }).fill(title);
  await page.getByRole('button', { name: 'Create task' }).click();
  await expect(page.getByRole('heading', { name: title, exact:true })).toBeVisible();
}
test('task create, reschedule, complete, refresh; planner shares records', async ({page}) => {
  await open(page); await addTask(page);
  await page.getByRole('button',{name:'Edit',exact:true}).click();
  await page.getByLabel('Reschedule · IST').fill('2027-01-04');
  await page.getByRole('button',{name:'Save task',exact:true}).click();
  await expect(page.getByText('Physics / 2027-01-04 / 120 MIN')).toBeVisible();
  await nav(page,'Planner'); await page.getByLabel('Agenda date').fill('2027-01-04');
  await expect(page.getByText('Vectors — first principles',{exact:true})).toBeVisible();
  await nav(page,'Tasks'); await page.getByRole('button',{name:'Complete Vectors — first principles',exact:true}).click();
  await page.getByRole('button',{name:'Completed',exact:true}).click();
  await expect(page.getByRole('heading',{name:'Vectors — first principles'})).toBeVisible();
  await page.reload(); await nav(page,'Tasks'); await page.getByRole('button',{name:'Completed',exact:true}).click();
  await expect(page.getByRole('button',{name:'Reopen Vectors — first principles'})).toBeVisible();
});
test('notes preserve literal content and chapter label after reload', async({page})=>{
  await open(page);await nav(page,'Notes');
  await page.getByLabel('Note title').fill('A derivation'); await page.getByLabel('Chapter / topic label',{exact:true}).fill('Kinematics');
  await page.getByLabel('Your notes').fill('v² = u² + 2as\n<script>alert("no")</script>');await page.getByRole('button',{name:'Save note ↗',exact:true}).click();
  await expect(page.getByRole('button',{name:'A derivation Kinematics'})).toBeVisible();
  await page.reload();await nav(page,'Notes');await page.getByRole('button',{name:'A derivation Kinematics'}).click();
  await expect(page.getByLabel('Your notes')).toHaveValue('v² = u² + 2as\n<script>alert("no")</script>');
});
test('attempts calculate independent accuracy, mistakes create recall cards',async({page})=>{
  await open(page);await nav(page,'Practice');
  await page.getByLabel('Question / topic / source').fill('Vectors Q1');await page.getByRole('button',{name:'Save attempt'}).click();
  await expect(page.locator('.tag').getByText('Independent',{exact:true})).toBeVisible();
  await page.getByLabel('Question / topic / source').fill('Vectors Q2');await page.getByLabel('Outcome').selectOption('Hinted');
  await page.getByLabel('Mistake / recall prompt (optional)').fill('Resolve perpendicular components first.');await page.getByRole('button',{name:'Save attempt'}).click();
  await expect(page.locator('.tag').getByText('Hinted',{exact:true})).toBeVisible();await nav(page,'Analytics');await expect(page.getByText('50.0%',{exact:true})).toBeVisible();
  await nav(page,'Revision');await expect(page.getByRole('heading',{name:'Revisit: Vectors Q2'})).toBeVisible();
  await page.getByRole('button',{name:'Reveal answer'}).click();await expect(page.getByText('Resolve perpendicular components first.',{exact:true})).toBeVisible();
  await page.getByRole('button',{name:'Recalled ✓'}).click();await expect(page.getByRole('heading',{name:'A little breathing room.'})).toBeVisible();
  await page.reload();await nav(page,'Analytics');await expect(page.getByText('50.0%',{exact:true})).toBeVisible();
});
test('themes persist and command palette is keyboard accessible',async({page})=>{
  await open(page);await page.keyboard.press('Control+k');await expect(page.getByRole('dialog')).toBeVisible();await expect(page.getByLabel('Search modules')).toBeFocused();
  await page.getByLabel('Search modules').fill('Settings');await page.getByRole('button',{name:'12 / Settings'}).click();
  await page.getByRole('button',{name:'PAPER',exact:true}).click();await expect(page.locator('.app')).toHaveClass(/theme-paper/);
  await page.reload();await expect(page.locator('.app')).toHaveClass(/theme-paper/);
});
test('backup restores into a clean browser context and rejects corruption',async({page,browser})=>{
  await open(page);await addTask(page,'Backup proof');await nav(page,'Settings');
  const dlPromise=page.waitForEvent('download');await page.getByRole('button',{name:'Export backup ↓'}).click();const dl=await dlPromise;const path=await dl.path();
  const fresh=await browser.newContext();const p=await fresh.newPage();await open(p);await nav(p,'Settings');
  const choose=p.waitForEvent('filechooser');await p.getByRole('button',{name:'Choose backup to restore'}).click();await (await choose).setFiles(path);
  await expect(p.getByRole('heading',{name:'Review restore'})).toBeVisible();await p.getByRole('checkbox').check();await p.getByRole('button',{name:'Restore verified backup'}).click();
  await expect(p.getByText('Saved on this device')).toBeVisible();await p.reload();await nav(p,'Tasks');await expect(p.getByRole('heading',{name:'Backup proof',exact:true})).toBeVisible();
  await nav(p,'Settings');const bad=p.waitForEvent('filechooser');await p.getByRole('button',{name:'Choose backup to restore'}).click();await(await bad).setFiles({name:'bad.json',mimeType:'application/json',buffer:Buffer.from('{"format":"invalid"}')});
  await expect(p.getByRole('alert')).toContainText('Malformed backup');await nav(p,'Tasks');await expect(p.getByRole('heading',{name:'Backup proof'})).toBeVisible();await fresh.close();
});
test('failed storage write is reported and never appears saved',async({page})=>{
  await page.addInitScript(()=>{
    const original=indexedDB.open.bind(indexedDB);
    indexedDB.open=function(...args){const r=original(...args);r.addEventListener('success',()=>{const db=r.result;const transaction=db.transaction.bind(db);db.transaction=function(names,mode,...rest){const tx=transaction(names,mode,...rest);if(mode==='readwrite'){queueMicrotask(()=>{try{tx.abort()}catch{}})}return tx}});return r};
  });
  await open(page);await nav(page,'Tasks');await page.getByLabel('Task title',{exact:true}).fill('Must not falsely save');await page.getByRole('button',{name:'Create task'}).click();
  await expect(page.getByRole('alert')).toContainText('Write failed');await expect(page.getByRole('heading',{name:'Must not falsely save'})).toHaveCount(0);await expect(page.getByText('Saved on this device')).toHaveCount(0);
});
test('stale tab cannot overwrite newer data',async({page,context})=>{
  await open(page);const tab=await context.newPage();await open(tab);await addTask(page,'Original tab');
  await nav(tab,'Tasks');await tab.getByLabel('Task title',{exact:true}).fill('Conflicting tab');await tab.getByRole('button',{name:'Create task'}).click();
  await expect(tab.getByRole('alert')).toContainText('Another KURO tab changed your data');await tab.reload();await nav(tab,'Tasks');await expect(tab.getByRole('heading',{name:'Original tab'})).toBeVisible();await expect(tab.getByRole('heading',{name:'Conflicting tab'})).toHaveCount(0);
});
test('all modules navigate without console or page errors; accessibility scan',async({page})=>{
  const errors=[];page.on('pageerror',e=>errors.push(e.message));page.on('console',m=>{if(m.type()==='error')errors.push(m.text())});await open(page);
  for(const name of ['Syllabus','Planner','Tasks','Library','Notes','Practice','Revision','Exams','Music','Analytics','Settings','Home']){await nav(page,name);await expect(page.locator('main h1')).toBeVisible();expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);const scan=await new AxeBuilder({page}).analyze();expect(scan.violations,`accessibility in ${name}`).toEqual([]);}
  const results=await new AxeBuilder({page}).analyze();expect(results.violations).toEqual([]);expect(errors).toEqual([]);
});
test('refresh during note editing recovers the latest draft, then autosaves',async({page})=>{
  await open(page);await nav(page,'Notes');await page.getByLabel('Note title').fill('Recovered idea');await page.getByLabel('Your notes').fill('The latest unsaved sentence.');
  await page.reload();await nav(page,'Notes');await expect(page.getByLabel('Your notes')).toHaveValue('The latest unsaved sentence.');
  await expect(page.getByRole('button',{name:'Recovered idea Unlinked note'})).toBeVisible();
});
test('invalid stored schema cannot be overwritten by a new session',async({page})=>{
  await open(page);
  await page.evaluate(async()=>{await new Promise((resolve,reject)=>{const r=indexedDB.open('kuro-study-os',1);r.onsuccess=()=>{const db=r.result;const tx=db.transaction('state','readwrite');tx.objectStore('state').put('{"schema":999,"revision":4}','main');tx.oncomplete=()=>{db.close();resolve()};tx.onerror=()=>reject(tx.error)}})});
  await page.reload();await expect(page.getByRole('alert')).toContainText('Original data was not overwritten');await nav(page,'Tasks');await expect(page.getByRole('button',{name:'Create task'})).toBeDisabled();
  expect(await page.evaluate(async()=>{return await new Promise(resolve=>{const r=indexedDB.open('kuro-study-os',1);r.onsuccess=()=>{const db=r.result;const tx=db.transaction('state');const q=tx.objectStore('state').get('main');tx.oncomplete=()=>{db.close();resolve(q.result)}}})})).toBe('{"schema":999,"revision":4}');
});
test('switching to a new note commits edits before replacing the draft',async({page})=>{
  await open(page);await nav(page,'Notes');await page.getByLabel('Note title').fill('Keep before switching');await page.getByLabel('Your notes').fill('Do not lose this sentence.');
  await page.getByRole('button',{name:'+ NEW',exact:true}).click();await expect(page.getByLabel('Note title')).toHaveValue('');
  await page.getByRole('button',{name:'Keep before switching Unlinked note'}).click();await expect(page.getByLabel('Your notes')).toHaveValue('Do not lose this sentence.');
});
