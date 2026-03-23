# Design

Produce a technical design for the current feature.

## What this does

Invokes the engineering-manager agent, which delegates to the principal-engineer
agent to:
1. Read the exec plan (requirements, scope, constraints)
2. Read `docs/ARCHITECTURE.md`, `docs/CONTRIBUTING.md`, `docs/RELIABILITY.md`
3. Scan the codebase for current structure
4. Write the Design section of the exec plan
5. Update `docs/ARCHITECTURE.md` if the design introduces new components
6. Update the state file
7. Stop and present the design for approval

## Input

$ARGUMENTS can provide design hints or constraints. If empty, the PE works
from the exec plan and codebase.

## Procedure

1. Invoke the engineering-manager agent with this instruction:

   "Run the Design stage ONLY for the current feature. Delegate to the
   principal-engineer agent to produce a technical design. The exec plan
   is at the path in `.state/feature-state.json` artifacts.requirements.
   Additional guidance from user: [$ARGUMENTS]. After the principal-engineer
   returns, update the state file. Present the design summary and stop.
   Do NOT proceed to Tasks or any other stage."

2. Relay the engineering-manager's output to the user.

3. Tell the user: "Review the design above. When approved, run `/tasks`
   to break the work into implementable tasks."

## Rules

- ONE stage only. Do not chain into Tasks.
- If the design is trivial (under 20 lines of code, single file), the PE
  should say so — a lightweight design note is fine.
- The Design section of the exec plan must be filled before this stage
  is considered complete.
