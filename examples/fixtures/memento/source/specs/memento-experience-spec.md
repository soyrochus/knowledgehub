# Memento - Experience Specification

Version 1.1 - 2026-10-29 - Outcome-led delivery cuts; story requirements/scenarios retained; human acceptance pending.

The [Product Specification](memento-spec.md) owns purpose, language, actors and product policies. This document owns observable product requirements R-01 through R-36 and scenarios S-01a through S-36b. The [Domain Specification](memento-domain-spec.md) owns referenced rules/use cases, [Technical](memento-technical-spec.md) owns AC release checks, and [Platform](memento-platform-spec.md) owns realization. These are developed target contracts, not implementation evidence.

## 1. Experience authority and common contract

The canonical entry is [index.html](../mockups/index.html), redirecting to [Access](../mockups/01-access.html). [Home](../mockups/02-home.html), [Activity](../mockups/03-activity.html), [Templates](../mockups/04-templates.html) and [Board](../mockups/05-board.html) define the product surfaces. Their supporting scripts supply dialogs and contextual interactions. Prototype outcomes and browser-local persistence demonstrate intent; real application operations must enforce these requirements server-side.

Every flow exposes loading/pending, empty, no access, recoverable error and success states where applicable. Acknowledged means committed by the application, not merely shown optimistically. Validation keeps the participant's draft available. Expired access returns to Access with a safe return destination; authorization is rechecked after entry. Unknown and forbidden object URLs do not disclose restricted titles or content.

Keyboard users can activate controls, inspect and edit material, open contextual actions and complete dialogs. Pointer/touch navigation and text input do not compete. Status is expressed in text and control state as well as colour. The board remains a spatial composition on narrow screens; context inspection remains readable independently of zoom. Technical section 8 defines the verification targets.

## 2. Journeys

### UJ-01 - Think together

**Actor/entry:** Member with an accessible Board, alone or with other members. **Flow:** create a Note, type directly on it, select/move material, create Frames/visual elements and connect drawings; deliberately establish supported semantic relationships where needed. Other sessions receive accepted changes and show presence. **Alternatives:** a new board is empty; long text wraps; notes may remain loose outside Frames. **Interruption/outcome:** incompatible content or arrangement changes preserve accepted work and offer draft recovery. The composition and content reopen independently after restart. Stories US-10 through US-18.

### UJ-02 - Bring in working context

**Actor/entry:** Member with read access to a workspace Connection. **Flow:** browse its scoped records, inspect the selection, explicitly import onto a Board and inspect captured content/provenance. Annotate locally or open the original separately. Later request refresh for that specific displayed reference. **Alternatives:** duplicate import locates/reuses captured material; unchanged refresh records a check. **Interruption/outcome:** failed refresh preserves prior evidence; provider edits, arrivals and reconnection do not silently change the Board. Stories US-19 through US-22.

### UJ-03 - Seek agent assistance

**Actor/entry:** Member in a Board's shared conversation. **Flow:** select records, deliberately attach their versions, choose Facilitator/Action Coordinator and send a message; inspect pinned references, progress and contribution. Reply to clarification within the same Run. **Alternatives:** send without attachments and answer the clarification; keep a draft while another Run is active. **Interruption/outcome:** cancellation, timeout, invalid output or lost authority is visible. The resulting Proposal/draft Action remains distinct from human Decisions/Approvals and is not automatically placed. Stories US-24 through US-28.

### UJ-04 - Establish a direction

**Actor/entry:** Member with meaningful Note, source or Proposal context. **Flow:** record a chosen statement, rationale and selected evidence directly, optionally show it on the Board; alternatively review/adopt a Proposal first. **Alternatives:** multiple unrelated Decisions remain current; a Decision may stay unplaced. **Interruption/outcome:** recording a replacement preserves its predecessor and exact historical evidence, rejecting stale/cyclic supersession. No Note is converted or locked. Stories US-27 through US-30.

### UJ-05 - Follow through

**Actor/entry:** Member with a current Decision and permitted external capability. **Flow:** prepare task/reply content, inspect the exact operation, approve its version, explicitly execute and inspect its real Outcome and external identity in the supplied client. **Alternatives:** reject or revise a draft; require another Action to succeed first. **Interruption/outcome:** uncertainty requires reconciliation before retry. A later provider task completion does not change the captured result until explicit refresh. Stories US-21, US-31 through US-33.

### UJ-06 - Recover shared memory

**Actor/entry:** Returning member seeking the reason for an outcome. **Flow:** search retained knowledge or inspect a record, follow Traceability, pinned revisions, superseded Decisions and resulting Actions; locate an existing placement or deliberately show an unplaced record. **Alternatives:** start from Activity or search text; several boards can present the same underlying record with their own arrangement. **Interruption/outcome:** provider unavailability does not remove preserved captures; process restart preserves the history. Stories US-34 and US-35.

### UJ-07 - Resolve interrupted work

**Actor/entry:** Member facing a revision conflict, failed requested read, waiting/stale agent work or uncertain submitted operation. **Flow:** inspect current accepted state and the preserved draft/attempt; choose the offered domain-appropriate next action. **Alternatives:** keep the accepted revision, apply a reviewed new revision, request fresh assessment, retry an unsubmitted read, or reconcile provider submission. **Outcome:** no accepted edit is silently overwritten and no unknown external operation is blindly repeated. Stories US-11, US-21, US-25, US-26 and US-33; applies across journeys.

