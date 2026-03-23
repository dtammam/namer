# Discover

Gather requirements and acceptance criteria for the current feature.

## What this does

Invokes the engineering-manager agent, which delegates to the product-manager
agent to:
1. Read the current feature context from `.state/feature-state.json`
2. Ask the user for goal, scope, out-of-scope, constraints, acceptance criteria
3. Cross-check output against `docs/CONTRIBUTING.md` (mandatory standards cannot
   be deferred or scoped out)
4. Write the exec plan to `docs/exec-plans/active/`
5. Update the state file with the artifact path
6. Stop and present results for approval

## Input

$ARGUMENTS can provide additional context or reference docs (e.g., "see
docs/references/namer-idea.md for the full spec"). If empty, the PM will
ask the user directly.

## Procedure

1. Invoke the engineering-manager agent with this instruction:

   "Run the Discovery stage ONLY for the current feature. Delegate to the
   product-manager agent to gather requirements. Additional context from
   user: [$ARGUMENTS]. After the product-manager returns, cross-check that
   nothing in 'out of scope' conflicts with docs/CONTRIBUTING.md mandatory
   standards. Update the state file. Present the requirements summary and
   stop. Do NOT proceed to Design or any other stage."

2. Relay the engineering-manager's output to the user.

3. Tell the user: "Review the requirements above. When approved, run `/design`
   to start technical design."

## Rules

- ONE stage only. Do not chain into Design.
- If CONTRIBUTING.md mandates something that the PM scoped out, the EM must
  re-engage the PM to fix it before presenting results.
- The exec plan file must exist at `docs/exec-plans/active/` before this
  stage is considered complete.
