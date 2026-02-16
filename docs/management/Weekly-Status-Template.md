# SynQ Weekly Status and Risk Cadence Template

## Purpose

This template standardizes weekly program reporting, risk tracking, and technical decision capture.

## Usage

- Frequency: weekly (recommended Monday morning in local Alabama time)
- Source of truth: commit this file or a per-week copy under `docs/management/status/`

## Weekly Status Template

```markdown
# SynQ Weekly Status - YYYY-MM-DD

## Overall Health

- Health: Green | Yellow | Red
- Summary: <one-paragraph factual status>

## Completed This Week

- [ ] <Task ID> - <evidence link/path>
- [ ] <Task ID> - <evidence link/path>

## In Progress

- [ ] <Task ID> - <current progress and next action>

## Blocked

- [ ] <Task ID> - Blocker: <what blocks it> - Owner: <role/name> - ETA to unblock: <date>

## Scope Changes

- <approved scope change, rationale, decision id>

## Validation Evidence

- `cargo test --workspace --all-targets --locked` -> PASS/FAIL
- `bash aegis-pqsynq/pqsynq/scripts/run_tests.sh` -> PASS/FAIL
- Additional evidence:
  - <path to logs/reports>

## Plan for Next Week

1. <highest priority task>
2. <second task>
3. <third task>
```

## Risk Register Template

```markdown
# SynQ Risk Register

| Risk ID | Date Opened | Description | Severity | Likelihood | Owner | Mitigation | Trigger/Indicator | Status |
|---|---|---|---|---|---|---|---|---|
| R-001 | YYYY-MM-DD | <risk statement> | High/Med/Low | High/Med/Low | <owner> | <mitigation> | <observable trigger> | Open |
```

## Decision Log Template

```markdown
# SynQ Decision Log

| Decision ID | Date | Topic | Decision | Alternatives Rejected | Impacted Areas | Owner |
|---|---|---|---|---|---|---|
| D-001 | YYYY-MM-DD | <topic> | <decision> | <alternatives> | <compiler/vm/sdk/docs/etc> | <owner> |
```

## First Cycle Entry (Recorded)

- Cycle Date: 2026-02-09
- Health: Yellow
- Completed Highlights:
  - PQ substrate validation matrix is passing end-to-end
  - parser annotations + `for` lowering landed with integration tests
- Active Risks:
  - semantic analysis and SDK crypto implementation lag remains schedule-critical
