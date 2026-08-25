# GitHub Actions Refactor Report

**Date**: 2026-08-25
**Scope**: `.github/` configuration refactoring, referenced against `easyofd-rust/.github/` mature template

---

## New File Structure

```
.github/
  workflows/
    ci.yml              -- check + test + clippy + fmt + block_on gate + concurrency bench + redis-test
    sync-feature.yml    -- cargo test -p wx-rust-miniapp --features sync (independent workflow)
    coverage.yml        -- cargo-llvm-cov 60% gate + codecov upload + HTML artifact
    security.yml        -- scheduled (weekly Monday) + push main, cargo audit + cargo deny
    release.yml         -- tag v*.*.* trigger, full validation + publish sequence
  dependabot.yml        -- cargo + github-actions dual-ecosystem, weekly Monday
  ISSUE_TEMPLATE/
    bug_report.md       -- Environment / Steps / Expected / Actual / Minimal Example
    feature_request.md  -- Description / Use Case / Proposed Solution / Alternatives
```

## WxRust Unique Features Preserved

| Feature | Location | Details |
|---------|----------|---------|
| block_on gate | ci.yml / release.yml | `scripts/check_block_on.sh` -- only blocking.rs may contain `block_on` |
| Concurrency bench (common) | ci.yml / release.yml | `cargo bench -p wx-rust-common --bench pipeline_concurrency_bench -- --test` |
| Concurrency bench (miniapp) | ci.yml / release.yml | `cargo bench -p wx-rust-miniapp --bench token_single_flight_bench -- --test` |
| sync-feature tests | sync-feature.yml / release.yml | `cargo test -p wx-rust-miniapp --features sync --test blocking_facade_test` |
| Coverage 60% gate | coverage.yml / release.yml | `cargo llvm-cov --workspace --fail-under-lines 60 --summary-only` |
| cargo audit | security.yml | Weekly Monday + push main |
| cargo deny check | security.yml | Weekly Monday + push main |
| Redis integration tests | ci.yml | `cargo test -p wx-rust-common --features redis --test redis_integration_test` |

## Differences from easyofd-rust Template

| Aspect | easyofd-rust | WxRust | Rationale |
|--------|-------------|--------|-----------|
| Test matrix | 3 OS (ubuntu/macos/windows) x 2 Rust (stable + MSRV) | ubuntu-latest + stable only | WxRust does not require multi-platform CI yet |
| Coverage | Daily schedule + codecov upload | push/PR + codecov upload | WxRust uses coverage as a CI gate, not just a daily metric |
| Coverage gate | No fail-under threshold | `--fail-under-lines 60` | WxRust enforces minimum coverage |
| Release matrix | 3 OS cross-build | ubuntu-latest only | Matches CI platform choice |
| Release publish | Easyofd crate list | WxRust crate list (10 crates in dependency order) | Different workspace structure |
| Security trigger | Weekly + push main | Weekly + push main | Identical pattern |
| Issue templates | easyofd-specific references | WxRust/WeChat-specific references | Adapted for WeChat SDK context |

## CI Gate Matrix

| Gate | Workflow | Trigger | Enforcement |
|------|----------|---------|-------------|
| rustfmt | ci.yml | push/PR to main | Fail on format violations |
| cargo check | ci.yml | push/PR to main | Fail on compile errors |
| clippy | ci.yml | push/PR to main | Fail on warnings (`-D warnings`) |
| cargo test | ci.yml | push/PR to main | Fail on test failures |
| block_on gate | ci.yml | push/PR to main | Fail if `block_on` found outside blocking.rs |
| concurrency bench | ci.yml | push/PR to main | Fail on assertion panic in bench --test |
| redis integration | ci.yml | push/PR to main | Fail on redis test failures |
| sync-feature | sync-feature.yml | push/PR to main | Fail on blocking facade test failures |
| coverage 60% | coverage.yml | push/PR to main | Fail if line coverage < 60% |
| cargo audit | security.yml | Weekly Monday + push main | Fail on known vulnerabilities |
| cargo deny | security.yml | Weekly Monday + push main | Fail on license/advisory violations |
| Release gate | release.yml | tag v*.*.* | All above gates + dry-run publish |

## Release Pipeline Flow

```
validate-tag -> build-and-test (all CI gates) -> dry-run-publish -> publish-crates -> create-release
```

The `build-and-test` job in release.yml consolidates all CI gates (check, test, clippy, fmt, block_on, concurrency bench, sync-feature, coverage 60%) into a single pre-release gate. This ensures no code reaches crates.io without passing every gate.

## YAML Validation

All 6 YAML files passed `python3 yaml.safe_load` validation:

- `.github/workflows/ci.yml`
- `.github/workflows/sync-feature.yml`
- `.github/workflows/coverage.yml`
- `.github/workflows/security.yml`
- `.github/workflows/release.yml`
- `.github/dependabot.yml`
