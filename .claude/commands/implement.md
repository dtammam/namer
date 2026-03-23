# Implement

Implement the next incomplete task from the task breakdown.

## What this does

Invokes the engineering-manager agent to:
1. Read current state and identify the next incomplete task
2. Update state to "implementation"
3. Output the exact prompt to run in the **software-developer** agent

The software-developer (run separately by you) will:
- Read the task description from the state file
- Read the exec plan design section for technical direction
- Read `docs/CONTRIBUTING.md` for coding standards
- Implement code and tests for that ONE task
- Run quality checks (fmt, clippy, test) and fix failures
- Report files changed and tests added

## Input

$ARGUMENTS can specify which task to implement if you want to override
the default order (e.g., "implement task 3"). If empty, picks the next
incomplete task in order.

## Procedure

1. Invoke the engineering-manager agent with this instruction:

   "Run the Implementation stage for ONE task only. Read `.state/feature-state.json`
   to identify the next incomplete task (or user-specified task: [$ARGUMENTS]).
   Output the exact prompt I should run in the software-developer agent to
   implement it. Do NOT invoke the software-developer yourself."

2. Relay the engineering-manager's routing instruction to the user verbatim.

## Rules

- ONE task per invocation.
- The EM outputs instructions — it does not run the SDE itself.
- If all tasks are already complete, the EM should tell the user to run `/accept`.
