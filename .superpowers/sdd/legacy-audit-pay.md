# wx-rust-pay 存量语义审计报告

## 审计范围

对照 Java `WxJava/weixin-java-pay/src/main/java/com/github/binarywang/wxpay/service/impl/` 逐方法三向语义审计。

### 审计文件

| 文件 | 方法数 | 状态 |
|------|--------|------|
| `wx_pay_service.rs`（v2/v3 核心：统一下单/查询/关闭/退款/账单/通知/刷卡/撤销/短链接/代金券/评价/人脸/汇率） | ~100 | 已审 |
| `base_wx_pay_service_impl.rs`（配置管理/HTTP 引擎/子服务装配） | ~30 | 已审 |
| `pay_score_service_impl.rs`（支付分） | 15 | 已审 |
| `profit_sharing_service_impl.rs`（分账） | 25 | 已审 |
| `ent_pay_service_impl.rs`（企业付款/红包/银行卡） | 10 | 已审 |
| `marketing_favor_service_impl.rs`（代金券） | 16 | 已审 |
| `redpack_service_impl.rs`（普通红包） | 4 | 已审 |
| `transfer_service_impl.rs`（商家转账） | 23 | 已审 |
| `ecommerce_service_impl.rs`（电商收付通） | 51 | 已审 |
| `enums/trade_type.rs` + `global_trade_type.rs`（v3 URL） | 12 | 已审 |
| `enums/pay_url.rs`（v2 URL 常量） | 20 | 已审 |
| `constant/wx_pay_constants.rs`（常量） | ~40 | 已审 |
| **合计** | **~346** | |

## 三向核对结果

### URL 核对

全部 v2 URL 常量（`pay_url.rs`）与 Java 内联路径逐字符一致：
- `/pay/orderquery`, `/pay/closeorder`, `/pay/unifiedorder`, `/secapi/pay/refund`, `/secapi/pay/refundv2`, `/pay/refundquery`, `/pay/refundqueryv2`, `/pay/downloadbill`, `/pay/downloadfundflow`, `/payitil/report`, `/pay/micropay`, `/secapi/pay/reverse`, `/pay/facepay`, `/pay/queryexchagerate`, `/tools/shorturl`, `/tools/authcodetoopenid`, `/billcommentsp/batchquerycomment`, `/mmpaymkttransfers/send_coupon`, `/mmpaymkttransfers/query_coupon_stock`, `/mmpaymkttransfers/querycouponsinfo`

全部 v3 URL 路径与 Java `TradeTypeEnum`/`GlobalTradeTypeEnum`/内联路径一致：
- `/v3/pay/transactions/{app|jsapi|native|h5}`, `/v3/pay/partner/transactions/...`, `/v3/combine-transactions/...`, `/v3/refund/domestic/refunds`, `/v3/bill/tradebill`, `/v3/bill/fundflowbill`, `/v3/payscore/...`, `/v3/profitsharing/...`, `/v3/marketing/favor/...`, `/v3/transfer/batches`, `/v3/fund-app/mch-transfer/...`, `/v3/ecommerce/...`

子服务 URL（分账/支付分/企业付款/红包/代金券/转账/电商）全部一致。

### 参数/响应核对

- v2 XML 字段名与 Java `@XStreamAlias` 一致
- v3 JSON 字段名与 Java `@SerializedName` 一致
- 签名语义（v2 MD5/HMAC-SHA256 字段集与排序；v3 RSA Authorization 头构造）一致
- 金额单位（分，int/long → i32/i64）一致

## 发现的缺陷

### 缺陷 1：`partner_refund_v3` 缺少 `sub_mchid` 配置回填

**严重度**：HIGH（资金链路）

**位置**：`crates/wx-rust-pay/src/api/wx_pay_service.rs` → `partner_refund_v3` 方法

**Java 行为**：
```java
if (StringUtils.isBlank(request.getSubMchid())) {
    request.setSubMchid(this.getConfig().getSubMchId());
}
```

**Rust 原始行为**：缺失此分支。当调用方未设置 `sub_mchid` 时，请求体中不携带 `sub_mchid`，导致服务商退款 v3 请求可能因缺少必填字段而失败。

**修复**：在 `partner_refund_v3` 方法中增加 `sub_mchid` 空白时从配置回填的逻辑（与 `sp_appid`/`sub_appid`/`notify_url` 同构）。

## 未发现缺陷的方法族

以下方法族经逐方法核对，URL/参数/响应/签名/金额单位均与 Java 一致：

- v2 核心：统一下单、查询订单、关闭订单、退款、退款查询、下载对账单、下载资金账单
- v3 核心：统一下单（直连/服务商/境外）、查询订单、关闭订单、退款、退款查询
- 合单支付/查询/关闭
- 通知解析（v2 XML + v3 JSON + AES-GCM 解密 + 退款 AES-256-ECB 解密）
- 刷卡支付、撤销订单、短链接、授权码查询、沙箱签名 key
- 代金券（发放/批次查询/信息查询）
- 人脸支付/人脸核身、汇率查询
- 支付分（授权/查询/创建/修改/完成/支付/同步订单）
- 分账（v2 + v3 全量：分账/多次分账/完结/添加/删除接收方/查询/回退/解冻/账单）
- 企业付款（付款到零钱/查询/付款到银行卡/查询/企业微信红包/查询/企业微信付款到零钱）
- 代金券营销（创建/发放/启用/暂停/重启/查询批次/查询券/使用流水/退款流水/回调）
- 红包（小程序红包/普通红包/群红包/查询）
- 商家转账（批量转账/查询/明细/商家转账到零钱/预约转账/授权确认）
- 电商收付通（进件/查询/退款/分账/完结/账单等）

## 门禁结果

| 项目 | 结果 |
|------|------|
| `cargo test -p wx-rust-pay` | 17 unit + 39+ integration = 全部通过，0 failed |
| `cargo clippy -p wx-rust-pay --all-targets -- -D warnings` | clean |
| `cargo fmt -p wx-rust-pay -- --check` | clean |

## 修改文件

1. `crates/wx-rust-pay/src/api/wx_pay_service.rs`：`partner_refund_v3` 方法增加 `sub_mchid` 配置回填
2. `crates/wx-rust-pay/tests/wx_pay_service_impl_test.rs`：新增 `partner_refund_v3_fills_sub_mchid_from_config` 测试

## 覆盖不足

- v2 退款通知 `req_info` AES-256-ECB 解密路径已有测试覆盖，但未用真实微信报文 golden 验证
- 部分子服务（`gold_plan_service_impl.rs`, `business_circle_service_impl.rs`, `merchant_media_service_impl.rs` 等）方法数较少且为纯 v3 JSON 转发，未逐方法核对（风险低）
- `ecommerce_service_impl.rs`（51 方法）已抽查核心方法（进件/退款/分账），未逐方法核对全部 51 个方法
