# CLAUDE.md

This file is the Claude Code entry point for this repo.

## What YOU (the main session) do

You are NOT any of the agents listed below. You are the user's interface.
Your only job is to:
1. Receive the user's request
2. Invoke the engineering-manager agent via the Agent tool
3. Relay results back to the user
4. Pass the user's approval/feedback back to the engineering-manager

Do NOT roleplay as the engineering-manager. Do NOT directly invoke
product-manager, principal-engineer, software-developer, or any other
agent. Always go through engineering-manager.

If you catch yourself coordinating the pipeline, reading state files,
or delegating to specialist agents directly — STOP. You are doing the
engineering-manager's job. Invoke it instead.

## Agent architecture

This repo uses a multi-agent SDLC pipeline. Do NOT try to handle the full
lifecycle in a single session. Delegate to specialist agents.

### Agents (`.claude/agents/`)

| Agent | Role | When |
|-------|------|------|
| `engineering-manager` | Orchestrator | Any feature/bug/refactor — start here |
| `product-manager` | Requirements & acceptance | Discovery and Acceptance stages |
| `principal-engineer` | Technical design | Design stage |
| `software-developer` | Implementation | Implementation stage (per task) |
| `build-specialist` | Build & test runner | After each implementation task |
| `quality-assurance` | Code review | Optional, before acceptance |

### SOP commands (`.claude/commands/`)

These are the primary interface for driving the pipeline. Each command
invokes the engineering-manager, which delegates to the right specialist.

| Command | Purpose | When |
|---------|---------|------|
| `/kickoff` | Bootstrap a new feature | User describes new work |
| `/discover` | Requirements gathering | After kickoff approval |
| `/design` | Technical design | After requirements approval |
| `/tasks` | Task breakdown | After design approval |
| `/implement` | Implement one task | After task breakdown approval |
| `/verify` | Run build and tests | After each implementation |
| `/accept` | Validate acceptance criteria | After all tasks complete |
| `/done` | Close out the feature | After acceptance passes |
| `/commit-only` | Stage and commit | Any time |
| `/commit-and-push` | Stage, commit, push | Any time |

### Shared state

`.state/feature-state.json` tracks the current feature lifecycle. Every agent
reads it on startup and the engineering-manager updates it at transitions.

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
Completed plans: (none yet)
