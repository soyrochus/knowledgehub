# Memento — Comparison Mockup versus Spec

Version 1.1 - 2026-10-28 - Historical comparison with replacement-baseline resolution ledger. Human review pending.

**Alignment record:** sections 1-10 preserve the original pre-rewrite comparison, including its references to then-current Rust, PostgreSQL-only and synchronization assumptions. The user subsequently authorized the five-document rewrite and selected the spike frontend, Bun backend, SQLite development/tests, PostgreSQL deployment and Podman development containers. The five specification owners now define target behaviour. Section 11 resolves each CM item and records that supersession; original findings are not continuing platform requirements or unfinished rewrite instructions.

The linked HTML mockups define a more complete Memento product than the current specifications describe. The specifications remain useful for domain meaning, architecture and delivery, but their visual reference is obsolete and several scope and interaction assumptions need revision. The largest behavioural correction is that an imported external reference is **static captured evidence**: Memento does not continuously synchronize it with the external system. A participant may explicitly refresh it.

This document records the differences and their consequences for the [Functional and Domain Specification](../memento-spec.md), [Technical Specification](../memento-technical-spec.md) and [Platform Specification](../memento-platform-spec.md). It is the requested comparison stage, not a rewrite of those three documents or an application implementation. The [release-plan record](../../delivery/releases/v1.0/plan.md#33-mockup-versus-specification-comparison) tracks this work and the pending alignment.

## 1. Authority and interpretation

For this comparison, apply the following order:

1. The user's explicit clarification: imported references are static evidence; there is no continuous board-to-provider synchronization; refresh may be requested manually.
2. The product experience expressed by the linked mockups, including their screens, panels, dialogs and reachable workflows.
3. Existing specifications for details the mockups do not redefine, including domain ownership, durable history, authorization, collaboration and reviewed external actions.
4. Earlier images and isolated technical spikes as historical design or implementation evidence only.

Consequently, an automatic-arrival checkbox or a label such as “Email sync” in a mockup does not override the static-evidence rule. Conversely, an old paragraph that defers templates or freehand drawing does not override their presence in the canonical product design. Those discrepancies must be reconciled explicitly.

Three kinds of statement are distinguished below:

| Kind | Meaning |
| --- | --- |
| Observed design | Visible in the current HTML/CSS/JavaScript or its rendered interface. It describes intended product experience, not verified backend behaviour. |
| Explicit product rule | The user's clarification about static evidence and manual refresh. It overrides conflicting mockup examples and specification text. |
| Derived contract / open decision | An engineering consequence needed to deliver the design, or a policy that the mockup does not settle. Proposed mechanisms are identified as such. |

The old [concept image](../images/memento-app-mockup.png) and its [generation prompt](../images/memento-app-mockup.prompt.txt) are obsolete as the current UI reference. Their existence does not make the functional and domain model obsolete. The later specification edit should replace their design-authority wording and embedded illustration with references or exports from the canonical HTML screens. Preserve historical assets where useful rather than treating them as current requirements.

## 2. Sources and review coverage

The comparison uses the working-tree baseline at commit `aa3f4ed1cdb3a1f4c7cd333912b84d28949b8f67`, inspected on 2026-10-28. The working tree was clean before this documentation task.

| Source | What it establishes |
| --- | --- |
| [Mockup entry point](../../mockups/index.html) | The entry URL redirects to Access. Its fallback index lists screens 01–04; the board is reached through the Home experience. The authenticated home is screen 02. |
| [01 — Access](../../mockups/01-access.html) | Account sign-in, invitation entry, display name, stay-signed-in choice, recovery entry, demo entry, and Terms/Privacy text. |
| [02 — Home and workspaces](../../mockups/02-home.html) | Application shell, owned/shared workspaces, workspace creation, recent/favorite boards, previews, search and changes since the previous visit. |
| [03 — Activity](../../mockups/03-activity.html) | Activity timeline, categories, search, unread state, mark-as-read and context entry points; supports a board-scoped variant. |
| [04 — Templates](../../mockups/04-templates.html) | Six template designs, category/search filters, previews and creation from a template. |
| [05 — Board](../../mockups/05-board.html) | The populated Atlas board, tool rail, history/view controls, presence, members, contextual panels and dialogs. |
| [Application behaviour](../../mockups/mockups.js) | `renderLogin`, `renderHome`, `drawWorkspaces`, `renderActivity`, `renderTemplates`, `openCreateModal`, notification scoping and navigation. |
| [Active board behaviour](../../mockups/atlas-board.js) | `AtlasBoard.mount`, Inspector/Sources/Agents/Activity panels, evidence and traceability dialogs, editing, decisions, source import/refresh, action review and simulated outcomes. |
| [From-zero board experience](../../mockups/board-demo.js) | Empty-board creation, guided playback, staged collaboration and personal-email capture sharing. Several controls are explicitly simulation or prototype controls. |
| [Shared styles](../../mockups/mockups.css), [themes](../../mockups/themes.css), [board styles](../../mockups/atlas-board.css), [demo styles](../../mockups/board-demo.css) | Dark/light presentation, colours, typography, layout, dialogs, responsive rules and interaction styling. |
| [Functional v0.5](../memento-spec.md), [Technical v0.5](../memento-technical-spec.md), [Platform v0.3](../memento-platform-spec.md) | Current documented behaviour and implementation contracts, dated 2026-10-13. |
| [Delivery guide](../../Design%20and%20Implementation%20Guide.md), [v1.0 plan](../../delivery/releases/v1.0/plan.md), [test environment](../../test-environment.md), [UI spike specification](spec-ui-spike.md) | Delivery boundaries, existing evidence, selected services and the limited authority of technical experiments. |

The active screen 05 uses `AtlasBoard.mount`; the older `renderBoard()` function still present in `mockups.js` is not the active board renderer. Its older pipeline composition must not be mistaken for the current design. Likewise, retained code for a memory mode does not establish a visible second canvas: the active interface opens an informational Traceability dialog.

## 3. Product structure expressed by the mockups

The design is an application around a collaborative board, with these connected journeys:

| Journey | Visible path and intended result |
| --- | --- |
| Account entry | Entry → My account → email/password → Home → an authorized workspace and board. |
| Invitation entry | Entry → I have an invitation → workspace/inviter/Editor preview → display name and code/link → access limited to the invited workspace. |
| Start work | Home → New workspace → name and initial access → first empty board; alternatively Templates → Use template → named workspace and editable first board. |
| Resume work | Home → owned/shared filter or search → workspace/board preview → recent/favorite board → inspect changes and continue. |
| Think and organize | Board → notes, text, shapes, frames, ink, visual links, selection and arrangement → inspect or edit material. |
| Bring evidence | Sources → configured, scoped service → browse/select record → import capture → inspect, annotate, open original, inspect history or manually refresh. |
| Ask AI | Select material → attach it to Agents chat → choose Facilitator or Action Coordinator → send request, answer clarification, inspect result or cancel. |
| Decide and act | Inspector/context menu → optional Proposal or direct Decision → preserve rationale/evidence → prepare an Action → review exact operation → approve → execute → inspect outcome. |
| Recover the reason | Inspector or Activity → Traceability → pinned evidence/details/history → locate an existing placement or deliberately show a retained record on the board. |

These paths are product requirements to specify and implement. Navigation currently ending in a toast, browser-local storage, a preview or a scripted outcome is incomplete mockup behaviour, not the target implementation.

## 4. Detailed difference catalogue

`F`, `T` and `P` below refer to sections in the functional/domain, technical and platform specifications respectively. An “extension” is additional detail absent from the existing definition; a “replacement” resolves a conflict; a “clarification” preserves an existing rule while making the UX precise.

### 4.1 Access, people and application entry

| ID / difference | Canonical design and existing gap | Functional/domain effect | Technical effect | Platform effect |
| --- | --- | --- | --- | --- |
| CM-01 — Design authority; replacement | Five linked application screens replace one illustrative concept image. F1.10 and the technical introduction still point to that image; F1.10 gives textual controls precedence over it. | Revise F1.10 and scope exclusions in F3. Link the whole screen set and this explicit static-evidence exception. Keep domain rules that the design preserves. | Replace the old visual-reference statement; trace visible workflows to contracts and checks rather than assuming the image covers them. | Add screen/state references to P5. Retain stack selection separately from UI-design authority. |
| CM-02 — Real account access; extension | Email/password, “Keep me signed in”, recovery and demo entry are explicit. F1.1/F3 say only basic sign-in; the mock form simply derives a display name and navigates. | Define sign-in, invalid credentials, session expiry, return visits and recovery outcomes. A real member identity is required. Account provisioning/registration is still unspecified; a sign-up screen is not shown. | Extend T1/T5/T6 with credential verification, session duration/revocation, recovery-token handling and unauthorized-route checks. Demo access must have a defined data boundary. | Extend P5.1/P8 with access routes, server-managed sessions and recovery integration. Browser storage and a chosen name cannot authenticate a person. Authentication implementation/provider remains a decision. |
| CM-03 — Invitation guest; extension | An invitation grants Editor access without creating a global account; it collects a display name and code/link. Existing Membership describes a named human but has no guest identity or invitation lifecycle. | Extend F1.1/F4.2–4.3/F4.7–4.9 with Invitation and workspace-scoped guest identity. Guest is an identity/access mode; Editor remains the contribution role. Define the guest's durable attribution and return path. | Add token validation, workspace/role binding, expiry/revocation handling and isolated guest sessions. Direct URLs to other workspaces remain denied. A display name is not an authorization credential. | Add invitation routing and persistence in `workspace_access`; distinguish account identity, guest identity, Membership and presence. Reuse authorization services rather than creating an unchecked guest UI path. |
| CM-04 — Members and permissions; clarification | Members dialog lists Owner and Editors, pending invitations and permitted operations. Its “Preview member role” selector is a demonstration device. Existing owner/editor rules are sound but do not define the management journey. | Specify inviting and viewing members; preserve Owner management of membership/connections and Editor contribution/Decision/Action approval rights. Specify removal/revocation and ownership policy before implementation. | Enforce membership and invitation changes on the server; revoke subscriptions and future commands as needed. Neither a local role selector nor agent output can grant rights. | Build member/invitation components over `workspace_access` use cases. Presence avatars remain ephemeral indicators, distinct from durable membership and attribution. |
| CM-05 — Application shell; extension | Global brand, collapsible navigation, Home, Activity, My workspaces, Shared with me, Templates, identity/avatar and theme control surround the board. Current routes in P5.1 are only an outline. | Describe navigation and access boundaries outside a board, including returning Home and retaining the active board when viewing scoped activity. “Personal workspace” in the profile is not enough to define a new tenant hierarchy. | Authorize shell queries, route loaders and deep links. Define unavailable/forbidden board states instead of a toast or silent fallback. | Expand React Router/feature structure in P5.1. Separate the shared shell, page content and board controller; route names need not retain `.html` filenames. |

### 4.2 Workspaces, boards and starting structures

| ID / difference | Canonical design and existing gap | Functional/domain effect | Technical effect | Platform effect |
| --- | --- | --- | --- | --- |
| CM-06 — Workspace catalogue; extension | Home shows several owned/shared workspaces, descriptions, people, previews, counts, search and filters. F3 is framed around a small workspace and demonstration board. | Extend F1.1/F2/F3 to create, list and enter multiple authorized workspaces; ownership and shared membership determine the filters. Search must define which names/descriptions it matches. | Add authorized list/search/create contracts, stable IDs, workspace metadata and an atomic first-board creation outcome. Counters derive from accessible records. | Add Home/workspace feature modules and `workspace_access` queries. Avoid hardcoded fixture counts or storing the workspace catalogue only in a browser. |
| CM-07 — Multiple boards and personal return state; extension | A workspace has named boards; Home shows Recent/Favorites, stars, last-opened information and “updates since your last visit”. Several fixture entries currently only toast. | Describe entering named boards, board/workspace ownership and empty/unavailable states. Define favorites and last-visit markers as personal navigation state, not shared knowledge. No cross-workspace semantic linking follows from this catalogue. Additional-board creation is not a completed visible journey. | Add board catalogue queries, per-member favorites/visit markers and authorized preview generation. Use stable board IDs rather than title-keyed maps. Define what an unread update count measures. | Extend P5.1's personal-state category: favorites/visits may require durable server storage; viewport/selection can remain personal. Derive previews from current board projections, with loading/empty/unavailable states. |
| CM-08 — Workspace/board creation; extension | “New workspace” collects a name and “Just me for now / Invite the team later”, then opens an empty first board. `BoardDemo.create` retains a named-board dialog, but there is no active Create board control; regression expectations explicitly preserve that absence. | Specify the workspace → first Board relationship, creator as Owner, initial private membership and an empty usable canvas. Both initial-access choices currently add no other member; neither means public access. Resolve additional-board creation separately rather than inferring a new toolbar button from retained code. | Validate input, create workspace/membership/board consistently, and handle duplicate submission or partial failure. A successful modal must lead to a persisted usable board. | Map creation across Access and Visual Collaboration through application orchestration. Preserve the selected stack; replace the mock's local registry with authorized persistence. |
| CM-09 — Template catalogue; replacement of deferral | Decision review, Research synthesis, Project kickoff, Dependency map, Quick retrospective and Open questions have previews, search/categories and Use template. F3 explicitly defers facilitation templates. The mock records a template name but does not fully instantiate its previewed content. | Bring the visible template workflow into the product baseline. Define templates as editable starting structures for a new workspace's first board, including which frames/notes/links are created. A template called “Decision review” must not pre-record a real Decision or Approval. | Add versioned template definitions and an atomic instantiation command that generates fresh scoped IDs and valid references. Record template origin/version; later template edits must not rewrite created work. | Add catalogue/creation UI and an explicit template storage/seed mechanism. Instantiate through owning use cases; a thumbnail or copied serialized board is not the domain implementation. |

### 4.3 Visual composition, tools and semantics

| ID / difference | Canonical design and existing gap | Functional/domain effect | Technical effect | Platform effect |
| --- | --- | --- | --- | --- |
| CM-10 — Current visual system; extension | Dark and light themes, green primary actions, coloured card families, dotted canvas, compact tool rails, distinct selected states, badges, avatars, dialogs and toast/status feedback are visible. Existing specifications give broad styling guidance. | Add the common shell/board/panel/dialog states to F1.10. Identify record type and lifecycle with text as well as colour; Note colour alone has no business meaning. Do not hardcode sample people, dates or record counts. | Define keyboard focus, named controls, theme persistence, readable inspection independent of zoom and responsive acceptance examples. CSS breakpoints alone do not establish supported-device acceptance. | Expand P5.3 into reusable tokens and components: shell, card, badge, palette, toolbar, inspector, chat, lists, modal and feedback. Current CSS uses IBM Plex Sans with system fallbacks; font packaging is still an implementation choice. |
| CM-11 — Whiteboard tools; scope replacement/extension | Select, Pan, Pen, Erase, Note, Frame, Text and Shape are visible; freehand, text and shapes can be created. F3 defers freehand, and existing domain records cover only part of this surface. | Include these tools in the intended product. Define ink, text and shape content, author, editing/removal and persistence. Erase removes ink; removing a record placement is a separate operation. Drawing does not establish evidence or approval. | Extend T2/T3.2 with IDs, revisions, attribution and collaboration for these elements. Define gesture cancellation, zoom-coordinate conversion and undo boundaries for shared changes. | Extend P5.2's board adapter to ink/text/shapes and input arbitration. Assess whether additional rendering primitives are needed under E-002; native mockup code does not select a replacement framework/library. |
| CM-12 — Note and frame interaction; clarification | Notes support inline editing, five selectable colours and explicit conflicts; frames can be renamed/resized and moved with contained cards. Loose cards also exist outside frames. Earlier specs leave most gestures unspecified. | Refine F1.2/F4.7: content revisions differ from visual properties; frames organize without assigning truth or workflow stage. Name the placement/frame-membership commands and define what moves with a frame. | Preserve independent Note/Placement revisions and explicit conflicts. Persist membership rather than re-deriving semantic ownership from coordinates. Define atomic validation of a multi-placement frame move. | Map colour to visual presentation, not evidence meaning. Keep text selection/editing separate from drag/pan. The mock's shared `record.revision`, coordinate containment and `placed` flag are simplified representations, not the durable schema. |
| CM-13 — Visual links versus Relationships; replacement | Dragging a card handle draws a `visualLink` and explicitly records no evidence or Decision. Semantic `supports`, `derived-from`, `depends-on`, `supersedes`, `results-in` records exist separately. P5.2 currently says dragging a connector requests a semantic Relationship. | Add a distinct visual-connection concept to F4.2/F4.7/F4.8. A drawn arrow has no evidential or execution meaning until an explicit semantic command is used. Preserve typed direction rules for actual Relationships. | Add separate visual-link persistence/commands and validation. Prevent a freehand/visual edge from creating supports, supersession or an execution prerequisite. Define removal and collaboration for each link kind. | Change P5.2's connector mapping. Render and inspect link kind distinctly; semantic links use Knowledge/Connected Work commands while visual links belong to the composition. |
| CM-14 — Arrangement history and personal view; clarification | Undo/redo are labelled visual changes; fit, search, zoom percentage, panel visibility and navigation can change without altering meaning. The specs already separate personal/ephemeral and committed state but need concrete examples. | Specify the toolbar behaviour and preserve DR-03/DR-11. Undoing arrangement cannot retract a Decision, restore approval or unsend email. Board search and fit do not mutate the knowledge graph. | Revision-aware shared undo is a new authorized command; local draft/view undo stays local. Define search scope and recoverable error feedback. Do not persist a browser's selection as shared domain evidence. | Retain P5.1/P6.2's state categories; map the mock controls to them. Keep inspector/chat controls outside the transformed canvas and allow context panel/navigation to collapse. |

The visual inventory to carry into F1.10, T2's projection definitions and P5's component conventions is more specific than “a canvas with cards”:

| Visual element | Observed design and behaviour to retain |
| --- | --- |
| Access composition | Split welcome/product-preview and sign-in panels; account/invitation tabs; labelled fields; a distinct invitation scope notice. On narrow layouts the panels stack rather than becoming board content. |
| Workspace and board cards | Scaled previews of actual composition, names/descriptions, ownership/guest badges, collaborator initials and activity metadata. Recent boards add a favorite star and visit/update signals. Empty/unavailable preview is an explicit state. |
| Template cards | Structure preview, name, explanation, category and Use template action; catalogue search and category chips sit above them. Preview content must match the resulting editable structure. |
| Board surface | Dotted background, spatial frames and loose cards; the active fixture is not a mandatory left-to-right process pipeline. Frame headings identify groups while card relationships explain meaning. |
| Notes | Inline multiline content; Yellow, Green, Blue, Pink and Purple palette; visible selected swatch. Notes, Text and Shape omit business-lifecycle badges. Colours are freely chosen presentation, not approval/status values. |
| Typed cards | Email/observed-task cards use the source family (purple in the dark theme), Proposals amber, Decisions green and external Actions blue. Type labels, actor, revision and lifecycle badges explain the difference independently of colour. |
| Relationships and handles | Visible connection points and directional curves; semantic edges use readable labels such as Evidence for, Based on, Needs first, Replaces and Follow-up action. Visual links remain separately identified. Superseded Decisions have a dashed border. |
| Toolbar and rails | Undo/redo, fit, expandable inline board search, presence, members and context-panel toggle above the canvas; creation/navigation tools and Agents shortcut at the side; zoom controls/percentage below. No persistent Create board button is shown. |
| Context panel | Exactly Inspector, Sources, Agents and Activity tabs, one active at a time. It can collapse independently of navigation; inspection and conversation remain readable outside canvas scaling. |
| Information versus actions | Pinned evidence and related-record entries are tinted information cards with type/icon, title, revision/direction and navigation affordance. Commands use visibly bordered buttons; primary commands use the product green (`#71efb2`). |
| Chat | Agent avatar/name and replies, differently aligned human messages, context chips, role selector, attach/send controls, expanding composer, run state/progress and secondary details. An empty conversation explains how to attach context. |
| Dialogs and feedback | Modal heading/close, scrollable content, explicit fields/evidence, Cancel/Save or operation-specific commands, inline validation and preserved focus. Status footer, toasts, badges, unread counts and activity outcome labels distinguish transient interaction from known durable outcomes. |
| Themes and responsive layout | Dark surfaces and a light theme with layered white/tinted surfaces; restrained shadows/borders, shared accent and focus states. Navigation becomes a drawer and the panel adapts at narrow widths; these states need implementation verification, not just copied breakpoints. |

The sample sizes, positions and zoom limits demonstrate interaction rather than a measured application capacity. Preserve the intended composition and control behaviour, then establish supported browsers, device/input targets and graph-size budgets through the existing platform verification process.

### 4.4 Proposals, decisions, memory and AI

| ID / difference | Canonical design and existing gap | Functional/domain effect | Technical effect | Platform effect |
| --- | --- | --- | --- | --- |
| CM-15 — Optional Proposals and deliberate placement; clarification | A human can submit a Proposal or record a Decision directly from selected context. Adoption creates editable working material, not a Decision; Proposal/Decision dialogs offer an explicit “Show … on the board” choice. AI output can initially remain unplaced. The existing rules agree but the older visual narrative suggests a fixed pipeline. | Refine F1.4–1.5/F2/F4.8: a Note need not progress through Proposal → Decision. Creating semantic material and placing it are separate choices. Define how unplaced material is found and deliberately placed. | Preserve separate record and Placement creation; maintain proposal state, evidence revisions and stale-context checks. Do not automatically add an agent result to every board. | Inspector/context menus/dialogs invoke Knowledge commands, then optional Visual Collaboration commands. Avoid making rendered node existence the condition for record existence. |
| CM-16 — Several current Decisions; clarification | The fixture has both a current briefing-time Decision and a current launch-date Decision, plus a superseded 10:00 Decision. “Current” is relative to a particular choice/supersession chain. | Clarify F1.5/F4.9: a workspace/board has no single mandatory “current direction”. Superseding one Decision does not retire unrelated Decisions. Preserve immutable accepted meaning and human recording. | Check the specific earlier Decision and prevent cycles/concurrent incompatible supersessions. Context change invalidates affected pending work, not all workspace knowledge. | Show per-record status/history and accurate counts; avoid a global selected-Decision singleton in the board controller. |
| CM-17 — Traceability instead of a second memory canvas; replacement | Inspector/Activity open a scrollable informational Traceability dialog with linked records, pinned revisions, details/history and Locate on board. Unplaced records remain inspectable. F1.7/F3 and AC-20 require navigation in a Memory Chart; P5.2 discusses a separate graph layout. | Replace the required memory-chart interaction with the visible traceability/history journey while preserving “why”, supersession and retained evidence. Define Locate on board versus Show on board: only the latter creates/restores a placement. | Revise T2 projections and AC-20's UI expectation. Provide authorized traversal and historical revision retrieval independent of board placements. Board search alone currently filters placed cards and cannot satisfy search over retained knowledge. | Rework P5.1/P5.2 memory routes and projections around traceability/details. A second graph canvas and ELK layout are not prerequisites for this journey; retain automatic arrangement only where separately justified. |
| CM-18 — Conversational AI; extension | Agents panel is a chat with role selector, message composer, explicit context chips, attach-selection control, progress, clarification, cancellation and inspect-contribution links. F1.4/T3.3 define assistance but not this conversation contract. | Specify conversations, human messages, agent replies, clarification and inspectable contributions. Selecting cards and sending a message are distinct from explicitly attaching their versions. “Ask agent about selection” is an explicit attach-and-open shortcut. | Add Conversation/Message references, run/message correlation, workspace/member scoping, persistence and cancellation/clarification transitions. Freeze and record the actual submitted Context Selection. Empty context prompts clarification; it does not silently attach the entire board. | Extend `agent_participation` and the Agents feature with conversation/run APIs and progress subscriptions. Separate draft text/role/attachments from persisted messages and accepted contributions; keep provider SDK types behind adapters. |
| CM-19 — Grounding, limits and stale output; clarification | Run details expose request/context and captured limits; context chips open pinned revisions. A run can wait for a reply, complete, fail or be cancelled. Scripted completion controls and example text do not establish real AI. | Preserve Facilitator and Action Coordinator responsibilities, human Decision/Approval authority and provenance. Clarification answers extend a run's recorded input; they must not silently replace its pinned references. | Extend T3.3's lifecycle to waiting/resuming and record messages supplied to inference as well as record revisions. Enforce configured timeout/tool budget, permitted tools and freshness server-side. Use real inference for AC-07/17. | Keep real provider integration behind `agent_participation`. Show operational status without exposing private model reasoning. A failed/cancelled run leaves attributable prior contributions and independently submitted Actions inspectable. |

### 4.5 External evidence and follow-through

| ID / difference | Canonical design and existing gap | Functional/domain effect | Technical effect | Platform effect |
| --- | --- | --- | --- | --- |
| CM-20 — Static external capture; behavioural replacement | The user explicitly rules out continuous synchronization after import. F1.3/F1.6/F2, DR-04, FA-06, T4.1 and AC-05/10 instead expect automatic arrivals/polling and returned external edits. Mockup Sources still offers automatic arrivals. | Replace live source updates with participant-requested import and optional refresh. A task status means “observed at capture”, not current provider truth. Update the demonstration and connected-work terminology as detailed in section 5. | Replace polling/cursor requirements with scoped browse/import/refresh commands and immutable observations. No scheduled refresh, webhook-driven mutation, reconnect catch-up or background agent reread may silently advance a captured reference. Revise the timed gates. | Keep connectors in `connected_work`, called on demand. Revise P7's integration description and worker responsibilities. Internal collaboration WebSockets remain; the external-record synchronization loop is removed from the target. |
| CM-21 — Connection management; extension/clarification | Sources displays provider, URL, account, scope, read/write capability, health and last check; Owner configures it. Endpoint/client values are illustrative. Connection configuration is already modeled but its UX is limited. | Specify configuration, scoped browsing, capture, refresh availability and failure states. Connection availability is separate from capture age and external task business status. Remove/reinterpret automatic-arrival controls. | Preserve backend credential references, scoped capability checks, provider ID/locator and fingerprints. Revalidate refresh permission and original source scope. Changing configuration cannot retarget an old capture; it can invalidate an Action approval. | Implement configuration and operation status through existing Access/Connected Work boundaries. Use environment configuration from the actual deployment; do not copy fixture URLs or its example `example.test` accounts as release settings. |
| CM-22 — Personal capture sharing; visible policy gap | Guided playback previews sharing selected personal email text, audience and provenance, then denies borrowing the owner's mailbox access. It is explicitly marked a proposed extension in the mock. Existing specs define workspace connections only. | Record this visible journey in the product alignment, including capture publisher, audience, selected text and exclusions. The prototype label signals unresolved sharing policy; it must not be silently discarded merely because old v1.0 scope is narrower. | If included in the revised release, authorize publication separately from private-source browsing and refresh; record provenance and permitted audience. Reading the capture grants no provider access, attachments, thread-wide access or send-as rights. | Model personal credential scope separately from workspace Connection scope. Reuse immutable capture and original-source navigation; do not share a personal session with board members. Confirm release inclusion and retention/revocation policy before implementation. |
| CM-23 — Reviewed writes remain distinct; clarification | Inspector can prepare a task or email reply, review exact fields, approve, execute, inspect result, fail or reconcile uncertainty. This largely agrees with F1.6/T3.4; no-continuous-sync does not remove the visible one-off Action workflow. | Preserve Action, Approval and Outcome as separate from imported evidence. Preparing/editing/moving a task-looking card does not write externally. Later provider changes become evidence only on explicit refresh. | Preserve exact-payload approval, scope/version checks, prerequisite validation, deduplication and uncertain-outcome reconciliation. A bounded worker executing/reconciling a requested Action is distinct from continuous record monitoring. | Retain P4/P7's action services and selected mail/Deck adapters. Store returned external identity and captured outcome separately from the Decision and Action. Real service calls are required to validate these operations. |

### 4.6 Activity, completeness and operational feedback

| ID / difference | Canonical design and existing gap | Functional/domain effect | Technical effect | Platform effect |
| --- | --- | --- | --- | --- |
| CM-24 — Activity and notifications; extension | There is a global timeline, bell/popover, unread badges, mark-all-read, category/search filters, a board-only timeline and View context links. F1.1/F4.2 describe only a short chronological view. | Specify global activity across authorized workspaces versus board-associated activity, personal read state and navigation to the event's record/run/action/evidence. Read does not mean adopted, decided or approved. | Define durable event identity, actor/time/outcome, explicit workspace and board association, authorized pagination/search and per-member read state. Do not infer association from a hardcoded event-ID set. Events for captures/refreshes record their initiator. | Add shared activity queries/components and notification state. Reuse committed internal publications. A provider event can enter this timeline only through an authorized capture, requested refresh or Action result, not an implied background sync. |
| CM-25 — Honest states and recovery; clarification | Empty results, unavailable previews, conflicts, stale proposals, pending approvals, offline connections, failed/uncertain actions and local-save errors are visible. Existing specs contain strong recovery rules but few screen-specific states. | Retain F1.8 and specify actionable states for every product flow: loading/empty/no access/error/success. A disconnected provider leaves captures readable; “Connected” never guarantees they are current. | Extend verification to auth/invite failures, creation failure, source disappearance/access loss, refresh failure and conversation interruptions. Keep accepted data and distinguish pending UI from durable results. | Reuse error/status/dialog patterns with accessible feedback. “Saved locally” must become honest server acknowledgement/pending/failure presentation in the application. |
| CM-26 — Demo mechanism versus product; clarification | Explore demo, playback/pause/next/restart, preview role switching, simulated arrival/outage/conflict and selectable execution outcomes are mockup mechanisms. The genuine product capability underneath each must still be delivered. | Define optional demo/onboarding behaviour separately. A real login, membership, AI run, import and Action cannot be satisfied by its simulation. Guided progression cannot force a team's work through one pipeline. | Preserve prospective AC status and real-service/inference gates. Add product tests over actual identities, persistence and adapters. Demo reset must be explicitly scoped, not a general deletion primitive. | Mockups remain design assets. Browser-local JSON, scripted AI and simulated provider results do not replace the Rust/PostgreSQL implementation. E-002 still owns renderer reassessment; the mockups alone select no new stack. |

## 5. External references: the corrected contract

### 5.1 What “static evidence” means

A Connection lets an authorized participant request an interaction with an external service. It does not maintain a continuously changing mirror of that service on the board. The external system owns its native email/task; Memento owns the captured observation and the knowledge built around it.

| Situation | Required Memento behaviour |
| --- | --- |
| A participant imports a message or task | Capture the selected external content and metadata as an immutable Source Observation; create/reuse its Source Record identity and explicitly place the chosen capture where requested. |
| The external task changes or is completed | The imported card remains at its captured state until a participant requests refresh. Do not silently update status, content, relationships, AI context or Decisions. |
| A new email arrives | It does not automatically appear on the board. A participant can browse the configured scope and import it. |
| The user opens the original | Navigate to/locate the external record. The external service applies that viewer's own authorization; viewing it does not refresh Memento. |
| A participant manually refreshes | Recheck current authority and source scope; fetch on demand; preserve previous captures; record who requested it, when, and the result. A changed capture becomes a new observation, never an overwrite. |
| The provider reports no content change | Record the refresh check/result without inventing a semantic content change. Deduplication may reuse the existing capture/fingerprint. |
| Refresh fails, access is revoked or the native record is gone | Preserve existing evidence and annotations. Report that the requested refresh failed/unavailable; do not replace the capture with empty content or imply it is now current. |
| The connection recovers or the application restarts | Restore Memento state and operation status. Recovery alone does not refresh imported references. |
| A real approved Action creates a task or delivers a reply | Record the returned/confirmed result and external identity. This is a specific operation outcome. Further native changes still require requested refresh. |

Internal collaboration remains live: when Maya deliberately imports or refreshes material, Leo can receive the resulting committed **Memento** update. This is not continuous synchronization with the external provider. Likewise, presence and agent progress can use WebSockets without changing this external-evidence rule.

### 5.2 Identity, history and display consequences

The existing Source Record / immutable Source Observation / Evidence Reference distinction is a strong foundation and should be retained. The amendment must tighten its consumers:

- Source Record identifies the native record through its original Connection/source identity. It can index observations, but a global “latest observation” pointer must not silently advance every card or evidence reference.
- A displayed imported reference needs an explicit observation reference. **Derived design contract:** record which capture a Placement/view presents, and change it only through the user's refresh/selection operation. If a refresh can update several placements, that scope must be explicit. It cannot quietly update unrelated boards.
- Evidence References on Decisions and Agent Runs continue to pin exact revisions. Refresh does not retarget existing evidence or retrospectively add the new data to a run.
- Local annotations, card geometry, frame membership and semantic relationships survive refresh. Their authorship and revisions remain independent of provider content.
- A pending Proposal may become stale when newer evidence is deliberately introduced or its actual referenced working context changes. An unseen external edit cannot trigger a freshness claim. A recorded Decision keeps its original meaning and evidence; a human may reassess/supersede it.
- Display “Captured at …”, “Observed status at capture …”, “Last refresh attempt …” and “Refresh source”. Connection availability, capture age and external business status are separate fields. Avoid “Live”, “Up to date” or “Synced” where only an old capture is known.
- Provider credentials remain server-side. Members may read published workspace evidence under workspace authorization, while opening the original and requesting refresh can require separate provider capability. Personal-source publication policy remains explicitly unresolved under CM-22.

For example: import a Deck task while it is Open; complete it in Deck; the Memento capture still says Open with its capture time. Press Refresh source; a new observation says Completed. A Decision recorded against the original Open observation still opens that original revision. Merely reconnecting or revisiting the board performs none of these refresh steps.

### 5.3 Contradictory mockup controls and labels

| Current mockup example | Required interpretation/revision in the aligned product |
| --- | --- |
| Sources: “New arrivals: place automatically”; configuration checkbox for automatic placement | Replace with explicit browse/import behaviour. Do not carry an automatic-arrival toggle into the corrected connector contract. |
| “Simulate scoped arrival” | Demo machinery; if retained, demonstrate a record becoming available for participant selection, not unrequested board mutation. |
| Actors such as “Email sync”, “Task sync”, “Mail sync”, “Deck sync” | Attribute to the importing member and connector capture/refresh operation. System-origin normalization remains attributable, but the copy must not imply a standing synchronization service. |
| Timeline “Task status updated” and demo step “Leo completes the task in Deck” | Include the explicit requested refresh before a new captured status appears. The provider edit and the Memento capture are different events. |
| “Reading resumes after recovery” | The connection becomes available for new requested operations. Existing captures remain unchanged until requested refresh. |
| Connection-level “Last checked” | Describe the connection/operation check, separately from each reference's captured version and time. |
| Inspect original / Refresh source / Observation history | Keep these separate controls: original navigation, deliberate capture update, and historical evidence inspection. |

### 5.4 Connector and worker consequences

T4.1's compulsory incremental retrieval/polling cursor becomes an on-demand browse/import/refresh contract. Pagination or bounded retrieval may still be used inside a requested browse/refresh; it does not imply scheduled monitoring. Stable provider IDs, locators, normalized fields, fingerprints, permission checks, error classification and import deduplication remain required.

Durable jobs remain useful for a requested refresh, model inference, approved external execution and uncertain-outcome recovery. They must record the initiating command and its scope. The static-evidence rule removes the product's need for a continuing external synchronization worker; it does not remove the internal committed-update outbox or the duty to establish the outcome of an already submitted Action.

Stalwart and Nextcloud Mail/Deck remain the selected local services. Their real-read/write/client checks still matter. No new mail, task, chat or document provider is selected by the mockups. Nextcloud Talk/Files availability remains independent of Memento connectors; the personal-email sharing prototype adds no general file/attachment integration.

## 6. Owning-specification amendment map

The catalogue above gives per-feature effects. This table groups the actual edits needed when the specifications are revised after this comparison.

| Owner | Sections to amend | Required amendment |
| --- | --- | --- |
| Functional/domain | 1.1, 2.1, 3, 4.2–4.3, 4.7–4.9 | Define account/invitation/guest journeys, workspace/board catalogue and creation, members, personal return state and template instantiation. Add domain terms/commands only where these capabilities introduce real lifecycle or ownership. |
| Functional/domain | 1.2, 1.10, 3, 4.2, 4.7–4.8 | Replace image authority; include visible editor tools and visual connections; specify presentation versus knowledge commands. Remove the blanket deferrals that conflict with the design. |
| Functional/domain | 1.3, 1.6, 1.8, 2.1–2.2, 3, 4.2–4.3, 4.8–4.10, 5 | Replace automatic ingestion/returned changes with explicit import/refresh; rewrite DR-04's synchronization wording, Capture Source Observation's initiator and FA-06's automatic-completion expectation. Retain native authority and evidence preservation. |
| Functional/domain | 1.4–1.7, 2, 4.2, 4.7–4.9, 5 | Define conversation/context attachment, optional Proposal/direct Decision, several current Decisions, unplaced results and traceability/details/history. Revise Memory Chart terminology and FA-07's presentation expectation without weakening retained evidence. |
| Functional/domain | 1.1, 4.2, 5–6 | Define Activity/notification scope and read state; resolve personal capture sharing, initial account provisioning and guest return policies. Update open questions that currently assume a continuously observed project mailbox. |
| Technical | 1–2, 3.1–3.3, 5 | Add identity/invitation, catalogue, template, visual element/link, conversation/message and notification contracts. Preserve stable IDs, context ownership, immutable observations, revisions and server authorization. |
| Technical | 2.1–2.3, 3.1, 4.1–4.3, 5–8 | Replace the continuous source-retrieval contract and workers with requested capture/refresh. Keep approval/execution/reconciliation. Update scenario setup, operating diagnostics, gate wording and sequence. |
| Technical | 2.1, 6, 6.1 | Replace mandatory Memory Chart navigation with the canonical traceability journey; extend acceptance coverage to the visible product shell and tools. Keep every existing AC identifier stable when revising its passing result. |
| Platform | 1, 3, 5.1–5.3 | Remove outdated scope exclusions; extend routes/features, view projections and reusable controls; add template and personal navigation-state persistence. Define visual elements/links separately from domain Relationships. |
| Platform | 4.1–4.3, 6–7 | Map access/invitation to `workspace_access`, templates/geometry/visual links to explicit composition use cases, knowledge/history to `collaborative_knowledge`, conversation/runs to `agent_participation`, requested captures/actions to `connected_work`. Cross-context orchestration must respect existing write ownership. |
| Platform | 5.2, 7, 9–10 | Revise connector-drag semantics, traceability/ELK assumptions, on-demand external adapters and verification. Preserve React/TypeScript, Rust/Axum/Tokio and PostgreSQL/SQLx until an explicit platform decision changes them. |

The existing five bounded contexts remain a suitable starting map. Home, Activity and Traceability are composed views; a new screen is not automatically a new bounded context. New aggregate/transaction boundaries for invitations, templates or conversations need specification detail, but the design does not require microservices, a graph database, CRDT adoption or a proprietary collaboration service.

The three existing Markdown specifications and their PDFs have not been rewritten in this task. When those sources are amended, regenerate their existing PDF exports together, correct image references and validate relative links, as required by the [delivery guide](../../Design%20and%20Implementation%20Guide.md). This new Markdown comparison has no pre-existing PDF export to synchronize.

## 7. Acceptance and demonstration consequences

### 7.1 Existing ACs

| Gate(s) | Effect of the canonical design / required revision |
| --- | --- |
| AC-01 | Clean start must expose actual Access/Home and authorized workspace entry, not merely a seeded ready board; document real account/invitation and provider prerequisites. |
| AC-02–04 | Preserve convergence, conflict and durability. Extend coverage to newly supported visual elements/links and records, personal return state where durable, and conversational/notification history. |
| AC-05 | Replace “mail appears within 15 seconds under 5-second polling” with real explicit browse/import, capture provenance and repeated-command deduplication. Define any operation-latency target after measuring the reference environment. |
| AC-06 | Change the source externally and first prove the capture stays unchanged; then request refresh and inspect the new capture and the original pinned Decision evidence. |
| AC-07 | Exercise actual inference through Agents chat with explicit pinned attachments, useful contributions, run progress/limits and clarification where needed. Scripted demo completion is insufficient. |
| AC-08–09, AC-11 | Preserve exact approval, real task creation and real reply delivery. The static-evidence rule does not weaken these operations or their independent client verification. |
| AC-10 | Replace automatic external completion return with “complete externally → verify unchanged Memento capture → request refresh → observe new captured completion”; preserve annotations/geometry/rationale. |
| AC-12 | Exercise an explicitly requested read/refresh and an Action during an actual outage. Preserve captures and show failed/uncertain operation states; reconnection does not silently refresh them. |
| AC-13 | Preserve duplicate-operation/Action recovery; include import/refresh deduplication and safe restart of requested operations. |
| AC-14 | Extend isolation across Home/search/previews, invited guests, templates instantiation, notification scopes, conversations, retained evidence and requested provider operations. |
| AC-15 | Extend attribution to invitation acceptance, publication/import/refresh initiators, conversation/run messages and the new visual elements. Avoid fictitious synchronization actors. |
| AC-16 | Preserve cancellation and approval freshness. Proposal freshness depends on recorded context changes/explicitly introduced observations, not unseen external edits. Include clarification/resume behaviour. |
| AC-17 | Rewrite the complete walkthrough for actual Access/Home, deliberate evidence import and requested refresh, canonical chat/traceability and genuine one-off external actions. Keep two reset runs and outage evidence. |
| AC-18 | Preserve actual delivery/verification evidence. This comparison supplies design-analysis evidence only. |
| AC-19 | Preserve human Decisions and acyclic supersession. Add direct Decision without mandatory Proposal, adoption without Decision, independent current Decisions and optional placement. |
| AC-20 | Preserve independent Placement/evidence history; replace compulsory Memory Chart navigation with Traceability/details/history, including unplaced records and a defined retained-record search path. |
| AC-21 | Run the expanded workflow with one member; multi-workspace navigation, templates, agent chat and approval must not require a second human. |

Do not mark any gate passed merely by changing its wording. Existing evidence retains the meaning of the behaviour it actually checked. New or changed contracts need new implementation evidence.

### 7.2 Additional acceptance examples for the specification rewrite

These are proposed coverage examples, not newly allocated release AC identifiers or executed product tests.

| Ref | Observable passing example |
| --- | --- |
| MC-V01 — Identity | Invalid credentials do not open a workspace; valid credentials establish a server session; expiry/logout ends authority; recovery cannot be simulated by a toast. |
| MC-V02 — Invitation | An authorized Owner issues an invitation; its valid use creates attributable Editor access only to that workspace. Invalid/expired/revoked invitations fail understandably; entering another workspace URL grants nothing. |
| MC-V03 — Workspace and board | Create a workspace and first empty board, reload/restart and reopen them. Owned/shared filters and previews contain only accessible work. A second workspace preserves the first; existing multiple boards open with their own identities and content. |
| MC-V04 — Personal navigation | Favorite and open a board; Recent/Favorites and changes-since-visit behave for that member, independently of another member's markers. |
| MC-V05 — Templates | Create from each visible template; the promised editable structure is present, all new IDs/references belong to the new workspace/board, and no actual Decision or approved Action is pre-invented. |
| MC-V06 — Composition | Create/edit/move a Note, Text, Shape, Frame and ink stroke; draw/erase ink; draw a visual link. Retain the result across reload/restart and concurrent sessions without inventing semantic evidence. |
| MC-V07 — Links and undo | A dragged visual connector does not establish `supports` or an execution prerequisite. A deliberate semantic command does. Visual undo changes arrangement only and respects concurrent accepted changes. |
| MC-V08 — Static capture | Import a real message/task; edit it externally; wait, revisit and reconnect; the displayed capture does not change and no unrequested refresh operation is recorded. |
| MC-V09 — Explicit refresh | Request refresh after that edit; inspect the new observation, initiator/time and old historical capture. Repeating the operation does not duplicate material. Historical Decision/run evidence remains pinned. |
| MC-V10 — Failed refresh | Make the provider unavailable or deny access; refresh reports failure while the original content, annotations and layout remain readable. Restoring connectivity alone changes none of them. |
| MC-V11 — Conversation | Attach selected versions, send, inspect run context and answer a clarification. Merely selecting other cards does not change submitted context. A request with no attachments does not silently ingest the board. |
| MC-V12 — Knowledge and memory | Adopt a Proposal without a Decision; separately record a Decision, optionally leave it unplaced, and inspect it in Traceability. Supersede one of two current Decisions; the other remains current. |
| MC-V13 — Activity | Home activity shows authorized workspace events; board activity shows that board's events. Mark-read affects only the member's read state. View context opens the actual permitted record/run/action or a clear unavailable result. |
| MC-V14 — Presentation | Use light/dark themes, collapsed navigation/context panel, keyboard context menus and dialogs. Inspect/edit at different zooms and narrow widths without accidental moves or inaccessible controls. |
| MC-V15 — Writes | Approve/execute a real task and reply; inspect external results. Completing the task later changes Memento only after requested refresh. Uncertain submission is reconciled before a repeat attempt. |
| MC-V16 — Personal sharing, if included | Preview selected personal email text and the audience, deliberately publish that capture, and inspect it as another member. That member receives neither private mailbox access nor permission to refresh/send as the publisher. |

The revised Atlas demonstration should enter through the actual product shell, import the stakeholder's message deliberately, attach evidence in Agents chat, record the revised Decision, approve and verify the real task/reply, then demonstrate that external task completion is invisible to the capture until Refresh source is requested. Finish with Traceability and restart recovery. This preserves the original decision-to-action story while correcting its source-update assumptions.

## 8. Release-plan and implementation effects

The existing plan is a useful record of prior scope/evidence but does not yet schedule the full canonical product. The comparison expands and refines its target; it does not authorize implementation or establish acceptance of a new plan.

| Existing area | Alignment needed before implementing against it |
| --- | --- |
| P1 / SL-001 | Include a defined real entry/session and workspace/board creation journey. Plan invitations/membership, Home and the visible editor capabilities in appropriately sized follow-on slices; the original minimum Note slice does not cover the full shell. |
| P1 / SL-002 | Preserve explicit conflicts/reconnect and authorized subscriptions. Extend their contracts to the new visual elements and selected durable user state when introduced. |
| P1 / SL-003 | Keep semantic Relationships and add a separate visual-connection workflow. The same drag gesture cannot stand for both without an explicit choice. Preserve frame/Placement and retained-record semantics. |
| P2 | Replace continuous external arrival/change monitoring with real scoped browse/import/manual refresh and historical captures. Keep actual-service isolation, failure, deduplication and client inspection. |
| P3 | Include Agents conversation, attachment/clarification, optional unplaced results, direct human Proposals/Decisions and informational Traceability. Retire a required second Memory Chart canvas. |
| P4 | Preserve real reviewed writes and reconciliation. Separate returned Action outcome capture from subsequent requested refresh of the native result. |
| P5 | Rehearse the complete canonical product journey, static-evidence behaviour, account and guest access as scoped, one-member use and updated acceptance checks. |
| E-002 | Evaluate the visible editor surface, rich cards, panels, visual/semantic links and server-command mapping. Keep the current platform baseline until a replacement is explicitly selected with evidence. |

Templates, member management, account recovery, favorites/visits and activity need allocated delivery work; they cannot be considered implemented by a seeded Atlas board. The next planning pass should distribute them into reviewable slices, keep existing slice IDs stable, and update scope/AC references together. No new slice IDs are allocated by this comparison.

The selected local services and their retained environment evidence remain useful. They do not demonstrate a Memento login, captured-reference contract, connector, inference integration or complete product workflow. No service was started or changed for this documentation comparison.

## 9. Decisions the mockups do not finish

| Open decision | What is known / what still needs definition |
| --- | --- |
| Account provisioning | Account login and recovery are visible. Self-registration, administrator provisioning or external identity federation is not selected. Choose the account lifecycle and authentication mechanism. |
| Invitation policy and guest continuity | Workspace-specific Editor entry without a global account is explicit. Define token lifetime/reuse, revocation, guest return after session loss, identity changes and the permitted guest approval policy within the existing Editor rights. |
| Ownership/member administration | Owner/Editor responsibilities exist. Define removal, owner transfer/last-owner rules and which management operations belong in the first revised release; the preview role selector is not that policy. |
| Workspace/board lifecycle | Workspace/first-board creation and existing-board opening are visible. Multiple boards are listed, but there is no active Create board control. Decide whether/how further boards are created without treating retained handlers as a request to restore removed controls. Rename/archive/delete and retained-evidence consequences are not designed comprehensively. |
| Template definitions | Six structures are promised by previews. Define exact seeded content, versioning and instantiation; decide whether user-authored templates are supported. The design does not show a template authoring product. |
| Capture refresh scope | No continuous synchronization is settled. Specify how a user selects which displayed references advance during one requested refresh and whether batch refresh is exposed; historical pinned evidence always stays fixed. |
| Personal-source publication | The guided prototype makes audience/provenance and separate original authorization clear. Confirm release inclusion, who can refresh a published capture, redaction/withdrawal and evidence-retention rules. Its prototype status is not completed product policy. |
| Retained-material search | Board search filters displayed material; Traceability exposes unplaced records. Define a usable search route for retained knowledge so removing a placement never makes required history undiscoverable. |
| Conversation scope/retention | Board chat is visible. Define who can see a conversation, whether multiple threads exist and retention; do not assume private or workspace-global chat from the panel alone. |
| Operational profile | Model provider, measured host/browser support, connector protocols/reconciliation and renderer reassessment remain open. The mockup resolves none of these by using native JavaScript or sample endpoints. |

There is no need to infer billing, enterprise SSO, public workspaces, calendar operations, general document/attachment connectors or autonomous agents from the visible screens. Those require their own product decisions. The actual visible template/freehand/access journeys, however, must be reconciled with release scope rather than dismissed by obsolete exclusions.

## 10. Verification record and review boundary

**Performed for this comparison:** read all six HTML files and reviewed their shared/application/active-board/demo scripts and styles; compared the three current specifications, guide, release plan, UI-spike specification and applicable environment boundaries. Checked actual navigation/renderer dispatch rather than treating every retained function as a visible feature.

Rendered the local files in isolated headless Chromium at 1440 × 1000 and captured 14 states: Access, Invitation, Home, New workspace, Activity, Templates, Create from template, Board/Inspector, Sources, Agents, Board Activity, Traceability, Members and light theme. Inspected the six primary screen/entry states and board/panel/dialog captures visually. The capture run reported **zero page errors and zero HTTP(S) requests**; HTTP(S) requests were blocked. This establishes local mockup rendering only. Mobile behaviour was reviewed in CSS, not validated across devices in this task.

The existing mockup regression files were inspected as supporting implementation context; their full suites were not rerun. Their simulations cannot establish the revised product contracts. Real authentication, concurrent users, persisted application workflows, connector operations, actual inference and the acceptance examples in section 7 remain unverified by this task.

**Documentation checks passed:** 53 relative file/section references and 21 Markdown tables across this document and the release plan; complete, unique CM-01–CM-26 and MC-V01–MC-V16 catalogues; source whitespace checks and `git diff --check`. The render report confirmed 14 captures, zero page errors and zero external requests. These checks validate the comparison artifact, not the prospective application workflows.

**Review boundary:** comparison document prepared; human acceptance pending. The next authorized stage can reconcile the three specifications and their PDFs, update release scope/briefs, and correct contradictory mockup copy/controls. No application or mockup changes are made by this comparison.


## 11. Resolution into the five-document baseline

The following records where each discrepancy is specified after the authorized rewrite. "Specified" means a normative target/policy is authored, not implemented, tested in the real product or human-accepted. Technical choices still requiring evidence are bounded in Platform section 9 and Technical section 9. Current product policies supersede the unresolved policy list preserved in historical section 9.

| Item | Resolution in the new baseline | Owning definition / trace |
| --- | --- | --- |
| CM-01 | HTML design authority and historical image status are explicit. | [Product 1](../memento-spec.md#1-reading-order-and-authority); Experience 1/6. |
| CM-02 | Provisioned account, session/sign-out and real recovery policy; no public registration. | Product 6.2; Technical 2; US-01/02, R-01/02; AC-22. |
| CM-03 | Seven-day single-use invitation, scoped guest Editor and bound re-entry preserve identity. | Domain 3; US-03/05; DR-15/16; AC-22. |
| CM-04 | Owner/member management, account-backed transfer and last-owner protection. | Domain 3; US-04; WA-06; AC-14/22. |
| CM-05 | Shared shell, authorized routes and narrow navigation/panel contracts. | Experience 6; US-36; Platform 4. |
| CM-06 | Multiple authorized workspaces and atomic first-board creation. | US-06/07; WA-03/WA-Q2; DR-17; AC-23. |
| CM-07 | Existing multiple boards, true previews and personal favorite/visit state; no extra Create board toolbar. | US-07/08; VC-07/VC-Q1; DR-23; AC-23. |
| CM-08 | Workspace-first creation is complete; additional-board creation UI is explicitly deferred. | Product 2/6.3; US-06; Experience 6. |
| CM-09 | Six built-in versioned structures and exact editable prompts are included. | [Template definitions](../memento-experience-spec.md#5-built-in-template-definitions); US-09; AC-24. |
| CM-10 | Canonical themed component/input states are defined and map to owned CSS. | Experience 6; US-36; Platform 4; AC-26. |
| CM-11 | Ink/erase, Text/Shape and board-owned visual material are included. | Domain 5; US-14/15; VC-04; AC-26. |
| CM-12 | Freeform inline Notes, separate colour/geometry and explicit Frame membership. | US-10/11/13; Domain 4/5; DR-03/12. |
| CM-13 | Drawn links are distinct from semantic Relationships and execution prerequisites. | Domain 4.2/5; US-16/17; DR-24. |
| CM-14 | Safe visual undo, personal view and fixed readable context controls. | US-18/36; Technical 3.3; DR-11/23. |
| CM-15 | Human Proposals/direct Decisions, adoption and optional Placement are separate. | US-27/28/29; CK-02/03/04; AC-19. |
| CM-16 | Independent current Decisions and specific predecessor supersession. | US-30; CK-05; DR-08; AC-19. |
| CM-17 | Informational Traceability plus retained search replaces mandatory second canvas. | US-34; CK-Q1/CK-Q2; DR-22; AC-20. |
| CM-18 | Board-shared conversation, explicit attachments and one active Run. | Domain 6; US-24/25; DR-21; AC-27. |
| CM-19 | Real inference, clarification expiry, active-time/tool bounds and cancellation. | Technical 6; US-25/26; AP-02/03/04; AC-07/16/27. |
| CM-20 | Import/manual refresh with immutable observations; no background source synchronization. | Product 6.1; Domain 7.1; US-20/21; DR-04/18/19; AC-05/06/10. |
| CM-21 | Scoped Connections, provider secrets and separate health/capture/business state. | US-19/22; CW-01/CW-Q1; Technical 5. |
| CM-22 | Narrow personal selected-text publication is included with explicit audience and no transferred private capability. | Product 6.3; Domain 7.1; US-23; AC-29. |
| CM-23 | Exact one-off task/reply Approval/execute/outcome/reconciliation retained. | Domain 7.2; US-31/32/33; AC-08/09/11/12/13. |
| CM-24 | Global/board Activity, actual context links and member read markers. | US-35; DR-26; Technical 3; AC-25. |
| CM-25 | Common pending/empty/denied/conflict/error/recovery states apply across stories. | Experience 1/4/6; Technical 2/3/5/6; AC-03/12/14. |
| CM-26 | Simulation remains isolated; current platform explicitly follows user selections and real acceptance gates. | [Platform](../memento-platform-spec.md); Product 9; Technical 8; release plan 3.4. |

The coordinated rewrite also adds DR-25/AC-28 for SQLite/PostgreSQL parity and AC-30 for the Podman/Bun/reference profiles, preserving AC-01 through AC-21 identities. The release plan records actual documentation/export checks independently of prospective implementation. Existing OpenSpec experiment artifacts retain their original evidence/scope; project context now points to the five-document target.
