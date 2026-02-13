# WS-H Security Checklist

This checklist is required for PRs touching save/content parsing, file IO, or panic behavior.

## Save File Hardening

- [ ] Decode through `omega-save` envelope APIs (`decode_json` / `decode_state_json`) only.
- [ ] Reject unknown save versions with a hard error.
- [ ] Preserve migration path coverage tests for legacy save shapes.
- [ ] Treat decode failures as recoverable errors, never as panics.
- [ ] Avoid unbounded memory growth from untrusted payload fields.

## Path Safety

- [ ] Use fixed workspace-relative locations for built-in assets/checks.
- [ ] Do not concatenate unchecked user input into filesystem paths.
- [ ] Reject or sanitize paths that escape expected roots for tooling.
- [ ] Prefer explicit extension filtering (`.map`, `.json`) when scanning.

## Panic Policy

- [ ] Runtime crates (`omega-core`, `omega-save`, `omega-content`) return errors for invalid external input.
- [ ] Build-time validation (`build.rs`) may fail fast to stop invalid assets at compile time.
- [ ] CLI tools should return non-zero exit codes on validation failures.
- [ ] New `unwrap()`/`expect()` in non-test code requires explicit justification in review.

## Fuzzing and Property Coverage

- [ ] Parser/save changes include `proptest` coverage for core invariants.
- [ ] Fuzz smoke corpus updated when new parser/save branches are added.
- [ ] LibFuzzer targets remain buildable under `fuzz/`.
