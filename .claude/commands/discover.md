# Discover

Gather requirements and acceptance criteria for the current feature.

## What this does

Invokes the engineering-manager agent to:
1. Read current state from `.state/feature-state.json`
2. Update state to "discovery"
3. Output the exact prompt to run in the **product-manager** agent

The product-manager (run separately by you) will:
- Gather goal, scope, out-of-scope, constraints, acceptance criteria
- Cross-check against `docs/CONTRIBUTING.md` mandatory standards
- Write the exec plan to `docs/exec-plans/active/`
- Update the state file with the artifact path

## Input

$ARGUMENTS can provide additional context for the PM (e.g., "focus on CLI UX").
If empty, the PM works from the state file.

## Procedure

1. Invoke the engineering-manager agent with this instruction:

   "Run the Discovery stage ONLY. Read `.state/feature-state.json`, update
   state to 'discovery', and output the exact prompt I should run in the
   product-manager agent to gather requirements. Additional context for the
   PM: [$ARGUMENTS]. Do NOT invoke the product-manager yourself."

2. Relay the engineering-manager's routing instruction to the user verbatim.

## Rules

- ONE stage only. Do not chain into Design.
- The EM outputs instructions — it does not run the PM itself.
- The exec plan file must exist at `docs/exec-plans/active/` before `/design`
  will proceed.
