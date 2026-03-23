# Verify

Run the build and test suite to verify the latest implementation.

## What this does

Invokes the engineering-manager agent, which delegates to the build-specialist
agent to:
1. Run `cargo build`
2. Run `cargo test`
3. Run `cargo fmt -- --check`
4. Run `cargo clippy -- -D warnings`
5. Report pass/fail for each

The engineering-manager then:
1. Presents the build report to the user
2. If all pass and more tasks remain: suggests running `/implement` next
3. If all pass and no tasks remain: suggests running `/accept` next
4. If any fail: presents the failures for the user to decide next steps

## Input

$ARGUMENTS is not typically needed. Can include "verbose" for full output.

## Procedure

1. Invoke the engineering-manager agent with this instruction:

   "Run Verification ONLY. Delegate to the build-specialist agent to run
   the full build and test suite. Present the results and stop. If
   everything passes and more tasks remain, tell the user to run
   `/implement` for the next task. If everything passes and all tasks
   are done, tell the user to run `/accept`. Do NOT proceed to any
   other stage automatically."

2. Relay the engineering-manager's output to the user.

## Rules

- The build-specialist does NOT fix code. It only reports.
- If tests fail, the user decides whether to run `/implement` again to
  fix the issue or handle it manually.