### UJ-08 - Enter and recover account access

**Actor/entry:** Provisioned account holder at Access or an expired protected route. **Flow:** enter email/password, select session persistence, authenticate and reach permitted Home/work; sign out to end authority. **Alternative:** request recovery using the verified account address, redeem the single-use recovery link and set a replacement password. **Interruption/outcome:** invalid/expired credentials or recovery tokens disclose no protected data. There is no public self-registration journey in this release. Stories US-01 and US-02.

### UJ-09 - Join and manage participation

**Actor/entry:** Owner in Members, or recipient at the invitation entry. **Flow:** Owner issues/revokes an Editor invitation; recipient sees its workspace/inviter/access, chooses a display name and redeems a valid code/link. Owner can remove a member or transfer ownership. **Alternatives:** an authenticated account accepts into its identity; an existing guest returns using a bound re-entry invitation. **Interruption/outcome:** expiry, replay and last-owner removal fail clearly; guest access never opens unrelated workspaces. Stories US-03 through US-05.

### UJ-10 - Start and resume workspaces

**Actor/entry:** Account holder at Home, or invited guest entering their single workspace. **Flow:** account holder creates a named workspace with a private first Board; members browse owned/shared work, search, inspect previews and open accessible boards. Favorite a board, inspect updates since the last visit and return later. **Alternatives:** empty catalogue or no matches; guest has no global creation authority. **Interruption/outcome:** creation failure leaves no false success; preferences/visit markers belong to the individual. Stories US-06 through US-08.

### UJ-11 - Start from a template

**Actor/entry:** Account holder at Templates from navigation or Home's starting strip. **Flow:** search/filter six built-in structures, inspect their visual preview, choose Use template, name a workspace and create its editable first Board. **Alternatives:** choose an empty workspace instead; cancel before creating anything. **Interruption/outcome:** instances have fresh IDs and keep their originating template version without receiving future template edits. Prompts labelled Decision or Next action are ordinary editable working material, never accepted commitments. Story US-09.

### UJ-12 - Follow activity and return to context

**Actor/entry:** Member at the global bell/page or a Board's Activity panel/page. **Flow:** review chronological events, filter/search, inspect actor/time/outcome and open the actual record/Run/Action; mark personal read state. **Alternatives:** a board-scoped view retains that scope when opened as a page and returns to that board. **Interruption/outcome:** empty filters and unavailable contexts are understandable; reading changes no business state and no other person's unread markers. Story US-35.

### UJ-13 - Publish selected personal evidence

**Actor/entry:** Member who owns a private mail authorization and wants to share selected evidence. **Flow:** privately select message text, preview exactly the content/metadata and workspace audience, confirm publication, then inspect the published capture as another member. This workflow is exposed through Sources and its publication dialog. **Alternatives:** cancel without sharing, or publish a later capture as a separate explicit version. **Outcome:** members can read only the published evidence; opening the original needs their own provider permission. No recipient can browse/refresh through or send as the publisher. Story US-23.

## 3. User story map

The backbone runs from entry and participation through composition, evidence/AI, commitment and recovered context. Each column groups related user tasks. Rows are cumulative outcome increments; the release plan owns their dependencies, scheduling rationale and independently reviewable slices. Every story remains required for v1.0. Early rows name partial coverage rather than claiming a whole story or gate is complete. Phase labels reflect the revised delivery sequence; journeys and US/R/S identities and definitions are unchanged.

| Delivery cut | Enter and resume | Think together | Capture and consult | Decide, act and remember |
| --- | --- | --- | --- | --- |
| P1: retain supported direction | US-01 session and US-06 creation foundations | US-10 to US-12 foundations; US-36 entry/inspection portions | Exact Note revisions as evidence; no provider or AI needed | US-29 direct human Decision; US-34 exact retained inspection; US-35 committed-event foundations |
| P2: capture evidence in context | Earlier entry retained | US-12/13/16/17/18 basics; source layout and Notes | US-19 to US-22 workspace capture and explicit refresh | Extend US-29/34 to pinned source observations; US-35 capture-event foundations |
| P3: perform reviewed follow-through | Earlier access retained | Exact Action review and result inspection | US-21 explicit result refresh; no AI dependency | US-31 to US-33: one task first, then reply/prerequisites; US-34 outcome inspection; US-35 Action-event foundations |
| P4: broaden participation and assistance | Complete US-01 to US-08 | Complete US-10 to US-18 editor portions; optional placement | US-23 personal publication; US-24 to US-28 grounded assistance and Proposal review | US-30 supersession; complete Proposal/Decision portions of US-34; expand US-35 to these events |
| P5: complete product rehearsal | US-09 templates and all return paths | Full US-36 target checks | All provider/AI failure paths and earlier flows | Complete US-34/35 navigation and all stories; one-member and deployment rehearsal |

Tasks under the backbone include account entry/recovery, invitation administration, workspace discovery/creation, personal return state, authoring/arrangement, evidence selection/refresh/publication, context attachment/clarification, proposal review, explicit direction, exact operation review, outcome recovery and history navigation. Their stories are the units below; technical tasks are introduced only in implementation changes.

Story dependencies are explicit: US-06 needs US-01; US-03 needs US-05; US-09 extends US-06; composition needs authorized entry; US-20 needs US-19; US-21/22 need US-20; US-23 requires private-source capability; US-24 attaches existing material; US-25/26 extend US-24; US-27 consumes US-28 or an agent Proposal; US-29 can bypass Proposals; US-30 needs US-29; US-31 needs a current Decision/connection; US-32/33 extend US-31. US-34/35 expand as their owned records are introduced.

