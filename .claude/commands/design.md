# Design

Produce a technical design for the current feature.

## What this does

Invokes the engineering-manager agent to:
1. Read current state and confirm the exec plan artifact exists
2. Update state to "design"
3. Output the exact prompt to run in the **principal-engineer** agent

The principal-engineer (run separately by you) will:
- Read the exec plan, ARCHITECTURE.md, CONTRIBUTING.md, RELIABILITY.md
- Scan the codebase for current structure
- Write a ## Design section into the exec plan
- Update ARCHITECTURE.md if new components are introduced
- Update the state file

## Input

$ARGUMENTS can provide design hints or constraints. If empty, the PE works
from the exec plan and codebase.

## Procedure

1. Invoke the engineering-manager agent with this instruction:

   "Run the Design stage ONLY. Read `.state/feature-state.json`, confirm the
   requirements artifact exists, update state to 'design', and output the
   exact prompt I should run in the principal-engineer agent. Additional
   guidance for the PE: [$ARGUMENTS]. Do NOT invoke the principal-engineer
   yourself."

2. Relay the engineering-manager's routing instruction to the user verbatim.

## Rules

- ONE stage only. Do not chain into Tasks.
- The EM outputs instructions — it does not run the PE itself.
- The Design section of the exec plan must be filled before `/tasks` will proceed.
