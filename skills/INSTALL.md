# Local agent-host packaging

Package contract: knowledgehub-authoring/1, catalogue-3, record/3 and request/3.
Both skill directories are self-contained. Their bundled tool references are
derived from maintained design/model-authoring.md and logical-design-contract.md;
refresh them when those current contracts change. No generated OpenSpec skills
or user configuration is replaced.

Observed installed binaries on 3 November 2026: Codex CLI 0.166.0, Claude Code
2.1.316 and GitHub Copilot CLI 1.0.105. A version/help observation is not an
end-to-end host pass. Actual import/verify results belong in delivery evidence.
Cloud/web modes without local Rust execution are outside this package's tested
execution boundary.

The recorded 3 November runs exercised actual import and a separate verify
process in Codex CLI and GitHub Copilot CLI. Those hosts reviewed/replayed a
maintained equipment plan. A separate source-only agent forward test authored
its own plan and expectations. These are distinct forms of evidence; see the
[host and model results](../delivery/evidence/mm-sl-01/20261103-apply/README.md).
Claude's local run stopped at an unavailable login and remains an unexecuted gate.

The skills guide source interpretation, typed planning, native publication and
independent verification. A `verified` source report can describe a valid,
incomplete model with explicit structured gaps. Further formalization and human
meaning acceptance remain separately reported; source capture and code artifacts
do not imply target application execution. The recorded native models are separate
from the preserved historical root Hub and from future Modelarium registration.

Install into a fresh project-scoped directory in the chosen local host:

| Host/mode | Project skill location | Invocation |
| --- | --- | --- |
| Codex local CLI | .agents/skills/knowledgehub-import and knowledgehub-verify | Ask to use `$knowledgehub-import`, then `$knowledgehub-verify` |
| Claude Code local | .claude/skills/knowledgehub-import and knowledgehub-verify | `/knowledgehub-import`, then `/knowledgehub-verify` |
| GitHub Copilot local CLI | .github/skills/knowledgehub-import and knowledgehub-verify | Ask to use knowledgehub-import, then knowledgehub-verify; inspect `/skills` |

Copy each complete skill directory only if its destination does not exist.
Inspect installation/discovery in a new session. The host needs the current
knowledgehub-model executable, explicit source/output paths and trusted current
host actor/grants. Local tool authorization follows that host's normal policy;
this package contains no permission-bypass flags, provider SDKs or credentials.

For noninteractive acceptance, the observed binaries expose `codex exec`,
`claude --print` and `copilot --prompt`. Use an isolated writable workspace,
normal sandbox/approval controls and an explicit small input. Record whether the
agent actually authored/reviewed a plan or merely replayed an existing one.
Unavailable login, local execution or denied tools remain failed/unexecuted gates.

Discovery paths are checked against the [official Codex skill documentation](https://learn.chatgpt.com/docs/build-skills),
[Claude Code skills guide](https://code.claude.com/docs/en/skills), and
[GitHub Copilot CLI skills guide](https://docs.github.com/en/copilot/how-tos/copilot-cli/customize-copilot/add-skills).
These document host setup; they do not establish application or agent acceptance.
