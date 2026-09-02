# 微信支付官方文档对齐核查 + 个人主体小程序虚拟支付补齐报告

日期：2026-09-02
文档基线：`https://pay.weixin.qq.com/doc/v3/merchant/llms.txt`（更新时间 2026.09.02）+
微信开放社区虚拟支付服务端 API（`developers.weixin.qq.com` `/xpay/*` 系列，37 个接口）

## 一、merchant 商户文档 vs wx-rust-pay 对齐结论

### 已覆盖（32 章节中 30 个 ✅）

merchant 文档 32 个产品章节与 `wx-rust-pay` 38 个 service 对照：

| merchant 章节 | pay service | 状态 |
|---|---|---|
| 付款码支付（V2） | wx_pay（micropay v2） | ✅ |
| 刷脸支付 | wx_pay（facepay 转发） | ✅ |
| JSAPI / APP / H5 / Native / 小程序支付 | wx_pay（v3 transactions 全套：下单/查单/关单/退款/账单/回调） | ✅ |
| 5 种合单支付 | wx_pay（combine transactions） | ✅ |
| 订单退款 / 下载账单 | wx_pay（refund/fundflowbill/tradedbill） | ✅ |
| 分账 | profit_sharing | ✅ |
| 商家转账 | transfer / brand_merchant_transfer / merchant_transfer / partner_transfer | ✅ |
| 微信支付分 | pay_score / partner_pay_score(+sign_plan) | ✅ |
| 支付分停车 | pay_score（parking） | ✅ |
| 现金红包（V2） | redpack | ✅ |
| 代金券 | marketing_busi_favor / marketing_favor | ✅ |
| 委托营销 | marketing_busi_favor（partner） | ✅ |
| 支付有礼 / 发券插件 / H5发券 | marketing_favor / marketing_media | ✅ |
| 智慧商圈 | business_circle / business_operation_transfer | ✅ |
| 支付即服务 | wx_pay（service shift） | ✅ |
| 清关报关（V2） | custom_declaration | ✅ |
| 消费者投诉 2.0 | complaint | ✅ |
| 证书密钥 / 微信支付公钥 / 平台证书 | cert v3（CertificateDownloader/Verifier） | ✅ |
| 商户进件/超级管理员/结算账户等 | applyment4_sub / bank / payroll / ent_pay / gold_plan / ecommerce / partner_invoice / real_name / subscription_billing / wx_deposit / wx_entrust_pap / mi_pay 等 | ✅（超出 WxJava 的 WxRust 增量） |

### 唯二缺口（均为 2025-2026 新产品，WxJava 4.8.6 亦未覆盖）⏸

| 章节 | API | 决策建议 |
|---|---|---|
| 医保支付（自费混合收款下单等） | 定向开放（需医疗机构资质） | 暂不实现：无公开 SDK 通路，接入需资质白名单 |
| AI 支付（x402 预下单等） | 极新（文档 2026 年上线） | 记录跟踪，待 API 稳定后评估 |

## 二、个人主体小程序虚拟支付（本次补齐 ✅）

### 背景

虚拟支付服务端 API 在**微信开放社区文档**（不在 pay 商户文档 llms.txt 内），
`/xpay/*` 共 37 个接口。WxRust 原有 30 个方法（镜像 WxJava 4.8.6 的 28 个 +
2 个 g4 补充），官方 2026 年新增的 8 个接口此前缺失——其中**订阅系列 4 个
正是个人主体小程序虚拟支付的新能力**（订阅制道具：签约→预通知→扣款→解约）。

### 本次新增 8 个接口（`WxMaXPayService` 30 → 38 方法）

| # | API | 用途 | 关键语义 |
|---|---|---|---|
| 1 | `query_subscribe_contract` | 查询签约关系 | authorization_state: SIGNED/TERMINATED/UNBINDUSER |
| 2 | `send_subscribe_pre_payment` | 预通知扣款 | 窗口约束：T-3 前禁发、T+8 内禁发、07:10~21:50 |
| 3 | `submit_subscribe_pay_order` | 发起订阅扣款 | 受理≠扣款成功；失败 T~T+6 可重试（同用户同道具 1 次/时） |
| 4 | `cancel_subscribe_contract` | 商家解约 | 终态后同协议号不可再签约 |
| 5 | `start_download_order` | 下载支付订单任务 | order_type 1 代币/2 道具/3 订阅/4 退款；≤31 天窗口 |
| 6 | `query_download_order` | 查询下载任务 | status 0 初始化/1 运行/2 成功/3 失败；url 到 expire_at 失效 |
| 7 | `download_ios_settlement_bill` | iOS 月结账单 | 逐月 bill_list，链接有时效 |
| 8 | `query_punishment_reasons` | 商户管控原因 | recovery_specifications：管控原因+影响能力+解脱路径 |

### 变更清单

- **bean**：14 个新文件（8 request + 5 response + iOS 结算单嵌套 `WxMaXPayIosSettlementBill`；
  punish 响应含 `WxMaXPayRecoverySpecification` 12 字段嵌套结构）
- **URL**：`enums/url_g4_ability.rs::xpay` 新增 8 个（全部单 `pay_sig` 签名，官方未要求用户态 signature）
- **trait/impl**：`WxMaXPayService` + `WxMaXPayServiceImpl` 各 +8 方法（复用现有
  `sign_uri_with_pay` + `post_signed` 通路）
- **测试**：`tests/xpay_subscribe_download_test.rs` 13 个用例——golden 取自
  官方文档请求/响应示例；断言响应解析、请求体字段、URL 路径 + pay_sig 注入、
  错误码上抛（-15027 重复下单、690000000 未签约）

### 个人主体限制（接入时注意）

- 个人主体小程序月支付限额 **10 万元**
- 不支持需电信业务资质的服务（电子邮件、信息发布平台等）
- 前端拉起用 `wx.requestVirtualPayment`（服务端即本 service）

## 三、验证

- `cargo test -p wx-rust-miniapp`：474 passed / 0 failed（含 13 新用例）
- 全量 workspace：见提交说明（fmt clean、clippy 0 error）

## 四、来源

- [微信支付商户平台文档 llms.txt](https://pay.weixin.qq.com/doc/v3/merchant/llms.txt)
- [虚拟支付：个人主体](https://developers.weixin.qq.com/miniprogram/dev/platform-capabilities/business-capabilities/virtual-payment/person.html)
- [虚拟支付服务端 API 列表](https://developers.weixin.qq.com/miniprogram/dev/server/API/VirtualPayment/)
