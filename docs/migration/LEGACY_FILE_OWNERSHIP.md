# Legacy File Ownership Decisions

Updated: 2026-02-06

## Decision Table

| Area | Current owner | Decision | Timing |
|---|---|---|---|
| Root legacy C sources (`*.c`, `*.h`) | Migration leads | Archived to `archive/legacy-c-runtime/2026-02-06/` | Completed |
| Legacy Makefiles (`Makefile*`) | Migration leads | Archived to `archive/legacy-c-runtime/2026-02-06/` after grace-path retirement | Completed |
| Legacy docs in `docs/` (`compile.*`, historical readmes) | Docs/WS-I | Preserve as historical references | Ongoing |
| `lib/` legacy content files | WS-C | Keep as source-of-truth input for content parser until equivalent data packaging is finalized | Ongoing |

## Notes

- No destructive deletion was performed as part of Milestone 4 closure.
- Archive inventory is recorded in `archive/legacy-c-runtime/2026-02-06/MANIFEST.json`.
