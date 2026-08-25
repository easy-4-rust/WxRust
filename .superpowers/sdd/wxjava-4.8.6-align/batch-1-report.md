# Batch 1 Report: 点金计划（GoldPlanService）

## 对齐 Java 源
- `GoldPlanService.java` — 7 个方法
- `GoldPlanServiceImpl.java` — 实现（POST/PATCH v3 + JSON body）
- `GoldPlanResult.java` — 响应 bean（`sub_mchid` 字段）

## 新增文件清单

| 文件 | 对应 Java 类型 |
|------|---------------|
| `crates/wx-rust-pay/src/bean/goldplan/mod.rs` | `bean.goldplan` 包 |
| `crates/wx-rust-pay/src/bean/goldplan/gold_plan_result.rs` | `GoldPlanResult` |
| `crates/wx-rust-pay/src/api/gold_plan_service.rs` | `GoldPlanService` trait |
| `crates/wx-rust-pay/src/api/impl/gold_plan_service_impl.rs` | `GoldPlanServiceImpl` |
| `crates/wx-rust-pay/tests/gold_plan_test.rs` | 测试（10 个） |

## 修改文件清单

| 文件 | 变更 |
|------|------|
| `crates/wx-rust-pay/src/bean/mod.rs` | +`pub mod goldplan` + `pub use GoldPlanResult` |
| `crates/wx-rust-pay/src/api/mod.rs` | +`pub mod gold_plan_service` + `pub use GoldPlanService` |
| `crates/wx-rust-pay/src/api/impl/mod.rs` | +`pub mod gold_plan_service_impl` |
| `crates/wx-rust-pay/src/api/wx_pay_service.rs` | +`GoldPlanService` import + `fn gold_plan_service()` getter |
| `crates/wx-rust-pay/src/api/impl/sub_service_bundle.rs` | +`gold_plan` 字段 + `GoldPlanServiceImpl::new()` 装配 |
| `crates/wx-rust-pay/src/api/impl/base_wx_pay_service_impl.rs` | +`GoldPlanService` import + `gold_plan_service()` getter 实现 |

## GoldPlanService trait — 7 个方法

| 方法 | HTTP | 路径 | 返回 |
|------|------|------|------|
| `open_gold_plan` | POST | `/v3/goldplan/merchants/changegoldplanstatus` | `GoldPlanResult` |
| `close_gold_plan` | POST | `/v3/goldplan/merchants/changegoldplanstatus` | `GoldPlanResult` |
| `open_custom_page` | POST | `/v3/goldplan/merchants/changecustompagestatus` | `GoldPlanResult` |
| `close_custom_page` | POST | `/v3/goldplan/merchants/changecustompagestatus` | `GoldPlanResult` |
| `set_advertising_industry_filter` | POST | `/v3/goldplan/merchants/set-advertising-industry-filter` | `()` |
| `open_advertising_show` | PATCH | `/v3/goldplan/merchants/open-advertising-show` | `()` |
| `close_advertising_show` | POST | `/v3/goldplan/merchants/close-advertising-show` | `()` |

## 测试覆盖 — 10 个测试

| 测试 | 覆盖 |
|------|------|
| `test_gold_plan_result_serde` | bean 反序列化 |
| `test_gold_plan_result_empty` | 空 JSON 边界 |
| `test_gold_plan_result_round_trip` | 序列化 round-trip |
| `test_open_gold_plan` | open_gold_plan + body 断言 |
| `test_close_gold_plan` | close_gold_plan + CLOSE 断言 |
| `test_open_custom_page` | open_custom_page + path 断言 |
| `test_set_advertising_industry_filter` | set filter + 数组断言 |
| `test_open_advertising_show` | PATCH 方法 + 过滤标签 |
| `test_open_advertising_show_without_filters` | 无过滤标签边界 |
| `test_close_advertising_show` | close + body 断言 |

## 质量门禁

- `cargo clippy --package wx-rust-pay --all-targets -- -D warnings`: PASS
- `cargo test --workspace`: 2305 passed, 0 failed
- 新增依赖: 0（httpmock 已有）
- 其他 crate 变更: 0
