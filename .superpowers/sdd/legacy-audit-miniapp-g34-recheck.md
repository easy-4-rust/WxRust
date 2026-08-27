# wx-rust-miniapp g3/g4 分组深度重审报告

## 审计日期
2026-08-27

## 审计范围
g3（电商服务组，20 个 impl 文件）+ g4（能力服务组，16 个 impl 文件），
对照 Java 源码 `WxJava/weixin-java-miniapp/src/main/java/cn/binarywang/wx/miniapp/api/impl/`
及 `WxMaApiUrlConstants.java`（1058 行）逐方法三向核对。

## 审计方法
1. **URL 逐字符**：Rust `url_g3_shop.rs`/`url_g4_ability.rs` 函数输出 == Java 常量字面量
2. **参数字段名**：Rust bean `#[serde(rename)]` == Java `@SerializedName`
3. **响应字段**：Rust 反序列化目标 == Java Result bean
4. **GET/POST 与特殊逻辑**：请求方式、签名注入、错误处理分支

## 审计结果

### 逐服务覆盖（g3 电商组）

| 服务 | Java 方法数 | Rust 方法数 | 缺陷数 | 状态 |
|------|------------|------------|--------|------|
| `wx_ma_shop_account_service_impl` | 4 | 4 | 0 | PASS |
| `wx_ma_shop_after_sale_service_impl` | 13 | 13 | 0 | PASS |
| `wx_ma_shop_audit_service_impl` | 4 | 4 | 0 | PASS |
| `wx_ma_shop_cat_service_impl` | 1 | 1 | 0 | PASS |
| `wx_ma_shop_coupon_service_impl` | 10 | 10 | 0 | PASS |
| `wx_ma_shop_delivery_service_impl` | 3 | 3 | 0 | PASS |
| `wx_ma_shop_img_service_impl` | 1 | 1 | 0 | PASS |
| `wx_ma_shop_order_service_impl` | 6 | 6 | 0 | PASS |
| `wx_ma_shop_pay_service_impl` | 3 | 3 | 0 | PASS |
| `wx_ma_shop_register_service_impl` | 4 | 4 | 0 | PASS |
| `wx_ma_shop_sharer_service_impl` | 7 | 7 | 0 | PASS |
| `wx_ma_shop_spu_service_impl` | 9 | 9 | 0 | PASS |
| `wx_ma_product_service_impl` | 16 | 16 | 0 | PASS |
| `wx_ma_product_order_service_impl` | 8 | 8 | 0 | PASS |
| `wx_ma_order_management_service_impl` | 2 | 2 | 0 | PASS |
| `wx_ma_order_shipping_service_impl` | 9 | 9 | 0 | PASS |
| `wx_ma_express_delivery_return_service_impl` | 3 | 3 | 0 | PASS |
| `wx_ma_immediate_delivery_service_impl` | 12 | 12 | 0 | PASS |
| `wx_ma_employee_relation_service_impl` | 2 | 2 | 0 | PASS |
| `wx_ma_customservice_work_service_impl` | 3 | 3 | 0 | PASS |
| **g3 小计** | **120** | **120** | **0** | |

### 逐服务覆盖（g4 能力组）

| 服务 | Java 方法数 | Rust 方法数 | 缺陷数 | 状态 |
|------|------------|------------|--------|------|
| `wx_ma_live_service_impl` | 27 | 27 | 0 | PASS |
| `wx_ma_live_goods_service_impl` | 9 | 9 | 0 | PASS |
| `wx_ma_live_member_service_impl` | 3 | 3 | 0 | PASS |
| `wx_ma_cloud_service_impl` | 30 | 30 | 0 | PASS |
| `wx_ma_vod_service_impl` | 15 | 15 | 0 | PASS |
| `wx_ma_xpay_service_impl` | 29 | 29 | 0 | PASS |
| `wx_ma_marketing_service_impl` | 2 | 2 | 0 | PASS |
| `wx_ma_promotion_service_impl` | 14 | 14 | 0 | PASS |
| `wx_ma_intracity_service_impl` | 17 | 17 | 0 | PASS |
| `wx_ma_complaint_service_impl` | 11 | 11 | 0 | PASS |
| `wx_ma_device_subscribe_service_impl` | 6 | 6 | 0 | PASS |
| `wx_ma_face_service_impl` | 2 | 2 | 0 | PASS |
| `wx_ma_reimburse_invoice_service_impl` | 4 | 4 | 0 | PASS |
| `wx_ma_qrcode_jump_service_impl` | 4 | 4 | 0 | PASS |
| `wx_ma_ocr_service_impl` | 7 | 7 | 1 | FIXED |
| `wx_ma_img_proc_service_impl` | 5 | 5 | 0 | PASS |
| **g4 小计** | **185** | **185** | **1** | |

### 缺陷清单

#### 缺陷 1：OCR 缺少 `menu` 菜单识别方法（URL + trait + impl 三层缺失）

- **文件**：
  - `crates/wx-rust-miniapp/src/enums/url_g4_ability.rs`（URL 常量）
  - `crates/wx-rust-common/src/service/wx_ocr_service.rs`（common trait）
  - `crates/wx-rust-miniapp/src/api/impl/wx_ma_ocr_service_impl.rs`（impl）
