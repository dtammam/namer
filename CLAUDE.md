# CLAUDE.md

This file is the Claude Code entry point for this repo.

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

### Shared state

`.state/feature-state.json` tracks the current feature lifecycle. Every agent
reads it on startup and the engineering-manager updates it at transitions.

### Workflow

```
User describes work
  → engineering-manager activates
    → product-manager (Discovery)
    → principal-engineer (Design)
    → engineering-manager (Task breakdown)
    → software-developer (Implementation, per task)
    → build-specialist (Verify, per task)
    → product-manager (Acceptance)
  → Done
```

Every stage transition requires explicit user approval. No auto-progression.

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
