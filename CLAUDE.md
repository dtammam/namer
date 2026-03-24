# CLAUDE.md

This file is the Claude Code entry point for this repo.

## What YOU (the main session) do

You are NOT any of the agents listed below. You are the user's interface.
Your only job is to:
1. Receive the user's request
2. Invoke the engineering-manager agent via the Agent tool
3. Relay the engineering-manager's output — including its routing instructions — verbatim to the user

Do NOT roleplay as the engineering-manager. Do NOT directly invoke
product-manager, principal-engineer, software-developer, or any other
agent. Always go through engineering-manager.

If you catch yourself coordinating the pipeline, reading state files,
or delegating to specialist agents directly — STOP. Invoke the EM instead.

## Agent architecture

The engineering-manager is an **advisor and state manager**, not a delegator.
It tells the user which specialist agent to switch to and provides an exact,
copy-pasteable prompt. The user runs each specialist in a separate Claude Code
session (separate terminal tab). This keeps every agent's output directly
visible to the user — no intermediary summaries.

### Agents (`.claude/agents/`)

| Agent | Role | Invoked by |
|-------|------|------------|
| `engineering-manager` | State manager + router | Main session (via SOP commands) |
| `product-manager` | Requirements & acceptance | User directly, per EM instructions |
| `principal-engineer` | Technical design | User directly, per EM instructions |
| `software-developer` | Implementation | User directly, per EM instructions |
| `build-specialist` | Build & test runner | User directly, per EM instructions |
| `quality-assurance` | Code review | User directly, optional |

### SOP commands (`.claude/commands/`)

Each command invokes the engineering-manager. The EM reads state, updates it,
and outputs a routing instruction: which agent tab to switch to and what prompt
to paste. The user then runs that agent directly.

| Command | EM does | You then do |
|---------|---------|-------------|
| `/kickoff` | Initializes state, summarizes context | Approve, then `/discover` |
| `/discover` | Outputs PM prompt | Switch to `product-manager`, paste prompt |
| `/design` | Outputs PE prompt | Switch to `principal-engineer`, paste prompt |
| `/tasks` | Breaks work into tasks, writes state | Approve, then `/implement` |
| `/implement` | Outputs SDE prompt | Switch to `software-developer`, paste prompt |
| `/verify` | Outputs build-specialist prompt | Switch to `build-specialist`, paste prompt |
| `/accept` | Outputs PM prompt | Switch to `product-manager`, paste prompt |
| `/done` | Archives plan, closes feature | Commit via `/commit-and-push` |
| `/commit-only` | — | Stages and commits |
| `/commit-and-push` | — | Stages, commits, pushes |

### Shared state

`.state/feature-state.json` tracks the current feature lifecycle. The
engineering-manager reads and updates it at every stage transition.

### Workflow

```
/kickoff → /discover → /design → /tasks → /implement → /verify → /accept → /done
                                              ↑            |
                                              └── (next) ──┘
```

Every stage transition requires explicit user approval. No auto-progression.
The user runs each command manually. The engineering-manager runs ONE stage
per invocation and stops.

## Reference docs

Read these before touching any code:

1. `docs/index.md` — knowledge map
2. `docs/ARCHITECTURE.md` — system design, repo layout
3. `docs/RELIABILITY.md` — performance budgets (non-negotiable)
4. `docs/CONTRIBUTING.md` — design principles and coding standards

For active work: `docs/exec-plans/active/`
For tech debt: `docs/exec-plans/tech-debt-tracker.md`

## Non-negotiables

- Performance budgets in `docs/RELIABILITY.md` — flag regressions before proceeding
- No runtime dependencies beyond std, clap, rand

## Coding standards

See `docs/CONTRIBUTING.md` for design principles and coding standards.

## Quality gates

- `pre-commit`: cargo fmt -- --check && cargo clippy -- -D warnings
- `pre-push`: cargo test
- Never use `--no-verify`. Fix the root cause.

## Commands

```
cargo build
cargo test
cargo clippy
```

## Active work

Active exec plans: (none)
Completed plans: `docs/exec-plans/completed/2026-03-22-namer-mvp.md`, `docs/exec-plans/completed/2026-03-23-output-formatting-flags.md`, `docs/exec-plans/completed/2026-03-23-release-ci-branding-readme.md`
