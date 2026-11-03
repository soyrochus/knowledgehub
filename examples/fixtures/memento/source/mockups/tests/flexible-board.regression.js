async function runFlexibleBoardRegression(page) {
  const origin = await page.evaluate(() => location.origin);
  const backup = await page.evaluate(() => ({ board: localStorage.getItem('memento-atlas-board-v2'), theme: localStorage.getItem('memento-theme'), session: sessionStorage.getItem('memento-mockup-state-v1') }));
  const errors = [], requests = [], results = [];
  const onError = error => errors.push(error.message);
  const onRequest = request => { if (/^https?:/.test(request.url()) && !request.url().startsWith(origin + '/')) requests.push(request.url()); };
  page.on('pageerror', onError); page.on('request', onRequest);
  const check = (value, message) => { if (!value) throw new Error(message); };
  const click = async (action, id, scope = '.atlas-board') => {
    if (['run-question', 'run-complete', 'run-fail'].includes(action)) {
      const run = page.locator('.atlas-chat-run').filter({ has: page.locator(`[data-atlas-action="${action}"][data-id="${id}"]`) });
      for (const selector of ['.atlas-run-details', '.atlas-demo-controls']) {
        const details = run.locator(selector);
        if (await details.getAttribute('open') === null) await details.locator(':scope > summary').click();
      }
    }
    return page.locator(`${scope} [data-atlas-action="${action}"]${id ? `[data-id="${id}"]` : ''}`).first().click();
  };
  const model = () => page.evaluate(() => JSON.parse(localStorage.getItem('memento-atlas-board-v2')));
  const save = () => page.locator('.atlas-modal button[type="submit"]').click();
  const close = () => page.getByRole('button', { name: 'Close dialog', exact: true }).click();
  async function checkCardContrast() {
    check(await page.locator('.flow-board-title strong').evaluate(el => getComputedStyle(el).color === getComputedStyle(document.body).color), 'Board title must use the high-contrast theme text color');
    const failures = await page.locator('.flow-node').evaluateAll(cards => {
      const luminance = color => {
        const rgb = color.match(/[\d.]+/g).slice(0, 3).map(Number).map(value => {
          const channel = value / 255;
          return channel <= .04045 ? channel / 12.92 : ((channel + .055) / 1.055) ** 2.4;
        });
        return rgb[0] * .2126 + rgb[1] * .7152 + rgb[2] * .0722;
      };
      return cards.flatMap(card => {
        const background = luminance(getComputedStyle(card).backgroundColor);
        return [...card.querySelectorAll('.flow-node-header, .flow-node-content strong, .flow-node-content p, .atlas-note-text, .flow-node-meta')].filter(el => el.textContent.trim()).flatMap(el => {
          const foreground = luminance(getComputedStyle(el).color);
          const ratio = (Math.max(foreground, background) + .05) / (Math.min(foreground, background) + .05);
          return ratio < 4.5 ? [`${card.dataset.record}: ${ratio.toFixed(2)}:1`] : [];
        });
      });
    });
    check(!failures.length, 'Card text contrast below 4.5:1: ' + failures.join('; '));
  }
  async function checkAccessContrast() {
    const failures = await page.locator('.welcome-copy h1, .welcome-copy p, .welcome-copy .eyebrow, .welcome-foot, .mini-board-head, .mini-source, .mini-source strong, .mini-decision, .mini-decision b, .field label, .field input, .entry-tab, .guest-note, .guest-note strong, .join-preview strong, .join-preview span, .join-preview-mark, .notice-line b, .notice-line span, .login-legal').evaluateAll(elements => {
      const luminance = color => {
        const channels = color.match(/[\d.]+/g).slice(0, 3).map(Number).map(v => { const c = v / 255; return c <= .04045 ? c / 12.92 : ((c + .055) / 1.055) ** 2.4; });
        return channels[0] * .2126 + channels[1] * .7152 + channels[2] * .0722;
      };
      return elements.filter(el => el.getClientRects().length && getComputedStyle(el).visibility !== 'hidden').flatMap(el => {
        let surface = el;
        while (surface.parentElement && getComputedStyle(surface).backgroundColor === 'rgba(0, 0, 0, 0)') surface = surface.parentElement;
        const background = luminance(getComputedStyle(surface).backgroundColor);
        const colors = [getComputedStyle(el).color];
        if (el.matches('input')) colors.push(getComputedStyle(el, '::placeholder').color);
        return colors.flatMap(color => {
          const foreground = luminance(color);
          const ratio = (Math.max(foreground, background) + .05) / (Math.min(foreground, background) + .05);
          return ratio < 4.5 ? [`${el.className || el.tagName}: ${ratio.toFixed(2)}:1`] : [];
        });
      });
    });
    check(!failures.length, 'Access text contrast below 4.5:1: ' + failures.join('; '));
  }
  async function focus(id) {
    if (!await page.getByRole('searchbox').count()) await click('search-toggle');
    await page.getByRole('searchbox').fill('');
    const title = (await model()).records.find(r => r.id === id).title;
    await page.getByRole('searchbox').fill(title);
    await click('fit'); await page.locator(`[data-record="${id}"]`).click(); await click('panel', 'inspect');
  }
  async function checkHoverContrast(button) {
    await button.hover();
    const ratio = await button.evaluate(el => {
      const luminance = color => {
        const rgb = color.match(/[\d.]+/g).slice(0, 3).map(Number).map(v => { const c = v / 255; return c <= .04045 ? c / 12.92 : ((c + .055) / 1.055) ** 2.4; });
        return rgb[0] * .2126 + rgb[1] * .7152 + rgb[2] * .0722;
      };
      let surface = el;
      while (surface.parentElement && getComputedStyle(surface).backgroundColor === 'rgba(0, 0, 0, 0)') surface = surface.parentElement;
      const fg = luminance(getComputedStyle(el).color), bg = luminance(getComputedStyle(surface).backgroundColor);
      return (Math.max(fg, bg) + .05) / (Math.min(fg, bg) + .05);
    });
    check(ratio >= 4.5, `Hovered ${await button.innerText()} contrast is ${ratio.toFixed(2)}:1`);
  }
  try {
    await page.evaluate(() => localStorage.removeItem('memento-atlas-board-v2'));
    await page.goto(`${origin}/05-board.html?ux=${Date.now()}`);
    check(await page.locator('.atlas-view-switch,.atlas-commandbar,.flow-kicker').count() === 0, 'Old memory navigation, command bar or breadcrumb remains');
    check(await page.getByRole('searchbox').count() === 0, 'Search must be collapsed initially');
    check(await page.getByRole('button', { name: 'Create board', exact: true }).count() === 0, 'Redundant Create board remains');
    await click('search-toggle');
    const seamlessSearch = page.getByRole('searchbox', { name: 'Search board' });
    check(await seamlessSearch.evaluate(el => el.parentElement.classList.contains('flow-history') && !el.closest('.atlas-search')), 'Search is nested inside an extra field container');
    await seamlessSearch.fill('Speaker');
    await seamlessSearch.press('ArrowLeft');
    await seamlessSearch.press('x');
    check(await seamlessSearch.inputValue() === 'Speakexr', 'Search rerender lost the caret position');
    await seamlessSearch.press('Escape');
    check(!await seamlessSearch.isVisible() && await page.locator('[data-record]').count() === 14, 'Escape did not close search and restore cards');
    await click('search-toggle');
    await click('search-toggle');
    check(!await seamlessSearch.isVisible(), 'Search icon did not collapse the input');
    results.push('Seamless toolbar search keeps its caret, toggles inline and clears on Escape; no Create board control');
    await focus('goal');
    await page.locator('[data-note-text="goal"]').click({ button: 'right' });
    check(await page.getByRole('menu', { name: 'Board actions' }).isVisible(), 'Right-click did not open board actions');
    await page.getByRole('menuitem', { name: 'Submit Proposal from context' }).click();
    check(await page.getByRole('dialog').innerText().then(text => text.includes('Submit a Proposal')), 'Context menu lost Proposal command');
    await close();
    await click('context-menu', 'goal');
    check(await page.getByRole('menuitem', { name: 'Record Decision from context' }).isVisible(), 'Card action button did not open contextual commands');
    await page.keyboard.press('Escape');
    check(!await page.getByRole('menu').count(), 'Escape did not close the context menu');
    await page.locator('[data-record="goal"]').focus(); await page.keyboard.press('Shift+F10');
    check(await page.getByRole('menu').isVisible(), 'Keyboard context menu unavailable');
    await page.keyboard.press('Escape');
    await click('context-menu', 'goal');
    await page.getByRole('menuitem', { name: 'Add Note', exact: true }).click();
    await page.getByRole('textbox', { name: 'Note text' }).fill('Created from the context menu');
    await page.getByRole('textbox', { name: 'Note text' }).press('Control+Enter');
    check((await model()).records.some(r => r.kind === 'note' && r.text === 'Created from the context menu'), 'Context menu did not create an editable Note');
    results.push('Context menu through right-click, card actions and keyboard; contextual Proposal command preserves the Note');
    await click('tool', 'note');
    await page.getByRole('group', { name: 'New Note color', exact: true }).getByRole('button', { name: 'Pink', exact: true }).click();
    const blankPoint = await page.locator('.flow-viewport').evaluate(el => {
      const box = el.getBoundingClientRect();
      for (let y = box.top + 100; y < box.bottom - 80; y += 40) for (let x = box.left + 100; x < box.right - 50; x += 40) {
        const hit = document.elementFromPoint(x, y);
        if (hit && !hit.closest('[data-record],.flow-tool-rail,.flow-zoom-controls')) return { x, y };
      }
      throw new Error('No free canvas point found');
    });
    await page.mouse.click(blankPoint.x, blankPoint.y);
    const note = (await model()).records.find(r => r.title === 'Untitled Note');
    check(await page.getByRole('textbox', { name: 'Note text' }).evaluate(el => el === document.activeElement), 'A new Note did not start editing immediately');
    check(await page.getByRole('dialog').count() === 0, 'Note editing opened a dialog');
    await page.getByRole('textbox', { name: 'Note text' }).fill('Freeform thought\nSecond line of the same Note');
    await page.getByRole('textbox', { name: 'Note text' }).press('Control+Enter');
    const editedNote = (await model()).records.find(r => r.id === note.id);
    check(editedNote.text === 'Freeform thought\nSecond line of the same Note' && editedNote.title === 'Freeform thought', 'Inline Note did not save text and derive its reference label');
    note.revision = editedNote.revision;
    check(note?.color === 'pink' && note.kind === 'note', 'Creation did not use the chosen color');
    for (const [name, color] of [['Yellow', 'amber'], ['Green', 'mint'], ['Blue', 'blue'], ['Purple', 'lavender'], ['Pink', 'pink']]) {
      await page.getByRole('group', { name: 'Note color', exact: true }).getByRole('button', { name, exact: true }).click();
      check((await model()).records.find(r => r.id === note.id).color === color, 'Inspector color change failed');
      check((await model()).records.find(r => r.id === note.id).revision === note.revision, 'Color changed the content revision');
    }
    await click('undo');
    check((await model()).records.find(r => r.id === note.id).color === 'lavender', 'Visual undo did not restore the color');
    await click('redo');
    await page.reload();
    check((await model()).records.find(r => r.id === note.id).color === 'pink', 'Color lost on reload');
    await page.locator(`[data-note-text="${note.id}"]`).dblclick();
    await page.getByRole('textbox', { name: 'Note text' }).fill('Cancelled edit');
    await page.getByRole('textbox', { name: 'Note text' }).press('Escape');
    check((await model()).records.find(r => r.id === note.id).text === editedNote.text, 'Escape changed the saved Note');
    await page.locator(`[data-note-text="${note.id}"]`).dblclick();
    await page.getByRole('textbox', { name: 'Note text' }).fill('Saved on blur\nStill a single freeform Note');
    await page.getByRole('group', { name: 'Note color', exact: true }).getByRole('button', { name: 'Blue', exact: true }).click();
    check((await model()).records.find(r => r.id === note.id).text.startsWith('Saved on blur'), 'Clicking outside did not save the inline Note');
    check((await model()).records.find(r => r.id === note.id).color === 'blue', 'Saving on blur swallowed the color click');
    await page.getByRole('group', { name: 'Note color', exact: true }).getByRole('button', { name: 'Pink', exact: true }).click();
    await focus(note.id);
    for (const theme of ['light', 'dark']) {
      if (await page.evaluate(() => document.documentElement.dataset.theme) !== theme) await page.getByRole('button', { name: `Switch to ${theme} theme` }).click();
      for (const name of ['Yellow', 'Green', 'Blue', 'Purple', 'Pink']) {
        await page.getByRole('group', { name: 'Note color', exact: true }).getByRole('button', { name, exact: true }).click();
        await checkCardContrast();
      }
    }
    results.push('Note color chosen before placement, editable in Inspector, visual undo/redo and durable reload without content revision changes');
    results.push('New Notes enter seamless inline editing; multiline text saves without a title form; Escape cancels the draft');
    await focus('goal');
    const initial = await model(), original = initial.records.find(r => r.id === 'goal');
    await click('proposal', 'goal');
    await page.locator('input[name="title"]').fill('Consider a short briefing');
    await page.locator('textarea[name="body"]').fill('An optional Proposal, not a required state transition.');
    await save();
    let data = await model(), proposal = data.records.find(r => r.title === 'Consider a short briefing');
    check(!proposal.placed && proposal.evidence.some(ref => ref.id === 'goal'), 'Proposal must reference context and stay off-board by default');
    check(JSON.stringify(data.records.find(r => r.id === 'goal')) === JSON.stringify(original), 'Submitting a Proposal mutated the Note');
    await close();
    await focus('goal'); await click('decision', 'goal');
    await page.locator('input[name="title"]').fill('Keep the original launch date');
    await page.locator('textarea[name="body"]').fill('Direct human Decision from a Note, no Proposal required.');
    await save();
    data = await model();
    const decision = data.records.find(r => r.title === 'Keep the original launch date');
    check(!decision.placed && decision.evidence[0].id === 'goal', 'Decision should preserve pinned context without a new placement');
    check(data.records.find(r => r.id === 'goal').kind === 'note', 'Decision converted the Note');
    check(data.records.find(r => r.id === proposal.id).status === 'Pending review', 'Direct Decision incorrectly adopted an unrelated Proposal');
    await close();
    await click('trace');
    check(await page.locator('.atlas-modal .flow-world').count() === 0, 'Traceability renders another canvas');
    await page.locator('.atlas-modal [data-atlas-action="detail"][data-id="goal"]').click();
    await page.locator('.atlas-modal [data-atlas-action="focus"][data-id="goal"]').click();
    check(await page.locator('[data-record="goal"]').getAttribute('aria-pressed') === 'true', 'Trace reference did not focus its card');
    results.push('Independent Proposal and Decision commands preserve Notes and do not auto-place records; traceability locates cards');
    await click('ask-selection');
    check(!await page.locator('.atlas-agent-settings, [data-atlas-action="chat-prompt"], [data-atlas-action="chat-references"]').count(), 'Chat retained nonessential settings or suggested prompts');
    check(!await page.locator('#atlas-agent-form input[type="number"]').count(), 'Technical runtime configuration leaked into the composer');
    check(await page.getByRole('combobox', { name: 'Agent Role' }).isVisible(), 'Agent Role must remain directly available');
    await page.locator('#atlas-agent-form textarea').fill('What are the alternatives?');
    await page.locator('#atlas-agent-form button[type="submit"]').click();
    const run = (await model()).runs[0];
    check(run.limits.timeout === (await model()).agentSettings.timeout && run.limits.toolBudget === (await model()).agentSettings.toolBudget, 'Agent Run did not retain its configured bounds');
    check(await page.locator('.atlas-run-progress').count() > 0, 'Agent Run progress is missing');
    await click('run-question', run.id);
    check(await page.locator(`[data-atlas-action="run-complete"][data-id="${run.id}"]`).isDisabled(), 'Completion remains enabled while awaiting clarification');
    check((await model()).runs[0].status === 'Working' && (await model()).runs[0].awaitingReply, 'Unanswered clarification was silently completed');
    await page.locator('#atlas-agent-form textarea').fill('The launch date must remain fixed.');
    await page.locator('#atlas-agent-form button[type="submit"]').click();
    data = await model();
    check(data.runs[0].id === run.id && data.runs[0].replies[0].includes('fixed') && !data.runs[0].awaitingReply, 'Clarification did not continue the same Agent Run');
    const chatCount = data.messages.length;
    await click('run-complete', run.id);
    data = await model();
    check(!data.records.find(r => r.id === data.runs[0].output).placed, 'Agent output was automatically placed');
    await page.reload(); await click('panel', 'agents');
    check((await model()).messages.length > chatCount && await page.getByRole('log').innerText().then(text => text.includes('launch date must remain fixed')), 'Chat lost conversation on reload');
    await click('capture', 'goal:1');
    check(await page.getByRole('dialog').innerText().then(text => text.includes(original.body)), 'Chat reference did not show captured content');
    await close();
    await click('panel', 'activity');
    check(await page.locator('.atlas-panel-tabs button').count() === 4, 'History and Activity remain separate');
    await click('activity-filter', 'agent');
    check(await page.locator('.atlas-panel-content').innerText().then(text => text.includes('Agent run')), 'Unified activity filter lost Agent Runs');
    check(await page.getByRole('radiogroup', { name: 'Filter activity' }).getByRole('radio', { name: /Agent Runs/ }).getAttribute('aria-checked') === 'true', 'Activity filters must be a single-choice radio group');
    check(await page.locator('.atlas-timeline-item').count() > 0 && await page.locator('.atlas-timeline-item').evaluateAll(items => items.every(item => item.dataset.category === 'agent')), 'Activity timeline does not honour the filter');
    const filterLook = await page.locator('.atlas-filter-chip').first().evaluate(el => { const c = getComputedStyle(el); return { radius: parseFloat(c.borderTopLeftRadius), height: el.getBoundingClientRect().height }; });
    const traceLook = await page.getByRole('button', { name: 'View traceability' }).evaluate(el => { const c = getComputedStyle(el); return { radius: parseFloat(c.borderTopLeftRadius), weight: Number(c.fontWeight), icon: !!el.querySelector('svg') }; });
    check(filterLook.radius >= filterLook.height / 2 - 1 && traceLook.radius < 14 && traceLook.weight >= 600 && traceLook.icon, 'Activity filters and the traceability action are not visually distinct');
    results.push('Persistent chat, explicit pinned context, clarifications, references and unified Activity');
    await click('panel', 'agents');
    check(await page.getByRole('button', { name: 'Clear context' }).isVisible() && await page.locator('.atlas-chat-context [data-atlas-action="capture"]').count() > 0, 'Pinned Context Selection is not available');
    check(await page.getByRole('button', { name: 'Send' }).isVisible() && await page.getByRole('button', { name: 'Attach selection' }).count() === 1, 'Compact composer controls are missing');
    check(!await page.getByRole('button', { name: 'Complete demo run' }).isVisible(), 'Demo controls must be hidden from the normal chat');
    results.push('Minimal composer, two specified Agent Roles, explicit pinned context, bounded runs and secondary demo controls');
    const panelWidths = [];
    for (const tab of ['inspect', 'sources', 'agents', 'activity']) { await click('panel', tab); panelWidths.push(Math.round((await page.locator('#atlas-context-panel').boundingBox()).width)); }
    check(new Set(panelWidths).size === 1, `Side panel width changes between tabs: ${panelWidths.join(', ')}`);
    await click('panel', 'inspect');
    const toggle = page.getByRole('button', { name: 'Hide side panel' });
    await toggle.click();
    check(!await page.locator('#atlas-context-panel').isVisible() && await page.getByRole('button', { name: 'Show side panel' }).getAttribute('aria-expanded') === 'false', 'Side panel did not collapse');
    await page.reload();
    check(!await page.locator('#atlas-context-panel').isVisible(), 'Collapsed side panel was not restored after reload');
    await page.getByRole('button', { name: 'Open Agents chat' }).click();
    check(await page.locator('#atlas-context-panel').isVisible() && await page.locator('.atlas-panel-tabs [aria-pressed="true"]').innerText() === 'Agents', 'Chat shortcut did not open the Agents panel');
    check(await page.locator('#atlas-agent-form textarea').evaluate(el => el === document.activeElement), 'Chat shortcut did not focus the message input');
    await page.getByRole('button', { name: 'Hide side panel' }).click();
    await page.getByRole('button', { name: 'Show side panel' }).click();
    check(await page.locator('#atlas-context-panel').isVisible(), 'Side panel did not expand again');
    await focus('proposal'); await click('panel', 'inspect');
    const info = await page.locator('.atlas-info-card').first().evaluate(el => { const c = getComputedStyle(el); return { border: c.borderTopWidth, bg: c.backgroundColor, shadow: c.boxShadow }; });
    const actionStyle = await page.locator('.atlas-panel-content .quiet-button:not(:disabled)').first().evaluate(el => { const c = getComputedStyle(el); return { border: c.borderTopWidth, bg: c.backgroundColor, weight: Number(c.fontWeight) }; });
    check(info.border === '0px' && info.shadow === 'none' && actionStyle.border === '1px' && actionStyle.weight >= 600 && info.bg !== actionStyle.bg, 'Information cards and action buttons are not visually distinct');
    results.push('Collapsible fixed-width side panel, Agents shortcut in the tool rail, and distinct information cards vs action buttons');
    await click('panel', 'inspect');
    if (!await page.getByRole('searchbox').count()) await click('search-toggle');
    await page.getByRole('searchbox').fill('');
    await click('fit');
    await page.locator('[data-record="goal"] .source-handle').focus(); await page.keyboard.press('Enter');
    await page.locator('[data-record="constraint"]').focus(); await page.keyboard.press('Enter');
    check((await model()).visualLinks.some(edge => edge.from === 'goal' && edge.to === 'constraint'), 'Keyboard connection failed');
    await click('undo');
    check(!(await model()).visualLinks.length, 'Keyboard connection did not participate in visual undo');
    results.push('Keyboard-accessible connections and visual undo');
    await page.evaluate(() => { localStorage.setItem('memento-theme', 'dark'); });
    await page.reload();
    await page.getByRole('button', { name: 'Switch to light theme' }).click();
    check(await page.evaluate(() => document.documentElement.dataset.theme === 'light'), 'Light theme switch failed');
    await page.mouse.move(0, 0);
    check(await page.locator('.atlas-panel-tabs [aria-pressed="true"]').evaluate(el => getComputedStyle(el).backgroundColor) === 'rgb(113, 239, 178)', 'Light active tab must use the same product green as dark');
    check(await page.locator('.atlas-panel-tabs [aria-pressed="true"]').evaluate(el => getComputedStyle(el).color) === 'rgb(9, 41, 29)', 'Light active tab text must stay dark on product green');
    for (const tab of ['inspect', 'sources', 'agents', 'activity']) {
      await click('panel', tab);
      await checkHoverContrast(page.locator(`.atlas-panel-tabs [data-id="${tab}"]`));
      const buttons = page.locator('.atlas-panel-content :is(button.quiet-button,button.primary-button,button.outline-button):visible:not(:disabled)');
      for (let i = 0; i < Math.min(await buttons.count(), 3); i++) await checkHoverContrast(buttons.nth(i));
    }
    await click('panel', 'inspect');
    for (const theme of ['light', 'dark']) {
      if (await page.evaluate(() => document.documentElement.dataset.theme) !== theme) await page.getByRole('button', { name: `Switch to ${theme} theme` }).click();
      await page.waitForTimeout(250);
      const weak = await page.locator('.flow-inspector-activity').evaluate(root => {
        const lum = color => color.match(/[\d.]+/g).slice(0, 3).map(Number).map(v => { v /= 255; return v <= .04045 ? v / 12.92 : ((v + .055) / 1.055) ** 2.4; }).reduce((sum, v, i) => sum + v * [.2126, .7152, .0722][i], 0);
        let node = root, bg = 'rgba(0, 0, 0, 0)';
        while (node && /rgba\(0, 0, 0, 0\)|transparent/.test(bg)) { bg = getComputedStyle(node).backgroundColor; node = node.parentElement; }
        const b = lum(bg);
        return [...root.querySelectorAll('strong, small, .atlas-small, h3')].filter(el => el.textContent.trim()).flatMap(el => { const f = lum(getComputedStyle(el).color); const r = (Math.max(f, b) + .05) / (Math.min(f, b) + .05); return r < 4.5 || parseFloat(getComputedStyle(el).fontSize) < 11 ? [`${el.textContent.trim().slice(0, 20)} ${r.toFixed(2)}:1 ${getComputedStyle(el).fontSize}`] : []; });
      });
      check(!weak.length, `Inspector Recent activity is hard to read in ${theme}: ${weak.join('; ')}`);
      const greens = await page.evaluate(() => {
        const css = sel => getComputedStyle(document.querySelector(sel));
        const body = getComputedStyle(document.body).color;
        const toggle = css('.atlas-panel-toggle'), viewAll = css('.flow-inspector-activity .text-button'), shortcut = css('.atlas-chat-shortcut');
        return { toggleBg: toggle.backgroundColor, toggleBorder: toggle.borderTopColor, viewAll: viewAll.color === body, underline: viewAll.textDecorationColor, shortcut: shortcut.color === body };
      });
      check(greens.toggleBg === 'rgba(0, 0, 0, 0)' && greens.toggleBorder === 'rgb(113, 239, 178)' && greens.viewAll && greens.underline === 'rgb(113, 239, 178)' && greens.shortcut, `Secondary controls must use only the product green as accent in ${theme}: ${JSON.stringify(greens)}`);
      await click('panel', 'activity');
      const chip = await page.locator('.atlas-filter-chip[aria-checked="true"]').evaluate(el => { const c = getComputedStyle(el), b = getComputedStyle(el.querySelector('.atlas-filter-count')); return [c.backgroundColor, c.borderTopColor, b.backgroundColor].join('|'); });
      check(chip === 'rgba(0, 0, 0, 0)|rgb(113, 239, 178)|rgb(113, 239, 178)', `Active activity filter must be border and badge only in ${theme}: ${chip}`);
      await click('panel', 'inspect');
    }
    await page.getByRole('button', { name: 'Switch to light theme' }).click();
    results.push('Light-theme panel tabs and action button hover states retain at least 4.5:1 text contrast');
    results.push('Inspector Recent activity text keeps 4.5:1 contrast and readable sizes in both themes');
    for (const viewport of [{ width: 1440, height: 900 }, { width: 390, height: 844 }]) {
      await page.setViewportSize(viewport);
      for (const theme of ['light', 'dark']) {
        if (await page.evaluate(() => document.documentElement.dataset.theme) !== theme) await page.getByRole('button', { name: `Switch to ${theme} theme` }).click();
        for (const [dialogAction, dialogId] of [['trace', ''], ['decision', 'proposal'], ['review', 'reply-action'], ['connection', 'mail']]) {
          await page.evaluate(([a, id]) => { const b = document.createElement('button'); b.dataset.atlasAction = a; b.dataset.id = id; b.hidden = true; document.querySelector('.atlas-board').append(b); b.click(); b.remove(); }, [dialogAction, dialogId]);
          const m = await page.locator('.atlas-modal').evaluate(el => {
            const box = el.getBoundingClientRect(), body = el.querySelector('.atlas-modal-body'), head = el.querySelector('.modal-head');
            const cards = [...el.querySelectorAll('.atlas-service[data-kind]')];
            return { fits: box.top >= 0 && box.bottom <= innerHeight && box.left >= 0 && box.right <= innerWidth, maxWidth: box.width, overflowX: body.scrollWidth - body.clientWidth, scrolls: getComputedStyle(body).overflowY, headBg: getComputedStyle(head).backgroundColor, bodyBg: getComputedStyle(el).backgroundColor, h2: parseFloat(getComputedStyle(head.querySelector('h2')).fontSize), text: parseFloat(getComputedStyle(body.querySelector('p, label') || body).fontSize), kinds: cards.every(card => { const c = getComputedStyle(card), dot = getComputedStyle(card.querySelector('.eyebrow'), '::before'); return c.borderLeftWidth === c.borderTopWidth && c.borderLeftColor === c.borderTopColor && dot.content === '""' && dot.backgroundColor !== 'rgba(0, 0, 0, 0)'; }) && new Set(cards.map(card => getComputedStyle(card.querySelector('.eyebrow'), '::before').backgroundColor)).size === new Set(cards.map(card => card.dataset.kind)).size, headAccent: getComputedStyle(head).boxShadow, gaps: [...el.querySelectorAll('.atlas-buttons')].every(row => getComputedStyle(row).columnGap !== 'normal' && parseFloat(getComputedStyle(row).columnGap) >= 8) };
          });
          check(m.fits && m.maxWidth <= 640 && m.overflowX <= 0 && m.scrolls === 'auto', `${dialogAction} modal exceeds the viewport or overflows (${theme} ${viewport.width}px): ${JSON.stringify(m)}`);
          check(m.headBg !== m.bodyBg && m.headAccent === 'none' && m.h2 >= m.text + 4 && m.kinds && m.gaps, `${dialogAction} modal lacks hierarchy (${theme}): ${JSON.stringify(m)}`);
          const offGreens = async () => page.evaluate(() => {
            const out = [];
            for (const el of document.querySelectorAll('.atlas-modal, .atlas-modal *')) for (const pseudo of [null, '::before']) {
              const c = getComputedStyle(el, pseudo);
              if (pseudo && c.content === 'none') continue;
              for (const p of ['color', 'backgroundColor', 'borderTopColor', 'boxShadow', 'outlineColor']) {
                if (p === 'borderTopColor' && c.borderTopWidth === '0px') continue;
                if (p === 'outlineColor' && c.outlineStyle === 'none') continue;
                if (p === 'color' && el.matches('.primary-button, .primary-button *')) continue;
                for (const col of (c[p].match(/rgba?\(\d+, \d+, \d+(, [\d.]+)?/g) || [])) { const [r, g, b, a] = col.match(/[\d.]+/g).map(Number); if (a !== 0 && g > r + 25 && g > b + 10 && !(r === 113 && g === 239 && b === 178)) out.push(`${el.className}${pseudo || ''} ${p} ${col}`); }
              }
            }
            return out;
          });
          let greens = await offGreens();
          for (const sel of ['.atlas-modal-body button', '.atlas-modal .modal-actions button']) {
            const target = page.locator(sel).first();
            if (await target.count() && await target.isVisible()) { await target.hover(); await page.waitForTimeout(220); greens = greens.concat(await offGreens()); }
          }
          check(!greens.length, `${dialogAction} modal uses a green other than the product green (${theme}): ${[...new Set(greens)].slice(0, 5).join(' | ')}`);
          await close();
        }
      }
    }
    await page.setViewportSize({ width: 1440, height: 900 });
    await page.getByRole('button', { name: 'Switch to light theme' }).click();
    results.push('Modals stay within the viewport with a fixed header, scrolling body, symmetric borders, a coloured type dot per record, spaced actions and only the product green in both themes and on mobile');
    const panTool = page.locator('.flow-tool-rail [data-id="hand"]');
    await panTool.hover();
    check(await panTool.evaluate(el => getComputedStyle(el).color) === 'rgb(16, 36, 58)', 'Hovered toolbar icon retained low-contrast dark-theme text');
    await checkCardContrast();
    for (const theme of ['light', 'dark']) {
      await page.goto(`${origin}/04-templates.html?nav=${Date.now()}`);
      if (await page.evaluate(() => document.documentElement.dataset.theme) !== theme) await page.getByRole('button', { name: `Switch to ${theme} theme` }).click();
      const nav = await page.evaluate(() => {
        const css = el => getComputedStyle(el);
        const side = document.querySelector('.side-link[aria-current="page"]'), chip = document.querySelector('.category-chip[aria-pressed="true"]'), art = document.querySelector('.template-art'), block = document.querySelector('.template-block.green');
        return { sideBg: css(side).backgroundColor, sideShadow: css(side).boxShadow, chipBg: css(chip).backgroundColor, chipBorder: css(chip).borderTopColor, artBg: css(art).backgroundColor, surface: css(document.querySelector('.template-card')).backgroundColor, blockBg: css(block).backgroundColor };
      });
      check(nav.sideShadow === 'none' && !/225, 241, 231|113, 239, 178/.test(nav.sideBg) && nav.sideBg !== 'rgba(0, 0, 0, 0)' && nav.chipBg === 'rgba(0, 0, 0, 0)' && nav.chipBorder === 'rgb(113, 239, 178)', `Navigation and category chips should use the product green only as an accent (${theme}): ${JSON.stringify(nav)}`);
      check(theme === 'light' ? nav.artBg === 'rgb(255, 255, 255)' && nav.blockBg === 'rgb(142, 235, 188)' : nav.artBg !== 'rgb(33, 44, 56)' && nav.blockBg === 'rgb(24, 86, 61)', `Template previews should follow the theme (${theme}): ${JSON.stringify(nav)}`);
    }
    await page.getByRole('button', { name: 'Switch to light theme' }).click();
    results.push('Sidebar items use a neutral active fill without an accent bar, template chips use a product-green border; template previews are white in light and follow board colours in each theme');
    for (const theme of ['light', 'dark']) {
      await page.goto(`${origin}/02-home.html?cta=${Date.now()}`);
      if (await page.evaluate(() => document.documentElement.dataset.theme) !== theme) await page.getByRole('button', { name: `Switch to ${theme} theme` }).click();
      await page.locator('.workspace-card').first().hover();
      await page.waitForTimeout(400);
      const home = await page.evaluate(() => {
        const css = el => getComputedStyle(el), green = v => { const m = (v.match(/rgba?\((\d+), (\d+), (\d+)/) || []).slice(1).map(Number); return m.length && m[1] > m[0] + 30 && m[1] > m[2] + 15; };
        const strip = document.querySelector('.starter-strip'), cta = strip.querySelector('button'), card = document.querySelector('.workspace-card:hover'), chip = document.querySelector('.access-chip:not(.guest)');
        return { browse: document.querySelectorAll('[data-action="browse-templates"]').length, stripGreen: green(css(strip).backgroundColor) || green(css(strip).borderTopColor), ctaBorder: css(cta).borderTopColor, ctaGreenText: green(css(cta).color), ctaBg: css(cta).backgroundColor, cardGreen: green(css(card).borderTopColor), chipGreen: green(css(chip).color) || green(css(chip).borderTopColor) || green(css(chip).backgroundColor), primaryBorder: css(document.querySelector('.primary-button')).borderTopColor };
      });
      check(home.browse === 1 && !home.stripGreen && home.ctaBorder === 'rgb(113, 239, 178)' && !home.ctaGreenText && home.ctaBg === 'rgba(0, 0, 0, 0)' && !home.cardGreen && !home.chipGreen && home.primaryBorder === 'rgb(113, 239, 178)', `Home should use one product green accent per secondary CTA and neutral cards/chips (${theme}): ${JSON.stringify(home)}`);
    }
    await page.getByRole('button', { name: 'Switch to light theme' }).click();
    results.push('Home has a single Browse templates CTA with only a product-green border; workspace hover and Owner chips stay neutral');
    for (const theme of ['light', 'dark']) {
      await page.goto(`${origin}/05-board.html?drawer=${Date.now()}`);
      if (await page.evaluate(() => document.documentElement.dataset.theme) !== theme) await page.getByRole('button', { name: `Switch to ${theme} theme` }).click();
      if (await page.locator('.board-app > .sidebar').isHidden()) await page.getByRole('button', { name: 'Open navigation' }).click();
      const drawer = await page.evaluate(() => {
        const s = document.querySelector('.board-app > .sidebar'), c = getComputedStyle(s), r = s.getBoundingClientRect(), f = document.querySelector('.flow-status').getBoundingClientRect(), t = document.querySelector('.topbar').getBoundingClientRect();
        const offsets = [...c.boxShadow.matchAll(/\)\s+(-?[\d.]+)px\s+(-?[\d.]+)px/g)].map(m => [Number(m[1]), Number(m[2])]);
        return { top: r.top, topbar: t.bottom, bottom: r.bottom, footer: f.top, offsets, clip: c.clipPath };
      });
      check(Math.abs(drawer.top - drawer.topbar) <= 1 && drawer.bottom <= drawer.footer + 0.5 && drawer.offsets.every(([x, y]) => x > 0 && y === 0) && drawer.clip.startsWith('inset(0px'), `Board navigation drawer should sit between top bar and footer with a right-only shadow (${theme}): ${JSON.stringify(drawer)}`);
      await page.getByRole('button', { name: 'Close navigation' }).click();
    }
    await page.getByRole('button', { name: 'Switch to light theme' }).click();
    results.push('Board navigation drawer leaves the status footer visible and casts its shadow only to the right');
    for (const file of ['02-home.html', '03-activity.html', '04-templates.html', '01-access.html', '05-board.html']) {
      await page.goto(`${origin}/${file}?ux=${Date.now()}`);
      check(await page.evaluate(() => document.documentElement.dataset.theme === 'light'), `Theme lost on ${file}`);
      const colors = await page.evaluate(() => ({ body: getComputedStyle(document.body).backgroundColor, text: getComputedStyle(document.body).color }));
      check(colors.body === 'rgb(236, 239, 244)' && colors.text === 'rgb(16, 36, 58)', `Light surface colors not applied on ${file}`);
      if (file === '02-home.html') {
        const preview = page.locator('[data-open-workspace="atlas"] .board-preview-svg');
        const data = await model();
        check(Number(await preview.getAttribute('data-preview-records')) === data.records.filter(r => r.placed).length, 'Home preview does not reflect placed records');
        check(!await preview.locator(`[data-preview-record="${decision.id}"]`).count(), 'Home thumbnail restored an off-board Decision');
        check(await preview.locator(`[data-preview-record="${note.id}"]`).getAttribute('x') === String(note.x), 'Thumbnail changed Note position');
        const pink = preview.locator(`[data-preview-record="${note.id}"] rect`);
        check(await pink.evaluate(el => getComputedStyle(el).fill) === 'rgb(255, 185, 211)', 'Home thumbnail did not use the light Note color');
        await page.getByRole('button', { name: 'Switch to dark theme' }).click();
        check(await pink.evaluate(el => getComputedStyle(el).fill) === 'rgb(113, 49, 78)', 'Home thumbnail did not update to dark theme');
        await page.getByRole('button', { name: 'Switch to light theme' }).click();
        await page.locator('.activity-bell').click();
        check(await page.locator('.activity-badge').evaluate(el => getComputedStyle(el).color) === 'rgb(255, 255, 255)', 'Activity badge has unreadable light theme text');
        check(await page.locator('.activity-entry.unread').first().evaluate(el => getComputedStyle(el).backgroundColor) === 'rgb(225, 241, 231)', 'Activity popover retained a dark surface');
        await page.locator('.activity-bell').click();
        await page.setViewportSize({ width: 390, height: 844 });
        check(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth), 'Home thumbnails overflow on mobile');
        await page.setViewportSize({ width: 1440, height: 1000 });
        results.push('Home previews mirror saved placement, position and Note color, switch theme live, and exclude off-board records; activity uses light surfaces');
      }
      if (file === '01-access.html') {
        for (const theme of ['light', 'dark']) {
          if (await page.evaluate(() => document.documentElement.dataset.theme) !== theme) await page.locator('#theme-toggle').click();
          for (const mode of ['account', 'invite']) {
            await page.locator(`[data-entry="${mode}"]`).click();
            await checkAccessContrast();
            await checkHoverContrast(page.locator(`[data-entry="${mode}"]`));
            await checkHoverContrast(page.locator(`#${mode}-form .primary-button`));
            const input = page.locator(`#${mode}-form input`).first();
            await input.focus();
            check(await input.evaluate(el => { const style = getComputedStyle(el); return style.outlineStyle === 'solid' && parseFloat(style.outlineWidth) >= 2 && style.outlineColor === style.borderColor; }), 'Access input lost its brand-colored focus indicator');
            for (const width of [320, 390, 768]) {
              await page.setViewportSize({ width, height: 844 });
              check(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth), `Access overflows at ${width}px in ${theme}/${mode}`);
              check(await page.locator(`#${mode}-form .primary-button`).isVisible(), 'Access submit button disappeared on narrow screens');
            }
            await page.setViewportSize({ width: 1440, height: 1000 });
          }
        }
        await page.getByRole('button', { name: 'Switch to light theme' }).click();
        results.push('Access account and invitation views: text and placeholder contrast, selected/hover/focus states, preview and responsive layout in both themes');
      }
    }
    await page.setViewportSize({ width: 390, height: 844 });
    check(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth), 'Mobile overflow');
    await page.getByRole('button', { name: 'Switch to dark theme' }).click();
    check(await page.evaluate(() => document.documentElement.dataset.theme === 'dark'), 'Dark theme switch failed');
    check(await page.locator('.atlas-panel-tabs [aria-pressed="true"]').evaluate(el => getComputedStyle(el).backgroundColor) === 'rgb(113, 239, 178)', 'Dark active tab lost product green');
    await checkCardContrast();
    await page.reload();
    check(await page.evaluate(() => document.documentElement.dataset.theme === 'dark'), 'Dark preference lost');
    results.push('Light/dark choice across all mockup routes, persistent preference and narrow viewport');
    results.push('Rendered card titles, body text, headers and metadata meet 4.5:1 contrast in both themes');
    check(!errors.length, errors.join('; ')); check(!requests.length, 'External requests: ' + requests.join('; '));
    return { passed: true, results, browserErrors: errors, externalRequests: requests };
  } finally {
    page.removeListener('pageerror', onError); page.removeListener('request', onRequest);
    await page.evaluate(backup => {
      for (const [key, value] of [['memento-atlas-board-v2', backup.board], ['memento-theme', backup.theme]]) {
        if (value === null) localStorage.removeItem(key); else localStorage.setItem(key, value);
      }
      if (backup.session === null) sessionStorage.removeItem('memento-mockup-state-v1'); else sessionStorage.setItem('memento-mockup-state-v1', backup.session);
    }, backup);
    await page.setViewportSize({ width: 1440, height: 1000 });
    await page.goto(`${origin}/05-board.html?ux=complete`);
  }
}
if (typeof module !== 'undefined') module.exports = { runFlexibleBoardRegression };