- **问题**：Java `WxOcrService` 有 7 个 URL 版方法（idCard/bankCard/driving/drivingLicense/bizLicense/comm/**menu**），
  Rust common trait 只有 6 个，缺少 `menu` 方法。URL 常量 `Ocr.MENU`（`/cv/ocr/menu?img_url=%s`）
  和 `Ocr.FILE_MENU`（`/cv/ocr/menu`）也缺失。`WxOcrMenuResult` bean 已存在但未被使用。
- **影响**：调用方无法使用菜单 OCR 识别功能。
- **修复**：
  1. `url_g4_ability.rs`：添加 `menu_url()` 和 `file_menu_url()` 函数
  2. `wx_ocr_service.rs`：trait 添加 `async fn ocr_menu()` 方法，导入 `WxOcrMenuResult`
  3. `wx_ma_ocr_service_impl.rs`：实现 `ocr_menu()` 方法，导入 `WxOcrMenuResult`

#### URL 常量补全（即时配送域 5 个缺失 URL）

- **文件**：`crates/wx-rust-miniapp/src/enums/url_g3_shop.rs`
- **问题**：Java `InstantDelivery` 接口有 5 个 URL 常量未在 Rust 中定义：
  - `GET_ALL_IMME_DELIVERY`（`/cgi-bin/express/local/business/delivery/getall`）
  - `PRE_ADD_ORDER`（`/cgi-bin/express/local/business/order/pre_add`）
  - `RE_ORDER`（`/cgi-bin/express/local/business/order/readd`）
  - `ADD_TIP`（`/cgi-bin/express/local/business/order/addtips`）
  - `PRE_CANCEL_ORDER`（`/cgi-bin/express/local/business/order/precancel`）
- **影响**：当前无对应 impl 方法使用这些 URL，但 URL 常量缺失会导致未来扩展时需要额外查找。
- **修复**：在 `instant_delivery` 模块中补充 5 个 URL 函数。

### 与上轮浅审的差异发现

| 项目 | 上轮浅审结论 | 本轮深度重审结论 |
|------|------------|----------------|
| OCR menu 方法 | 未发现（浅审自述"Java source not available"） | **缺失**：URL + trait + impl 三层均缺 |
| InstantDelivery URL 常量 | 未覆盖 | 补全 5 个缺失 URL（无对应 impl，仅常量） |
| g3 全部 120 方法 | 浅审 PASS | 深度核对 PASS（逐方法对照 Java 源码确认） |
| g4 全部 185 方法 | 浅审 PASS | 深度核对发现 1 处缺陷（OCR menu）并修复 |

### 无缺陷确认（关键域逐核对摘要）

**交易组件 Shop（12 个子服务，~60 方法）**：
- 全部 URL 路径与 Java `Shop.*` 常量逐字符一致（含 `recieve` 原拼写）
- 全部 POST 方法，请求体序列化方式与 Java 一致
- `shop_aftersale` 的 `update`/`ec_update` 双版本 URL 正确区分

**标准版商品 Product（4 个子域，~32 方法）**：
- SPU/SKU/Order/OTHER 全部 URL 与 Java `Product.*` 常量一致
- `update_stock_url` 路径 `/product/stock/update`（非 `/product/sku/update_stock`）与 Java 一致

**即时配送 InstantDelivery（12 方法）**：
- `build_delivery_sign` 签名逻辑与 Java `WxMaDeliveryBaseRequest.getDeliverySign()` 一致
- `parse` 方法双层错误检查（微信 errcode + 运力方 resultcode）与 Java 一致

**直播 Broadcast（3 个子服务，~39 方法）**：
- Room/Goods/Role 全部 URL 与 Java `Broadcast.*` 常量一致
- `get_live_infos` 分页循环逻辑与 Java 一致（含 100ms 间隔）

**云开发 Cloud（~30 方法）**：
- 全部 `/tcb/*` URL 与 Java `Cloud.*` 常量一致
- `invoke_cloud_function` 的 env 参数处理与 Java 一致（忽略入参，取配置值）

**虚拟支付 XPay（29 方法）**：
- 全部 `/xpay/*` URL 与 Java `XPay.*` 常量一致
- `pay_sig=%s`/`signature=%s` 占位符与 Java `String.format` 语义一致

**同城配送 Intracity（17 方法）**：
- 全部 `/cgi-bin/express/intracity/*` URL 与 Java `Intracity.*` 常量一致
- `queryStore` 私有方法的 3 个公开包装（listAll/queryByWx/queryByOut）与 Java 一致

**投诉 Complaint（11 方法）**：
- 全部 `/cgi-bin/miniapp/complaint/*` URL 与 Java `Complaint.*` 常量一致

## 测试结果

```
cargo test -p wx-rust-miniapp: 331 passed, 0 failed
cargo clippy -p wx-rust-miniapp -- -D warnings: clean
cargo fmt -p wx-rust-miniapp -- --check: clean
```

## 修复文件清单

1. `crates/wx-rust-miniapp/src/enums/url_g4_ability.rs` — 添加 `menu_url()` + `file_menu_url()`
2. `crates/wx-rust-miniapp/src/enums/url_g3_shop.rs` — 补全 5 个 InstantDelivery URL
3. `crates/wx-rust-common/src/service/wx_ocr_service.rs` — trait 添加 `ocr_menu()` 方法
4. `crates/wx-rust-miniapp/src/api/impl/wx_ma_ocr_service_impl.rs` — 实现 `ocr_menu()` 方法
