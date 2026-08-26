# 语义审计批次 A：channel 5 个子服务逐方法对齐

审计日期：2026-08-27
审计范围：ewaybill / gift / supplier / qic / kf
审计标准：URL 逐字符、参数字段名/类型、响应解析目标类型、特殊逻辑（GET/POST、包装层、multipart）

---

## 1. Ewaybill 服务（16 方法）

| 方法 | URL 对齐 | 参数对齐 | 响应对齐 | HTTP 方法 | 状态 |
|------|---------|---------|---------|----------|------|
| get_template_config | OK | OK (空 JSON) | TemplateConfigResponse | POST | PASS |
| create_template | OK | OK (TemplateCreateRequest) | TemplateIdResponse | POST | PASS |
| delete_template | OK | OK (template_id) | WxChannelBaseResponse | POST | PASS |
| update_template | OK | OK (TemplateUpdateRequest) | WxChannelBaseResponse | POST | PASS |
| get_template | OK | OK (template_code) | TemplateInfoResponse | POST | PASS |
| get_template_by_id | OK | OK (template_id) | TemplateInfoResponse | POST | PASS |
| get_account | OK | OK (空 JSON) | AccountInfoResponse | POST | PASS |
| get_delivery_list | OK | OK (空 JSON) | DeliveryListResponse | POST | PASS |
| pre_create_order | OK | OK (PreCreateRequest) | PreCreateResponse | POST | PASS |
| create_order | OK | OK (CreateOrderRequest) | CreateOrderResponse | POST | PASS |
| add_sub_order | OK | OK (AddSubOrderRequest) | WxChannelBaseResponse | POST | PASS |
| cancel_order | OK | OK (PrintOrderRequest) | WxChannelBaseResponse | POST | PASS |
| get_order | OK | OK (ewaybill_order_id) | OrderDetailResponse | POST | PASS |
| get_print_content | OK | OK (ewaybill_order_id + template_id) | PrintContentResponse | POST | PASS |
| print_order | OK | OK (PrintOrderRequest) | WxChannelBaseResponse | POST | PASS |
| batch_print_order | OK | OK (BatchPrintOrderRequest) | WxChannelBaseResponse | POST | PASS |

**结论：全部 16 方法 PASS，无需修复。**

---

## 2. Gift 服务（9 方法）

| 方法 | URL 对齐 | 参数对齐 | 响应对齐 | HTTP 方法 | 状态 |
|------|---------|---------|---------|----------|------|
| add_gift_product | OK | OK (GiftProductInfo 直接) | GiftProductAddResponse | POST | PASS |
| update_gift_product | OK | OK (GiftProductInfo 直接) | WxChannelBaseResponse | POST | PASS |
| set_product_as_gift | OK | OK (product_id) | WxChannelBaseResponse | POST | PASS |
| get_gift_product | OK | OK (product_id) | GiftProductGetResponse | POST | PASS |
| list_gift_product | OK | OK (GiftProductListParam) | GiftProductListResponse | POST | PASS |
| update_gift_stock | OK | OK (product_id/sku_id/diff_type/num) | WxChannelBaseResponse | POST | PASS |
| add_gift_activity | OK | **修复** (需 GiftActivityAddParam 包装) | GiftActivityAddResponse | POST | **FIXED** |
| delete_gift_activity | OK | OK (activity_id) | WxChannelBaseResponse | POST | PASS |
| stop_gift_activity | OK | OK (activity_id) | WxChannelBaseResponse | POST | PASS |

**修复项：**
- `add_gift_activity`：Java 用 `GiftActivityAddParam` 包装 `GiftActivityInfo`，序列化为 `{"gift_activity": {...}}`。Rust 之前直接序列化 `GiftActivityInfo`，缺少 `gift_activity` 包装层。
- 新增 `GiftActivityAddParam` bean（`bean/product/gift_activity_add_param.rs`）
- 更新 `wx_channel_gift_service_impl.rs` 使用 `GiftActivityAddParam::new(info)`

---

## 3. Supplier 服务（13 方法）

