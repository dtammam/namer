# Reliability

## Performance budgets

- Build time: < 10s
- Test suite: < 10s
- No runtime dependencies beyond std, clap, rand

## Invariants

- `generate_name` always returns exactly one adjective and one noun (enforced by `NameParts` struct).
- Word lists (`ADJECTIVES`, `NOUNS`) each contain at least one entry.
- `format_name` only applies casing to words, never to the delimiter.

## When a budget is at risk

Flag the regression before proceeding. Do not silently ship a performance
regression. The engineering-manager agent will decide how to handle it.
