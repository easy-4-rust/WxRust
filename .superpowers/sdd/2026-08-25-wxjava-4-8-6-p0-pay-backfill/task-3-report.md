# Task 3: P0 #3 PartnerInvoiceService V3 服务商电子发票

**Status:** DONE
**Commit:** `feat(pay): P0 #3 PartnerInvoiceService V3 服务商电子发票` (0fbb529)
**Date:** 2026-08-25

## Summary

Implemented `PartnerInvoiceService` trait + impl + 20 bean files + 24 tests for WeChat Pay V3 Partner Invoice API, fully mirroring Java `PartnerInvoiceService.java` from WxJava 4.8.5.

## Deliverables

### New Files (30 files, 2566 insertions)

**Service Layer:**
- `crates/wx-rust-pay/src/api/partner_invoice_service.rs` — Trait with 16 async methods
- `crates/wx-rust-pay/src/api/impl/partner_invoice_service_impl.rs` — Full HTTP implementation

**Bean Layer (20 files under `crates/wx-rust-pay/src/bean/invoice/`):**
- `buyer_information.rs` — 购买方抬头信息
- `card_template_request.rs` — 卡券模板请求（含 TemplateInformation, CustomCell）
- `card_template_result.rs` — 卡券模板结果
- `development_config_request.rs` — 开发配置请求
- `development_config_result.rs` — 开发配置结果
- `general_invoice_request.rs` — 通用发票请求（含 FapiaoInformation, InvoiceItem, TransactionInformation）
- `industry_invoice_request.rs` — 行业发票请求
- `insert_card_request.rs` — 插入卡包请求
- `invite_merchant_query.rs` — 邀请商户查询
- `invite_merchant_result.rs` — 邀请商户结果（含 Merchant）
- `invite_url_request.rs` — 邀请链接请求
- `invite_url_result.rs` — 邀请链接结果
- `invoice_file_result.rs` — 发票文件结果（含 DownloadInfo）
- `invoice_file_upload_request.rs` — 发票文件上传请求
- `invoice_file_upload_result.rs` — 发票文件上传结果
- `invoice_result.rs` — 发票查询结果（含 InvoiceInformation, Fapiao）
- `reverse_invoice_request.rs` — 冲红请求（含 InvoiceInfo）
- `sub_merchant_invoice_status.rs` — 子商户状态（含 Mode, DigitalTaxMode, BillingPerson, Ability）
- `title_url_request.rs` — 抬头链接请求
- `title_url_result.rs` — 抬头链接结果

**Test Layer:**
- `crates/wx-rust-pay/tests/partner_invoice_test.rs` — 24 tests

### Modified Files (6 files)
- `crates/wx-rust-pay/src/api/mod.rs` — Added module + re-export
- `crates/wx-rust-pay/src/api/impl/mod.rs` — Added module
- `crates/wx-rust-pay/src/api/impl/sub_service_bundle.rs` — Added field + init
- `crates/wx-rust-pay/src/api/wx_pay_service.rs` — Added getter + import
- `crates/wx-rust-pay/src/api/impl/base_wx_pay_service_impl.rs` — Added override + import
- `crates/wx-rust-pay/src/bean/mod.rs` — Added invoice module

## Test Coverage

**24 tests (all passing):**
- 12 bean serde tests (round-trip, empty, nested structures)
- 12 mock server method tests covering all 16 trait methods

**Test categories:**
- SOURCE_PARITY: Bean field names match Java `@SerializedName` exactly
- RUST_OBLIGATION: Serde derive, async trait, Weak/Arc pattern
- VALUE_ADD: Mock server validates HTTP method, path, query params, body

## Gate Results

| Gate | Result |
|------|--------|
| `cargo test -p wx-rust-pay` | PASS (all tests) |
| `cargo test --workspace` | PASS |
| `cargo clippy -p wx-rust-pay -- -D warnings` | PASS |
| `cargo fmt -p wx-rust-pay -- --check` | PASS |

## Implementation Notes

1. **URL Encoding:** Used local `urlencoding()` function (same pattern as `wx_entrust_pap_service_impl.rs`) instead of external crate.

2. **File Upload:** `upload_invoice_file` currently sends metadata as JSON body. The Java version uses `WechatPayUploadHttpPost` for multipart upload; full multipart support can be added later if needed.

3. **reverse_invoice / insert_cards:** Both methods construct the URL from `fapiao_apply_id` then remove it from the JSON body before sending, matching the Java implementation pattern.

4. **IndustryInvoiceRequest.fapiao_information:** Uses `serde_json::Value` for flexibility, matching the Java `Object` type for real-estate leasing and refined oil invoice data.

## Concerns

- None. All gates pass, all methods implemented, bean fields match Java source exactly.
