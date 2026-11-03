(() => {
  'use strict';
  const copy = value => JSON.parse(JSON.stringify(value));
  const escape = value => String(value ?? '').replace(/[&<>"']/g, char => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' })[char]);
  const kindNames = { note: 'Note', source: 'Email source', task: 'Observed task', proposal: 'Proposal', decision: 'Decision', action: 'External action', text: 'Text', shape: 'Shape' };
  const relationNames = { supports: 'Evidence for', 'derived-from': 'Based on', 'depends-on': 'Needs first', supersedes: 'Replaces', 'results-in': 'Follow-up action' };
  const noteColors = { amber: 'Yellow', mint: 'Green', blue: 'Blue', pink: 'Pink', lavender: 'Purple' };
  const cardClass = r => ({ note: `flow-node-note note-${Object.hasOwn(noteColors, r.color) ? r.color : 'amber'}`, source: 'flow-source', task: 'flow-source', proposal: 'flow-agent', decision: 'flow-decision', action: 'flow-task', text: 'atlas-text-node', shape: 'atlas-shape-node' })[r.kind] || 'atlas-text-node';
  function fixture() {
    const records = [
      ['goal', 'note', 'Launch a clear, coordinated briefing', 'Align the speaker, room and guest communications. Launch date: 30 October.', 80, 40, 'Maya Chen'],
      ['email', 'source', 'Speaker unavailable at 10:00', 'From: Jordan Lee <jordan@example.test>\nReceived: today, 09:42\nPlease move the briefing to 11:00. Keep the launch date unchanged.', 100, 410, 'Mail sync'],
      ['room', 'task', 'Prepare the briefing room', 'Owner: Leo Martin\nDue: today\nObserved status: In progress', 470, 690, 'Deck sync'],
      ['constraint', 'note', 'Keep the launch date fixed', 'Only the briefing time can move. Confirm room availability before notifying guests.', 480, 20, 'Maya Chen'],
      ['proposal', 'proposal', 'Move the briefing to 11:00', 'The speaker is available at 11:00. Keep the release date; notify guests after the room is confirmed.', 1000, 135, 'Facilitator'],
      ['alternative', 'proposal', 'Offer a remote fallback', 'If the room is unavailable, retain 11:00 and use a remote briefing.', 1360, 410, 'Facilitator'],
      ['old-decision', 'decision', 'Hold the briefing at 10:00', 'Original slot agreed before the speaker availability changed.', 1000, 895, 'Maya Chen'],
      ['decision', 'decision', 'Hold the briefing at 11:00', 'Resolve the speaker conflict without changing the launch date.', 1360, 895, 'Maya Chen'],
      ['date-decision', 'decision', 'Keep the launch date: 30 October', 'The schedule adjustment does not affect release readiness.', 1180, 1210, 'Maya Chen'],
      ['task-action', 'action', 'Create task: confirm room at 11:00', 'Confirm room availability and update the briefing checklist.', 1920, 575, 'Action Coordinator'],
      ['reply-action', 'action', 'Reply to Jordan with the new time', 'We have moved the briefing to 11:00. The launch date remains 30 October.', 2270, 875, 'Action Coordinator'],
      ['uncertain-action', 'action', 'Earlier guest notification', 'An earlier send was interrupted after submission. Check the sent folder before another attempt.', 1920, 975, 'Action Coordinator'],
      ['completed-task', 'task', 'Publish the briefing agenda', 'Owner: Sam Rivera\nDue: today\nObserved status: Completed', 2160, 100, 'Deck sync'],
      ['failed-action', 'action', 'Create task: reserve projector', 'The external service rejected the operation. No task was created.', 2280, 1390, 'Action Coordinator']
    ].map(([id, kind, title, body, x, y, actor]) => ({ id, kind, title, body, x, y, actor, revision: 1, status: 'Current', placed: true, history: [] }));
    const get = id => records.find(record => record.id === id);
    Object.assign(get('email'), { connection: 'mail', sourceEndpoint: 'https://mail.localhost:8443/', sourceScope: 'atlas@example.test / Inbox / Atlas thread', externalId: 'mail/inbox/42', observed: 'Today, 09:43', annotation: 'Speaker availability is confirmed.', status: 'Observed' });
    Object.assign(get('room'), { connection: 'deck', sourceEndpoint: 'http://localhost:8080/', sourceScope: 'Atlas Launch / Next steps', externalId: 'deck/atlas/cards/17', observed: 'Today, 10:05', status: 'In progress' });
    Object.assign(get('completed-task'), { connection: 'deck', sourceEndpoint: 'http://localhost:8080/', sourceScope: 'Atlas Launch / Next steps', externalId: 'deck/atlas/cards/12', observed: 'Today, 10:12', status: 'Completed' });
    get('old-decision').status = 'Superseded';
    get('old-decision').replacedBy = 'decision';
    get('decision').supersedes = 'old-decision';
    get('proposal').status = 'Adopted';
    get('alternative').status = 'Pending review';
    for (const id of ['task-action', 'reply-action', 'uncertain-action', 'failed-action']) {
      Object.assign(get(id), { operation: id === 'task-action' || id === 'failed-action' ? 'create-task' : 'send-reply', connection: id === 'task-action' || id === 'failed-action' ? 'deck' : 'mail', endpoint: id === 'task-action' || id === 'failed-action' ? 'http://localhost:8080/' : 'https://mail.localhost:8443/', status: 'Awaiting approval', owner: 'Leo Martin', due: '2026-10-26', sender: 'atlas@example.test', recipients: 'jordan@example.test', subject: 'Re: Atlas briefing time', target: id === 'task-action' || id === 'failed-action' ? 'Atlas Launch / Next steps' : 'atlas@example.test / Inbox / Atlas thread', approval: null });
    }
    get('uncertain-action').status = 'Uncertain';
    get('uncertain-action').externalId = 'mail/submission/previous';
    get('failed-action').status = 'Failed';
    const snapshot = id => {
      const r = get(id);
      return { id, revision: r.revision, title: r.title, body: r.body };
    };
    for (const id of ['proposal', 'alternative', 'decision', 'date-decision']) get(id).evidence = ['email', 'constraint'].map(snapshot);
    get('old-decision').evidence = [snapshot('goal')];
    for (const id of ['task-action', 'reply-action', 'uncertain-action', 'failed-action']) get(id).evidence = [snapshot('decision')];
    return {
      version: 2, layoutVersion: 1, records, camera: { x: 30, y: 45, zoom: .48 }, role: 'Owner', strokes: [],
      frames: [
        ['Context & sources', 60, 330, 760, 680],
        ['Team thinking', 960, 60, 760, 650],
        ['Decisions & history', 960, 820, 760, 690],
        ['Proposed operations', 1880, 500, 780, 800],
        ['Observed outcomes', 2120, 20, 440, 390]
      ].map(([title, x, y, width, height], index) => ({ id: `frame-${index}`, title, x, y, width, height })),
      relations: [
        ['email', 'proposal', 'supports'], ['constraint', 'proposal', 'supports'],
        ['decision', 'proposal', 'derived-from'], ['decision', 'old-decision', 'supersedes'],
        ['decision', 'task-action', 'results-in'], ['decision', 'reply-action', 'results-in'],
        ['reply-action', 'task-action', 'depends-on'], ['date-decision', 'constraint', 'derived-from']
      ].map(([from, to, type], index) => ({ id: `edge-${index}`, from, to, type })),
      connections: [
        { id: 'mail', name: 'Project mailbox', provider: 'Stalwart mail', endpoint: 'https://mail.localhost:8443/', client: 'Nextcloud Mail / http://localhost:8080', transport: 'Mail adapter protocol pending validation: IMAP/SMTP or JMAP', auth: 'Server-held service credential', scope: 'atlas@example.test / Inbox / Atlas thread', account: 'atlas@example.test', status: 'Connected', checked: 'Today, 10:20', automatic: true, read: true, write: true, scopeRevision: 1 },
        { id: 'deck', name: 'Project tasks', provider: 'Nextcloud Deck', endpoint: 'http://localhost:8080/', client: 'Nextcloud Deck / http://localhost:8080', transport: 'Scoped Deck REST adapter', auth: 'Server-held service credential', scope: 'Atlas Launch / Next steps', account: 'Maya Chen', status: 'Connected', checked: 'Today, 10:20', automatic: false, read: true, write: true, scopeRevision: 1 }
      ],
      runs: [{ id: 'run-initial', role: 'Facilitator', request: 'What changes if we move the briefing?', status: 'Completed', context: ['email', 'constraint', 'old-decision'].map(snapshot), output: 'proposal' }],
      log: [{ title: 'Decision recorded', detail: 'Maya recorded 11:00; the 10:00 decision and its evidence remain available.', category: 'decisions', actor: 'Maya Chen', time: 'Today, 10:18' }],
      invitations: []
    };
  }
  window.AtlasBoard = {
    preview(data, label) {
      if (!data) return '<span class="board-preview-empty">Preview unavailable</span>';
      const records = data.records.filter(r => r.placed), frames = data.frames || [];
      const inkPoints = (data.strokes || []).flatMap(stroke => {
        const values = (stroke.path.match(/-?\d+(?:\.\d+)?/g) || []).map(Number);
        return values.flatMap((value, index) => index % 2 === 0 && index + 1 < values.length ? [{ x: value, y: values[index + 1], width: 1, height: 1 }] : []);
      });
      const boxes = [...records.map(r => ({ ...r, width: 310, height: 230 })), ...frames, ...inkPoints];
      if (!boxes.length) return '<span class="board-preview-empty">Empty board</span>';
      const x = Math.min(...boxes.map(r => r.x)) - 55, y = Math.min(...boxes.map(r => r.y)) - 55;
      const width = Math.max(...boxes.map(r => r.x + r.width)) - x + 55, height = Math.max(...boxes.map(r => r.y + r.height)) - y + 55;
      const links = [...data.relations || [], ...data.visualLinks || []].map(edge => {
        const from = records.find(r => r.id === edge.from), to = records.find(r => r.id === edge.to);
        return from && to ? `<path class="board-preview-link" d="M${from.x + 155} ${from.y + 115} L${to.x + 155} ${to.y + 115}"/>` : '';
      }).join('');
      return `<svg class="board-preview-svg" viewBox="${x} ${y} ${width} ${height}" role="img" aria-label="${escape(label)}" data-preview-records="${records.length}">${frames.map(f => `<g class="board-preview-frame"><rect x="${f.x}" y="${f.y}" width="${f.width}" height="${f.height}" rx="8"/><text x="${f.x + 12}" y="${f.y + 25}">${escape(f.title)}</text></g>`).join('')}${links}${(data.strokes || []).map(stroke => `<path class="board-preview-ink" d="${escape(stroke.path)}"/>`).join('')}${records.map(r => `<svg x="${r.x}" y="${r.y}" width="310" height="230" viewBox="0 0 310 230" data-preview-record="${escape(r.id)}" class="board-preview-card ${cardClass(r)}"><rect x="2" y="2" width="306" height="226" rx="5"/><text x="15" y="32">${escape(kindNames[r.kind])}</text><text class="board-preview-title" x="15" y="70">${escape(r.title.slice(0, 34))}</text><text x="15" y="105">${escape(r.body.split('\n')[0].slice(0, 40))}</text></svg>`).join('')}</svg>`;
    },
    example: fixture,
    empty() {
      const data = fixture();
      return { ...data, records: [], frames: [], relations: [], visualLinks: [], runs: [], log: [], selection: [], panel: 'inspect', camera: { x: 70, y: 60, zoom: .7 } };
    },
    mount(root, { icon, notify, onEvent, storageKey = 'memento-atlas-board-v2', initialData, boardId = 'atlas-launch-plan', boardName = 'Atlas Launch', workspaceName = 'Atlas Launch' }) {
      let data = initialData ? copy(initialData) : fixture(), restored = false;
      try {
        const saved = JSON.parse(localStorage.getItem(storageKey) || 'null');
        if (saved?.version === 2 && Array.isArray(saved.records) && Array.isArray(saved.connections)) {
          restored = true;
          if (!saved.layoutVersion) {
            const legacyPositions = [[100, 130], [100, 390], [100, 700], [540, 130], [540, 410], [540, 730], [990, 130], [990, 430], [990, 740], [1440, 130], [1440, 460], [1440, 800], [1890, 130], [1890, 460]];
            const unchanged = saved.records.length === data.records.length && !saved.strokes.length
              && data.records.every((record, index) => saved.records.some(r => r.id === record.id && r.x === legacyPositions[index][0] && r.y === legacyPositions[index][1]))
              && saved.frames.length === 5 && saved.frames.every((frame, index) => frame.id === `frame-${index}` && frame.x === 60 + index * 450 && frame.y === 60 && frame.width === 410 && frame.height === 1080);
            if (unchanged) {
              saved.records.forEach(record => Object.assign(record, { x: data.records.find(r => r.id === record.id).x, y: data.records.find(r => r.id === record.id).y }));
              saved.frames.forEach((frame, index) => Object.assign(frame, { x: data.frames[index].x, y: data.frames[index].y, width: data.frames[index].width, height: data.frames[index].height }));
              restored = false;
            }
            saved.layoutVersion = 1;
          }
          data = saved;
        }
        else if (saved) notify('Saved board format is not supported. The example board has been loaded.');
      } catch (error) { notify(`Could not restore the local board: ${error.message}`); }
      let selected = data.selection || ['decision'], panel = data.panel || 'inspect', panelOpen = data.panelOpen !== false, mode = data.view || 'board', tool = 'select', search = '', gesture = null;
      data.visualLinks ||= [];
      data.messages ||= [];
      data.chatContext ||= [];
      data.agentSettings ||= { timeout: 120, toolBudget: 8 };
      if (panel === 'memory') panel = 'activity';
      mode = 'board';
      let searchOpen = false, chatDraft = '', chatRole = 'Facilitator', activityFilter = 'all';
      let noteColor = 'amber';
      let editingNote = null;
      let keyboardConnection = null;
      let undo = [], redo = [], conflict = null, dialogReturnFocus = null;
      const get = id => data.records.find(record => record.id === id);
      const uid = prefix => `${prefix}-${crypto.randomUUID()}`;
      const current = () => get(selected[selected.length - 1]);
      const connection = record => data.connections.find(item => item.id === record.connection);
      const allowed = () => data.role === 'Owner' || data.role === 'Editor';
      function canApprove(record) {
        const c = connection(record);
        return allowed() && ['Awaiting approval', 'Failed'].includes(record.status) && c.status === 'Connected' && c.write && record.target === c.scope && record.endpoint === c.endpoint && !record.evidence.some(ref => get(ref.id)?.kind === 'decision' && get(ref.id).status !== 'Current');
      }
      const btn = (label, action, id = '', style = 'quiet-button', disabled = false) => `<button type="button" class="${style}" data-atlas-action="${action}" data-id="${escape(id)}" ${disabled ? 'disabled' : ''}>${escape(label)}</button>`;
      const ibtn = (label, action, symbol, id = '') => `<button type="button" class="icon-button" title="${escape(label)}" aria-label="${escape(label)}" data-atlas-action="${action}" data-id="${escape(id)}">${icon(symbol)}</button>`;
      const badge = text => `<span class="atlas-badge">${escape(text)}</span>`;
      const recordStatus = record => ['note', 'text', 'shape'].includes(record.kind) ? '' : badge(record.status);
      function persist() {
        data.selection = selected; data.panel = panel; data.panelOpen = panelOpen; data.view = mode;
        try { localStorage.setItem(storageKey, JSON.stringify(data)); status('Saved locally. Demo only; no external service contacted.'); }
        catch (error) { status(`Changes are not saved: ${error.message}`); notify('Local storage failed. Keep this page open; changes are not durable.'); }
      }
      function status(message) { root.querySelector('.atlas-status-text')?.replaceChildren(document.createTextNode(message)); }
      function log(title, detail, category = 'collaboration', actor = 'Maya Chen') {
        const event = { id: uid('atlas-event'), title, detail, category, actor, time: 'Just now', board: boardId, workspace: workspaceName, read: false, event: title };
        data.log.unshift(event);
        onEvent(event);
      }
      function snapshot(id) {
        const r = get(id);
        return { id, revision: r.revision, title: r.title, body: r.body };
      }
      function changed(record) {
        record.revision += 1;
        for (const proposal of data.records.filter(r => r.kind === 'proposal' && ['Pending review', 'Stale'].includes(r.status))) {
          if (proposal.evidence?.some(ref => ref.id === record.id && ref.revision !== record.revision)) proposal.status = 'Stale';
        }
      }
      function visualState() { return copy({ records: data.records.map(({ id, x, y, placed, color }) => ({ id, x, y, placed, color: color || 'amber' })), frames: data.frames, strokes: data.strokes, visualLinks: data.visualLinks }); }
      function visualCheckpoint() { undo.push(visualState()); redo = []; }
      function freePosition(anchorId) {
        const anchor = get(anchorId) || current() || get('goal') || { x: 100, y: 100 };
        for (let ring = 1; ; ring += 1) {
          for (const [dx, dy] of [[1, 0], [0, 1], [1, 1], [-1, 0], [0, -1], [-1, 1], [1, -1], [-1, -1]]) {
            const x = anchor.x + dx * 370 * ring, y = anchor.y + dy * 330 * ring;
            if (!data.records.some(r => r.placed && x < r.x + 340 && x + 340 > r.x && y < r.y + 300 && y + 300 > r.y)
              && !data.frames.some(f => x < f.x + f.width && x + 340 > f.x && y < f.y + 45 && y + 300 > f.y)) return { x, y };
          }
        }
      }
      function restoreVisual(stack, other) {
        const entry = stack.pop();
        if (!entry) { notify('No visual changes to restore.'); return; }
        other.push(visualState());
        const ids = new Set(entry.records.map(r => r.id));
        for (const r of data.records) {
          const saved = entry.records.find(item => item.id === r.id);
          if (saved) Object.assign(r, saved);
          else if (!ids.has(r.id)) r.placed = false;
        }
        data.frames = entry.frames; data.strokes = entry.strokes; data.visualLinks = entry.visualLinks;
        persist(); render();
        notify('Visual layout restored. Decisions, approvals and external operations are unchanged.');
      }
      function recordCard(r, index) {
        const p = r;
        const cls = cardClass(r);
        const stale = r.connection && connection(r)?.status !== 'Connected';
        const outsideScope = r.sourceScope && (r.sourceScope !== connection(r)?.scope || r.sourceEndpoint !== connection(r)?.endpoint);
        return `<article class="flow-node ${cls} ${r.status === 'Superseded' ? 'atlas-superseded' : ''}" style="left:${p.x}px;top:${p.y}px;width:310px;min-height:190px" data-record="${escape(r.id)}" role="button" tabindex="0" aria-label="${escape(kindNames[r.kind] + ': ' + r.title)}" aria-pressed="${selected.includes(r.id)}">
          <div class="flow-node-header"><span class="flow-grip">${icon('grip')}</span><span>${escape(kindNames[r.kind])} / r${r.revision}</span>${ibtn('Actions for ' + r.title, 'context-menu', 'more', r.id)}</div>
          <div class="flow-node-content">${r.kind === 'note' ? `<div class="atlas-note-text" data-note-text="${escape(r.id)}">${escape(noteText(r))}</div>` : `<strong>${escape(r.title)}</strong><p>${escape(r.body.slice(0, 140))}</p>`}<div class="atlas-card-badges">${recordStatus(r)}${stale ? badge('Last known / connection offline') : ''}${outsideScope ? badge('Outside current connection scope') : ''}${!r.placed ? badge('Memory only') : ''}</div><div class="flow-node-meta">${escape(r.actor)}</div></div>
          <i class="flow-handle source-handle"></i><i class="flow-handle target-handle"></i><i class="flow-handle atlas-handle-top"></i><i class="flow-handle atlas-handle-bottom"></i></article>`;
      }
      function noteText(r) { return r.text ?? [r.title, r.body].filter(Boolean).join('\n\n'); }
      function startNoteEdit(id) {
        const r = get(id);
        if (!allowed()) { notify('Your role cannot edit this Note.'); return; }
        if (editingNote?.id === id) return;
        finishNoteEdit();
        if (!r.placed) { notify('Show this Note on the board before editing it.'); return; }
        selected = [id]; panel = 'inspect'; renderPanel();
        const el = root.querySelector(`[data-note-text="${id}"]`);
        if (!el) return;
        editingNote = { id, revision: r.revision, original: noteText(r) };
        renderPanel();
        el.contentEditable = 'plaintext-only';
        el.setAttribute('role', 'textbox'); el.setAttribute('aria-label', 'Note text'); el.setAttribute('aria-multiline', 'true');
        el.focus();
        const selection = window.getSelection(), range = document.createRange();
        range.selectNodeContents(el); selection.removeAllRanges(); selection.addRange(range);
      }
      function finishNoteEdit(cancel = false) {
        if (!editingNote) return;
        const edit = editingNote, r = get(edit.id), el = root.querySelector(`[data-note-text="${edit.id}"]`);
        const text = el?.innerText.replace(/\r\n/g, '\n') ?? edit.original;
        editingNote = null;
        if (el) { el.removeAttribute('contenteditable'); el.removeAttribute('role'); el.removeAttribute('aria-label'); el.removeAttribute('aria-multiline'); }
        if (cancel || text === edit.original) { if (el) el.textContent = noteText(r); return; }
        const draft = { text, title: text.split('\n').find(line => line.trim())?.trim().slice(0, 100) || 'Untitled Note', body: text };
        if (r.revision !== edit.revision) {
          if (el) el.textContent = noteText(r);
          conflict = { id: edit.id, draft, annotation: false };
          openDialog('Resolve concurrent edit', `<p>A newer accepted revision exists. Your draft has not overwritten it.</p><h3 class="atlas-modal-section">Accepted content</h3><p class="atlas-body">${escape(noteText(r))}</p><h3 class="atlas-modal-section">Your draft</h3><p class="atlas-body">${escape(text)}</p>${btn('Keep accepted version', 'conflict-keep')}${btn('Apply my draft as a new revision', 'conflict-apply')}`);
          return;
        }
        saveEdit(r, draft, false);
        if (el) {
          el.textContent = text;
          const card = el.closest('[data-record]');
          card.setAttribute('aria-label', `Note: ${r.title}`);
          card.querySelector('.flow-node-header > span:nth-child(2)').textContent = `Note / r${r.revision}`;
        }
        const inspector = panel === 'inspect' && current()?.id === r.id ? root.querySelector('.atlas-panel-content') : null;
        if (inspector) {
          inspector.querySelector('h2').textContent = r.title;
          inspector.querySelector('.atlas-body').textContent = r.body;
          inspector.querySelector('.eyebrow').textContent = `Note / r${r.revision}`;
        }
      }
      function visibleRecords() {
        return data.records.filter(r => r.placed && `${r.title} ${r.body} ${r.kind} ${r.status}`.toLowerCase().includes(search.toLowerCase()));
      }
      function colorPalette(action, color) {
        const label = action === 'new-note-color' ? 'New Note color' : 'Note color';
        return `<div class="note-palette" role="group" aria-label="${label}"><span>${label}</span>${Object.entries(noteColors).map(([id, label]) => `<button type="button" class="note-swatch flow-node-note note-${id}" data-atlas-action="${action}" data-id="${id}" aria-label="${label}" title="${label}" aria-pressed="${id === (color || 'amber')}"><span aria-hidden="true">${id === (color || 'amber') ? '✓' : ''}</span></button>`).join('')}<span>Color only, no semantic meaning</span></div>`;
      }
      let menuPosition = null, menuReturnFocus = null;
      function closeContextMenu() { document.querySelector('.atlas-context-menu')?.remove(); }
      function showContextMenu(id, x, y) {
        closeContextMenu(); finishNoteEdit();
        if (id) { if (!selected.includes(id)) selected = [id]; panel = 'inspect'; renderPanel(); }
        else { selected = []; renderPanel(); }
        const r = get(id), bounds = root.querySelector('.flow-viewport').getBoundingClientRect(), c = data.camera;
        menuPosition = { x: (x - bounds.left - c.x) / c.zoom, y: (y - bounds.top - c.y) / c.zoom };
        menuReturnFocus = document.activeElement;
        const menu = document.createElement('div');
        menu.className = 'atlas-context-menu'; menu.setAttribute('role', 'menu'); menu.setAttribute('aria-label', 'Board actions');
        const items = [['context-add-note', 'Add Note'], ...(r?.kind === 'note' ? [['edit', 'Edit Note']] : []), ['proposal', 'Submit Proposal from context'], ['decision', 'Record Decision from context'], ...(r ? [['inspect', 'Inspect details'], ['trace', 'View traceability']] : [])];
        menu.innerHTML = items.map(([name, label]) => `<button type="button" role="menuitem" data-command="${name}">${label}</button>`).join('');
        menu.style.left = Math.max(8, Math.min(x, innerWidth - 253)) + 'px';
        menu.style.top = Math.max(8, Math.min(y, innerHeight - items.length * 40 - 20)) + 'px';
        menu.addEventListener('click', event => {
          const button = event.target.closest('[data-command]'); if (!button) return;
          closeContextMenu();
          if (button.dataset.command === 'context-add-note') {
            if (!allowed()) { notify('Your role cannot add Notes.'); return; }
            visualCheckpoint();
            const note = { id: uid('note'), kind: 'note', title: 'Untitled Note', body: '', text: '', color: noteColor, actor: 'Maya Chen', revision: 1, history: [], placed: true, status: 'Current', ...menuPosition };
            data.records.push(note); selected = [note.id]; panel = 'inspect'; search = ''; log('Material added', 'Note'); persist(); render(); startNoteEdit(note.id);
          } else action(button.dataset.command, id);
        });
        menu.addEventListener('keydown', event => {
          const buttons = [...menu.querySelectorAll('button')], index = buttons.indexOf(document.activeElement);
          if (['ArrowDown', 'ArrowUp', 'Home', 'End'].includes(event.key)) {
            event.preventDefault(); buttons[event.key === 'Home' ? 0 : event.key === 'End' ? buttons.length - 1 : (index + (event.key === 'ArrowDown' ? 1 : -1) + buttons.length) % buttons.length].focus();
          }
          if (event.key === 'Escape' || event.key === 'Tab') { closeContextMenu(); menuReturnFocus?.focus(); }
        });
        document.body.append(menu); menu.querySelector('button').focus();
      }
      document.addEventListener('pointerdown', event => { if (!event.target.closest('.atlas-context-menu')) closeContextMenu(); });
      root.addEventListener('contextmenu', event => {
        if (!event.target.closest('.flow-viewport') || event.target.closest('[contenteditable]')) return;
        event.preventDefault(); showContextMenu(event.target.closest('[data-record]')?.dataset.record, event.clientX, event.clientY);
      });
      function render() {
        const presence = data.presence || [{ name: 'Maya Chen', initials: 'MC' }, { name: 'Leo Martin', initials: 'LM' }, { name: 'Sam Rivera', initials: 'SR' }];
        root.closest('.main-content').classList.add('board-content');
        root.closest('.app-frame').classList.add('board-app');
        root.innerHTML = `<div class="flow-board atlas-board">
          <header class="flow-toolbar"><div class="flow-board-title"><strong>${escape(boardName)}</strong></div>
            <div class="flow-history">${ibtn('Undo visual change', 'undo', 'undo')}${ibtn('Redo visual change', 'redo', 'redo')}${ibtn('Fit board to view', 'fit', 'fit')}<button type="button" class="icon-button" data-atlas-action="search-toggle" aria-label="Search board" aria-expanded="${searchOpen}" aria-controls="atlas-board-search">${icon('search')}</button><input id="atlas-board-search" class="atlas-board-search ${searchOpen ? 'is-open' : ''}" type="search" aria-label="Search board" placeholder="Search this board…" value="${escape(search)}" ${searchOpen ? '' : 'hidden'}></div>
            ${tool === 'note' ? colorPalette('new-note-color', noteColor) : ''}
            <div class="flow-toolbar-end"><span class="board-presence" aria-label="${escape(presence.map(person => person.name).join(', '))} / demo presence">${presence.map(person => `<i>${escape(person.initials)}</i>`).join('')}</span>${ibtn('Members and permissions', 'members', 'members')}<button type="button" class="icon-button atlas-panel-toggle" data-atlas-action="panel-toggle" aria-controls="atlas-context-panel" aria-expanded="${panelOpen}" aria-label="${panelOpen ? 'Hide side panel' : 'Show side panel'}" title="${panelOpen ? 'Hide side panel' : 'Show side panel'}">${icon('panel')}</button></div>
          </header>
          <div class="flow-board-layout"><section class="flow-viewport tool-${tool}" aria-label="${mode === 'memory' ? 'Memory chart' : 'Infinite Atlas Launch board'}" tabindex="0">
            <div class="flow-caption"><span>Shift-click to select context. Drag card connection points to link Notes.</span></div>
            <div class="flow-tool-rail" role="toolbar" aria-label="Board tools">${[['select', 'Select'], ['hand', 'Pan'], ...mode === 'board' ? [['pen', 'Pen'], ['erase', 'Erase'], ['note', 'Add note'], ['frame', 'Add frame'], ['text', 'Add text'], ['shape', 'Add shape']] : []].map(([id, label]) => `<button type="button" class="icon-button" data-atlas-action="tool" data-id="${id}" aria-label="${label}" title="${label}" aria-pressed="${tool === id}">${icon(id === 'hand' ? 'hand' : id)}</button>`).join('')}<i class="flow-tool-divider" aria-hidden="true"></i><button type="button" class="icon-button atlas-chat-shortcut" data-atlas-action="open-chat" aria-label="Open Agents chat" title="Open Agents chat">${icon('chat')}</button></div>
            <div class="flow-world"><svg class="flow-edges atlas-edges" aria-label="Typed relationships"><defs><marker id="atlas-arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="5" markerHeight="5" orient="auto-start-reverse"><path d="M0 0 10 5 0 10Z"/></marker></defs><g class="atlas-relationship-paths"></g><g class="atlas-ink"></g></svg>${mode === 'board' ? data.frames.map(f => `<div class="flow-frame" data-frame="${f.id}" style="left:${f.x}px;top:${f.y}px;width:${f.width}px;height:${f.height}px"><div class="flow-frame-heading"><span>${escape(f.title)}</span>${ibtn('Edit frame ' + f.title, 'frame-edit', 'pen', f.id)}</div></div>`).join('') : ''}${visibleRecords().map(recordCard).join('')}</div>
            <div class="flow-zoom-controls">${ibtn('Zoom out', 'zoom-out', 'minus')}<span class="flow-zoom-level"></span>${ibtn('Zoom in', 'zoom-in', 'plus')}</div>
          </section><aside id="atlas-context-panel" class="flow-inspector atlas-inspector" aria-label="Context panel" ${panelOpen ? '' : 'hidden'}><nav class="atlas-panel-tabs" aria-label="Context panels">${[['inspect', 'Inspector'], ['sources', 'Sources'], ['agents', 'Agents'], ['activity', 'Activity']].map(([id, label]) => `<button type="button" data-atlas-action="panel" data-id="${id}" aria-pressed="${panel === id}">${label}</button>`).join('')}</nav><div class="atlas-panel-content"></div></aside></div>
          <footer class="flow-status"><i></i><span class="atlas-status-text">Saved locally. Demo only; no external service contacted.</span><span class="flow-status-right">${data.records.filter(r => r.kind === 'decision' && r.status === 'Current').length} current decisions / ${data.records.filter(r => r.kind === 'action').length} actions</span></footer></div>`;
        root.querySelector('input[type="search"]')?.addEventListener('input', event => {
          search = event.target.value; refreshSearch();
        });
        root.querySelector('input[type="search"]')?.addEventListener('blur', event => { if (!search && !event.relatedTarget?.closest('[data-atlas-action="search-toggle"]')) toggleSearch(false); });
        root.querySelectorAll('[data-record]').forEach(el => {
          el.setAttribute('aria-description', 'Drag a connection point to another card to draw a visual connection.');
          el.querySelectorAll('.flow-handle').forEach(handle => { handle.tabIndex = 0; handle.setAttribute('role', 'button'); handle.setAttribute('aria-label', 'Start visual connection from ' + get(el.dataset.record).title); });
        });
        renderPanel(); applyCamera();
      }
      function refreshSearch() {
        const world = root.querySelector('.flow-world');
        world.querySelectorAll('[data-record]').forEach(el => el.remove());
        world.insertAdjacentHTML('beforeend', visibleRecords().map(recordCard).join(''));
        wireHandles(); drawEdges();
      }
      function wireHandles() {
        root.querySelectorAll('[data-record]').forEach(el => {
          el.querySelectorAll('.flow-handle').forEach(handle => { handle.tabIndex = 0; handle.setAttribute('role', 'button'); handle.setAttribute('aria-label', 'Start visual connection from ' + get(el.dataset.record).title); });
        });
      }
      function toggleSearch(open) {
        searchOpen = open;
        const input = root.querySelector('#atlas-board-search');
        input.hidden = !open; input.classList.toggle('is-open', open);
        root.querySelector('[data-atlas-action="search-toggle"]').setAttribute('aria-expanded', String(open));
        if (open) input.focus();
        else { search = ''; input.value = ''; refreshSearch(); }
      }
      function setPanelOpen(open) {
        panelOpen = open;
        const aside = root.querySelector('#atlas-context-panel'), toggle = root.querySelector('[data-atlas-action="panel-toggle"]');
        aside.hidden = !open;
        toggle.setAttribute('aria-expanded', String(open));
        toggle.setAttribute('aria-label', open ? 'Hide side panel' : 'Show side panel');
        toggle.title = toggle.getAttribute('aria-label');
        renderPanel();
      }
      function infoCard(action, id, title, meta, symbol = 'link') {
        return `<button type="button" class="atlas-list-button atlas-info-card" data-atlas-action="${action}" data-id="${escape(id)}"><span class="atlas-info-icon">${icon(symbol)}</span><span>${escape(title)}<small>${escape(meta)}</small></span>${icon('chevron')}</button>`;
      }
      function evidenceList(r) {
        return (r.evidence || []).map((ref, index) => infoCard('evidence', `${r.id}:${index}`, ref.title, `Pinned revision ${ref.revision}${get(ref.id)?.revision !== ref.revision ? ' / current content has changed' : ''}`)).join('') || '<p class="muted">No pinned evidence.</p>';
      }
      function renderPanel() {
        const target = root.querySelector('.atlas-panel-content');
        if (!target) return;
        target.classList.toggle('is-agent-chat', panel === 'agents');
        root.querySelectorAll('.atlas-panel-tabs [data-atlas-action="panel"]').forEach(button => {
          button.setAttribute('aria-pressed', String(button.dataset.id === panel));
        });
        const r = current();
        let html = '';
        if (panel === 'inspect') {
          html = r ? `<span class="eyebrow">${escape(kindNames[r.kind])} / r${r.revision}</span><h2>${escape(r.title)}</h2>${recordStatus(r)}<p class="atlas-body">${escape(r.body)}</p><p class="atlas-small">${escape(r.actor)} / ${r.placed ? 'On this board' : 'Preserved in memory'}</p>` : '<h2>Board context</h2><p>Select material to inspect it or ask an agent for help.</p>';
          if (selected.length > 1) html += `<div class="atlas-notice">${selected.length} records selected for context.</div>`;
          if (r) {
            if (r.sharedCapture) html += `<h3>Shared email capture / prototype</h3><p class="atlas-small">Shared by ${escape(r.sharedCapture.by)} with Atlas Launch members.<br>Captured revision: ${r.sharedCapture.revision}<br>From: ${escape(r.sharedCapture.from)}<br>Original: ${escape(r.externalId)}</p><p class="atlas-body">${escape(r.annotation || 'No local annotation.')}</p><div class="atlas-notice">Only this selected text was shared. No attachments, mailbox access, live refresh or permission to send as the owner. Personal sharing policy is a proposed extension.</div><div class="atlas-buttons">${btn('Inspect original', 'original', r.id)}${btn('Annotate', 'annotate', r.id)}${btn('Observation history', 'observations', r.id)}</div>`;
            else if (['source', 'task'].includes(r.kind)) html += `<h3>External observation</h3><p class="atlas-small">${escape(connection(r)?.provider)}<br>Scope: ${escape(connection(r)?.scope)}<br>External ID: ${escape(r.externalId)}<br>Last checked: ${escape(r.observed)}<br>Connection: ${escape(connection(r)?.status)}</p><p class="atlas-body">${escape(r.annotation || 'No local annotation.')}</p><div class="atlas-buttons">${btn('Inspect original', 'original', r.id)}${btn('Annotate', 'annotate', r.id)}${btn('Observation history', 'observations', r.id)}${btn('Refresh source', 'refresh', r.id)}</div>`;
            if (['note', 'text', 'shape', 'proposal'].includes(r.kind)) html += btn('Edit content', 'edit', r.id);
            if (r.kind === 'note') html += colorPalette('note-color', r.color);
            if (r.kind === 'note' && editingNote?.id === r.id) html += btn('Simulate concurrent edit', 'conflict', r.id);
            if (r.kind === 'proposal') html += `<h3>Review proposal</h3><div class="atlas-buttons">${btn('Adopt as working note', 'adopt', r.id, 'primary-button', !['Pending review'].includes(r.status))}${btn('Reject', 'reject', r.id, 'outline-button', !['Pending review', 'Stale'].includes(r.status))}${r.status === 'Stale' ? btn('Request fresh assessment', 'reassess', r.id) : ''}${btn('Record a separate decision', 'decision', r.id)}</div><p class="atlas-small">Adopting creates editable material. It does not record a team decision.</p>`;
            if (r.kind === 'decision') html += `<h3>Decision history</h3>${r.supersedes ? btn('Previous decision', 'focus', r.supersedes) : ''}${r.replacedBy ? btn('Replacement decision', 'focus', r.replacedBy) : btn('Supersede decision', 'supersede', r.id)}<p class="atlas-small">Accepted meaning is immutable. A replacement is a new human-recorded decision.</p>`;
            if (r.kind === 'action') html += `<h3>Reviewed operation</h3><p class="atlas-small">${escape(r.operation)} / revision ${r.revision}<br>Target: ${escape(r.target)}<br>${r.approval ? `Approved by ${escape(r.approval.actor)} for r${r.approval.revision}` : 'No current approval'}</p><div class="atlas-buttons">${btn('Review operation', 'review', r.id)}${btn('Edit proposed content', 'edit-action', r.id, 'outline-button', ['Succeeded', 'In execution', 'Uncertain'].includes(r.status))}${r.status === 'Approved' ? btn('Execute approved operation', 'execute', r.id, 'primary-button') : ''}${r.status === 'In execution' ? btn('Inspect execution', 'execution', r.id) : ''}${r.status === 'Uncertain' ? btn('Reconcile external outcome', 'reconcile', r.id, 'primary-button') : ''}${r.result ? btn('Inspect resulting record', 'focus', r.result) : ''}</div>`;
            html += `<h3>Pinned evidence</h3>${evidenceList(r)}<h3>Related records</h3>${data.relations.filter(edge => edge.from === r.id || edge.to === r.id).map(edge => { const other = edge.from === r.id ? edge.to : edge.from; return infoCard('focus', other, get(other)?.title || other, `${relationNames[edge.type] || edge.type} ${edge.from === r.id ? 'to' : 'from'}`, 'arrow'); }).join('') || '<p class="muted">No relationships yet.</p>'}`;
            if (r.kind === 'decision' && r.status === 'Current') html += `<div class="atlas-buttons">${btn('Prepare follow-up', 'follow-up', r.id)}</div>`;
            if (!['action', 'decision'].includes(r.kind)) html += `<div class="atlas-buttons">${btn('Record Decision from context', 'decision', r.id)}${r.kind !== 'proposal' ? btn('Submit Proposal from context', 'proposal', r.id) : ''}</div>`;
            if (r.kind === 'action' && ['Awaiting approval', 'Failed'].includes(r.status)) html += btn('Require another action first', 'prerequisite', r.id);
            html += `<div class="atlas-buttons">${btn('Ask agent about selection', 'ask-selection')}${btn('View traceability', 'trace', r.id)}${r.placed ? btn('Remove from board', 'remove', r.id, 'outline-button') : btn('Show on board', 'restore', r.id)}</div><p class="atlas-small">Removing a placement never deletes its evidence or reverses an external operation.</p>`;
          }
          if (!r) html += btn('View traceability', 'trace');
          if (selected.length) html += `<div class="atlas-selection-actions"><strong>${selected.length} selected</strong>${btn('Ask agent', 'ask-selection')}${btn('Submit Proposal', 'proposal')}${btn('Record Decision', 'decision')}</div>`;
          html += `<section class="flow-inspector-activity"><h3>Recent activity ${btn('View all', 'panel', 'activity', 'text-button')}</h3><div class="atlas-small">${escape(boardName)} only</div>${data.log.slice(0, 3).map(event => `<p><span><strong>${escape(event.title)}</strong><small>${escape(event.actor)} / ${escape(event.time)}</small></span></p>`).join('')}</section>`;
        } else if (panel === 'sources') {
          html = '<span class="eyebrow">Workspace connections</span><h2>Sources & services</h2><p>Read scoped external records into this board. Writes are separate, reviewed actions.</p>';
          html += data.connections.map(c => `<section class="atlas-service"><h3>${escape(c.name)} ${badge(c.status)}</h3><p class="atlas-small">Provider: ${escape(c.provider)}<br>Service URL (illustrative): ${escape(c.endpoint)}<br>Independent client: ${escape(c.client)}<br>${escape(c.transport)}<br>Authorization: ${escape(c.auth)} (demo)<br>Account: ${escape(c.account)}<br>Scope: ${escape(c.scope)}<br>Read: ${c.read ? 'allowed' : 'off'} / Write: ${c.write ? 'requires approval' : 'off'}<br>Last checked: ${escape(c.checked)}<br>New arrivals: ${c.automatic ? 'place automatically on this board' : 'manual import'}</p><div class="atlas-buttons">${btn('Configure connection', 'connection', c.id)}${btn('Browse scoped records', 'browse', c.id, 'quiet-button', c.status !== 'Connected' || !c.read)}${btn('Simulate scoped arrival', 'arrive', c.id, 'quiet-button', c.status !== 'Connected' || !c.read)}${btn(c.status === 'Connected' ? 'Simulate outage' : 'Reconnect', 'connection-toggle', c.id)}</div></section>`).join('');
          html += '<div class="atlas-notice">Local simulation. No credentials are stored and no network calls are made. A real connector would keep credentials server-side and enforce workspace, resource scope and capabilities for every operation.</div>';
        } else if (panel === 'agents') {
          const initials = role => role.split(' ').map(word => word[0]).join('');
          const run = data.runs[0];
          const runLabel = run && (run.awaitingReply ? 'Waiting for your reply' : run.status === 'Working' ? 'Working' : run.status);
          const icon = path => `<svg viewBox="0 0 24 24" aria-hidden="true"><path d="${path}"/></svg>`;
          html = `<header class="atlas-chat-heading"><h2>Agents</h2><span>Demo</span></header>
            <div class="atlas-conversation">
              <div class="atlas-chat" role="log" aria-label="Agent conversation">${data.messages.map(message => {
                const user = message.actor === 'Maya Chen';
                return `<section class="atlas-chat-message${user ? ' is-user' : ''}">${user ? '' : `<i class="atlas-chat-avatar" aria-hidden="true">${escape(initials(message.actor))}</i>`}<div><strong${user ? ' class="sr-only"' : ''}>${escape(message.actor)}</strong><p>${escape(message.body)}</p><div class="atlas-chat-refs">${(message.refs || []).map(ref => btn(`${ref.title} · r${ref.revision}`, 'capture', `${ref.id}:${ref.revision}`, 'atlas-chip')).join('')}</div></div></section>`;
              }).join('') || `<div class="atlas-chat-empty"><strong>How can I help with this board?</strong><span>Select cards and attach them to share context.</span></div>`}</div>
              ${run ? `<section class="atlas-chat-run is-${run.awaitingReply ? 'waiting' : run.status.toLowerCase()}" aria-label="${escape(run.role)} run">
                <div class="atlas-run-status"><i class="atlas-run-dot" aria-hidden="true"></i><span>${escape(run.role)} · ${escape(runLabel)}</span>
                  ${run.output ? btn('Inspect contribution', 'focus', run.output, 'atlas-chip') : ''}
                  ${run.status === 'Working' ? `<button class="atlas-icon-action" type="button" data-atlas-action="run-cancel" data-id="${run.id}" aria-label="Cancel run" title="Cancel run">${icon('M6 6l12 12M18 6L6 18')}</button>` : ''}
                </div>
                ${run.status === 'Working' && !run.awaitingReply ? '<progress class="atlas-run-progress" aria-label="Agent Run in progress"></progress>' : ''}
                <details class="atlas-run-details"><summary>Details</summary><p>${escape(run.request)}</p><small>${run.limits ? `${run.limits.timeout}s timeout · ${run.limits.toolBudget} tool calls maximum` : 'Limits not captured'}</small>${btn('Inspect run context', 'run-context', run.id, 'text-button')}
                  ${run.status === 'Working' ? `<details class="atlas-demo-controls"><summary>Demo controls</summary><div class="atlas-buttons">${btn('Simulate clarification', 'run-question', run.id)}${btn('Complete demo run', 'run-complete', run.id, 'quiet-button', !!run.awaitingReply)}${btn('Fail demo run', 'run-fail', run.id)}</div></details>` : ''}
                </details>
              </section>` : ''}
            </div>
            <form id="atlas-agent-form">
              ${data.chatContext.length ? `<div class="atlas-chat-context" aria-label="Context Selection"><div class="atlas-chat-refs">${data.chatContext.map(ref => btn(`${ref.title} · r${ref.revision}`, 'capture', `${ref.id}:${ref.revision}`, 'atlas-chip')).join('')}</div><button class="atlas-icon-action" type="button" data-atlas-action="clear-context" aria-label="Clear context" title="Clear context">${icon('M6 6l12 12M18 6L6 18')}</button></div>` : ''}
              <textarea name="request" aria-label="Message" required rows="1" placeholder="Ask the ${escape(chatRole)}…">${escape(chatDraft)}</textarea>
              <div class="atlas-composer-tools">
                <button class="atlas-icon-action" type="button" data-atlas-action="attach-context" aria-label="Attach selection" title="${selected.length ? `Attach ${selected.length} selected` : 'Select cards to attach'}" ${selected.length ? '' : 'disabled'}>${icon('M21 11.5l-8.6 8.6a5 5 0 01-7.1-7.1l8.6-8.6a3.3 3.3 0 014.7 4.7l-8.6 8.6a1.7 1.7 0 01-2.4-2.4l7.9-7.9')}</button>
                <select name="role" aria-label="Agent Role"><option ${chatRole === 'Facilitator' ? 'selected' : ''}>Facilitator</option><option ${chatRole === 'Action Coordinator' ? 'selected' : ''}>Action Coordinator</option></select>
                <button class="primary-button atlas-send" type="submit" aria-label="Send" title="Send">${icon('M12 19V5M5 12l7-7 7 7')}</button>
              </div>
            </form>`;
        } else {
          const activityLabels = { all: 'All', collaboration: 'Collaboration', sources: 'Sources', agent: 'Agent Runs', decisions: 'Decisions', actions: 'Actions' };
          const activityCount = category => data.log.filter(event => category === 'all' || event.category === category).length;
          const filterChips = Object.entries(activityLabels).map(([category, label]) => `<button type="button" class="atlas-filter-chip" role="radio" aria-checked="${activityFilter === category}" data-atlas-action="activity-filter" data-id="${category}">${escape(label)}<span class="atlas-filter-count">${activityCount(category)}</span></button>`).join('');
          const events = data.log.filter(event => activityFilter === 'all' || event.category === activityFilter);
          html = `<span class="eyebrow">${escape(boardName)} only</span><div class="atlas-activity-head"><h2>Activity</h2><button type="button" class="outline-button atlas-trace-button" data-atlas-action="trace" data-id="">${icon('link')}<span>View traceability</span></button></div><p>One timeline for contributions, Decisions, Agent Runs and Action outcomes. Open record details for revision history.</p><div class="atlas-filter-chips" role="radiogroup" aria-label="Filter activity">${filterChips}</div>${events.length ? `<ol class="atlas-timeline">${events.map(event => `<li class="atlas-timeline-item" data-category="${escape(event.category)}"><strong>${escape(event.title)}</strong><p>${escape(event.detail)}</p><small>${escape(event.actor)} / ${escape(event.time)}</small></li>`).join('')}</ol>` : '<p class="atlas-timeline-empty">No events in this filter.</p>'}`;
        }
        target.innerHTML = html;
        if (panel === 'agents') {
          const conversation = target.querySelector('.atlas-conversation');
          conversation.scrollTop = conversation.scrollHeight;
        }
        target.querySelector('#atlas-agent-form')?.addEventListener('submit', startRun);
        const composer = target.querySelector('textarea[name="request"]');
        const fit = () => { composer.style.height = 'auto'; composer.style.height = `${Math.min(composer.scrollHeight, 140)}px`; };
        composer?.addEventListener('input', event => { chatDraft = event.target.value; fit(); });
        composer?.addEventListener('keydown', event => { if (event.key === 'Enter' && !event.shiftKey && !event.isComposing) { event.preventDefault(); event.target.form.requestSubmit(); } });
        if (composer) fit();
        target.querySelector('select[name="role"]')?.addEventListener('change', event => { chatRole = event.target.value; composer.placeholder = `Ask the ${chatRole}…`; });
        persist();
      }
      function applyCamera() {
        const world = root.querySelector('.flow-world'), camera = data.camera;
        if (!world) return;
        world.style.transform = `translate(${camera.x}px,${camera.y}px) scale(${camera.zoom})`;
        root.querySelector('.flow-zoom-level').textContent = `${Math.round(camera.zoom * 100)}%`;
        drawEdges();
      }
      function drawEdges() {
        const nodes = new Map([...root.querySelectorAll('[data-record]')].map(el => [el.dataset.record, el]));
        const links = mode === 'memory' ? data.relations : [...data.relations, ...data.visualLinks];
        root.querySelector('.atlas-relationship-paths').innerHTML = links.filter(e => nodes.has(e.from) && nodes.has(e.to)).map(edge => {
          const a = nodes.get(edge.from), b = nodes.get(edge.to);
          const ax = a.offsetLeft + a.offsetWidth / 2, ay = a.offsetTop + a.offsetHeight / 2;
          const bx = b.offsetLeft + b.offsetWidth / 2, by = b.offsetTop + b.offsetHeight / 2;
          const vertical = Math.abs(by - ay) > Math.abs(bx - ax), direction = Math.sign(vertical ? by - ay : bx - ax) || 1;
          const sx = ax + (vertical ? 0 : direction * a.offsetWidth / 2), sy = ay + (vertical ? direction * a.offsetHeight / 2 : 0);
          const tx = bx - (vertical ? 0 : direction * b.offsetWidth / 2), ty = by - (vertical ? direction * b.offsetHeight / 2 : 0);
          const bend = direction * Math.max(70, Math.abs(vertical ? ty - sy : tx - sx) * .4);
          const path = vertical ? `M${sx} ${sy} C${sx} ${sy + bend},${tx} ${ty - bend},${tx} ${ty}` : `M${sx} ${sy} C${sx + bend} ${sy},${tx - bend} ${ty},${tx} ${ty}`;
          return `<g data-relation="${edge.id}" class="atlas-edge-${edge.type}"><path d="${path}" marker-end="url(#atlas-arrow)"/><text x="${(sx + tx) / 2}" y="${(sy + ty) / 2 - 12}">${escape(edge.type === 'visual' ? edge.label || 'Visual connection' : relationNames[edge.type] || edge.type)}</text></g>`;
        }).join('');
        root.querySelector('.atlas-ink').innerHTML = data.strokes.map(stroke => `<path data-stroke="${stroke.id}" d="${stroke.path}" class="atlas-stroke"/>`).join('');
      }
      function focusRecord(id) {
        const r = get(id);
        if (!r) { notify('The referenced record is unavailable.'); return; }
        selected = [id]; panel = 'inspect'; search = '';
        closeDialog();
        if (!r.placed) { persist(); render(); recordDetail(id); return; }
        render();
        const el = root.querySelector(`[data-record="${id}"]`);
        data.camera.x = 120 - el.offsetLeft * data.camera.zoom;
        data.camera.y = 90 - el.offsetTop * data.camera.zoom;
        applyCamera(); persist();
      }
      function recordDetail(id) {
        const r = get(id);
        if (!r) { notify('This record is unavailable.'); return; }
        openDialog(`${kindNames[r.kind]} details`, `<section class="atlas-service atlas-record-summary" data-kind="${r.kind}"><div class="atlas-trace-head"><span class="eyebrow">${escape(kindNames[r.kind])} / r${r.revision}</span>${recordStatus(r)}</div><h3>${escape(r.title)}</h3><p class="atlas-body">${escape(r.body)}</p><p class="atlas-small">${escape(r.actor)}${r.placed ? ' / On this board' : ' / Not placed on this board. Its record and evidence remain preserved.'}</p></section><div class="atlas-buttons">${r.placed ? btn('Locate on board', 'focus', id) : btn('Show on board', 'restore', id)}${btn('Follow traceability', 'trace', id)}</div><h3 class="atlas-modal-section">Revision history</h3>${r.history.length ? `<ol class="atlas-timeline">${r.history.map(ref => `<li class="atlas-timeline-item"><strong>r${ref.revision} / ${escape(ref.title)}</strong><p class="atlas-body">${escape(ref.body)}</p></li>`).join('')}</ol>` : '<p class="atlas-small">No earlier content revisions.</p>'}`);
        const actions = document.createElement('div');
        actions.className = 'atlas-buttons';
        actions.innerHTML = ['note', 'proposal', 'source', 'task'].includes(r.kind) ? btn('Record Decision from context', 'decision', id) : '';
        if (['note', 'text', 'shape'].includes(r.kind)) actions.innerHTML += btn('Edit content', 'edit', id);
        if (r.kind === 'proposal') actions.innerHTML += btn('Adopt as working Note', 'adopt', id, 'quiet-button', r.status !== 'Pending review') + btn('Reject Proposal', 'reject', id, 'quiet-button', !['Pending review', 'Stale'].includes(r.status));
        if (r.kind === 'decision' && r.status === 'Current') actions.innerHTML += btn('Supersede Decision', 'supersede', id) + btn('Prepare follow-up', 'follow-up', id);
        actions.addEventListener('click', event => { const button = event.target.closest('[data-atlas-action]'); if (button) action(button.dataset.atlasAction, button.dataset.id); });
        document.querySelector('#atlas-dialog-form .atlas-modal-body').append(actions);
      }
      function traceDialog(id = '') {
        const ids = new Set(id ? [id] : data.records.map(r => r.id));
        let changed = true;
        while (changed) {
          changed = false;
          for (const edge of data.relations) if (ids.has(edge.from) || ids.has(edge.to)) {
            for (const endpoint of [edge.from, edge.to]) if (!ids.has(endpoint)) { ids.add(endpoint); changed = true; }
          }
          for (const record of data.records.filter(r => ids.has(r.id))) for (const ref of record.evidence || []) if (!ids.has(ref.id)) { ids.add(ref.id); changed = true; }
        }
        const traceRecords = data.records.filter(r => ids.has(r.id));
        openDialog('Traceability', `<p class="atlas-modal-lead">Information and preserved Evidence References, not another board. Inspect a captured revision or locate a current card without changing its placement.</p>${traceRecords.map(r => {
          const links = data.relations.filter(edge => edge.from === r.id).map(edge => `<li>${icon('arrow')}<span><b>${escape(relationNames[edge.type] || edge.type)}</b> ${escape(get(edge.to)?.title || 'Unavailable record')}</span></li>`).join('');
          const pins = (r.evidence || []).map((ref, index) => `<button type="button" class="atlas-chip" data-atlas-action="evidence" data-id="${escape(r.id)}:${index}">${escape(ref.title)} / r${ref.revision}</button>`).join('');
          return `<section class="atlas-service atlas-trace-item" data-kind="${r.kind}"><div class="atlas-trace-head"><span class="eyebrow">${escape(kindNames[r.kind])}</span>${recordStatus(r)}</div><h3>${escape(r.title)}</h3><p class="atlas-body atlas-clamp">${escape(r.body)}</p>${pins ? `<div class="atlas-trace-pins"><span>Pinned evidence</span>${pins}</div>` : ''}${links ? `<ul class="atlas-trace-links">${links}</ul>` : ''}<div class="atlas-buttons">${btn('View details', 'detail', r.id)}${r.placed ? btn('Locate on board', 'focus', r.id) : '<span class="atlas-small">Not placed on board</span>'}</div></section>`;
        }).join('') || '<p>No recorded Proposals, Decisions or Actions yet. Select a Note to inspect its context.</p>'}`, null, { wide: true, eyebrow: `${traceRecords.length} linked records` });
      }
      function openDialog(title, content, submit, options = {}) {
        const overlay = document.querySelector('#overlay-root');
        dialogReturnFocus = document.activeElement;
        overlay.innerHTML = `<div class="modal-backdrop"><section class="modal atlas-modal${options.wide ? ' is-wide' : ''}" role="dialog" aria-modal="true" aria-labelledby="atlas-modal-title"><div class="modal-head"><div class="atlas-modal-heading">${options.eyebrow ? `<span class="eyebrow">${escape(options.eyebrow)}</span>` : ''}<h2 id="atlas-modal-title">${escape(title)}</h2></div>${ibtn('Close dialog', 'close-dialog', 'close')}</div><form id="atlas-dialog-form"><div class="atlas-modal-body">${content}<p class="atlas-form-error" role="alert"></p></div>${submit ? '<div class="modal-actions"><button type="button" class="quiet-button" data-atlas-action="close-dialog">Cancel</button><button class="primary-button" type="submit">Save</button></div>' : ''}</form></section></div>`;
        overlay.querySelector('[data-atlas-action="close-dialog"]').addEventListener('click', closeDialog);
        overlay.querySelector('.modal-backdrop').addEventListener('click', event => { if (event.target === event.currentTarget) closeDialog(); });
        overlay.querySelectorAll('[data-atlas-action]').forEach(button => {
          if (button.dataset.atlasAction !== 'close-dialog') button.addEventListener('click', () => action(button.dataset.atlasAction, button.dataset.id));
          else button.onclick = closeDialog;
        });
        overlay.querySelector('form').addEventListener('submit', event => {
          event.preventDefault();
          if (submit) submit(new FormData(event.currentTarget));
        });
        overlay.querySelector('input,textarea,select,button')?.focus();
      }
      function closeDialog() { document.querySelector('#overlay-root').innerHTML = ''; if (dialogReturnFocus?.isConnected) dialogReturnFocus.focus(); }
      function formError(message) { document.querySelector('.atlas-form-error').textContent = message; }
      const field = (label, name, value = '', type = 'text') => `<label>${escape(label)}<input name="${name}" type="${type}" value="${escape(value)}" required></label>`;
      const area = (label, name, value = '') => `<label>${escape(label)}<textarea name="${name}" required rows="3">${escape(value)}</textarea></label>`;
      function editRecord(id, annotation = false) {
        const r = get(id), revision = annotation ? r.annotationRevision || 0 : r.revision;
        if (r.kind === 'note' && !annotation) { closeDialog(); startNoteEdit(id); return; }
        if (!allowed()) { notify('Your role cannot edit this record.'); return; }
        openDialog(annotation ? 'Local annotation' : 'Edit working content', `${annotation ? area('Annotation (does not change the external record)', 'body', r.annotation || '') : field('Title', 'title', r.title) + area('Content', 'body', r.body)}<p class="atlas-small">Editing revision ${revision}. Accepted decisions and external observations cannot be overwritten.</p>${btn('Preview concurrent edit', 'conflict', id)}`, form => {
          const draft = { title: annotation ? r.title : form.get('title').trim(), body: form.get('body').trim() };
          if (!draft.title || !draft.body) { formError('Title and content cannot be blank.'); return; }
          if ((annotation ? r.annotationRevision || 0 : r.revision) !== revision) {
            conflict = { id, draft, annotation };
            openDialog('Resolve concurrent edit', `<p>A newer accepted revision exists. Nothing has been overwritten.</p><h3 class="atlas-modal-section">Accepted content</h3><p>${escape(annotation ? r.annotation : r.body)}</p><h3 class="atlas-modal-section">Your draft</h3><p>${escape(draft.body)}</p>${btn('Keep accepted version', 'conflict-keep')}${btn('Apply my draft as a new revision', 'conflict-apply')}`);
            return;
          }
          saveEdit(r, draft, annotation); closeDialog(); render();
        });
      }
      function saveEdit(r, draft, annotation) {
        if (annotation) { r.annotation = draft.body; r.annotationRevision = (r.annotationRevision || 0) + 1; }
        else { r.history.push(snapshot(r.id)); r.title = draft.title; r.body = draft.body; if (r.kind === 'note' && draft.text !== undefined) r.text = draft.text; changed(r); }
        log(annotation ? 'Local annotation revised' : 'Content revised', `${r.title} / revision ${annotation ? r.annotationRevision : r.revision}.`); persist();
      }
      function decisionDialog(replaces = '', basedOn = '') {
        const earlier = get(replaces);
        const evidenceIds = [...new Set([...selected, basedOn].filter(id => get(id) && get(id).kind !== 'action'))];
        openDialog(replaces ? 'Record a superseding Decision' : 'Record a Decision', `${field('Chosen direction', 'title', earlier?.title || get(basedOn)?.title || '')}${area('Rationale', 'body')}<h3 class="atlas-modal-section">Evidence References</h3>${evidenceIds.map(id => `<label class="atlas-checkbox"><input type="checkbox" name="evidence" value="${id}" checked>${escape(get(id).title)} / r${get(id).revision}</label>`).join('')}<label class="atlas-checkbox"><input type="checkbox" name="place">Show the Decision on the board</label><p class="atlas-small">${earlier ? `Replaces only "${escape(earlier.title)}". Its meaning and evidence remain available.` : 'Notes remain editable and unchanged. A Proposal is optional, never a required stage.'}</p>`, form => {
          if (!allowed()) { formError('Only workspace members can record decisions.'); return; }
          if (earlier && earlier.status !== 'Current') { formError('This decision has already been superseded. Inspect its replacement.'); return; }
          const title = form.get('title').trim(), body = form.get('body').trim(), refs = form.getAll('evidence');
          if (!title || !body || !refs.length) { formError('Provide a direction, rationale and at least one evidence reference.'); return; }
          const r = { id: uid('decision'), kind: 'decision', title, body, actor: 'Maya Chen', revision: 1, status: 'Current', evidence: refs.map(snapshot), history: [], placed: form.has('place'), ...freePosition(replaces || basedOn) };
          if (earlier) {
            earlier.status = 'Superseded'; earlier.replacedBy = r.id; r.supersedes = earlier.id;
            data.records.filter(record => record.kind === 'proposal' && record.status === 'Pending review' && record.evidence?.some(ref => ref.id === earlier.id)).forEach(record => { record.status = 'Stale'; });
            data.records.filter(record => record.kind === 'action' && record.status === 'Approved' && record.evidence?.some(ref => ref.id === earlier.id)).forEach(record => { record.approval = null; record.status = 'Awaiting approval'; });
            data.relations.push({ id: uid('edge'), from: r.id, to: earlier.id, type: 'supersedes' });
          }
          data.records.push(r);
          const adopted = get(basedOn);
          if (adopted?.kind === 'proposal' && adopted.status === 'Pending review') {
            adopted.status = 'Adopted';
            log('Proposal adopted', `Explicitly selected as the basis of ${title}.`, 'agent');
          }
          refs.forEach(id => data.relations.push({ id: uid('edge'), from: id, to: r.id, type: 'supports' }));
          log(earlier ? 'Decision superseded' : 'Decision recorded', title, 'decisions');
          closeDialog(); persist(); focusRecord(r.id);
        });
      }
      function proposalDialog(id = '') {
        const refs = [...new Set([...selected, id])].filter(id => get(id));
        openDialog('Submit a Proposal', `${field('Proposed option', 'title', get(id)?.title || '')}${area('Explanation', 'body')}<p>${refs.length} selected context revisions will be referenced. Existing Notes remain unchanged.</p><label class="atlas-checkbox"><input type="checkbox" name="place">Show the Proposal on the board</label>`, form => {
          if (!allowed()) { formError('Only workspace Members can submit a Proposal.'); return; }
          const title = form.get('title').trim(), body = form.get('body').trim();
          if (!title || !body) { formError('Provide the option and explanation.'); return; }
          const r = { id: uid('proposal'), kind: 'proposal', title, body, actor: 'Maya Chen', revision: 1, history: [], evidence: refs.map(snapshot), status: 'Pending review', placed: form.has('place'), ...freePosition(id) };
          data.records.push(r); refs.forEach(from => data.relations.push({ id: uid('edge'), from, to: r.id, type: 'supports' }));
          log('Proposal submitted', title, 'collaboration'); persist(); focusRecord(r.id);
        });
      }
      function actionDialog(id = '', runId = '') {
        const r = get(id);
        if (r && ['Succeeded', 'In execution', 'Uncertain'].includes(r.status)) { notify('This submitted operation cannot be edited. Inspect or reconcile its result.'); return; }
        openDialog(r ? 'Revise proposed action' : 'Prepare an external action', `<label>Operation<select name="operation"><option value="create-task" ${r?.operation === 'create-task' ? 'selected' : ''}>Create groupware task</option><option value="send-reply" ${r?.operation === 'send-reply' ? 'selected' : ''}>Send email reply</option></select></label>${field('Title', 'title', r?.title || 'Confirm the briefing arrangements')}${area('Task content or email body', 'body', r?.body || 'Confirm the room for the 11:00 briefing.')}<fieldset class="atlas-operation-fields" data-operation="create-task">${field('Task owner', 'owner', r?.owner || 'Leo Martin')}${field('Task due date', 'due', r?.due || '2026-10-26', 'date')}</fieldset><fieldset class="atlas-operation-fields" data-operation="send-reply">${field('Reply recipients', 'recipients', r?.recipients || 'jordan@example.test', 'email')}${field('Reply subject', 'subject', r?.subject || 'Re: Atlas briefing time')}</fieldset><label>Related decision<select name="decision">${data.records.filter(r => r.kind === 'decision' && r.status === 'Current').map(d => `<option value="${d.id}" ${r?.evidence?.some(e => e.id === d.id) ? 'selected' : ''}>${escape(d.title)}</option>`).join('')}</select></label><p class="atlas-small">Replies use the configured mailbox identity and scoped Atlas thread. Tasks target the configured Atlas Deck stack. Saving or revising never sends; revision invalidates prior approval.</p>`, form => {
          if (!allowed()) { formError('Only a workspace member can propose an operation.'); return; }
          const operation = form.get('operation'), c = data.connections.find(c => c.id === (operation === 'create-task' ? 'deck' : 'mail'));
          if (!c.write) { formError('Writing is disabled for this connection. Configure its capabilities first.'); return; }
          if (!get(form.get('decision'))) { formError('Select a current decision.'); return; }
          const title = form.get('title').trim(), body = form.get('body').trim();
          if (!title || !body) { formError('Title and content cannot be blank.'); return; }
          const record = r || { id: uid('action'), kind: 'action', actor: 'Maya Chen', revision: 0, history: [], placed: true, ...freePosition(form.get('decision')) };
          Object.assign(record, { title, body, operation, connection: c.id, target: c.scope, endpoint: c.endpoint, sender: c.account, recipients: form.get('recipients'), subject: form.get('subject'), owner: form.get('owner'), due: form.get('due'), approval: null, status: 'Awaiting approval', evidence: [snapshot(form.get('decision'))] });
          changed(record);
          if (!r) data.records.push(record);
          if (runId) {
            const run = data.runs.find(run => run.id === runId);
            run.output = record.id;
            record.actor = run.role;
            record.run = run.id;
            record.context = copy(run.context);
          }
          data.relations = data.relations.filter(edge => !(edge.to === record.id && edge.type === 'results-in'));
          data.relations.push({ id: uid('edge'), from: form.get('decision'), to: record.id, type: 'results-in' });
          log('Action proposed', `${record.title}. Exact content requires new approval.`, 'actions');
          closeDialog(); persist(); focusRecord(record.id);
        });
        const operationSelect = document.querySelector('.atlas-modal select[name="operation"]');
        if (!r && current()?.kind === 'decision' && current().status === 'Current') document.querySelector('.atlas-modal select[name="decision"]').value = current().id;
        function showOperationFields() {
          document.querySelectorAll('.atlas-operation-fields').forEach(fields => {
            fields.hidden = fields.dataset.operation !== operationSelect.value;
            fields.disabled = fields.hidden;
          });
        }
        operationSelect.addEventListener('change', showOperationFields);
        showOperationFields();
      }
      function reviewAction(id) {
        const r = get(id), c = connection(r);
        openDialog('Review exact external operation', `<div class="atlas-notice">LOCAL SIMULATION. Approval does not contact a real service.</div><h3>${escape(r.title)} / r${r.revision}</h3><dl><dt>Operation</dt><dd>${escape(r.operation)}</dd><dt>Connection and target</dt><dd>${escape(c.provider)} / ${escape(r.target)}</dd><dt>Service endpoint</dt><dd>${escape(r.endpoint)}</dd>${r.operation === 'create-task' ? `<dt>Owner / due date</dt><dd>${escape(r.owner)} / ${escape(r.due)}</dd>` : `<dt>Sender</dt><dd>${escape(r.sender)}</dd><dt>Recipients</dt><dd>${escape(r.recipients)}</dd><dt>Subject</dt><dd>${escape(r.subject)}</dd>`}<dt>Content</dt><dd class="atlas-body">${escape(r.body)}</dd><dt>Status</dt><dd>${escape(r.status)}</dd></dl><p class="atlas-small">Approval binds this revision, target and connection scope. Editing or changing scope revokes approval. An agent cannot approve.</p>${btn('Approve exact version', 'approve', id, 'primary-button', !canApprove(r))}${r.status === 'Approved' ? btn('Execute approved operation', 'execute', id) : ''}${r.status === 'Uncertain' ? btn('Reconcile external outcome', 'reconcile', id) : ''}`);
      }
      function executionDialog(id) {
        const r = get(id);
        openDialog('Simulated connector execution', `<p>${escape(r.title)}</p><p>The operation is submitted. Choose a demonstration outcome; these controls are not production operations.</p>${btn('Simulate success', 'outcome-success', id)}${btn('Simulate failure', 'outcome-failure', id)}${btn('Simulate interrupted response', 'outcome-uncertain', id)}`);
      }
      function finishAction(r, outcome) {
        if (outcome === 'success') {
          r.status = 'Succeeded';
          if (!r.result) {
            const result = { id: uid('external'), kind: r.operation === 'create-task' ? 'task' : 'source', title: r.operation === 'create-task' ? r.title.replace(/^Create task: /, '') : `Sent: ${r.subject}`, body: r.body, actor: 'Connector observation', revision: 1, history: [], placed: true, connection: r.connection, sourceScope: connection(r).scope, sourceEndpoint: connection(r).endpoint, externalId: `demo/${r.connection}/${r.id}`, observed: 'Just now', status: r.operation === 'create-task' ? 'Open' : 'Delivered', ...freePosition(r.id) };
            data.records.push(result); r.result = result.id;
            data.relations.push({ id: uid('edge'), from: result.id, to: r.id, type: 'derived-from' });
          }
        } else r.status = outcome === 'failure' ? 'Failed' : 'Uncertain';
        log(`Action ${r.status.toLowerCase()}`, r.title, 'actions', 'Demo connector');
        closeDialog(); persist(); render();
      }
      function configureConnection(id) {
        const c = data.connections.find(c => c.id === id);
        openDialog('Configure workspace connection', `<p>${escape(c.provider)} / workspace: Atlas Launch</p>${field('Service URL (demo, not contacted)', 'endpoint', c.endpoint, 'url')}<label>Authorization model<select name="auth">${['Server-held service credential', 'Delegated provider authorization'].map(method => `<option ${c.auth === method ? 'selected' : ''}>${method}</option>`).join('')}</select></label>${field('Account identity (no credentials)', 'account', c.account)}${field('Resource scope', 'scope', c.scope)}<label class="atlas-checkbox"><input type="checkbox" name="read" ${c.read ? 'checked' : ''}>Read scoped messages or tasks</label><label class="atlas-checkbox"><input type="checkbox" name="write" ${c.write ? 'checked' : ''}>Allow proposed writes, always with member approval</label><label class="atlas-checkbox"><input type="checkbox" name="automatic" ${c.automatic ? 'checked' : ''}>Automatically place new scoped arrivals on this board</label><p class="atlas-small">Authorization choices describe the intended UX, not implemented provider support. Real setup would redirect to the provider or use a server-held service credential. Never enter passwords or tokens here. Changed configuration revokes pending approvals; existing observations remain pinned.</p>`, form => {
          if (data.role !== 'Owner') { formError('Only the workspace Owner can configure connections.'); return; }
          if (!URL.canParse(form.get('endpoint'))) { formError('Enter a valid service URL.'); return; }
          const scope = form.get('scope').trim(), account = form.get('account').trim(), endpoint = new URL(form.get('endpoint'));
          if (!scope || !account) { formError('Account and resource scope cannot be blank.'); return; }
          if (!['http:', 'https:'].includes(endpoint.protocol) || endpoint.username || endpoint.password || endpoint.search || endpoint.hash) { formError('Use an HTTP(S) server URL without embedded credentials, query parameters or fragments.'); return; }
          c.scope = scope; c.account = account; c.endpoint = endpoint.href; c.auth = form.get('auth'); c.read = form.has('read'); c.write = form.has('write'); c.automatic = form.has('automatic'); c.scopeRevision += 1;
          for (const r of data.records.filter(r => r.kind === 'action' && r.connection === c.id && r.status === 'Approved')) { r.approval = null; r.status = 'Awaiting approval'; }
          log('Connection scope configured', `${c.name}: ${scope}. Pending approvals revoked.`, 'sources');
          closeDialog(); persist(); renderPanel();
        });
      }
      function browseSources(id) {
        const c = data.connections.find(c => c.id === id);
        if (!c.read || c.status !== 'Connected') { notify('Scoped reading requires an active connection and read permission.'); return; }
        const title = id === 'mail' ? 'Room availability confirmed' : 'Check the presentation equipment';
        openDialog('Browse scoped external records', `<p>${escape(c.provider)} / ${escape(c.scope)}</p><section class="atlas-service"><h3>${title}</h3><p>External record / ${id === 'mail' ? 'message' : 'task'} #new-7</p>${btn('Place source on board', 'import', id)}</section>${data.records.filter(r => r.connection === id && ['source', 'task'].includes(r.kind) && r.sourceScope === c.scope && r.sourceEndpoint === c.endpoint && !r.placed).map(r => `<section class="atlas-service"><h3>${escape(r.title)}</h3>${btn('Place ' + r.title, 'restore-source', r.id)}</section>`).join('')}<p class="atlas-small">Only records authorized by this connection scope are listed. Imported text is evidence, never an instruction or approval.</p>`);
      }
      function startRun(event) {
        event.preventDefault();
        const form = new FormData(event.target), request = form.get('request').trim();
        if (!request) { notify('Enter a message.'); return; }
        const waiting = data.runs.find(run => run.status === 'Working' && run.awaitingReply && run.role === form.get('role'));
        data.messages.push({ actor: 'Maya Chen', body: request, refs: [] });
        chatDraft = '';
        if (waiting) {
          waiting.awaitingReply = false; waiting.replies ||= []; waiting.replies.push(request);
          data.messages.push({ actor: waiting.role, body: 'Demo: clarification received. I will keep the original pinned context.', refs: waiting.context });
          persist(); renderPanel(); return;
        }
        const run = { id: uid('run'), role: form.get('role'), request, context: copy(data.chatContext), status: 'Working', limits: copy(data.agentSettings) };
        if (!run.context.length) {
          run.awaitingReply = true;
          data.messages.push({ actor: run.role, body: 'Demo clarification: what should I help with, and which material should I consider? You may reply without attaching board content; attachments are explicit.', refs: [] });
        } else data.messages.push({ actor: run.role, body: 'Demo: I have your request and these pinned references.', refs: run.context });
        data.runs.unshift(run); log('Agent run started', request, 'agent', run.role); persist(); renderPanel();
      }
      function action(name, id) {
        if (name === 'context-menu') {
          const button = root.querySelector(`[data-atlas-action="context-menu"][data-id="${id}"]`), box = button.getBoundingClientRect();
          showContextMenu(id, box.right, box.bottom); return;
        }
        const r = get(id);
        if (name === 'close-dialog') { closeDialog(); return; }
        if (name === 'panel') { panel = id === 'memory' ? 'activity' : id; renderPanel(); return; }
        if (name === 'panel-toggle') { setPanelOpen(!panelOpen); return; }
        if (name === 'open-chat') { panel = 'agents'; setPanelOpen(true); root.querySelector('#atlas-agent-form textarea')?.focus(); return; }
        if (name === 'search-toggle') { toggleSearch(!searchOpen); return; }
        if (name === 'trace') { traceDialog(id); return; }
        if (name === 'detail') { recordDetail(id); return; }
        if (name === 'proposal') { proposalDialog(id); return; }
        if (name === 'activity-filter') { activityFilter = id; renderPanel(); return; }
        if (name === 'ask-selection' || name === 'attach-context') { data.chatContext = selected.filter(id => get(id)).map(snapshot); panel = 'agents'; setPanelOpen(true); return; }
        if (name === 'clear-context') { data.chatContext = []; renderPanel(); return; }
        if (name === 'capture') {
          const [refId, revision] = id.split(':');
          const ref = [...data.messages.flatMap(m => m.refs || []), ...data.chatContext].find(ref => ref.id === refId && ref.revision === Number(revision));
          if (ref) openDialog('Evidence Reference', `<h3>${escape(ref.title)} / r${ref.revision}</h3><p class="atlas-body">${escape(ref.body)}</p>${btn('View current record', 'detail', refId)}`);
          else notify('This captured revision is unavailable.');
          return;
        }
        if (name === 'inspect' || name === 'focus') { focusRecord(id); return; }
        if (name === 'mode') { if (id === 'memory') traceDialog(); else { mode = 'board'; render(); } return; }
        if (name === 'new-note-color' || name === 'note-color') {
          if (!Object.hasOwn(noteColors, id)) { notify('Choose an available Note color.'); return; }
          if (name === 'new-note-color') noteColor = id;
          else {
            const note = current();
            if (!note || note.kind !== 'note' || !allowed()) { notify('Select an editable Note to change its color.'); return; }
            visualCheckpoint(); note.color = id; persist();
          }
          render(); root.querySelector(`[data-atlas-action="${name}"][data-id="${id}"]`)?.focus(); return;
        }
        if (name === 'tool') { tool = id; render(); status(`${id === 'select' ? 'Shift-click cards to select multiple records.' : id === 'note' ? 'Choose a color, then click the canvas to place a Note.' : 'Active tool: ' + id}`); return; }
        if (name === 'fit') { fit(); return; }
        if (name === 'undo' || name === 'redo') { restoreVisual(name === 'undo' ? undo : redo, name === 'undo' ? redo : undo); return; }
        if (name.startsWith('zoom-')) { zoom(name === 'zoom-in' ? .1 : -.1); return; }
        if (name === 'edit' || name === 'annotate') { editRecord(id, name === 'annotate'); return; }
        if (name === 'decision' || name === 'supersede') { decisionDialog(name === 'supersede' ? id : '', name === 'decision' ? id : ''); return; }
        if (name === 'new-action' || name === 'edit-action') { actionDialog(id); return; }
        if (name === 'follow-up') { selected = [id]; actionDialog(); return; }
        if (name === 'prerequisite') {
          const candidates = data.records.filter(other => other.kind === 'action' && other.id !== id);
          if (!candidates.length) { notify('Prepare another action before adding a prerequisite.'); return; }
          openDialog('Require another action first', `<p>${escape(r.title)} must wait for:</p><label>Required action<select name="required">${candidates.map(other => `<option value="${other.id}">${escape(other.title)}</option>`).join('')}</select></label><p>This records an execution prerequisite, not a drawing.</p>`, form => {
            const required = form.get('required'), visited = new Set();
            const reaches = node => {
              if (node === id) return true;
              if (visited.has(node)) return false;
              visited.add(node);
              return data.relations.some(edge => edge.type === 'depends-on' && edge.from === node && reaches(edge.to));
            };
            if (reaches(required)) { formError('These actions would wait for each other. Choose a different prerequisite.'); return; }
            if (data.relations.some(edge => edge.type === 'depends-on' && edge.from === id && edge.to === required)) { formError('This prerequisite is already recorded.'); return; }
            data.relations.push({ id: uid('edge'), from: id, to: required, type: 'depends-on' });
            r.approval = null; r.status = 'Awaiting approval';
            log('Action prerequisite recorded', `${r.title} must wait for ${get(required).title}.`, 'actions');
            closeDialog(); persist(); render();
          }); return;
        }
        if (name === 'remove' || name === 'restore') { visualCheckpoint(); r.placed = name === 'restore'; log('Placement ' + (r.placed ? 'restored' : 'removed'), `${r.title}. Underlying record and evidence preserved.`); closeDialog(); persist(); render(); if (r.placed) focusRecord(id); return; }
        if (name === 'adopt' || name === 'reject') {
          if (name === 'adopt' && r.status !== 'Pending review') { notify('Only a current pending proposal can be adopted. Request a fresh assessment.'); return; }
          r.status = name === 'adopt' ? 'Adopted' : 'Rejected';
          if (name === 'adopt') {
            const note = { ...copy(r), id: uid('note'), kind: 'note', actor: 'Maya Chen', status: 'Current', placed: false, ...freePosition(r.id), revision: 1, history: [] };
            data.records.push(note); data.relations.push({ id: uid('edge'), from: note.id, to: r.id, type: 'derived-from' });
          }
          log('Proposal ' + r.status.toLowerCase(), `${r.title}. No Decision was recorded and no card was added automatically.`, 'agent'); closeDialog(); persist(); render(); recordDetail(id); return;
        }
        if (name === 'reassess') { selected = [id, ...(r.evidence || []).map(e => e.id)]; panel = 'agents'; renderPanel(); return; }
        if (name === 'evidence') {
          const [recordId, index] = id.split(':'), ref = get(recordId).evidence[Number(index)];
          openDialog('Pinned evidence revision', `<h3>${escape(ref.title)} / r${ref.revision}</h3><p class="atlas-body">${escape(ref.body)}</p><p>This capture stays unchanged when current material is edited or removed from the board.</p>${btn('Inspect current record', 'focus', ref.id)}`); return;
        }
        if (name === 'review') { reviewAction(id); return; }
        if (name === 'approve') {
          const c = connection(r);
          if (!canApprove(r)) { notify('Approval is not allowed for this operation, decision or connection scope. Revise and review it.'); return; }
          r.approval = { actor: 'Maya Chen', revision: r.revision, scopeRevision: c.scopeRevision, target: r.target }; r.status = 'Approved';
          log('Action approved', `${r.title} / r${r.revision}`, 'actions'); closeDialog(); persist(); render(); return;
        }
        if (name === 'execute') {
          const c = connection(r);
          if (r.status !== 'Approved' || !r.approval || r.approval.revision !== r.revision || r.approval.scopeRevision !== c.scopeRevision || r.approval.target !== r.target || !c.write || c.status !== 'Connected') { notify('A current approval and connected, permitted target are required. No operation was submitted.'); return; }
          const pending = data.relations.filter(e => e.from === r.id && e.type === 'depends-on').map(e => get(e.to)).filter(d => d.kind === 'action' && d.status !== 'Succeeded');
          if (pending.length) { notify('Complete prerequisite action first: ' + pending.map(d => d.title).join(', ')); return; }
          r.status = 'In execution'; log('Action execution started', r.title, 'actions'); closeDialog(); persist(); render(); executionDialog(id); return;
        }
        if (name === 'execution') { executionDialog(id); return; }
        if (name.startsWith('outcome-')) { if (r.status !== 'In execution') { notify('This operation is not in execution.'); return; } finishAction(r, name.slice(8)); return; }
        if (name === 'reconcile') {
          const c = connection(r);
          if (c.status !== 'Connected' || !c.read || c.scope !== r.target || c.endpoint !== r.endpoint) { notify('Reconciliation requires a connected, readable service with the original target scope. No retry has been submitted.'); return; }
          openDialog('Reconcile before retry', `<p>${escape(r.title)}</p><p>Compare the submitted operation with the provider result. Until reconciled, this operation cannot be edited or resubmitted.</p>${btn('Demo: operation found in provider', 'reconcile-found', id)}${btn('Demo: confirmed not submitted', 'reconcile-absent', id)}`); return;
        }
        if (name === 'reconcile-found') { finishAction(r, 'success'); return; }
        if (name === 'reconcile-absent') { r.status = 'Awaiting approval'; r.approval = null; log('Action reconciled', 'Confirmed not submitted. New approval required.', 'actions'); closeDialog(); persist(); render(); return; }
        if (name === 'connection') { configureConnection(id); return; }
        if (name === 'connection-toggle') {
          if (data.role !== 'Owner') { notify('Only the Owner manages connections.'); return; }
          const c = data.connections.find(c => c.id === id); c.status = c.status === 'Connected' ? 'Disconnected' : 'Connected';
          log('Connection ' + c.status.toLowerCase(), `${c.name}. Last-known observations and annotations are preserved.`, 'sources'); persist(); render(); return;
        }
        if (name === 'browse') { browseSources(id); return; }
        if (name === 'restore-source') { r.placed = true; closeDialog(); persist(); focusRecord(id); return; }
        if (name === 'arrive') {
          const c = data.connections.find(c => c.id === id);
          if (c.status !== 'Connected' || !c.read) { notify('Reconnect and enable scoped reading before receiving observations.'); return; }
          const incoming = { id: uid('arrival'), kind: id === 'mail' ? 'source' : 'task', title: 'New scoped arrival: logistics update', body: 'A new fictional update arrived through the configured workspace connection.', actor: 'Connector observation', revision: 1, status: 'Observed', placed: c.automatic, history: [], ...freePosition(id === 'mail' ? 'email' : 'room'), connection: id, sourceScope: c.scope, sourceEndpoint: c.endpoint, externalId: uid('external'), observed: 'Just now' };
          data.records.push(incoming); log('Source observed', `${incoming.title}. ${c.automatic ? 'Placed automatically on this board.' : 'Available in the scoped source browser; not placed automatically.'}`, 'sources'); persist(); render();
          notify(c.automatic ? 'New scoped source placed on the board.' : 'New source available in Browse scoped records.'); return;
        }
        if (name === 'import') {
          const c = data.connections.find(c => c.id === id);
          if (!c.read || c.status !== 'Connected') { notify('This connection cannot read external records.'); return; }
          const externalId = `${id}/scoped/${c.scopeRevision}/new-7`, existing = data.records.find(r => r.externalId === externalId);
          if (existing) { existing.placed = true; closeDialog(); persist(); focusRecord(existing.id); return; }
          const imported = { id: uid('source'), kind: id === 'mail' ? 'source' : 'task', title: id === 'mail' ? 'Room availability confirmed' : 'Check the presentation equipment', body: id === 'mail' ? 'From: Jordan Lee\nThe room is available at 11:00. Please confirm to the guests.' : 'Owner: Leo Martin\nDue: today\nObserved status: Open', actor: 'Connector observation', revision: 1, status: 'Observed', placed: true, history: [], ...freePosition(id === 'mail' ? 'email' : 'room'), connection: id, sourceScope: c.scope, sourceEndpoint: c.endpoint, externalId, observed: 'Just now' };
          data.records.push(imported); log('Source imported', imported.title, 'sources'); closeDialog(); persist(); focusRecord(imported.id); return;
        }
        if (name === 'original' && r.sharedCapture) { openDialog('Original requires separate permission', `<div class="atlas-notice">Prototype permission boundary, not real authentication.</div><p>Workspace members can read the shared capture. Opening the original requires their own authorization in the external mail service. Memento does not lend ${escape(r.sharedCapture.by)}'s session or mailbox access.</p><p>No original message was opened and no service was contacted.</p>`); return; }
        if (name === 'original') { openDialog('External record preview', `<div class="atlas-notice">Demo preview, not a live external client. No service was contacted.</div><p>${escape(connection(r).provider)} / ${escape(r.externalId)}</p><h3>${escape(r.title)}</h3><p class="atlas-body">${escape(r.body)}</p>${r.kind === 'task' ? btn('Simulate completion in Deck', 'task-complete', id) : ''}`); return; }
        if (name === 'task-complete' || name === 'refresh') {
          const c = connection(r);
          if (c.status !== 'Connected' || !c.read || r.sourceScope !== c.scope || r.sourceEndpoint !== c.endpoint) { notify('This observation is outside the current readable connection scope or the connection is unavailable. Last-known content is preserved.'); return; }
          r.history.push(snapshot(id)); if (name === 'task-complete') { r.status = 'Completed'; r.body = r.body.replace(/Observed status: .*/, 'Observed status: Completed') + '\nCompleted in the external client (demo).'; }
          r.observed = 'Just now'; changed(r); c.checked = 'Just now';
          log('Source observation refreshed', `${r.title}: ${r.status}. Local annotations and layout preserved.`, 'sources'); closeDialog(); persist(); render(); return;
        }
        if (name === 'observations') { openDialog('Source observation history', `<h3>Current r${r.revision}</h3><p class="atlas-body">${escape(r.body)}</p>${r.history.map(ref => `<h3>Observed r${ref.revision}</h3><p class="atlas-body">${escape(ref.body)}</p>`).join('')}<p class="atlas-small">Current observations do not rewrite pinned evidence.</p>`); return; }
        if (name.startsWith('run-')) {
          const run = data.runs.find(run => run.id === id);
          if (name === 'run-context') { openDialog('Pinned agent context', `<p>${escape(run.role)} / ${escape(run.status)}</p>${run.context.map(ref => `<h3>${escape(ref.title)} / r${ref.revision}</h3><p class="atlas-body">${escape(ref.body)}</p>`).join('')}`); return; }
          if (run.status !== 'Working') { notify('This run has already finished.'); return; }
          if (name === 'run-question') {
            run.awaitingReply = true;
            data.messages.push({ actor: run.role, body: 'Demo clarification: which constraints must remain fixed, and what outcome would you like? Reply here; I will keep this run’s pinned references.', refs: run.context });
            persist(); renderPanel(); return;
          }
          if (name === 'run-complete' && run.awaitingReply) { notify('Reply to the clarification before completing this Agent Run.'); return; }
          run.awaitingReply = false;
          run.status = name === 'run-cancel' ? 'Cancelled' : name === 'run-fail' ? 'Failed' : 'Completed';
          if (run.status === 'Completed') {
            if (run.role === 'Action Coordinator') {
              data.messages.push({ actor: run.role, body: 'Demo: review the proposed Action content and destination next. Saving is not approval or execution.', refs: run.context });
              log('Agent run completed', run.request, 'agent', run.role); persist(); renderPanel(); actionDialog('', run.id); return;
            }
            const r = { id: uid('proposal'), kind: 'proposal', title: 'Impact assessment: ' + run.request.slice(0, 65), body: 'Demo suggestion: review the constraints and confirm availability before notifying participants. This is scripted example content, not inference from your request.', actor: 'Facilitator', revision: 1, history: [], evidence: copy(run.context), status: run.context.some(ref => get(ref.id)?.revision !== ref.revision) ? 'Stale' : 'Pending review', placed: false, ...freePosition(run.context[0]?.id) };
            data.records.push(r); run.output = r.id;
            data.messages.push({ actor: run.role, body: 'Demo: a Proposal is ready to inspect. It has not changed your Notes or added a card to the board.', refs: run.context });
            run.context.forEach(ref => data.relations.push({ id: uid('edge'), from: ref.id, to: r.id, type: 'supports' }));
          }
          if (run.status !== 'Completed') data.messages.push({ actor: run.role, body: `Agent Run ${run.status.toLowerCase()}. No further contribution will be generated by this run.`, refs: [] });
          log('Agent run ' + run.status.toLowerCase(), run.request, 'agent', run.role); persist(); render(); return;
        }
        if (name === 'conflict') {
          if (['source', 'task'].includes(r.kind)) { r.annotation = 'Leo: please confirm the room.'; r.annotationRevision = (r.annotationRevision || 0) + 1; }
          else { r.history.push(snapshot(id)); if (r.kind === 'note') r.text = noteText(r) + '\nLeo: room confirmation is still required.'; r.body += '\nLeo: room confirmation is still required.'; changed(r); }
          log('Concurrent edit accepted', `${r.title} / Leo's accepted revision is preserved.`); persist(); notify('A concurrent edit by Leo was accepted. Save your draft to resolve the conflict.'); return;
        }
        if (name === 'conflict-keep' || name === 'conflict-apply') { if (name === 'conflict-apply') saveEdit(get(conflict.id), conflict.draft, conflict.annotation); conflict = null; closeDialog(); render(); return; }
        if (name === 'frame-edit') {
          const f = data.frames.find(f => f.id === id);
          openDialog('Edit named frame', `${field('Frame name', 'title', f.title)}${field('Width', 'width', f.width, 'number')}${field('Height', 'height', f.height, 'number')}`, form => {
            const width = Number(form.get('width')), height = Number(form.get('height')), title = form.get('title').trim();
            if (!title || width < 200 || height < 150 || width > 4000 || height > 4000) { formError('Use a name, width 200–4000 and height 150–4000.'); return; }
            visualCheckpoint(); Object.assign(f, { title, width, height }); closeDialog(); persist(); render();
          }); return;
        }
        if (name === 'members') {
          openDialog('Workspace members & access', `<p>Owner: Maya Chen / Editors: Leo Martin, Sam Rivera</p><p>Owners manage connections and invitations. Editors contribute, record decisions and approve exact operations. Agents cannot approve.</p><label>Preview member role<select name="role"><option ${data.role === 'Owner' ? 'selected' : ''}>Owner</option><option ${data.role === 'Editor' ? 'selected' : ''}>Editor</option></select></label><label>Invite participant (optional)<input type="email" name="email" placeholder="name@example.test"></label>${data.invitations.map(email => `<p>Pending invitation: ${escape(email)}</p>`).join('')}<p class="atlas-small">Role and invitations are visual demo state, not authentication or real invitations.</p>`, form => {
            const email = form.get('email').trim();
            if (email && data.role !== 'Owner') { formError('Only the Owner may invite members.'); return; }
            if (email && !data.invitations.includes(email)) data.invitations.push(email);
            data.role = form.get('role'); closeDialog(); persist(); render();
          }); return;
        }
      }
      function fit() {
        const viewport = root.querySelector('.flow-viewport'), nodes = [...root.querySelectorAll('[data-record]')];
        if (!nodes.length) { notify('No matching records to frame. Clear the search or add a note.'); return; }
        const left = Math.min(...nodes.map(n => n.offsetLeft)), top = Math.min(...nodes.map(n => n.offsetTop));
        const right = Math.max(...nodes.map(n => n.offsetLeft + n.offsetWidth)), bottom = Math.max(...nodes.map(n => n.offsetTop + n.offsetHeight));
        const zoom = Math.max(.15, Math.min(.85, (viewport.clientWidth - 130) / (right - left), (viewport.clientHeight - 100) / (bottom - top)));
        data.camera = { x: 80 - left * zoom, y: 55 - top * zoom, zoom }; applyCamera(); persist();
      }
      function zoom(delta, x, y) {
        const viewport = root.querySelector('.flow-viewport').getBoundingClientRect(), c = data.camera;
        const z = Math.max(.15, Math.min(1.5, c.zoom + delta)), factor = z / c.zoom;
        const px = (x ?? viewport.left + viewport.width / 2) - viewport.left, py = (y ?? viewport.top + viewport.height / 2) - viewport.top;
        c.x = px - (px - c.x) * factor; c.y = py - (py - c.y) * factor; c.zoom = z; applyCamera(); persist();
      }
      function pointerDown(event) {
        if (editingNote && event.target.closest('[data-atlas-action="conflict"]')) { event.preventDefault(); return; }
        if (event.target.closest('[contenteditable]')) return;
        const viewport = event.target.closest('.flow-viewport');
        if (!viewport || event.button !== 0 || event.target.closest('button,.flow-tool-rail,.flow-zoom-controls')) return;
        const el = event.target.closest('[data-record]'), f = event.target.closest('[data-frame]');
        if (el && event.target.closest('.flow-handle')) {
          const bounds = viewport.getBoundingClientRect(), c = data.camera;
          gesture = { type: 'connect', from: el.dataset.record, x: (event.clientX - bounds.left - c.x) / c.zoom, y: (event.clientY - bounds.top - c.y) / c.zoom, pointerId: event.pointerId };
          viewport.setPointerCapture(event.pointerId);
          status('Release on another card to draw a visual connection. Escape cancels.');
          return;
        }
        const bounds = viewport.getBoundingClientRect(), c = data.camera;
        const point = { x: (event.clientX - bounds.left - c.x) / c.zoom, y: (event.clientY - bounds.top - c.y) / c.zoom };
        if (tool === 'erase') {
          const paths = [...root.querySelectorAll('.atlas-stroke')];
          const hit = paths.find(path => { const box = path.getBoundingClientRect(); return event.clientX >= box.left - 8 && event.clientX <= box.right + 8 && event.clientY >= box.top - 8 && event.clientY <= box.bottom + 8; });
          if (hit) { visualCheckpoint(); data.strokes = data.strokes.filter(s => s.id !== hit.dataset.stroke); persist(); drawEdges(); }
          else notify('Erase removes ink strokes only. Remove cards using the inspector.');
          return;
        }
        if (mode === 'board' && ['note', 'text', 'shape', 'frame'].includes(tool) && !el) {
          event.preventDefault();
          visualCheckpoint();
          if (tool === 'frame') data.frames.push({ id: uid('frame'), title: 'New frame', x: point.x, y: point.y, width: 410, height: 500 });
          else {
            const r = { id: uid(tool), kind: tool, title: tool === 'note' ? 'Untitled Note' : tool === 'shape' ? 'New shape' : 'New text', body: tool === 'note' ? '' : 'Double-click or use Edit content to write here.', ...(tool === 'note' ? { text: '' } : {}), actor: 'Maya Chen', revision: 1, history: [], placed: true, status: 'Current', x: point.x, y: point.y };
            data.records.push(r); selected = [r.id]; log('Material added', r.title);
            if (tool === 'note') { r.color = noteColor; panel = 'inspect'; }
          }
          const createdNote = tool === 'note' ? selected[0] : null;
          if (createdNote) search = '';
          tool = 'select'; persist(); render(); if (createdNote) startNoteEdit(createdNote); return;
        }
        if (tool === 'pen' && mode === 'board') {
          visualCheckpoint(); const stroke = { id: uid('ink'), path: `M${point.x} ${point.y}` }; data.strokes.push(stroke);
          gesture = { type: 'ink', stroke, pointerId: event.pointerId };
        } else if (tool === 'hand' || !el && !f) {
          gesture = { type: 'pan', x: c.x, y: c.y, startX: event.clientX, startY: event.clientY, pointerId: event.pointerId };
        } else if (mode === 'board' && event.target.closest('.flow-node-header,.flow-frame-heading')) {
          visualCheckpoint(); const record = el ? get(el.dataset.record) : data.frames.find(frame => frame.id === f.dataset.frame);
          const children = f ? data.records.filter(r => r.placed && r.x >= record.x && r.x < record.x + record.width && r.y >= record.y && r.y < record.y + record.height).map(r => ({ record: r, x: r.x, y: r.y })) : [];
          gesture = { type: 'move', record, children, el: el || f, x: record.x, y: record.y, startX: event.clientX, startY: event.clientY, pointerId: event.pointerId };
        }
        if (gesture) viewport.setPointerCapture(event.pointerId);
      }
      function pointerMove(event) {
        if (!gesture || event.pointerId !== gesture.pointerId) return;
        const c = data.camera;
        if (gesture.type === 'connect') {
          const bounds = root.querySelector('.flow-viewport').getBoundingClientRect();
          root.querySelector('.atlas-connect-preview')?.remove();
          const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
          path.classList.add('atlas-connect-preview');
          path.setAttribute('d', `M${gesture.x} ${gesture.y} L${(event.clientX - bounds.left - c.x) / c.zoom} ${(event.clientY - bounds.top - c.y) / c.zoom}`);
          root.querySelector('.atlas-edges').append(path);
        }
        else if (gesture.type === 'pan') { c.x = gesture.x + event.clientX - gesture.startX; c.y = gesture.y + event.clientY - gesture.startY; applyCamera(); }
        else if (gesture.type === 'move') {
          gesture.record.x = gesture.x + (event.clientX - gesture.startX) / c.zoom; gesture.record.y = gesture.y + (event.clientY - gesture.startY) / c.zoom;
          gesture.el.style.left = gesture.record.x + 'px'; gesture.el.style.top = gesture.record.y + 'px'; drawEdges();
          for (const child of gesture.children) {
            child.record.x = child.x + gesture.record.x - gesture.x; child.record.y = child.y + gesture.record.y - gesture.y;
            const el = root.querySelector(`[data-record="${child.record.id}"]`); if (el) { el.style.left = child.record.x + 'px'; el.style.top = child.record.y + 'px'; }
          }
          drawEdges();
        } else {
          const bounds = root.querySelector('.flow-viewport').getBoundingClientRect();
          gesture.stroke.path += ` L${(event.clientX - bounds.left - c.x) / c.zoom} ${(event.clientY - bounds.top - c.y) / c.zoom}`; drawEdges();
        }
      }
      function pointerUp(event) {
        if (!gesture || event.pointerId !== gesture.pointerId) return;
        if (gesture.type === 'connect' && event.type !== 'pointercancel') {
          const target = document.elementFromPoint(event.clientX, event.clientY)?.closest('[data-record]');
          if (target && target.dataset.record !== gesture.from) {
            visualCheckpoint();
            data.visualLinks.push({ id: uid('visual'), from: gesture.from, to: target.dataset.record, type: 'visual', label: '' });
            log('Visual connection drawn', 'A drawing between cards; no evidence or Decision recorded.');
            drawEdges();
          } else notify('Connection cancelled. Release on a different card.');
        }
        gesture = null; persist();
        root.querySelector('.atlas-connect-preview')?.remove();
      }
      root.addEventListener('pointerdown', pointerDown);
      root.addEventListener('pointermove', pointerMove);
      root.addEventListener('pointerup', pointerUp);
      root.addEventListener('pointercancel', pointerUp);
      root.addEventListener('wheel', event => {
        if (!event.target.closest('.flow-viewport') || event.target.closest('.flow-node-content')) return;
        event.preventDefault(); zoom(event.deltaY < 0 ? .05 : -.05, event.clientX, event.clientY);
      }, { passive: false });
      root.addEventListener('click', event => {
        if (event.target.closest('[contenteditable]')) return;
        const button = event.target.closest('[data-atlas-action]');
        if (button) { action(button.dataset.atlasAction, button.dataset.id); return; }
        const card = event.target.closest('[data-record]');
        if (!card) return;
        const id = card.dataset.record;
        selected = event.shiftKey ? selected.includes(id) ? selected.filter(item => item !== id) : [...selected, id] : [id];
        root.querySelectorAll('[data-record]').forEach(el => el.setAttribute('aria-pressed', String(selected.includes(el.dataset.record))));
        renderPanel();
      });
      root.addEventListener('dblclick', event => {
        const card = event.target.closest('[data-record]'), r = card && get(card.dataset.record);
        if (r && ['note', 'text', 'shape', 'proposal'].includes(r.kind) && !event.target.closest('button')) editRecord(r.id);
      });
      root.addEventListener('focusout', event => {
        if (event.target.matches('[data-note-text][contenteditable]')) finishNoteEdit();
      });
      root.addEventListener('keydown', event => {
        if ((event.shiftKey && event.key === 'F10') || event.key === 'ContextMenu') {
          const card = event.target.closest('[data-record]'), box = (card || root.querySelector('.flow-viewport')).getBoundingClientRect();
          event.preventDefault(); showContextMenu(card?.dataset.record, box.left + 25, box.top + 35); return;
        }
        if (event.target.closest('[contenteditable]')) {
          if (event.key === 'Escape' || (event.key === 'Enter' && (event.ctrlKey || event.metaKey))) {
            event.preventDefault(); event.stopPropagation();
            const card = event.target.closest('[data-record]');
            finishNoteEdit(event.key === 'Escape'); renderPanel(); card?.focus();
          }
          return;
        }
        if (event.key === 'Escape') {
          gesture = null; keyboardConnection = null; root.querySelector('.atlas-connect-preview')?.remove();
          if (searchOpen) { toggleSearch(false); root.querySelector('[data-atlas-action="search-toggle"]').focus(); }
          return;
        }
        if (event.target.matches('input,textarea,select,button')) return;
        if (event.key === 'Enter' || event.key === ' ') {
          const card = event.target.closest('[data-record]');
          if (card && event.target.closest('.flow-handle')) {
            event.preventDefault(); keyboardConnection = card.dataset.record;
            status('Focus another card and press Enter to connect. Escape cancels.'); return;
          }
          if (card && keyboardConnection) {
            event.preventDefault();
            if (card.dataset.record === keyboardConnection) { notify('Choose a different card.'); return; }
            visualCheckpoint();
            data.visualLinks.push({ id: uid('visual'), from: keyboardConnection, to: card.dataset.record, type: 'visual', label: '' });
            keyboardConnection = null; persist(); drawEdges(); return;
          }
          if (card) {
            event.preventDefault(); selected = [card.dataset.record]; panel = 'inspect';
            if (event.key === 'Enter' && get(card.dataset.record).kind === 'note') startNoteEdit(card.dataset.record);
            else renderPanel();
          }
        }
      });
      document.querySelector('#overlay-root').addEventListener('keydown', event => {
        if (!event.target.closest('.atlas-modal')) return;
        if (event.key === 'Escape') { event.stopPropagation(); closeDialog(); }
        if (event.key === 'Tab') {
          const focusable = [...event.target.closest('.atlas-modal').querySelectorAll('button:not(:disabled),input,textarea,select,a[href]')];
          const first = focusable[0], last = focusable[focusable.length - 1];
          if (event.shiftKey && document.activeElement === first) { event.preventDefault(); last.focus(); }
          else if (!event.shiftKey && document.activeElement === last) { event.preventDefault(); first.focus(); }
        }
      });
      render();
      if (!restored && data.records.length) fit();
      return {
        snapshot: () => copy(data),
        update(change, view = {}) {
          change(data);
          data.visualLinks ||= []; data.messages ||= []; data.chatContext ||= [];
          if (view.selection) selected = view.selection;
          if (view.panel) panel = view.panel === 'memory' ? 'activity' : view.panel;
          mode = 'board';
          closeDialog(); persist(); render();
          if (view.fit && visibleRecords().length) fit();
          if (view.trace || view.mode === 'memory') traceDialog();
        },
        action,
        dialog: openDialog,
        closeDialog
      };
    }
  };
})();
