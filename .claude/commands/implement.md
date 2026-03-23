# Implement

Implement the next incomplete task from the task breakdown.

## What this does

Invokes the engineering-manager agent, which delegates to the software-developer
agent to:
1. Read `.state/feature-state.json` to find the next incomplete task
2. Read the exec plan (design section) for technical direction
3. Read `docs/CONTRIBUTING.md` for coding standards
4. Implement the code and tests for that ONE task
5. Run quality checks (fmt, clippy, test)
6. Report completion with a summary of changes

The engineering-manager then:
1. Updates the state file (moves task to completed_tasks)
2. Presents results to the user
3. Stops

## Input

$ARGUMENTS can specify which task to implement if you want to override
the default order (e.g., "implement task 3"). If empty, picks the next
incomplete task in order.

## Procedure

1. Invoke the engineering-manager agent with this instruction:

   "Run the Implementation stage for ONE task only. Pick the next
   incomplete task from the state file (or the user-specified task:
   [$ARGUMENTS]). Delegate to the software-developer agent to implement
   it. After the software-developer returns, update the state file.
   Present the implementation summary and stop. Do NOT proceed to
   the next task, Verification, or any other stage."

2. Relay the engineering-manager's output to the user.

3. Tell the user: "Run `/verify` to run the build and test suite against
   this change."

## Rules

- ONE task per invocation. Not two, not all remaining.
- The software-developer writes code. The EM does not.
- If the software-developer reports a failure after 3 AutoSDE iterations,
  surface the failure to the user — do not retry indefinitely.
- If all tasks are already complete, tell the user to run `/accept` instead.
