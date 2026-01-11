---
name: verify
description: Run repository quality checks (lint, format, type check, test). Use /verify to run all checks, or /verify lint, /verify format, /verify test, /verify e2e to run individually. Use after code changes or before commits.
---

# Verify Skill

Run quality checks for this repository.

## Run All Checks

`/verify` runs the following in order (E2E not included):

1. `pnpm format:check` - Format check
2. `pnpm lint` - ESLint
3. `pnpm exec tsc --noEmit` - Type check
4. `pnpm test` - Unit tests

## Run Individual Checks

| Argument | Command |
|----------|---------|
| `format` | `pnpm format:check` |
| `lint` | `pnpm lint` |
| `type` | `pnpm exec tsc --noEmit` |
| `test` | `pnpm test` |
| `e2e` | `pnpm e2e` |
| `fix` | `pnpm format && pnpm lint:fix` |

## Error Handling

- Format errors: Auto-fix with `pnpm format`
- Lint errors: Auto-fix where possible with `pnpm lint:fix`
- Type errors: Manually fix the code
- Test errors: Review and fix failing tests
