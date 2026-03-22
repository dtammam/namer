# Reliability

## Performance budgets

{{Fill in project-specific budgets. Examples:}}

{{- Primary UI actions: < 16ms}}
{{- API response time: < 200ms p95}}
{{- Build time: < 30s}}
{{- Test suite: < 60s}}

## Invariants

{{Things that must never be violated, regardless of what a task requires.}}

## When a budget is at risk

Flag the regression before proceeding. Do not silently ship a performance
regression. The engineering-manager agent will decide how to handle it.
