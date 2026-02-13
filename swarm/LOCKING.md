# Swarm Lock Protocol

To avoid concurrent file stomps, claim a workstream lock before editing shared crates.

## Commands

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\ws-lock.ps1 -Action claim -Workstream WS-D -Owner your-github-handle
powershell -ExecutionPolicy Bypass -File .\scripts\ws-lock.ps1 -Action status -Workstream WS-D
powershell -ExecutionPolicy Bypass -File .\scripts\ws-lock.ps1 -Action release -Workstream WS-D
```

## Rules
- One active lock per workstream.
- Do not edit files owned by another locked workstream.
- If a lock exists and seems stale, coordinate before removal.
- Owner must match format `[A-Za-z0-9._-]+`.

## CI Enforcement
- CI runs `scripts/check-lock-ownership.ps1` in `lock_guard` job.
- If a touched workstream has an active lock and the lock owner differs from `github.actor`, CI fails.