Human Decisions may start with Notes alone; provider evidence extends the same pinning contract later. Assistance consumes only its required authoring, attachment and published Proposal interfaces, without waiting for every drawing tool. Reviewed Actions can originate from a human Decision without a Run. Invitations, private publication, full Activity UI and templates complete in their scheduled cuts while existing authority, attribution, persistence and failure rules apply from each workflow's first use. See the release plan for hard prerequisites, scheduling preferences and scope transfers between slices. Experience remains the R/S authority until a separately authorized capability migration.

## 4. Story catalogue, requirements and scenarios

Each story has value, a normative observable requirement and concrete passing/failure examples. Scenario IDs are stable trace anchors, not assertions that tests have passed. Shared access/input contracts in section 1 also apply. Domain command IDs resolve in the Domain Specification; AC IDs resolve in Technical section 8.

### US-01 - Sign in and end a session

**Story:** As an account holder, I want to enter and later leave my permitted work using my real identity. **Requirement R-01:** Memento SHALL establish a server-authenticated session only after valid credentials, honor the selected persistence policy and revoke it on sign-out. **Trace:** UJ-08; WA-01; DR-01/DR-15; AC-22.

**S-01a:** Given a provisioned active account, when valid credentials are submitted, then Home lists only its authorized workspaces and protected return URLs are re-authorized. **S-01b:** Given an invalid, expired or signed-out session, when a protected route/API/subscription is requested, then protected content is denied and the UI offers Access without losing a safe local draft.

### US-02 - Recover account access

**Story:** As an account holder who cannot sign in, I want a controlled way to recover my credentials. **Requirement R-02:** Memento SHALL redeem a valid single-use recovery capability to replace the account credential and revoke its prior sessions. **Trace:** UJ-08; WA-02; DR-01; AC-22.

**S-02a:** Given the account's verified recovery address, when recovery is requested and the delivered token is redeemed, then a replacement credential works and old sessions fail. **S-02b:** Given an unknown address or expired/used token, when requested/redeemed, then the request discloses no account existence and redemption grants no access; failures are understandable.

### US-03 - Join or re-enter an invited workspace

**Story:** As an invitee, I want to join as the named participant the team will recognize without needing a global account. **Requirement R-03:** Memento SHALL redeem a valid invitation into only its specified Editor Membership and scoped identity/session. **Trace:** UJ-09; WA-05; DR-15/DR-16; AC-14/AC-22.

**S-03a:** Given a valid invitation, when its recipient supplies a display name and redeems it, then the invited workspace opens with attributable Editor access; an Owner-issued bound re-entry link resumes the existing guest identity. **S-03b:** Given a revoked, expired, consumed or recipient-mismatched token, when redeemed or used for another workspace, then no new authority is granted; a duplicate successful operation returns its original result.

### US-04 - Manage members and ownership

**Story:** As an Owner, I want the membership list and authority changes to reflect who may participate. **Requirement R-04:** Memento SHALL apply authorized member removal/ownership changes without leaving a workspace with no active Owner. **Trace:** UJ-09; WA-06/WA-Q2; DR-01/DR-16; AC-14/AC-22.

**S-04a:** Given another active member, when an Owner transfers ownership or revokes their Membership, then the committed roles/access and open subscriptions reflect the change while past contributions remain attributed. **S-04b:** Given an Editor or a sole Owner trying to remove/demote the last Owner, when the change is requested, then it is denied; changing a client role selector grants nothing.

### US-05 - Issue and revoke invitations

**Story:** As an Owner, I want to invite participants with a reviewable scope and revoke unused access. **Requirement R-05:** Memento SHALL create and revoke scoped, expiring single-use Editor invitations under Owner authority. **Trace:** UJ-09; WA-04; DR-15; AC-22.

**S-05a:** Given Owner access, when a participant invitation is created, then the UI shows workspace/role/expiry, provides its link once, and lists pending invitation metadata. **S-05b:** Given a revoked invitation or an Editor requesting issuance, when redemption/issuance is attempted, then access is denied and no token secret appears in Activity.

### US-06 - Create a workspace and first board

**Story:** As an account holder, I want a named private starting place in which I can begin immediately. **Requirement R-06:** Memento SHALL create the Workspace, initial Owner Membership and empty first Board as one recoverable operation. **Trace:** UJ-10; WA-03/VC-01; DR-17; AC-23.

**S-06a:** Given an active account, when a valid name and initial-access choice are submitted, then its empty Starter board opens and survives reload/restart; both access choices initially admit only the creator. **S-06b:** Given validation, storage failure or a repeated operation, when creation is attempted/retried, then the UI preserves input, avoids partial false success and never creates duplicate workspaces.

### US-07 - Find and open permitted work

**Story:** As a member, I want to find the right workspace/board and recognize its actual content. **Requirement R-07:** Memento SHALL return authorized owned/shared workspace and board catalogues with meaningful search, metadata and composition previews. **Trace:** UJ-10; WA-Q2/VC-Q1; DR-01; AC-14/AC-23.

**S-07a:** Given owned and invited workspaces, when filters/search are applied and a listed Board is opened, then counts/previews match accessible records and the selected board's own content appears. **S-07b:** Given no matches, an empty board or a forbidden direct ID, when queried/opened, then the UI shows the appropriate empty/unavailable/access result without fabricating a preview or disclosing restricted metadata.

