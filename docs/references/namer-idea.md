# namer

## What This Is

A CLI tool that generates random name pairs. You invoke it, you get a name. That's it.

The primary use case is generating hostnames, project codenames, throwaway identifiers, or anything else where you need a short, memorable, human-readable label on demand.

---

## The Problem It Solves

You need a quick random name. Right now you either make one up (slow, biased, repetitive) or go to a website (unnecessary friction for something that should be a one-liner in a terminal). This should be a local, instant, zero-dependency CLI call.

---

## Who Uses It

You. From a terminal. On any machine where you have a shell.

---

## MVP — The Rotten Core

The MVP does exactly one thing:

```
$ namer
witty-banana
```

That's it. One command, one output. An adjective-noun pair, hyphen-separated, lowercase. The word lists are hardcoded in the source. No flags, no config, no files to read. The entire thing should be achievable in under 50 lines of code.

**MVP acceptance criteria:**
- Invoke `namer` with no arguments
- Receive one random `adjective-noun` pair on stdout
- Exit cleanly

---

## Roadmap

Each item below is a single, isolated, atomic task. They are ordered by dependency (later items may depend on earlier ones) but each should be implementable and testable independently.

### R1: `--count N` flag
Generate multiple names at once. `namer --count 5` outputs 5 names, one per line. Default remains 1.

### R2: `--separator` flag
Control the delimiter between words. `namer --sep _` → `witty_banana`. `namer --sep none` → `wittybanana`. Default remains hyphen.

### R3: `--style` flag with adjective-noun as default
Introduce the concept of named generation styles. The only style at MVP+R3 is `adjective-noun` (the current default). This lays the groundwork for R4+. The flag exists, it accepts a value, but only one value works. `namer --style adjective-noun` behaves identically to `namer`.

### R4: Additional styles
Add at least two more styles:
- `corporate` — business-speak pairs like "synergy-nexus", "pivot-catalyst", "agile-framework"
- `fantasy` — fantasy/RPG-flavored names like "shadow-wyrm", "iron-glade", "ember-thorn"

Each style has its own hardcoded word lists.

### R5: External word list files
Instead of only hardcoded lists, allow loading word lists from a file (TOML, JSON, or plain text — pick one). A style can reference a file path. This enables user-created custom styles without modifying source code.

### R6: `--alliterative` flag
Constrain output so both words start with the same letter. `namer --alliterative` → `curious-clementine`. Works with any style.

### R7: Add tests
Unit tests covering:
- Default invocation produces valid adjective-noun output
- `--count` produces correct number of lines
- `--separator` applies correctly
- `--style` selects the right word pool
- `--alliterative` output actually alliterates
- Invalid flag/style handling produces useful errors

### R8: CI pipeline
Set up a basic CI workflow (GitHub Actions) that runs the test suite on push/PR.

---

## What This Is NOT

- **Not a library.** It's a CLI tool. No public API, no importable module (yet).
- **Not a naming service.** No HTTP server, no REST endpoint, no web UI.
- **Not a uniqueness guarantor.** It generates random names. Collisions are the caller's problem.
- **Not a dictionary.** Word lists are curated for "sounds good" vibes, not linguistic completeness.
- **Not permanent infrastructure.** This is a toy project used as a vehicle for testing agent-driven development workflows. The code matters less than the process around it.

---

## Notes for Agent Context

This project exists primarily as a testbed for agent-driven development (Claude Code, skills, slash commands, etc.). The code itself is intentionally trivial. The value is in exercising the agent workflow:

- Each roadmap item (R1–R8) is designed to be a clean, isolated task that an agent can pick up, implement, test, and complete in a single pass.
- The tasks escalate in complexity gradually: R1–R2 are flag parsing, R3–R4 are structural (introducing a style system), R5 is I/O, R6 is algorithmic constraint, R7–R8 are tooling.
- Language choice is deliberately left open. Pick whatever best serves the agent experimentation goals.
- There is no "right" architecture. If the agent wants to restructure between tasks, that's fine — observing how agents handle refactoring is part of the experiment.