| 方法 | URL 对齐 | 参数对齐 | 响应对齐 | HTTP 方法 | 状态 |
|------|---------|---------|---------|----------|------|
| get_supplier_list_default | OK | OK (委托 get_supplier_list) | SupplierListResponse | POST | PASS |
| get_supplier_list | OK | OK (page_size/next_key) | SupplierListResponse | POST | PASS |
| get_distribute | OK | OK (空 JSON) | DistributeTypeResponse | POST | PASS |
| set_manually_distribute | OK | OK (空 JSON) | WxChannelBaseResponse | POST | PASS |
| set_all_distribute | OK | OK (supplier_id) | WxChannelBaseResponse | POST | PASS |
| set_product_distribute | OK | OK (ProductDistributeRequest) | WxChannelBaseResponse | POST | PASS |
| get_product_default_distribute | OK | OK (product_id) | SupplierInfoResponse | POST | PASS |
| get_product_list | OK | OK (supplier_id) | ProductListResponse | POST | PASS |
| assign_order | OK | OK (DropshipAssignRequest) | DropshipResponse | POST | PASS |
| cancel_dropship | OK | OK (order_id) | WxChannelBaseResponse | POST | PASS |
| get_dropship | OK | OK (order_id) | DropshipDetailResponse | POST | PASS |
| list_dropship | OK | OK (DropshipListRequest) | DropshipListResponse | POST | PASS |
| search_dropship | OK | OK (DropshipSearchRequest) | DropshipListResponse | POST | PASS |

**结论：全部 13 方法 PASS，无需修复。**

---

## 4. Qic 服务（6 方法）

| 方法 | URL 对齐 | 参数对齐 | 响应对齐 | HTTP 方法 | 状态 |
|------|---------|---------|---------|----------|------|
| get_inspect_config | OK | OK (无参数) | InspectConfigResponse | **修复** GET | **FIXED** |
| get_submit_config_with_order | OK | **修复** (query param) | SubmitConfigResponse | **修复** GET | **FIXED** |
| get_submit_config | OK | OK (无参数) | SubmitConfigResponse | **修复** GET | **FIXED** |
| print_inspect_code | OK | OK (order_id) | InspectCodeResponse | POST | PASS |
| submit_inspect_info | OK | OK (SubmitInspectRequest) | WxChannelBaseResponse | POST | PASS |
| register_logistics | OK | OK (RegisterLogisticsRequest) | WxChannelBaseResponse | POST | PASS |

**修复项：**
- `get_inspect_config`：Java 用 `shopService.get(GET_INSPECT_CONFIG_URL, null)`，Rust 错误地用 `svc.post(url, "{}")`。改为 `svc.get(url, "")`。
- `get_submit_config`：Java 用 `shopService.get(GET_SUBMIT_CONFIG_URL, null)`，Rust 错误地用 `svc.post(url, "{}")`。改为 `svc.get(url, "")`。
- `get_submit_config_with_order`：Java 用 `shopService.get(url, "order_id=" + orderId)`，Rust 错误地用 `svc.post(url, body)`。改为 `svc.get(url, &format!("order_id={order_id}"))`。

---

## 5. Kf 服务（2 方法）

| 方法 | URL 对齐 | 参数对齐 | 响应对齐 | HTTP 方法 | 状态 |
|------|---------|---------|---------|----------|------|
| upload_media | OK | **修复** (multipart 实现) | **修复** (WxChannelKfCosUploadResponse) | POST (upload) | **FIXED** |
| send_message | OK | OK (WxChannelKfSendMsgParam) | WxChannelKfSendMsgResponse | POST | PASS |

**修复项：**
- `upload_media`：之前返回 `Err("文件上传暂未实现")`。Java 用 `CommonUploadParam.fromBytes("file", fileName, file).addFormField("open_id", openId).addFormField("msg_type", msgType)` + `channelService.upload(COS_UPLOAD_URL, uploadParam)`。
- 实现：使用 `CommonUploadData::new(Some(file_name), file)` + `CommonUploadParam::with_form_fields("file", data, form_fields)` + `svc.upload(url::COS_UPLOAD_URL, param)`。
- 新增 `WxChannelKfCosUploadResponse` bean（`bean/kf/wx_channel_kf_cos_upload_response.rs`），包含 `err_code`/`err_msg`/`cos_url` 字段。
- 响应解析后返回 `resp.cos_url`（对齐 Java `ResponseUtils.decode(...).getCosUrl()`）。

