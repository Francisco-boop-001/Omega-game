# Contributing

## Scope
This repository is migrating from legacy C sources to a Rust multi-crate workspace.
All new modernization work should target `crates/*` unless explicitly marked as legacy maintenance.

## Branching Convention
- Use branch names prefixed with `codex/`.
- Use short, scoped suffixes, for example:
  - `codex/ws-a-governance`
  - `codex/ws-b-core-turn-loop`
  - `codex/ws-e-save-v1`

## Pull Request Requirements
- Keep changes scoped to one workstream when possible.
- Complete the PR template checklist.
- Ensure CI passes for `fmt`, `clippy`, `test`, and `doc`.
- Update docs/ADR for architecture or process changes.

## Ownership and Reviews
- Ownership is declared in `.github/CODEOWNERS`.
- At least one owner from each touched crate must review.
- Cross-workstream changes require one additional reviewer from another stream.

## Workstream Locking
- Claim a lock before editing a workstream:
  - `powershell -ExecutionPolicy Bypass -File .\scripts\ws-lock.ps1 -Action claim -Workstream WS-D`
- Release lock when done:
  - `powershell -ExecutionPolicy Bypass -File .\scripts\ws-lock.ps1 -Action release -Workstream WS-D`
- Protocol details: `swarm/LOCKING.md`

## Commit Guidelines
- Use clear, imperative subjects.
- Prefix message with stream when useful, e.g.:
  - `WS-A: add CODEOWNERS and governance docs`
  - `WS-B: add deterministic command dispatch`

## Testing
Run locally before opening PR:

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo doc --workspace --no-deps
```
