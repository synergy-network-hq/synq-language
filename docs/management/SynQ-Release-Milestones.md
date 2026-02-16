# SynQ Release Milestones

## Purpose

This document defines milestone gates, owners, and target dates for SynQ delivery. It is the canonical release plan used by engineering, QA, and security tracks.

## Date Baseline

- Plan Date: 2026-02-09
- Time Zone: America/Chicago (Alabama locale)

## Milestone Map

## M0 - Truth and Stability

- Target Window: 2026-02-09 through 2026-02-20
- Owner: Platform Lead (SynQ Core)
- Supporting Owners:
  - PQC Lead (`aegis-pqsynq`)
  - Tooling Lead (CI/release)
  - Docs Lead (spec/status alignment)
- Scope:
  - Documentation and implementation reconciliation
  - Core parser TODO elimination
  - Baseline validation matrix enforcement
  - PQ substrate quality-gate closure
- Exit Criteria:
  - `docs/PQSynQ-Completion-Execution-Plan.md` reflects current implementation truth
  - `INTEGRATION_STATUS.md` reflects current implementation truth
  - `cargo test --workspace --all-targets --locked` passes
  - `bash aegis-pqsynq/pqsynq/scripts/run_tests.sh` passes
  - parser annotation and loop lowering tasks are closed with integration tests

## M1 - Compiler and VM Functional Core

- Target Window: 2026-02-23 through 2026-03-20
- Owner: Compiler Lead
- Supporting Owners:
  - VM Lead
  - Language/Parser Lead
  - QA Lead
- Scope:
  - semantic analysis foundation
  - deterministic and correct function-call lowering
  - runtime model hardening for contract execution
- Exit Criteria:
  - `SEM-001` through `SEM-003` status moved to DONE in backlog
  - `CG-002` and `CG-003` have deterministic integration coverage
  - canonical non-PQC sample contracts compile and execute with expected outputs

## M2 - Developer Workflow Usability

- Target Window: 2026-03-23 through 2026-04-24
- Owner: Developer Experience Lead
- Supporting Owners:
  - CLI Lead
  - SDK Lead
  - QA Lead
- Scope:
  - complete CLI test/deploy workflow
  - SDK key, tx, and contract-interaction hardening
  - canonical contract examples and scenario suites
- Exit Criteria:
  - `CLI-002` and `CLI-003` are DONE
  - SDK package has install/build/test pipeline and documented compatibility matrix
  - `SC-001` and `SC-002` are DONE with CI enforcement

## M3 - Production Candidate

- Target Window: 2026-04-27 through 2026-05-29
- Owner: Release/Security Lead
- Supporting Owners:
  - VM Lead
  - PQC Lead
  - QA Lead
- Scope:
  - security hardening completion
  - release-candidate regression and coverage gates
  - audit readiness package finalization
- Exit Criteria:
  - `SEC-001`, `SEC-002`, and `SEC-004` are DONE
  - `QA-004` one-command RC validation gate passes
  - release checklist signed by engineering, QA, and security owners

## Workstream Mapping

- Parser/Language -> M0, M1
- Semantic Analysis -> M1
- Codegen/VM Core -> M1
- CLI/SDK Workflow -> M2
- Contract Fixtures/Acceptance -> M2
- Security and RC Hardening -> M3

## Governance Rules

- No milestone can be marked complete while any listed exit criterion is unmet.
- Exit criteria require command evidence, not verbal confirmation.
- Scope changes must be logged in the decision log before milestone status changes.