### US-08 - Keep personal return markers

**Story:** As a returning member, I want favorites, recency and updates to help me resume work. **Requirement R-08:** Memento SHALL retain favorite/visit/read-navigation markers independently for each member and accessible board. **Trace:** UJ-10; VC-07; DR-23; AC-23.

**S-08a:** Given a favorite and an older visit watermark, when Home is reopened, then Favorites/Recent and updates-since-visit reflect that member's state; opening the snapshot advances only the displayed watermark. **S-08b:** Given another member's markers or a new event during snapshot loading, when one member visits, then the other member's state and the newer unseen event are not falsely marked seen.

### US-09 - Create from a built-in template

**Story:** As an account holder, I want a useful editable starting structure instead of arranging every prompt myself. **Requirement R-09:** Memento SHALL instantiate the selected versioned template's defined structure into a new workspace and first Board using fresh scoped identities. **Trace:** UJ-11; WA-03/VC-01/VC-Q2; DR-17; AC-24.

**S-09a:** Given a chosen template/category/search result, when creation is confirmed, then its frames/prompts/visual links match section 5 and remain editable independently of the template. **S-09b:** Given cancellation, repeated submission or a later template revision, when the flow is cancelled/replayed/the catalogue changes, then no unintended duplicate instance or retrospective content change occurs and no real Decision is pre-recorded.

### US-10 - Write a freeform Note

**Story:** As a member, I want to capture an idea directly on the canvas without a formal title form. **Requirement R-10:** Memento SHALL create and edit freeform Notes inline, preserving authored text and independent visual colour. **Trace:** UJ-01; CK-01/VC-02/VC-03; DR-02/DR-03; AC-26.

**S-10a:** Given Add note and a selected palette colour, when the canvas is activated, then a Note is placed and accepts multiline typing immediately; outside-click/Ctrl-or-Cmd+Enter saves one content revision and derives a reference label. **S-10b:** Given an active edit, when Escape is pressed or colour alone is changed, then respectively the draft is cancelled or only presentation changes; no content revision/Decision is invented.

### US-11 - Preserve concurrent contributions

**Story:** As an editor, I want incompatible concurrent changes explained without losing accepted work or my draft. **Requirement R-11:** Memento SHALL reject stale incompatible content edits and retain the accepted revision and recoverable draft for explicit resolution. **Trace:** UJ-01/UJ-07; CK-01; DR-12; AC-03.

**S-11a:** Given two sessions editing one Note revision, when both save incompatible text, then one accepts and the other sees current content plus its draft; applying the reviewed draft creates a new revision. **S-11b:** Given disconnect/reconnect or a duplicate command, when recovery completes, then accepted text remains once, pending state is reconciled and no silent overwrite occurs.

### US-12 - Arrange and place material

**Story:** As a member, I want to arrange the board without changing what its records mean. **Requirement R-12:** Memento SHALL persist authorized placement/movement/removal independently of represented record content. **Trace:** UJ-01/UJ-06; VC-02/VC-03; DR-03/DR-11; AC-20.

**S-12a:** Given a record, when it is deliberately placed/moved/removed, then arrangement changes propagate and survive restart while content/history stay unchanged. **S-12b:** Given an unplaced cited record, when it is inspected, then evidence still resolves; Locate is unavailable until an explicit Show on board operation restores a Placement.

### US-13 - Organize with Frames

**Story:** As a member, I want named groups that help the team read the composition. **Requirement R-13:** Memento SHALL maintain explicit Frame membership and revision-aware frame naming, sizing and member arrangement. **Trace:** UJ-01; VC-03; DR-03/DR-12; AC-26.

**S-13a:** Given an explicitly grouped set and loose cards, when its Frame is moved/resized/renamed, then only intended members move and loose cards remain independent. **S-13b:** Given a conflicting member placement revision, when a frame move batch commits, then it conflicts atomically rather than partially overwriting another member's accepted arrangement.

### US-14 - Draw and erase ink

**Story:** As a participant, I want to sketch on the shared board without creating business commitments. **Requirement R-14:** Memento SHALL create attributable board-space ink strokes and erase selected ink through composition operations. **Trace:** UJ-01; VC-04; DR-03; AC-26.

**S-14a:** Given Pen at different zoom/pan values, when a stroke is drawn and saved, then it remains attached to board coordinates across viewing/reload and appears to other sessions. **S-14b:** Given Erase over an ink stroke or card, when used, then only a hit stroke is removed; cards, Decisions and external records are not erased.

### US-15 - Add visual text and shapes

**Story:** As a participant, I want labels and shapes to explain a composition beyond sticky notes. **Requirement R-15:** Memento SHALL create/edit/place supported Text and Shape visual elements with durable identity and attribution. **Trace:** UJ-01; VC-04; DR-03/DR-12; AC-26.

**S-15a:** Given Text/Shape and a canvas position, when material is created/edited/moved, then its accepted content/geometry reopen and remain distinguishable from typed knowledge records. **S-15b:** Given incompatible edits or invalid geometry, when submitted, then accepted state is preserved and a clear conflict/validation result appears; no lifecycle badge or semantic assertion is invented.

### US-16 - Draw a visual connection

**Story:** As a member, I want an explanatory arrow that carries no implied evidential meaning. **Requirement R-16:** Memento SHALL create handle-drawn links as visual composition records distinct from semantic Relationships. **Trace:** UJ-01; VC-05; DR-24; AC-26.

