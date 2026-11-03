(() => {
  'use strict';
  const registryKey = 'memento-demo-boards-v1';
  const escape = value => String(value).replace(/[&<>"']/g, c => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' })[c]);
  function list(notify) {
    try {
      const items = JSON.parse(localStorage.getItem(registryKey) || '[]');
      if (!Array.isArray(items) || items.some(item => !/^[a-f0-9-]{36}$/.test(item.id) || typeof item.name !== 'string')) throw new Error('Invalid board catalog');
      return items;
    } catch (error) { notify(`Could not load demo boards: ${error.message}`); return []; }
  }
  function create({ notify, icon, workspaceId, workspaceName = 'Atlas Launch' }) {
    const root = document.querySelector('#overlay-root'), returnFocus = document.activeElement;
    root.innerHTML = `<div class="modal-backdrop"><section class="modal" role="dialog" aria-modal="true" aria-labelledby="create-board-title"><div class="modal-head"><div><h2 id="create-board-title">Create a board</h2><p>Start empty in Atlas Launch, then watch a simulated participant build the board.</p></div><button type="button" class="icon-button" aria-label="Close dialog">${icon('close')}</button></div><form><div class="field"><label for="demo-board-name">Board name</label><input id="demo-board-name" name="name" required maxlength="60" value="From first idea to shared decision"></div><div class="notice-line"><b>Local experience demo</b><span>Automatic playback with pause, next step and restart. Includes a clearly labelled personal-email sharing prototype. No real accounts, connections or messages.</span></div><p class="atlas-form-error" role="alert"></p><div class="modal-actions"><button type="button" class="quiet-button">Cancel</button><button type="submit" class="primary-button">Create board</button></div></form></section></div>`;
    root.querySelector('.modal-head p').textContent = `Start an empty board in ${workspaceName}, then watch a simulated participant build it.`;
    const close = () => { root.innerHTML = ''; document.removeEventListener('keydown', keys); returnFocus?.focus(); };
    const keys = event => {
      if (event.key === 'Escape') { event.stopPropagation(); close(); }
      if (event.key === 'Tab') {
        const els = [...root.querySelectorAll('button,input')], first = els[0], last = els[els.length - 1];
        if (event.shiftKey && document.activeElement === first) { event.preventDefault(); last.focus(); }
        else if (!event.shiftKey && document.activeElement === last) { event.preventDefault(); first.focus(); }
      }
    };
    document.addEventListener('keydown', keys);
    root.querySelectorAll('button[type="button"]').forEach(button => button.addEventListener('click', close));
    root.querySelector('form').addEventListener('submit', event => {
      event.preventDefault();
      const name = new FormData(event.currentTarget).get('name').trim();
      if (!name) { root.querySelector('[role="alert"]').textContent = 'Enter a board name.'; return; }
      const entry = { id: crypto.randomUUID(), name, workspaceId, workspaceName };
      try { localStorage.setItem(registryKey, JSON.stringify([...list(notify), entry])); }
      catch (error) { root.querySelector('[role="alert"]').textContent = `Board was not created: ${error.message}`; return; }
      location.href = `05-board.html?board=${entry.id}`;
    });
    root.querySelector('input').focus();
  }
  function mount(root, options) {
    const id = new URLSearchParams(location.search).get('board'), entry = list(options.notify).find(item => item.id === id);
    if (!entry) {
      root.innerHTML = '<h1>Board not available</h1><p>This local demo board is not in this browser catalog.</p><a href="02-home.html">Return home</a>';
      return;
    }
    document.title = `Memento / ${entry.name}`;
    root.innerHTML = '<section class="demo-player" aria-label="Experience playback"></section><div class="demo-canvas"></div>';
    const player = root.querySelector('.demo-player'), canvas = root.querySelector('.demo-canvas');
    const seed = () => ({ ...window.AtlasBoard.empty(), presence: [{ name: 'Maya Chen', initials: 'MC' }], demo: { step: 0 } });
    const workspaceName = entry.workspaceName || 'Atlas Launch';
    const api = window.AtlasBoard.mount(canvas, { ...options, initialData: seed(), storageKey: `memento-demo-board-${id}`, boardId: id, boardName: entry.name, workspaceName });
    let timer = null, playing = false;
    const record = (id, kind, title, body, x, y, extra = {}) => ({ id, kind, title, body, x, y, actor: 'Maya Chen', revision: 1, status: 'Current', placed: true, history: [], ...extra });
    const get = (data, id) => data.records.find(r => r.id === id);
    const evidence = (data, id) => {
      const r = get(data, id);
      return { id, revision: r.revision, title: r.title, body: r.body };
    };
    const edge = (data, from, to, type) => data.relations.push({ id: crypto.randomUUID(), from, to, type });
    const event = (data, title, detail, actor = 'Maya Chen', category = 'collaboration') => {
      const item = { id: crypto.randomUUID(), board: id, workspace: workspaceName, title, event: title, detail, actor, category, time: 'Just now', read: false };
      data.log.unshift(item); options.onEvent(item);
    };
    const steps = [
      { title: 'Your empty board', text: 'Nothing has been added. Press Play to watch Maya start from zero. Pause at any point to inspect or edit the actual local board.', panel: 'inspect' },
      { title: 'Maya writes the first note', text: 'The first contribution is a human note, not a decision or an agent instruction.', selection: ['goal'], apply(data) {
        data.records.push(record('goal', 'note', 'Prepare a useful customer workshop', 'Agree a time, collect the constraints and assign follow-up work.', 100, 80));
        event(data, 'First note added', 'Maya starts with the workshop objective.');
      } },
      { title: 'Leo joins and adds a constraint', text: 'Simulated collaboration: Leo contributes independently. Presence is illustrative, not another connected browser.', selection: ['constraint'], apply(data) {
        data.presence = [{ name: 'Maya Chen', initials: 'MC' }, { name: 'Leo Martin', initials: 'LM' }];
        data.records.push(record('constraint', 'note', 'Keep the workshop on Friday', 'Only the time can change. Confirm the room before contacting participants.', 500, 40, { actor: 'Leo Martin' }));
        event(data, 'Participant contribution', 'Leo adds a scheduling constraint.', 'Leo Martin');
      } },
      { title: 'Owner checks the project connection', text: 'This connection belongs to Atlas Launch. Its scoped project mailbox is separate from Maya’s personal mailbox.', panel: 'sources', apply(data) {
        event(data, 'Project source scope reviewed', 'The owner checks project mailbox reading and reviewed writes.', 'Maya Chen', 'sources');
      } },
      { title: 'A project email arrives', text: 'An observed message appears from the shared project source. It is not an inferred fact or an editable copy of the mailbox.', panel: 'inspect', selection: ['email'], apply(data) {
        data.records.push(record('email', 'source', 'Customer can attend at 11:00', 'From: Jordan Lee <jordan@example.test>\nReceived: today, 09:42\nPlease move the workshop to 11:00 on Friday.', 100, 440, { actor: 'Mail observation', status: 'Observed', connection: 'mail', sourceScope: data.connections[0].scope, sourceEndpoint: data.connections[0].endpoint, externalId: 'mail/workshop/42', observed: 'Just now' }));
        event(data, 'Project email observed', 'The scoped project source adds the customer request.', 'Mail observation', 'sources');
      } },
      { title: 'Maya selects context for the facilitator', text: 'The request includes the selected revisions, not unrestricted access to every email or every workspace.', panel: 'agents', selection: ['goal', 'constraint', 'email'], apply(data) {
        data.runs.push({ id: 'demo-run', role: 'Facilitator', request: 'What changes if we move the workshop to 11:00? Suggest a clearer arrangement.', status: 'Working', context: ['goal', 'constraint', 'email'].map(id => evidence(data, id)) });
        data.chatContext = data.runs[0].context;
        data.messages.push({ actor: 'Maya Chen', body: data.runs[0].request, refs: data.chatContext }, { actor: 'Facilitator', body: 'Demo: I will compare the options against these pinned references. Friday stays fixed; is the room constraint still current?', refs: data.chatContext });
        event(data, 'Agent run started', 'Maya requests an impact assessment with three pinned records.', 'Maya Chen', 'agent');
      } },
      { title: 'The facilitator proposes, not decides', text: 'Maya answers the clarification, then explicitly chooses to show the Proposal on the board. This does not convert any Note. The arrangement remains a separate choice.', panel: 'inspect', selection: ['proposal'], apply(data) {
        const run = data.runs[0]; run.status = 'Completed'; run.output = 'proposal';
        data.messages.push({ actor: 'Maya Chen', body: 'Yes, Friday stays fixed and the room still needs confirmation.', refs: [] }, { actor: 'Facilitator', body: 'Demo: here is a reviewable Proposal. You can inspect it without adding it to the board; Maya chooses Show on board for this walkthrough.', refs: run.context });
        data.records.push(record('proposal', 'proposal', 'Move to 11:00 and confirm the room', 'Keep Friday. Ask Leo to confirm the room before notifying the customer.', 930, 220, { actor: 'Facilitator', status: 'Pending review', evidence: run.context }));
        edge(data, 'email', 'proposal', 'supports'); edge(data, 'constraint', 'proposal', 'supports');
        event(data, 'Proposal submitted', 'The facilitator offers a reviewable option and a layout suggestion.', 'Facilitator', 'agent');
      } },
      { title: 'Maya reviews a proposed arrangement', text: 'Preview: put the source and constraint in a compact Context frame; keep the objective outside. Content and decision state will not change.', panel: 'inspect', selection: ['proposal'] },
      { title: 'Maya applies only the arrangement', text: 'A frame and new positions organize the discussion. This is a visual change, not an adopted proposal or a decision.', apply(data) {
        data.frames.push({ id: 'context-frame', title: 'Context for Friday', x: 60, y: 350, width: 780, height: 650 });
        Object.assign(get(data, 'email'), { x: 100, y: 430 });
        Object.assign(get(data, 'constraint'), { x: 480, y: 690 });
        event(data, 'Arrangement accepted', 'Maya accepts the suggested frame and positions. No decision recorded.');
      } },
      { title: 'Maya adopts working material', text: 'Adoption creates an editable note. A separate explicit decision follows in the next step.', selection: ['working-note'], apply(data) {
        get(data, 'proposal').status = 'Adopted';
        data.records.push(record('working-note', 'note', 'Workshop follow-up plan', 'Confirm the room, then notify Jordan. Keep Friday unchanged.', 950, 610, { evidence: [evidence(data, 'proposal')] }));
        edge(data, 'working-note', 'proposal', 'derived-from');
        event(data, 'Proposal adopted', 'Maya keeps an editable working note, without implying a decision.', 'Maya Chen', 'agent');
      } },
      { title: 'Maya records the chosen direction', text: 'Maya records a Decision directly from the email and constraint, with rationale and pinned evidence, and chooses Show on board. A Proposal is not a required stage.', selection: ['decision'], apply(data) {
        data.records.push(record('decision', 'decision', 'Hold the workshop at 11:00 on Friday', 'Resolve the customer conflict while retaining the agreed day.', 1400, 650, { evidence: ['email', 'constraint'].map(id => evidence(data, id)) }));
        edge(data, 'email', 'decision', 'supports'); edge(data, 'constraint', 'decision', 'supports');
        event(data, 'Decision recorded', 'Maya records the direction and rationale.', 'Maya Chen', 'decisions');
      } },
      { title: 'The coordinator prepares a task', text: 'The operation has an owner, due date, destination and decision reference. It has not been approved or submitted.', selection: ['task-action'], apply(data) {
        data.records.push(record('task-action', 'action', 'Create task: confirm the workshop room', 'Confirm the room for Friday at 11:00 before notifying Jordan.', 1840, 400, { actor: 'Action Coordinator', status: 'Awaiting approval', operation: 'create-task', connection: 'deck', endpoint: data.connections[1].endpoint, target: data.connections[1].scope, owner: 'Leo Martin', due: '2026-10-30', evidence: [evidence(data, 'decision')], approval: null }));
        edge(data, 'decision', 'task-action', 'results-in');
        event(data, 'Action proposed', 'The coordinator drafts a task for Leo.', 'Action Coordinator', 'actions');
      } },
      { title: 'Maya reviews the exact operation', text: 'The real review dialog is open. Playback will simulate approval next. Pause here to review or change the draft yourself.', selection: ['task-action'], after() { api.action('review', 'task-action'); } },
      { title: 'Maya approves that version', text: 'Approval belongs to this exact revision and destination. It is not a general permission for the agent.', selection: ['task-action'], apply(data) {
        const r = get(data, 'task-action'); r.status = 'Approved';
        r.approval = { actor: 'Maya Chen', revision: r.revision, scopeRevision: data.connections[1].scopeRevision, target: r.target };
        event(data, 'Action approved', 'Maya approves the displayed task version.', 'Maya Chen', 'actions');
      } },
      { title: 'The simulated connector reports success', text: 'The external task is a separate observed record, connected to its initiating action. No real task is created.', selection: ['observed-task'], apply(data) {
        const r = get(data, 'task-action'); r.status = 'Succeeded'; r.result = 'observed-task';
        data.records.push(record('observed-task', 'task', 'Confirm the workshop room', 'Owner: Leo Martin\nDue: Friday\nObserved status: Open', 2260, 400, { actor: 'Deck observation', status: 'Open', connection: 'deck', sourceScope: data.connections[1].scope, sourceEndpoint: data.connections[1].endpoint, externalId: 'deck/workshop/17', observed: 'Just now' }));
        edge(data, 'observed-task', 'task-action', 'derived-from');
        event(data, 'Task creation observed', 'The demo adds a separate external task result.', 'Connector simulation', 'actions');
      } },
      { title: 'Leo completes the task in Deck', text: 'A new observation updates the task. The accepted decision and its original evidence do not change.', selection: ['observed-task'], apply(data) {
        const task = get(data, 'observed-task'); task.history.push(evidence(data, task.id)); task.revision++; task.status = 'Completed'; task.body = 'Owner: Leo Martin\nDue: Friday\nObserved status: Completed\nRoom confirmed at 11:00.';
        event(data, 'Task completion observed', 'Leo completes the task in the simulated external client.', 'Deck observation', 'sources');
      } },
      { title: 'A concurrent edit is detected', text: 'Leo’s accepted text and Maya’s draft are both visible. Playback will preserve Leo’s version and save Maya’s contribution separately.', selection: ['working-note'], after() {
        api.dialog('Concurrent edit / simulated', '<h3>Accepted revision from Leo</h3><p>Room confirmed. Notify Jordan after the equipment check.</p><h3>Maya’s draft</h3><p>Also prepare the customer handout.</p><div class="atlas-notice">Neither contribution is silently overwritten. Next: keep the accepted revision and preserve the draft as another note.</div>');
      } },
      { title: 'Both contributions are preserved', text: 'The accepted note has a new revision. Maya’s incompatible draft becomes a separate attributable note.', selection: ['draft-note'], apply(data) {
        const note = get(data, 'working-note'); note.history.push(evidence(data, note.id)); note.revision++; note.body = 'Room confirmed. Notify Jordan after the equipment check.'; note.actor = 'Leo Martin';
        data.records.push(record('draft-note', 'note', 'Prepare the customer handout', 'Maya’s preserved contribution from the conflict review.', 1320, 1050));
        event(data, 'Edit conflict resolved', 'Accepted text and the other contribution are both preserved.');
      } },
      { title: 'The source connection goes offline', text: 'The last captured email remains readable. It is explicitly marked as last-known content, not fresh data.', selection: ['email'], apply(data) {
        data.connections[0].status = 'Disconnected';
        event(data, 'Connection interrupted', 'Last-known content is retained while the project source is offline.', 'Connector simulation', 'sources');
      } },
      { title: 'Reading resumes after recovery', text: 'The project source reconnects. Its observations, local notes and accepted decisions remain distinct.', panel: 'sources', apply(data) {
        data.connections[0].status = 'Connected'; data.connections[0].checked = 'Just now';
        event(data, 'Connection recovered', 'Observation resumes without erasing board contributions.', 'Connector simulation', 'sources');
      } },
      { title: 'Personal email sharing / proposed extension', text: 'Maya previews one fictional message from her private mailbox. Nothing has been shared yet. This is a product-policy prototype, not a defined MVP connection.', panel: 'inspect', after() {
        api.dialog('Share a personal email capture / prototype', '<p><strong>Private connection:</strong> Maya’s mailbox. Not a workspace-wide connection.</p><h3>Selected content</h3><p>From: alex@example.test<br>Subject: Accessibility for Friday<br>Please include a dial-in option.</p><p><strong>Audience:</strong> Atlas Launch members and permitted agent context.</p><div class="atlas-notice">Share selected message text only. Exclude attachments and the rest of the thread. No mailbox access, automatic refresh or send-as permission is granted. Playback simulates Maya confirming this preview in the next step.</div>');
      } },
      { title: 'Maya publishes the selected capture', text: 'Members can read only the published material and its provenance. Personal connection credentials and other messages are not shared.', selection: ['personal-email'], apply(data) {
        data.records.push(record('personal-email', 'source', 'Accessibility for Friday', 'From: alex@example.test\nPlease include a dial-in option.', 450, 1120, { status: 'Observed', actor: 'Shared by Maya Chen', externalId: 'personal-message/demo-7', observed: 'Just now', sharedCapture: { by: 'Maya Chen', from: 'alex@example.test', revision: 1 } }));
        event(data, 'Email capture shared / prototype', 'Maya explicitly publishes only the selected text to Atlas Launch.', 'Maya Chen', 'sources');
      } },
      { title: 'Leo cannot borrow Maya’s mailbox access', text: 'The original requires separate external authorization. Reading the shared capture does not authorize opening the private mailbox or sending as Maya.', selection: ['personal-email'], after() { api.action('original', 'personal-email'); } },
      { title: 'An Editor cannot manage workspace connections', text: 'The demo switches to the Editor role. Configure connection is rejected by the existing permission check. This is a local role preview, not server authorization.', panel: 'sources', apply(data) {
        data.role = 'Editor';
      }, after() { api.action('connection', 'mail'); document.querySelector('#atlas-dialog-form').requestSubmit(); } },
      { title: 'Inspect traceability without another board', text: 'The email is removed only from the working canvas. The Inspector opens an informational traceability dialog with preserved evidence and outcomes. Locate on board is available only for existing cards.', panel: 'inspect', trace: true, apply(data) {
        data.role = 'Owner'; get(data, 'email').placed = false;
        event(data, 'Placement removed', 'The source and pinned decision evidence remain in memory.');
      } },
      { title: 'Demo complete: continue exploring', text: 'Playback is complete. Inspect evidence and outcomes in this dialog, then close it to continue on the same board. The board, chat and progress are saved locally. The original Atlas Launch example remains unchanged.', panel: 'inspect', trace: true }
    ];
    function stepIndex() { return Math.min(api.snapshot().demo?.step || 0, steps.length - 1); }
    function controls() {
      const index = stepIndex(), step = steps[index];
      player.innerHTML = `<div class="demo-narration"><span class="eyebrow">SIMULATED USER / ${index + 1} OF ${steps.length}${index >= 20 && index <= 22 ? ' / PERSONAL SHARING PROTOTYPE' : ''}</span><h2>${escape(step.title)}</h2><p>${escape(step.text.replaceAll('Atlas Launch', workspaceName))}</p></div><div class="demo-controls"><button type="button" class="primary-button" data-demo="play" ${index === steps.length - 1 ? 'disabled' : ''}>${index === steps.length - 1 ? 'Demo complete' : playing ? 'Pause' : index ? 'Resume demo' : 'Play demo'}</button><button type="button" class="outline-button" data-demo="next" ${index === steps.length - 1 ? 'disabled' : ''}>Next step</button><button type="button" class="quiet-button" data-demo="restart">Restart demo</button><label>Step duration<select aria-label="Step duration"><option value="4500">4.5 seconds</option><option value="8000">8 seconds</option></select></label><small>${playing ? 'Playing. Interacting with the board pauses playback.' : 'Paused. You can inspect and edit the board.'}</small></div>`;
      player.querySelector('select').value = String(duration);
      player.querySelector('select').addEventListener('change', e => { duration = Number(e.target.value); if (playing) schedule(); });
    }
    let duration = 4500;
    function pause() { playing = false; clearTimeout(timer); controls(); }
    function schedule() { clearTimeout(timer); timer = setTimeout(advance, duration); }
    function advance() {
      const index = stepIndex() + 1;
      if (index >= steps.length) { pause(); return; }
      const step = steps[index];
      api.update(data => { step.apply?.(data); data.demo = { step: index }; }, { selection: step.selection, panel: step.panel || 'inspect', trace: step.trace, fit: true });
      step.after?.();
      const dialog = document.querySelector('.atlas-modal');
      if (dialog) {
        const controls = document.createElement('div');
        controls.className = 'atlas-buttons';
        controls.innerHTML = '<button type="button" class="quiet-button" data-demo-dialog="pause">Pause playback</button><button type="button" class="outline-button" data-demo-dialog="next">Next demo step</button>';
        controls.addEventListener('click', event => {
          const action = event.target.closest('[data-demo-dialog]')?.dataset.demoDialog;
          if (action) pause();
          if (action === 'next') advance();
        });
        dialog.append(controls);
      }
      if (index === steps.length - 1) playing = false;
      controls();
      if (playing) schedule();
    }
    player.addEventListener('click', e => {
      const action = e.target.closest('[data-demo]')?.dataset.demo;
      if (action === 'play') { if (playing) pause(); else { playing = true; controls(); schedule(); } }
      if (action === 'next') { pause(); advance(); }
      if (action === 'restart') {
        pause();
        api.dialog('Restart this demo board?', '<p>This resets only this new board, including your manual edits and playback progress. The original Atlas example and other boards are not changed.</p>', () => {
          api.update(data => { Object.keys(data).forEach(key => delete data[key]); Object.assign(data, seed()); }, { selection: [], panel: 'inspect', mode: 'board' });
          options.onReset(id);
          controls();
        });
      }
    });
    const pauseOnInput = e => { if (e.isTrusted && playing) pause(); };
    canvas.addEventListener('pointerdown', pauseOnInput, true);
    canvas.addEventListener('keydown', pauseOnInput, true);
    document.querySelector('#overlay-root').addEventListener('pointerdown', pauseOnInput, true);
    document.querySelector('#overlay-root').addEventListener('keydown', pauseOnInput, true);
    document.addEventListener('visibilitychange', () => { if (document.hidden) pause(); });
    controls();
  }
  window.BoardDemo = { list, create, mount };
})();
