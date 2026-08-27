# channel 老存量 service 4.8.6 方法补齐报告

## 状态: DONE

## 补齐清单 (35 methods x 4 services)

### basic_service (3 methods)

| 方法 | URL | Bean 状态 |
|------|-----|-----------|
| `get_shop_h5_url` | `GET_SHOP_H5URL` | `ShopH5UrlResponse` -- 新建 |
| `get_shop_qr_code` | `GET_SHOP_QRCODE` | `ShopQrCodeResponse` -- 新建 |
| `get_shop_tag_link` | `GET_SHOP_TAGLINK` | `ShopTagLinkResponse` -- 新建 |

### order_service (13 methods)

| 方法 | URL | Bean 状态 |
|------|-----|-----------|
| `add_present_note` | `PRESENT_NOTE_ADD_URL` | `PresentNoteAddParam` -- 新建 |
| `get_present_sub_orders` | `PRESENT_SUB_ORDER_GET_URL` | `PresentSubOrderResponse` -- 新建 |
| `get_pre_shipment_change_sku` | `PRE_SHIPMENT_CHANGE_SKU_GET_URL` | `PreShipmentChangeSkuResponse` -- 新建 |
| `approve_pre_shipment_change_sku` | `PRE_SHIPMENT_CHANGE_SKU_APPROVE_URL` | 复用 `OrderIdParam` |
| `reject_pre_shipment_change_sku` | `PRE_SHIPMENT_CHANGE_SKU_REJECT_URL` | `PreShipmentChangeSkuRejectParam` -- 新建 |
| `apply_real_number` | `REAL_NUMBER_APPLY_URL` | 复用 `OrderIdParam` |
| `get_real_number_view_audit` | `REAL_NUMBER_VIEW_AUDIT_GET_URL` | `RealNumberViewAuditResponse` -- 新建 |
| `apply_virtual_number_again` | `VIRTUAL_NUMBER_APPLY_AGAIN_URL` | 复用 `OrderIdParam` |
| `delay_virtual_number` | `VIRTUAL_NUMBER_DELAY_URL` | 复用 `OrderIdParam` |
| `add_private_phone` | `ADD_PHONE_URL` | `PrivateNumberAddPhoneParam` -- 新建 |
| `send_private_phone_verify_code` | `SEND_VERIFY_CODE_URL` | `PrivateNumberSendVerifyCodeParam` -- 新建 |
| `get_private_phone` | `GET_PHONE_URL` | `PrivateNumberGetPhoneResponse` + `PrivateNumberPhoneInfo` -- 新建 |
| `compensation_delivery` | `DELIVERY_COMPENSATION_URL` | `OrderCompensationDeliveryParam` -- 新建 |

### after_sale_service (6 methods)

| 方法 | URL | Bean 状态 |
|------|-----|-----------|
| `list_guarantee_order` | `GUARANTEE_ORDER_LIST_URL` | `GuaranteeOrderListParam` + `GuaranteeOrderListResponse` -- 新建 |
| `get_guarantee_order` | `GUARANTEE_ORDER_GET_URL` | `GuaranteeOrderIdParam` + `GuaranteeOrderInfoResponse` -- 新建 |
| `accept_guarantee` | `GUARANTEE_ORDER_ACCEPT_URL` | 复用 `GuaranteeOrderIdParam` |
| `modify_guarantee` | `GUARANTEE_ORDER_MODIFY_URL` | `GuaranteeModifyRequest` -- 新建 |
| `proof_guarantee` | `GUARANTEE_ORDER_PROOF_URL` | `GuaranteeProofRequest` -- 新建 |
| `refuse_guarantee` | `GUARANTEE_ORDER_REFUSE_URL` | `GuaranteeRefuseRequest` -- 新建 |

### product_service (13 methods)

| 方法 | URL | Bean 状态 |
|------|-----|-----------|
| `get_product_scheme` | `SPU_SCHEME_URL` | `ProductSchemeParam` + `ProductSchemeResponse` -- 新建 |
| `classify_product_category` | `SPU_CATEGORY_CLASSIFY_URL` | `ProductCategoryClassifyParam` + `ProductCategoryClassifyResponse` -- 新建 |
| `begin_timing_sale` | `SPU_BEGIN_TIMING_SALE_URL` | `BeginTimingSaleParam` -- 已有，补 `task_id` 字段 |
| `cancel_timing_sale` | `SPU_CANCEL_TIMING_SALE_URL` | 内联 `{"product_id":..}` |
| `external_product_mapping` | `SPU_EXTERNAL_PRODUCT_MAPPING_URL` | `ExternalProductMappingParam` -- 已有 |
| `category_pre_check` | `SPU_CATEGORY_PRE_CHECK_URL` | `CategoryPreCheckParam` -- 已有 |
| `get_product_audit_strategy` | `SPU_AUDIT_STRATEGY_GET_URL` | `ProductAuditStrategyResponse` + `ProductAuditStrategyInfo` -- 新建 |
| `set_product_audit_strategy` | `SPU_AUDIT_STRATEGY_SET_URL` | `ProductAuditStrategySetParam` -- 新建 |
| `get_product_audit_quota` | `SPU_GET_AUDIT_QUOTA_URL` | `ProductAuditQuotaResponse` + `AuditQuota` -- 新建 |
| `external_product_mapping_new` | `SPU_EXTERNAL_PRODUCT_MAPPING_NEW_URL` | `ExternalProductMappingNewParam` -- 已有 |
| `product_brand_recommend` | `SPU_PRODUCT_BRAND_RECOMMEND_URL` | `ProductBrandRecommendParam` -- 已有 |
| `add_product_third_party_source` | `SPU_ADD_PRODUCT_THIRD_PARTY_SOURCE_URL` | `AddProductThirdPartySourceParam` + `AddProductThirdPartySourceResponse` -- 新建 |
| `get_stock_flow` | `SPU_GET_STOCK_FLOW_URL` | `StockFlowParam` + `StockFlowResponse` -- 已有 |