**S-16a:** Given two placed items, when a pointer or keyboard connection is completed, then the arrow stays attached while endpoints move and persists as a visual link. **S-16b:** Given self/duplicate endpoints, cancellation or removed placement, when creation/removal occurs, then invalid links are rejected/cancelled and no supports, supersedes or Action prerequisite is asserted.

### US-17 - Express semantic connections

**Story:** As a member, I want explicit relationships to explain evidence, origin or dependencies. **Requirement R-17:** Memento SHALL establish only valid typed/directed semantic Relationships through the owning contextual command. **Trace:** UJ-01/UJ-04/UJ-05; CK-06/CW-06; DR-01/DR-24; AC-19/AC-20.

**S-17a:** Given permitted same-workspace records, when an evidence/origin command or Action prerequisite is recorded, then Traceability shows its correct direction and related versions. **S-17b:** Given cross-workspace endpoints, prohibited types or an execution-dependency cycle, when requested, then the command fails without fabricating a relationship or changing approval/execution implicitly.

### US-18 - Undo visual work safely

**Story:** As a member, I want to reverse an arrangement mistake while retaining team knowledge. **Requirement R-18:** Memento SHALL treat shared visual undo/redo as new revision-checked composition operations. **Trace:** UJ-01/UJ-07; VC-06; DR-11/DR-12; AC-20/AC-26.

**S-18a:** Given a committed arrangement/colour/visual-link operation, when undone/redone, then the applicable composition changes and other sessions receive the new accepted revision. **S-18b:** Given intervening incompatible work or a succeeded Action, when undo is invoked, then it conflicts where necessary and never reverses a Decision, Approval, delivery or provider task operation.

### US-19 - Configure a scoped service

**Story:** As an Owner, I want members to understand which external account/resources and capabilities are available. **Requirement R-19:** Memento SHALL allow only Owners to configure workspace connection bindings and separately display read/write availability and operation health. **Trace:** UJ-02; CW-01; DR-01/DR-09; AC-14.

**S-19a:** Given Owner authority, when valid provider/account/resource configuration is saved, then Sources shows its scope/capabilities and server-held authorization reference without exposing secrets. **S-19b:** Given an Editor or a changed binding, when configuration/refresh/approval is attempted, then unauthorized management fails and old captures are not retargeted; affected unsubmitted approvals require review.

### US-20 - Browse and import evidence

**Story:** As a member, I want to deliberately bring the relevant external message/task into our discussion. **Requirement R-20:** Memento SHALL capture only selected permitted external records on explicit import, retaining their identity, captured content and provenance. **Trace:** UJ-02; CW-02/VC-02; DR-04/DR-05; AC-05.

**S-20a:** Given a real scoped mailbox/task collection, when a member browses, selects and imports a record, then its card shows the chosen capture, provider identity and capture time. **S-20b:** Given duplicate import, a new provider arrival or read failure, when respectively retried/arriving/failing, then existing material is reused where applicable, no arrival auto-places and failure creates no false capture.

### US-21 - Refresh an addressed capture

**Story:** As a member, I want to deliberately check whether a particular captured reference has changed. **Requirement R-21:** Memento SHALL refresh only explicitly addressed source references while preserving earlier observations and exact cited evidence. **Trace:** UJ-02/UJ-05/UJ-07; CW-03; DR-18/DR-19; AC-06/AC-10/AC-12.

**S-21a:** Given an imported task later completed externally, when the board is revisited/reconnected, then the card remains at its captured status; when Refresh source is requested, a new captured completion may advance that reference and old Decision evidence remains unchanged. **S-21b:** Given lost scope/provider availability or a concurrent placement change, when refresh completes/fails, then prior content/annotations/layout are preserved and unavailable/conflicted advancement is explained.

### US-22 - Inspect the original and observation history

**Story:** As a member, I want to distinguish the source's live native record from our captured evidence. **Requirement R-22:** Memento SHALL expose original navigation and immutable observation history as distinct operations from refresh. **Trace:** UJ-02/UJ-06; CW-Q1/CK-Q1; DR-05; AC-06/AC-20.

**S-22a:** Given a captured message/task, when its history or original is opened, then history resolves exact captures and original navigation uses the external viewer's own permission without updating Memento. **S-22b:** Given missing provider access or a removed native record, when original navigation fails, then the captured evidence remains readable and no borrowed owner session is supplied.

### US-23 - Publish selected personal message text

**Story:** As a member with a private message, I want to share only the evidence I deliberately choose. **Requirement R-23:** Memento SHALL publish a private-source capture only after its owner confirms exact selected text/provenance and workspace audience. **Trace:** UJ-13; CW-04; DR-20; AC-29.

**S-23a:** Given an authorized private selection and membership, when the publisher confirms the preview, then another member can read the published immutable text and provenance, excluding attachments/unselected messages. **S-23b:** Given cancellation or a recipient requesting private browse/refresh/send-as, when attempted, then no publication/private capability is granted; opening the original requires that viewer's separate authorization.

### US-24 - Ask with explicit attached context

**Story:** As a member, I want an agent to understand the request and the exact material I chose. **Requirement R-24:** Memento SHALL record a board-shared message and bounded Run with the explicitly submitted role/context. **Trace:** UJ-03; AP-01; DR-13/DR-21; AC-07/AC-27.

