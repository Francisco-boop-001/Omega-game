# Agent Guidelines for Omega

This document provides essential information for AI coding agents working on the Omega codebase.

## Build / Lint / Test Commands

Always run these commands before submitting changes:

```bash
# Format check
cargo fmt --all -- --check

# Linting
cargo clippy --workspace --all-targets -- -D warnings

# Run all tests
cargo test --workspace

# Run a single test (example)
cargo test --workspace -p omega-core -- my_test_name

# Run tests with nextest (if installed)
cargo nextest run --workspace

# Build documentation
cargo doc --workspace --no-deps
```

## Workspace Structure

This is a Rust workspace with the following crates:
- `omega-core` - Core game state, types, and logic
- `omega-content` - Content definitions (items, monsters, etc.)
- `omega-save` - Save/load serialization
- `omega-bevy` - Bevy frontend
- `omega-tui` - Terminal UI frontend
- `omega-tools` - Development and testing tools

## Code Style Guidelines

### Formatting
- Max width: 100 characters (see `rustfmt.toml`)
- Use small heuristics at "Max" setting
- Run `cargo fmt --all` before committing

### Imports
- Group imports: std, external crates, internal modules
- Use workspace dependencies where defined
- Prefer `use crate::` for internal imports

### Types and Naming
- Use PascalCase for types, enums, structs
- Use snake_case for functions, variables, modules
- Use SCREAMING_SNAKE_CASE for constants
- Prefer explicit types over `impl Trait` in public APIs
- Use `#[derive(Debug, Clone, ...)]` for data types

### Error Handling
- Use `thiserror` for error enums
- Use `anyhow` for application-level errors
- Propagate errors with `?` operator
- Avoid unwrap() in production code; use expect() with messages

### Code Organization
- Place unit tests in `#[cfg(test)]` modules at end of files
- Use `mod.rs` for module re-exports
- Keep modules focused on single responsibility
- Public APIs should be documented

### Enums and Constants
- Use `#[derive(Default)]` for enums with sensible defaults
- Define flag constants as `pub const` with type annotations
- Group related constants near their usage

## Git Workflow

- Branch naming: `codex/<scope>-<description>`
  - Example: `codex/ws-b-core-turn-loop`
- Commit messages: Use imperative mood, prefix with workstream
  - Example: `WS-B: add deterministic command dispatch`
- Workstream locking required - see `swarm/LOCKING.md`

## Ownership and Reviews

- Check `.github/CODEOWNERS` for per-crate ownership
- All changes require at least one owner review
- Cross-workstream changes need additional reviewer

## Legacy Code

- This repo is migrating from C to Rust
- New work targets `crates/*` only
- Legacy code in `lib/` and root is maintenance-only

## CI Requirements

PRs must pass:
- `fmt` - Formatting check
- `clippy` - Linting
- `test` - Unit tests
- `doc` - Documentation builds

## Tooling

- Rust toolchain: stable (see `rust-toolchain.toml`)
- Required components: rustfmt, clippy
- MSRV: 1.93.0 (see `clippy.toml`)
