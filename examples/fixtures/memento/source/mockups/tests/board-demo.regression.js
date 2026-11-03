async function runBoardDemoRegression(page) {
  const origin = await page.evaluate(() => location.origin);
  const backup = await page.evaluate(() => ({ catalog: localStorage.getItem('memento-demo-boards-v1'), atlas: localStorage.getItem('memento-atlas-board-v2'), session: sessionStorage.getItem('memento-mockup-state-v1') }));
  const created = [], errors = [], externalRequests = [], results = [];
  const check = (value, message) => { if (!value) throw new Error(message); };
  const onError = e => errors.push(e.message);
  const onRequest = r => { if (/^https?:/.test(r.url()) && !r.url().startsWith(origin + '/')) externalRequests.push(r.url()); };
  page.on('pageerror', onError); page.on('request', onRequest);
  let id;
  const model = () => page.evaluate(id => JSON.parse(localStorage.getItem(`memento-demo-board-${id}`)), id);
  async function next() {
    const modalNext = page.locator('[data-demo-dialog="next"]');
    if (await modalNext.count()) await modalNext.click();
    else await page.locator('[data-demo="next"]').click();
  }
  try {
    await page.goto(`${origin}/02-home.html?regression=${Date.now()}`);
    check(await page.getByRole('button', { name: 'Create board', exact: true }).count() === 0, 'Home must start from New workspace');
    await page.getByRole('button', { name: 'New workspace', exact: true }).click();
    await page.getByLabel('Workspace name', { exact: true }).fill('Customer workshop / regression');
    await page.getByRole('button', { name: 'Create workspace', exact: true }).click();
    await page.locator('.demo-player').waitFor();
    id = await page.evaluate(() => new URLSearchParams(location.search).get('board')); created.push(id);
    check((await model()).records.length === 0, 'New board was not empty');
    check(await page.locator('[data-record]').count() === 0, 'Empty board has populated cards');
    check(await page.locator('.board-presence i').count() === 1, 'Empty board starts with fictional other participants online');
    check(await page.locator('.flow-board-title').innerText().then(text => text.includes('Starter board') && !text.includes('/ BOARD')), 'Board title contains breadcrumb or is missing');
    results.push('New workspace opens its own isolated empty starter board');
    await page.locator('[data-demo="play"]').click();
    await page.waitForFunction(id => JSON.parse(localStorage.getItem(`memento-demo-board-${id}`)).demo.step === 1, id, { timeout: 7000 });
    await page.locator('[data-demo="play"]').click();
    const paused = (await model()).demo.step;
    await page.waitForTimeout(4800);
    check((await model()).demo.step === paused, 'Pause did not stop playback');
    await page.reload();
    check((await model()).demo.step === 1, 'Playback progress not restored');
    check(await page.locator('[data-demo="play"]').innerText() === 'Resume demo', 'Reload must restore paused playback');
    results.push('Automatic progression, pause and durable paused reload');
    for (let stage = 2; stage <= 25; stage++) {
      await next();
      const data = await model();
      check(data.demo.step === stage, `Expected playback stage ${stage}`);
      if (stage === 2) check(await page.locator('.board-presence i').count() === 2, 'Simulated collaborator did not join');
      if (stage === 5) check(data.runs[0].status === 'Working' && data.runs[0].context.length === 3, 'Agent context missing');
      if (stage === 8) check(data.frames.length === 1 && !data.records.some(r => r.kind === 'decision'), 'Arrangement implies a decision');
      if (stage === 9) check(data.records.some(r => r.id === 'working-note') && !data.records.some(r => r.kind === 'decision'), 'Adoption implies a decision');
      if (stage === 12) check(await page.getByRole('heading', { name: 'Review exact external operation' }).count() || await page.getByRole('dialog').count(), 'Exact operation review missing');
      if (stage === 13) check(data.records.find(r => r.id === 'task-action').approval.revision === 1, 'Exact approval missing');
      if (stage === 15) check(data.records.find(r => r.id === 'observed-task').status === 'Completed', 'Observed completion missing');
      if (stage === 17) check(data.records.find(r => r.id === 'working-note').history.length === 1 && data.records.some(r => r.id === 'draft-note'), 'Conflict lost a contribution');
      if (stage === 18) check(data.connections[0].status === 'Disconnected' && await page.locator('[data-record="email"]').innerText().then(t => t.includes('offline')), 'Outage not visible');
      if (stage === 20) check(!data.records.some(r => r.sharedCapture) && await page.getByRole('dialog').innerText().then(t => t.includes('No mailbox access')), 'Sharing preview boundary missing');
      if (stage === 21) check(data.records.find(r => r.id === 'personal-email').sharedCapture && !data.connections.some(c => c.id === 'personal'), 'Personal connection leaked into workspace connections');
      if (stage === 22) check(await page.getByRole('dialog').innerText().then(t => t.includes('their own authorization')), 'Original access is not distinguished');
      if (stage === 23) check(await page.locator('.atlas-form-error').innerText().then(t => t.includes('Owner')), 'Editor write was not rejected');
      if (stage === 24) check(!data.records.find(r => r.id === 'email').placed && await page.locator('[data-record="email"]').count() === 0 && await page.getByRole('dialog').innerText().then(text => text.includes('Customer can attend')), 'Traceability did not retain the off-board source');
    }
    check(await page.locator('[data-demo="play"]').isDisabled(), 'Completed demo should stop playback');
    results.push('Full user story, layout adoption, agent, approval, outcomes, conflict, recovery and memory');
    results.push('Personal email preview, explicit capture publication and separate original access');
    await page.reload();
    check((await model()).demo.step === 25, 'Completed story lost on reload');
    await page.setViewportSize({ width: 390, height: 844 });
    check(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth), 'Mobile horizontal overflow');
    check(await page.locator('[data-demo="restart"]').isVisible(), 'Mobile playback controls unavailable');
    await page.locator('[data-demo="restart"]').click();
    await page.getByRole('button', { name: 'Save', exact: true }).click();
    check((await model()).records.length === 0 && (await model()).demo.step === 0, 'Confirmed restart did not return to empty');
    check(await page.evaluate(id => JSON.parse(sessionStorage.getItem('memento-mockup-state-v1')).activityItems.every(item => item.board !== id), id), 'Restart retained stale scoped activity');
    await page.setViewportSize({ width: 1440, height: 1000 });
    check(await page.getByRole('button', { name: /Create (another )?board/i }).count() === 0, 'Board contains redundant board creation controls');
    await page.goto(`${origin}/02-home.html?regression=${Date.now()}`);
    check(await page.getByRole('link', { name: /Starter board Customer workshop \/ regression/ }).count() === 1, 'Created board not reachable from home');
    await page.getByRole('button', { name: 'Open workspace Customer workshop / regression', exact: true }).click();
    await page.locator('.demo-player').waitFor();
    check(await page.evaluate(() => new URLSearchParams(location.search).get('board')) === id, 'Workspace did not reopen its first board');
    check(await page.evaluate(() => localStorage.getItem('memento-atlas-board-v2')) === backup.atlas, 'Original Atlas board was changed');
    check(errors.length === 0, errors.join('; ')); check(externalRequests.length === 0, 'Unexpected external requests');
    results.push('Home reopen, mobile controls, confirmed restart and original Atlas isolation');
    return { passed: true, results, browserErrors: errors, externalRequests };
  } finally {
    page.removeListener('pageerror', onError); page.removeListener('request', onRequest);
    await page.evaluate(({ backup, created }) => {
      created.forEach(id => localStorage.removeItem(`memento-demo-board-${id}`));
      if (backup.catalog === null) localStorage.removeItem('memento-demo-boards-v1'); else localStorage.setItem('memento-demo-boards-v1', backup.catalog);
      if (backup.session === null) sessionStorage.removeItem('memento-mockup-state-v1'); else sessionStorage.setItem('memento-mockup-state-v1', backup.session);
    }, { backup, created });
    await page.setViewportSize({ width: 1440, height: 1000 });
    await page.goto(`${origin}/02-home.html?regression=${Date.now()}`);
  }
}
if (typeof module !== 'undefined') module.exports = { runBoardDemoRegression };
