# Reliability

## Performance budgets

- Build time: < 10s
- Test suite: < 10s
- No runtime dependencies beyond std, clap, rand

## Invariants

(None yet — to be defined as the project grows)

## When a budget is at risk

Flag the regression before proceeding. Do not silently ship a performance
regression. The engineering-manager agent will decide how to handle it.