# Accept

Validate that the implementation meets all acceptance criteria.

## What this does

Invokes the engineering-manager agent to:
1. Read current state and confirm all tasks are complete
2. Update state to "acceptance"
3. Output the exact prompt to run in the **product-manager** agent

The product-manager (run separately by you) will:
- Read the exec plan's acceptance criteria
- Verify each criterion against the current code and test results
- Report explicit pass/fail for every criterion — "looks good" is not acceptance
- Not implement fixes — report only

## Input

$ARGUMENTS is not typically needed.

## Procedure

1. Invoke the engineering-manager agent with this instruction:

   "Run the Acceptance stage ONLY. Read `.state/feature-state.json`, confirm
   all tasks are in completed_tasks, update state to 'acceptance', and output
   the exact prompt I should run in the product-manager agent to validate
   acceptance criteria. Do NOT invoke the product-manager yourself."

2. Relay the engineering-manager's routing instruction to the user verbatim.

## Rules

- All tasks must be complete before running acceptance.
- The PM verifies — it does not implement fixes.
- If criteria fail, run `/implement` to fix or defer to tech debt.
- All criteria must be explicitly checked.
- If all pass, run `/done`.
