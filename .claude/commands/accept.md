# Accept

Validate that the implementation meets all acceptance criteria.

## What this does

Invokes the engineering-manager agent, which delegates to the product-manager
agent to:
1. Read the exec plan's acceptance criteria
2. Verify each criterion against the current code and test results
3. Report pass/fail for each criterion

The engineering-manager then:
1. Presents the acceptance report to the user
2. If all pass: suggests running `/done` to close out
3. If any fail: presents failures for the user to decide (fix or defer)

## Input

$ARGUMENTS is not typically needed.

## Procedure

1. Invoke the engineering-manager agent with this instruction:

   "Run the Acceptance stage ONLY. Delegate to the product-manager agent
   to validate all acceptance criteria from the exec plan. Present the
   pass/fail report and stop. If all criteria pass, tell the user to run
   `/done`. If any fail, present the failures and let the user decide.
   Do NOT mark the feature as done automatically."

2. Relay the engineering-manager's output to the user.

## Rules

- The product-manager verifies — it does not implement fixes.
- If criteria fail, the user may run `/implement` to fix, or defer
  the failure to tech debt.
- All criteria must be explicitly checked. "Looks good" is not acceptance.
