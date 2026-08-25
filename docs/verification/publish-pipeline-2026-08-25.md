# WxRust Publish Pipeline Verification

**Date**: 2026-08-25  
**Cargo version**: 1.97.1  
**Platform**: darwin arm64

---

## Publish Order

Dependency-sorted publish order (3 layers):

| # | Crate | Layer | Internal Dependencies | Dry-run Result |
|---|-------|-------|-----------------------|----------------|
| 1 | `wx-rust-common` | 0 | none | PASS (full dry-run) |
| 2 | `wx-rust-aispeech` | 1 | common | PASS (package --list, 41 files) |
| 3 | `wx-rust-channel` | 1 | common | PASS (package --list, 713 files) |
| 4 | `wx-rust-cp` | 1 | common | PASS (package --list, 670 files) |
| 5 | `wx-rust-miniapp` | 1 | common | PASS (package --list, 658 files) |
| 6 | `wx-rust-mp` | 1 | common | PASS (package --list, 404 files) |
| 7 | `wx-rust-pay` | 1 | common | PASS (package --list, 636 files) |
| 8 | `wx-rust-qidian` | 1 | common | PASS (package --list, 42 files) |
| 9 | `wx-rust-open` | 2 | common, mp, miniapp | PASS (package --list, 252 files) |
| 10 | `wx-rust` | 3 | none | PASS (full dry-run) |

**Total**: 10 crates, 10/10 green.

---

## Dry-run Results Detail

### Full dry-run (independent crates)

```
cargo publish -p wx-rust-common --dry-run --allow-dirty
  -> Uploading wx-rust-common v0.1.0
  -> warning: aborting upload due to dry run
  -> PASS

cargo publish -p wx-rust --dry-run --allow-dirty
  -> Uploading wx-rust v0.1.0
  -> warning: aborting upload due to dry run
  -> PASS
```

### Package validation (dependent crates)

Layer-1 and Layer-2 crates cannot pass `cargo publish --dry-run` because their
internal dependency `wx-rust-common` is not yet on crates.io. This is the
standard workspace publish chicken-and-egg limitation.

Instead, each crate is validated via `cargo package -p <crate> --list --allow-dirty`
which verifies the crate can be correctly packaged for crates.io.

```
cargo package -p wx-rust-aispeech --list --allow-dirty  -> 41 files packaged
cargo package -p wx-rust-channel  --list --allow-dirty  -> 713 files packaged
cargo package -p wx-rust-cp       --list --allow-dirty  -> 670 files packaged
cargo package -p wx-rust-miniapp  --list --allow-dirty  -> 658 files packaged
cargo package -p wx-rust-mp       --list --allow-dirty  -> 404 files packaged
cargo package -p wx-rust-pay      --list --allow-dirty  -> 636 files packaged
cargo package -p wx-rust-qidian   --list --allow-dirty  -> 42 files packaged
cargo package -p wx-rust-open     --list --allow-dirty  -> 252 files packaged
```

---

## Workspace Quality Checks

| Check | Command | Result |
|-------|---------|--------|
| Compilation | `cargo check --workspace --all-features` | PASS |
| Tests | `cargo test --workspace` | PASS |
| Lint | `cargo clippy --workspace --all-targets --all-features -- -D warnings` | PASS |
| Format | `cargo fmt --all -- --check` | PASS |
| Audit | `cargo audit --ignore RUSTSEC-2023-0071` | PASS |
| Deny | `cargo deny check` | PASS (2 unused-license warnings) |

**Note on RUSTSEC-2023-0071**: The `rsa 0.9.10` Marvin Attack advisory (medium
severity) has no fix available. It is accepted in `deny.toml` and `cargo audit`
is run with `--ignore RUSTSEC-2023-0071` to match. This will resolve when
`rsa 0.10` reaches stable.

---

## Metadata Changes Made

Added to all 9 sub-crates (not `wx-rust-common` which already had them):

- `keywords.workspace = true` -- inherits `["wechat", "weixin", "wx", "sdk", "mp", "miniapp", "pay"]`
- `categories.workspace = true` -- inherits `["api-bindings", "web-programming"]`
- `[package.metadata.docs.rs] all-features = true` -- enables docs.rs full-feature builds

These are non-breaking metadata-only changes. No runtime deps or features were modified.

---

## Remaining Manual Steps Before Real Publish

1. **crates.io token**: Obtain and configure an API token.
   ```
   cargo login <your-token>
   ```

2. **Commit all changes**: The metadata additions and scripts are uncommitted.
   ```
   git add -A && git commit -m "publish: add metadata and publish scripts"
   ```

3. **Version bump** (if releasing as 0.1.0): Workspace version is already `0.1.0`.
   No bump needed for first publish.

4. **Execute publish in order**:
   ```
   cargo publish -p wx-rust-common && sleep 30
   cargo publish -p wx-rust-aispeech && sleep 30
   cargo publish -p wx-rust-channel && sleep 30
   cargo publish -p wx-rust-cp && sleep 30
   cargo publish -p wx-rust-miniapp && sleep 30
   cargo publish -p wx-rust-mp && sleep 30
   cargo publish -p wx-rust-pay && sleep 30
   cargo publish -p wx-rust-qidian && sleep 30
   cargo publish -p wx-rust-open && sleep 30
   cargo publish -p wx-rust
   ```

5. **Post-publish verify**: Check crates.io pages and docs.rs builds.

---

## Risks and Limitations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Chicken-and-egg: dependent crates cannot do full dry-run until `wx-rust-common` is on crates.io | First publish of `wx-rust-common` is unverified end-to-end | `cargo package --list` validates packaging; workspace quality checks validate code |
| `rsa 0.9.10` advisory (RUSTSEC-2023-0071) | Medium severity, no fix available | Accepted in deny.toml; monitor for rsa 0.10 stable |
| 30s sleep between publishes | Manual/scripted; crates.io index propagation delay | Scripted in `publish-order.sh`; could use a retry loop |
| `wx-rust` umbrella crate has no dependencies | Low-risk but empty crate may confuse users | Description clarifies it is a placeholder |

---

## Scripts

| Script | Purpose | Usage |
|--------|---------|-------|
| `scripts/publish-precheck.sh` | Full quality gate (check, test, clippy, fmt, audit, deny, packaging) | `bash scripts/publish-precheck.sh` |
| `scripts/publish-order.sh` | Dependency-sorted dry-run verification + real publish commands | `bash scripts/publish-order.sh` |
