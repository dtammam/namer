# Verify

Run the build and test suite to verify the latest implementation.

## What this does

Invokes the engineering-manager agent to:
1. Read current state
2. Output the exact prompt to run in the **build-specialist** agent

The build-specialist (run separately by you) will:
- Run `cargo build`
- Run `cargo test`
- Run `cargo fmt -- --check`
- Run `cargo clippy -- -D warnings`
- Report pass/fail for each with full output on failures

## Input

$ARGUMENTS is not typically needed. Can include "verbose" for full output.

## Procedure

1. Invoke the engineering-manager agent with this instruction:

   "Run the Verification stage ONLY. Read `.state/feature-state.json` and
   output the exact prompt I should run in the build-specialist agent to
   verify the build. Do NOT invoke the build-specialist yourself."

2. Relay the engineering-manager's routing instruction to the user verbatim.

## Rules

- The build-specialist reports only — it does not fix code.
- If tests fail, run `/implement` to fix or handle manually.
- If all pass and tasks remain: run `/implement` for the next task.
- If all pass and no tasks remain: run `/accept`.
