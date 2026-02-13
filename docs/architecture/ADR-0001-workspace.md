# ADR-0001: Workspace and Crate Boundaries

Status: Accepted

## Decision
Use a Rust workspace split into:
- `omega-core`
- `omega-content`
- `omega-save`
- `omega-tui`
- `omega-bevy`
- `omega-tools`

## Rationale
- Isolates domain logic from UI frameworks.
- Enables parallel execution by multiple agents.
- Allows incremental migration with feature parity checks.

## Consequences
- Requires strict interface contracts between crates.
- Frontends must not embed gameplay rules.
