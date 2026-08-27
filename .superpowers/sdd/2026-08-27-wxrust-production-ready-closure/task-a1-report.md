# Task A1 Report: wx-rust-pay Deep Coverage

## Commit
- Hash: `f0f65c382627c46b740e0bb8a941690a45e9da58`
- Message: `test(pay): A1 深度覆盖——order/refund/bill/xml 分支（cov_pay_deep）`

## Files
- `crates/wx-rust-pay/tests/cov_pay_deep.rs` (2281 lines, 114 tests)

## Test Count
- **cov_pay_deep**: 114 tests (all pass)
- **workspace total**: 3415 tests (no regression, baseline was 3301)

## Coverage (llvm-cov)
| Metric | Baseline | After | Delta |
|--------|----------|-------|-------|
| Regions | 54.80% (5423 uncovered) | 55.13% (5384 uncovered) | +0.33% |
| Lines | 58.41% (3380 uncovered) | 58.71% (3356 uncovered) | +0.30% |
| Functions | 35.92% (1452 uncovered) | 36.23% (1445 uncovered) | +0.31% |

## Coverage Gap Analysis
Target was 75% line coverage. Current is 58.71%. Gap: ~16%.

### Root Cause
The main service trait implementations in `wx_pay_service.rs` (3400+ lines) contain large method bodies for:
- v2 full flows (unified_order, create_order, micropay, reverse_order, etc.) - most require p12 cert (`use_key=true`)
- v3 API methods (unified_order_v3, refund_v3, query_order_v3, etc.) - require RSA key setup
- Notification parsing with AES-GCM decryption

These methods can't be fully tested without p12 certificate configuration. The tests added cover:
- Request XML generation (check_and_sign + to_xml)
- Error paths and constraint checks
- Bill/fund flow parsing utilities
- Config management
- Notification parsing (v2 XML)

### What's Covered (114 tests)
1. **query_order 7 trade_state branches**: SUCCESS/REFUND/NOTPAY/CLOSED/REVOKED/USERPAYING/PAYERROR
2. **close_order**: OK + error code paths
3. **refund XML fields**: out_refund_no/total_fee/refund_fee assertions
4. **download_bill parsing**: ALL/SUCCESS/REFUND/RECHARGE_REFUND types, header stripping
5. **XML utilities**: root_children_map, expand_empty_elements, CDATA, special chars
6. **Error code mapping**: SIGN_ERROR/ORDERNOTEXIST/SYSTEMERROR/BANKERROR/USERPAYING
7. **WxPayException**: build/from_base_result/builder chain
8. **Config management**: add/remove/switchover/set_multi/get_config
9. **Notification parsing**: order notify, scan pay notify, refund notify
10. **Other services**: micropay, reverse_order, shorturl, authcode2openid, report, coupon, sandbox sign key

## Quality Gates
- [x] `cargo test -p wx-rust-pay --test cov_pay_deep` — 114 pass
- [x] `cargo test --workspace` — 3415 pass (no regression)
- [x] `cargo clippy -p wx-rust-pay --all-targets -- -D warnings` — clean
- [x] `cargo fmt --all -- --check` — clean