## 新建 Bean 统计

- shop: 3 (`ShopH5UrlResponse`, `ShopQrCodeResponse`, `ShopTagLinkResponse`)
- order: 10 (`PresentNoteAddParam`, `PresentSubOrderResponse`, `PreShipmentChangeSkuResponse`, `PreShipmentChangeSkuRejectParam`, `RealNumberViewAuditResponse`, `PrivateNumberAddPhoneParam`, `PrivateNumberSendVerifyCodeParam`, `PrivateNumberGetPhoneResponse`, `PrivateNumberPhoneInfo`, `OrderCompensationDeliveryParam`)
- after: 7 (`GuaranteeOrderIdParam`, `GuaranteeOrderListParam`, `GuaranteeOrderListResponse`, `GuaranteeOrderInfoResponse`, `GuaranteeModifyRequest`, `GuaranteeProofRequest`, `GuaranteeRefuseRequest` + 辅助结构体 `GuaranteeOrderDetail`, `GuaranteeProductInfo`, `GuaranteeOrderListItem`, `GuaranteeListItemProductInfo`)
- product: 10 (`ProductSchemeParam`, `ProductSchemeResponse`, `ProductCategoryClassifyParam`, `ProductCategoryClassifyResponse`, `ProductAuditStrategyInfo`, `ProductAuditStrategyResponse`, `ProductAuditStrategySetParam`, `ProductAuditQuotaResponse`, `AddProductThirdPartySourceParam`, `AddProductThirdPartySourceResponse` + 辅助 `AuditQuota`, `CategoryClassifyInfo`, `CategoryLevel`, `CategoryLevelInfo`)

## 测试

- 新增测试文件: `tests/channel_legacy_completion_test.rs`
- service mock 测试: 35 (每个方法 1 个)
- bean serde 测试: 19
- 总新增: 54 tests
- channel 总测试数: 517 (baseline 463 + 新增 54)
- 0 failed, clippy `-D warnings` clean, fmt clean

## 修改文件清单

### 新建文件 (30)
- `src/bean/shop/shop_h5_url_response.rs`
- `src/bean/shop/shop_qr_code_response.rs`
- `src/bean/shop/shop_tag_link_response.rs`
- `src/bean/order/present_note_add_param.rs`
- `src/bean/order/present_sub_order_response.rs`
- `src/bean/order/pre_shipment_change_sku_response.rs`
- `src/bean/order/pre_shipment_change_sku_reject_param.rs`
- `src/bean/order/real_number_view_audit_response.rs`
- `src/bean/order/private_number_phone_info.rs`
- `src/bean/order/private_number_add_phone_param.rs`
- `src/bean/order/private_number_send_verify_code_param.rs`
- `src/bean/order/private_number_get_phone_response.rs`
- `src/bean/order/order_compensation_delivery_param.rs`
- `src/bean/after/guarantee_order_id_param.rs`
- `src/bean/after/guarantee_order_list_param.rs`
- `src/bean/after/guarantee_order_info_response.rs`
- `src/bean/after/guarantee_order_list_response.rs`
- `src/bean/after/guarantee_modify_request.rs`
- `src/bean/after/guarantee_proof_request.rs`
- `src/bean/after/guarantee_refuse_request.rs`
- `src/bean/product/product_scheme_param.rs`
- `src/bean/product/product_scheme_response.rs`
- `src/bean/product/product_category_classify_param.rs`
- `src/bean/product/product_category_classify_response.rs`
- `src/bean/product/product_audit_strategy_info.rs`
- `src/bean/product/product_audit_strategy_response.rs`
- `src/bean/product/product_audit_strategy_set_param.rs`
- `src/bean/product/product_audit_quota_response.rs`
- `src/bean/product/add_product_third_party_source_param.rs`
- `src/bean/product/add_product_third_party_source_response.rs`
- `tests/channel_legacy_completion_test.rs`

### 修改文件 (12)
- `src/bean/shop/mod.rs`
- `src/bean/order/mod.rs`
- `src/bean/after/mod.rs`
- `src/bean/product/mod.rs`
- `src/bean/product/assistant/begin_timing_sale_param.rs`
- `src/bean/mod.rs`
- `src/enums/url_basics.rs`
- `src/enums/url_order.rs`
- `src/enums/url_after_sale.rs`
- `src/enums/url_product.rs`
- `src/api/wx_channel_basic_service.rs`
- `src/api/wx_channel_order_service.rs`
- `src/api/wx_channel_after_sale_service.rs`
- `src/api/wx_channel_product_service.rs`
- `src/api/impl/wx_channel_basic_service_impl.rs`
- `src/api/impl/wx_channel_order_service_impl.rs`
- `src/api/impl/wx_channel_after_sale_service_impl.rs`
- `src/api/impl/wx_channel_product_service_impl.rs`
