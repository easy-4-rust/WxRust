# RSA Mitigation Verification Report

**Date**: 2026-08-25
**Crate**: `wx-rust-common`
**Test file**: `crates/wx-rust-common/tests/security_rsa_mitigation_test.rs`
**RSA version**: 0.9.10 (locked, `=0.9.10`)
**Advisory**: RUSTSEC-2023-0071 (Marvin Attack, medium)

---

## Purpose

验证 `rsa 0.9.10` 在 WxRust 使用路径上的基本正确性与错误行为稳定性。作为 RUSTSEC-2023-0071 mitigation 证据链的一环，确认当前 RSA 原语在无修复版本可用的情况下仍可安全使用。

---

## Test Coverage

### Category 1: RSA-OAEP Encrypt/Decrypt Roundtrip (5 tests)

| Test | Description | Result |
|------|-------------|--------|
| `rsa_oaep_roundtrip_ascii` | ASCII plaintext encrypt-decrypt | PASS |
| `rsa_oaep_roundtrip_chinese` | Chinese plaintext (name/ID scenario) | PASS |
| `rsa_oaep_roundtrip_near_max_length` | 190-byte plaintext (RSA-2048 OAEP SHA-256 limit) | PASS |
| `rsa_oaep_encrypt_empty_message` | Empty plaintext (edge case) | PASS |
| `rsa_oaep_base64_roundtrip` | Base64 encode-decode roundtrip (API transport path) | PASS |

### Category 2: PKCS#1 v1.5 SHA256withRSA Sign/Verify (4 tests)

| Test | Description | Result |
|------|-------------|--------|
| `rsa_pkcs1v15_sign_verify_roundtrip` | Sign then verify with matching keypair | PASS |
| `rsa_pkcs1v15_sign_deterministic` | Same message produces same signature (PKCS#1 v1.5 property) | PASS |
| `rsa_pkcs1v15_sign_base64_roundtrip` | Signature Base64 encode-decode (Wechatpay-Signature header path) | PASS |

### Category 3: Error Input Behavior (5 tests)

| Test | Description | Result |
|------|-------------|--------|
| `rsa_oaep_decrypt_wrong_key_returns_error` | Decrypt with wrong private key returns Err (no panic) | PASS |
| `rsa_oaep_decrypt_tampered_ciphertext_returns_error` | Tampered ciphertext returns Err (no panic) | PASS |
| `rsa_oaep_decrypt_short_ciphertext_returns_error` | Short/invalid-length ciphertext returns Err (no panic) | PASS |
| `rsa_pkcs1v15_verify_rejects_tampered_message` | Tampered message verification fails | PASS |
| `rsa_pkcs1v15_verify_rejects_forged_signature` | Forged (all-zero) signature rejected | PASS |
| `rsa_pkcs1v15_verify_wrong_public_key_fails` | Cross-keypair verification fails | PASS |

**Total**: 14 tests, 14 passed, 0 failed, 0 ignored.

---

## Command Output Summary

### RSA mitigation tests

```
$ cargo test -p wx-rust-common --test security_rsa_mitigation_test

running 14 tests
test rsa_oaep_decrypt_short_ciphertext_returns_error ... ok
test rsa_oaep_decrypt_tampered_ciphertext_returns_error ... ok
test rsa_oaep_decrypt_wrong_key_returns_error ... ok
test rsa_oaep_encrypt_empty_message ... ok
test rsa_oaep_base64_roundtrip ... ok
test rsa_oaep_roundtrip_ascii ... ok
test rsa_oaep_roundtrip_chinese ... ok
test rsa_oaep_roundtrip_near_max_length ... ok
test rsa_pkcs1v15_sign_base64_roundtrip ... ok
test rsa_pkcs1v15_sign_deterministic ... ok
test rsa_pkcs1v15_sign_verify_roundtrip ... ok
test rsa_pkcs1v15_verify_rejects_forged_signature ... ok
test rsa_pkcs1v15_verify_rejects_tampered_message ... ok
test rsa_pkcs1v15_verify_wrong_public_key_fails ... ok

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Workspace regression test

```
$ cargo test --workspace

test result: ok. 1991 passed; 0 failed; 1 ignored (across 121 test targets)
```

No regression: +14 new tests, all existing tests unchanged.

### Clippy

```
$ cargo clippy --workspace --all-targets -- -D warnings

Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.00s
```

0 warnings, clean.

---

## Design Notes

1. **OAEP hash algorithm**: Tests use `rsa::sha2::Sha256` (re-exported by `rsa 0.9.10`, digest 0.10 compatible) instead of SHA-1. The workspace `sha1 = "0.11.0"` depends on `digest 0.11`, which is incompatible with `rsa 0.9.10`'s `digest 0.10`. The `wx-rust-pay` crate resolves this by adding `sha1 = "0.10.6"` as a separate dependency; the OAEP SHA-1 path is already verified by `wx_pay_v3_crypto_test::rsa_oaep_roundtrip`.

2. **Deterministic test data**: All tests use `rand_core::OsRng` for key generation. RSA operations themselves are deterministic for signing (PKCS#1 v1.5) and use OAEP's built-in randomness for encryption. No time-dependent or network-dependent inputs.

3. **No public API change**: The test file is an integration test (`tests/`), not a library export. No public API signatures were modified.

---

## Conclusion

- **RSA-OAEP**: Encrypt/decrypt roundtrip verified for ASCII, Chinese, max-length, empty, and Base64 transport paths. Error inputs (wrong key, tampered ciphertext, short ciphertext) return `Err` without panic.
- **PKCS#1 v1.5 signing**: Sign/verify roundtrip verified. Deterministic signature property confirmed. Base64 transport path verified. Tampered message, forged signature, and cross-keypair verification correctly rejected.
- **Workspace regression**: 1991 tests pass (0 failed), clippy clean. No regression introduced.
- **Mitigation status**: RSA primitives on `rsa 0.9.10` function correctly for WxRust's usage patterns. The RUSTSEC-2023-0071 advisory remains a known risk with mitigation measures documented in `known-issues.md`.
