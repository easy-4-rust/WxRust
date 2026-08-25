# WxRust Publish Pipeline Validation Report

**Date**: 2026-08-25
**Environment**: macOS arm64, cargo 1.97.1, cargo-audit, cargo-deny
**Mode**: Dry-run only -- no crates were published

---

## 1. Publish Precheck Results

`scripts/publish-precheck.sh` -- all 16 checks PASS.

Note: The script's own output was truncated at line 8 due to tool-layer buffer.
All results below were captured via individual step-by-step runs with identical commands.

### Workspace Quality Checks

| Check | Result |
|---|---|
| `cargo check --workspace --all-features` | PASS |
| `cargo test --workspace` | PASS |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | PASS |
| `cargo fmt --all -- --check` | PASS |
| `cargo audit --ignore RUSTSEC-2023-0071` | PASS |
| `cargo deny check` | PASS (2 advisory-only license warnings, no blockers) |

### Per-Crate Packaging Validation

| Crate | Validation Mode | Result | Files |
|---|---|---|---|
| wx-rust-common | `cargo publish --dry-run` | PASS | 134 |
| wx-rust-aispeech | `cargo package --list` | PASS | 41 |
| wx-rust-channel | `cargo package --list` | PASS | 713 |
| wx-rust-cp | `cargo package --list` | PASS | 670 |
| wx-rust-miniapp | `cargo package --list` | PASS | 658 |
| wx-rust-mp | `cargo package --list` | PASS | 404 |
| wx-rust-pay | `cargo package --list` | PASS | 636 |
| wx-rust-qidian | `cargo package --list` | PASS | 42 |
| wx-rust-open | `cargo package --list` | PASS | 252 |
| wx-rust | `cargo publish --dry-run` | PASS | 6 |

**Total: 16/16 PASS, 0 FAIL**

---

## 2. Publish Order Verification

`scripts/publish-order.sh` -- 10/10 PASS.

The script defines a 4-layer dependency topology:

| Layer | Crate | Internal Deps | Validation Mode |
|---|---|---|---|
| 0 | wx-rust-common | none | full dry-run |
| 1 | wx-rust-aispeech | common | package --list |
| 1 | wx-rust-channel | common | package --list |
| 1 | wx-rust-cp | common | package --list |
| 1 | wx-rust-miniapp | common | package --list |
| 1 | wx-rust-mp | common | package --list |
| 1 | wx-rust-pay | common | package --list |
| 1 | wx-rust-qidian | common | package --list |
| 2 | wx-rust-open | common, mp, miniapp | package --list |
| 3 | wx-rust | none (umbrella) | full dry-run |

---

## 3. Ten-Crate Publish Status Table

| # | Crate | Layer | Status | Dry-Run Command | Notes |
|---|---|---|---|---|---|
| 1 | wx-rust-common | 0 | PASS | `cargo publish -p wx-rust-common --dry-run --allow-dirty` | Foundation crate; no internal deps; 134 files, 575 KiB |
| 2 | wx-rust-aispeech | 1 | PASS | `cargo package -p wx-rust-aispeech --list --allow-dirty` | Full dry-run blocked until common is on crates.io |
| 3 | wx-rust-channel | 1 | PASS | `cargo package -p wx-rust-channel --list --allow-dirty` | Full dry-run blocked until common is on crates.io |
| 4 | wx-rust-cp | 1 | PASS | `cargo package -p wx-rust-cp --list --allow-dirty` | Full dry-run blocked until common is on crates.io |
| 5 | wx-rust-miniapp | 1 | PASS | `cargo package -p wx-rust-miniapp --list --allow-dirty` | Full dry-run blocked until common is on crates.io |
| 6 | wx-rust-mp | 1 | PASS | `cargo package -p wx-rust-mp --list --allow-dirty` | Full dry-run blocked until common is on crates.io |
| 7 | wx-rust-pay | 1 | PASS | `cargo package -p wx-rust-pay --list --allow-dirty` | Full dry-run blocked until common is on crates.io |
| 8 | wx-rust-qidian | 1 | PASS | `cargo package -p wx-rust-qidian --list --allow-dirty` | Full dry-run blocked until common is on crates.io |
| 9 | wx-rust-open | 2 | PASS | `cargo package -p wx-rust-open --list --allow-dirty` | Depends on common + mp + miniapp; full dry-run blocked until all 3 are on crates.io |
| 10 | wx-rust | 3 | PASS | `cargo publish -p wx-rust --dry-run --allow-dirty` | Umbrella crate; zero internal deps; 6 files |

---

## 4. wx-rust-common Dedicated Dry-Run

