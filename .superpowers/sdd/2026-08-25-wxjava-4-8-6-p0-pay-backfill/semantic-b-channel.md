# 语义审计批次 B：favorite / limited_discount / product_assistant / product_stock / talent

## 审计总览

| Service | 方法数 | Java 接口方法数 | 对齐 | 发现问题 | 修复数 |
|---------|--------|-----------------|------|----------|--------|
| wx_channel_favorite_service | 1 | 1 | OK | 0 | 0 |
| wx_channel_limited_discount_service | 5 | 5 | OK | 0 | 0 |
| wx_channel_product_assistant_service | 6 | 6 | OK | 0 | 0 |
| wx_channel_product_stock_service | 4 | 4 | 3/4 OK | 3 | 3 |
| wx_talent_service | 4 | 4 | OK | 0 | 0 |
| **合计** | **20** | **20** | | **3** | **3** |

## 逐 Service 审计表

### 1. wx_channel_favorite_service (1/1 OK)

| 方法 | URL 匹配 | 参数匹配 | 响应解析 | 特殊逻辑 | 结论 |
|------|----------|----------|----------|----------|------|
| `get_favorite_count` | `/channels/ec/favorites/count/get` == Java `GET_FAVORITE_COUNT` | 请求体 `{}` == Java `post(url, "{}")` | `FavoriteCountResponse` 字段对齐 | 无 | OK |

### 2. wx_channel_limited_discount_service (5/5 OK)

| 方法 | URL 匹配 | 参数匹配 | 响应解析 | 结论 |
|------|----------|----------|----------|------|
| `add_limit_task` | `limiteddiscounttask/add` == Java | `LimitTaskParam` serde 序列化 == Java `JsonUtils.encode` | `LimitTaskAddResponse` 对齐 | OK |
| `list_limit_task` | `limiteddiscounttask/list/get` == Java | `page_size/next_key/status` 字段名 == Java `LimitTaskListParam` | `LimitTaskListResponse` 对齐 | OK |
| `stop_limit_task` | `limiteddiscounttask/stop` == Java | `{"task_id": "..."}` == Java 手拼 JSON | `WxChannelBaseResponse` 对齐 | OK |
| `delete_limit_task` | `limiteddiscounttask/delete` == Java | `{"task_id": "..."}` == Java 手拼 JSON | `WxChannelBaseResponse` 对齐 | OK |
| `update_limit_task` | `limiteddiscounttask/update` == Java | `LimitTaskUpdateParam` serde 序列化 == Java | `LimitTaskUpdateResponse` 对齐 | OK |

### 3. wx_channel_product_assistant_service (6/6 OK)

| 方法 | URL 匹配 | 参数匹配 | 响应解析 | 结论 |
|------|----------|----------|----------|------|
| `category_pre_check` | `categoryprecheck` == Java | `CategoryPreCheckParam` serde == Java | `CategoryPreCheckResponse` 对齐 | OK |
| `get_product_brand_recommend` | `productbrandrecommend` == Java | `ProductBrandRecommendParam` serde == Java | `ProductBrandRecommendResponse` 对齐 | OK |
| `external_product_mapping` | `externalproductmapping` == Java | serde == Java | `ExternalProductMappingResponse` 对齐 | OK |
| `external_product_mapping_new` | `externalproductmappingnew` == Java | serde == Java | `ExternalProductMappingNewResponse` 对齐 | OK |
| `begin_timing_sale` | `begintimingsale` == Java | serde == Java | `WxChannelBaseResponse` 对齐 | OK |
| `cancel_timing_sale` | `canceltimingsale` == Java | serde == Java | `WxChannelBaseResponse` 对齐 | OK |

### 4. wx_channel_product_stock_service (3/4 -> 4/4 修复后)

| 方法 | URL 匹配 | 参数匹配 | 响应解析 | 问题 | 修复 |
|------|----------|----------|----------|------|------|
| `update_stock` | `stock/update` == Java | `product_id/sku_id/diff_type/num` == Java `SkuStockParam` | `WxChannelBaseResponse` 对齐 | 无 | - |
| `get_sku_stock` | **BUG**: `/sku/stock/get` != Java `stock/get` | `product_id/sku_id` OK | `SkuStockResponse` 对齐 | URL 硬编码且路径错误 | 改用 `url::GET_STOCK_URL` 常量 |
| `get_sku_stock_batch` | **BUG**: `/sku/stock/batch/get` != Java `stock/batchget` | **BUG**: `product_ids`(复数) != Java `@JsonProperty("product_id")`(单数) | `SkuStockBatchResponse` 对齐 | URL + 字段名双错 | 改用 `url::GET_STOCK_BATCH_URL` + 修正字段名 |
| `get_stock_flow` | `stock/getflow` == Java | `StockFlowParam` serde == Java | `StockFlowResponse` 对齐 | 无 | - |