**S-24a:** Given selected records, when they are attached and a message is sent, then chips/run details resolve the submitted versions and actual supplied context; other members can inspect the shared exchange. **S-24b:** Given later selection changes, no attachments or another active Run, when sending, then submitted context is not replaced, empty context may prompt clarification and a busy response preserves the draft.

### US-25 - Continue a clarification

**Story:** As the requesting member, I want to answer an agent's question without starting an unrelated run. **Requirement R-25:** Memento SHALL resume an authorized waiting Run from a recorded clarification while retaining its pinned context and remaining limits. **Trace:** UJ-03/UJ-07; AP-02; DR-13/DR-21; AC-27.

**S-25a:** Given a waiting unexpired Run, when its requester replies, then the reply is recorded and the same Run resumes with its original references. **S-25b:** Given another requester, expired wait or revoked membership, when continuation is requested, then it is denied/terminated visibly and no extra tool authority or fresh provider read is implied.

### US-26 - Understand and stop agent work

**Story:** As a requester or Owner, I want to see useful progress and stop work within known limits. **Requirement R-26:** Memento SHALL expose run progress/outcome and enforce cancellation, active-time and tool-call bounds server-side. **Trace:** UJ-03/UJ-07; AP-03/AP-Q1; DR-13; AC-07/AC-16.

**S-26a:** Given an active Run, when cancelled or a bound is exhausted, then further tool admissions stop and the Run reports its terminal state with prior contributions retained. **S-26b:** Given an Action already submitted before cancellation or invalid generated references, when the Run ends, then the Action's eventual/uncertain outcome stays tracked and invalid evidence is not persisted as valid output.

### US-27 - Review a Proposal without deciding

**Story:** As a member, I want to adopt useful working material or reject it without creating a team commitment. **Requirement R-27:** Memento SHALL review the exact applicable Proposal version and keep adoption distinct from Decision recording and placement. **Trace:** UJ-03/UJ-04; CK-03; DR-06/DR-07; AC-19.

**S-27a:** Given a current pending Proposal, when adopted, then a separately editable Note records the adopted version/origin and stays unplaced unless requested; no Decision is created. **S-27b:** Given a stale/rejected/adopted Proposal, when adoption is attempted, then the applicable state rule blocks it or requests reassessment; rejecting a stale suggestion does not rewrite its evidence.

### US-28 - Submit a human Proposal

**Story:** As a member, I want to express an option independently of AI and leave our original Notes unchanged. **Requirement R-28:** Memento SHALL create a separate human Proposal with chosen content and pinned selected context. **Trace:** UJ-03/UJ-04; CK-02; DR-02/DR-06; AC-19.

**S-28a:** Given selected human/source material, when Submit Proposal from context is confirmed, then a pending Proposal retains its exact references and is shown on the board only if selected. **S-28b:** Given invalid content or a changed expected revision during edit, when saved, then the input/conflict is explained and the original Notes remain editable and unchanged.

### US-29 - Record a supported Decision directly

**Story:** As a member, I want to state our chosen direction and preserve why we chose it. **Requirement R-29:** Memento SHALL record a human Decision only from an explicit statement, rationale and valid pinned evidence. **Trace:** UJ-04; CK-04; DR-05/DR-06; AC-19.

**S-29a:** Given Notes or captured evidence without a Proposal, when Record Decision is confirmed, then a new current Decision identifies recorder/reason/revisions and optional placement. **S-29b:** Given missing rationale/evidence or an agent attempting to record the Decision, when submitted, then it is rejected; moving/adopting material alone creates no Decision.

### US-30 - Replace one Decision and preserve history

**Story:** As a member, I want changed direction to retain the earlier reasoning and unrelated choices. **Requirement R-30:** Memento SHALL supersede an identified current Decision through a new human-recorded Decision and acyclic preserved history. **Trace:** UJ-04/UJ-06; CK-05; DR-08; AC-19.

**S-30a:** Given current time and launch-date Decisions, when a new time Decision supersedes its predecessor, then both time versions/evidence remain inspectable and the date Decision stays current. **S-30b:** Given an already superseded predecessor or a cycle/concurrent successor, when replacement is attempted, then the stale/invalid operation is rejected without rewriting accepted meaning.

### US-31 - Prepare exact follow-up work

**Story:** As a member, I want a task or reply draft that clearly expresses what would happen externally. **Requirement R-31:** Memento SHALL prepare task/reply Actions with complete reviewable content, target, current Decision and explicit prerequisites. **Trace:** UJ-05; CW-05/CW-06; DR-09/DR-24; AC-08.

**S-31a:** Given a current Decision and allowed Connection, when follow-up is prepared, then review shows task assignee/due/content/project or reply sender/recipients/subject/body/thread, without submitting. **S-31b:** Given missing capability, stale Decision or a cyclic prerequisite, when saved, then the draft is rejected or renewed review is required and no provider write happens.

### US-32 - Approve and execute the reviewed version

**Story:** As an authorized member, I want only the precise operation I reviewed to run. **Requirement R-32:** Memento SHALL dispatch an Action only after exact human Approval and an explicit execute request pass current authority/version/prerequisite checks. **Trace:** UJ-05; CW-07/CW-08; DR-09; AC-08/AC-09/AC-11.

**S-32a:** Given a valid reviewed version with succeeded prerequisites, when approved then explicitly executed, then one corresponding real task/reply is submitted and its identity/outcome is recorded. **S-32b:** Given changed payload/binding/authority/Decision or an agent-supplied approval, when dispatch is requested, then no operation is admitted and the UI requests the required fresh review.