---

## 子服务 getter 补齐

5 个服务之前未注册到 `WxChannelService` trait 的子服务 getter 体系中。本次补齐：

- `wx_channel_service.rs`：新增 `ewaybill_service()` / `gift_service()` / `supplier_service()` / `qic_service()` / `kf_service()` 5 个 trait 方法（默认返回 `None`）
- `wx_channel_service_impl.rs`：`SubServices` 结构体新增 5 个字段 + 构造器初始化 + getter 覆写

---

## 修复清单

| # | 文件 | 修复内容 |
|---|------|---------|
| 1 | `bean/product/gift_activity_add_param.rs` | 新增：`GiftActivityAddParam` 包装层 |
| 2 | `bean/product/mod.rs` | 新增模块声明 + re-export |
| 3 | `api/impl/wx_channel_gift_service_impl.rs` | `add_gift_activity` 使用 `GiftActivityAddParam::new(info)` |
| 4 | `api/impl/wx_channel_qic_service_impl.rs` | `get_inspect_config` / `get_submit_config` / `get_submit_config_with_order` 从 POST 改为 GET |
| 5 | `bean/kf/wx_channel_kf_cos_upload_response.rs` | 新增：`WxChannelKfCosUploadResponse` bean |
| 6 | `bean/kf/mod.rs` | 新增模块声明 + re-export |
| 7 | `api/impl/wx_channel_kf_service_impl.rs` | `upload_media` 实现 multipart 上传 |
| 8 | `api/wx_channel_service.rs` | 新增 5 个子服务 getter trait 方法 |
| 9 | `api/impl/wx_channel_service_impl.rs` | `SubServices` + 构造器 + getter 覆写 |

## 新增测试

| # | 测试名 | 覆盖点 |
|---|--------|--------|
| 1 | qic_get_inspect_config_uses_get | GET 方法 + URL 路径 |
| 2 | qic_get_submit_config_uses_get | GET 方法 + URL 路径 |
| 3 | qic_get_submit_config_with_order_uses_get | GET 方法 + query 参数 |
| 4 | qic_print_inspect_code_uses_post | POST + order_id |
| 5 | qic_submit_inspect_info_uses_post | POST 方法 |
| 6 | qic_register_logistics_uses_post | POST 方法 |
| 7 | gift_add_gift_activity_wraps_in_param | gift_activity 包装 + 字段验证 |
| 8 | gift_add_gift_product_sends_info_directly | 无包装层 |
| 9 | gift_delete_gift_activity_sends_activity_id | activity_id |
| 10 | gift_stop_gift_activity_sends_activity_id | activity_id |
| 11 | gift_update_gift_stock_sends_all_fields | 4 字段 |
| 12 | supplier_get_supplier_list_sends_params | page_size + next_key |
| 13 | supplier_get_distribute_sends_empty | 空 JSON |
| 14 | supplier_set_manually_distribute | 空 JSON |
| 15 | supplier_set_all_distribute_sends_supplier_id | supplier_id |
| 16 | supplier_cancel_dropship_sends_order_id | order_id |
| 17 | supplier_get_dropship_sends_order_id | order_id |
| 18 | ewaybill_get_template_config | URL |
| 19 | ewaybill_create_template | 请求体 |
| 20 | ewaybill_delete_template_sends_template_id | template_id |
| 21 | ewaybill_get_account | URL |
| 22 | ewaybill_get_delivery_list | URL |
| 23 | ewaybill_create_order | 请求体 |
| 24 | ewaybill_get_order_sends_order_id | ewaybill_order_id |
| 25 | ewaybill_get_print_content_sends_both_ids | 双 ID |
| 26 | ewaybill_print_order | URL |
| 27 | ewaybill_batch_print_order | URL |
| 28 | kf_send_message_sends_to_correct_url | URL + 请求体 |

**新增测试数：28**