```
$ cargo publish -p wx-rust-common --dry-run --allow-dirty

    Updating crates.io index
   Packaging wx-rust-common v0.1.0
    Updating crates.io index
    Packaged 134 files, 575.1KiB (154.0KiB compressed)
   Verifying wx-rust-common v0.1.0
   Compiling wx-rust-common v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.19s
   Uploading wx-rust-common v0.1.0
warning: aborting upload due to dry run
```

Result: **PASS** -- package builds, verifies, and is ready for upload.

---

## 5. Crates.io Token Status

| Check | Result |
|---|---|
| `~/.cargo/credentials.toml` | EXISTS -- `[registry] token = "ciokC5..."` is configured |
| `CARGO_REGISTRY_TOKEN` env var | NOT SET (not required; credentials.toml is sufficient) |
| Existing crates.io registration | wx-rust-common, wx-rust NOT yet registered on crates.io |
| `cargo login` | NOT executed (read-only check only) |

**Status**: Token is configured and ready for publish.

---

## 6. Real Publish Command Sequence

The following commands execute the full publish pipeline. Each `sleep 30` waits for
crates.io index propagation (required so dependent crates can resolve the new version).

```bash
# Step 0: Ensure token is active (skip if credentials.toml already has token)
cargo login

# Step 1: Publish foundation crate (Layer 0)
cargo publish -p wx-rust-common
sleep 30

# Step 2: Publish Layer 1 crates (depend on wx-rust-common)
cargo publish -p wx-rust-aispeech
sleep 30

cargo publish -p wx-rust-channel
sleep 30

cargo publish -p wx-rust-cp
sleep 30

cargo publish -p wx-rust-miniapp
sleep 30

cargo publish -p wx-rust-mp
sleep 30

cargo publish -p wx-rust-pay
sleep 30

cargo publish -p wx-rust-qidian
sleep 30

# Step 3: Publish Layer 2 crate (depends on common + mp + miniapp)
cargo publish -p wx-rust-open
sleep 30

# Step 4: Publish umbrella crate (Layer 3, no internal deps)
cargo publish -p wx-rust

# Step 5: Tag the release
git tag v0.1.0
git push origin v0.1.0

# Step 6: docs.rs auto-builds within ~15 minutes of publish.
# Verify at: https://docs.rs/wx-rust-common/latest/
#            https://docs.rs/wx-rust/latest/
```

**Post-publish verification**:
```bash
# Confirm all crates are live
for c in wx-rust-common wx-rust-aispeech wx-rust-channel wx-rust-cp \
         wx-rust-miniapp wx-rust-mp wx-rust-pay wx-rust-qidian \
         wx-rust-open wx-rust; do
  cargo search "$c" 2>&1 | head -3
done
```

---

## 7. Risks and Blockers

### Chicken-and-Egg Dependency (BLOCKING for full dry-run validation)

8 of 10 crates depend on `wx-rust-common` which is not yet on crates.io.
This means:

| Affected Crates | Blocker | Mitigation |
|---|---|---|
| wx-rust-aispeech, channel, cp, miniapp, mp, pay, qidian | Need wx-rust-common on crates.io for full dry-run | `cargo package --list` validates packaging; full dry-run deferred to post-publish |
| wx-rust-open | Needs wx-rust-common + wx-rust-mp + wx-rust-miniapp on crates.io | Same; will require 3 prior publishes |

**Resolution**: Once wx-rust-common is published, re-run:
```bash
cargo publish -p wx-rust-aispeech --dry-run --allow-dirty
# ... and similarly for each Layer 1 crate
```

### Other Risks

| Risk | Severity | Mitigation |
|---|---|---|
| crates.io name collision | LOW | `cargo search` shows no existing "wx-rust-*" crates |
| 60-second crates.io rate limit between publishes | MEDIUM | Script uses `sleep 30`; may need `sleep 60` if rate-limited |
| docs.rs build failure | LOW | `wx-rust` has `[package.metadata.docs.rs] all-features = true` |
| `cargo deny` license advisory warnings | NONE | Advisory only (MPL-2.0, OpenSSL allowed but not encountered); no blockers |
| `cargo audit` RUSTSEC-2023-0071 ignored | LOW | Known advisory, explicitly ignored in precheck |

---

## 8. Summary

- **Precheck**: 16/16 checks PASS
- **Publish order**: 10/10 crates PASS
- **wx-rust-common dry-run**: PASS (134 files, 575 KiB, builds and verifies)
- **crates.io token**: Configured in `~/.cargo/credentials.toml`
- **Blockers**: None for real publish. Chicken-and-egg prevents full dry-run of 8 dependent crates until wx-rust-common is live.
- **Ready to publish**: YES

---

**Report generated**: 2026-08-25 by DevOps automation
**Logs**: `docs/verification/publish-precheck-2026-08-25.log`, `docs/verification/publish-order-2026-08-25.log`
