# Branching and Integration

## Primary Model
- Trunk-based development with short-lived branches.
- Branch prefix is mandatory: `codex/`.

## Integration Rhythm
- Daily merges from active workstream branches into main after CI passes.
- Weekly integration checkpoint for cross-stream compatibility.

## Merge Policy
- Squash merge by default for feature branches.
- Rebase branch on latest main before merge if conflicts arise.
- Do not bypass required checks.

## Workstream Isolation
- Keep ownership by crate boundary.
- Prefer additive interfaces over cross-crate rewrites.
- Freeze shared contracts before parallel feature bursts.

## Emergency Fixes
- Prefix hotfix branches as `codex/hotfix-*`.
- Require post-merge retro to capture root cause and test gap.
