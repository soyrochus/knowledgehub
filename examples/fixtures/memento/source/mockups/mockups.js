const app = document.querySelector('#app');
    const themeStyles = document.createElement('link');
    themeStyles.rel = 'stylesheet'; themeStyles.href = 'themes.css?v=20261027r';
    document.head.append(themeStyles);
    let theme = 'dark';
    try { theme = localStorage.getItem('memento-theme') === 'light' ? 'light' : 'dark'; }
    catch (error) { console.error('Could not restore theme preference', error); }
    document.documentElement.dataset.theme = theme;
    function renderThemeControl() {
      document.querySelector('#theme-toggle')?.remove();
      const control = document.createElement('button');
      control.id = 'theme-toggle'; control.type = 'button'; control.className = 'quiet-button';
      control.textContent = theme === 'dark' ? 'Light theme' : 'Dark theme';
      control.setAttribute('aria-label', theme === 'dark' ? 'Switch to light theme' : 'Switch to dark theme');
      control.addEventListener('click', () => {
        theme = theme === 'dark' ? 'light' : 'dark';
        document.documentElement.dataset.theme = theme;
        try { localStorage.setItem('memento-theme', theme); }
        catch (error) { console.error('Could not save theme preference', error); showToast('Theme changed, but the preference could not be saved.'); }
        renderThemeControl();
      });
      document.querySelector('#topbar-right').prepend(control);
    }
    const overlayRoot = document.querySelector('#overlay-root');
    const toastRoot = document.querySelector('#toast-root');
    const topbarRight = document.querySelector('#topbar-right');
    const activityRoot = document.querySelector('#activity-root');
    const state = { name: 'Maya Chen', workspaceFilter: 'all', search: '', boardView: 'recent', activityCategory: 'all', activitySearch: '', templateCategory: 'All', templateSearch: '', templateName: '', boardSelection: 'decision', boardTool: 'select', boardNotes: [] };

    const workspaces = [
      { id: 'atlas', name: 'Atlas Launch', description: 'Briefing, guests, and launch decisions.', type: 'owned', marker: '◆', activity: 'Updated 4 min ago', people: ['MC', 'LC', 'SR'], boards: ['Launch plan', 'Messages and sources'] },
      { id: 'research', name: 'Research · Q4', description: 'Findings, interviews, and opportunities.', type: 'owned', marker: '◈', activity: 'Updated yesterday', people: ['MC', 'AL'], boards: ['Interview synthesis'] },
      { id: 'service', name: 'Service redesign', description: 'A shared workspace from Alex Rivera.', type: 'shared', marker: '↗', activity: 'Invitation from Alex Rivera', people: ['AR', 'MC', 'JP'], boards: ['Current journey', 'Service hypotheses'] }
    ];

    const templates = [
      { title: 'Decision review', category: 'Decisions', description: 'Connect the decision to its evidence, alternatives, and next steps.', style: 'cool', kind: 'decision' },
      { title: 'Research synthesis', category: 'Research', description: 'Group observations and link findings to their original sources.', style: 'warm', kind: 'research' },
      { title: 'Project kickoff', category: 'Planning', description: 'Align on the goal, participants, constraints, and first actions.', style: 'cool', kind: 'kickoff' },
      { title: 'Dependency map', category: 'Diagrams', description: 'Make prerequisites and decision impacts visible.', style: 'warm', kind: 'dependencies' },
      { title: 'Quick retrospective', category: 'Meetings', description: 'Capture signals, identify patterns, and record agreed actions.', style: 'cool', kind: 'retro' },
      { title: 'Open questions', category: 'Ideation', description: 'Separate what we know, what is missing, and who can resolve it.', style: 'warm', kind: 'questions' }
    ];
    const categories = ['All', 'Decisions', 'Research', 'Planning', 'Diagrams', 'Meetings', 'Ideation'];
    const boardSignals = {
      'Launch plan': { favorite: true, changes: 2, lastOpened: 'Opened yesterday' },
      'Messages and sources': { favorite: false, changes: 0, lastOpened: 'Opened 4 min ago' },
      'Interview synthesis': { favorite: false, changes: 1, lastOpened: 'Opened 5 days ago' },
      'Current journey': { favorite: false, changes: 3, lastOpened: 'Opened 2 days ago' },
      'Service hypotheses': { favorite: false, changes: 0, lastOpened: 'Opened today' }
    };
    const activityItems = [
      { id: 'note-added', category: 'collaboration', title: 'Note added', event: 'Note Added', detail: 'Leo added a note about the speaker’s availability.', workspace: 'Atlas Launch', actor: 'Leo Martin', time: '2 min ago', read: false },
      { id: 'source-observed', category: 'sources', title: 'New email observed', event: 'Source Observed', detail: 'The request to move the briefing arrived in the project mailbox.', workspace: 'Atlas Launch', actor: 'Email sync', time: '5 min ago', read: false, outcome: 'Source captured' },
      { id: 'proposal-submitted', category: 'agent', title: 'Proposal ready for review', event: 'Proposal Submitted', detail: 'Facilitator found a dependency in the plan and added supporting evidence.', workspace: 'Atlas Launch', actor: 'Facilitator', time: '15 min ago', read: false, outcome: 'Pending review', outcomeKind: 'pending' },
      { id: 'decision-recorded', category: 'decisions', title: 'Decision recorded', event: 'Decision Recorded', detail: 'Maya confirmed the 11:00 briefing and kept the launch date.', workspace: 'Atlas Launch', actor: 'Maya Chen', time: '35 min ago', read: false },
      { id: 'action-uncertain', category: 'actions', title: 'Action outcome uncertain', event: 'Action Outcome Uncertain', detail: 'The reply send was interrupted; check the mailbox before retrying.', workspace: 'Atlas Launch', actor: 'Action Coordinator', time: '1 hr ago', read: false, outcome: 'Review needed', outcomeKind: 'uncertain' },
      { id: 'note-revised', category: 'collaboration', title: 'Note revised', event: 'Note Revised', detail: 'Maya updated the room scheduling constraint.', workspace: 'Atlas Launch', actor: 'Maya Chen', time: '2 hr ago', read: true },
      { id: 'placement-arranged', category: 'collaboration', title: 'Item rearranged', event: 'Placement Arranged', detail: 'Leo moved the logistics note into the “Next steps” frame.', workspace: 'Atlas Launch', actor: 'Leo Martin', time: '3 hr ago', read: true },
      { id: 'relationship-established', category: 'collaboration', title: 'Relationship connected', event: 'Relationship Established', detail: 'The speaker observation now supports the proposed schedule.', workspace: 'Atlas Launch', actor: 'Maya Chen', time: '4 hr ago', read: true },
      { id: 'source-changed', category: 'sources', title: 'Task status updated', event: 'Source State Changed', detail: 'The “Prepare the room” task is marked complete in Deck.', workspace: 'Service redesign', actor: 'Task sync', time: 'Yesterday', read: true, outcome: 'State updated' },
      { id: 'agent-requested', category: 'agent', title: 'Assistance requested', event: 'Agent Assistance Requested', detail: 'Maya asked for an analysis of the impact of changing the briefing time.', workspace: 'Atlas Launch', actor: 'Maya Chen', time: 'Yesterday', read: true },
      { id: 'agent-started', category: 'agent', title: 'Facilitator started an analysis', event: 'Agent Run Started', detail: 'The email, note, and decision revisions used as context were pinned.', workspace: 'Atlas Launch', actor: 'Facilitator', time: 'Yesterday', read: true, outcome: 'Completed' },
      { id: 'proposal-adopted', category: 'agent', title: 'Proposal adopted', event: 'Proposal Adopted', detail: 'Maya adopted the 11:00 option as editable material.', workspace: 'Atlas Launch', actor: 'Maya Chen', time: 'Yesterday', read: true },
      { id: 'proposal-rejected', category: 'agent', title: 'Proposal rejected', event: 'Proposal Rejected', detail: 'Leo rejected the alternative that also moved the date.', workspace: 'Atlas Launch', actor: 'Leo Martin', time: 'Yesterday', read: true },
      { id: 'agent-cancelled', category: 'agent', title: 'Run cancelled', event: 'Agent Run Cancelled', detail: 'Maya stopped the analysis before an external action started.', workspace: 'Research · Q4', actor: 'Maya Chen', time: '2 days ago', read: true, outcome: 'Cancelled' },
      { id: 'decision-superseded', category: 'decisions', title: 'Earlier decision superseded', event: 'Decision Superseded', detail: 'The 11:00 decision superseded the earlier 10:00 decision; both retain their evidence.', workspace: 'Atlas Launch', actor: 'Maya Chen', time: '2 days ago', read: true },
      { id: 'placement-added', category: 'collaboration', title: 'Item added to board', event: 'Placement Added', detail: 'An agenda note was placed in the “Briefing” frame.', workspace: 'Atlas Launch', actor: 'Leo Martin', time: '2 days ago', read: true },
      { id: 'placement-removed', category: 'collaboration', title: 'Item removed from board', event: 'Placement Removed', detail: 'A visual placement was removed; the record and linked evidence were preserved.', workspace: 'Research · Q4', actor: 'Maya Chen', time: '3 days ago', read: true },
      { id: 'action-proposed', category: 'actions', title: 'Action proposed', event: 'Action Proposed', detail: 'Action Coordinator prepared a task for Leo with reviewable content and a due date.', workspace: 'Atlas Launch', actor: 'Action Coordinator', time: '3 days ago', read: true, outcome: 'Awaiting approval', outcomeKind: 'pending' },
      { id: 'action-approved', category: 'actions', title: 'Action approved', event: 'Action Approved', detail: 'Maya approved this exact version of the task and its assignee.', workspace: 'Atlas Launch', actor: 'Maya Chen', time: '3 days ago', read: true },
      { id: 'action-started', category: 'actions', title: 'Execution started', event: 'Action Execution Started', detail: 'The connector is creating the approved task in Deck.', workspace: 'Atlas Launch', actor: 'Action worker', time: '3 days ago', read: true, outcome: 'In progress', outcomeKind: 'pending' },
      { id: 'action-succeeded', category: 'actions', title: 'Action completed', event: 'Action Succeeded', detail: 'The task was created and its external ID was linked to the decision.', workspace: 'Atlas Launch', actor: 'Task connector', time: '3 days ago', read: true, outcome: 'Succeeded' },
      { id: 'action-failed', category: 'actions', title: 'Action failed', event: 'Action Failed', detail: 'The service rejected the operation; it was not recorded as complete.', workspace: 'Service redesign', actor: 'Email connector', time: '4 days ago', read: true, outcome: 'Failed', outcomeKind: 'failed' }
    ];
    const activityFilters = [
      { id: 'all', label: 'All' },
      { id: 'collaboration', label: 'Collaboration' },
      { id: 'sources', label: 'Sources' },
      { id: 'agent', label: 'Agents' },
      { id: 'decisions', label: 'Decisions' },
      { id: 'actions', label: 'Actions' }
    ];
    const launchPlanActivityIds = new Set([
      'note-added', 'source-observed', 'proposal-submitted', 'decision-recorded',
      'note-revised', 'placement-arranged', 'relationship-established',
      'agent-requested', 'agent-started', 'proposal-adopted', 'proposal-rejected',
      'decision-superseded', 'placement-added', 'action-proposed', 'action-approved',
      'action-started', 'action-succeeded'
    ]);
    function isBoardActivity() {
      return route() === 'board' || (route() === 'activity' && new URLSearchParams(location.search).has('board'));
    }
    function activeBoardId() {
      return new URLSearchParams(location.search).get('board') || 'atlas-launch-plan';
    }
    function activeBoardName() {
      return activeBoardId() === 'atlas-launch-plan' ? 'Launch plan' : window.BoardDemo.list(showToast).find(item => item.id === activeBoardId())?.name || 'Board';
    }
    function scopedActivityItems() {
      return isBoardActivity() ? activityItems.filter(item => item.board === activeBoardId() || (activeBoardId() === 'atlas-launch-plan' && launchPlanActivityIds.has(item.id))) : activityItems;
    }

    const screenFiles = { login: '01-access.html', home: '02-home.html', activity: '03-activity.html', templates: '04-templates.html', board: '05-board.html' };
    const stateStorageKey = 'memento-mockup-state-v1';
    function route() { return document.body.dataset.screen || 'login'; }
    function persistMockState() {
      try { sessionStorage.setItem(stateStorageKey, JSON.stringify({ state, workspaces, boardSignals, activityItems })); } catch {}
    }
    function restoreMockState() {
      try {
        const saved = JSON.parse(sessionStorage.getItem(stateStorageKey) || 'null');
        if (!saved) return;
        if (saved.state) Object.assign(state, saved.state);
        if (Array.isArray(saved.workspaces)) workspaces.splice(0, workspaces.length, ...saved.workspaces);
        if (Array.isArray(saved.activityItems)) activityItems.splice(0, activityItems.length, ...saved.activityItems);
        if (saved.boardSignals) Object.assign(boardSignals, saved.boardSignals);
      } catch {}
    }
    function navigateTo(screen) {
      persistMockState();
      const target = new URL(screenFiles[screen] || screenFiles.login, location.href);
      target.searchParams.set('rev', '20261027r');
      if (['activity', 'board'].includes(screen) && isBoardActivity()) target.searchParams.set('board', activeBoardId());
      location.href = target.href;
    }
    restoreMockState();
    function iconSearch() { return '<svg viewBox="0 0 24 24" fill="none" aria-hidden="true"><circle cx="10.8" cy="10.8" r="6.3" stroke="currentColor" stroke-width="1.7"/><path d="m16 16 4 4" stroke="currentColor" stroke-width="1.7" stroke-linecap="round"/></svg>'; }
    function iconBell() { return '<svg viewBox="0 0 24 24" fill="none" aria-hidden="true"><path d="M18 9a6 6 0 0 0-12 0c0 7-3 7-3 9h18c0-2-3-2-3-9M10 21h4" stroke="currentColor" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round"/></svg>'; }
    function iconStar(filled = false) { return `<svg viewBox="0 0 24 24" fill="${filled ? 'currentColor' : 'none'}" aria-hidden="true"><path d="m12 3 2.75 5.58 6.16.9-4.45 4.34 1.05 6.13L12 17.06l-5.51 2.89 1.05-6.13-4.45-4.34 6.16-.9L12 3Z" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"/></svg>`; }
    function iconBoard(name) {
      const shapes = {
        menu: '<path d="M4 6h16M4 12h16M4 18h16"/>',
        members: '<circle cx="9" cy="8" r="3"/><path d="M3 21v-3a6 6 0 0 1 12 0v3M17 5a3 3 0 0 1 0 6M21 21v-3a6 6 0 0 0-3-5"/>',
        select: '<path d="m4 3 7 18 2-8 8-2Z"/>',
        hand: '<path d="M8 13V6a2 2 0 0 1 4 0v6-8a2 2 0 0 1 4 0v8-6a2 2 0 0 1 4 0v8c0 5-3 8-7 8-3 0-5-2-7-5l-3-4a2 2 0 0 1 3-3l2 3Zm12-1V9a2 2 0 0 1 4 0v5" transform="translate(-1 -1) scale(.95)"/>',
        pen: '<path d="m16 3 5 5M3 21l5-1L21 7a2 2 0 0 0-5-5L3 15Z"/>',
        erase: '<path d="m16 3 5 5a2 2 0 0 1 0 3L11 21H6l-4-4a2 2 0 0 1 0-3L13 3a2 2 0 0 1 3 0ZM7 9l8 8M11 21h10"/>',
        note: '<path d="M14 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h9l7-7V5a2 2 0 0 0-2-2Zm0 18v-7h7"/>',
        frame: '<path d="M4 2v20M20 2v20M2 4h20M2 20h20"/>',
        text: '<path d="M4 7V4h16v3M12 4v16M8 20h8"/>',
        shape: '<rect x="3" y="3" width="18" height="18" rx="2"/>',
        undo: '<path d="M3 10h11a7 7 0 0 1 0 14" transform="translate(0 -4)"/><path d="m8 1-5 5 5 5"/>',
        redo: '<path d="M21 6H10a7 7 0 0 0 0 14M16 1l5 5-5 5"/>',
        reset: '<path d="M3 10a9 9 0 1 1 2 8M3 3v7h7"/>',
        agent: '<rect x="4" y="7" width="16" height="14" rx="2"/><path d="M12 3v4M2 12h2M20 12h2M8 16h8M9 11v1M15 11v1"/>',
        external: '<path d="M7 17 17 7M7 7h10v10"/>',
        search: '<circle cx="10" cy="10" r="6"/><path d="m15 15 5 5"/>',
        arrow: '<path d="M4 12h16m-6-6 6 6-6 6"/>',
        link: '<path d="M4 4v8h16m-6-6 6 6-6 6"/>',
        minus: '<path d="M5 12h14"/>',
        plus: '<path d="M5 12h14M12 5v14"/>',
        fit: '<path d="M8 3H3v5M16 3h5v5M21 16v5h-5M8 21H3v-5"/>',
        more: '<circle cx="5" cy="12" r="1"/><circle cx="12" cy="12" r="1"/><circle cx="19" cy="12" r="1"/>',
        grip: '<circle cx="6" cy="8" r="1"/><circle cx="12" cy="8" r="1"/><circle cx="18" cy="8" r="1"/><circle cx="6" cy="16" r="1"/><circle cx="12" cy="16" r="1"/><circle cx="18" cy="16" r="1"/>',
        check: '<path d="m5 12 4 4L19 6"/>',
        close: '<path d="m6 6 12 12M6 18 18 6"/>',
        panel: '<rect x="3" y="4" width="18" height="16" rx="2"/><path d="M15 4v16"/>',
        chat: '<path d="M21 12a8 8 0 0 1-11.6 7.1L4 20.5l1.4-4.9A8 8 0 1 1 21 12Z"/><path d="M8.5 12h.01M12 12h.01M15.5 12h.01"/>',
        chevron: '<path d="m9 6 6 6-6 6"/>'
      };
      return `<svg class="board-icon-svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" focusable="false">${shapes[name]}</svg>`;
    }
    function render() {
      const current = route();
      if (current === 'login' || current === 'invite') renderLogin(current === 'invite');
      else if (current === 'activity') renderApp('activity');
      else if (current === 'templates') renderApp('templates');
      else if (current === 'board') renderApp('board');
      else renderApp('home');
    }
    function renderLogin(invite = false) {
      document.querySelector('#navigation-toggle')?.remove();
      app.replaceChildren(document.querySelector('#login-view').content.cloneNode(true));
      document.querySelector('.brand').href = '01-access.html';
      topbarRight.innerHTML = '<span>New to Memento?</span><button class="quiet-button" type="button" data-action="demo-login">Explore the demo</button>';
      renderThemeControl();
      activityRoot.innerHTML = '';
      const accountForm = document.querySelector('#account-form');
      const inviteForm = document.querySelector('#invite-form');
      const tabs = [...document.querySelectorAll('.entry-tab')];
      function showMode(mode) {
        const isInvite = mode === 'invite';
        accountForm.classList.toggle('hidden', isInvite);
        inviteForm.classList.toggle('hidden', !isInvite);
        document.querySelector('#login-eyebrow').textContent = isInvite ? 'Invitation access' : 'Your workspace';
        document.querySelector('#login-heading').textContent = isInvite ? 'Join a workspace' : 'Sign in to your workspace';
        document.querySelector('#login-subheading').textContent = isInvite ? 'Choose how the team will see you, then join with the access they shared.' : 'Access your workspaces and pick up where you left off.';
        tabs.forEach(tab => tab.setAttribute('aria-selected', String(tab.dataset.entry === mode)));
      }
      tabs.forEach(tab => tab.addEventListener('click', () => showMode(tab.dataset.entry)));
      if (invite) showMode('invite');
      accountForm.addEventListener('submit', event => { event.preventDefault(); state.name = document.querySelector('#email').value.split('@')[0].replace(/[._-]/g, ' ').replace(/\b\w/g, c => c.toUpperCase()) || 'Maya Chen'; navigateTo('home'); });
      inviteForm.addEventListener('submit', event => { event.preventDefault(); state.name = document.querySelector('#invite-name').value.trim(); navigateTo('home'); });
    }
    function renderApp(page) {
      app.replaceChildren(document.querySelector('#app-view').content.cloneNode(true));
      document.querySelector('.brand').href = '02-home.html?rev=20261027r';
      if (!document.querySelector('#navigation-toggle')) {
        document.querySelector('.topbar').insertAdjacentHTML('afterbegin', `<button class="icon-button navigation-toggle" id="navigation-toggle" type="button" data-action="toggle-navigation" aria-controls="main-navigation">${iconBoard('menu')}</button>`);
      }
      document.querySelector('.sidebar').id = 'main-navigation';
      updateNavigation();
      if (page === 'activity') app.querySelector('.app-frame').classList.add('activity-scroll');
      const unreadCount = scopedActivityItems().filter(item => !item.read).length;
      topbarRight.innerHTML = `<button class="icon-button activity-bell" type="button" data-action="toggle-activity" aria-label="Recent activity, ${unreadCount} unread" aria-expanded="false" title="Recent activity">${iconBell()}${unreadCount ? `<span class="activity-badge">${unreadCount}</span>` : ''}</button><span>Demo session</span><span class="avatar" aria-label="${escapeHtml(state.name)}">${initials(state.name)}</span>`;
      renderThemeControl();
      updateActivityButton();
      activityRoot.innerHTML = '';
      document.querySelector('#profile-avatar').textContent = initials(state.name);
      document.querySelector('#profile-name').textContent = state.name;
      document.querySelector('.side-link[data-filter="owned"] .side-count').textContent = workspaces.filter(space => space.type === 'owned').length;
      document.querySelector('.side-link[data-filter="shared"] .side-count').textContent = workspaces.filter(space => space.type === 'shared').length;
      document.querySelectorAll('.side-link').forEach(link => {
        if ((page === 'templates' && link.dataset.route === 'templates') || (page === 'activity' && link.dataset.route === 'activity') || (page === 'home' && link.textContent.trim().startsWith('Home') && state.workspaceFilter === 'all') || (page === 'home' && link.dataset.filter === state.workspaceFilter && state.workspaceFilter !== 'all')) link.setAttribute('aria-current', 'page');
        link.addEventListener('click', () => {
          if (link.dataset.filter) {
            state.workspaceFilter = link.dataset.filter;
            persistMockState();
            if (route() === 'home') renderApp('home');
            else navigateTo('home');
          } else {
            state.workspaceFilter = 'all';
            state.search = '';
            persistMockState();
            if (route() === link.dataset.route) renderApp(route());
            else navigateTo(link.dataset.route);
          }
        });
      });
      if (page === 'home') renderHome();
      else if (page === 'activity') renderActivity();
      else if (page === 'templates') renderTemplates();
      else (activeBoardId() === 'atlas-launch-plan' ? window.AtlasBoard : window.BoardDemo).mount(document.querySelector('#view-content'), {
        icon: iconBoard,
        notify: showToast,
        onEvent(event) {
          activityItems.unshift(event);
          persistMockState();
          updateActivityButton();
        },
        onReset(boardId) {
          for (let index = activityItems.length - 1; index >= 0; index--) {
            if (activityItems[index].board === boardId) activityItems.splice(index, 1);
          }
          persistMockState();
          updateActivityButton();
        }
      });
    }
    function updateNavigation() {
      const sidebar = document.querySelector('.sidebar');
      if (!sidebar) return;
      const open = state.sidebarOpen ?? route() !== 'board';
      sidebar.hidden = !open;
      sidebar.closest('.app-frame').classList.toggle('navigation-closed', !open);
      const toggle = document.querySelector('#navigation-toggle');
      toggle.setAttribute('aria-expanded', String(open));
      toggle.setAttribute('aria-label', open ? 'Close navigation' : 'Open navigation');
      toggle.title = open ? 'Close navigation' : 'Open navigation';
    }
    function renderHome() {
      const root = document.querySelector('#view-content');
      root.innerHTML = `
        <div class="page-head">
          <div><span class="eyebrow">Your workspace</span><h1>Hello, ${escapeHtml(state.name.split(' ')[0])}</h1><p>Return to a project or set up your next workspace.</p></div>
          <div class="head-actions"><button class="primary-button" type="button" data-action="new-workspace"><span aria-hidden="true">＋</span> New workspace</button></div>
        </div>
        <div class="search-row"><label class="search-box">${iconSearch()}<input id="workspace-search" type="search" placeholder="Search workspaces and projects" value="${escapeAttribute(state.search)}" aria-label="Search workspaces"></label><div class="segmented" role="group" aria-label="Filter workspaces"><button class="segment" data-workspace-filter="all" aria-pressed="${state.workspaceFilter === 'all'}">All</button><button class="segment" data-workspace-filter="owned" aria-pressed="${state.workspaceFilter === 'owned'}">Owned</button><button class="segment" data-workspace-filter="shared" aria-pressed="${state.workspaceFilter === 'shared'}">Shared</button></div></div>
        <div class="section-head"><h2>Workspaces</h2><small id="workspace-count"></small></div>
        <div class="workspace-grid" id="workspace-grid"></div>
        <div class="section-head board-section-head"><h2 id="boards-heading">Recent boards</h2><div class="board-head-actions"><div class="segmented" role="group" aria-label="Filter boards"><button class="segment" data-board-view="recent" aria-pressed="${state.boardView === 'recent'}">Recent</button><button class="segment" data-board-view="favorites" aria-pressed="${state.boardView === 'favorites'}">Favorites</button></div><button class="text-button" type="button" data-action="all-activity">View activity →</button></div></div>
        <div class="recent-grid" id="recent-grid"></div>
        <div class="starter-strip"><div><strong>Starting something new?</strong><p>Start from a template to give your team context from day one.</p></div><button class="outline-button" type="button" data-action="browse-templates">Browse templates <span aria-hidden="true">→</span></button></div>`;
      const search = root.querySelector('#workspace-search');
      search.addEventListener('input', () => { state.search = search.value; drawWorkspaces(); });
      root.querySelectorAll('[data-workspace-filter]').forEach(button => button.addEventListener('click', () => {
        state.workspaceFilter = button.dataset.workspaceFilter;
        root.querySelectorAll('[data-workspace-filter]').forEach(item => item.setAttribute('aria-pressed', String(item === button)));
        drawWorkspaces();
      }));
      root.querySelectorAll('[data-board-view]').forEach(button => button.addEventListener('click', () => {
        state.boardView = button.dataset.boardView;
        root.querySelectorAll('[data-board-view]').forEach(item => item.setAttribute('aria-pressed', String(item === button)));
        document.querySelector('#boards-heading').textContent = state.boardView === 'favorites' ? 'Favorite boards' : 'Recent boards';
        drawWorkspaces();
      }));
      drawWorkspaces();
    }
    function drawWorkspaces() {
      const grid = document.querySelector('#workspace-grid');
      if (!grid) return;
      const demoBoards = window.BoardDemo.list(showToast);
      demoBoards.filter(item => item.workspaceId).forEach(item => {
        if (!workspaces.some(space => space.id === item.workspaceId)) workspaces.push({ id: item.workspaceId, name: item.workspaceName, description: 'A new space to organize your work.', type: 'owned', marker: '', activity: 'Created locally', people: [initials(state.name)], boards: [] });
      });
      const filtered = workspaces.filter(space => (state.workspaceFilter === 'all' || space.type === state.workspaceFilter) && `${space.name} ${space.description}`.toLowerCase().includes(state.search.toLowerCase()));
      document.querySelector('#workspace-count').textContent = `${filtered.length} ${filtered.length === 1 ? 'workspace' : 'workspaces'}`;
      grid.innerHTML = filtered.length ? filtered.map(space => workspaceCard(space, demoBoards)).join('') : '<div class="empty-state">No workspaces match that name.</div>';
      grid.querySelectorAll('[data-open-workspace]').forEach(card => card.addEventListener('click', () => {
        const first = demoBoards.find(item => item.workspaceId === card.dataset.openWorkspace);
        if (first) location.href = `05-board.html?board=${first.id}`;
        else if (card.dataset.openWorkspace === 'atlas') navigateTo('board');
        else {
          const space = workspaces.find(item => item.id === card.dataset.openWorkspace);
          showToast(`Next mockup: board “${space.name}”.`);
        }
      }));
      const recent = document.querySelector('#recent-grid');
      const boards = filtered.flatMap(space => space.boards.map(board => ({ board, space, signal: boardSignals[board] || { favorite: false, changes: 0, lastOpened: space.activity } })));
      const visibleBoards = (state.boardView === 'favorites' ? boards.filter(item => item.signal.favorite) : boards).slice(0, 3);
      recent.innerHTML = visibleBoards.map(({ board, space, signal }) => `<article class="board-card"><button class="board-open" type="button" data-open-board="${escapeAttribute(board)}" data-board-space="${escapeAttribute(space.name)}"><span class="board-thumbnail">${boardPreview(space.id === 'atlas' && board === 'Launch plan' ? 'memento-atlas-board-v2' : null, board)}</span><span class="board-info"><strong>${escapeHtml(board)}</strong><small>${escapeHtml(space.name)} · ${escapeHtml(signal.lastOpened)}</small>${signal.changes ? `<span class="board-signal"><span class="board-updates">${signal.changes} ${signal.changes === 1 ? 'update' : 'updates'} since your last visit</span></span>` : ''}</span></button><button class="board-favorite" type="button" data-toggle-favorite="${escapeAttribute(board)}" aria-label="${signal.favorite ? 'Remove' : 'Add'} ${escapeAttribute(board)} ${signal.favorite ? 'from' : 'to'} favorites" aria-pressed="${signal.favorite}" title="${signal.favorite ? 'Remove from favorites' : 'Add to favorites'}">${iconStar(signal.favorite)}</button></article>`).join('') || `<div class="board-empty">${state.boardView === 'favorites' ? 'You have no favorite boards yet.' : 'Recent boards will appear here.'}</div>`;
      if (state.boardView !== 'favorites' && state.workspaceFilter !== 'shared') {
        demoBoards.filter(item => filtered.some(space => space.id === (item.workspaceId || 'atlas'))).forEach(item => {
          recent.insertAdjacentHTML('beforeend', `<article class="board-card"><a class="board-open" href="05-board.html?board=${encodeURIComponent(item.id)}"><span class="board-thumbnail">${boardPreview('memento-demo-board-' + item.id, item.name)}</span><span class="board-info"><strong>${escapeHtml(item.name)}</strong><small>${escapeHtml(item.workspaceName || 'Atlas Launch')} / From-zero experience demo</small></span></a></article>`);
        });
      }
      recent.querySelectorAll('[data-toggle-favorite]').forEach(button => button.addEventListener('click', () => {
        const board = button.dataset.toggleFavorite;
        const signal = boardSignals[board] || (boardSignals[board] = { favorite: false, changes: 0, lastOpened: 'Created just now' });
        signal.favorite = !signal.favorite;
        drawWorkspaces();
      }));
      recent.querySelectorAll('[data-open-board]').forEach(button => button.addEventListener('click', () => {
        const board = button.dataset.openBoard;
        const signal = boardSignals[board] || (boardSignals[board] = { favorite: false, changes: 0, lastOpened: 'Created just now' });
        signal.changes = 0;
        signal.lastOpened = 'Opened just now';
        persistMockState();
        if (board === 'Launch plan' && button.dataset.boardSpace === 'Atlas Launch') navigateTo('board');
        else {
          drawWorkspaces();
          showToast(`Next mockup: board “${board}”. Updates marked as seen.`);
        }
      }));
    }
    function renderBoard() {
      const root = document.querySelector('#view-content');
      root.closest('.main-content').classList.add('board-content');
      root.closest('.app-frame').classList.add('board-app');
      root.innerHTML = `
        <div class="flow-board">
          <header class="flow-toolbar">
            <div class="flow-board-title"><span class="flow-kicker">WORKSPACE</span><strong>Atlas Launch</strong></div>
            <div class="flow-history" aria-label="Board history"><button class="icon-button" type="button" aria-label="Undo" title="Undo">${iconBoard('undo')}</button><button class="icon-button" type="button" aria-label="Redo" title="Redo">${iconBoard('redo')}</button><i></i><button class="icon-button" type="button" aria-label="Reset board view" title="Reset board view" data-action="board-fit">${iconBoard('reset')}</button></div>
            <div class="flow-toolbar-end"><span class="board-presence" aria-label="Maya, Leo and Sam are here"><i>MC</i><i>LM</i><i>SR</i></span></div>
          </header>
          <div class="flow-board-layout">
            <section class="flow-viewport tool-select" aria-label="Infinite Atlas Launch board" tabindex="0">
              <div class="flow-caption"><span>Atlas briefing / 10:00 → 11:00</span><span><i class="flow-legend-process"></i> Process flow <i class="flow-legend-note"></i> Notes <i class="flow-legend-evidence"></i> Evidence</span></div>
              <div class="flow-tool-rail" role="toolbar" aria-label="Board tools">
                <button class="icon-button" type="button" title="Select" aria-label="Select" data-board-tool="select" aria-pressed="${(state.boardTool || 'select') === 'select'}">${iconBoard('select')}</button>
                <button class="icon-button" type="button" title="Pan" aria-label="Pan" data-board-tool="hand" aria-pressed="${state.boardTool === 'hand'}">${iconBoard('hand')}</button>
                <button class="icon-button" type="button" title="Pen" aria-label="Pen" data-board-tool="pen" aria-pressed="${state.boardTool === 'pen'}">${iconBoard('pen')}</button>
                <button class="icon-button" type="button" title="Erase" aria-label="Erase" data-board-tool="erase" aria-pressed="${state.boardTool === 'erase'}">${iconBoard('erase')}</button>
                <i></i>
                <button class="icon-button" type="button" title="Add note" aria-label="Add note" data-action="board-add-note" aria-pressed="false">${iconBoard('note')}</button>
                <button class="icon-button" type="button" title="Add frame" aria-label="Add frame" data-action="board-add-frame">${iconBoard('frame')}</button>
                <button class="icon-button" type="button" title="Add text" aria-label="Add text" data-action="board-add-text">${iconBoard('text')}</button>
                <button class="icon-button" type="button" title="Add shape" aria-label="Add shape" data-action="board-add-shape">${iconBoard('shape')}</button>
              </div>
              <div class="flow-zoom-controls" aria-label="Canvas zoom controls"><button class="icon-button" type="button" aria-label="Zoom out" data-zoom="-1">${iconBoard('minus')}</button><span class="flow-zoom-level">72%</span><button class="icon-button" type="button" aria-label="Zoom in" data-zoom="1">${iconBoard('plus')}</button><i></i><button class="icon-button" type="button" aria-label="Fit board to view" title="Fit board to view" data-action="board-fit">${iconBoard('fit')}</button></div>
              <div class="flow-world" style="transform:translate(18px,18px) scale(.72)">
                <svg class="flow-edges" viewBox="0 0 1600 980" aria-label="Board relationships">
                  <defs><marker id="flow-arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse"><path d="M 0 0 L 10 5 L 0 10 z"></path></marker></defs>
                  <path data-edge="email-proposal" class="edge-evidence" marker-end="url(#flow-arrow)"></path><text data-edge-label="email-proposal">supports</text>
                  <path data-edge="note-proposal" class="edge-evidence" marker-end="url(#flow-arrow)"></path><text data-edge-label="note-proposal">supports</text>
                  <path data-edge="proposal-decision" class="edge-process" marker-end="url(#flow-arrow)"></path><text data-edge-label="proposal-decision">adopted as</text>
                  <path data-edge="decision-task" class="edge-action" marker-end="url(#flow-arrow)"></path><text data-edge-label="decision-task">results in</text>
                </svg>
                <div class="flow-frame" style="left:70px;top:85px;width:445px;height:715px"><div class="flow-frame-heading"><span>Incoming context</span><small>2 objects</small></div></div>
                <div class="flow-frame frame-thinking" style="left:555px;top:85px;width:445px;height:715px"><div class="flow-frame-heading"><span>Team thinking</span><small>2 objects</small></div></div>
                <div class="flow-frame frame-outcome" style="left:1040px;top:85px;width:460px;height:715px"><div class="flow-frame-heading"><span>Decision & next steps</span><small>2 objects</small></div></div>
                <article class="flow-node flow-source" role="button" aria-label="Email source: Briefing time change" style="left:125px;top:155px;width:315px;height:260px" data-board-item="email" aria-pressed="${state.boardSelection === 'email'}" tabindex="0">
                  <div class="flow-node-header"><span class="flow-grip">${iconBoard('grip')}</span><span>EMAIL CONVERSATION · E1</span><button class="icon-button" type="button" aria-label="Inspect email">${iconBoard('more')}</button></div>
                  <div class="flow-node-content"><strong>Briefing time change</strong><p class="flow-source-excerpt">The speaker is unavailable at 10:00. Could we move the briefing to 11:00?</p><div class="flow-node-meta">Jordan Lee · Today, 9:42 AM</div><button class="quiet-button" type="button" data-action="open-source">Inspect source ${iconBoard('external')}</button><i class="flow-handle source-handle"></i></div>
                </article>
                <article class="flow-node flow-node-note note-amber" role="button" aria-label="Maya's note: Keep the launch date fixed" style="left:165px;top:535px;width:265px;height:165px" data-board-item="constraint" aria-pressed="${state.boardSelection === 'constraint'}" tabindex="0">
                  <div class="flow-node-header"><span class="flow-grip">${iconBoard('grip')}</span><span>ANNOTATION · MAYA</span><button class="icon-button" type="button" aria-label="Edit note">${iconBoard('pen')}</button></div><div class="flow-node-content"><strong>Keep the launch date fixed</strong><div class="flow-node-meta">Maya / note</div></div><i class="flow-handle source-handle"></i>
                </article>
                <article class="flow-node flow-agent" role="button" aria-label="Facilitator proposal: Move the briefing to 11:00" style="left:615px;top:150px;width:330px;height:290px" data-board-item="proposal" aria-pressed="${state.boardSelection === 'proposal'}" tabindex="0">
                  <div class="flow-node-header"><span class="flow-grip">${iconBoard('grip')}</span><span>FACILITATOR · PROPOSAL</span><button class="icon-button" type="button" aria-label="Inspect proposal">${iconBoard('more')}</button></div><div class="flow-node-content"><strong>Move the briefing to 11:00</strong><p>The speaker’s availability conflicts with the current 10:00 plan. Keep the launch date unchanged.</p><div class="flow-evidence-list"><span>E1 · Email</span><span>Constraint · Maya</span><span>Decision · 10:00</span></div><div class="flow-node-meta">Adopted by Maya · Agent run complete</div></div>
                </article>
                <article class="flow-node flow-node-note note-mint" role="button" aria-label="Leo's note: Notify guests before confirming" style="left:655px;top:555px;width:270px;height:155px" data-board-item="agenda" aria-pressed="${state.boardSelection === 'agenda'}" tabindex="0">
                  <div class="flow-node-header"><span class="flow-grip">${iconBoard('grip')}</span><span>ANNOTATION · LEO</span><button class="icon-button" type="button" aria-label="Edit note">${iconBoard('pen')}</button></div><div class="flow-node-content"><strong>Notify guests before confirming</strong><div class="flow-node-meta">Leo / note</div></div>
                </article>
                <article class="flow-node flow-decision" role="button" aria-label="Decision: Hold the briefing at 11:00. Keep the launch date." style="left:1100px;top:170px;width:330px;height:270px" data-board-item="decision" aria-pressed="${state.boardSelection === 'decision'}" tabindex="0">
                  <div class="flow-node-header"><span class="flow-grip">${iconBoard('grip')}</span><span>DECISION · MAYA</span><button class="icon-button" type="button" aria-label="Inspect decision">${iconBoard('more')}</button></div><div class="flow-node-content"><strong>Hold the briefing at 11:00. Keep the launch date.</strong><p>The speaker is unavailable at 10:00; moving the briefing preserves the agreed release plan.</p><div class="flow-node-meta">Recorded today, 10:18 AM · Supersedes 10:00</div></div><i class="flow-handle target-handle"></i>
                </article>
                <article class="flow-node flow-task" role="button" aria-label="Groupware task: Update briefing invite to 11:00" style="left:1105px;top:535px;width:325px;height:195px" data-board-item="task" aria-pressed="${state.boardSelection === 'task'}" tabindex="0">
                  <div class="flow-node-header"><span class="flow-grip">${iconBoard('grip')}</span><span>GROUPWARE TASK · ACTION</span><button class="icon-button" type="button" aria-label="Inspect action">${iconBoard('more')}</button></div><div class="flow-node-content"><strong>Update briefing invite to 11:00</strong><p>Leo Martin · Due today · Deck</p><span class="flow-task-status ${state.taskApproved ? 'approved' : ''}">${state.taskApproved ? 'Approved' : 'Awaiting approval'}</span><button class="quiet-button" type="button" data-action="review-action">${state.taskApproved ? 'View approved action' : 'Review action'} ${iconBoard('arrow')}</button></div><i class="flow-handle target-handle"></i></article>
              </div>
            </section>
            <aside class="flow-inspector" aria-label="Inspector"><div class="inspector-title"><span>Inspector</span><span class="flow-kicker">ATLAS</span></div><div id="board-inspector"></div><section class="flow-inspector-activity" aria-label="Launch plan recent activity"></section></aside>
          </div>
          <footer class="flow-status"><i></i><span id="flow-status-message">Interactive mockup · board actions are local</span><span class="flow-status-right">Atlas Launch · Shared board</span></footer>
        </div>`;
      root.querySelectorAll('[data-board-item]').forEach(node => node.addEventListener('click', event => {
        selectBoardNode(node);
      }));
      root.querySelectorAll('[data-board-item]').forEach(node => node.addEventListener('keydown', event => {
        if (event.key === 'Enter' || event.key === ' ') { event.preventDefault(); selectBoardNode(node); }
      }));
      root.querySelectorAll('[data-board-tool]').forEach(button => button.addEventListener('click', () => {
        selectBoardTool(button.dataset.boardTool);
      }));
      root.querySelectorAll('.flow-node-header button').forEach(button => button.addEventListener('click', event => {
        event.stopPropagation();
        showToast(`${button.getAttribute('aria-label')} is shown as a preview control.`);
      }));
      root.querySelectorAll('[data-zoom]').forEach(button => button.addEventListener('click', () => zoomBoard(Number(button.dataset.zoom) * .1)));
      root.querySelectorAll('[data-action="board-fit"]').forEach(button => button.addEventListener('click', fitBoardView));
      root.querySelector('[data-action="board-add-note"]').addEventListener('click', () => selectBoardTool('note'));
      root.querySelectorAll('[data-action="board-add-frame"],[data-action="board-add-text"],[data-action="board-add-shape"]').forEach(button => button.addEventListener('click', () => showToast(`${button.title} is shown as a tool preview in this mockup.`)));
      root.querySelectorAll('.flow-history button:not([data-action])').forEach(button => button.addEventListener('click', () => showToast(`${button.title} is available in the full React Flow prototype.`)));
      root.querySelector('[data-action="review-action"]').addEventListener('click', event => { event.stopPropagation(); openActionReview(); });
      const viewport = root.querySelector('.flow-viewport');
      viewport.addEventListener('wheel', event => {
        event.preventDefault();
        zoomBoard(event.deltaY < 0 ? .06 : -.06, event.clientX, event.clientY);
      }, { passive: false });
      viewport.addEventListener('pointerdown', startBoardGesture);
      viewport.addEventListener('pointermove', moveBoardGesture);
      viewport.addEventListener('pointerup', endBoardGesture);
      viewport.addEventListener('pointercancel', endBoardGesture);
      drawBoardInspector();
      drawBoardActivity();
      updateBoardEdges();
    }
    function drawBoardActivity() {
      const panel = document.querySelector('.flow-inspector-activity');
      if (!panel) return;
      panel.innerHTML = `<h3>Recent activity <button class="text-button" type="button" data-action="all-activity">View all ${iconBoard('arrow')}</button></h3><div class="inspector-meta">Launch plan only</div>${scopedActivityItems().slice(0, 3).map(item => `<p><i class="activity-avatar">${iconBoard(item.category === 'agent' ? 'agent' : item.category === 'sources' ? 'external' : 'pen')}</i><span><strong>${escapeHtml(item.actor)}</strong> · ${escapeHtml(item.title)}<small>${escapeHtml(item.time)}</small></span></p>`).join('')}`;
    }
    function selectBoardNode(node) {
      state.boardSelection = node.dataset.boardItem;
      document.querySelectorAll('[data-board-item]').forEach(item => item.setAttribute('aria-pressed', String(item === node)));
      drawBoardInspector();
    }
    function selectBoardTool(tool) {
      state.boardTool = tool;
      document.querySelectorAll('[data-board-tool]').forEach(button => button.setAttribute('aria-pressed', String(button.dataset.boardTool === tool)));
      document.querySelector('[data-action="board-add-note"]')?.setAttribute('aria-pressed', String(tool === 'note'));
      const viewport = document.querySelector('.flow-viewport');
      if (viewport) viewport.className = `flow-viewport tool-${tool}`;
      const message = document.querySelector('#flow-status-message');
      if (message) message.textContent = tool === 'note' ? 'Click the canvas to place a note' : `${tool[0].toUpperCase()}${tool.slice(1)} tool selected`;
    }
    function addBoardNote(clientX, clientY) {
      const camera = getBoardViewport(), bounds = document.querySelector('.flow-viewport').getBoundingClientRect();
      const id = `note-${Date.now()}`;
      const note = document.createElement('article');
      note.className = 'flow-node flow-node-note note-amber';
      note.setAttribute('role', 'button');
      note.setAttribute('aria-label', 'New note');
      note.setAttribute('tabindex', '0');
      note.setAttribute('aria-pressed', 'false');
      note.dataset.boardItem = id;
      note.style.cssText = `left:${Math.max(30, (clientX - bounds.left - camera.x) / camera.zoom)}px;top:${Math.max(70, (clientY - bounds.top - camera.y) / camera.zoom)}px;width:265px;height:150px`;
      note.innerHTML = `<div class="flow-node-header"><span class="flow-grip">${iconBoard('grip')}</span><span>ANNOTATION · MAYA</span><button class="icon-button" type="button" aria-label="Edit note">${iconBoard('pen')}</button></div><div class="flow-node-content"><strong>New note</strong><div class="flow-node-meta">Maya / note · Just now</div></div>`;
      note.addEventListener('click', () => selectBoardNode(note));
      note.addEventListener('keydown', event => { if (event.key === 'Enter' || event.key === ' ') { event.preventDefault(); selectBoardNode(note); } });
      document.querySelector('.flow-world').append(note);
      state.boardNotes.push(id);
      state.boardSelection = id;
      document.querySelectorAll('[data-board-item]').forEach(item => item.setAttribute('aria-pressed', String(item === note)));
      drawBoardInspector();
      selectBoardTool('select');
    }
    function getBoardViewport() {
      const world = document.querySelector('.flow-world');
      if (!world) return null;
      const match = world.style.transform.match(/translate\(([-\d.]+)px,\s*([-\d.]+)px\)\s*scale\(([\d.]+)\)/);
      return { world, x: Number(match?.[1] || 18), y: Number(match?.[2] || 18), zoom: Number(match?.[3] || .72) };
    }
    function setBoardViewport(x, y, zoom) {
      const viewport = getBoardViewport();
      if (!viewport) return;
      viewport.world.style.transform = `translate(${x}px, ${y}px) scale(${zoom})`;
      const level = document.querySelector('.flow-zoom-level');
      if (level) level.textContent = `${Math.round(zoom * 100)}%`;
      updateBoardEdges();
    }
    function zoomBoard(delta, pointerX, pointerY) {
      const current = getBoardViewport();
      if (!current) return;
      const zoom = Math.min(1.2, Math.max(.35, current.zoom + delta));
      if (pointerX !== undefined && pointerY !== undefined) {
        const bounds = document.querySelector('.flow-viewport').getBoundingClientRect();
        const localX = pointerX - bounds.left, localY = pointerY - bounds.top;
        const factor = zoom / current.zoom;
        setBoardViewport(localX - (localX - current.x) * factor, localY - (localY - current.y) * factor, zoom);
      } else setBoardViewport(current.x, current.y, zoom);
    }
    function fitBoardView() {
      setBoardViewport(18, 18, .72);
      document.querySelector('#flow-status-message').textContent = 'Board fitted to view';
    }
    function startBoardGesture(event) {
      const viewport = event.currentTarget, node = event.target.closest('.flow-node'), header = event.target.closest('.flow-node-header');
      if (event.target.closest('.flow-tool-rail, .flow-zoom-controls')) return;
      const camera = getBoardViewport();
      if (!node && state.boardTool === 'note') { addBoardNote(event.clientX, event.clientY); return; }
      if (node && header && !event.target.closest('button') && state.boardTool !== 'hand') {
        viewport.setPointerCapture(event.pointerId);
        window.boardGesture = { kind: 'node', pointerId: event.pointerId, node, startX: event.clientX, startY: event.clientY, left: node.offsetLeft, top: node.offsetTop, zoom: camera.zoom };
      } else if (!node && (state.boardTool === 'hand' || !event.target.closest('button'))) {
        viewport.setPointerCapture(event.pointerId);
        window.boardGesture = { kind: 'pan', pointerId: event.pointerId, startX: event.clientX, startY: event.clientY, x: camera.x, y: camera.y, zoom: camera.zoom };
      }
    }
    function moveBoardGesture(event) {
      const gesture = window.boardGesture;
      if (!gesture || gesture.pointerId !== event.pointerId) return;
      if (gesture.kind === 'node') {
        gesture.node.style.left = `${gesture.left + (event.clientX - gesture.startX) / gesture.zoom}px`;
        gesture.node.style.top = `${gesture.top + (event.clientY - gesture.startY) / gesture.zoom}px`;
        updateBoardEdges();
      } else setBoardViewport(gesture.x + event.clientX - gesture.startX, gesture.y + event.clientY - gesture.startY, gesture.zoom);
    }
    function endBoardGesture(event) {
      if (window.boardGesture?.pointerId === event.pointerId) {
        window.boardGesture = null;
        const status = document.querySelector('#flow-status-message');
        if (status) status.textContent = 'Interactive mockup · board actions are local';
      }
    }
    function updateBoardEdges() {
      const world = document.querySelector('.flow-world');
      if (!world) return;
      const points = {
        'email-proposal': ['email', 'proposal'],
        'note-proposal': ['constraint', 'proposal'],
        'proposal-decision': ['proposal', 'decision'],
        'decision-task': ['decision', 'task']
      };
      for (const [edge, [sourceId, targetId]] of Object.entries(points)) {
        const source = world.querySelector(`[data-board-item="${sourceId}"]`), target = world.querySelector(`[data-board-item="${targetId}"]`);
        if (!source || !target) continue;
        const sx = source.offsetLeft + source.offsetWidth, sy = source.offsetTop + source.offsetHeight / 2;
        const tx = target.offsetLeft, ty = target.offsetTop + target.offsetHeight / 2;
        const curve = Math.max(70, Math.abs(tx - sx) * .45);
        const path = world.querySelector(`[data-edge="${edge}"]`);
        const label = world.querySelector(`[data-edge-label="${edge}"]`);
        const d = `M ${sx} ${sy} C ${sx + curve} ${sy}, ${tx - curve} ${ty}, ${tx} ${ty}`;
        path.setAttribute('d', d);
        label.setAttribute('x', String((sx + tx) / 2));
        label.setAttribute('y', String((sy + ty) / 2 - 8));
      }
    }
    function drawBoardInspector() {
      const inspector = document.querySelector('#board-inspector');
      if (!inspector) return;
      const details = {
        decision: { kind: 'Decision', color: 'decision', title: 'Briefing at 11:00', status: 'Current direction', description: 'Hold the briefing at 11:00 and keep the launch date.', meta: 'Recorded by Maya · Today, 10:18 AM', links: ['Email · Briefing time change', 'Proposal · Move briefing to 11:00', 'Earlier decision · 10:00'] },
        email: { kind: 'Source email', color: 'source', title: 'Briefing time change', status: 'Synced 5 min ago', description: 'Jordan Lee reports that the speaker cannot attend at 10:00 and asks to move the briefing to 11:00.', meta: 'Received today · Project mailbox', links: ['Supports facilitator proposal', 'Cited by current decision'] },
        constraint: { kind: 'Participant note', color: 'note', title: 'Keep the launch date fixed', status: 'Current', description: 'Changing the briefing time should not move the release.', meta: 'Added by Maya · Today, 9:55 AM', links: ['Used in facilitator context', 'Supports current decision'] },
        proposal: { kind: 'Agent proposal', color: 'agent', title: 'Move briefing to 11:00', status: 'Adopted', description: 'The speaker’s availability conflicts with the current plan. Keep the launch date unchanged.', meta: 'Facilitator · Run completed', links: ['Evidence: email E1', 'Evidence: Maya’s constraint', 'Adopted into current decision'] },
        agenda: { kind: 'Participant note', color: 'note', title: 'Notify guests before confirming', status: 'Open follow-up', description: 'Send the revised time once the room is confirmed.', meta: 'Added by Leo · Today, 10:06 AM', links: ['Related to groupware task'] },
        task: { kind: 'Action', color: 'action', title: 'Update briefing invite to 11:00', status: state.taskApproved ? 'Approved' : 'Awaiting approval', description: state.taskApproved ? 'Approved in this interactive mockup. No real groupware operation was sent.' : 'Assigned to Leo Martin · Due today. This action will update the groupware task only after a participant approves its exact content.', meta: 'Prepared by Action Coordinator', links: ['Results from current decision', 'External system: Deck'] }
      };
      if (state.boardNotes.includes(state.boardSelection)) details[state.boardSelection] = { kind: 'Participant note', color: 'note', title: 'New note', status: 'Draft · local mockup', description: 'A new note placed on the board. Drag it to arrange it with the team context.', meta: 'Added by Maya · Just now', links: [] };
      const item = details[state.boardSelection] || details.decision;
      const icons = { agent: 'agent', decision: 'check', source: 'external', action: 'arrow', note: 'pen' };
      inspector.innerHTML = `<section class="board-inspector"><div class="board-panel-heading"><div><span class="eyebrow">Inspecting · ${item.kind}</span><h2>${item.title}</h2></div><span class="inspector-type ${item.color}" aria-hidden="true">${iconBoard(icons[item.color])}</span></div><span class="inspector-status ${item.color}">${item.status}</span><p class="inspector-description">${item.description}</p><div class="inspector-meta">${item.meta}</div><div class="inspector-links"><h3>Connected context</h3>${item.links.map(link => `<div><span>${iconBoard('link')}</span>${link}</div>`).join('')}</div>${state.boardSelection === 'task' ? `<button class="primary-button inspector-action" type="button" data-action="review-action">${state.taskApproved ? 'View approved action' : 'Review action'} ${iconBoard('arrow')}</button>` : state.boardSelection === 'email' ? `<button class="outline-button inspector-action" type="button" data-action="open-source">Open original email ${iconBoard('external')}</button>` : ''}</section>`;
      inspector.querySelector('[data-action="review-action"]')?.addEventListener('click', openActionReview);
      inspector.querySelector('[data-action="open-source"]')?.addEventListener('click', () => showToast('Original email opens in the connected mail client.'));
    }
    function openActionReview() {
      overlayRoot.innerHTML = `<div class="modal-backdrop" role="presentation"><section class="modal action-review-modal" role="dialog" aria-modal="true" aria-labelledby="action-review-title"><div class="modal-head"><div><span class="eyebrow">Action review · Groupware</span><h2 id="action-review-title">Update briefing invite to 11:00</h2><p>Review the exact operation before it is sent to Deck.</p></div><button class="icon-button" type="button" data-close-modal aria-label="Close review">×</button></div><div class="review-fields"><div><span>Assignee</span><strong>Leo Martin</strong></div><div><span>Due date</span><strong>Today</strong></div><div><span>Task content</span><strong>Update the Atlas Launch briefing invitation to 11:00. Keep the launch date unchanged.</strong></div><div><span>Based on</span><strong>Decision · Briefing at 11:00</strong></div></div><div class="notice-line"><b>${state.taskApproved ? 'Mockup result' : 'Approval required'}</b><span>${state.taskApproved ? 'Marked approved in this interactive mockup. No external service was contacted.' : 'Approving authorizes this specific task update. The board layout itself does not change the external task.'}</span></div><div class="modal-actions"><button class="quiet-button" type="button" data-close-modal>Cancel</button><button class="primary-button" type="button" data-action="approve-task" ${state.taskApproved ? 'disabled' : ''}>${state.taskApproved ? 'Approved' : 'Approve task'}</button></div></section></div>`;
      overlayRoot.querySelectorAll('[data-close-modal]').forEach(button => button.addEventListener('click', closeModal));
      overlayRoot.querySelector('.modal-backdrop').addEventListener('click', event => { if (event.target === event.currentTarget) closeModal(); });
    }
    function approveTask() {
      state.taskApproved = true;
      persistMockState();
      closeModal();
      renderBoard();
      showToast('Task approved in this interactive mockup. No external service was contacted.');
    }
    function boardPreview(key, name) {
      let data = null;
      if (key) {
        try {
          const saved = localStorage.getItem(key);
          data = saved ? JSON.parse(saved) : key === 'memento-atlas-board-v2' ? window.AtlasBoard.example() : window.AtlasBoard.empty();
          if (!Array.isArray(data.records)) throw new Error('Missing board records');
        } catch (error) {
          console.error('Could not load board preview', error);
          showToast('Could not load the saved preview for ' + name + '.');
          data = null;
        }
      }
      return window.AtlasBoard.preview(data, name + ' / saved board preview');
    }
    function workspaceCard(space, demoBoards) {
      const badge = space.type === 'shared' ? '<span class="access-chip guest">Guest · Editor</span>' : '<span class="access-chip">Owner</span>';
      const people = space.people.map(person => `<i>${person}</i>`).join('');
      const first = demoBoards.find(item => item.workspaceId === space.id);
      const key = first ? 'memento-demo-board-' + first.id : space.id === 'atlas' ? 'memento-atlas-board-v2' : null;
      const name = first?.name || space.boards[0] || 'Starter board';
      return `<button class="workspace-card" type="button" data-open-workspace="${space.id}" aria-label="Open workspace ${escapeAttribute(space.name)}"><div class="workspace-preview">${boardPreview(key, name)}<span class="workspace-preview-label">${escapeHtml(name)}</span></div><div class="card-meta"><div class="card-title-row"><h3>${escapeHtml(space.name)}</h3>${badge}</div><p>${escapeHtml(space.description)}</p><div class="card-foot"><span>${escapeHtml(space.activity)}</span><span class="collaborator-stack" aria-label="${space.people.length} collaborators">${people}</span></div></div></button>`;
    }
    function renderTemplates() {
      const root = document.querySelector('#view-content');
      root.innerHTML = `<div class="page-head"><div><span class="eyebrow">Starting point</span><h1>Templates for contextual work</h1><p>Choose a structure and make it your team’s workspace.</p></div><div class="head-actions"><button class="outline-button" type="button" data-action="back-home">← Workspaces</button></div></div>
        <label class="search-box">${iconSearch()}<input id="template-search" type="search" placeholder="Search templates" value="${escapeAttribute(state.templateSearch)}" aria-label="Search templates"></label>
        <div class="template-controls" id="category-list"></div>
        <div class="section-head"><h2>Start here</h2><small id="template-count"></small></div>
        <div class="template-grid" id="template-grid"></div>`;
      const search = root.querySelector('#template-search');
      search.addEventListener('input', () => { state.templateSearch = search.value; drawTemplates(); });
      root.querySelector('[data-action="back-home"]').addEventListener('click', () => navigateTo('home'));
      drawTemplates();
    }
    function drawTemplates() {
      const categoryRoot = document.querySelector('#category-list');
      const grid = document.querySelector('#template-grid');
      if (!categoryRoot || !grid) return;
      categoryRoot.innerHTML = categories.map(category => `<button class="category-chip" type="button" data-category="${escapeAttribute(category)}" aria-pressed="${category === state.templateCategory}">${escapeHtml(category)}</button>`).join('');
      categoryRoot.querySelectorAll('[data-category]').forEach(button => button.addEventListener('click', () => { state.templateCategory = button.dataset.category; drawTemplates(); }));
      const filtered = templates.filter(item => (state.templateCategory === 'All' || item.category === state.templateCategory) && `${item.title} ${item.description} ${item.category}`.toLowerCase().includes(state.templateSearch.toLowerCase()));
      document.querySelector('#template-count').textContent = `${filtered.length} templates`;
      grid.innerHTML = filtered.length ? filtered.map(templateCard).join('') : '<div class="empty-state">No templates match those filters.</div>';
      grid.querySelectorAll('[data-use-template]').forEach(button => button.addEventListener('click', () => openCreateModal(button.dataset.useTemplate)));
    }
    function templateCard(item) {
      const art = item.kind === 'decision'
        ? '<div class="template-block green" style="top:53px;left:10%;width:26%">Evidence<br>Linked sources</div><div class="template-block yellow" style="top:53px;left:39%;width:25%">Alternatives<br>What changes</div><div class="template-block coral" style="top:53px;right:7%;width:25%">Decision<br>Next action</div><div class="template-link" style="top:78px;left:36%;width:12%"></div><div class="template-link" style="top:78px;left:64%;width:14%"></div>'
        : item.kind === 'research'
        ? '<div class="template-block yellow" style="top:49px;left:9%;width:24%">Source<br>Interview 04</div><div class="template-block green" style="top:80px;left:38%;width:24%">Finding<br>Need for clarity</div><div class="template-block" style="top:49px;right:8%;width:24%">Question<br>What is causing this?</div><div class="template-link" style="top:72px;left:32%;width:17%"></div><div class="template-link" style="top:71px;left:62%;width:16%"></div>'
        : item.kind === 'kickoff'
        ? '<div class="template-block green" style="top:50px;left:10%;width:25%">Goal<br>What we want to change</div><div class="template-block" style="top:50px;left:39%;width:24%">Context<br>Sources and constraints</div><div class="template-block yellow" style="top:50px;right:8%;width:24%">First step<br>Owner and due date</div><div class="template-link" style="top:72px;left:34%;width:16%"></div><div class="template-link" style="top:72px;left:62%;width:16%"></div>'
        : '<div class="template-block green" style="top:53px;left:12%;width:22%">Input</div><div class="template-block yellow" style="top:53px;left:41%;width:22%">Question</div><div class="template-block coral" style="top:53px;right:9%;width:22%">Next step</div><div class="template-link" style="top:77px;left:34%;width:15%"></div><div class="template-link" style="top:77px;left:63%;width:15%"></div>';
      return `<article class="template-card"><div class="template-art ${item.style}"><div class="template-art-title">${escapeHtml(item.title)}</div>${art}</div><div class="template-meta"><h3>${escapeHtml(item.title)}</h3><p>${escapeHtml(item.description)}</p><div class="template-meta-foot"><span class="template-tag">${escapeHtml(item.category)}</span><button class="outline-button use-template" type="button" data-use-template="${escapeAttribute(item.title)}">Use template <span aria-hidden="true">→</span></button></div></div></article>`;
    }
    function updateActivityButton() {
      const button = topbarRight.querySelector('.activity-bell');
      if (!button) return;
      const unread = scopedActivityItems().filter(item => !item.read).length;
      const label = isBoardActivity() ? `${activeBoardName()} activity` : 'Recent activity';
      button.setAttribute('aria-label', `${label}, ${unread} unread`);
      button.title = label;
      const badge = button.querySelector('.activity-badge');
      if (unread && badge) badge.textContent = unread;
      else if (unread) button.insertAdjacentHTML('beforeend', `<span class="activity-badge">${unread}</span>`);
      else badge?.remove();
    }
    function renderActivityPanel() {
      const items = scopedActivityItems();
      const unread = items.filter(item => !item.read).length;
      const preview = items.slice(0, 3);
      activityRoot.innerHTML = `<section class="activity-popover" aria-label="Recent activity"><div class="activity-popover-head"><div><strong>Recent activity</strong><small>${unread ? `${unread} unread updates` : 'You’re all caught up'}</small></div><button class="icon-button" type="button" data-action="toggle-activity" aria-label="Close activity">×</button></div>${unread ? '<button class="text-button activity-mark-all" type="button" data-action="mark-all-activity-read">Mark all as read</button>' : ''}<div class="activity-list">${preview.map(item => `<button class="activity-entry ${item.read ? '' : 'unread'}" type="button" data-action="open-activity-item" data-activity-id="${escapeAttribute(item.id)}"><span class="activity-dot"></span><span><strong>${escapeHtml(item.title)}</strong><p>${escapeHtml(item.detail)} · ${escapeHtml(item.workspace)}</p></span><time>${escapeHtml(item.time)}</time></button>`).join('')}</div><button class="text-button activity-all" type="button" data-action="all-activity">View all activity <span aria-hidden="true">→</span></button></section>`;
      topbarRight.querySelector('.activity-bell')?.setAttribute('aria-expanded', 'true');
      if (isBoardActivity()) {
        activityRoot.querySelector('.activity-popover').setAttribute('aria-label', `${activeBoardName()} activity`);
        activityRoot.querySelector('.activity-popover-head strong').textContent = `${activeBoardName()} activity`;
        activityRoot.querySelector('.activity-all').textContent = 'View all board activity';
      }
    }
    function toggleActivityPanel() {
      if (activityRoot.firstElementChild) {
        activityRoot.innerHTML = '';
        topbarRight.querySelector('.activity-bell')?.setAttribute('aria-expanded', 'false');
      } else {
        renderActivityPanel();
      }
    }
    function renderActivity() {
      const root = document.querySelector('#view-content');
      const unread = scopedActivityItems().filter(item => !item.read).length;
      root.innerHTML = `<div class="page-head activity-page-head"><div><span class="eyebrow">Shared memory</span><h1>Activity</h1><p>A chronological record of contributions, observations, proposals, decisions, and operations, with their source and outcome.</p></div><div class="head-actions"><button class="outline-button" type="button" data-action="back-home">← Workspaces</button></div></div><div class="activity-page-tools"><div class="activity-filter-row" id="activity-filters" role="group" aria-label="Filter activity"></div><div class="activity-page-count"><span id="activity-count"></span>${unread ? '<button class="text-button" type="button" data-action="mark-all-activity-read">Mark all as read</button>' : ''}</div></div><label class="search-box activity-page-search">${iconSearch()}<input id="activity-search" type="search" placeholder="Search activity" value="${escapeAttribute(state.activitySearch)}" aria-label="Search activity"></label><div class="activity-timeline" id="activity-timeline"></div>`;
      if (isBoardActivity()) {
        const boardName = activeBoardName();
        root.querySelector('h1').textContent = `${boardName} activity`;
        root.querySelector('.activity-page-head p').textContent = 'Contributions, sources, proposals, decisions, and actions associated with this board only.';
        root.querySelector('[data-action="back-home"]').textContent = `Back to ${boardName}`;
      }
      root.querySelector('[data-action="back-home"]').addEventListener('click', () => navigateTo(isBoardActivity() ? 'board' : 'home'));
      root.querySelector('#activity-search').addEventListener('input', event => { state.activitySearch = event.target.value; drawActivityTimeline(); });
      drawActivityTimeline();
    }
    function drawActivityTimeline() {
      const filters = document.querySelector('#activity-filters');
      const timeline = document.querySelector('#activity-timeline');
      if (!filters || !timeline) return;
      filters.innerHTML = activityFilters.map(filter => `<button class="category-chip" type="button" data-activity-category="${filter.id}" aria-pressed="${filter.id === state.activityCategory}">${filter.label}</button>`).join('');
      filters.querySelectorAll('[data-activity-category]').forEach(button => button.addEventListener('click', () => {
        state.activityCategory = button.dataset.activityCategory;
        drawActivityTimeline();
      }));
      const term = state.activitySearch.toLowerCase();
      const items = scopedActivityItems().filter(item => (state.activityCategory === 'all' || item.category === state.activityCategory) && `${item.event} ${item.title} ${item.detail} ${item.workspace} ${item.actor}`.toLowerCase().includes(term));
      document.querySelector('#activity-count').textContent = `${items.length} ${items.length === 1 ? 'event' : 'events'}`;
      const glyphs = { collaboration: '↔', sources: '↗', agent: '✦', decisions: '◆', actions: '↻' };
      timeline.innerHTML = items.length ? items.map(item => `<article class="activity-record ${item.read ? '' : 'unread'}"><span class="activity-record-icon ${item.category}" aria-hidden="true">${glyphs[item.category]}</span><div class="activity-record-main"><div class="activity-record-top"><h3>${escapeHtml(item.title)}</h3><time>${escapeHtml(item.time)}</time></div><div class="activity-domain-event">${escapeHtml(item.event)}</div><p>${escapeHtml(item.detail)}</p><div class="activity-record-footer"><span class="activity-actor">${escapeHtml(item.actor)}</span><span>${escapeHtml(item.workspace)}</span>${item.outcome ? `<span class="activity-outcome ${item.outcomeKind || ''}">${escapeHtml(item.outcome)}</span>` : ''}<button class="activity-context" type="button" data-action="open-activity-item" data-activity-id="${escapeAttribute(item.id)}">View context →</button></div></div></article>`).join('') : '<div class="empty-state">No activity matches this search.</div>';
    }
    function handleAction(action, activityId) {
      if (action === 'toggle-navigation') {
        state.sidebarOpen = document.querySelector('.sidebar').hidden;
        updateNavigation();
        persistMockState();
      }
      if (action === 'demo-login') { state.name = 'Maya Chen'; navigateTo('home'); }
      if (action === 'browse-templates') navigateTo('templates');
      if (action === 'new-workspace') openCreateModal('');
      if (action === 'new-board') {
        const board = window.BoardDemo.list(showToast).find(item => item.id === activeBoardId());
        window.BoardDemo.create({ notify: showToast, icon: iconBoard, workspaceId: board?.workspaceId, workspaceName: board?.workspaceName });
      }
      if (action === 'toggle-activity') toggleActivityPanel();
      if (action === 'all-activity') navigateTo('activity');
      if (action === 'back-home') navigateTo(isBoardActivity() ? 'board' : 'home');
      if (action === 'mark-all-activity-read') {
        scopedActivityItems().forEach(item => { item.read = true; });
        persistMockState();
        updateActivityButton();
        if (activityRoot.firstElementChild) renderActivityPanel();
        if (route() === 'activity') drawActivityTimeline();
      }
      if (action === 'open-activity-item') {
        const item = scopedActivityItems().find(entry => entry.id === activityId);
        if (item) {
          item.read = true;
          persistMockState();
          updateActivityButton();
          if (activityRoot.firstElementChild) renderActivityPanel();
          if (route() === 'activity') drawActivityTimeline();
          showToast(`Activity in ${item.workspace}: ${item.title.toLowerCase()}.`);
        }
      }
      if (action === 'approve-task') approveTask();
      if (action === 'forgot') showToast('Mockup: recover access by email.');
    }
    function openCreateModal(templateName) {
      state.templateName = templateName;
      overlayRoot.innerHTML = `<div class="modal-backdrop" role="presentation"><section class="modal" role="dialog" aria-modal="true" aria-labelledby="modal-title"><div class="modal-head"><div><h2 id="modal-title">${templateName ? 'Create from template' : 'New workspace'}</h2><p>${templateName ? 'The template will be added to the workspace’s first board.' : 'Choose a name; you can invite others later.'}</p></div><button class="icon-button" type="button" data-close-modal aria-label="Cerrar">×</button></div><form id="create-form"><div class="field"><label for="workspace-name">Workspace name</label><input id="workspace-name" name="name" required maxlength="60" value="${escapeAttribute(templateName ? templateName : '')}" placeholder="For example, Product plan"></div><div class="field"><label for="workspace-visibility">Initial access</label><select id="workspace-visibility" name="visibility"><option value="private">Just me for now</option><option value="invite">Invite the team later</option></select></div>${templateName ? `<div class="notice-line"><b>Template</b><span>${escapeHtml(templateName)} will be used as the starting structure. Its content will be editable.</span></div>` : ''}<div class="modal-actions"><button class="quiet-button" type="button" data-close-modal>Cancel</button><button class="primary-button" type="submit">Create workspace</button></div></form></section></div>`;
      const modal = overlayRoot.querySelector('.modal-backdrop');
      overlayRoot.querySelectorAll('[data-close-modal]').forEach(button => button.addEventListener('click', closeModal));
      modal.addEventListener('click', event => { if (event.target === modal) closeModal(); });
      overlayRoot.querySelector('#create-form').addEventListener('submit', event => {
        event.preventDefault();
        const name = overlayRoot.querySelector('#workspace-name').value.trim();
        if (!name) return;
        if (!templateName) {
          const entry = { id: crypto.randomUUID(), name: 'Starter board', workspaceId: crypto.randomUUID(), workspaceName: name };
          try { localStorage.setItem('memento-demo-boards-v1', JSON.stringify([...window.BoardDemo.list(showToast), entry])); }
          catch (error) { showToast(`Workspace was not created: ${error.message}`); return; }
          persistMockState();
          location.href = `05-board.html?board=${entry.id}`;
          return;
        }
        const initialBoard = templateName || 'Starter board';
        boardSignals[initialBoard] = { favorite: false, changes: 0, lastOpened: 'Created just now' };
        workspaces.unshift({ id: `new-${Date.now()}`, name, description: templateName ? `Workspace created from “${templateName}”.` : 'A new space to organize your work.', type: 'owned', marker: '✦', activity: 'Created just now', people: [initials(state.name)], boards: [initialBoard] });
        state.workspaceFilter = 'all';
        state.search = '';
        closeModal();
        persistMockState();
        if (route() === 'home') renderApp('home');
        else navigateTo('home');
      });
      overlayRoot.querySelector('#workspace-name').focus();
    }
    function closeModal() { overlayRoot.innerHTML = ''; }
    function showToast(message) {
      toastRoot.innerHTML = `<div class="toast" role="status">${escapeHtml(message)}</div>`;
      window.clearTimeout(showToast.timeout);
      showToast.timeout = window.setTimeout(() => { toastRoot.innerHTML = ''; }, 3200);
    }
    function initials(value) { return value.split(/\s+/).filter(Boolean).slice(0, 2).map(part => part[0].toUpperCase()).join('') || 'M'; }
    function escapeHtml(value) { return String(value).replace(/[&<>"']/g, char => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' })[char]); }
    function escapeAttribute(value) { return escapeHtml(value); }

    window.addEventListener('keydown', event => {
      if (event.key === 'Escape' && overlayRoot.innerHTML) { closeModal(); return; }
      if (event.key === 'Escape' && activityRoot.firstElementChild) { toggleActivityPanel(); return; }
      if (event.key === 'Escape' && !overlayRoot.innerHTML && !activityRoot.firstElementChild && document.querySelector('.sidebar:not([hidden])')) {
        state.sidebarOpen = false;
        updateNavigation();
        persistMockState();
        document.querySelector('#navigation-toggle').focus();
      }
    });
    document.addEventListener('click', event => {
      const action = event.target.closest('[data-action]');
      if (action) handleAction(action.dataset.action, action.dataset.activityId);
    });
    render();
