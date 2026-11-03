/* Run with a Playwright Page against the mockups HTTP server. No live services are required. */
async function runAtlasBoardRegression(page) {
  const key = 'memento-atlas-board-v2';
  const origin = await page.evaluate(() => location.origin);
  const backup = await page.evaluate(key => ({ board: localStorage.getItem(key), activity: sessionStorage.getItem('memento-mockup-state-v1') }), key);
  const results = [];
  const errors = [];
  const externalRequests = [];
  const onError = error => errors.push(error.message);
  const onRequest = request => { if (/^https?:/.test(request.url()) && !request.url().startsWith(origin + '/')) externalRequests.push(request.url()); };
  page.on('pageerror', onError);
  page.on('request', onRequest);
  const check = (value, message) => { if (!value) throw new Error(message); };
  const model = () => page.evaluate(key => JSON.parse(localStorage.getItem(key)), key);
  const click = async (action, id, scope = '.atlas-board') => {
    if (action === 'trace' && !await page.locator(`${scope} [data-atlas-action="trace"]`).count()) await page.locator('[data-atlas-action="panel"][data-id="activity"]').click();
    if (['run-question', 'run-complete', 'run-fail'].includes(action)) {
      const run = page.locator('.atlas-chat-run').filter({ has: page.locator(`[data-atlas-action="${action}"][data-id="${id}"]`) });
      for (const selector of ['.atlas-run-details', '.atlas-demo-controls']) {
        const details = run.locator(selector);
        if (await details.getAttribute('open') === null) await details.locator(':scope > summary').click();
      }
    }
    return page.locator(`${scope} [data-atlas-action="${action}"]${id ? `[data-id="${id}"]` : ''}`).first().click();
  };
  const dialogClick = (action, id) => click(action, id, '.atlas-modal');
  const save = () => page.locator('.atlas-modal button[type="submit"]').click();
  const close = () => page.getByRole('button', { name: 'Close dialog', exact: true }).click();
  async function focus(id) {
    if (await page.getByRole('dialog').count()) await close();
    if (!(await recordState(id)).placed) {
      await click('panel', 'activity');
      await click('trace', undefined);
      await page.locator(`.atlas-modal [data-atlas-action="detail"][data-id="${id}"]`).click();
      await dialogClick('restore', id);
    }
    if (!await page.getByRole('searchbox', { name: 'Search board' }).count()) await click('search-toggle');
    await page.getByRole('searchbox', { name: 'Search board' }).fill('');
    const record = page.locator(`[data-record="${id}"]`);
    const title = (await recordState(id)).title;
    await page.getByRole('searchbox', { name: 'Search board' }).fill(title);
    await click('fit');
    await record.click();
    await click('panel', 'inspect');
  }
  if (typeof module !== 'undefined') module.exports = { runAtlasBoardRegression };
  async function recordState(id) { return (await model()).records.find(r => r.id === id); }
  try {
    await page.evaluate(key => localStorage.removeItem(key), key);
    await page.goto(`${origin}/05-board.html?regression=${Date.now()}`);
    await page.locator('.atlas-board').waitFor();
    check(await page.locator('[data-record]').count() === 14, 'Expected 14 initial records');
    check(await page.locator('.atlas-panel-tabs [data-atlas-action="panel"]').count() === 4, 'Expected Inspector, Sources, Agents and unified Activity');
    check(await page.locator('[data-action="board-agent"],[data-action="board-update"],.flow-presence,.flow-board-title a').count() === 0, 'Removed POC controls or duplicate chrome returned');
    await click('fit');
    check((await model()).records.filter(r => r.kind === 'decision' && r.status === 'Current').length === 2, 'Multiple current decisions not represented');
    results.push('Populated board, contextual panels and multiple decisions');

    for (const id of ['sources', 'agents', 'activity', 'inspect']) {
      await click('panel', id);
      check(await page.locator('.atlas-panel-tabs [aria-pressed="true"]').count() === 1, 'Exactly one context tab must be selected');
      check(await page.locator(`.atlas-panel-tabs [data-id="${id}"]`).getAttribute('aria-pressed') === 'true', 'Selected tab does not match the displayed panel');
      await page.reload();
      check(await page.locator(`.atlas-panel-tabs [data-id="${id}"]`).getAttribute('aria-pressed') === 'true', 'Selected context panel was lost on reload');
    }
    await click('panel', 'agents');
    await page.locator('[data-record="goal"]').click();
    check(await page.locator('.atlas-panel-tabs [data-id="agents"]').getAttribute('aria-pressed') === 'true', 'Selecting context unexpectedly changed the chosen panel');
    await click('panel', 'inspect');
    results.push('Context tab highlight, card selection and chosen panel survive reload');

    const initial = await model();
    check(initial.frames.filter(f => Math.min(f.width, f.height) / Math.max(f.width, f.height) >= .8).length >= 3, 'Expected at least three compact, near-square frames');
    check(new Set(initial.frames.map(f => f.y)).size >= 4, 'Frames still form one aligned row of columns');
    const layout = await page.locator('.flow-world').evaluate(world => {
      const rect = el => ({ x: el.offsetLeft, y: el.offsetTop, width: el.offsetWidth, height: el.offsetHeight });
      const nodes = [...world.querySelectorAll('[data-record]')].map(el => ({ id: el.dataset.record, ...rect(el) }));
      const frames = [...world.querySelectorAll('[data-frame]')].map(rect);
      const contained = (n, f) => n.x >= f.x && n.y >= f.y + 37 && n.x + n.width <= f.x + f.width && n.y + n.height <= f.y + f.height;
      return {
        outside: nodes.filter(n => !frames.some(f => contained(n, f))).map(n => n.id),
        overlaps: nodes.some((a, i) => nodes.slice(i + 1).some(b => a.x < b.x + b.width && a.x + a.width > b.x && a.y < b.y + b.height && a.y + a.height > b.y)),
        invalidPaths: [...world.querySelectorAll('[data-relation] path')].some(path => !path.getAttribute('d') || /NaN|Infinity/.test(path.getAttribute('d')))
      };
    });
    check(['goal', 'constraint', 'failed-action'].every(id => layout.outside.includes(id)), 'Expected loose notes and an operation outside the frames');
    check(layout.outside.length === 3, 'A grouped card is clipped outside its frame');
    check(!layout.overlaps && !layout.invalidPaths, 'Initial cards overlap or connectors are invalid');
    const heading = await page.locator('[data-frame="frame-0"] .flow-frame-heading').boundingBox();
    await page.mouse.move(heading.x + heading.width / 2, heading.y + heading.height / 2);
    await page.mouse.down();
    await page.mouse.move(heading.x + heading.width / 2 + 35, heading.y + heading.height / 2 + 25, { steps: 4 });
    await page.mouse.up();
    const moved = await model();
    const shift = moved.frames[0].x - initial.frames[0].x;
    check(shift > 0, 'Frame heading did not move the frame');
    check(['email', 'room'].every(id => Math.abs(moved.records.find(r => r.id === id).x - initial.records.find(r => r.id === id).x - shift) < .01), 'Frame movement did not carry its grouped cards');
    check(moved.records.find(r => r.id === 'goal').x === initial.records.find(r => r.id === 'goal').x, 'Loose note moved with an unrelated frame');
    await click('undo');
    check((await model()).frames[0].x === initial.frames[0].x, 'Undo did not restore the frame');
    results.push('Mixed spatial composition, loose cards, non-overlapping groups and frame dragging');

    const legacy = JSON.parse(JSON.stringify(initial));
    delete legacy.layoutVersion;
    const oldPositions = [[100, 130], [100, 390], [100, 700], [540, 130], [540, 410], [540, 730], [990, 130], [990, 430], [990, 740], [1440, 130], [1440, 460], [1440, 800], [1890, 130], [1890, 460]];
    legacy.records.forEach((r, i) => { [r.x, r.y] = oldPositions[i]; });
    legacy.records[0].body = 'Preserve this edited content during layout upgrade.';
    legacy.frames.forEach((f, i) => Object.assign(f, { x: 60 + i * 450, y: 60, width: 410, height: 1080 }));
    await page.evaluate(({ key, data }) => localStorage.setItem(key, JSON.stringify(data)), { key, data: legacy });
    await page.reload();
    check((await model()).frames[0].width === initial.frames[0].width, 'Untouched legacy geometry was not upgraded');
    check((await recordState('goal')).body === legacy.records[0].body, 'Layout migration overwrote record content');
    legacy.records[0].x += 37;
    await page.evaluate(({ key, data }) => localStorage.setItem(key, JSON.stringify(data)), { key, data: legacy });
    await page.reload();
    check(await page.locator('[data-record="goal"]').evaluate(el => el.offsetLeft) === legacy.records[0].x, 'Customized geometry was overwritten');
    await page.evaluate(({ key, data }) => localStorage.setItem(key, JSON.stringify(data)), { key, data: initial });
    await page.reload();
    results.push('Legacy layout upgrade preserves content and leaves customized geometry intact');

    async function blankPoint() {
      return page.locator('.flow-viewport').evaluate(viewport => {
        const rect = viewport.getBoundingClientRect();
        for (let y = rect.bottom - 80; y > rect.top + 60; y -= 30) {
          for (let x = rect.right - 50; x > rect.left + 90; x -= 30) {
            const target = document.elementFromPoint(x, y);
            if (target && viewport.contains(target) && !target.closest('[data-record],[data-frame],button,.flow-zoom-controls,.flow-tool-rail')) return { x, y };
          }
        }
        throw new Error('No empty canvas point available');
      });
    }
    for (const tool of ['note', 'text', 'shape', 'frame']) {
      await click('tool', tool);
      const point = await blankPoint();
      await page.mouse.click(point.x, point.y);
    }
    let visual = await model();
    check(visual.records.some(r => r.title === 'Untitled Note'), 'Note tool did not create a record');
    check(visual.records.some(r => r.title === 'New text'), 'Text tool did not create a record');
    check(visual.records.some(r => r.title === 'New shape'), 'Shape tool did not create a record');
    check(visual.frames.length === 6, 'Frame tool did not create a frame');
    for (const record of visual.records.filter(r => ['note', 'text', 'shape'].includes(r.kind))) {
      await focus(record.id);
      check(await page.locator(`[data-record="${record.id}"] .atlas-card-badges .atlas-badge`).count() === 0, 'Ordinary material has a lifecycle badge on its card');
      check(await page.locator('.atlas-panel-content > .atlas-badge').count() === 0, 'Ordinary material has a lifecycle badge in the Inspector');
    }
    await focus('decision');
    check(await page.locator('[data-record="decision"] .atlas-badge').filter({ hasText: 'Current' }).count() === 1, 'Decision lifecycle badge was removed');
    await page.getByRole('searchbox', { name: 'Search board' }).fill('');
    await click('fit');
    results.push('Notes, text and shapes omit lifecycle badges; decisions retain their state');
    await click('tool', 'pen');
    const inkPoint = await blankPoint();
    await page.mouse.move(inkPoint.x, inkPoint.y);
    await page.mouse.down();
    await page.mouse.move(inkPoint.x - 35, inkPoint.y - 25, { steps: 5 });
    await page.mouse.up();
    check((await model()).strokes.length === 1, 'Pen did not create an ink stroke');
    await click('tool', 'erase');
    await page.mouse.click(inkPoint.x - 18, inkPoint.y - 13);
    check((await model()).strokes.length === 0, 'Erase did not remove the stroke');
    await click('undo');
    check((await model()).strokes.length === 1, 'Visual undo did not restore ink');
    await click('redo');
    check((await model()).strokes.length === 0, 'Visual redo did not remove ink');
    await click('tool', 'select');
    const source = await page.locator('[data-record="goal"] .source-handle').boundingBox();
    const destination = await page.locator('[data-record="constraint"]').boundingBox();
    await page.mouse.move(source.x + source.width / 2, source.y + source.height / 2);
    await page.mouse.down(); await page.mouse.move(destination.x + 25, destination.y + 50, { steps: 8 }); await page.mouse.up();
    check((await model()).visualLinks.some(e => e.from === 'goal' && e.to === 'constraint'), 'Handle drag did not create a visual connection');
    check(!(await model()).relations.some(e => e.from === 'goal' && e.to === 'constraint'), 'Drawing incorrectly created a business relationship');
    await click('undo');
    check((await model()).visualLinks.length === 0, 'Undo did not remove visual link');
    await click('redo');
    check((await model()).visualLinks.length === 1, 'Redo did not restore visual link');
    const position = await page.locator('[data-record="goal"]').evaluate(el => ({ x: el.offsetLeft, y: el.offsetTop }));
    await click('trace');
    check(await page.locator('[data-record="goal"]').evaluate((el, p) => el.offsetLeft === p.x && el.offsetTop === p.y, position), 'Memory rearranged the board');
    check(await page.locator('.atlas-modal .flow-world').count() === 0, 'Traceability must be information, not another board');
    await close();
    results.push('Handle connections, visual undo/redo and informational traceability');
    await focus('date-decision');
    await click('follow-up', 'date-decision');
    check(await page.locator('.atlas-modal select[name="decision"]').inputValue() === 'date-decision', 'Follow-up lost the contextual decision');
    await close();
    await focus('goal');
    await click('decision', 'goal', '.atlas-panel-content');
    check(await page.locator('.atlas-modal input[name="evidence"][value="goal"]').isChecked(), 'Contextual evidence was not selected');
    await close();
    await focus('failed-action');
    await click('prerequisite', 'failed-action');
    await page.locator('.atlas-modal select[name="required"]').selectOption('uncertain-action');
    await save();
    check((await model()).relations.some(e => e.from === 'failed-action' && e.to === 'uncertain-action' && e.type === 'depends-on'), 'Contextual prerequisite not recorded');
    results.push('Contextual evidence, preselected follow-up and explicit execution prerequisite');

    await click('panel', 'sources');
    await click('browse', 'mail');
    await dialogClick('import', 'mail');
    check((await model()).records.some(r => r.title === 'Room availability confirmed'), 'Scoped source import failed');
    await click('panel', 'sources');
    await click('arrive', 'mail');
    await click('arrive', 'deck');
    const arrivals = (await model()).records.filter(r => r.id.startsWith('arrival-'));
    check(arrivals.some(r => r.connection === 'mail' && r.placed), 'Automatic placement failed');
    check(arrivals.some(r => r.connection === 'deck' && !r.placed), 'Manual import scope failed');
    results.push('Scoped browse, import, automatic and manual arrivals');

    await focus('email');
    await click('annotate', 'email');
    await page.locator('.atlas-modal textarea[name="body"]').fill('Local annotation regression');
    await save();
    check((await recordState('email')).revision === 1, 'Local annotation mutated external observation revision');
    await click('panel', 'sources');
    await click('connection-toggle', 'mail');
    await focus('email');
    await click('refresh', 'email');
    check((await recordState('email')).revision === 1, 'Offline refresh modified observation');
    check((await recordState('email')).annotation === 'Local annotation regression', 'Outage lost annotation');
    await click('panel', 'sources');
    await click('connection-toggle', 'mail');
    results.push('External observations separate from local annotations; offline protection');

    await focus('constraint');
    await click('panel', 'agents');
    await click('attach-context');
    await page.locator('#atlas-agent-form textarea').fill('Assess the revised logistics');
    await page.locator('#atlas-agent-form button[type="submit"],#atlas-agent-form button.primary-button').click();
    let run = (await model()).runs[0];
    await click('run-cancel', run.id);
    check((await model()).runs[0].status === 'Cancelled', 'Agent cancellation failed');
    await page.locator('#atlas-agent-form textarea').fill('Assess the room dependency');
    await page.locator('#atlas-agent-form button.primary-button').click();
    run = (await model()).runs[0];
    await click('run-complete', run.id);
    const proposalId = (await model()).runs[0].output;
    check((await recordState(proposalId)).status === 'Pending review', 'Agent contribution missing');
    await focus('constraint');
    await click('edit', 'constraint');
    await page.getByRole('textbox', { name: 'Note text' }).fill('My proposed room constraint');
    await click('conflict', 'constraint');
    await page.getByRole('textbox', { name: 'Note text' }).press('Control+Enter');
    check((await page.getByRole('dialog').innerText()).includes('Resolve concurrent edit'), 'Conflict was silently overwritten');
    await dialogClick('conflict-apply');
    check((await recordState('constraint')).body === 'My proposed room constraint', 'Conflict resolution failed');
    check((await recordState(proposalId)).status === 'Stale', 'Changed context did not stale pending proposal');
    const evidence = (await recordState('decision')).evidence.find(ref => ref.id === 'constraint');
    check(evidence.revision === 1 && evidence.body !== 'My proposed room constraint', 'Pinned decision evidence was rewritten');
    results.push('Agent context, cancellation, proposal staleness, pinned evidence and explicit conflict resolution');

    await focus('alternative');
    await click('reject', 'alternative');
    check((await recordState('alternative')).status === 'Rejected', 'Proposal rejection failed');
    await focus('goal');
    await click('panel', 'agents');
    await click('attach-context');
    await page.locator('#atlas-agent-form textarea').fill('Summarize the launch objective');
    await page.locator('#atlas-agent-form button.primary-button').click();
    await click('run-complete', (await model()).runs[0].id);
    const adoptable = (await model()).runs[0].output;
    const decisionsBeforeAdoption = (await model()).records.filter(r => r.kind === 'decision').length;
    await focus(adoptable);
    await click('adopt', adoptable);
    check((await recordState(adoptable)).status === 'Adopted', 'Proposal adoption failed');
    check((await model()).records.filter(r => r.kind === 'decision').length === decisionsBeforeAdoption, 'Adoption implicitly recorded a decision');
    results.push('Proposal adoption creates editable material without recording a decision');
    await focus('decision');
    await click('supersede', 'decision');
    await page.locator('.atlas-modal input[name="title"]').fill('Hold the briefing at 11:30');
    await page.locator('.atlas-modal textarea[name="body"]').fill('The confirmed room slot starts at 11:30.');
    await save();
    let board = await model();
    const replacement = board.records.find(r => r.title === 'Hold the briefing at 11:30');
    check(replacement && replacement.supersedes === 'decision', 'Replacement decision missing');
    check(board.records.find(r => r.id === 'decision').replacedBy === replacement.id, 'Previous decision not linked');
    check(board.records.find(r => r.id === 'old-decision').status === 'Superseded', 'Earlier history lost');
    check(board.records.find(r => r.id === 'date-decision').status === 'Current', 'Unrelated decision was replaced');
    check(!replacement.placed, 'Decision was placed without explicit choice');
    await close();
    await click('trace');
    check(await page.locator(`.atlas-modal [data-atlas-action="detail"][data-id="${replacement.id}"]`).count() === 1, 'Off-board Decision erased from traceability');
    await close();
    results.push('Decision supersession chain preserves parallel decisions and off-board memory');

    await focus('task-action');
    await click('edit-action', 'task-action');
    await page.locator('.atlas-modal select[name="decision"]').selectOption(replacement.id);
    await save();
    await focus('task-action');
    await click('review', 'task-action');
    await dialogClick('approve', 'task-action');
    check((await recordState('task-action')).status === 'Approved', 'Exact-version approval failed');
    await click('edit-action', 'task-action');
    await page.locator('.atlas-modal textarea[name="body"]').fill('Confirm the room at 11:30 and prepare equipment.');
    await save();
    check((await recordState('task-action')).approval === null, 'Content change retained approval');
    await click('review', 'task-action');
    await dialogClick('approve', 'task-action');
    await click('panel', 'sources');
    await click('connection', 'deck');
    await page.locator('.atlas-modal input[name="scope"]').fill('Atlas Launch / Logistics');
    await save();
    check((await recordState('task-action')).approval === null, 'Scope change retained approval');
    await focus('task-action');
    await click('edit-action', 'task-action');
    await save();
    await click('review', 'task-action');
    await dialogClick('approve', 'task-action');
    await click('execute', 'task-action');
    await dialogClick('outcome-success', 'task-action');
    const completed = await recordState('task-action');
    check(completed.status === 'Succeeded' && completed.result, 'Successful action did not produce an observed task');
    await click('focus', completed.result);
    await click('original', completed.result);
    await dialogClick('task-complete', completed.result);
    check((await recordState(completed.result)).status === 'Completed', 'Returned groupware completion failed');
    results.push('Task approval, content/scope invalidation, execution and observed completion');

    await focus('reply-action');
    await click('edit-action', 'reply-action');
    await page.locator('.atlas-modal select[name="decision"]').selectOption(replacement.id);
    await save();
    await click('review', 'reply-action');
    check((await page.getByRole('dialog').innerText()).includes('jordan@example.test'), 'Reply review lacks recipient');
    check((await page.getByRole('dialog').innerText()).includes('atlas@example.test'), 'Reply review lacks sender');
    await dialogClick('approve', 'reply-action');
    await click('execute', 'reply-action');
    await dialogClick('outcome-uncertain', 'reply-action');
    check((await recordState('reply-action')).status === 'Uncertain', 'Uncertain outcome not preserved');
    check(await page.locator('.atlas-panel-content [data-atlas-action="execute"]').count() === 0, 'Uncertain operation can be resubmitted');
    await click('reconcile', 'reply-action');
    await dialogClick('reconcile-found', 'reply-action');
    check((await recordState('reply-action')).status === 'Succeeded', 'Outcome reconciliation failed');
    results.push('Exact email review, uncertain submission and reconciliation before retry');

    await page.reload();
    await page.locator('.atlas-board').waitFor();
    check((await recordState('decision')).status === 'Superseded', 'Decision history lost on reload');
    check((await recordState('reply-action')).status === 'Succeeded', 'Action outcome lost on reload');
    await page.setViewportSize({ width: 390, height: 844 });
    check(await page.evaluate(() => document.documentElement.scrollWidth === innerWidth), 'Mobile horizontal overflow');
    await click('panel', 'sources');
    check(await page.getByRole('heading', { name: 'Sources & services' }).isVisible(), 'Mobile connection panel unavailable');
    await click('connection', 'mail');
    await page.keyboard.press('Tab');
    check(await page.locator('.atlas-modal').evaluate(el => el.contains(document.activeElement)), 'Dialog keyboard focus escaped');
    await page.keyboard.press('Escape');
    check(await page.getByRole('dialog').count() === 0, 'Escape did not close dialog');
    results.push('Durable reload, narrow viewport and dialog keyboard controls');
    check(errors.length === 0, 'Browser errors: ' + errors.join('; '));
    check(externalRequests.length === 0, 'Mockup contacted external services: ' + externalRequests.join('; '));
    return { passed: true, results, browserErrors: errors, externalRequests };
  } finally {
    page.removeListener('pageerror', onError);
    page.removeListener('request', onRequest);
    await page.evaluate(({ key, backup }) => {
      if (backup.board === null) localStorage.removeItem(key); else localStorage.setItem(key, backup.board);
      if (backup.activity === null) sessionStorage.removeItem('memento-mockup-state-v1'); else sessionStorage.setItem('memento-mockup-state-v1', backup.activity);
    }, { key, backup });
    await page.setViewportSize({ width: 1440, height: 1000 });
    await page.goto(`${origin}/05-board.html?regression=complete`);
  }
}