### US-33 - Recover an uncertain external outcome

**Story:** As a member, I want to establish what happened before risking a repeated task or message. **Requirement R-33:** Memento SHALL retain submission uncertainty until provider evidence establishes success or confirmed absence permits renewed review. **Trace:** UJ-05/UJ-07; CW-09; DR-10; AC-12/AC-13.

**S-33a:** Given an interrupted response after submission, when inspected/reconciled, then the original attempt/result is sought and a found native record establishes success without resubmission. **S-33b:** Given lease expiry, inconclusive evidence or confirmed absence, when recovery runs, then respectively no automatic repeat occurs, uncertainty remains, or Approval is cleared before a newly reviewed attempt.

### US-34 - Search and inspect retained meaning

**Story:** As a returning member, I want to recover an outcome's history even when its cards were removed. **Requirement R-34:** Memento SHALL search and traverse authorized retained records, exact evidence and history independently of their current board placement. **Trace:** UJ-06; CK-Q1/CK-Q2/CW-Q1; DR-05/DR-22; AC-20.

**S-34a:** Given unplaced source/Note/Decision material, when text search or View traceability is used, then current details, pinned versions, supersession and resulting Actions are navigable without restoring placements. **S-34b:** Given a missing placement or unrelated workspace, when Locate/search is requested, then Show on board remains explicit and unauthorized evidence is absent.

### US-35 - Follow scoped Activity

**Story:** As a member, I want to understand changes and navigate back to their actual context. **Requirement R-35:** Memento SHALL provide authorized global/board Activity with category/search filters, personal read markers and context links to committed events. **Trace:** UJ-12/UJ-06; event projections; DR-23/DR-26; AC-25.

**S-35a:** Given events in several accessible workspaces/boards, when opening global or board Activity/bell, then the corresponding scoped events/counts and real context links appear with actor/time/outcome. **S-35b:** Given Mark all as read or a restricted/unavailable context, when used, then only the person's scoped read state changes and navigation grants no extra permission, Approval or source refresh.

### US-36 - Navigate, inspect and use both themes

**Story:** As a participant using different inputs/screen sizes, I want the workspace controls and details to remain usable. **Requirement R-36:** Memento SHALL preserve the canonical shell, themes, spatial controls and accessible contextual interactions across declared target viewports/inputs. **Trace:** All journeys; Visual Collaboration; DR-03; AC-26/AC-30.

**S-36a:** Given dark/light appearance, different zooms and collapsed navigation/context panel, when editing/inspecting, then controls remain readable, information is distinct from commands, and pan/drag does not consume typing or button activation. **S-36b:** Given keyboard focus, Shift+F10/context menu or a modal, when navigating/cancelling, then actions are reachable, Escape/focus-return works and no accidental record mutation occurs.

## 5. Built-in template definitions

Templates create editable Notes and visual structure only. Prompts are ordinary text; evidence placeholders are not Source Observations and a box labelled Decision is not a Decision record. All six contain a named Board, three Frames, the listed prompt Notes and visual links from the first group through the second to the third. No native provider identities, credentials, actual Action or human Approval are instantiated.

| Stable template ID / name | Frames | Prompt Notes (one per semicolon-delimited item) |
| --- | --- | --- |
| decision-review / Decision review | Evidence; Alternatives; Direction and next steps | What information supports this choice?; Which constraints are fixed?; Which options remain?; What would each option change?; What direction should we record?; Who should prepare follow-up? |
| research-synthesis / Research synthesis | Sources; Findings; Open questions | Which observation should we capture?; What did the participant report?; What pattern do we see?; Which evidence supports it?; What is still unknown?; What should we investigate next? |
| project-kickoff / Project kickoff | Goal; Context and constraints; First steps | What outcome are we seeking?; Who needs to participate?; Which constraints apply?; What evidence do we need?; What is the first useful action?; Who will prepare it and by when? |
| dependency-map / Dependency map | Inputs; Dependencies; Outcomes | What must be available first?; Which source supports that assumption?; Which work depends on it?; Where might a cycle or blocker exist?; What becomes possible next?; Who should confirm the prerequisite? |
| quick-retrospective / Quick retrospective | Signals; Patterns; Agreed follow-up | What helped?; What obstructed us?; Which repeated pattern matters?; Which evidence should we retain?; What change should we decide?; What follow-up should we prepare? |
| open-questions / Open questions | Known context; Missing knowledge; Resolution | What do we know?; Which source can we capture?; What remains uncertain?; Why does it matter?; Who can resolve it?; What should we check next? |

Template version 1 uses three side-by-side readable Frames, two Notes in each, and explanatory visual links; users may rearrange everything. Search matches name/description/category; categories are Decisions, Research, Planning, Diagrams, Meetings and Ideation, plus All. Preview generation uses the same template structure and theme tokens as the created composition.

## 6. Screen and interaction contract

