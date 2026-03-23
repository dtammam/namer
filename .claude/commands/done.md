# Done

Close out the current feature after acceptance passes.

## What this does

Invokes the engineering-manager agent to:
1. Set the feature stage to "done" in `.state/feature-state.json`
2. Move the exec plan from `docs/exec-plans/active/` to `docs/exec-plans/completed/`
3. Update CLAUDE.md's "Active work" section
4. Summarize: what was built, artifacts produced, any tech debt created
5. Remind the user to commit via `/commit-and-push`

## Input

$ARGUMENTS is not typically needed.

## Procedure

1. Invoke the engineering-manager agent with this instruction:

   "Run the Done stage ONLY. Mark the feature as complete: update the
   state file, move the exec plan to completed, update CLAUDE.md's
   active work section. Summarize what was built and remind the user
   to run `/commit-and-push`."

2. Relay the engineering-manager's output to the user.

## Rules

- Only run this after `/accept` has passed.
- The state file should be reset to `{}` after completion so the next
  `/kickoff` starts clean.
- If tech debt was created during implementation, it must be recorded in
  `docs/exec-plans/tech-debt-tracker.md` before closing.