### 5. wx_talent_service (4/4 OK)

| 方法 | URL 匹配 | 参数匹配 | 响应解析 | 结论 |
|------|----------|----------|----------|------|
| `get_order_list` | `talent/get_order_list` == Java `GET_ORDER_LIST_URL` | `TalentOrderListParam` serde == Java | `TalentOrderListResponse` 对齐 | OK |
| `get_order_detail` | `talent/get_order_detail` == Java | serde == Java | `TalentOrderDetailResponse` 对齐 | OK |
| `get_window_product_list` | `talent/window/product/list/get` == Java | serde == Java | `TalentWindowProductListResponse` 对齐 | OK |
| `get_window_product_detail` | `talent/window/product/get` == Java | serde == Java | `TalentWindowProductDetailResponse` 对齐 | OK |

## 修复清单

### Fix 1: `url_product_stock.rs` 缺失 2 个 URL 常量

**文件**: `crates/wx-rust-channel/src/enums/url_product_stock.rs`

**问题**: 模块仅有 `UPDATE_STOCK_URL` 和 `GET_STOCK_FLOW_URL`，缺失 `GET_STOCK_URL` 和 `GET_STOCK_BATCH_URL`。

**修复**: 新增两个常量：
- `GET_STOCK_URL` = `https://api.weixin.qq.com/channels/ec/product/stock/get` (对应 Java `SPU_GET_STOCK_URL`)
- `GET_STOCK_BATCH_URL` = `https://api.weixin.qq.com/channels/ec/product/stock/batchget` (对应 Java `SPU_GET_STOCK_BATCH_URL`)

### Fix 2: `get_sku_stock` URL 硬编码且路径错误

**文件**: `crates/wx-rust-channel/src/api/impl/wx_channel_product_stock_service_impl.rs`

**问题**: URL 硬编码为 `https://api.weixin.qq.com/channels/ec/product/sku/stock/get`（含多余的 `/sku/` 段），与 Java `SPU_GET_STOCK_URL` (`/product/stock/get`) 不一致。

**修复**: 改用 `url::GET_STOCK_URL` 常量引用。

### Fix 3: `get_sku_stock_batch` URL 错误 + JSON 字段名错误

**文件**: `crates/wx-rust-channel/src/api/impl/wx_channel_product_stock_service_impl.rs`

**问题**:
1. URL 硬编码为 `https://api.weixin.qq.com/channels/ec/product/sku/stock/batch/get`，与 Java `SPU_GET_STOCK_BATCH_URL` (`/product/stock/batchget`) 不一致。
2. JSON 字段名为 `"product_ids"`（复数），但 Java `SkuStockBatchParam` 使用 `@JsonProperty("product_id")`（单数）。

**修复**:
1. 改用 `url::GET_STOCK_BATCH_URL` 常量引用。
2. JSON 字段名从 `"product_ids"` 改为 `"product_id"`。

## 新增测试

**文件**: `crates/wx-rust-channel/tests/semantic_b_batch_services.rs`

新增 **20 个测试**，覆盖全部 5 个 service 的 20 个方法：

| Service | 测试数 | 测试方法 |
|---------|--------|----------|
| favorite | 1 | `favorite_get_count` |
| limited_discount | 5 | `limited_discount_add_task`, `_list_task`, `_stop_task`, `_delete_task`, `_update_task` |
| product_assistant | 6 | `assistant_category_pre_check`, `_brand_recommend`, `_external_mapping`, `_external_mapping_new`, `_begin_timing_sale`, `_cancel_timing_sale` |
| product_stock | 4 | `stock_update`, `stock_get_sku_stock`, `stock_get_sku_stock_batch`, `stock_get_flow` |
| talent | 4 | `talent_get_order_list`, `_get_order_detail`, `_get_window_product_list`, `_get_window_product_detail` |

每个测试断言：
- 请求路径包含正确的 API endpoint（与 Java URL 常量逐字符对齐）
- 请求体包含正确的 JSON 字段名（与 Java bean `@SerializedName`/`@JsonProperty` 对齐）
- 响应能正确反序列化到目标类型

## 门禁结果

- `cargo test -p wx-rust-channel`: **435 passed**, 0 failed (>= 415 门禁通过)
- `cargo clippy -p wx-rust-channel --all-targets -- -D warnings`: clean
- `cargo fmt -p wx-rust-channel -- --check`: clean