| Surface | Required elements and distinctions |
| --- | --- |
| Access | Split welcome and account/invitation panels; themed illustration, labelled forms, stay-signed-in, recovery, demo entry and scope notice. Recovery/provisioning controls do not grant workspace membership. Decorative board is hidden where narrow layout would obscure forms. |
| Home | Shared top bar/navigation/profile; owned/shared counts/search; workspace cards and actual schematic board previews; Recent/Favorites, star and update markers; one New workspace action and starting strip for Templates. No standalone Create board control. |
| Templates | Six definitions, category/search controls, preview/description and Use template; creation modal names new workspace/initial private access and confirms chosen structure. |
| Activity | Chronological committed records, categories/counts/search, personal unread state and context actions. Board context is explicit; opening a full page retains it. |
| Board toolbar | Board title, visual undo/redo, fit, expandable inline search, presence, Members, panel toggle. Search does not create a nested competing toolbar or alter content. |
| Tool rail and viewport | Select, Pan, Pen, Erase, Note, Frame, Text, Shape and Agents shortcut; board-space dotted canvas, mixed Frames/loose cards, attached directional links and zoom controls/percentage. No forced stage columns. |
| Notes and cards | Freeform inline Notes with five presentation colours and no lifecycle badge. Source, Proposal, Decision and Action types retain text labels/revisions/actor; superseded Decisions remain readable with a dashed border. |
| Context panel | Fixed consistent desktop width for Inspector, Sources, Agents and Activity, one selected tab; collapse/expand independent of navigation. Information cards and bordered command buttons are visually distinct. |
| Inspector and Traceability | Type/current state, content, author/revision, exact evidence and connected records; contextual Proposal/Decision/action commands; scrollable informational Traceability with details/history and explicit Locate versus Show. Retained-material search is available from Traceability, including when no card is selected. |
| Agents | Board-shared messages, role avatar, human bubbles, explicit context chips, composer/attachment/role/send controls, latest Run status/progress/cancel and secondary Details; prior Runs remain accessible. Enter sends, Shift+Enter inserts newline. No required runtime tuning in the composer. |
| Sources | Workspace Connections, original binding/scope/capabilities and operation status; explicit browse/import/refresh/history/original. Owner configuration and personal capture preview/publication respect separate authorization. No automatic-arrival setting. |
| Dialogs and feedback | Compact viewport-bounded modal with fixed header/actions and scrolling body; labelled fields, inline errors, Cancel/Save or exact operation action; status/toasts distinguish pending and committed results. Dialog controls remain readable at any board zoom. |
| Themes | Shared product green #71efb2 for primary accents with readable text/focus; paper colours independent of type/semantic status; light layered surfaces and dark high-contrast surfaces; focus/hover/disabled states, text labels and reduced-motion support. |

Right-click, card action button and Shift+F10 open the same contextual Board actions. Empty-canvas invocation clears unrelated prior selection. Arrow/Home/End navigation and Escape work in the menu. New Note placement immediately edits inline; Enter is a newline, Ctrl/Cmd+Enter or focus-out saves, Escape cancels. Selecting cards does not change the chosen context tab; explicit Inspect may open Inspector. Colour selection changes presentation only.

Play/Pause/Next/Restart, role preview, simulated conflicts/arrivals/outages and chosen provider outcomes are optional isolated demo tools. They are not production authority or proof of real AI/connector behaviour. Explore demo uses an explicitly labelled isolated demo workspace/profile; it cannot bypass normal access to other work. Restart resets only its disclosed demo data. Mockup playback copy describing auto-sync must be updated before it is reused as a product walkthrough.

## 7. Atlas Launch acceptance journey

1. Provision Maya and Leo, open real service clients, and prepare the scoped mailbox/task collection. Both sign in separately and open Atlas Launch from Home.
2. Inspect the initial 10:00 Decision, launch-date Note and captured task. Send the stakeholder's real message through the supplied mail client.
3. Browse the project mailbox and explicitly import the message. Verify it did not auto-appear merely on arrival.
4. Collaborate on Notes/Frames, preserving accepted contributions and attribution. Attach the displayed message, constraint and earlier Decision to a Facilitator message.
5. Use real inference; inspect pinned context, respond to clarification if necessary, and review the resulting Proposal. Adoption alone creates no Decision.
6. Record the human 11:00 Decision with rationale/evidence, superseding the 10:00 Decision while keeping the date choice current.
7. Prepare a task for Leo, inspect exact target/assignee/due/content, approve and explicitly execute. Verify the actual result in Deck and its separate captured Outcome.
8. Prepare/review/approve/execute a reply; verify delivery and sender/recipients/content in the real client.
9. Complete the task in Deck. Verify Memento still shows the captured earlier status after revisit/reconnect. Explicitly refresh and inspect the new completion plus old pinned evidence.
10. Restart using retained storage and recover the request, both Decisions and outcomes through search/Traceability, including a removed Placement.

Run the journey twice from its documented reset state, then repeat with one member. Separately exercise actual service outage, failed refresh, post-submission uncertainty, cancellation and concurrent edit recovery. Technical timing targets and actual evidence belong to their AC gates and delivery records.

## 8. Traceability and readiness

Every R-01 through R-36 belongs to its corresponding US and has scenarios a/b above. Journey membership and command/rule/AC references form the trace to model and verification. All five contexts and CM-01 through CM-26 are covered by the Product/Domain/Technical/Platform set and the comparison resolution ledger. No story is implemented merely because it is scheduled in the map.

Before an implementation change starts, resolve any ambiguity that affects that story's actor, permission, state transition, evidence identity or observable outcome. Model/provider SDK choice may remain a design decision under the fixed constraints. A later OpenSpec capability may contain several stories; a story may affect more than one capability. Migrate an adopted requirement/scenario block explicitly and replace its local definition with a reference, preserving IDs and coverage rather than creating a competing copy.
